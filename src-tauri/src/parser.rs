use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum FiscalDocumentType {
    Nfe,
    Nfce,
    Cte,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ParsedFiscalDocument {
    pub source_name: String,
    pub access_key: String,
    pub document_type: FiscalDocumentType,
    pub issue_date: Option<String>,
    pub document_number: String,
    pub total_value: Option<String>,
    pub cfops: Vec<String>,
    pub descriptions: Vec<String>,
    pub issuer: Option<FiscalParty>,
    pub taker: Option<FiscalParty>,
    pub recipient: Option<FiscalParty>,
    pub sender: Option<FiscalParty>,
    pub product_items: Vec<ProductItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProductItem {
    pub description: String,
    pub ncm: String,
    pub cest: String,
    pub gtin: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FiscalParty {
    pub name: String,
    pub document: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ParseWarning {
    pub source_name: String,
    pub reason: ParseWarningReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ParseWarningReason {
    MalformedXml,
    UnsupportedDocumentType,
    MissingRequiredField,
    IgnoredEventDocument,
}

pub fn parse_fiscal_document(
    source_name: impl Into<String>,
    xml_content: &str,
) -> Result<ParsedFiscalDocument, ParseWarning> {
    let source_name = source_name.into();
    let document = roxmltree::Document::parse(xml_content).map_err(|_| ParseWarning {
        source_name: source_name.clone(),
        reason: ParseWarningReason::MalformedXml,
    })?;

    if is_event_document(&document) {
        return Err(ParseWarning {
            source_name,
            reason: ParseWarningReason::IgnoredEventDocument,
        });
    }

    if let Some(inf_cte) = first_descendant(&document, "infCte") {
        return parse_cte(source_name, inf_cte);
    }

    if let Some(inf_nfe) = first_descendant(&document, "infNFe") {
        return parse_nfe(source_name, inf_nfe);
    }

    Err(ParseWarning {
        source_name,
        reason: ParseWarningReason::UnsupportedDocumentType,
    })
}

fn parse_nfe(
    source_name: String,
    inf_nfe: roxmltree::Node<'_, '_>,
) -> Result<ParsedFiscalDocument, ParseWarning> {
    let ide = child(inf_nfe, "ide");
    let total = child(inf_nfe, "total").and_then(|node| child(node, "ICMSTot"));
    let document_number = ide
        .and_then(|node| child_text(node, "nNF"))
        .ok_or_else(|| missing_required_field(source_name.clone()))?;
    let model = ide.and_then(|node| child_text(node, "mod"));
    let document_type = if model.as_deref() == Some("65") {
        FiscalDocumentType::Nfce
    } else {
        FiscalDocumentType::Nfe
    };

    Ok(ParsedFiscalDocument {
        source_name,
        access_key: access_key_from_id(inf_nfe),
        document_type,
        issue_date: ide
            .and_then(|node| child_text(node, "dhEmi").or_else(|| child_text(node, "dEmi"))),
        document_number,
        total_value: total.and_then(|node| child_text(node, "vNF")),
        cfops: descendants_text(inf_nfe, "CFOP"),
        descriptions: descendants_text(inf_nfe, "xProd"),
        issuer: child(inf_nfe, "emit").and_then(party_from_node),
        taker: None,
        recipient: child(inf_nfe, "dest").and_then(party_from_node),
        sender: None,
        product_items: nfe_product_items(inf_nfe),
    })
}

fn parse_cte(
    source_name: String,
    inf_cte: roxmltree::Node<'_, '_>,
) -> Result<ParsedFiscalDocument, ParseWarning> {
    let ide = child(inf_cte, "ide");
    let document_number = ide
        .and_then(|node| child_text(node, "nCT"))
        .ok_or_else(|| missing_required_field(source_name.clone()))?;

    Ok(ParsedFiscalDocument {
        source_name,
        access_key: access_key_from_id(inf_cte),
        document_type: FiscalDocumentType::Cte,
        issue_date: ide
            .and_then(|node| child_text(node, "dhEmi").or_else(|| child_text(node, "dEmi"))),
        document_number,
        total_value: child(inf_cte, "vPrest")
            .and_then(|node| child_text(node, "vTPrest"))
            .or_else(|| child(inf_cte, "imp").and_then(|node| child_text(node, "vTotTrib"))),
        cfops: descendants_text(inf_cte, "CFOP"),
        descriptions: cte_descriptions(inf_cte),
        issuer: child(inf_cte, "emit").and_then(party_from_node),
        taker: cte_taker(inf_cte),
        recipient: child(inf_cte, "dest").and_then(party_from_node),
        sender: child(inf_cte, "rem").and_then(party_from_node),
        product_items: Vec::new(),
    })
}

fn first_descendant<'a>(
    document: &'a roxmltree::Document<'a>,
    name: &str,
) -> Option<roxmltree::Node<'a, 'a>> {
    document.descendants().find(|node| has_name(*node, name))
}

fn child<'a>(node: roxmltree::Node<'a, '_>, name: &str) -> Option<roxmltree::Node<'a, 'a>> {
    node.children().find(|child| has_name(*child, name))
}

fn child_text(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    child(node, name).and_then(node_text)
}

fn node_text(node: roxmltree::Node<'_, '_>) -> Option<String> {
    node.text()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn descendants_text(node: roxmltree::Node<'_, '_>, name: &str) -> Vec<String> {
    node.descendants()
        .filter(|descendant| has_name(*descendant, name))
        .filter_map(node_text)
        .collect()
}

fn cte_descriptions(inf_cte: roxmltree::Node<'_, '_>) -> Vec<String> {
    let descriptions = descendants_text(inf_cte, "xProd");

    if !descriptions.is_empty() {
        return descriptions;
    }

    let service_descriptions = descendants_text(inf_cte, "xServ");
    if !service_descriptions.is_empty() {
        return service_descriptions;
    }

    descendants_text(inf_cte, "proPred")
}

fn nfe_product_items(inf_nfe: roxmltree::Node<'_, '_>) -> Vec<ProductItem> {
    inf_nfe
        .children()
        .filter(|node| has_name(*node, "det"))
        .filter_map(|det| child(det, "prod"))
        .filter_map(product_item_from_prod)
        .collect()
}

fn product_item_from_prod(prod: roxmltree::Node<'_, '_>) -> Option<ProductItem> {
    let description = child_text(prod, "xProd")?;

    Some(ProductItem {
        description,
        ncm: child_text(prod, "NCM").unwrap_or_default(),
        cest: child_text(prod, "CEST").unwrap_or_default(),
        gtin: gtin_from_prod(prod),
    })
}

fn gtin_from_prod(prod: roxmltree::Node<'_, '_>) -> String {
    child_text(prod, "cEAN")
        .filter(|value| is_real_gtin(value))
        .or_else(|| child_text(prod, "cEANTrib").filter(|value| is_real_gtin(value)))
        .unwrap_or_default()
}

fn is_real_gtin(value: &str) -> bool {
    let normalized = value.trim();
    !normalized.is_empty() && !normalized.eq_ignore_ascii_case("SEM GTIN")
}

fn cte_taker(inf_cte: roxmltree::Node<'_, '_>) -> Option<FiscalParty> {
    if let Some(taker) = child(inf_cte, "toma").and_then(party_from_node) {
        return Some(taker);
    }

    if let Some(taker) = child(inf_cte, "toma4").and_then(party_from_node) {
        return Some(taker);
    }

    let ide = child(inf_cte, "ide")?;
    let toma_code = child(ide, "toma3").and_then(|node| child_text(node, "toma"))?;

    match toma_code.as_str() {
        "0" => child(inf_cte, "rem").and_then(party_from_node),
        "1" => child(inf_cte, "exped").and_then(party_from_node),
        "2" => child(inf_cte, "receb").and_then(party_from_node),
        "3" => child(inf_cte, "dest").and_then(party_from_node),
        _ => None,
    }
}

fn party_from_node(node: roxmltree::Node<'_, '_>) -> Option<FiscalParty> {
    let name = child_text(node, "xNome").or_else(|| child_text(node, "xFant"))?;
    let document = child_text(node, "CNPJ").or_else(|| child_text(node, "CPF"))?;

    Some(FiscalParty { name, document })
}

fn access_key_from_id(node: roxmltree::Node<'_, '_>) -> String {
    node.attribute("Id")
        .map(|id| id.trim_start_matches("NFe").trim_start_matches("CTe"))
        .unwrap_or_default()
        .to_string()
}

fn has_name(node: roxmltree::Node<'_, '_>, name: &str) -> bool {
    node.is_element() && node.tag_name().name() == name
}

fn is_event_document(document: &roxmltree::Document<'_>) -> bool {
    document.descendants().any(|node| {
        has_name(node, "procEventoCTe")
            || has_name(node, "eventoCTe")
            || has_name(node, "retEventoCTe")
            || has_name(node, "procEventoNFe")
            || has_name(node, "evento")
            || has_name(node, "retEvento")
    })
}

fn missing_required_field(source_name: String) -> ParseWarning {
    ParseWarning {
        source_name,
        reason: ParseWarningReason::MissingRequiredField,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_nfe() {
        let parsed = parse_fiscal_document("nfe.xml", VALID_NFE).unwrap();

        assert_eq!(parsed.document_type, FiscalDocumentType::Nfe);
        assert_eq!(
            parsed.access_key,
            "35260412345678000190550010000001231000001234"
        );
        assert_eq!(parsed.document_number, "123");
        assert_eq!(
            parsed.issue_date.as_deref(),
            Some("2026-04-24T10:30:00-03:00")
        );
        assert_eq!(parsed.total_value.as_deref(), Some("150.75"));
        assert_eq!(parsed.cfops, vec!["5102"]);
        assert_eq!(parsed.descriptions, vec!["Produto Teste"]);
        assert_eq!(
            parsed.product_items,
            vec![ProductItem {
                description: "Produto Teste".to_string(),
                ncm: "12345678".to_string(),
                cest: "1200100".to_string(),
                gtin: "7891234567895".to_string(),
            }]
        );
        assert_eq!(
            parsed.issuer,
            Some(FiscalParty {
                name: "Empresa Emitente".to_string(),
                document: "12345678000190".to_string()
            })
        );
        assert_eq!(
            parsed.recipient,
            Some(FiscalParty {
                name: "Cliente Destinatario".to_string(),
                document: "98765432000110".to_string()
            })
        );
        assert_eq!(parsed.taker, None);
        assert_eq!(parsed.sender, None);
    }

    #[test]
    fn parses_valid_cte() {
        let parsed = parse_fiscal_document("cte.xml", VALID_CTE).unwrap();

        assert_eq!(parsed.document_type, FiscalDocumentType::Cte);
        assert_eq!(
            parsed.access_key,
            "35260412345678000190570010000004561000004567"
        );
        assert_eq!(parsed.document_number, "456");
        assert_eq!(parsed.total_value.as_deref(), Some("320.00"));
        assert_eq!(parsed.cfops, vec!["5353"]);
        assert_eq!(parsed.descriptions, vec!["Servico de transporte"]);
        assert!(parsed.product_items.is_empty());
        assert_eq!(
            parsed.taker,
            Some(FiscalParty {
                name: "Tomador do Servico".to_string(),
                document: "11111111000111".to_string()
            })
        );
        assert_eq!(
            parsed.sender,
            Some(FiscalParty {
                name: "Remetente Ltda".to_string(),
                document: "22222222000122".to_string()
            })
        );
        assert_eq!(
            parsed.recipient,
            Some(FiscalParty {
                name: "Destinatario Ltda".to_string(),
                document: "33333333000133".to_string()
            })
        );
    }

    #[test]
    fn parses_real_nfe_example() {
        let parsed = parse_fiscal_document("entrada.xml", REAL_NFE_EXAMPLE).unwrap();

        assert_eq!(parsed.document_type, FiscalDocumentType::Nfe);
        assert_eq!(
            parsed.access_key,
            "26260120554816000174550010000223161222456849"
        );
        assert_eq!(parsed.document_number, "22316");
        assert_eq!(parsed.total_value.as_deref(), Some("4336.54"));
        assert_eq!(parsed.cfops, vec!["5929", "5929"]);
        assert_eq!(
            parsed.descriptions,
            vec!["DIESEL S10", "ARLA32 - IPE ARLA 32 - 20L"]
        );
        assert_eq!(
            parsed.product_items,
            vec![
                ProductItem {
                    description: "DIESEL S10".to_string(),
                    ncm: "27101921".to_string(),
                    cest: "0600605".to_string(),
                    gtin: String::new(),
                },
                ProductItem {
                    description: "ARLA32 - IPE ARLA 32 - 20L".to_string(),
                    ncm: "31021010".to_string(),
                    cest: "0600700".to_string(),
                    gtin: String::new(),
                },
            ]
        );
        assert_eq!(
            parsed.recipient,
            Some(FiscalParty {
                name: "RODRIGUES & SOUZA COMERCIO DE CARNE LTDA".to_string(),
                document: "32747045000110".to_string()
            })
        );
    }

    #[test]
    fn parses_real_cte_example_with_toma3_and_propred() {
        let parsed = parse_fiscal_document("CTE SAIDA.xml", REAL_CTE_EXAMPLE).unwrap();

        assert_eq!(parsed.document_type, FiscalDocumentType::Cte);
        assert_eq!(
            parsed.access_key,
            "15260132747045000110570010000001641300000804"
        );
        assert_eq!(parsed.document_number, "164");
        assert_eq!(parsed.total_value.as_deref(), Some("6310.70"));
        assert_eq!(parsed.cfops, vec!["5352"]);
        assert_eq!(parsed.descriptions, vec!["BOI PARA ABATE"]);
        assert!(parsed.product_items.is_empty());
        assert_eq!(
            parsed.issuer,
            Some(FiscalParty {
                name: "CASA DE CARNE MINEIRA".to_string(),
                document: "32747045000110".to_string()
            })
        );
        assert_eq!(
            parsed.taker,
            Some(FiscalParty {
                name: "AMILCAR LEAO GONCALVES DIAS".to_string(),
                document: "17137063268".to_string()
            })
        );
    }

    #[test]
    fn prioritizes_cte_when_document_contains_nested_infnfe_reference() {
        let parsed = parse_fiscal_document("cte.xml", REAL_CTE_EXAMPLE).unwrap();

        assert_eq!(parsed.document_type, FiscalDocumentType::Cte);
        assert_eq!(parsed.document_number, "164");
    }

    #[test]
    fn unsupported_xml_returns_warning() {
        let warning =
            parse_fiscal_document("outro.xml", "<root><valor>1</valor></root>").unwrap_err();

        assert_eq!(warning.source_name, "outro.xml");
        assert_eq!(warning.reason, ParseWarningReason::UnsupportedDocumentType);
    }

    #[test]
    fn malformed_xml_returns_warning() {
        let warning =
            parse_fiscal_document("quebrado.xml", "<nfeProc><NFe></nfeProc>").unwrap_err();

        assert_eq!(warning.source_name, "quebrado.xml");
        assert_eq!(warning.reason, ParseWarningReason::MalformedXml);
    }

    #[test]
    fn event_xml_is_ignored_without_being_treated_as_supported_document() {
        let warning = parse_fiscal_document("evento.xml", EVENT_CTE_EXAMPLE).unwrap_err();

        assert_eq!(warning.source_name, "evento.xml");
        assert_eq!(warning.reason, ParseWarningReason::IgnoredEventDocument);
    }

    #[test]
    fn nfe_product_items_keep_optional_fields_blank_and_skip_missing_description() {
        let parsed = parse_fiscal_document("nfe.xml", NFE_WITH_OPTIONAL_PRODUCT_FIELDS).unwrap();

        assert_eq!(
            parsed.product_items,
            vec![
                ProductItem {
                    description: "Produto sem opcionais".to_string(),
                    ncm: String::new(),
                    cest: String::new(),
                    gtin: String::new(),
                },
                ProductItem {
                    description: "Produto com GTIN tributavel".to_string(),
                    ncm: "22021000".to_string(),
                    cest: String::new(),
                    gtin: "7890000000001".to_string(),
                },
            ]
        );
    }

    const VALID_NFE: &str = r#"
        <nfeProc>
          <NFe>
            <infNFe Id="NFe35260412345678000190550010000001231000001234">
              <ide>
                <mod>55</mod>
                <nNF>123</nNF>
                <dhEmi>2026-04-24T10:30:00-03:00</dhEmi>
              </ide>
              <emit>
                <CNPJ>12345678000190</CNPJ>
                <xNome>Empresa Emitente</xNome>
              </emit>
              <dest>
                <CNPJ>98765432000110</CNPJ>
                <xNome>Cliente Destinatario</xNome>
              </dest>
              <det nItem="1">
                <prod>
                  <cEAN>7891234567895</cEAN>
                  <CFOP>5102</CFOP>
                  <xProd>Produto Teste</xProd>
                  <NCM>12345678</NCM>
                  <CEST>1200100</CEST>
                </prod>
              </det>
              <total>
                <ICMSTot>
                  <vNF>150.75</vNF>
                </ICMSTot>
              </total>
            </infNFe>
          </NFe>
        </nfeProc>
    "#;

    const NFE_WITH_OPTIONAL_PRODUCT_FIELDS: &str = r#"
        <nfeProc>
          <NFe>
            <infNFe Id="NFe35260412345678000190550010000001231000001234">
              <ide>
                <mod>55</mod>
                <nNF>123</nNF>
              </ide>
              <det nItem="1">
                <prod>
                  <xProd>Produto sem opcionais</xProd>
                  <cEAN>SEM GTIN</cEAN>
                </prod>
              </det>
              <det nItem="2">
                <prod>
                  <NCM>01012100</NCM>
                  <cEAN>7899999999999</cEAN>
                </prod>
              </det>
              <det nItem="3">
                <prod>
                  <xProd>Produto com GTIN tributavel</xProd>
                  <NCM>22021000</NCM>
                  <cEAN>SEM GTIN</cEAN>
                  <cEANTrib>7890000000001</cEANTrib>
                </prod>
              </det>
            </infNFe>
          </NFe>
        </nfeProc>
    "#;

    const VALID_CTE: &str = r#"
        <cteProc>
          <CTe>
            <infCte Id="CTe35260412345678000190570010000004561000004567">
              <ide>
                <nCT>456</nCT>
                <dhEmi>2026-04-24T11:00:00-03:00</dhEmi>
                <CFOP>5353</CFOP>
              </ide>
              <emit>
                <CNPJ>12345678000190</CNPJ>
                <xNome>Transportadora Emitente</xNome>
              </emit>
              <rem>
                <CNPJ>22222222000122</CNPJ>
                <xNome>Remetente Ltda</xNome>
              </rem>
              <dest>
                <CNPJ>33333333000133</CNPJ>
                <xNome>Destinatario Ltda</xNome>
              </dest>
              <toma>
                <CNPJ>11111111000111</CNPJ>
                <xNome>Tomador do Servico</xNome>
              </toma>
              <vPrest>
                <vTPrest>320.00</vTPrest>
              </vPrest>
              <infCTeNorm>
                <infCarga>
                  <xProd>Servico de transporte</xProd>
                </infCarga>
              </infCTeNorm>
            </infCte>
          </CTe>
        </cteProc>
    "#;

    const REAL_NFE_EXAMPLE: &str = include_str!("../../exemplos_xml/entrada.xml");
    const REAL_CTE_EXAMPLE: &str = include_str!("../../exemplos_xml/CTE SAÍDA.xml");
    const EVENT_CTE_EXAMPLE: &str = include_str!("../../exemplos_xml/evento.xml");
}

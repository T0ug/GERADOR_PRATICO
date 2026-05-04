use crate::parser::ParsedFiscalDocument;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum DocumentClassification {
    Entrada,
    Saida,
    SemCnpjIdentificado,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ClassifiedDocument {
    pub document: ParsedFiscalDocument,
    pub classification: DocumentClassification,
}

pub fn classify_document(
    document: ParsedFiscalDocument,
    informed_cnpj: &str,
) -> ClassifiedDocument {
    let sanitized_informed = sanitize_cnpj(informed_cnpj);

    let classification = if let Some(issuer) = &document.issuer {
        if sanitize_cnpj(&issuer.document) == sanitized_informed {
            DocumentClassification::Saida
        } else if is_entrada(&document, &sanitized_informed) {
            DocumentClassification::Entrada
        } else {
            DocumentClassification::SemCnpjIdentificado
        }
    } else if is_entrada(&document, &sanitized_informed) {
        DocumentClassification::Entrada
    } else {
        DocumentClassification::SemCnpjIdentificado
    };

    ClassifiedDocument {
        document,
        classification,
    }
}

fn is_entrada(document: &ParsedFiscalDocument, sanitized_informed: &str) -> bool {
    if let Some(recipient) = &document.recipient {
        if sanitize_cnpj(&recipient.document) == sanitized_informed {
            return true;
        }
    }

    if let Some(taker) = &document.taker {
        if sanitize_cnpj(&taker.document) == sanitized_informed {
            return true;
        }
    }

    false
}

fn sanitize_cnpj(cnpj: &str) -> String {
    cnpj.chars().filter(|c| c.is_ascii_digit()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{FiscalDocumentType, FiscalParty};

    fn make_doc(
        issuer_cnpj: Option<&str>,
        recipient_cnpj: Option<&str>,
        taker_cnpj: Option<&str>,
        sender_cnpj: Option<&str>,
    ) -> ParsedFiscalDocument {
        ParsedFiscalDocument {
            source_name: "test.xml".to_string(),
            access_key: "123".to_string(),
            document_type: FiscalDocumentType::Nfe,
            issue_date: None,
            document_number: "1".to_string(),
            total_value: None,
            cfops: vec![],
            descriptions: vec![],
            issuer: issuer_cnpj.map(|d| FiscalParty {
                name: "N".to_string(),
                document: d.to_string(),
            }),
            recipient: recipient_cnpj.map(|d| FiscalParty {
                name: "N".to_string(),
                document: d.to_string(),
            }),
            taker: taker_cnpj.map(|d| FiscalParty {
                name: "N".to_string(),
                document: d.to_string(),
            }),
            sender: sender_cnpj.map(|d| FiscalParty {
                name: "N".to_string(),
                document: d.to_string(),
            }),
            product_items: vec![],
        }
    }

    #[test]
    fn issuer_equals_informed_is_saida() {
        let doc = make_doc(Some("12.345.678/0001-90"), None, None, None);
        let classified = classify_document(doc, "12345678000190");
        assert_eq!(classified.classification, DocumentClassification::Saida);
    }

    #[test]
    fn recipient_equals_informed_is_entrada() {
        let doc = make_doc(
            Some("99.999.999/0001-99"),
            Some("12.345.678/0001-90"),
            None,
            None,
        );
        let classified = classify_document(doc, "12345678000190");
        assert_eq!(classified.classification, DocumentClassification::Entrada);
    }

    #[test]
    fn taker_equals_informed_is_entrada() {
        let doc = make_doc(
            Some("99.999.999/0001-99"),
            None,
            Some("12345678000190"),
            None,
        );
        let classified = classify_document(doc, "12345678000190");
        assert_eq!(classified.classification, DocumentClassification::Entrada);
    }

    #[test]
    fn not_found_in_expected_parties_is_sem_cnpj_identificado() {
        let doc = make_doc(
            Some("99.999.999/0001-99"),
            Some("88.888.888/0001-88"),
            None,
            None,
        );
        let classified = classify_document(doc, "12345678000190");
        assert_eq!(
            classified.classification,
            DocumentClassification::SemCnpjIdentificado
        );
    }

    #[test]
    fn comparison_ignores_mask() {
        let doc = make_doc(Some("12345678000190"), None, None, None);
        let classified = classify_document(doc, "12.345.678/0001-90");
        assert_eq!(classified.classification, DocumentClassification::Saida);

        let doc2 = make_doc(None, Some("12.345.678/0001-90"), None, None);
        let classified2 = classify_document(doc2, "12345678000190");
        assert_eq!(classified2.classification, DocumentClassification::Entrada);
    }

    #[test]
    fn sender_alone_does_not_classify_as_entrada() {
        let doc = make_doc(
            Some("99.999.999/0001-99"),
            None,
            None,
            Some("12345678000190"),
        );
        let classified = classify_document(doc, "12345678000190");
        // It matches sender, but sender is NOT entry criteria. So it should be SemCnpjIdentificado
        assert_eq!(
            classified.classification,
            DocumentClassification::SemCnpjIdentificado
        );
    }
}

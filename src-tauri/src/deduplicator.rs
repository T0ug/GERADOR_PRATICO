use crate::parser::ParsedFiscalDocument;
use std::collections::HashSet;

#[derive(Default)]
pub struct AccessKeySet {
    keys: HashSet<String>,
}

impl AccessKeySet {
    pub fn insert(&mut self, key: String) -> bool {
        self.keys.insert(key)
    }
}

pub fn deduplicate_by_access_key(
    documents: Vec<ParsedFiscalDocument>,
) -> Vec<ParsedFiscalDocument> {
    let mut access_keys = AccessKeySet::default();
    let mut unique_documents = Vec::with_capacity(documents.len());

    for document in documents {
        if document.access_key.is_empty() || access_keys.insert(document.access_key.clone()) {
            unique_documents.push(document);
        }
    }

    unique_documents
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::FiscalDocumentType;

    #[test]
    fn keeps_documents_with_different_access_keys() {
        let documents = vec![
            document("primeiro.xml", "CHAVE-1", "1"),
            document("segundo.xml", "CHAVE-2", "2"),
        ];

        let deduplicated = deduplicate_by_access_key(documents);

        assert_eq!(source_names(&deduplicated), vec!["primeiro.xml", "segundo.xml"]);
    }

    #[test]
    fn keeps_only_first_document_with_duplicate_access_key() {
        let documents = vec![
            document("primeiro.xml", "CHAVE-1", "1"),
            document("duplicado.xml", "CHAVE-1", "2"),
            document("terceiro.xml", "CHAVE-3", "3"),
        ];

        let deduplicated = deduplicate_by_access_key(documents);

        assert_eq!(source_names(&deduplicated), vec!["primeiro.xml", "terceiro.xml"]);
        assert_eq!(deduplicated[0].document_number, "1");
    }

    #[test]
    fn keeps_documents_without_access_key() {
        let documents = vec![
            document("sem-chave-1.xml", "", "1"),
            document("sem-chave-2.xml", "", "2"),
            document("com-chave.xml", "CHAVE-1", "3"),
        ];

        let deduplicated = deduplicate_by_access_key(documents);

        assert_eq!(
            source_names(&deduplicated),
            vec!["sem-chave-1.xml", "sem-chave-2.xml", "com-chave.xml"]
        );
    }

    #[test]
    fn preserves_order_of_kept_documents() {
        let documents = vec![
            document("a.xml", "A", "1"),
            document("b.xml", "B", "2"),
            document("a-duplicado.xml", "A", "3"),
            document("c.xml", "C", "4"),
        ];

        let deduplicated = deduplicate_by_access_key(documents);

        assert_eq!(source_names(&deduplicated), vec!["a.xml", "b.xml", "c.xml"]);
    }

    fn source_names(documents: &[ParsedFiscalDocument]) -> Vec<&str> {
        documents
            .iter()
            .map(|document| document.source_name.as_str())
            .collect()
    }

    fn document(source_name: &str, access_key: &str, document_number: &str) -> ParsedFiscalDocument {
        ParsedFiscalDocument {
            source_name: source_name.to_string(),
            access_key: access_key.to_string(),
            document_type: FiscalDocumentType::Nfe,
            issue_date: None,
            document_number: document_number.to_string(),
            total_value: None,
            cfops: Vec::new(),
            descriptions: Vec::new(),
            issuer: None,
            taker: None,
            recipient: None,
            sender: None,
        }
    }
}

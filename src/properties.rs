use std::collections::HashMap;

use model::UniparcXRef2Property;

/// Keep all properties for a given UniParc ID.
pub struct Properties<T> {
    pub ncbi_gi: T,
    pub ncbi_taxonomy_id: T,
    pub protein_name: T,
    pub gene_name: T,
    pub chain: T,
    pub uniprot_kb_accession: T,
    pub proteome_id: T,
    pub component: T,
}

impl Default for Properties<Vec<UniparcXRef2Property>> {
    fn default() -> Properties<Vec<UniparcXRef2Property>> {
        Properties {
            ncbi_gi: Vec::new(),
            ncbi_taxonomy_id: Vec::new(),
            protein_name: Vec::new(),
            gene_name: Vec::new(),
            chain: Vec::new(),
            uniprot_kb_accession: Vec::new(),
            proteome_id: Vec::new(),
            component: Vec::new(),
        }
    }
}

impl Default for Properties<HashMap<String, u64>> {
    fn default() -> Properties<HashMap<String, u64>> {
        Properties {
            ncbi_gi: HashMap::new(),
            ncbi_taxonomy_id: HashMap::new(),
            protein_name: HashMap::new(),
            gene_name: HashMap::new(),
            chain: HashMap::new(),
            uniprot_kb_accession: HashMap::new(),
            proteome_id: HashMap::new(),
            component: HashMap::new(),
        }
    }
}

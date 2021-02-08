use std::collections::HashMap;

use model::UniparcProperty;

/// Keep all properties for a given UniParc ID
pub struct Properties<T> {
    pub component: T,
    pub gene_name: T,
    pub ncbi_gi: T,
    pub ncbi_taxonomy_id: T,
    pub pdb_chain: T,
    pub protein_name: T,
    pub proteome_id: T,
    pub uniprot_kb_accession: T,
}

impl Default for Properties<Vec<UniparcProperty>> {
    fn default() -> Properties<Vec<UniparcProperty>> {
        Properties {
            component: Vec::new(),
            gene_name: Vec::new(),
            ncbi_gi: Vec::new(),
            ncbi_taxonomy_id: Vec::new(),
            pdb_chain: Vec::new(),
            protein_name: Vec::new(),
            proteome_id: Vec::new(),
            uniprot_kb_accession: Vec::new(),
        }
    }
}

impl Default for Properties<HashMap<String, u64>> {
    fn default() -> Properties<HashMap<String, u64>> {
        Properties {
            component: HashMap::new(),
            gene_name: HashMap::new(),
            ncbi_gi: HashMap::new(),
            ncbi_taxonomy_id: HashMap::new(),
            pdb_chain: HashMap::new(),
            protein_name: HashMap::new(),
            proteome_id: HashMap::new(),
            uniprot_kb_accession: HashMap::new(),
        }
    }
}

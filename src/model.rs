#[derive(Default)]
pub struct Uniparc {
    pub id: String,
    pub sequence: String,
    pub sequence_length: u32,
    pub sequence_checksum: String,
}

pub struct UniparcXRef {
    pub uniparc_id: String,
    pub idx: u64,
    pub db_type: String,
    pub db_id: String,
    pub version_i: String,
    pub active: String,
    pub version: String,
    pub created: String,
    pub last: String,
}

pub struct UniparcXRef2Property {
    pub uniparc_id: String,
    pub uniparc_xref_idx: u64,
    pub property_name: String,
    pub property_idx: u64,
}

pub struct UniparcProperty {
    pub uniparc_id: String,
    pub name: String,
    pub idx: u64,
    pub value: String,
}

#[derive(Clone)]
pub struct UniparcDomain {
    pub uniparc_id: String,
    pub database: String,
    pub database_id: String,
    pub interpro_name: String,
    pub interpro_id: String,
    pub domain_start: u32,
    pub domain_end: u32,
}

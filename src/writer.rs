use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use flate2::write::GzEncoder;
use flate2::{Compression, GzBuilder};

use model::{Uniparc, UniparcDomain, UniparcProperty, UniparcXRef, UniparcXRef2Property};
use properties::Properties;

/// Struct for keeping buffers to output files.
pub struct OutputBuffers<T> {
    /// The base file which contains all uniparc sequences.
    uniparc: T,
    /// File containing all uniparc cross-references.
    uniparc_xref: T,
    /// Files for each of the uniparc cross-references.
    uniparc_xref2ncbi_gi: T,
    uniparc_xref2ncbi_taxonomy_id: T,
    uniparc_xref2protein_name: T,
    uniparc_xref2gene_name: T,
    uniparc_xref2chain: T,
    uniparc_xref2uniprot_kb_accession: T,
    uniparc_xref2proteome_id: T,
    uniparc_xref2component: T,
    //
    ncbi_gi: T,
    ncbi_taxonomy_id: T,
    protein_name: T,
    gene_name: T,
    chain: T,
    uniprot_kb_accession: T,
    proteome_id: T,
    component: T,
    // File containing domain definitions for UniParc sequences
    uniparc_domain: T,
}

/// Initialize all output buffers.
pub fn initialize_outputs(basedir: PathBuf) -> OutputBuffers<BufWriter<File>> {
    let create_outfile = |filename: &str| {
        let file = File::create(basedir.join(filename)).unwrap();
        BufWriter::new(file)
    };

    OutputBuffers {
        uniparc: create_outfile("uniparc.tsv"),
        uniparc_xref: create_outfile("uniparc_xref.tsv"),
        uniparc_xref2ncbi_gi: create_outfile("uniparc_xref2ncbi_gi.tsv"),
        uniparc_xref2ncbi_taxonomy_id: create_outfile("uniparc_xref2ncbi_taxonomy_id.tsv"),
        uniparc_xref2protein_name: create_outfile("uniparc_xref2protein_name.tsv"),
        uniparc_xref2gene_name: create_outfile("uniparc_xref2gene_name.tsv"),
        uniparc_xref2chain: create_outfile("uniparc_xref2chain.tsv"),
        uniparc_xref2uniprot_kb_accession: create_outfile("uniparc_xref2uniprot_kb_accession.tsv"),
        uniparc_xref2proteome_id: create_outfile("uniparc_xref2proteome_id.tsv"),
        uniparc_xref2component: create_outfile("uniparc_xref2component.tsv"),
        ncbi_gi: create_outfile("ncbi_gi.tsv"),
        ncbi_taxonomy_id: create_outfile("ncbi_taxonomy_id.tsv"),
        protein_name: create_outfile("protein_name.tsv"),
        gene_name: create_outfile("gene_name.tsv"),
        chain: create_outfile("chain.tsv"),
        uniprot_kb_accession: create_outfile("uniprot_kb_accession.tsv"),
        proteome_id: create_outfile("proteome_id.tsv"),
        component: create_outfile("component.tsv"),
        uniparc_domain: create_outfile("uniparc_domain.tsv"),
    }
}

pub fn initialize_outputs_compressed(basedir: PathBuf) -> OutputBuffers<GzEncoder<File>> {
    let create_outfile = |filename: &str| {
        let f = File::create(basedir.join(format!("{}{}", filename, ".gz"))).unwrap();
        GzBuilder::new()
            .filename(filename)
            .write(f, Compression::default())
    };

    OutputBuffers {
        uniparc: create_outfile("uniparc.tsv"),
        uniparc_xref: create_outfile("uniparc_xref.tsv"),
        uniparc_xref2ncbi_gi: create_outfile("uniparc_xref2ncbi_gi.tsv"),
        uniparc_xref2ncbi_taxonomy_id: create_outfile("uniparc_xref2ncbi_taxonomy_id.tsv"),
        uniparc_xref2protein_name: create_outfile("uniparc_xref2protein_name.tsv"),
        uniparc_xref2gene_name: create_outfile("uniparc_xref2gene_name.tsv"),
        uniparc_xref2chain: create_outfile("uniparc_xref2chain.tsv"),
        uniparc_xref2uniprot_kb_accession: create_outfile("uniparc_xref2uniprot_kb_accession.tsv"),
        uniparc_xref2proteome_id: create_outfile("uniparc_xref2proteome_id.tsv"),
        uniparc_xref2component: create_outfile("uniparc_xref2component.tsv"),
        ncbi_gi: create_outfile("ncbi_gi.tsv"),
        ncbi_taxonomy_id: create_outfile("ncbi_taxonomy_id.tsv"),
        protein_name: create_outfile("protein_name.tsv"),
        gene_name: create_outfile("gene_name.tsv"),
        chain: create_outfile("chain.tsv"),
        uniprot_kb_accession: create_outfile("uniprot_kb_accession.tsv"),
        proteome_id: create_outfile("proteome_id.tsv"),
        component: create_outfile("component.tsv"),
        uniparc_domain: create_outfile("uniparc_domain.tsv"),
    }
}

/// Trait which means that a struct can be serialized to CSV format.
pub trait Writable {
    fn to_csv<T: Write>(&self, output: &mut T);
}

impl Writable for Uniparc {
    fn to_csv<T: Write>(&self, output: &mut T) {
        write!(
            output,
            "{:?}\t{:?}\t{:?}\t{:?}\n",
            self.id, self.sequence, self.sequence_length, self.sequence_checksum
        )
        .unwrap();
    }
}

impl Writable for UniparcXRef {
    fn to_csv<T: Write>(&self, output: &mut T) {
        write!(
            output,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\n",
            self.uniparc_id,
            self.idx,
            self.db_type,
            self.db_id,
            self.version_i,
            self.active,
            self.version,
            self.created,
            self.last
        )
        .unwrap();
    }
}

impl Writable for UniparcXRef2Property {
    fn to_csv<T: Write>(&self, output: &mut T) {
        write!(
            output,
            "{:?}\t{:?}\t{:?}\t{:?}\n",
            self.uniparc_id, self.uniparc_xref_idx, self.property_name, self.property_idx,
        )
        .unwrap();
    }
}

impl Writable for UniparcProperty {
    fn to_csv<T: Write>(&self, output: &mut T) {
        write!(
            output,
            "{:?}\t{:?}\t{:?}\t{:?}\n",
            self.uniparc_id, self.name, self.idx, self.value,
        )
        .unwrap();
    }
}

impl Writable for UniparcDomain {
    fn to_csv<T: Write>(&self, output: &mut T) {
        write!(
            output,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\n",
            self.uniparc_id,
            self.database,
            self.database_id,
            self.interpro_name,
            self.interpro_id,
            self.domain_start,
            self.domain_end,
        )
        .unwrap();
    }
}

// Writers

fn sorted(hash: &HashMap<String, u64>) -> Vec<(&u64, &String)> {
    let mut list = Vec::new();
    for (key, value) in hash {
        list.push((value, key));
    }
    list.sort();
    list
}

pub fn write_uniparc<T: Write>(outputs: &mut OutputBuffers<T>, uniparc: &Uniparc) {
    uniparc.to_csv(&mut outputs.uniparc);
}

pub fn write_uniparc_xrefs<T: Write>(
    outputs: &mut OutputBuffers<T>,
    uniparc_xrefs: &Vec<UniparcXRef>,
) {
    for uniparc_xref in uniparc_xrefs {
        uniparc_xref.to_csv(&mut outputs.uniparc_xref);
    }
}

pub fn write_uniparc_domains<T: Write>(
    outputs: &mut OutputBuffers<T>,
    uniparc_domains: &Vec<UniparcDomain>,
) {
    for uniparc_domain in uniparc_domains {
        uniparc_domain.to_csv(&mut outputs.uniparc_domain);
    }
}

pub fn write_uniparc_xref2properties<T: Write>(
    headers: &mut OutputBuffers<T>,
    uniparc_xref2properties: &Properties<Vec<UniparcXRef2Property>>,
) {
    for xref2prop in &uniparc_xref2properties.ncbi_gi {
        (*xref2prop).to_csv(&mut headers.uniparc_xref2ncbi_gi);
    }
    for xref2prop in &uniparc_xref2properties.ncbi_taxonomy_id {
        (*xref2prop).to_csv(&mut headers.uniparc_xref2ncbi_taxonomy_id);
    }
    for xref2prop in &uniparc_xref2properties.protein_name {
        (*xref2prop).to_csv(&mut headers.uniparc_xref2protein_name);
    }
    for xref2prop in &uniparc_xref2properties.gene_name {
        (*xref2prop).to_csv(&mut headers.uniparc_xref2gene_name);
    }
    for xref2prop in &uniparc_xref2properties.chain {
        (*xref2prop).to_csv(&mut headers.uniparc_xref2chain);
    }
    for xref2prop in &uniparc_xref2properties.uniprot_kb_accession {
        (*xref2prop).to_csv(&mut headers.uniparc_xref2uniprot_kb_accession);
    }
    for xref2prop in &uniparc_xref2properties.proteome_id {
        (*xref2prop).to_csv(&mut headers.uniparc_xref2proteome_id);
    }
    for xref2prop in &uniparc_xref2properties.component {
        (*xref2prop).to_csv(&mut headers.uniparc_xref2component);
    }
}

pub fn write_uniparc_properties<T: Write>(
    headers: &mut OutputBuffers<T>,
    properties: &Properties<HashMap<String, u64>>,
    uniparc_id: String,
) {
    let write_property = |idx: &u64, value: &str, name: &str, output_stream: &mut T| {
        let property = UniparcProperty {
            uniparc_id: uniparc_id.clone(),
            name: name.to_string(),
            idx: *idx,
            value: value.to_string(),
        };
        property.to_csv(output_stream);
    };
    for (idx, value) in sorted(&properties.ncbi_gi) {
        write_property(idx, value, "ncbi_gi", &mut headers.ncbi_gi);
    }
    for (idx, value) in sorted(&properties.ncbi_taxonomy_id) {
        write_property(
            idx,
            value,
            "ncbi_taxonomy_id",
            &mut headers.ncbi_taxonomy_id,
        );
    }
    for (idx, value) in sorted(&properties.protein_name) {
        write_property(idx, value, "protein_name", &mut headers.protein_name);
    }
    for (idx, value) in sorted(&properties.gene_name) {
        write_property(idx, value, "gene_name", &mut headers.gene_name);
    }
    for (idx, value) in sorted(&properties.chain) {
        write_property(idx, value, "chain", &mut headers.chain);
    }
    for (idx, value) in sorted(&properties.uniprot_kb_accession) {
        write_property(
            idx,
            value,
            "uniprot_kb_accession",
            &mut headers.uniprot_kb_accession,
        );
    }
    for (idx, value) in sorted(&properties.proteome_id) {
        write_property(idx, value, "proteome_id", &mut headers.proteome_id);
    }
    for (idx, value) in sorted(&properties.component) {
        write_property(idx, value, "component", &mut headers.component);
    }
}

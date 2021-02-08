use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use flate2::write::GzEncoder;
use flate2::{Compression, GzBuilder};

use model::{Uniparc, UniparcDomain, UniparcProperty, UniparcXRef};
use properties::Properties;

/// Struct for keeping buffers to output files.
pub struct OutputBuffers<T> {
    /// The base file which contains all uniparc sequences
    uniparc: T,
    /// File containing domain definitions for UniParc sequences
    domain: T,
    /// File containing all uniparc cross-references
    xref: T,
    /// Files for each of the uniparc cross-references.
    component: T,
    gene_name: T,
    ncbi_gi: T,
    ncbi_taxonomy_id: T,
    pdb_chain: T,
    protein_name: T,
    proteome_id: T,
    uniprot_kb_accession: T,
}

/// Initialize all output buffers
pub fn initialize_outputs(basedir: PathBuf) -> OutputBuffers<BufWriter<File>> {
    let create_outfile = |filename: &str| {
        let file = File::create(basedir.join(filename)).unwrap();
        BufWriter::new(file)
    };

    OutputBuffers {
        uniparc: create_outfile("uniparc.tsv"),
        domain: create_outfile("domain.tsv"),
        xref: create_outfile("xref.tsv"),
        component: create_outfile("component.tsv"),
        gene_name: create_outfile("gene_name.tsv"),
        ncbi_gi: create_outfile("ncbi_gi.tsv"),
        ncbi_taxonomy_id: create_outfile("ncbi_taxonomy_id.tsv"),
        pdb_chain: create_outfile("pdb_chain.tsv"),
        protein_name: create_outfile("protein_name.tsv"),
        proteome_id: create_outfile("proteome_id.tsv"),
        uniprot_kb_accession: create_outfile("uniprot_kb_accession.tsv"),
    }
}

// Initialize compressed output buffers
pub fn initialize_outputs_compressed(basedir: PathBuf) -> OutputBuffers<GzEncoder<File>> {
    let create_outfile = |filename: &str| {
        let f = File::create(basedir.join(format!("{}{}", filename, ".gz"))).unwrap();
        GzBuilder::new()
            .filename(filename)
            .write(f, Compression::default())
    };

    OutputBuffers {
        uniparc: create_outfile("uniparc.tsv"),
        domain: create_outfile("domain.tsv"),
        xref: create_outfile("xref.tsv"),
        component: create_outfile("component.tsv"),
        gene_name: create_outfile("gene_name.tsv"),
        ncbi_gi: create_outfile("ncbi_gi.tsv"),
        ncbi_taxonomy_id: create_outfile("ncbi_taxonomy_id.tsv"),
        pdb_chain: create_outfile("pdb_chain.tsv"),
        protein_name: create_outfile("protein_name.tsv"),
        proteome_id: create_outfile("proteome_id.tsv"),
        uniprot_kb_accession: create_outfile("uniprot_kb_accession.tsv"),
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
            self.uniparc_id, self.sequence, self.sequence_length, self.sequence_checksum
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

impl Writable for UniparcXRef {
    fn to_csv<T: Write>(&self, output: &mut T) {
        write!(
            output,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\n",
            self.uniparc_id,
            self.xref_id,
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

impl Writable for UniparcProperty {
    fn to_csv<T: Write>(&self, output: &mut T) {
        write!(
            output,
            "{:?}\t{:?}\t{:?}\t{:?}\n",
            self.uniparc_id, self.xref_id, self.property, self.value,
        )
        .unwrap();
    }
}

// Writers
pub fn write_uniparc<T: Write>(outputs: &mut OutputBuffers<T>, uniparc: &Uniparc) {
    uniparc.to_csv(&mut outputs.uniparc);
}

pub fn write_uniparc_domains<T: Write>(
    outputs: &mut OutputBuffers<T>,
    uniparc_domains: &Vec<UniparcDomain>,
) {
    for uniparc_domain in uniparc_domains {
        uniparc_domain.to_csv(&mut outputs.domain);
    }
}

pub fn write_uniparc_xrefs<T: Write>(
    outputs: &mut OutputBuffers<T>,
    uniparc_xrefs: &Vec<UniparcXRef>,
) {
    for uniparc_xref in uniparc_xrefs {
        uniparc_xref.to_csv(&mut outputs.xref);
    }
}

pub fn write_uniparc_properties<T: Write>(
    headers: &mut OutputBuffers<T>,
    properties: &Properties<Vec<UniparcProperty>>,
) {
    for prop in properties.component.iter() {
        prop.to_csv(&mut headers.component);
    }

    for prop in properties.gene_name.iter() {
        prop.to_csv(&mut headers.gene_name);
    }

    for prop in properties.ncbi_gi.iter() {
        prop.to_csv(&mut headers.ncbi_gi);
    }

    for prop in properties.ncbi_taxonomy_id.iter() {
        prop.to_csv(&mut headers.ncbi_taxonomy_id);
    }

    for prop in properties.pdb_chain.iter() {
        prop.to_csv(&mut headers.pdb_chain);
    }

    for prop in properties.protein_name.iter() {
        prop.to_csv(&mut headers.protein_name);
    }

    for prop in properties.proteome_id.iter() {
        prop.to_csv(&mut headers.proteome_id);
    }

    for prop in properties.uniprot_kb_accession.iter() {
        prop.to_csv(&mut headers.uniprot_kb_accession);
    }
}

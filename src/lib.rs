//! This is documentation for the `uniparc_xml_parser` crate.
//!
//!
extern crate xml;

use std::io;
use std::io::{BufReader, BufWriter, Stdin};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use std::collections::HashMap;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;


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
}

/// Initialize all output files.
fn initialize_outputs(basedir: &str) -> OutputBuffers<BufWriter<File>> {
    let create_outfile = |filename: &str| {
        io::BufWriter::new(File::create(PathBuf::from(basedir).join(filename)).unwrap())
    };

    OutputBuffers {
        uniparc: create_outfile("uniparc.tsv"),
        uniparc_xref: create_outfile("uniparc_xref.tsv"),
        uniparc_xref2ncbi_gi: create_outfile("_uniparc_xref2ncbi_gi.tsv"),
        uniparc_xref2ncbi_taxonomy_id: create_outfile("_uniparc_xref2ncbi_taxonomy_id.tsv"),
        uniparc_xref2protein_name: create_outfile("_uniparc_xref2protein_name.tsv"),
        uniparc_xref2gene_name: create_outfile("_uniparc_xref2gene_name.tsv"),
        uniparc_xref2chain: create_outfile("_uniparc_xref2chain.tsv"),
        uniparc_xref2uniprot_kb_accession: create_outfile("_uniparc_xref2uniprot_kb_accession.tsv"),
        uniparc_xref2proteome_id: create_outfile("_uniparc_xref2proteome_id.tsv"),
        uniparc_xref2component: create_outfile("_uniparc_xref2component.tsv"),
        ncbi_gi: create_outfile("_ncbi_gi.tsv"),
        ncbi_taxonomy_id: create_outfile("_ncbi_taxonomy_id.tsv"),
        protein_name: create_outfile("_protein_name.tsv"),
        gene_name: create_outfile("_gene_name.tsv"),
        chain: create_outfile("_chain.tsv"),
        uniprot_kb_accession: create_outfile("_uniprot_kb_accession.tsv"),
        proteome_id: create_outfile("_proteome_id.tsv"),
        component: create_outfile("_component.tsv"),
    }
}


/// Keep all properties for a given UniParc ID.
struct Properties<T> {
    ncbi_gi: T,
    ncbi_taxonomy_id: T,
    protein_name: T,
    gene_name: T,
    chain: T,
    uniprot_kb_accession: T,
    proteome_id: T,
    component: T,
}

impl Default for Properties<HashMap<String, i64>> {
    fn default() -> Properties<HashMap<String, i64>> {
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


/// Trait which means that a struct can be serialized to CSV format.
pub trait Writable {
    fn to_csv(&self, output: &mut BufWriter<File>);
}

#[derive(Default)]
struct Uniparc {
    id: String,
    sequence: String,
    sequence_length: String,
    sequence_checksum: String,
}

impl Writable for Uniparc {
    fn to_csv(&self,output:  &mut io::BufWriter<File>) {
        writeln!(
            output,
            "{}\t{}\t{}\t{}\n",
            self.id,
            self.sequence,
            self.sequence_length,
            self.sequence_checksum
        ).unwrap();
    }
}

struct UniparcXRef {
    uniparc_id: String,
    idx: i64,
    db_type: String,
    db_id: String,
    version_i: String,
    active: String,
    version: String,
    created: String,
    last: String,
}

impl Writable for UniparcXRef {
    fn to_csv(&self, output: &mut io::BufWriter<File>) {
        writeln!(
            output,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            self.uniparc_id,
            self.idx,
            self.db_type,
            self.db_id,
            self.version_i,
            self.active,
            self.version,
            self.created,
            self.last
        ).unwrap();
    }
}

struct UniparcXRef2Property {
    uniparc_id: String,
    uniparc_xref_idx: i64,
    property_idx: i64,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Property {
    uniparc_id: String,
    idx: i64,
    xtype: String,
    value: String,
}

#[derive(Clone)]
struct UniparcDomain {
    // Filled in by `add_signature_sequence_match`
    uniparc_id: String,
    database: String,
    database_id: String,
    // Filled in by `add_interpro_annotation`
    interpro_name: String,
    interpro_id: String,
    // Filled in by `add_domain_definitions`
    domain_start: u32,
    domain_end: u32,
}

impl Writable for UniparcDomain {
    fn to_csv(&self, output: &mut BufWriter<File>) {
        writeln!(
            output,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            self.uniparc_id,
            self.database,
            self.database_id,
            self.interpro_name,
            self.interpro_id,
            self.domain_start,
            self.domain_end,
        ).unwrap();
    }
}


/// Add new data
fn add_uniparc_xref(
    uniparc_id: String,
    uniparc_xrefs: &mut Vec<UniparcXRef>,
    attributes: Vec<OwnedAttribute>,
) -> bool {
    let mut uniparc_xref = UniparcXRef {
        uniparc_id: uniparc_id,
        idx: (uniparc_xrefs.len() + 1) as i64,
        db_type: String::new(),
        db_id: String::new(),
        version_i: String::new(),
        active: String::new(),
        version: String::new(),
        created: String::new(),
        last: String::new(),
    };
    for attribute in attributes {
        match attribute.name.local_name.as_ref() {
            "type" => {
                uniparc_xref.db_type = attribute.value;
            }
            "id" => {
                uniparc_xref.db_id = attribute.value;
            }
            "version_i" => {
                uniparc_xref.version_i = attribute.value;
            }
            "active" => {
                uniparc_xref.active = attribute.value;
            }
            "version" => {
                uniparc_xref.version = attribute.value;
            }
            "created" => {
                uniparc_xref.created = attribute.value;
            }
            "last" => {
                uniparc_xref.last = attribute.value;
            }
            _ => {
                println!("Skipping attribute '{:?}' for dbReference.", attribute);
            }
        }
    }
    if uniparc_xref.active == "Y" {
        uniparc_xrefs.push(uniparc_xref);
        return true;
    }
    false
}

fn add_property(
    uniparc_id: String,
    uniparc_xrefs: &Vec<UniparcXRef>,
    properties: &mut Properties<HashMap<String, i64>>,
    uniparc_xref2properties: &mut Properties<Vec<UniparcXRef2Property>>,
    attributes: Vec<OwnedAttribute>,
) {
    let (attr_type, mut attr_value) = (attributes[0].value.clone(), attributes[1].value.clone());

    let uniparc_xref_idx = uniparc_xrefs.len() as i64;
    if attr_type == "chain" {
        assert!(uniparc_xrefs.last().unwrap().db_type == "PDB");
        attr_value = uniparc_xrefs.last().unwrap().db_id.clone() + &attr_value;
    }

    let (property, uniparc_xref2property) = match attr_type.as_ref() {
        "NCBI_GI" => (
            &mut properties.ncbi_gi,
            &mut uniparc_xref2properties.ncbi_gi,
        ),
        "NCBI_taxonomy_id" => (
            &mut properties.ncbi_taxonomy_id,
            &mut uniparc_xref2properties.ncbi_taxonomy_id,
        ),
        "protein_name" => (
            &mut properties.protein_name,
            &mut uniparc_xref2properties.protein_name,
        ),
        "gene_name" => (
            &mut properties.gene_name,
            &mut uniparc_xref2properties.gene_name,
        ),
        "chain" => (&mut properties.chain, &mut uniparc_xref2properties.chain),
        "UniProtKB_accession" => (
            &mut properties.uniprot_kb_accession,
            &mut uniparc_xref2properties.uniprot_kb_accession,
        ),
        "proteome_id" => (
            &mut properties.proteome_id,
            &mut uniparc_xref2properties.proteome_id,
        ),
        "component" => (
            &mut properties.component,
            &mut uniparc_xref2properties.component,
        ),
        _ => panic!("Unmatched value: '{}'.", attr_type),
    };

    let property_idx: i64;
    if !property.contains_key(&attr_value) {
        property_idx = (property.len() + 1) as i64;
        property.insert(attr_value, property_idx);
    } else {
        property_idx = *property.get(&attr_value).unwrap();
    }

    uniparc_xref2property.push(UniparcXRef2Property {
        uniparc_id: uniparc_id,
        uniparc_xref_idx: uniparc_xref_idx,
        property_idx: property_idx,
    });
}

fn add_signature_sequence_match(
    uniparc_id: String,
    uniparc_domains: &mut Vec<UniparcDomain>,
    attributes: Vec<OwnedAttribute>,
) {
    let mut database = String::new();
    let mut database_id = String::new();
    for attribute in attributes {
        match attribute.name.local_name.as_ref() {
            "database" => {
                database = attribute.value;
            }
            "id" => {
                database_id = attribute.value;
            }
            _ => panic!("Unmatched value: '{}'.", attribute.name),
        }
    }
    let uniparc_domain = UniparcDomain {
        uniparc_id,
        database,
        database_id,
        interpro_name: String::new(),
        interpro_id: String::new(),
        domain_start: 0,
        domain_end: 0,
    };
    uniparc_domains.push(uniparc_domain);
}


fn add_interpro_annotation(
    uniparc_id: String,
    uniparc_domains: &mut Vec<UniparcDomain>,
    attributes: Vec<OwnedAttribute>,
) {
    let mut interpro_name = String::new();
    let mut interpro_id = String::new();
    for attribute in attributes {
        match attribute.name.local_name.as_ref() {
            "name" => {
                interpro_name = attribute.value;
            }
            "id" => {
                interpro_id = attribute.value;
            }
            _ => panic!("Unmatched value: '{}'.", attribute.name),
        }
    }
    let mut uniparc_domain = uniparc_domains.pop().unwrap();
    assert!(uniparc_domain.uniparc_id == uniparc_id);
    assert!(uniparc_domain.interpro_name == "");
    assert!(interpro_name != "");
    assert!(uniparc_domain.interpro_id == "");
    assert!(interpro_name != "");
    uniparc_domain.interpro_name = interpro_name;
    uniparc_domain.interpro_id = interpro_id;
    uniparc_domains.push(uniparc_domain);
}

fn add_domain_definitions(
    uniparc_id: String,
    uniparc_domains: &mut Vec<UniparcDomain>,
    attributes: Vec<OwnedAttribute>,
) {
    let mut domain_start: u32 = 0;
    let mut domain_end: u32 = 0;
    for attribute in attributes {
        match attribute.name.local_name.as_ref() {
            "name" => domain_start = attribute.value.parse::<u32>().unwrap(),
            "id" => domain_end = attribute.value.parse::<u32>().unwrap(),
            _ => panic!("Unmatched value: '{}'.", attribute.name),
        }
    }
    let mut uniparc_domain = uniparc_domains.pop().unwrap();
    assert!(uniparc_domain.uniparc_id == uniparc_id);
    assert!(domain_start != 0);
    assert!(domain_end != 0);
    if (uniparc_domain.domain_start == 0) && (uniparc_domain.domain_end == 0) {
        uniparc_domain.domain_start = domain_start;
        uniparc_domain.domain_end = domain_end;
        uniparc_domains.push(uniparc_domain);
    } else {
        let uniparc_domain_bak = uniparc_domain.clone();
        uniparc_domain.domain_start = domain_start;
        uniparc_domain.domain_end = domain_end;
        uniparc_domains.push(uniparc_domain_bak);
        uniparc_domains.push(uniparc_domain);
    }
}


fn add_sequence(uniparc: &mut Uniparc, attributes: Vec<OwnedAttribute>) {
    for attribute in attributes {
        match attribute.name.local_name.as_ref() {
            "length" => {
                uniparc.sequence_length = attribute.value;
            }
            "checksum" => {
                uniparc.sequence_checksum = attribute.value;
            }
            _ => {
                println!("Skipping attribute '{:?}' for sequence.", attribute);
            }
        }
    }
}


/// Main loop
pub fn run(input_stream: Stdin) -> Result<usize, Box<Error>> {
    let parser = EventReader::new(BufReader::new(input_stream));

    let mut handlers = initialize_outputs(".");

    // Variables created for each UniParc ID
    let mut uniparc: Uniparc = Default::default();
    let mut uniparc_xrefs: Vec<UniparcXRef> = Vec::new();
    let mut uniparc_domains: Vec<UniparcDomain> = Vec::new();
    let mut properties: Properties<HashMap<String, i64>> = Default::default();
    let mut uniparc_xref2properties: Properties<Vec<UniparcXRef2Property>> = Default::default();

    let mut keep_uniparc_xref = true;
    let mut current_element = Vec::new();

    // Number of UniParc entries that are in the proces of being processed.
    // When we start processing an entry, we *increment* `depth` by 1.
    // When we finish processing an entry, we *decrement* `depth` by 1.
    let mut depth = 0;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                match name.local_name.as_ref() {
                    "entry" => {
                        uniparc = Default::default();
                        uniparc_xrefs = Vec::new();
                        properties = Default::default();
                        uniparc_xref2properties = Default::default();
                    }
                    "dbReference" => {
                        keep_uniparc_xref =
                            add_uniparc_xref(uniparc.id.clone(), &mut uniparc_xrefs, attributes);
                    }
                    "property" => if keep_uniparc_xref {
                        add_property(
                            uniparc.id.clone(),
                            &uniparc_xrefs,
                            &mut properties,
                            &mut uniparc_xref2properties,
                            attributes,
                        );
                    },
                    "signatureSequenceMatch" => add_signature_sequence_match(
                        uniparc.id.clone(),
                        &mut uniparc_domains,
                        attributes,
                    ),
                    "ipr" => add_interpro_annotation(
                        uniparc.id.clone(),
                        &mut uniparc_domains,
                        attributes,
                    ),
                    "lcn" => {
                        add_domain_definitions(uniparc.id.clone(), &mut uniparc_domains, attributes)
                    }
                    "sequence" => {
                        add_sequence(&mut uniparc, attributes);
                    }
                    "accession" => {
                        // This is where we get the uniparc id from the character field.
                    }
                    _ => {
                        println!(
                            "Skipping StartElement '{}' with attributes {:?}.",
                            name.local_name,
                            attributes
                        );
                    }
                }
                current_element.push(name.local_name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_ref() {
                    "entry" => {
                        println!(
                            "Saving entry with id: '{}' and sequence: '{}'",
                            uniparc.id,
                            uniparc.sequence
                        );
                        write_entry(&mut handlers, &uniparc, &uniparc_xrefs);
                        write_properties(&mut handlers, &properties, uniparc.id.clone());
                        write_uniparc_xref2properties(&mut handlers, &uniparc_xref2properties);
                    }
                    _ => {}
                }
                assert!(current_element.pop().unwrap().as_ref() == name.local_name);
                depth -= 1;
            }
            Ok(XmlEvent::CData(cdata)) => {
                println!("Skipping CData '{}'.", cdata);
            }
            Ok(XmlEvent::Comment(comment)) => {
                println!("Skipping Comment: '{}'", comment);
            }
            Ok(XmlEvent::Characters(characters)) => {
                match current_element.last().unwrap().as_ref() {
                    "accession" => {
                        uniparc.id = String::from(characters.trim());
                    }
                    "sequence" => {
                        uniparc.sequence = characters.lines().collect();
                    }
                    _ => {
                        println!("Skipping Characters: '{}'", characters);
                    }
                }
            }
            Ok(XmlEvent::Whitespace(whitespace)) => {
                let whitespace = String::from(whitespace.trim());
                if !whitespace.is_empty() {
                    println!("Skipping Whitespace: '{}'", whitespace);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {
                println!("Error: did not match e: '{:?}'.", e);
            }
        }
    }
    println!("Depth: {}", depth);
    assert!(depth == 0);
    Ok(depth)
}

// Write output
fn write_entry(
    outputs: &mut OutputBuffers<BufWriter<File>>,
    uniparc: &Uniparc,
    uniparc_xrefs: &Vec<UniparcXRef>,
) {
    uniparc.to_csv(&mut outputs.uniparc);
    for uniparc_xref in uniparc_xrefs {
        uniparc_xref.to_csv(&mut outputs.uniparc_xref);
    }
}

fn sorted(hash: &HashMap<String, i64>) -> Vec<(&i64, &String)> {
    let mut list = Vec::new();
    for (key, value) in hash {
        list.push((value, key));
    }
    list.sort();
    list
}

fn write_properties(
    headers: &mut OutputBuffers<io::BufWriter<File>>,
    properties: &Properties<HashMap<String, i64>>,
    uniparc_id: String,
) {
    for (idx, value) in sorted(&properties.ncbi_gi) {
        let _ = writeln!(
            headers.ncbi_gi,
            "{}\t{}\t{}\t{}",
            uniparc_id,
            "ncbi_gi",
            idx,
            value
        );
    }
    for (idx, value) in sorted(&properties.ncbi_taxonomy_id) {
        let _ = writeln!(
            headers.ncbi_taxonomy_id,
            "{}\t{}\t{}\t{}",
            uniparc_id,
            "ncbi_taxonomy_id",
            idx,
            value
        );
    }
    for (idx, value) in sorted(&properties.protein_name) {
        let _ = writeln!(
            headers.protein_name,
            "{}\t{}\t{}\t{}",
            uniparc_id,
            "protein_name",
            idx,
            value
        );
    }
    for (idx, value) in sorted(&properties.gene_name) {
        let _ = writeln!(
            headers.gene_name,
            "{}\t{}\t{}\t{}",
            uniparc_id,
            "gene_name",
            idx,
            value
        );
    }
    for (idx, value) in sorted(&properties.chain) {
        let _ = writeln!(
            headers.chain,
            "{}\t{}\t{}\t{}",
            uniparc_id,
            "chain",
            idx,
            value
        );
    }
    for (idx, value) in sorted(&properties.uniprot_kb_accession) {
        let _ = writeln!(
            headers.uniprot_kb_accession,
            "{}\t{}\t{}\t{}",
            uniparc_id,
            "uniprot_kb_accession",
            idx,
            value
        );
    }
    for (idx, value) in sorted(&properties.proteome_id) {
        let _ = writeln!(
            headers.proteome_id,
            "{}\t{}\t{}\t{}",
            uniparc_id,
            "proteome_id",
            idx,
            value
        );
    }
    for (idx, value) in sorted(&properties.component) {
        let _ = writeln!(
            headers.component,
            "{}\t{}\t{}\t{}",
            uniparc_id,
            "component",
            idx,
            value
        );
    }
}

fn write_uniparc_xref2properties(
    headers: &mut OutputBuffers<io::BufWriter<File>>,
    uniparc_xref2properties: &Properties<Vec<UniparcXRef2Property>>,
) {
    for uniparc_xref2property in &uniparc_xref2properties.ncbi_gi {
        let _ = writeln!(
            headers.uniparc_xref2ncbi_gi,
            "{}\t{}\t{}\t{}",
            uniparc_xref2property.uniparc_id,
            "ncbi_gi",
            uniparc_xref2property.uniparc_xref_idx,
            uniparc_xref2property.property_idx
        );
    }
    for uniparc_xref2property in &uniparc_xref2properties.ncbi_taxonomy_id {
        let _ = writeln!(
            headers.uniparc_xref2ncbi_taxonomy_id,
            "{}\t{}\t{}\t{}",
            uniparc_xref2property.uniparc_id,
            "ncbi_taxonomy_id",
            uniparc_xref2property.uniparc_xref_idx,
            uniparc_xref2property.property_idx
        );
    }
    for uniparc_xref2property in &uniparc_xref2properties.protein_name {
        let _ = writeln!(
            headers.uniparc_xref2protein_name,
            "{}\t{}\t{}\t{}",
            uniparc_xref2property.uniparc_id,
            "protein_name",
            uniparc_xref2property.uniparc_xref_idx,
            uniparc_xref2property.property_idx
        );
    }
    for uniparc_xref2property in &uniparc_xref2properties.gene_name {
        let _ = writeln!(
            headers.uniparc_xref2gene_name,
            "{}\t{}\t{}\t{}",
            uniparc_xref2property.uniparc_id,
            "gene_name",
            uniparc_xref2property.uniparc_xref_idx,
            uniparc_xref2property.property_idx
        );
    }
    for uniparc_xref2property in &uniparc_xref2properties.chain {
        let _ = writeln!(
            headers.uniparc_xref2chain,
            "{}\t{}\t{}\t{}",
            uniparc_xref2property.uniparc_id,
            "chain",
            uniparc_xref2property.uniparc_xref_idx,
            uniparc_xref2property.property_idx
        );
    }
    for uniparc_xref2property in &uniparc_xref2properties.uniprot_kb_accession {
        let _ = writeln!(
            headers.uniparc_xref2uniprot_kb_accession,
            "{}\t{}\t{}\t{}",
            uniparc_xref2property.uniparc_id,
            "uniprot_kb_accession",
            uniparc_xref2property.uniparc_xref_idx,
            uniparc_xref2property.property_idx
        );
    }
    for uniparc_xref2property in &uniparc_xref2properties.proteome_id {
        let _ = writeln!(
            headers.uniparc_xref2proteome_id,
            "{}\t{}\t{}\t{}",
            uniparc_xref2property.uniparc_id,
            "proteome_id",
            uniparc_xref2property.uniparc_xref_idx,
            uniparc_xref2property.property_idx
        );
    }
    for uniparc_xref2property in &uniparc_xref2properties.component {
        let _ = writeln!(
            headers.uniparc_xref2component,
            "{}\t{}\t{}\t{}",
            uniparc_xref2property.uniparc_id,
            "component",
            uniparc_xref2property.uniparc_xref_idx,
            uniparc_xref2property.property_idx
        );
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        outputs = Outputs {
            //
            uniparc: vec!['a'],
            //
            uniparc_xref: vec!['a'],
            //
            uniparc_xref2ncbi_gi: vec!['a'],
            uniparc_xref2ncbi_taxonomy_id: vec!['a'],
            uniparc_xref2protein_name: vec!['a'],
            uniparc_xref2gene_name: vec!['a'],
            uniparc_xref2chain: vec!['a'],
            uniparc_xref2uniprot_kb_accession: vec!['a'],
            uniparc_xref2proteome_id: vec!['a'],
            uniparc_xref2component: vec!['a'],
            //
            ncbi_gi: vec!['a'],
            ncbi_taxonomy_id: vec!['a'],
            protein_name: vec!['a'],
            gene_name: vec!['a'],
            chain: vec!['a'],
            uniprot_kb_accession: vec!['a'],
            proteome_id: vec!['a'],
            component: vec!['a'],
        }
    }
}

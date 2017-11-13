extern crate xml;

use std::io;
use std::io::{BufReader, Stdin};
use std::error;
use std::fs::File;
use std::io::Write;
use std::process;

use std::collections::HashMap;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

type Error = Box<error::Error + Send + Sync>;

struct Outputs<T> {
    //
    uniparc: T,
    //
    uniparc_xref: T,
    //
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

struct Uniparc {
    id: String,
    sequence: String,
    sequence_length: String,
    sequence_checksum: String,
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

#[derive(Hash, Eq, PartialEq, Debug)]
struct Property {
    uniparc_id: String,
    idx: i64,
    xtype: String,
    value: String,
}

struct UniparcXRef2Property {
    uniparc_id: String,
    uniparc_xref_idx: i64,
    property_idx: i64,
}

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

// struct SignatureSequenceMatch {
//     uniparc_id: String,
//     interpro_id: String,
//     domain_start: i32,
//     domain_end: i32,
// }


// Get empties
fn get_handlers() -> Outputs<io::BufWriter<File>> {
    Outputs {
        //
        uniparc: io::BufWriter::new(File::create("uniparc.tsv").unwrap()),
        //
        uniparc_xref: io::BufWriter::new(File::create("uniparc_xref.tsv").unwrap()),
        //
        uniparc_xref2ncbi_gi: io::BufWriter::new(
            File::create("_uniparc_xref2ncbi_gi.tsv").unwrap(),
        ),
        uniparc_xref2ncbi_taxonomy_id: io::BufWriter::new(
            File::create("_uniparc_xref2ncbi_taxonomy_id.tsv").unwrap(),
        ),
        uniparc_xref2protein_name: io::BufWriter::new(
            File::create("_uniparc_xref2protein_name.tsv").unwrap(),
        ),
        uniparc_xref2gene_name: io::BufWriter::new(
            File::create("_uniparc_xref2gene_name.tsv").unwrap(),
        ),
        uniparc_xref2chain: io::BufWriter::new(File::create("_uniparc_xref2chain.tsv").unwrap()),
        uniparc_xref2uniprot_kb_accession: io::BufWriter::new(
            File::create("_uniparc_xref2uniprot_kb_accession.tsv").unwrap(),
        ),
        uniparc_xref2proteome_id: io::BufWriter::new(
            File::create("_uniparc_xref2proteome_id.tsv").unwrap(),
        ),
        uniparc_xref2component: io::BufWriter::new(
            File::create("_uniparc_xref2component.tsv").unwrap(),
        ),
        //
        ncbi_gi: io::BufWriter::new(File::create("_ncbi_gi.tsv").unwrap()),
        ncbi_taxonomy_id: io::BufWriter::new(File::create("_ncbi_taxonomy_id.tsv").unwrap()),
        protein_name: io::BufWriter::new(File::create("_protein_name.tsv").unwrap()),
        gene_name: io::BufWriter::new(File::create("_gene_name.tsv").unwrap()),
        chain: io::BufWriter::new(File::create("_chain.tsv").unwrap()),
        uniprot_kb_accession: io::BufWriter::new(
            File::create("_uniprot_kb_accession.tsv").unwrap(),
        ),
        proteome_id: io::BufWriter::new(File::create("_proteome_id.tsv").unwrap()),
        component: io::BufWriter::new(File::create("_component.tsv").unwrap()),
        //
 






    }
}

fn get_uniparc() -> Uniparc {
    Uniparc {
        id: String::new(),
        sequence: String::new(),
        sequence_length: String::new(),
        sequence_checksum: String::new(),
    }
}

fn get_properties() -> Properties<HashMap<String, i64>> {
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

fn get_uniparc_xrefs2properties() -> Properties<Vec<UniparcXRef2Property>> {
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

// Add new data
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

// Write output
fn write_entry(
    handlers: &mut Outputs<io::BufWriter<File>>,
    uniparc: &Uniparc,
    uniparc_xrefs: &Vec<UniparcXRef>,
) {
    // uniparc
    let _ = writeln!(
        handlers.uniparc,
        "{}\t{}\t{}\t{}",
        uniparc.id,
        uniparc.sequence,
        uniparc.sequence_length,
        uniparc.sequence_checksum
    );
    // uniparc_xref
    for uniparc_xref in uniparc_xrefs {
        let _ = writeln!(
            handlers.uniparc_xref,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            uniparc_xref.uniparc_id,
            uniparc_xref.idx,
            uniparc_xref.db_type,
            uniparc_xref.db_id,
            uniparc_xref.version_i,
            uniparc_xref.active,
            uniparc_xref.version,
            uniparc_xref.created,
            uniparc_xref.last
        );
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
    headers: &mut Outputs<io::BufWriter<File>>,
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
    headers: &mut Outputs<io::BufWriter<File>>,
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

/// Main loop
fn run(parser: EventReader<BufReader<Stdin>>) -> Result<usize, Error> {
    let mut handlers = get_handlers();

    let mut uniparc = get_uniparc();
    let mut uniparc_xrefs: Vec<UniparcXRef> = Vec::new();
    let mut properties = get_properties();
    let mut uniparc_xref2properties = get_uniparc_xrefs2properties();

    let mut keep_uniparc_xref = true;
    let mut current_element = Vec::new();
    let mut depth = 0;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                match name.local_name.as_ref() {
                    "entry" => {
                        uniparc = get_uniparc();
                        uniparc_xrefs = Vec::new();
                        properties = get_properties();
                        uniparc_xref2properties = get_uniparc_xrefs2properties();
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
                    "sequence" => {
                        add_sequence(&mut uniparc, attributes);
                    }
                    "accession" => {
                        // This is where we get the uniparc id from the character field.
                    }
                    // "signatureSequenceMatch" => add_domains(&mut uniparc, attributes),
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

fn main() {
    let parser = EventReader::new(BufReader::new(io::stdin()));
    match run(parser) {
        Ok(count) => println!("{}", count),
        Err(err) => {
            let _ = writeln!(&mut io::stderr(), "{}", err);
            process::exit(1);
        }
    }
}

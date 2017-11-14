//! This is documentation for the `uniparc_xml_parser` crate.
//!
//!
extern crate flate2;
extern crate xml;

mod model;
mod properties;
mod writer;

use std::io::{BufReader, Stdin};
use std::error::Error;
use std::path::PathBuf;
use std::collections::HashMap;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

use model::{Uniparc, UniparcDomain, UniparcXRef, UniparcXRef2Property};
use properties::Properties;
use writer::{initialize_outputs, write_uniparc, write_uniparc_domains, write_uniparc_properties,
             write_uniparc_xrefs, write_uniparc_xref2properties};


/// Add new data
fn add_uniparc_xref(
    uniparc_id: String,
    uniparc_xrefs: &mut Vec<UniparcXRef>,
    attributes: Vec<OwnedAttribute>,
) -> bool {
    let mut uniparc_xref = UniparcXRef {
        uniparc_id: uniparc_id,
        idx: (uniparc_xrefs.len() + 1) as u64,
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
    properties: &mut Properties<HashMap<String, u64>>,
    uniparc_xref2properties: &mut Properties<Vec<UniparcXRef2Property>>,
    attributes: Vec<OwnedAttribute>,
) {
    let (attr_type, mut attr_value) = (attributes[0].value.clone(), attributes[1].value.clone());

    let uniparc_xref_idx = uniparc_xrefs.len() as u64;
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

    let property_idx: u64;
    if !property.contains_key(&attr_value) {
        property_idx = (property.len() + 1) as u64;
        property.insert(attr_value, property_idx);
    } else {
        property_idx = *property.get(&attr_value).unwrap();
    }

    uniparc_xref2property.push(UniparcXRef2Property {
        uniparc_id,
        uniparc_xref_idx,
        property_name: String::from(attr_type).to_ascii_lowercase(),
        property_idx,
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
    assert!(uniparc_domain.interpro_name == "");
    assert!(interpro_name != "");
    assert!(uniparc_domain.interpro_id == "");
    assert!(interpro_name != "");
    uniparc_domain.interpro_name = interpro_name;
    uniparc_domain.interpro_id = interpro_id;
    uniparc_domains.push(uniparc_domain);
}

fn add_domain_definitions(
    uniparc_domains: &mut Vec<UniparcDomain>,
    attributes: Vec<OwnedAttribute>,
) {
    let mut domain_start: u32 = 0;
    let mut domain_end: u32 = 0;
    for attribute in attributes {
        match attribute.name.local_name.as_ref() {
            "start" => domain_start = attribute.value.parse::<u32>().unwrap(),
            "end" => domain_end = attribute.value.parse::<u32>().unwrap(),
            _ => panic!("Unmatched value: '{}'.", attribute.name),
        }
    }
    let mut uniparc_domain = uniparc_domains.pop().unwrap();
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
                uniparc.sequence_length = attribute.value.parse::<u32>().unwrap();
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
pub fn run(input_stream: Stdin, basedir: PathBuf) -> Result<usize, Box<Error>> {
    let parser = EventReader::new(BufReader::new(input_stream));

    let mut handlers = initialize_outputs(basedir);

    // Variables created for each UniParc ID
    let mut uniparc: Uniparc = Default::default();
    let mut uniparc_xrefs: Vec<UniparcXRef> = Vec::new();
    let mut uniparc_domains: Vec<UniparcDomain> = Vec::new();
    let mut properties: Properties<HashMap<String, u64>> = Default::default();
    let mut uniparc_xref2properties: Properties<Vec<UniparcXRef2Property>> = Default::default();

    let mut keep_uniparc_xref = true;
    let mut current_element = Vec::new();

    // Number of UniParc entries that are in the proces of being processed.
    // When we start processing an entry, we *increment* `depth` by 1.
    // When we finish processing an entry, we *decrement* `depth` by 1.
    let mut depth = 0;
    // The number of UniParc sequences that have been processed.
    let mut count = 0;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                match name.local_name.as_ref() {
                    "entry" => {
                        uniparc = Default::default();
                        uniparc_xrefs = Vec::new();
                        uniparc_domains = Vec::new();
                        properties = Default::default();
                        uniparc_xref2properties = Default::default();
                        count += 1;
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
                    "ipr" => add_interpro_annotation(&mut uniparc_domains, attributes),
                    "lcn" => add_domain_definitions(&mut uniparc_domains, attributes),
                    "sequence" => add_sequence(&mut uniparc, attributes),
                    "accession" => {
                        // This is where we get the uniparc id from the character field.
                    }
                    _ => println!(
                        "Skipping StartElement '{}' with attributes {:?}.",
                        name.local_name,
                        attributes
                    ),
                }
                current_element.push(name.local_name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_ref() {
                    "entry" => {
                        write_uniparc(&mut handlers, &uniparc);
                        write_uniparc_xrefs(&mut handlers, &uniparc_xrefs);
                        write_uniparc_xref2properties(&mut handlers, &uniparc_xref2properties);
                        write_uniparc_properties(&mut handlers, &properties, uniparc.id.clone());
                        write_uniparc_domains(&mut handlers, &uniparc_domains);
                        if count % 10_000 == 0 {
                            println!("Finished processing UniParc number {}...", count);
                        }
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
            Ok(XmlEvent::EndDocument) => println!("Done processing document!"),
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





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

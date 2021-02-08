//! This is documentation for the `uniparc_xml_parser` crate.
//!
//!
extern crate flate2;
extern crate quick_xml;

pub mod writer;

mod model;
mod properties;

use std::error::Error;
use std::io::{BufReader, Stdin, Write};
use std::str;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use model::{Uniparc, UniparcDomain, UniparcProperty, UniparcXRef};
use properties::Properties;
pub use writer::{initialize_outputs, initialize_outputs_compressed};
use writer::{
    write_uniparc, write_uniparc_domains, write_uniparc_properties, write_uniparc_xrefs,
    OutputBuffers,
};

/// Add new data
fn add_uniparc_xref(
    uniparc_id: String,
    uniparc_xrefs: &mut Vec<UniparcXRef>,
    attributes: Vec<Attribute>,
) -> bool {
    let mut uniparc_xref = UniparcXRef {
        uniparc_id: uniparc_id,
        xref_id: (uniparc_xrefs.len() + 1) as u64,
        db_type: String::new(),
        db_id: String::new(),
        version_i: String::new(),
        active: String::new(),
        version: String::new(),
        created: String::new(),
        last: String::new(),
    };
    for attribute in attributes {
        match attribute.key {
            b"type" => {
                uniparc_xref.db_type = str::from_utf8(attribute.value).unwrap().to_string();
            }
            b"id" => {
                uniparc_xref.db_id = str::from_utf8(attribute.value).unwrap().to_string();
            }
            b"version_i" => {
                uniparc_xref.version_i = str::from_utf8(attribute.value).unwrap().to_string();
            }
            b"active" => {
                uniparc_xref.active = str::from_utf8(attribute.value).unwrap().to_string();
            }
            b"version" => {
                uniparc_xref.version = str::from_utf8(attribute.value).unwrap().to_string();
            }
            b"created" => {
                uniparc_xref.created = str::from_utf8(attribute.value).unwrap().to_string();
            }
            b"last" => {
                uniparc_xref.last = str::from_utf8(attribute.value).unwrap().to_string();
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
    properties: &mut Properties<Vec<UniparcProperty>>,
    attributes: Vec<Attribute>,
) {
    let attr_type = str::from_utf8(attributes[0].value).unwrap();
    let mut attr_value = str::from_utf8(attributes[1].value).unwrap().to_string();

    let xref_id = uniparc_xrefs.len() as u64;

    if attr_type == "chain" {
        assert!(uniparc_xrefs.last().unwrap().db_type == "PDB");
        attr_value = uniparc_xrefs.last().unwrap().db_id.clone() + &attr_value;
    }

    let (property_vec, attr_type_clean) = match attr_type {
        "NCBI_GI" => (&mut properties.ncbi_gi, String::from("ncbi_gi")),
        "NCBI_taxonomy_id" => (
            &mut properties.ncbi_taxonomy_id,
            String::from("ncbi_taxonomy_id"),
        ),
        "protein_name" => (&mut properties.protein_name, String::from("protein_name")),
        "gene_name" => (&mut properties.gene_name, String::from("gene_name")),
        "chain" => (&mut properties.pdb_chain, String::from("pdb_chain")),
        "UniProtKB_accession" => (
            &mut properties.uniprot_kb_accession,
            String::from("uniprot_kb_accession"),
        ),
        "proteome_id" => (&mut properties.proteome_id, String::from("proteome_id")),
        "component" => (&mut properties.component, String::from("component")),
        _ => panic!("Unmatched value: '{:?}'.", attr_type),
    };

    property_vec.push(UniparcProperty {
        uniparc_id,
        xref_id,
        property: attr_type_clean,
        value: attr_value,
    });
}

fn add_signature_sequence_match(
    uniparc_id: String,
    uniparc_domains: &mut Vec<UniparcDomain>,
    attributes: Vec<Attribute>,
) {
    let mut database = String::new();
    let mut database_id = String::new();
    for attribute in attributes {
        match attribute.key {
            b"database" => {
                database = str::from_utf8(attribute.value).unwrap().to_string();
            }
            b"id" => {
                database_id = str::from_utf8(attribute.value).unwrap().to_string();
            }
            _ => panic!("Unmatched value: '{:?}'.", attribute.key),
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

fn add_interpro_annotation(uniparc_domains: &mut Vec<UniparcDomain>, attributes: Vec<Attribute>) {
    let mut interpro_name = String::new();
    let mut interpro_id = String::new();
    for attribute in attributes {
        match attribute.key {
            b"name" => {
                interpro_name = str::from_utf8(attribute.value).unwrap().to_string();
            }
            b"id" => {
                interpro_id = str::from_utf8(attribute.value).unwrap().to_string();
            }
            _ => panic!("Unmatched value: '{:?}'.", attribute.key),
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

fn add_domain_definitions(uniparc_domains: &mut Vec<UniparcDomain>, attributes: Vec<Attribute>) {
    let mut domain_start: u32 = 0;
    let mut domain_end: u32 = 0;
    for attribute in attributes {
        match attribute.key {
            b"start" => {
                domain_start = str::from_utf8(attribute.value)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            }
            b"end" => {
                domain_end = str::from_utf8(attribute.value)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            }
            _ => panic!("Unmatched value: '{:?}'.", attribute.key),
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

fn add_sequence(uniparc: &mut Uniparc, attributes: Vec<Attribute>) {
    for attribute in attributes {
        match attribute.key {
            b"length" => {
                uniparc.sequence_length = str::from_utf8(attribute.value)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
            }
            b"checksum" => {
                uniparc.sequence_checksum = str::from_utf8(attribute.value).unwrap().to_string();
            }
            _ => {
                println!("Skipping attribute '{:?}' for sequence.", attribute);
            }
        }
    }
}

fn attribute_to_string(a: Attribute) -> (String, String) {
    let key = str::from_utf8(a.key).unwrap().to_string();
    let value = str::from_utf8(a.value).unwrap().to_string();
    (key, value)
}

enum TextField {
    Accession,
    Sequence,
}

/// Main loop
pub fn run<T: Write>(
    input_stream: Stdin,
    mut handlers: OutputBuffers<T>,
) -> Result<usize, Box<dyn Error>> {
    let mut reader = Reader::from_reader(BufReader::new(input_stream));
    reader.trim_text(true);

    // Variables created for each UniParc ID
    let mut uniparc: Uniparc = Default::default();
    let mut domains: Vec<UniparcDomain> = Vec::new();
    let mut xrefs: Vec<UniparcXRef> = Vec::new();
    let mut properties: Properties<Vec<UniparcProperty>> = Default::default();

    let mut keep_uniparc_xref = true;
    let mut current_element = Vec::new();

    // Number of UniParc entries that are in the proces of being processed.
    // When we start processing an entry, we *increment* `depth` by 1.
    // When we finish processing an entry, we *decrement* `depth` by 1.
    let mut depth = 0;
    // The number of UniParc sequences that have been processed.
    let mut count = 0;
    let mut buf = Vec::new();

    let mut text_field = TextField::Accession;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"entry" => {
                        uniparc = Default::default();
                        domains = Vec::new();
                        xrefs = Vec::new();
                        properties = Default::default();
                        count += 1;
                    }
                    b"dbReference" => {
                        keep_uniparc_xref = add_uniparc_xref(
                            uniparc.uniparc_id.clone(),
                            &mut xrefs,
                            e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>(),
                        );
                    }
                    b"signatureSequenceMatch" => {
                        add_signature_sequence_match(
                            uniparc.uniparc_id.clone(),
                            &mut domains,
                            e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>(),
                        );
                    }
                    b"accession" => {
                        text_field = TextField::Accession;
                    }
                    b"sequence" => {
                        text_field = TextField::Sequence;
                        add_sequence(
                            &mut uniparc,
                            e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>(),
                        );
                    }
                    _ => println!(
                        "Skipping StartElement '{}' with attributes {:?}.",
                        str::from_utf8(e.name()).unwrap(),
                        e.attributes()
                            .map(|a| attribute_to_string(a.unwrap()))
                            .collect::<Vec<_>>()
                    ),
                }
                current_element.push(e.name().to_ascii_lowercase());
                depth += 1;
            }
            Ok(Event::Empty(ref e)) => match e.name() {
                b"dbReference" => {
                    add_uniparc_xref(
                        uniparc.uniparc_id.clone(),
                        &mut xrefs,
                        e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>(),
                    );
                }
                b"property" => {
                    if keep_uniparc_xref {
                        add_property(
                            uniparc.uniparc_id.clone(),
                            &xrefs,
                            &mut properties,
                            e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>(),
                        );
                    }
                }
                b"ipr" => add_interpro_annotation(
                    &mut domains,
                    e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>(),
                ),
                b"lcn" => add_domain_definitions(
                    &mut domains,
                    e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>(),
                ),
                _ => println!(
                    "Skipping Empty element '{:?}' with attributes {:?}.",
                    str::from_utf8(e.name()).unwrap(),
                    e.attributes()
                        .map(|a| attribute_to_string(a.unwrap()))
                        .collect::<Vec<_>>()
                ),
            },
            Ok(Event::Text(text)) => match text_field {
                TextField::Accession => {
                    uniparc.uniparc_id =
                        text.unescape_and_decode(&reader).unwrap().replace("\n", "");
                }
                TextField::Sequence => {
                    uniparc.sequence = text.unescape_and_decode(&reader).unwrap().replace("\n", "");
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"entry" => {
                        write_uniparc(&mut handlers, &uniparc);
                        write_uniparc_domains(&mut handlers, &domains);
                        write_uniparc_xrefs(&mut handlers, &xrefs);
                        write_uniparc_properties(&mut handlers, &properties);
                        if count % 10_000 == 0 {
                            println!("Finished processing UniParc number {}...", count);
                        }
                    }
                    _ => {}
                }
                assert!(current_element.pop().unwrap() == e.name().to_ascii_lowercase());
                depth -= 1;
            }
            Ok(Event::CData(e)) => println!("Skipping CData '{:?}'.", e),
            Ok(Event::Decl(e)) => println!("Skipping Decl '{:?}'.", e),
            Ok(Event::PI(e)) => println!("Skipping PI '{:?}'.", e),
            Ok(Event::Comment(comment)) => println!("Skipping Comment: '{:?}'", comment),
            Ok(Event::DocType(e)) => println!("Skipping DocType: '{:?}'", e),
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear();
    }
    // buf.clear();
    println!("Depth: {}", depth);
    assert!(depth == 0);
    Ok(count)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

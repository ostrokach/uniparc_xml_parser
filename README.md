# UniParc XML parser

Process the UniParc XML file (`uniparc_all.xml.gz`) downloaded from the UniProt [website](http://www.uniprot.org/downloads) into CSV files that can be loaded into a relational database.

## Usage

Uncompressed XML data can be piped into `uniparc_xml_parser` in order to

```bash
$ curl -sS ftp://ftp.uniprot.org/pub/databases/uniprot/current_release/uniparc/uniparc_all.xml.gz \
    | zcat \
    | uniparc_xml_parser
```

The output is a set of CSV (or more specifically TSV) files:

```bash
$ ls
component.tsv  gene_name.tsv  ncbi_taxonomy_id.tsv  protein_name.tsv  uniparc.tsv               xref.tsv
domain.tsv     ncbi_gi.tsv    pdb_chain.tsv         proteome_id.tsv   uniprot_kb_accession.tsv
```

## Schema

The generated CSV files conform to the following schema:

<div align="center">
<img src="docs/schema/uml-diagram.svg" width="800px" />
</div>

## Benchmarks

Parsing 1 million lines takes about 5.5 seconds (the process is mostly IO-bound):

```txt
$ mkdir uniparc
$ time bash -c "zcat tests/uniparc_1mil.xml.gz | uniparc_xml_parser >/dev/null"

real    0m5.564s
user    0m5.528s
sys     0m0.132s
```

The actual `uniparc_all.xml.gz` file is about 5 billion rows.

Parsing 10,000 XML entires takes around 30 seconds (the process is mostly IO-bound):

```bash
$ time bash -c "zcat ../data/uniparc_top_10000.xml.gz | ./uniparc_xml_parser" >/dev/null

real    0m33.925s
user    0m36.800s
sys     0m1.892s
```

## FAQ (Frequently Asked Questions)

**Why not split `uniparc_all.xml.gz` into multiple small files and process them in parallel?**

- Splitting the file requires reading the entire file. If we're reading the entire file anyway, why not parse it as we read it?
- Having a single process which parses `uniparc_all.xml.gz` makes it easier to create an incremental unique index column (e.g. `UniparcXRef.idx`, `Property.idx`, etc.).

## FUQ (Frequently Used Queries)

TODO

## Roadmap

- [ ] Keep everything in bytes all the way until output.

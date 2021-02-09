# UniParc XML parser <!-- omit in toc -->

[![docs](https://img.shields.io/badge/docs-v0.2.0-blue.svg)](https://ostrokach.gitlab.io/uniparc_xml_parser/v0.2.0/)
[![conda](https://img.shields.io/conda/pn/ostrokach-forge/uniparc_xml_parser)](https://anaconda.org/ostrokach-forge/uniparc_xml_parser/)
[![pipeline status](https://gitlab.com/ostrokach/uniparc_xml_parser/badges/v0.2.0/pipeline.svg)](https://gitlab.com/ostrokach/uniparc_xml_parser/commits/v0.2.0/)

- [Introduction](#introduction)
- [Usage](#usage)
- [Table schema](#table-schema)
- [Installation](#installation)
  - [Binaries](#binaries)
  - [Cargo](#cargo)
  - [Conda](#conda)
- [Output files](#output-files)
  - [Parquet](#parquet)
  - [Google BigQuery](#google-bigquery)
- [Benchmarks](#benchmarks)
- [Roadmap](#roadmap)
- [FAQ (Frequently Asked Questions)](#faq-frequently-asked-questions)
- [FUQ (Frequently Used Queries)](#fuq-frequently-used-queries)

## Introduction

`uniparc_xml_parser` is a small utility which can process the UniParc XML file (`uniparc_all.xml.gz`), available from the UniProt [website](http://www.uniprot.org/downloads), into a set of CSV files that can be loaded into a relational database.

We also provide Parquet files, which can be queried using tools such as AWS Athena and Apache Presto, and have uploaded the generated data to Google BigQuery (see: [Output files](#output-files)).

## Usage

Uncompressed XML data can be piped into `uniparc_xml_parser` in order to parse the data into a set of CSV files on the fly:

```bash
$ curl -sS ftp://ftp.uniprot.org/pub/databases/uniprot/current_release/uniparc/uniparc_all.xml.gz \
    | zcat \
    | uniparc_xml_parser
```

The output is a set of CSV (or more specifically TSV) files:

```bash
$ ls
-rw-r--r-- 1 user group 174G Feb  9 13:52 xref.tsv
-rw-r--r-- 1 user group 149G Feb  9 13:52 domain.tsv
-rw-r--r-- 1 user group 138G Feb  9 13:52 uniparc.tsv
-rw-r--r-- 1 user group 107G Feb  9 13:52 protein_name.tsv
-rw-r--r-- 1 user group  99G Feb  9 13:52 ncbi_taxonomy_id.tsv
-rw-r--r-- 1 user group  74G Feb  9 20:13 uniparc.parquet
-rw-r--r-- 1 user group  64G Feb  9 13:52 gene_name.tsv
-rw-r--r-- 1 user group  39G Feb  9 13:52 component.tsv
-rw-r--r-- 1 user group  32G Feb  9 13:52 proteome_id.tsv
-rw-r--r-- 1 user group  15G Feb  9 13:52 ncbi_gi.tsv
-rw-r--r-- 1 user group  21M Feb  9 13:52 pdb_chain.tsv
-rw-r--r-- 1 user group  12M Feb  9 13:52 uniprot_kb_accession.tsv
-rw-r--r-- 1 user group 656K Feb  9 04:04 uniprot_kb_accession.parquet
```

## Table schema

The generated CSV files conform to the following schema:

<div align="center">
<img src="docs/schema/uml-diagram.svg" width="800px" />
</div>

## Installation

### Binaries

Linux binaries are available here: https://gitlab.com/ostrokach/uniparc_xml_parser/-/packages.

### Cargo

Use [`cargo`](https://crates.io/) to compile and install `uniparc_xml_parser` for your target platform:

```bash
cargo install uniparc_xml_parser
```

### Conda

Use [`conda`](https://docs.conda.io/en/latest/miniconda.html) to install precompiled binaries:

```bash
conda install -c ostrokach-forge uniparc_xml_parser
```

## Output files

### Parquet

Parquet files containing the processed data are available at the following URL and are updated monthly: <http://uniparc.data.proteinsolver.org/>.

### Google BigQuery

The latest data can also be queried directly using Google BigQuery: <https://console.cloud.google.com/bigquery?project=ostrokach-data&p=ostrokach-data&page=dataset&d=uniparc>.

## Benchmarks

Parsing 10,000 XML entires takes around 30 seconds (the process is mostly IO-bound):

```bash
$ time bash -c "zcat uniparc_top_10k.xml.gz | uniparc_xml_parser >/dev/null"

real    0m33.925s
user    0m36.800s
sys     0m1.892s
```

The actual `uniparc_all.xml.gz` file has around 373,914,570 elements.

## Roadmap

- [ ] Add support for writing Apache Parquet files directly.
- [ ] Add support for writing output to object stores (such as S3 and GCS).
- [ ] Avoid decoding the input into strings (keep everything as bytes throughout).

## FAQ (Frequently Asked Questions)

**Why not split `uniparc_all.xml.gz` into multiple small files and process them in parallel?**

- Splitting the file requires reading the entire file. If we're reading the entire file anyway, why not parse it as we read it?
- Having a single process which parses `uniparc_all.xml.gz` makes it easier to create an incremental unique index column (e.g. `xref.xref_id`).

## FUQ (Frequently Used Queries)

TODO

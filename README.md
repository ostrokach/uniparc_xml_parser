# UniParc XML parser

Process the UniParc XML file (`uniparc_all.xml.gz`) downloaded from the UniProt [website](http://www.uniprot.org/downloads) into CSV files that can be loaded into a relational database.

## Example

Parsing 1 million lines takes about 5.5 seconds: 

```
$ mkdir uniparc
$ time bash -c "zcat tests/uniparc_1mil.xml.gz | uniparc_xml_parser >/dev/null"

real    0m5.564s
user    0m5.528s
sys     0m0.132s
```

The actual `uniparc_all.xml.gz` file is about 5 billion rows.


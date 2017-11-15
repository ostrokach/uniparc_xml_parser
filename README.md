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


## To do

Keep everything in bytes all the way until output.


## FAQ

#### Why not split `uniparc_all.xml.gz` into multiple small files and process them in parallel

- Splitting the file requires reading the entire file. If we're reading the entire file anyway, why not parse it as we read it?
- Having a single process which parses `uniparc_all.xml.gz` makes it easier to create an incremental unique index column (e.g. `UniparcXRef.idx`, `Property.idx`, etc.).
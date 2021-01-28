-- SVG column 1

CREATE TABLE uniparc (
	id varchar PRIMARY KEY,
	sequence varchar NOT NULL,
	sequence_length integer NOT NULL,
	sequence_checksum varchar NOT NULL
);

CREATE TABLE domain (
	uniparc_id varchar NOT NULL REFERENCES uniparc (id),
	database varchar NOT NULL,
	database_id varchar NOT NULL,
	interpro_name varchar NOT NULL,
	interpro_id varchar NOT NULL,
	domain_start integer NOT NULL,
	domain_end integer NOT NULL
);


-- SVG column 2

CREATE TABLE xref (
	uniparc_id varchar NOT NULL REFERENCES uniparc (id),
	idx bigint NOT NULL,
	db_type varchar NOT NULL,
	db_id varchar NOT NULL,
	version_i varchar NOT NULL,
	active varchar NOT NULL,
	version varchar NOT NULL,
	created varchar NOT NULL,
	last varchar NOT NULL,
	PRIMARY KEY (uniparc_id, idx)
);

-- SVG column 4

CREATE TABLE gene_name (
	uniparc_id varchar NOT NULL,
	name varchar NOT NULL,
	idx bigint NOT NULL,
	value varchar NOT NULL,
	PRIMARY KEY (uniparc_id, idx)
);

CREATE TABLE proteome_id (
	uniparc_id varchar NOT NULL,
	name varchar NOT NULL,
	idx bigint NOT NULL,
	value varchar NOT NULL,
	PRIMARY KEY (uniparc_id, idx)
);

CREATE TABLE protein_name (
	uniparc_id varchar NOT NULL,
	name varchar NOT NULL,
	idx bigint NOT NULL,
	value varchar NOT NULL,
	PRIMARY KEY (uniparc_id, idx)
);

CREATE TABLE component (
	uniparc_id varchar NOT NULL,
	name varchar NOT NULL,
	idx bigint NOT NULL,
	value varchar NOT NULL,
	PRIMARY KEY (uniparc_id, idx)
);

CREATE TABLE ncbi_gi (
	uniparc_id varchar NOT NULL,
	name varchar NOT NULL,
	idx bigint NOT NULL,
	value varchar NOT NULL,
	PRIMARY KEY (uniparc_id, idx)
);

CREATE TABLE ncbi_taxonomy_id (
	uniparc_id varchar NOT NULL,
	name varchar NOT NULL,
	idx bigint NOT NULL,
	value varchar NOT NULL,
	PRIMARY KEY (uniparc_id, idx)
);

CREATE TABLE chain (
	uniparc_id varchar NOT NULL,
	name varchar NOT NULL,
	idx bigint NOT NULL,
	value varchar NOT NULL,
	PRIMARY KEY (uniparc_id, idx)
);

CREATE TABLE kb_accession (
	uniparc_id varchar NOT NULL,
	name varchar NOT NULL,
	idx bigint NOT NULL,
	value varchar NOT NULL,
	PRIMARY KEY (uniparc_id, idx)
);

-- SVG column 3

CREATE TABLE xref_gene_name (
	uniparc_id varchar NOT NULL,
	xref_idx bigint NOT NULL,
	property_name varchar NOT NULL,
	property_idx bigint NOT NULL,
	FOREIGN KEY (uniparc_id, xref_idx) REFERENCES gene_name (uniparc_id, idx)
);

CREATE TABLE xref_proteome_id (
	uniparc_id varchar NOT NULL,
	xref_idx bigint NOT NULL,
	property_name varchar NOT NULL,
	property_idx bigint NOT NULL,
	FOREIGN KEY (uniparc_id, xref_idx) REFERENCES proteome_id (uniparc_id, idx)
);

CREATE TABLE xref_protein_name (
	uniparc_id varchar NOT NULL,
	xref_idx bigint NOT NULL,
	property_name varchar NOT NULL,
	property_idx bigint NOT NULL,
	FOREIGN KEY (uniparc_id, xref_idx) REFERENCES protein_name (uniparc_id, idx)
);

CREATE TABLE xref_component (
	uniparc_id varchar NOT NULL,
	xref_idx bigint NOT NULL,
	property_name varchar NOT NULL,
	property_idx bigint NOT NULL,
	FOREIGN KEY (uniparc_id, xref_idx) REFERENCES component (uniparc_id, idx)
);

CREATE TABLE xref_ncbi_gi (
	uniparc_id varchar NOT NULL,
	xref_idx bigint NOT NULL,
	property_name varchar NOT NULL,
	property_idx bigint NOT NULL,
	FOREIGN KEY (uniparc_id, xref_idx) REFERENCES ncbi_gi (uniparc_id, idx)
);

CREATE TABLE xref_ncbi_taxonomy_id (
	uniparc_id varchar NOT NULL,
	xref_idx bigint NOT NULL,
	property_name varchar NOT NULL,
	property_idx bigint NOT NULL,
	FOREIGN KEY (uniparc_id, xref_idx) REFERENCES ncbi_taxonomy_id (uniparc_id, idx)
);

CREATE TABLE xref_chain (
	uniparc_id varchar NOT NULL,
	xref_idx bigint NOT NULL,
	property_name varchar NOT NULL,
	property_idx bigint NOT NULL,
	FOREIGN KEY (uniparc_id, xref_idx) REFERENCES chain (uniparc_id, idx)
);

CREATE TABLE xref_kb_accession (
	uniparc_id varchar NOT NULL,
	xref_idx bigint NOT NULL,
	property_name varchar NOT NULL,
	property_idx bigint NOT NULL,
	FOREIGN KEY (uniparc_id, xref_idx) REFERENCES kb_accession (uniparc_id, idx)
);


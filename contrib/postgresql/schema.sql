-- SVG column 1

CREATE TABLE uniparc (
	uniparc_id varchar PRIMARY KEY,
	sequence varchar NOT NULL,
	sequence_length integer NOT NULL,
	sequence_checksum varchar NOT NULL
);

CREATE TABLE domain (
	uniparc_id varchar NOT NULL REFERENCES uniparc (uniparc_id),
	database varchar NOT NULL,
	database_id varchar NOT NULL,
	interpro_name varchar NOT NULL,
	interpro_id varchar NOT NULL,
	domain_start integer NOT NULL,
	domain_end integer NOT NULL
);


-- SVG column 2

CREATE TABLE xref (
	uniparc_id varchar NOT NULL REFERENCES uniparc (uniparc_id),
	xref_id bigint NOT NULL,
	db_type varchar NOT NULL,
	db_id varchar NOT NULL,
	version_i varchar NOT NULL,
	active varchar NOT NULL,
	version varchar NOT NULL,
	created varchar NOT NULL,
	last varchar NOT NULL,
	PRIMARY KEY (uniparc_id, xref_id)
);


-- SVG column 3

CREATE TABLE component (
	uniparc_id varchar NOT NULL,
	xref_id bigint NOT NULL,
	property varchar NOT NULL,
	value varchar NOT NULL,
	FOREIGN KEY (uniparc_id, xref_id) REFERENCES xref (uniparc_id, xref_id)
);

CREATE TABLE gene_name (
	uniparc_id varchar NOT NULL,
	xref_id bigint NOT NULL,
	property varchar NOT NULL,
	value varchar NOT NULL,
	FOREIGN KEY (uniparc_id, xref_id) REFERENCES xref (uniparc_id, xref_id)
);

CREATE TABLE ncbi_gi (
	uniparc_id varchar NOT NULL,
	xref_id bigint NOT NULL,
	property varchar NOT NULL,
	value varchar NOT NULL,
	FOREIGN KEY (uniparc_id, xref_id) REFERENCES xref (uniparc_id, xref_id)
);

CREATE TABLE ncbi_taxonomy_id (
	uniparc_id varchar NOT NULL,
	xref_id bigint NOT NULL,
	property varchar NOT NULL,
	value varchar NOT NULL,
	FOREIGN KEY (uniparc_id, xref_id) REFERENCES xref (uniparc_id, xref_id)
);

CREATE TABLE pdb_chain (
	uniparc_id varchar NOT NULL,
	xref_id bigint NOT NULL,
	property varchar NOT NULL,
	value varchar NOT NULL,
	FOREIGN KEY (uniparc_id, xref_id) REFERENCES xref (uniparc_id, xref_id)
);

CREATE TABLE protein_name (
	uniparc_id varchar NOT NULL,
	xref_id bigint NOT NULL,
	property varchar NOT NULL,
	value varchar NOT NULL,
	FOREIGN KEY (uniparc_id, xref_id) REFERENCES xref (uniparc_id, xref_id)
);

CREATE TABLE proteome_id (
	uniparc_id varchar NOT NULL,
	xref_id bigint NOT NULL,
	property varchar NOT NULL,
	value varchar NOT NULL,
	FOREIGN KEY (uniparc_id, xref_id) REFERENCES xref (uniparc_id, xref_id)
);

CREATE TABLE uniprot_kb_accession (
	uniparc_id varchar NOT NULL,
	xref_id bigint NOT NULL,
	property varchar NOT NULL,
	value varchar NOT NULL,
	FOREIGN KEY (uniparc_id, xref_id) REFERENCES xref (uniparc_id, xref_id)
);

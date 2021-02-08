\set ECHO all
\timing

-- SVG column 1
COPY uniparc               FROM 'uniparc.tsv'               DELIMITER E'\t' csv;
COPY domain                FROM 'domain.tsv'                DELIMITER E'\t' csv;

-- SVG column 3
COPY component             FROM 'component.tsv'             DELIMITER E'\t' csv;
COPY gene_name             FROM 'gene_name.tsv'             DELIMITER E'\t' csv;
COPY ncbi_gi               FROM 'ncbi_gi.tsv'               DELIMITER E'\t' csv;
COPY ncbi_taxonomy_id      FROM 'ncbi_taxonomy_id.tsv'      DELIMITER E'\t' csv;
COPY pdb_chain             FROM 'pdb_chain.tsv'             DELIMITER E'\t' csv;
COPY protein_name          FROM 'protein_name.tsv'          DELIMITER E'\t' csv;
COPY proteome_id           FROM 'proteome_id.tsv'           DELIMITER E'\t' csv;
COPY uniprot_kb_accession  FROM 'uniprot_kb_accession.tsv'  DELIMITER E'\t' csv;

-- SVG column 2
COPY xref                  FROM 'xref.tsv'                  DELIMITER E'\t' csv;

VACUUM ANALYZE;

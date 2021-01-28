\set ECHO all
\timing

-- SVG column 1
COPY uniparc               FROM 'uniparc.tsv'               DELIMITER E'\t' csv;
COPY domain                FROM 'domain.tsv'                DELIMITER E'\t' csv;

-- SVG column 4
COPY kb_accession          FROM 'kb_accession.tsv'          DELIMITER E'\t' csv;
COPY chain                 FROM 'chain.tsv'                 DELIMITER E'\t' csv;
COPY ncbi_taxonomy_id      FROM 'ncbi_taxonomy_id.tsv'      DELIMITER E'\t' csv;
COPY ncbi_gi               FROM 'ncbi_gi.tsv'               DELIMITER E'\t' csv;
COPY component             FROM 'component.tsv'             DELIMITER E'\t' csv;
COPY protein_name          FROM 'protein_name.tsv'          DELIMITER E'\t' csv;
COPY proteome_id           FROM 'proteome_id.tsv'           DELIMITER E'\t' csv;
COPY gene_name             FROM 'gene_name.tsv'             DELIMITER E'\t' csv;

-- SVG column 3
COPY xref_gene_name        FROM 'xref_gene_name.tsv'        DELIMITER E'\t' csv;
COPY xref_proteome_id      FROM 'xref_proteome_id.tsv'      DELIMITER E'\t' csv;
COPY xref_protein_name     FROM 'xref_protein_name.tsv'     DELIMITER E'\t' csv;
COPY xref_component        FROM 'xref_component.tsv'        DELIMITER E'\t' csv;
COPY xref_ncbi_gi          FROM 'xref_ncbi_gi.tsv'          DELIMITER E'\t' csv;
COPY xref_ncbi_taxonomy_id FROM 'xref_ncbi_taxonomy_id.tsv' DELIMITER E'\t' csv;
COPY xref_chain            FROM 'xref_chain.tsv'            DELIMITER E'\t' csv;
COPY xref_kb_accession     FROM 'xref_kb_accession.tsv'     DELIMITER E'\t' csv;

-- SVG column 2
COPY xref                  FROM 'xref.tsv'                  DELIMITER E'\t' csv;

VACUUM ANALYZE;


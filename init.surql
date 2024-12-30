DEFINE TABLE emoji TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD created_at       ON emoji TYPE datetime          VALUE time::now() READONLY;
DEFINE FIELD updated_at       ON emoji TYPE datetime          VALUE time::now() READONLY;
DEFINE FIELD deleted_at       ON emoji TYPE option<datetime>  DEFAULT NONE PERMISSIONS FULL;
DEFINE FIELD code             ON emoji TYPE string            PERMISSIONS FULL;
DEFINE FIELD description      ON emoji TYPE option<string>    PERMISSIONS FULL;
DEFINE FIELD file             ON emoji TYPE string            PERMISSIONS FULL;
DEFINE FIELD hash             ON emoji TYPE string            PERMISSIONS FULL;
DEFINE FIELD pack             ON emoji TYPE record<emojipack> PERMISSIONS FULL;
DEFINE FIELD dsc_embedding    ON emoji TYPE array<number>     DEFAULT [] PERMISSIONS FULL;
DEFINE FIELD vis_embedding    ON emoji TYPE array<number>     DEFAULT [] PERMISSIONS FULL;

DEFINE INDEX idx_code_unique      ON TABLE emoji COLUMNS code UNIQUE;
DEFINE INDEX idx_dsc_embedding_mtree_cosine ON TABLE emoji FIELDS dsc_embedding MTREE DIMENSION 384 DIST COSINE;
DEFINE INDEX idx_vis_embedding_mtree_cosine ON TABLE emoji FIELDS vis_embedding MTREE DIMENSION 384 DIST COSINE;

DEFINE TABLE emojipack TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD created_at   ON emojipack TYPE datetime      VALUE time::now() READONLY;
DEFINE FIELD updated_at   ON emojipack TYPE datetime      VALUE time::now() READONLY;
DEFINE FIELD deleted_at   ON emojipack TYPE option<datetime>      DEFAULT NONE PERMISSIONS FULL;
DEFINE FIELD name         ON emojipack TYPE string        PERMISSIONS FULL;
DEFINE FIELD description  ON emojipack TYPE string        PERMISSIONS FULL;
DEFINE FIELD hash         ON emojipack TYPE string        PERMISSIONS FULL;
DEFINE FIELD embedding    ON emojipack TYPE array<number> DEFAULT [] PERMISSIONS FULL;

DEFINE INDEX idx_name_unique ON TABLE emojipack COLUMNS name UNIQUE;
DEFINE INDEX idx_embedding_mtree_cosine ON TABLE emojipack FIELDS embedding MTREE DIMENSION 384 DIST COSINE;

DEFINE TABLE IF NOT EXISTS total_emoji_count TYPE NORMAL AS
SELECT count() FROM emoji GROUP ALL;

DEFINE TABLE IF NOT EXISTS indexed_emoji_count TYPE NORMAL AS
SELECT count() FROM emoji WHERE array::len(dsc_embedding) > 0 AND array::len(vis_embedding) > 0 GROUP ALL;

DEFINE TABLE IF NOT EXISTS total_emojipack_count TYPE NORMAL AS
SELECT count() FROM emojipack GROUP ALL;

DEFINE TABLE IF NOT EXISTS indexed_emojipack_count TYPE NORMAL AS
SELECT count() FROM emojipack WHERE array::len(embedding) > 0 GROUP ALL;

DEFINE TABLE OVERWRITE emoji TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD created_at       ON emoji TYPE datetime          VALUE time::now() READONLY;
DEFINE FIELD updated_at       ON emoji TYPE datetime          VALUE time::now() READONLY;
DEFINE FIELD deleted_at       ON emoji TYPE option<datetime>  DEFAULT NONE PERMISSIONS FULL;
DEFINE FIELD code             ON emoji TYPE string            PERMISSIONS FULL;
DEFINE FIELD description      ON emoji TYPE option<string>    PERMISSIONS FULL;
DEFINE FIELD file             ON emoji TYPE string            PERMISSIONS FULL;
DEFINE FIELD hash             ON emoji TYPE string            PERMISSIONS FULL;
DEFINE FIELD pack             ON emoji TYPE record<emojipack> PERMISSIONS FULL;
DEFINE INDEX idx_code_unique      ON TABLE emoji COLUMNS code UNIQUE;

DEFINE TABLE OVERWRITE emoji_embedding TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD emoji ON emoji_embedding TYPE record<emoji> PERMISSIONS FULL;
DEFINE FIELD dsc   ON emoji_embedding TYPE array<number> PERMISSIONS FULL;
DEFINE FIELD viz   ON emoji_embedding TYPE array<number> PERMISSIONS FULL;
DEFINE INDEX idx_dsc_embedding_mtree_cosine ON TABLE emoji_embedding FIELDS dsc MTREE DIMENSION 384 DIST COSINE;
DEFINE INDEX idx_viz_embedding_mtree_cosine ON TABLE emoji_embedding FIELDS viz MTREE DIMENSION 384 DIST COSINE;

DEFINE TABLE OVERWRITE emojipack TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD created_at   ON emojipack TYPE datetime      VALUE time::now() READONLY;
DEFINE FIELD updated_at   ON emojipack TYPE datetime      VALUE time::now() READONLY;
DEFINE FIELD deleted_at   ON emojipack TYPE option<datetime>      DEFAULT NONE PERMISSIONS FULL;
DEFINE FIELD name         ON emojipack TYPE string        PERMISSIONS FULL;
DEFINE FIELD description  ON emojipack TYPE string        PERMISSIONS FULL;
DEFINE FIELD hash         ON emojipack TYPE string        PERMISSIONS FULL;
DEFINE INDEX idx_name_unique ON TABLE emojipack COLUMNS name UNIQUE;

DEFINE TABLE OVERWRITE total_emoji_count TYPE NORMAL AS
SELECT count() FROM emoji GROUP ALL;

DEFINE TABLE OVERWRITE indexed_emoji_count TYPE NORMAL AS
SELECT count() FROM emoji_embedding GROUP ALL;

DEFINE TABLE OVERWRITE total_emojipack_count TYPE NORMAL AS
SELECT count() FROM emojipack GROUP ALL;

DEFINE TABLE OVERWRITE indexed_emojipack_count TYPE NORMAL AS
SELECT count() FROM emojipack GROUP ALL;

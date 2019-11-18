PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: input_discourse
DROP TABLE IF EXISTS input_discourse;

CREATE TABLE input_discourse (
    discourse_id       INTEGER PRIMARY KEY,
    hypernym_synset_id INTEGER,
    document_hash      TEXT,
    author_public_hash TEXT,
    author_title       TEXT,
    author_first_name  TEXT,
    author_middle_name TEXT,
    author_surname     TEXT,
    author_zone        TEXT,
    date_unix_epoch    INTEGER,
    x                  DOUBLE,
    y                  DOUBLE
);


-- Table: input_discourse_title
DROP TABLE IF EXISTS input_discourse_title;

CREATE TABLE input_discourse_title (
    discourse_id INTEGER REFERENCES input_discourse (discourse_id),
    word_id      INTEGER REFERENCES input_word (word_id) 
);


-- Table: input_exempt_feature
DROP TABLE IF EXISTS input_exempt_feature;

CREATE TABLE input_exempt_feature (
    discourse_id     INTEGER REFERENCES input_discourse (discourse_id),
    ud_feature       TEXT,
    ud_feature_value TEXT
);


-- Table: input_new_word_def
DROP TABLE IF EXISTS input_new_word_def;

CREATE TABLE input_new_word_def (
    new_word_id  INTEGER PRIMARY KEY,
    is_a         INTEGER,
    discourse_id INTEGER REFERENCES input_discourse (discourse_id),
    lexeme       TEXT,
    x            DOUBLE,
    y            DOUBLE
);


-- Table: input_section
DROP TABLE IF EXISTS input_section;

CREATE TABLE input_section (
    section_id   INTEGER PRIMARY KEY,
    triplet_id   INTEGER REFERENCES input_triplet (triplet_id),
    word_id      INTEGER REFERENCES input_word (word_id),
    section_type INTEGER
);


-- Table: input_sentence
DROP TABLE IF EXISTS input_sentence;

CREATE TABLE input_sentence (
    sentence_id  INTEGER PRIMARY KEY,
    discourse_id INTEGER REFERENCES input_discourse (discourse_id),
    is_question  BOOL
);


-- Table: input_triplet
DROP TABLE IF EXISTS input_triplet;

CREATE TABLE input_triplet (
    triplet_id  INTEGER PRIMARY KEY,
    sentence_id INTEGER REFERENCES input_sentence (sentence_id),
    tense       INTEGER,
    is_virtual  BOOLEAN,
    is_passive  BOOLEAN
);


-- Table: input_word
DROP TABLE IF EXISTS input_word;

CREATE TABLE input_word (
    word_id        INTEGER PRIMARY KEY,
    sentence_id    INTEGER REFERENCES input_sentence (sentence_id),
    synset_id      INTEGER DEFAULT (0),
    index_of_word  INTEGER,
    lexeme         TEXT,
    instance_name  TEXT,
    instance_index TEXT  DEFAULT (0),
    pos            TEXT,
    x              DOUBLE  DEFAULT (0),
    y              DOUBLE  DEFAULT (0),
    is_transition  BOOLEAN,
    new_word_id    INTEGER DEFAULT ( -1) 
                           REFERENCES input_new_word_def (new_word_id) 
);


-- Table: input_word_feature
DROP TABLE IF EXISTS input_word_feature;

CREATE TABLE input_word_feature (
    word_id          INTEGER REFERENCES input_word (word_id),
    ud_relation      TEXT,
    ud_feature       TEXT,
    ud_feature_value TEXT
);


-- Table: input_word_relation
DROP TABLE IF EXISTS input_word_relation;

CREATE TABLE input_word_relation (
    word_id          INTEGER REFERENCES input_word (word_id),
    word_id_modified INTEGER REFERENCES input_word (word_id),
    ud_relation      TEXT
);


COMMIT TRANSACTION;
PRAGMA foreign_keys = on;

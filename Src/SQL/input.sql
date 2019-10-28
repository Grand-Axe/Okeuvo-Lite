PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: input_discourse
CREATE TABLE input_discourse (
    discourse_id       integer primary key,
    hypernym_synset_id integer,
    document_hash      string,
    author_public_hash string,
    author_title       string,
    author_first_name  string,
    author_middle_name string,
    author_surname     string,
    author_zone        string,
    date_unix_epoch    integer,
    x double,
    y double
);

-- Table: input_discourse_title
CREATE TABLE input_discourse_title (
    discourse_id    integer primary key,
    word_id           integer
);

-- Table: input_exempt_feature
CREATE TABLE input_exempt_feature (
    discourse_id          integer,
    ud_feature       string,
    ud_feature_value string
);

-- Table: input_new_word_def
CREATE TABLE input_new_word_def (
    new_word_id  integer primary key,
	is_a integer,
    discourse_id integer,
    lexeme       string,
    x            double,
    y            double
);

-- Table: input_section
CREATE TABLE input_section(
    section_id integer primary key,
    triplet_id integer,
	word_id integer,
	section_type integer
);

-- Table: input_sentence
CREATE TABLE input_sentence (
    sentence_id  integer primary key,
    discourse_id integer,
    is_question bool
);

-- Table: input_triplet
CREATE TABLE input_triplet (
    triplet_id integer primary key,
    sentence_id      integer,
    tense      integer,
    is_virtual boolean,
    is_passive boolean
);

-- Table: input_word
CREATE TABLE input_word (
    word_id        integer primary key,
    sentence_id    integer,
    synset_id      integer default (0),
    index_of_word  integer,
    lexeme         string,
    instance_name  string,
    instance_index string  default (0),
    pos            string,
    x              double  default (0),
    y              double  default (0),
    is_transition  boolean,
	new_word_id integer default (-1)
);

-- Table: input_word_feature
CREATE TABLE input_word_feature (
    word_id          integer,
    ud_relation       string,
    ud_feature       string,
    ud_feature_value string
);

-- Table: input_word_relation
CREATE TABLE input_word_relation (
    word_id            integer,
    word_id_modified integer,
    ud_relation        string
);

COMMIT TRANSACTION;
PRAGMA foreign_keys = on;

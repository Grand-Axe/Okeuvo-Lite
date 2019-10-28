PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: antonyms
CREATE TABLE antonyms
(
    coarse_class integer,
    synset_id1 integer,
    lemma1 string,
    synset_id2 integer,
    lemma2 string
);

-- Table: meaning_grid_item
CREATE TABLE meaning_grid_item (
    x         double,
    y         double,
    synset_id integer
);

COMMIT TRANSACTION;
PRAGMA foreign_keys = on;

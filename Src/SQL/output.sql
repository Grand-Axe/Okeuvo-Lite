PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: entity
CREATE TABLE entity (
    entity_id      integer primary key,
    instance_index integer,
    discourse_id   integer,
    synset_id      integer,
    word_id        integer,
    rank           double,
    x              double,
    y              double,
    triplet_id     integer
);

-- Table: hash_item
CREATE TABLE hash_item (
    radius                     integer,
    radius_original            double,
    angle                      integer,
    angle_original             double,
    hash_type                  integer,
    discourse_id               integer,
    excited_radius double,
	excited_angle double,
	order_by integer,
     is_virtual bool
);

-- Table: unit_tensor
CREATE TABLE unit_tensor (
    unit_tensor_id      integer primary key,
    sentence_id         integer,
    discourse_id        integer,
    object_entity_id    integer,
    subject_entity_id   integer,
    where_entity_id     integer,
    when_entity_id      integer,
    predicate_entity_id integer,
    tense               integer,
    mood                string,
    excited_x          double,
    excited_y          double
);

-- Table: unit_tensor_ethereal_def
CREATE TABLE unit_tensor_ethereal_def (
id integer primary key,
    branch_id integer,
    rejoin_id integer
    );

COMMIT TRANSACTION;
PRAGMA foreign_keys = on;

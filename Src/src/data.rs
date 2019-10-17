/*
DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS HEADER.

Copyright © 2019 Asame Imoni Obiomah. All rights reserved.

Artificial intelligence ethics is of existential importance.
The licensing model of OkeuvoLite enforces adherence to a strict ethical code.

The contents of this file are subject to the terms of both the GNU General Public License Version 2 only (“GPL”)
and Inverse license (collectively, the “License”). You may not use this file except in compliance with the License.
You can obtain a copy of the License at LICENSE.txt. See the License for the specific language governing
permissions and limitations under the License.

When distributing the software, include this License Header Notice in each file and include the License file at LICENSE.txt.
*/

use crate::math::Point2D;
use crate::METAPATH;
use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ExcitationData {
    pub magnitude: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct MeaningGridItem {
    pub x: f64,
    pub y: f64,
    pub synset_id: i32,
}

#[derive(Debug)]
pub struct Entity {
    pub entity_id: i32,
    pub instance_index: i32,
    pub discourse_id: i32,
    pub synset_id: i32,
    pub word_id: i32,
    pub rank: f64,
    pub x: f64,
    pub y: f64,
    pub triplet_id: i32,
}

#[derive(Debug)]
pub struct UnitTensorEtherealDef {
    pub branch_id: i64,
    pub rejoin_id: i64,
}

#[derive(Debug)]
pub struct UnitTensor {
    pub unit_tensor_id: i32,
    pub sentence_id: i32,
    pub discourse_id: i32,
    /// This field can only have a value
    /// when no explicit time has been given.
    /// In this case, sister field, when_entity_id must be empty.
    pub object_entity_id: i64,
    pub subject_entity_id: i64,
    pub where_entity_id: i64,
    /// This field can only have a value
    /// when an explicit time is given.
    /// In this case, sister field, object_entity_id must be empty.
    pub when_entity_id: i64,
    pub predicate_entity_id: i64,
    pub tense: i32,
    /// Verb mood.
    pub mood: String,
    pub excited_x: f64,
    pub excited_y: f64,
}

#[derive(Debug)]
pub struct UniversalDependencyTags {
    pub tag: String,
    pub rank: i32,
}

#[derive(Debug)]
pub struct InputDiscourse {
    pub discourse_id: i32,
    pub hypernym_synset_id: i32,
    pub document_hash: String,
    pub author_public_hash: String,
    pub author_title: String,
    pub author_first_name: String,
    pub author_middle_name: String,
    pub author_surname: String,
    pub author_zone: String,
    pub date_unix_epoch: i32,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct InputDiscourseTitle {
    pub discourse_id: i32,
    pub word_id: i32,
}

#[derive(Debug)]
pub struct InputSentence {
    pub sentence_id: i32,
    pub discourse_id: i32,
    pub is_question: bool,
}

#[derive(Debug, Clone)]
pub struct InputWord {
    pub word_id: i32,
    pub sentence_id: i32,
    pub synset_id: i32,
    pub index_of_word: i32,
    pub lexeme: String,
    pub instance_name: String,
    pub instance_index: i32,
    pub pos: String,
    pub x: f64,
    pub y: f64,
    pub is_transition: bool,
    pub new_word_id: i32,
}

#[derive(Debug)]
pub struct InputWordFeature {
    pub word_id: i32,
    pub ud_relation: String,
    pub ud_feature: String,
    pub ud_feature_value: String,
}

pub struct InputNewWordDef {
    pub new_word_id: i32,
    pub hypernym_synset_id: i32,
    pub discourse_id: i32,
    pub lexeme: String,
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub struct InputExemptFeature {
    pub discourse_id: i32,
    pub ud_feature: String,
    pub ud_feature_value: String,
}

#[derive(Debug)]
pub struct InputWordRelation {
    pub word_id: i32,
    pub word_id_modified: i32,
    pub ud_relation: String,
}

#[derive(Debug)]
pub struct InputSection {
    pub section_id: i32,
    pub triplet_id: i32,
    pub word_id: i32,
    pub section_type: i32,
}

#[derive(Debug)]
pub struct InputTriplet {
    pub triplet_id: i32,
    pub sentence_id: i32,
    /// 1 = past, 2 = present and 3 = future.
    pub tense: i32,
    pub is_virtual: bool,
    pub is_passive: bool,
}

#[derive(Debug, Clone)]
pub struct HashItem {
    pub radius: f64,
    pub angle: f64,
    pub hash_type: i32,
    pub discourse_id: i32,
    pub excited_radius: f64,
    pub excited_angle: f64,
}

#[derive(Debug)]
pub struct HashItemFormatted {
    pub right_leaning: i32,
    pub upper_angle: f64,
    pub right_leaning_excited: i32,
    pub upper_angle_excited: f64,
}

pub(crate) fn update_input_new_word_def(
    conn: &Connection,
    x: &f64,
    y: &f64,
    new_word_id: &i32,
) -> Result<()> {
    conn.execute(
        "update input_new_word_def set x = ?1, y = ?2 where new_word_id = ?3",
        &[&x as &ToSql, &y as &ToSql, &new_word_id as &ToSql],
    )?;

    Ok(())
}

pub(crate) fn insert_unit_tensor(conn: &Connection, unit_tensor: &UnitTensor) -> Result<(i64)> {
    conn.execute(
        "insert into unit_tensor
        (sentence_id, discourse_id, object_entity_id, subject_entity_id, where_entity_id, when_entity_id,
        predicate_entity_id, tense, mood,excited_x, excited_y)
        values (?1,?2,?3,?4,?5,?6.?7,?8,?9,?10)",
        &[
            &unit_tensor.sentence_id as &ToSql,
            &unit_tensor.discourse_id as &ToSql,
            &unit_tensor.object_entity_id as &ToSql,
            &unit_tensor.subject_entity_id as &ToSql,
            &unit_tensor.where_entity_id as &ToSql,
            &unit_tensor.when_entity_id as &ToSql,
            &unit_tensor.predicate_entity_id as &ToSql,
            &unit_tensor.tense as &ToSql,
            &unit_tensor.mood as &ToSql,
            &unit_tensor.excited_x as &ToSql,
            &unit_tensor.excited_y as &ToSql
        ],
    )?;

    let last_unit_tensor_id = conn.last_insert_rowid();

    Ok(last_unit_tensor_id)
}

pub(crate) fn insert_hash(
    conn: &Connection,
    hash_item: &HashItem,
    order_by: &i32,
    is_virtual: &bool,
) -> Result<()> {
    conn.execute(
        "insert into hashes (radius,radius_original,angle,angle_original,hash_type,discourse_id,
        excited_radius,excited_angle,order_by,is_virtual)
        values (?1,?2,?3,?4,?5,?6,?7,?8)",
        &[
            &hash_item.radius.round() as &ToSql,
            &hash_item.radius as &ToSql,
            &hash_item.angle.round() as &ToSql,
            &hash_item.angle as &ToSql,
            &hash_item.hash_type as &ToSql,
            &hash_item.discourse_id as &ToSql,
            &hash_item.excited_radius as &ToSql,
            &hash_item.excited_angle as &ToSql,
            &order_by as &ToSql,
            &is_virtual as &ToSql,
        ],
    )?;

    Ok(())
}

pub(crate) fn insert_unit_tensor_ethereal(conn: &Connection, branch_id: &i64) -> Result<i64> {
    conn.execute(
        "insert into unit_tensor_ethereal_def (branch_id)
        values (?1)",
        &[&branch_id as &ToSql],
    )?;

    Ok(conn.last_insert_rowid())
}

pub(crate) fn update_unit_tensor_ethereal(
    conn: &Connection,
    last_row_id: &i64,
    rejoin_id: &i64,
) -> Result<()> {
    conn.execute(
        "update unit_tensor_ethereal_def set rejoin_id = ?1 where id = ?2)",
        &[&rejoin_id as &ToSql, &last_row_id as &ToSql],
    )?;

    Ok(())
}

/// Inserts an entity if it does not occur in the database or updates it if it does.
pub(crate) fn insert_or_update_entity(conn: &Connection, entity: &Entity) -> Result<(i64)> {
    let mut current_entity_id: i64 = -1;
    // 0 is the default value, so it doesn't count in the check for existing
    let has_duplicate_instance_index: i64 = conn.query_row(
        "select exists (select 1 from entity where instance_index > 0 and instance_index = ?1)",
        &[&entity.instance_index],
        |row| row.get(0),
    )?;

    // No existing record for this instance, so insert.
    if has_duplicate_instance_index == 0 {
        conn.execute(
            "insert into entity (instance_index, synset_id, word_id, rank, x, y, triplet_id)
        values (?1,?2,?3,?4,?5,?6,?7)",
            &[
                &entity.instance_index as &ToSql,
                &entity.synset_id as &ToSql,
                &entity.word_id as &ToSql,
                &entity.rank as &ToSql,
                &entity.x as &ToSql,
                &entity.y as &ToSql,
                &entity.triplet_id as &ToSql,
            ],
        )?;

        current_entity_id = conn.last_insert_rowid();
    }
    // There is an existing record, so update.
    else {
        conn.execute(
        "update set instance_index = ?1, synset_id = ?2, word_id = ?3, rank = ?4, x = ?5, y = ?6, triplet_id = ?7",
        &[
            &entity.instance_index as &ToSql,
                &entity.synset_id as &ToSql,
                &entity.word_id as &ToSql,
                &entity.rank as &ToSql,
                &entity.x as &ToSql,
                &entity.y as &ToSql,
                &entity.triplet_id as &ToSql,
        ],
    )?;

        current_entity_id = conn.query_row(
            "select entity_id from entity where instance_index = ?1",
            &[&entity.instance_index],
            |row| row.get(0),
        )?;
    }

    // Return the last entity_id.
    Ok(current_entity_id)
}

pub(crate) fn insert_discourse(conn: &Connection) -> Result<()> {
    conn.execute("insert into discourse default values", NO_PARAMS)?;
    Ok(())
}

/// Gets unit tensors filtered by real or virtual.
pub(crate) fn select_unit_tensor(
    conn: &Connection,
    discourse_id: &i32,
    is_virtual: &bool,
) -> Result<Vec<UnitTensor>> {
    let sql =
        "select unit_tensor_id, sentence_id, discourse_id, object_entity_id, subject_entity_id,
where_entity_id, when_entity_id, predicate_entity_id, tense, mood, excited_x, excited_y
  from unit_tensor u1 ";

    // Filter for real events.
    let sql_real: String = format!(
        "{}{}",
        sql,
        "left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id
    not between u2.branch_id and u2.rejoin_id
where mood = 'ind' or mood = 'imp"
    );

    // Filter for virtual events.
    let sql_virtual: String = format!(
        "{}{}",
        sql,
        "left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id
    between u2.branch_id and u2.rejoin_id
where mood = 'ind' or mood = 'imp"
    );

    let mut stmt = conn.prepare(match is_virtual {
        true => sql_virtual.as_str(),
        false => sql_real.as_str(),
    })?;
    let unit_tensor_iter = stmt.query_map(&[&discourse_id], |row| {
        Ok(UnitTensor {
            unit_tensor_id: row.get(0)?,
            sentence_id: row.get(1)?,
            discourse_id: row.get(2)?,
            object_entity_id: row.get(3)?,
            subject_entity_id: row.get(4)?,
            where_entity_id: row.get(5)?,
            when_entity_id: row.get(6)?,
            predicate_entity_id: row.get(7)?,
            tense: row.get(8)?,
            mood: row.get(9)?,
            excited_x: row.get(10)?,
            excited_y: row.get(11)?,
        })
    })?;

    unit_tensor_iter.collect::<Result<Vec<UnitTensor>>>()
}

/// Get degree centrality of unit tensors filtered by real or virtual.
/// Only coreferenced items are considered.
pub(crate) fn select_unit_tensor_centrality(
    conn: &Connection,
    discourse_id: &i32,
    is_virtual: &bool,
) -> Result<HashMap<i64, i32>> {
    // Filter for real events.
    let sql_real = "select entity_id, count(entity_id) from
(
    select object_entity_id entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id not between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
    union
    select subject_entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id not between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
    union
    select where_entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id not between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
    union
    select when_entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id not between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
    union
    select predicate_entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id not between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
)
group by entity_id";

    // Filter for virtual events.
    let sql_virtual = "select entity_id, count(entity_id) from
(
    select object_entity_id entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
    union
    select subject_entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
    union
    select where_entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
    union
    select when_entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
    union
    select predicate_entity_id
    from unit_tensor u1
    left join unit_tensor_ethereal_def u2 on u1.unit_tensor_id between u2.branch_id and u2.rejoin_id
    where discourse_id = ?1 and instance_index > 0
)
group by entity_id";

    let mut stmt = conn.prepare(match is_virtual {
        true => sql_virtual,
        false => sql_real,
    })?;
    let unit_tensor_iter =
        stmt.query_map(&[&discourse_id], |row| Ok((row.get(0)?, row.get(1)?)))?;

    unit_tensor_iter.collect::<Result<HashMap<i64, i32>>>()
}

pub(crate) fn select_entity(conn: &Connection, discourse_id: &i32) -> Result<HashMap<i64, Entity>> {
    let mut stmt = conn.prepare(
        "select entity_id,instance_index,discourse_id,synset_id,word_id,rank,x,y,triplet_id
from entity where discourse_id = ?1",
    )?;
    let mut rows = stmt.query(&[&discourse_id])?;

    let mut entity_map = HashMap::<i64, Entity>::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let entity = Entity {
            entity_id: row.get(0)?,
            instance_index: row.get(1)?,
            discourse_id: row.get(2)?,
            synset_id: row.get(3)?,
            word_id: row.get(4)?,
            rank: row.get(5)?,
            x: row.get(6)?,
            y: row.get(7)?,
            triplet_id: row.get(8)?,
        };

        entity_map.insert(entity.entity_id.into(), entity);
    }

    Ok(entity_map)
}

pub(crate) fn select_hash_item(
    conn: &Connection,
    discourse_id: &i32,
    is_virtual: &i32,
) -> Result<Vec<HashItem>> {
    let mut stmt = conn.prepare(
        "select radius,angle,hash_type,discourse_id,excited_radius,excited_angle       
from hash_item order by order_by asc where discourse_id = ?1 and is_virtual = ?2",
    )?;
    let input_hash_item_iter = stmt.query_map(&[&discourse_id, &is_virtual], |row| {
        Ok(HashItem {
            radius: row.get(0)?,
            angle: row.get(1)?,
            hash_type: row.get(2)?,
            discourse_id: row.get(3)?,
            excited_radius: row.get(4)?,
            excited_angle: row.get(5)?,
        })
    })?;

    input_hash_item_iter.collect::<Result<Vec<HashItem>>>()
}

pub(crate) fn select_input_sentences_all(
    conn: &Connection,
    discourse_id: &i32,
) -> Result<Vec<InputSentence>> {
    let mut stmt = conn.prepare(
        "select sentence_id,discourse_id,is_question from input_sentence where discourse_id = ?1",
    )?;
    let input_sentence_iter = stmt.query_map(&[&discourse_id], |row| {
        Ok(InputSentence {
            sentence_id: row.get(0)?,
            discourse_id: row.get(1)?,
            is_question: row.get(2)?,
        })
    })?;

    input_sentence_iter.collect::<Result<Vec<InputSentence>>>()
}

pub(crate) fn select_input_triplets_by_sentence(
    conn: &Connection,
    sentence_id: &i32,
) -> Result<HashMap<i32, InputTriplet>> {
    let mut stmt = conn.prepare(
        "select triplet_id, sentence_id, tense, is_virtual, is_passive
from input_triplet where sentence_id = ?1",
    )?;
    let mut rows = stmt.query(&[&sentence_id])?;

    let mut triplet_map = HashMap::<i32, InputTriplet>::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_triplet = InputTriplet {
            triplet_id: row.get(0)?,
            sentence_id: row.get(1)?,
            tense: row.get(2)?,
            is_virtual: row.get(3)?,
            is_passive: row.get(4)?,
        };

        triplet_map.insert(input_triplet.triplet_id, input_triplet);
    }

    Ok(triplet_map)
}

pub(crate) fn select_new_def_isa(is_a: &i32) -> Result<Point2D> {
    let conn_meta = Connection::open(METAPATH)?;

    let isa = conn_meta.query_row(
        "select x, y, synset_id  from meaning_grid_item where synset_id = ?1",
        &[&is_a],
        |row| {
            Ok(Point2D {
                x: row.get(0)?,
                y: row.get(1)?,
            })
        },
    )?;

    if let Err(e) = conn_meta.close() {
        return Err(e.1);
    }

    Ok(isa)
}

pub(crate) fn select_input_discourse(
    conn: &Connection,
    discourse_id: &i32,
) -> Result<InputDiscourse> {
    let input_discourse: InputDiscourse = conn.query_row(
        "select discourse_id,hypernym_synset_id,document_hash,author_public_hash,author_title,author_first_name,
        author_middle_name,author_surname,author_zone,date_unix_epoch from input_discourse
        where discourse_id = ?1",
        &[&discourse_id],
        |row| Ok(InputDiscourse {
            discourse_id: row.get(0)?,
            hypernym_synset_id: row.get(1)?,
            document_hash: row.get(2)?,
            author_public_hash: row.get(3)?,
            author_title: row.get(4)?,
            author_first_name: row.get(5)?,
            author_middle_name: row.get(6)?,
            author_surname: row.get(7)?,
            author_zone: row.get(8)?,
            date_unix_epoch: row.get(9)?,
            x: row.get(10)?,
            y: row.get(11)?,
        }),
    )?;

    Ok(input_discourse)
}

pub(crate) fn select_input_discourse_title(
    conn: &Connection,
    discourse_id: &i32,
) -> Result<Vec<InputWord>> {
    let mut stmt = conn.prepare(
        "select w.word_id, w.sentence_id, w.synset_id, w.index_of_word,  
w.lexeme, w.instance_name, w.instance_index, w.pos, w.x, w.y, w.is_transition, w.new_word_id
from input_word w
inner join input_discourse_title d on d.word_id = w.word_id
	where w.discourse_id = ?1",
    )?;
    let mut rows = stmt.query(&[&discourse_id])?;

    let mut word_vec = Vec::<InputWord>::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word = InputWord {
            word_id: row.get(0)?,
            sentence_id: row.get(1)?,
            synset_id: row.get(2)?,
            index_of_word: row.get(3)?,
            lexeme: row.get(4)?,
            instance_name: row.get(5)?,
            instance_index: row.get(6)?,
            pos: row.get(7)?,
            x: row.get(8)?,
            y: row.get(9)?,
            is_transition: row.get(10)?,
            new_word_id: row.get(11)?,
        };

        word_vec.push(input_word);
    }

    Ok(word_vec)
}

pub(crate) fn select_input_words_by_sentence(
    conn: &Connection,
    sentence_id: &i32,
) -> Result<HashMap<i32, InputWord>> {
    let mut stmt = conn.prepare(
        "select w.word_id, w.sentence_id, w.synset_id, w.index_of_word,  
w.lexeme, w.instance_name, w.instance_index, w.pos, w.x, w.y, w.is_transition, w.new_word_id
from input_sentence s1
inner join input_triplet t on t.sentence_id = s1.sentence_id
inner join input_section s2 on s2.triplet_id = t.triplet_id
inner join input_word w on w.word_id = s3.word_id
	where s1.sentence_id = ?1 and w.new_word_id > 0",
    )?;
    let mut rows = stmt.query(&[&sentence_id])?;

    let mut word_map = HashMap::<i32, InputWord>::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word = InputWord {
            word_id: row.get(0)?,
            sentence_id: row.get(1)?,
            synset_id: row.get(2)?,
            index_of_word: row.get(3)?,
            lexeme: row.get(4)?,
            instance_name: row.get(5)?,
            instance_index: row.get(6)?,
            pos: row.get(7)?,
            x: row.get(8)?,
            y: row.get(9)?,
            is_transition: row.get(10)?,
            new_word_id: row.get(11)?,
        };

        word_map.insert(input_word.word_id, input_word);
    }

    Ok(word_map)
}

pub(crate) fn select_input_words_by_triplet(
    conn: &Connection,
    triplet_id: &i32,
) -> Result<Vec<InputWord>> {
    let mut stmt = conn.prepare(
        "select w.word_id, w.sentence_id, w.synset_id, w.index_of_word,  
w.lexeme, w.instance_name, w.instance_index, w.pos, w.x, w.y, w.is_transition, w.new_word_id
from input_triplet t
inner join input_section s on s.triplet_id = t.triplet_id
inner join input_word w on w.word_id = s.word_id
where t.triplet_id = ?1 and w.new_word_id > 0",
    )?;
    let mut rows = stmt.query(&[&triplet_id])?;

    let mut word_vec = Vec::<InputWord>::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word = InputWord {
            word_id: row.get(0)?,
            sentence_id: row.get(1)?,
            synset_id: row.get(2)?,
            index_of_word: row.get(3)?,
            lexeme: row.get(4)?,
            instance_name: row.get(5)?,
            instance_index: row.get(6)?,
            pos: row.get(7)?,
            x: row.get(8)?,
            y: row.get(9)?,
            is_transition: row.get(10)?,
            new_word_id: row.get(11)?,
        };

        word_vec.push(input_word);
    }

    Ok(word_vec)
}

pub(crate) fn select_input_words_new_def(
    conn: &Connection,
    discourse_id: &i32,
    new_word_id: i32,
) -> Result<Vec<InputWord>> {
    let mut stmt = conn.prepare(
        "select w.word_id, w.sentence_id, w.synset_id, w.index_of_word,  
w.lexeme, w.instance_name, w.instance_index, w.pos, w.x, w.y, w.is_transition, w.new_word_id from input_word w
inner join input_new_word_def n on n.new_word_id = w.new_word_id
where s1.discourse_id = ?1 and w.new_word_id = ?2",
    )?;
    let mut rows = stmt.query(&[&discourse_id, &new_word_id])?;

    let mut word_vec = Vec::<InputWord>::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word = InputWord {
            word_id: row.get(0)?,
            sentence_id: row.get(1)?,
            synset_id: row.get(2)?,
            index_of_word: row.get(3)?,
            lexeme: row.get(4)?,
            instance_name: row.get(5)?,
            instance_index: row.get(6)?,
            pos: row.get(7)?,
            x: row.get(8)?,
            y: row.get(9)?,
            is_transition: row.get(10)?,
            new_word_id: row.get(11)?,
        };

        word_vec.push(input_word);
    }

    Ok(word_vec)
}

pub(crate) fn select_input_word_relation(
    conn: &Connection,
    p1: &i32,
) -> Result<HashMap<i32, InputWordRelation>> {
    let mut stmt = conn.prepare(
        "select word_id, word_id_modified, ud_relation
from input_word_relation where word_id = ?1",
    )?;
    let mut rows = stmt.query(&[&p1])?;

    let mut relation_map = HashMap::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_relation = InputWordRelation {
            word_id: row.get(0)?,
            word_id_modified: row.get(1)?,
            ud_relation: row.get(2)?,
        };

        relation_map.insert(input_word_relation.word_id, input_word_relation);
    }

    Ok(relation_map)
}

pub(crate) fn select_input_word_relation_by_sentence(
    conn: &Connection,
    p1: &i32,
) -> Result<Vec<InputWordRelation>> {
    let mut stmt = conn.prepare(
        "select r.word_id, r.word_id_modified, r.ud_relation
from input_word w
inner join input_word_relation r on r.word_id = w.word_id
where w.sentence_id = ?1 and w.new_word_id > 0",
    )?;
    let mut rows = stmt.query(&[&p1])?;

    let mut relation_vec = Vec::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_relation = InputWordRelation {
            word_id: row.get(0)?,
            word_id_modified: row.get(1)?,
            ud_relation: row.get(2)?,
        };

        relation_vec.push(input_word_relation);
    }

    Ok(relation_vec)
}

pub(crate) fn select_input_new_word_defs(
    conn: &Connection,
    discourse_id: i32,
) -> Result<Vec<InputNewWordDef>> {
    let mut stmt = conn.prepare(
        "select new_word_id, is_a, discourse_id, lexeme, x, y from input_new_word_def where discourse_id = ?1;",
    )?;
    let mut rows = stmt.query(&[&discourse_id])?;

    let mut new_words: Vec<InputNewWordDef> = Vec::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_feature = InputNewWordDef {
            new_word_id: row.get(0)?,
            hypernym_synset_id: row.get(1)?,
            discourse_id: row.get(2)?,
            lexeme: row.get(3)?,
            x: row.get(4)?,
            y: row.get(5)?,
        };

        new_words.push(input_word_feature);
    }

    Ok(new_words)
}

pub(crate) fn select_input_exempt_features_by_discourse_id(
    conn: &Connection,
    discourse_id: &i32,
) -> Result<Vec<InputExemptFeature>> {
    let mut stmt = conn.prepare(
        "select discourse_id, ud_feature, ud_feature_value from input_exempt_feature where discourse_id = ?1",
    )?;
    let mut rows = stmt.query(&[&discourse_id])?;

    let mut feature_vec: Vec<InputExemptFeature> = Vec::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_feature = InputExemptFeature {
            discourse_id: row.get(0)?,
            ud_feature: row.get(1)?,
            ud_feature_value: row.get(2)?,
        };

        feature_vec.push(input_word_feature);
    }

    Ok(feature_vec)
}

pub(crate) fn select_input_features_by_word_id(
    conn: &Connection,
    p1: &i32,
) -> Result<Vec<InputWordFeature>> {
    let mut stmt = conn.prepare(
        "select word_id, ud_relation, ud_feature, ud_feature_value from input_word_feature where word_id = ?1",
    )?;
    let mut rows = stmt.query(&[&p1])?;

    let mut feature_vec: Vec<InputWordFeature> = Vec::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_feature = InputWordFeature {
            word_id: row.get(0)?,
            ud_relation: row.get(1)?,
            ud_feature: row.get(2)?,
            ud_feature_value: row.get(3)?,
        };

        feature_vec.push(input_word_feature);
    }

    Ok(feature_vec)
}

pub(crate) fn select_input_features_by_sentence_and_section(
    conn: &Connection,
    section_id: &i32,
) -> Result<Vec<InputWordFeature>> {
    let mut stmt = conn.prepare(
        "select distinct f.word_id, f.ud_relation, f.ud_feature, f.ud_feature_value
from input_triplet t
inner join input_section s on s.triplet_id = t.triplet_id
inner join input_word w on w.word_id = s.word_id
inner join input_word_feature f on f.word_id = w.word_id
section_id = ?1 and w.new_word_id > 0",
    )?;
    let mut rows = stmt.query(&[&section_id])?;

    let mut feature_vec: Vec<InputWordFeature> = Vec::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_feature = InputWordFeature {
            word_id: row.get(0)?,
            ud_relation: row.get(1)?,
            ud_feature: row.get(2)?,
            ud_feature_value: row.get(3)?,
        };

        feature_vec.push(input_word_feature);
    }

    Ok(feature_vec)
}

pub(crate) fn select_input_features_by_section(
    conn: &Connection,
    word_id: &i32,
) -> Result<Vec<InputWordFeature>> {
    let mut stmt = conn.prepare(
        "select f2.word_id, f2.ud_relation, f2.ud_feature, f2.ud_feature_value
from input_word_feature f1
inner join input_section s on s.word_id = f1.word_id
inner join input_word_feature f2 on f2.word_id = s.word_id
where f1.word_id = ?1;",
    )?;
    let mut rows = stmt.query(&[&word_id])?;

    let mut feature_vec: Vec<InputWordFeature> = Vec::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_feature = InputWordFeature {
            word_id: row.get(0)?,
            ud_relation: row.get(1)?,
            ud_feature: row.get(2)?,
            ud_feature_value: row.get(3)?,
        };

        feature_vec.push(input_word_feature);
    }

    Ok(feature_vec)
}

pub(crate) fn select_input_features_by_triplet(
    conn: &Connection,
    triplet_id: &i32,
) -> Result<Vec<InputWordFeature>> {
    let mut stmt = conn.prepare(
        "select distinct f.word_id, f.ud_relation, f.ud_feature,  f.ud_feature_value
from input_triplet t
inner join input_section s on s.triplet_id = t.triplet_id
inner join input_word w on w.word_id = s.word_id
inner join input_word_feature f on f.word_id = w.word_id
t.triplet_id = ?1 and w.new_word_id > 0",
    )?;
    let mut rows = stmt.query(&[&triplet_id])?;

    let mut feature_vec: Vec<InputWordFeature> = Vec::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_feature = InputWordFeature {
            word_id: row.get(0)?,
            ud_relation: row.get(1)?,
            ud_feature: row.get(2)?,
            ud_feature_value: row.get(3)?,
        };

        feature_vec.push(input_word_feature);
    }

    Ok(feature_vec)
}

pub(crate) fn select_input_features_by_sentence_and_section_map(
    conn: &Connection,
    section_id: &i32,
) -> Result<HashMap<i32, Vec<InputWordFeature>>> {
    let mut stmt = conn.prepare(
        "select distinct f.word_id, f.ud_relation, f.ud_feature, f.ud_feature_value
from input_triplet t
inner join input_section s on s.triplet_id = t.triplet_id
inner join input_word w on w.word_id = s.word_id
inner join input_word_feature f on f.word_id = w.word_id
section_id = ?1 and w.new_word_id > 0",
    )?;
    let mut rows = stmt.query(&[&section_id])?;

    let mut feature_map: HashMap<i32, Vec<InputWordFeature>> = HashMap::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_feature = InputWordFeature {
            word_id: row.get(0)?,
            ud_relation: row.get(1)?,
            ud_feature: row.get(2)?,
            ud_feature_value: row.get(3)?,
        };

        if !feature_map.contains_key(&input_word_feature.word_id) {
            feature_map.insert(input_word_feature.word_id, Vec::<InputWordFeature>::new());
        }

        if let Some(val) = feature_map.get_mut(&input_word_feature.word_id) {
            val.push(input_word_feature);
        }
    }

    Ok(feature_map)
}

pub(crate) fn select_input_features_by_sentence_and_section_and_word(
    conn: &Connection,
    section_id: &i32,
    temporary_word_id: &i32,
) -> Result<Vec<InputWordFeature>> {
    let mut stmt = conn.prepare(
        "select distinct f.word_id, f.ud_relation, f.ud_feature, f.ud_feature_value
from input_triplet t
inner join input_section s on s.triplet_id = t.triplet_id
inner join input_word w on w.word_id = s.word_id
inner join input_word_feature f on f.word_id = w.word_id
section_id = ?1 and w.word_id != ?2 and w.new_word_id > 0",
    )?;
    let mut rows = stmt.query(&[&section_id, &temporary_word_id])?;

    let mut feature_vec: Vec<InputWordFeature> = Vec::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let input_word_feature = InputWordFeature {
            word_id: row.get(0)?,
            ud_relation: row.get(1)?,
            ud_feature: row.get(2)?,
            ud_feature_value: row.get(3)?,
        };

        feature_vec.push(input_word_feature);
    }

    Ok(feature_vec)
}

pub(crate) fn select_input_section_all_ranked(
    conn: &Connection,
    triplet_id: &i32,
) -> Result<Vec<InputSection>> {
    let mut stmt = conn.prepare(
        "select t.section_id,
       t.triplet_id,
       t.word_id,
       t.section_type,
       tbl.grade
  from input_section t
  -- apply rules
  inner join input_word w on w.word_id = t.word_id and w.new_word_id > 0
  inner join input_word_relation r on r.word_id = t.word_id 
 left join
 (
    select ud_relation, grade from
    ( 
        select 'nsubj' ud_relation, 40 grade
        union
        select 'obj', 40
        union
        select 'iobj', 40
        union
        select 'csubj', 39
        union
        select 'ccomp', 39
        union
        select 'xcomp', 39
        union
        select 'nmod', 38
        union
        select 'appos', 38
        union
        select 'nummod', 38
        union
        select 'amod', 38
        union
        select 'acl', 38
    )
) tbl on instr(r.ud_relation, tbl.ud_relation) > 0 where t.triplet_id = ?1
order by t.section_type, tbl.grade desc",
    )?;
    let input_section_iter = stmt.query_map(&[&triplet_id], |row| {
        Ok(InputSection {
            section_id: row.get(0)?,
            triplet_id: row.get(1)?,
            word_id: row.get(2)?,
            section_type: row.get(3)?,
        })
    })?;

    input_section_iter.collect::<Result<Vec<InputSection>>>()
}

/// Used to pre-load meaning grid items as lazy_static.
pub(crate) fn get_meaning_grid() -> Result<HashMap<i32, MeaningGridItem>> {
    let conn_meta = Connection::open(METAPATH)?;
    let meaning_grid_items: HashMap<i32, MeaningGridItem> = select_meaning_grid_all(&conn_meta)?;

    if let Err(e) = conn_meta.close() {
        return Err(e.1);
    }

    Ok(meaning_grid_items)
}

pub(crate) fn select_meaning_grid_all(conn: &Connection) -> Result<HashMap<i32, MeaningGridItem>> {
    let mut stmt = conn.prepare("select x,y,synset_id from meaning_grid_item")?;
    let mut rows = stmt.query(NO_PARAMS)?;

    let mut meaning_map = HashMap::new();

    while let Ok(Some(result_row)) = rows.next() {
        let row = result_row;
        let meaning_item = MeaningGridItem {
            x: row.get(0)?,
            y: row.get(1)?,
            synset_id: row.get(2)?,
        };

        meaning_map.insert(meaning_item.synset_id, meaning_item);
    }

    Ok(meaning_map)
}

pub(crate) fn select_meaning_grid_max_xy() -> Result<(f64, f64)> {
    let conn_meta = Connection::open(METAPATH)?;

    let max_xy = conn_meta.query_row(
        "select max(x), max(y) from meaning_grid_item",
        NO_PARAMS,
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    if let Err(e) = conn_meta.close() {
        return Err(e.1);
    }

    Ok(max_xy)
}

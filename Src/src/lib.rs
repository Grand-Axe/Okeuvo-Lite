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

#![allow(dead_code)]
mod data;
mod math;
mod utils;
use crate::data::{
    insert_hash, insert_or_update_entity, insert_unit_tensor, insert_unit_tensor_ethereal,
    select_entity, select_hash_item, select_input_discourse, select_input_discourse_title,
    select_input_exempt_features_by_discourse_id, select_input_features_by_section,
    select_input_features_by_sentence_and_section_and_word, select_input_features_by_triplet,
    select_input_features_by_word_id, select_input_new_word_defs, select_input_section_all_ranked,
    select_input_sentences_all, select_input_triplets_by_sentence,
    select_input_word_relation_by_sentence, select_input_words_by_sentence,
    select_input_words_new_def, select_meaning_grid_max_xy, select_new_def_isa, select_unit_tensor,
    select_unit_tensor_centrality, update_input_new_word_def, update_unit_tensor_ethereal, Entity,
    ExcitationData, HashItem, HashItemFormatted, InputDiscourse, InputExemptFeature,
    InputNewWordDef, InputSection, InputSentence, InputTriplet, InputWord, InputWordFeature,
    InputWordRelation, UnitTensor,
};
use crate::math::{cartesian_to_polar, vector_addition_2d, vector_magnitude_2d, Point2D, Vector2D};
use crate::utils::{
    calculate_convex_hull, distance, point_vec_to_position_vec_2d, position_vec_to_point_vec_2d,
};
use rusqlite::Connection;
use std::collections::HashMap;

/// Path to the input database which contains raw triplets and triplet-word relations.
const INPUTPATH: &str = "./storage/input.db";
/// Path to the output database that contains an encoding of the input as "hash items".
const OUTPUTPATH: &str = "./storage/output.db";
/// Path to the metadata database whose contents include the meaning grid.
const METAPATH: &str = "./storage/metadata.db";
const TOLERANCE: f64 = 1.0e-6;

extern crate rusqlite;

/// Creates a cross platform friendly root.
/// Useful for database files.
/// Not yet used ***.
fn path_root() -> String {
    let mut result: String = "".to_string();

    if let Ok(dir) = std::env::current_exe() {
        //let dir = std::path::PathBuf::from("./");

        if let Ok(canonicalized) = std::fs::canonicalize(&dir) {
            if let Ok(path_string) = canonicalized.into_os_string().into_string() {
                if let Some(start_index) =
                    path_string.chars().position(|x| x.is_alphabetic() == true)
                {
                    result = path_string
                        .chars()
                        .skip(start_index)
                        .take(path_string.chars().count() - start_index)
                        .collect();
                    
                    return result
                }
            }
        }
    }

    result
}

/// Encodes a discourse (or communication).
/// It is the prelimnary step to hashing.
/// The resulting encoding is dumped in output.db as a set of "HashItem's".
/// Hashing methods work off this encoding.
pub extern "C" fn encode_discourse(
    discourse_id: &i32,
    agrees_to_the_creed: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check that the user has agreed to The Creed.
    // Stop processing with an error if they haven't.
    // The user interface must present a choice whose value we use here.
    if !agrees_to_the_creed {
        Err("You must agree to The Creed to continue".to_string())?
    }

    // (Step 1)--------------------------------------- Process Input ---------------------------------------\\

    let conn_input = Connection::open(INPUTPATH)?;
    let conn_output = Connection::open(OUTPUTPATH)?;

    let mut sentences_and_words: HashMap<i32, Vec<InputWord>> = HashMap::new();

    // Fetch sentences for the current discourse.
    let sentences_vec: Vec<InputSentence> = select_input_sentences_all(&conn_input, discourse_id)?;

    // Get universal features that are exempt from virtuality checks.
    let exempt_features: Vec<InputExemptFeature> =
        select_input_exempt_features_by_discourse_id(&conn_input, discourse_id)?;

    // Iterate all sentences in the current discourse.
    for sentence in &sentences_vec {
        let sentence_id: i32 = sentence.sentence_id;

        // Grab input words for sentence.
        // For use in building output.
        let input_words: HashMap<i32, InputWord> =
            select_input_words_by_sentence(&conn_input, &sentence_id)?;
        // Get values as Vec<InputWord> for storage in sentences_and_words.
        let input_word_vec: Vec<InputWord> = input_words.values().cloned().collect();
        sentences_and_words.insert(sentence_id, input_word_vec);

        // Get triplets for current sentence.
        let triplets_by_sentence: HashMap<i32, InputTriplet> =
            select_input_triplets_by_sentence(&conn_input, &sentence_id)?;

        let mut triplets_pruned: Vec<InputTriplet> = Vec::new();
        let mut sections_pruned: HashMap<i32, (i32, i32, i32)> = HashMap::new();

        // Eliminate duplicate sections.
        for triplet in &triplets_by_sentence {
            // Fetch subject, predicate, object for this triplet as a hash map in the format:
            // HashMap<triplet_id, (word_id, word_id, word_id)>.
            let mut subject_predicate_object: HashMap<i32, (i32, i32, i32)> =
                get_subject_predicate_object(&conn_input, &triplet.0)?;

            // Swap subject and object word_id's if the triplet is in passive voice.
            match (triplet.1).is_passive {
                true =>
                // Swap subject and object word_ids.
                {
                    swap_passive_subject_object(
                        &mut subject_predicate_object,
                        &(triplet.1).triplet_id,
                    )?
                }
                false => {}
            };

            let mut contains: bool = false;

            for item in &triplets_pruned {
                if item.triplet_id == (triplet.1).triplet_id {
                    contains = true;
                    break;
                }
            }

            if !contains {
                let triplet_current = InputTriplet { ..*triplet.1 };
                triplets_pruned.push(triplet_current);
                sections_pruned.insert(*triplet.0, subject_predicate_object[triplet.0]);
            }
        }

        // Sort triplets_pruned by triplet_id (ascending) so we can maintain chronological order.
        // For use in building output.
        triplets_pruned.sort_by(|a, b| a.triplet_id.cmp(&b.triplet_id));

        // (Step 2)--------------------------------------- Begin Ouput ---------------------------------------\\

        // Get excitation value.
        // Returned as Vec<(subject word_id, excitation value)>.
        let excitation_data: (i32, ExcitationData) = get_excitation(&input_words, &sections_pruned);

        // Tracks changes to variable, is_virtual_triplet.
        let mut is_virtual_triplet_tracker = false;
        let mut last_unit_tensor_index = -1;
        let mut last_ethereal_branch_index = -1;

        for triplet in &triplets_pruned {
            let tense: i32 = triplet.tense;
            let triplet_id: i32 = triplet.triplet_id;
            let sections = sections_pruned[&triplet_id];

            let subject_focus: &InputWord = &input_words[&sections.0];
            let predicate_focus: &InputWord = &input_words[&sections.1];
            let object_focus: &InputWord = &input_words[&sections.2];

            let excitation: f64 = (excitation_data.1).magnitude;

            // Create Entities for the focii of each triplet section.
            // An entity can be an instance (in which case it has an
            // instance_index, and possibly an instance_name); an entity
            // can also be anonymous (marked by lacking both
            // instance_index, and instance_name).
            let subject_entity: Entity =
                create_entity(&subject_focus, &excitation, &triplet_id, &discourse_id);
            let predicate_entity: Entity = create_entity(
                &predicate_focus,
                &(predicate_focus.x * predicate_focus.y),
                &triplet_id,
                discourse_id,
            );
            let object_entity: Entity = create_entity(
                &object_focus,
                &(object_focus.x * object_focus.y),
                &triplet_id,
                discourse_id,
            );

            // Insert or update new Entities for the focii of each triplet sections.
            // There is a unique constraint on the database field, instance_index to ensure
            // that attempts to insert duplicates of this column are ignored.
            let subject_entity_id: i64 = insert_or_update_entity(&conn_output, &subject_entity)?;
            let predicate_entity_id: i64 =
                insert_or_update_entity(&conn_output, &predicate_entity)?;
            let object_entity_id: i64 = insert_or_update_entity(&conn_output, &object_entity)?;

            // Every object is the location of an event. Locations are of varying types
            // so, test the current object - 3rd (rightmost) section of triplet - to
            // determine its location type.
            // 1 = virtual location (the default),
            // 2 = geographic location,
            // 3 = temporal location.
            // 4 = temporal location (terminative).
            let location_type: i32 = get_location_type(&conn_input, &triplet_id, &sentence_id)?;

            // Set where and when entities based on location_type.
            let mut where_entity_id = -1;
            let mut when_entity_id = -1;
            match location_type {
                2 => where_entity_id = object_entity_id,
                3 | 4 => when_entity_id = object_entity_id,
                _ => {}
            }

            // Determine if triplet is virtual or real.
            // A virtual triplet is one whose mood is not indicative,
            // such as conditionals and future tense events for example.
            // Each virtual is a new dimension and as many as is
            // necessary should be spawned.
            let is_virtual_and_mood: (bool, String) = match exempt_features.len() {
                0 => is_virtual(&conn_input, triplet, &vec![])?,
                _ => is_virtual(&conn_input, triplet, &exempt_features)?,
            };

            let is_virtual_triplet: bool = is_virtual_and_mood.0;
            let mood: String = is_virtual_and_mood.1;

            // Important consideration -
            // Check for negations.

            // Create UnitTensor for current interaction.
            // Each UnitTensor is a node in the graph of
            // the current discourse. It has a set of
            // subnodes (subject, predicate and object).
            // It can be real or virtual.
            // Its edges are chronology (tense and order
            // of index, entity_id), instance entity,
            // anonymous entity and branch and rejoin
            // indices of virtual UnitTensor's.
            let unit_tensor: UnitTensor = create_unit_tensor(
                &sentence_id,
                discourse_id,
                &object_entity_id,
                &subject_entity_id,
                &where_entity_id,
                &when_entity_id,
                &predicate_entity_id,
                &tense,
                &mood,
                &(excitation_data.1).x,
                &(excitation_data.1).y,
            );
            // Insert the tensor.
            // It's ok if it goes in the database as a duplicate.
            // We can select distinct values.
            let unit_tensor_id: i64 = insert_unit_tensor(&conn_output, &unit_tensor)?;

            // If the triplet is virtual and not part of an existing block, create a UnitTensorEtherealDef to record
            // the current index at which it branches off the main UnitTensor column.
            // The test to determine if the triplet is transitional has been commented out for now.
            if is_virtual_triplet && is_virtual_triplet != is_virtual_triplet_tracker
            // && !is_triplet_transitional(&input_words)
            {
                last_ethereal_branch_index =
                    insert_unit_tensor_ethereal(&conn_output, &unit_tensor_id)?;
            }

            // If the triplet is virtual and not part of the current block, update its UnitTensorEtherealDef to record
            // the previous index as the index at which it rejoins the main UnitTensor column.
            // The test to determine if the triplet is transitional has been commented out for now.
            if (!is_virtual_triplet
                && is_virtual_triplet != is_virtual_triplet_tracker
                && last_ethereal_branch_index > -1)
            // && !is_triplet_transitional(&input_words)
            {
                update_unit_tensor_ethereal(
                    &conn_output,
                    &last_unit_tensor_index,
                    &unit_tensor_id,
                )?;
            }

            // Set is_virtual_triplet_tracker to current is_virtual_triplet value.
            match is_virtual_triplet {
                true => is_virtual_triplet_tracker = true,
                false => is_virtual_triplet_tracker = false,
            }

            last_unit_tensor_index = unit_tensor_id;
        }
    }

    // (Step 3)--------------------------------------- Vector Representation ---------------------------------------\\

    // Fetch the discourse title.
    let discourse_title: Vec<InputWord> = select_input_discourse_title(&conn_input, &discourse_id)?;

    if discourse_title.len() > 0 {
        // Retrieve discourse properties.
        let discourse: InputDiscourse = select_input_discourse(&conn_input, &discourse_id)?;

        let degree_centralities_real: HashMap<i64, i32> =
            select_unit_tensor_centrality(&conn_input, &discourse_id, &false)?;

        let degree_centralities_virtual: HashMap<i64, i32> =
            select_unit_tensor_centrality(&conn_input, &discourse_id, &true)?;

        // Get unit tensors for real events.
        let unit_tensors_real: Vec<UnitTensor> =
            select_unit_tensor(&conn_output, discourse_id, &false)?;

        // Get unit tensors for virtual events.
        let unit_tensors_virtual: Vec<UnitTensor> =
            select_unit_tensor(&conn_output, discourse_id, &false)?;

        let discourse_entities: HashMap<i64, Entity> = select_entity(&conn_output, discourse_id)?;

        let hash_real: (HashItem, Vec<HashItem>) = get_hash_raw(
            &discourse,
            &degree_centralities_real,
            &unit_tensors_real,
            &sentences_vec,
            &discourse_entities,
            &1,
        )?;

        insert_hash(&conn_output, &hash_real.0, &0, &false)?;
        let mut i: i32 = 0;
        for hash_item in &hash_real.1 {
            insert_hash(&conn_output, &hash_item, &i, &false)?;
            i = i + 1;
        }

        let hash_virtual: (HashItem, Vec<HashItem>) = get_hash_raw(
            &discourse,
            &degree_centralities_virtual,
            &unit_tensors_virtual,
            &sentences_vec,
            &discourse_entities,
            &2,
        )?;

        // Insert the discource hypernym portion of the hash.
        insert_hash(&conn_output, &hash_virtual.0, &0, &true)?;
        // Insert the rest of the hash.
        i = 0;
        for hash_item in &hash_virtual.1 {
            insert_hash(&conn_output, &hash_item, &i, &true)?;
            i = i + 1;
        }
    }

    // All done!
    if let Err(e) = conn_input.close() {
        Err(e.1.to_string())?
    }
    if let Err(e) = conn_output.close() {
        Err(e.1.to_string())?
    }

    Ok(())
}

/// Creates the discourse hash.
/// Invoke after calling encode_discourse.
pub extern "C" fn get_hash(
    discourse_id: &i32,
    agrees_to_the_creed: &bool,
    is_virtual: &bool,
) -> Result<String, Box<dyn std::error::Error>> {
    // Check that the user has agreed to The Creed.
    // Stop processing with an error if they haven't.
    // The user interface must present a choice whose value we use here.
    if !agrees_to_the_creed {
        Err("You must agree to The Creed to continue".to_string())?
    }
    // Allow a hash that's 199 characters long
    // That's n * element length + n - 1
    // => 20 * 9 + 20 - 1 = 199.
    let max_rows: usize = 20;

    let mut virtual_marker = 1;
    if *is_virtual {
        virtual_marker = 0;
    }

    let conn_output = Connection::open(OUTPUTPATH)?;
    let hash_item_vec: Vec<HashItem> =
        select_hash_item(&conn_output, &discourse_id, &virtual_marker)?;

    let preformatted_hash_items: Vec<HashItemFormatted> = preformat_hash_items(&hash_item_vec)?;

    let formatted_hash_items: Vec<String> =
        round_pad_stringulate(&preformatted_hash_items, &max_rows)?;

    if let Err(e) = conn_output.close() {
        Err(e.1.to_string())?
    }

    // Complete the hash by joining the vector to string with a "-" separator.
    let result: String = formatted_hash_items.join("-");

    Ok(result)
}

/// Gets a unique angle for each vector as well as an indicator
/// for the direction in which that angle falls relative to the
/// full span of the x axis.
/// It first creates a unique triangle for each vector. The base
/// of this triangle is the span of the x axis.
/// The apex of this triangle is the unique angle mentioned above.
/// The indicator of the direction in which the apex angle falls
/// is a function of the length of the limbs between the apex
/// angle and the triangle vertices on the x axis.
fn preformat_hash_items(
    hash_item_vec: &Vec<HashItem>,
) -> Result<Vec<HashItemFormatted>, Box<dyn std::error::Error>> {
    // Note that calculations are done with the direction set as "max x --> origin".
    let mut result: Vec<HashItemFormatted> = Vec::new();

    let max_xy: (f64, f64) = select_meaning_grid_max_xy()?;

    for item in hash_item_vec {
        // Apply cosine formula to get length of last triangle side.
        let a1_squared: f64 = (item.radius.powi(2) + max_xy.0.powi(2))
            - (2.0 * item.radius * max_xy.0 * item.angle.cos());
        let a1 = a1_squared.sqrt();

        // Apply cosine formula to get length of last triangle side for (excited).
        let a2_squared: f64 = (item.excited_radius.powi(2) + max_xy.0.powi(2))
            - (2.0 * item.excited_radius * max_xy.0 * item.excited_angle.cos());
        let a2 = a2_squared.sqrt();

        // Apply cosine formula to get angle opposite item.angle along x (max_xy.0).
        let angle1_opp_cos: f64 = (a1_squared + max_xy.0.powi(2) - item.radius.powi(2))
            / (2.0 * a1_squared * max_xy.0.powi(2));
        let angle1_opp: f64 = angle1_opp_cos.acos();

        // Apply cosine formula to get angle opposite excited_angle along x (max_xy.0).
        let angle2_opp_cos: f64 = (a2_squared + max_xy.0.powi(2) - item.excited_radius.powi(2))
            / (2.0 * a2_squared * max_xy.0.powi(2));
        let angle2_opp: f64 = angle2_opp_cos.acos();

        // Get upper angle.
        let upper_angle = std::f64::consts::PI - item.angle - angle1_opp;

        // Get upper angle (excited).
        let upper_angle_excited = std::f64::consts::PI - item.excited_angle - angle2_opp;

        // Compute lean direction (for unexcited and excited).
        // 0 = left leaning (toward origin).
        // 1 = no lean.
        // 2 = right leaning (away from origin).
        let hash_item_formatted = HashItemFormatted {
            right_leaning: match a1 {
                a if a > item.radius => 0,
                a if a < item.radius => 2,
                _ => 1,
            },
            upper_angle: upper_angle,
            right_leaning_excited: match a2 {
                a if a > item.excited_radius => 0,
                a if a < item.excited_radius => 2,
                _ => 1,
            },
            upper_angle_excited: upper_angle_excited,
        };

        result.push(hash_item_formatted);
    }

    Ok(result)
}

/// Does the final formatting of hashes.
/// Rounds floats that denote polar angles and radii to integers.
/// Truncates the hash vector to a fixed length.
/// Pads resulting integers less than ten with a leading "0",
/// so that each element is two characters long.
/// Inserts a "!" between each element of the sections of the hash.
fn round_pad_stringulate(
    formatted_hash_items: &Vec<HashItemFormatted>,
    max_rows: &usize,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = Vec::new();
    let length = &formatted_hash_items.len();

    for i in 1..*length {
        let item = &formatted_hash_items[i];
        let mut hash_local = item.right_leaning.to_string();
        hash_local.push_str(&"!".to_string());
        // Ensure upper_angle is two digits long.
        let upper_angle = item.upper_angle.round() as i32;
        if upper_angle < 10 {
            hash_local.push_str("0");
        }
        hash_local.push_str(&upper_angle.to_string());
        hash_local.push_str(&"!".to_string());
        hash_local.push_str(&item.right_leaning_excited.to_string());
        hash_local.push_str(&"!".to_string());
        // Ensure excited upper_angle is two digits long.
        let upper_angle_excited = item.upper_angle.round() as i32;
        if upper_angle_excited < 10 {
            hash_local.push_str("0");
        }
        hash_local.push_str(&item.upper_angle_excited.round().to_string());

        result.push(hash_local);

        // Truncate vectors that are longer than max_rows,
        if i == *max_rows {
            break;
        }
    }

    // Pad vectors that are shorter than max_rows,
    // replace all values with "#" to signify that
    // they are empty.
    if result.len() < *max_rows {
        let start: usize = result.len();
        for _i in start..*max_rows {
            let empty: String = "#!##!#!##".to_string();
            result.push(empty);
        }
    }

    Ok(result)
}

/// Gets a vector collection of entities sorted in descending order by aggregate excitation.
/// This vector of entities will be the input to the hash function (get_hash).
/// Only entities that are either coreferenced or have a degree centrality above zero are considered.
fn get_hash_raw(
    discourse: &InputDiscourse,
    degree_centralities: &HashMap<i64, i32>,
    unit_tensors: &Vec<UnitTensor>,
    sentences_vec: &Vec<InputSentence>,
    discourse_entities: &HashMap<i64, Entity>,
    hash_type: &i32,
) -> Result<(HashItem, Vec<HashItem>), Box<dyn std::error::Error>> {
    // Get the hash_item for the hypernym of this dicourse.
    // This item will be at the beginning of the discourses hash.

    // Possible future consideration -
    // 1. areal_jaccard (shared properties).

    let mut hash_item_vec: Vec<HashItem> = Vec::new();
    let mut excitation_totals: HashMap<i64, Vector2D> = HashMap::new();

    for item in unit_tensors {
        // Disallow questions.
        let mut is_question = false;
        for sentence in sentences_vec {
            if sentence.sentence_id == item.sentence_id {
                if sentence.is_question {
                    is_question = true;
                    break;
                }
            }
        }
        if is_question {
            continue;
        }

        // Eject items without a centrality value.
        // We might use a threshold centrality in the future.
        let mut has_centrality_value = false;
        if degree_centralities.contains_key(&item.object_entity_id) {
            has_centrality_value = true;
        }
        if degree_centralities.contains_key(&item.predicate_entity_id) {
            has_centrality_value = true;
        }
        if degree_centralities.contains_key(&item.subject_entity_id) {
            has_centrality_value = true;
        }
        if degree_centralities.contains_key(&item.when_entity_id) {
            has_centrality_value = true;
        }
        if degree_centralities.contains_key(&item.where_entity_id) {
            has_centrality_value = true;
        }
        if !has_centrality_value {
            continue;
        }

        let vector = Vector2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D {
                x: item.excited_x,
                y: item.excited_y,
            },
        };

        // Accummulate excitation values for each unit tensor.
        if excitation_totals.contains_key(&item.subject_entity_id) {
            excitation_totals.insert(
                item.subject_entity_id,
                vector_addition_2d(&excitation_totals[&item.subject_entity_id], &vector),
            );
        } else {
            excitation_totals.insert(item.subject_entity_id, vector);
        }
    }

    let mut summed_excitation = Vector2D {
        start: Point2D { x: 0.0, y: 0.0 },
        end: Point2D { x: 0.0, y: 0.0 },
    };

    for item in &excitation_totals {
        // Get the current items excitation vector
        // so that we can add it to the excitation total.
        if let Some(entity) = discourse_entities.get(&item.0) {
            let item_complexity_plus_excitation: Vector2D = vector_addition_2d(
                &Vector2D {
                    start: Point2D { x: 0.0, y: 0.0 },
                    end: Point2D {
                        x: entity.x,
                        y: entity.y,
                    },
                },
                item.1,
            );

            summed_excitation =
                vector_addition_2d(&summed_excitation, &item_complexity_plus_excitation);

            // Get the polar coordinates of the current entity.
            let entity_polar_coord: (f64, f64) = cartesian_to_polar(&Point2D {
                x: entity.x,
                y: entity.y,
            });

            // Get the excitation polar coordinates of the current entity.
            let excitation_polar_coord: (f64, f64) = cartesian_to_polar(&Point2D {
                x: entity.x,
                y: entity.y,
            });

            let hash_item = HashItem {
                radius: entity_polar_coord.0,
                angle: entity_polar_coord.1,
                hash_type: *hash_type,
                discourse_id: discourse.discourse_id,
                excited_radius: excitation_polar_coord.0,
                excited_angle: excitation_polar_coord.1,
            };

            hash_item_vec.push(hash_item);
        }
    }

    // Process discourse hypernym.
    let isa_polar_coord: (f64, f64) = cartesian_to_polar(&Point2D {
        x: discourse.x,
        y: discourse.y,
    });

    let mut isa_hash_item = HashItem {
        radius: isa_polar_coord.0,
        angle: isa_polar_coord.1,
        hash_type: *hash_type,
        discourse_id: discourse.discourse_id,
        excited_radius: isa_polar_coord.0,
        excited_angle: isa_polar_coord.1,
    };

    // Create vector for discourse hypernym.
    let isa_vector = Vector2D {
        start: Point2D { x: 0.0, y: 0.0 },
        end: Point2D {
            x: discourse.x,
            y: discourse.y,
        },
    };

    // Add summed_excitation to the discourse hypernym vector.
    let isa_hash_item_excitation: Vector2D = vector_addition_2d(&isa_vector, &summed_excitation);
    let isa_excitation_polar_coord: (f64, f64) = cartesian_to_polar(&Point2D {
        x: isa_hash_item_excitation.end.x,
        y: isa_hash_item_excitation.end.y,
    });
    // Set the hypernyms excitation polar coordinates.
    isa_hash_item.excited_radius = isa_excitation_polar_coord.0;
    isa_hash_item.excited_angle = isa_excitation_polar_coord.1;

    // Sort hash_item_vec in descending order by excited_radius.
    hash_item_vec.sort_by(|a, b| {
        b.excited_radius
            .partial_cmp(&a.excited_radius)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let result = (isa_hash_item, hash_item_vec);

    Ok(result)
}

/// Not yet implemented.
/// Tests if a triplet is transitional.
/// This is particularly important as part of a test to determine
/// if a triplet is a continuation of an existing information block
/// that is either real or virtual.
/// It works primarily by detecting mood and coreference boundaries
/// - these are clauses in which the moods or coreferences being
/// considered first occur, or have are re-established or
/// where there are breaks in their use.
///
/// Notes:
/// (a) Regardless of language, all meanings reduce to the language fractal - [Subject [Predicate [Object]]]
///
/// (b) Word groups as functions.
/// 1. Cats are smaller than dogs.
/// 2. Even so, a cornered cat can frighten a giant dog.
///
/// Let evenSo(x) = "Even if" [x].
///
/// Then we can do away with needing to write the full sentence (1), instead we can replace sentence (2) with;
/// evenSo(1), a conered cat can frighten a giant dog.
///
/// 1. He had come there everyday since he was a boy.
/// 2. The man, now wizened with age, looked back wistfully.
///
/// Let the(y) = [y].
///
/// This leads to:
/// (1), now wizened with age, looked back wistfully.
///
/// By deduction, evenSo(x) = evenSo(the(y)), having a pattern;
///
/// outer_function(definite_article_function())
///
/// (c) Coreferences as links.
///
/// (d) Transition words and phrases.
/// Coverage of these is parchy in both parsing methods and tagging systems,
/// never extending beyond the clause or sentence.
/// In Universal Dependencies, transition words are only given the tag "mark"
/// in relation to their parent clause or sentence.
fn is_triplet_transitional(input_words: &HashMap<i32, InputWord>) -> bool {
    let mut result: bool = false;

    for item in input_words {
        if item.1.is_transition {
            result = true;
            break;
        }
    }

    result
}

/// Get the excitation for a triplet.
/// Excitation is the value of complexity added to
/// the subject in a specific interaction within
/// a broader comminication.
///
/// Return format: Vec<(subject word_id, excitation)>.
fn get_excitation(
    input_words: &HashMap<i32, InputWord>,
    sections_pruned: &HashMap<i32, (i32, i32, i32)>,
) -> (i32, ExcitationData) {
    let mut subject_word_id: i32 = -1;
    if let Some(pruned_row) = sections_pruned.iter().nth(0) {
        subject_word_id = (pruned_row.1).0;
    }

    let mut resultant = Vector2D {
        start: Point2D { x: 0.0, y: 0.0 },
        end: Point2D { x: 0.0, y: 0.0 },
    };

    // Sum up resultants.
    for word in input_words {
        // Don't add the subject vector.
        if (word.1).word_id == subject_word_id {
            continue;
        }

        let vector = Vector2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D {
                x: (word.1).x,
                y: (word.1).y,
            },
        };

        resultant = vector_addition_2d(&resultant, &vector);
    }

    let magnitude: f64 = vector_magnitude_2d(&resultant);
    let excitation = ExcitationData {
        magnitude,
        x: resultant.end.x,
        y: resultant.end.y,
    };

    let result: (i32, ExcitationData) = (subject_word_id, excitation);

    result
}

/// Creates a new Entity with entity_id set to -1.
fn create_entity(word: &InputWord, rank: &f64, triplet_id: &i32, discourse_id: &i32) -> Entity {
    let result = Entity {
        entity_id: -1,
        discourse_id: *discourse_id,
        synset_id: word.synset_id,
        instance_index: word.instance_index,
        word_id: word.word_id,
        rank: *rank,
        x: word.x,
        y: word.y,
        triplet_id: *triplet_id,
    };

    result
}

/// Creates a new UnitTensor with unit_tensor_id set to -1.
fn create_unit_tensor(
    sentence_id: &i32,
    discourse_id: &i32,
    object_entity_id: &i64,
    subject_entity_id: &i64,
    where_entity_id: &i64,
    when_entity_id: &i64,
    predicate_entity_id: &i64,
    tense: &i32,
    mood: &String,
    excited_x: &f64,
    excited_y: &f64,
) -> UnitTensor {
    let result = UnitTensor {
        unit_tensor_id: -1,
        sentence_id: *sentence_id,
        discourse_id: *discourse_id,
        object_entity_id: *object_entity_id,
        subject_entity_id: *subject_entity_id,
        where_entity_id: *where_entity_id,
        when_entity_id: *when_entity_id,
        predicate_entity_id: *predicate_entity_id,
        tense: *tense,
        mood: mood.clone(),
        excited_x: *excited_x,
        excited_y: *excited_y,
    };

    result
}

/// Swaps subject and object of passive sections.
/// The parameter section has signature
///
/// HashMap<triplet_id, (word_id, word_id, word_id)>.
///
/// The function returns a value with the same signature above, but wrapped in a Result.
fn swap_passive_subject_object(
    sections: &mut HashMap<i32, (i32, i32, i32)>,
    triplet_id: &i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let word_ids: (i32, i32, i32) = sections[&triplet_id];
    // Swap subject and object word_ids and update sections.
    sections.insert(*triplet_id, (word_ids.2, word_ids.1, word_ids.0));

    Ok(())
}

/*/// Swaps subject and object of passive sections.
/// NOTE: This method does not handle errors. Call only in methods that return std::result::Result.
fn swap_passive_subject_object(
    section: &Vec<InputSection>,
    triplet: &InputTriplet,
) -> Result<(), Box<std::error::Error>> {
    if triplet.is_passive {
        std::mem::swap(&mut section.get(0), &mut section.get(2));
    }
    Ok(())
}*/

/// Groups sections by subject, predicate and object.
/// A HashMap that has tuple of word_id's as key and a vector of triplet_id's as value is returned.
///
/// Result format: Result<HashMap<(word_id, word_id, word_id), Vec<triplet_id>>.
fn get_section_groups(
    subject_predicate_object: HashMap<i32, (i32, i32, i32)>,
) -> Result<HashMap<(i32, i32, i32), Vec<i32>>, Box<dyn std::error::Error>> {
    let mut result: HashMap<(i32, i32, i32), Vec<i32>> = HashMap::new();

    for item in &subject_predicate_object {
        if !result.contains_key(item.1) {
            let mut new_vec: Vec<i32> = Vec::new();
            new_vec.push(*item.0);
            result.insert(*item.1, new_vec);
        } else {
            if let Some(existing) = result.get_mut(item.1) {
                existing.push(*item.0);
            }
        }
    }

    Ok(result)
}

/// Yields the dominant word (subject, predicate or object) in each section of the section.
/// It returns a HashMap that has section_id's as keys and a tuple of word_id's as values.
///
/// Return format: HashMap<triplet_id, (word_id, word_id, word_id)>.
fn get_subject_predicate_object(
    conn: &Connection,
    triplet_id: &i32,
) -> Result<HashMap<i32, (i32, i32, i32)>, rusqlite::Error> {
    // result is a HashMap that has section_id's as keys and a tuple of word_id's as values.
    let mut result: HashMap<i32, (i32, i32, i32)> = HashMap::new();

    // Fetch all sections.
    let sections_ranked: Vec<InputSection> = select_input_section_all_ranked(&conn, &triplet_id)?;

    // Tracker to mark if best candidate word_id has been chosen for
    // each unique combination of section_id and section type.
    // Format - Vec<(section_id, section_type, word_id)>
    let mut processed_section_type: i32 = 0;
    let mut section = (-1, -1, -1);

    // sections_ranked is sorted so that the first item in each section_id-section_type group
    // is most likely the subject, predicate or object (depending on section type).
    // We will temporarily assign it to the sections tuple pending further best candidate checks.
    for item in &sections_ranked {
        if item.section_type <= processed_section_type {
            continue;
        }
        if item.section_type == 1 {
            section.0 =
                further_subject_predicate_object_checks(&conn, item.word_id, &item.section_id)?;
            processed_section_type = 1;
        }
        if item.section_type == 2 {
            section.1 =
                further_subject_predicate_object_checks(&conn, item.word_id, &item.section_id)?;
            processed_section_type = 1;
        }
        if item.section_type == 3 {
            section.2 =
                further_subject_predicate_object_checks(&conn, item.word_id, &item.section_id)?;
            break;
        }
    }

    result.insert(*triplet_id, section);

    Ok(result)
}

/// Returns the best candidate word_id for the focus of a triplet (subject, predicate or object).
///
/// Return format: Result<word_id, error>.
fn further_subject_predicate_object_checks(
    conn: &Connection,
    temporary_word_id: i32,
    section_id: &i32,
) -> Result<i32, rusqlite::Error> {
    let mut result: i32 = temporary_word_id;

    // Grade the feature groups.
    let mut weights: HashMap<String, i32> = HashMap::new();
    weights.insert("person".to_string(), 5);
    weights.insert("poss".to_string(), 4);
    weights.insert("prontype".to_string(), 3);
    weights.insert("reflex".to_string(), 2);
    weights.insert("animacy".to_string(), 1);

    let temporary_word_features: Vec<InputWordFeature> =
        select_input_features_by_word_id(&conn, &temporary_word_id)?;
    let mut temporary_word_max_weight: i32 = 0;

    // Get the maximum weight group for temporary_word_id's features.
    for feature in &temporary_word_features {
        if weights.contains_key(&feature.ud_feature.to_lowercase()) {
            if let Some(weight) = weights.get(&feature.ud_feature.to_lowercase()) {
                // Update temporary_word_max_weight if a greater weight is found.
                if weight > &temporary_word_max_weight {
                    temporary_word_max_weight = *weight;
                }
            }
        }
    }

    // Vector to hold candidates for focus of the section (feature.word_id, weight, feature_tag, feature_value).
    let mut candidates: Vec<(i32, i32, String, String)> = Vec::new();
    // Fetch features for all words in the section.
    let features_by_sentence_and_section: Vec<InputWordFeature> =
        select_input_features_by_sentence_and_section_and_word(
            &conn,
            &section_id,
            &temporary_word_id,
        )?;

    // Test for more suitable candidates for the focus of the current section by
    // comparing their feature group weights to temporary_word_max_weight.
    for feature in &features_by_sentence_and_section {
        let feature_tag: String = feature.ud_feature.to_lowercase();
        let feature_value: String = feature.ud_feature_value.to_lowercase();
        if weights.contains_key(&feature_tag) {
            if let Some(weight) = weights.get(&feature.ud_feature.to_lowercase()) {
                // Update candidates with current feature if its weight is
                // greater than or equal to temporary_word_max_weight.
                if weight >= &temporary_word_max_weight {
                    candidates.push((
                        feature.word_id,
                        *weight,
                        feature_tag.clone(),
                        feature_value.clone(),
                    ));
                }
            }
        }
    }

    // Sort candidates in descending order by weight group
    // to get most potent items for the section focus to the top.
    candidates.sort_by(|a, b| b.1.cmp(&a.1));

    for item in &candidates {
        if item.1 > temporary_word_max_weight {
            let mut can_proceed: bool = false;

            if item.2 == "prontype".to_string() && item.3 == "prs".to_string() {
                can_proceed = true;
            } /*else {
                can_proceed = true;
            }*/

            if can_proceed {
                result = item.0;
                break;
            }
        }

        // When weight = temporary_word_max_weight, we must check the individual feature values for primacy.
        if item.1 == temporary_word_max_weight {
            if item.2 == "person".to_string() {
                // Vaue gradient to represent grammatical person values, from 1st to 4th.
                let gradient: String = "12345".to_string();
                // Create a collection from the gradients chars.
                let mut iter = gradient.chars();
                // Convert the current items feature value into a char.
                if let Some(ch1) = item.3.chars().nth(0) {
                    // Get the position of the current items feature value (ch1) in the gradient.
                    if let Some(item_index) = iter.position(|x| x == ch1) {
                        for feature in &temporary_word_features {
                            // Find the feature value corresponding to "person" in temporary_word_features.
                            if feature.ud_feature == "person" {
                                // Get its position.
                                if let Some(temp_index) =
                                    feature.ud_feature_value.chars().position(|x| x == ch1)
                                {
                                    // set result to the word_id of the current item if
                                    // its person is less than the current item's.
                                    if item_index < temp_index {
                                        result = item.0;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            /* These don't matter when feature equals "person".
            if item.2 == "poss".to_string() {
                result = item.0;
                break;
            }

            if item.2 == "prontype".to_string() && item.3 == "prs".to_string() {
                result = item.0;
                break;
            }

            if item.2 == "reflex".to_string() {
                result = item.0;
                break;
            }*/

            if item.2 == "animacy".to_string() {
                // vector to be used to grade animacy types (by index).
                let animacy_vec: Vec<String> = vec![
                    "hum".to_string(),
                    "anim".to_string(),
                    "nhum".to_string(),
                    "inan".to_string(),
                ];

                // Get indices.
                if let Some(item_index) = animacy_vec.iter().position(|x| x.to_string() == item.3) {
                    let mut temp_index = 6;
                    for (i, feature) in temporary_word_features.iter().enumerate() {
                        if feature.ud_feature_value == "poss" {
                            temp_index = i;
                            break;
                        }
                    }

                    if item_index < temp_index {
                        result = item.0;
                        break;
                    }
                }
            }
        }
    }

    Ok(result)
}

/// Tests the 3rd (rightmost) section of triplet. It returns an integer in a Result.
/// The value of the integer depicts the location type.
/// 1 = virtual location (the default),
/// 2 = geographic location,
/// 3 = temporal location.
/// 4 = temporal location (terminative).
fn get_location_type(
    conn: &Connection,
    object_word_id: &i32,
    sentence_id: &i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut result: i32 = 1;

    let features_by_section: Vec<InputWordFeature> =
        select_input_features_by_section(&conn, &object_word_id)?;

    let input_word_relations: Vec<InputWordRelation> =
        select_input_word_relation_by_sentence(&conn, &sentence_id)?;

    for feature in &features_by_section {
        if feature.ud_feature.to_lowercase() == "case".to_string() {
            // Check if this feature has a connection to object_word_id.
            // If there is a connection, check if it satisfies location types that are of interest.
            for item in &input_word_relations {
                if (item.word_id == feature.word_id && item.word_id_modified == *object_word_id)
                    || (item.word_id_modified == feature.word_id && item.word_id == *object_word_id)
                {
                    if feature.ud_feature_value.to_lowercase() == "loc".to_string() {
                        result = 2;
                    }
                    if feature.ud_feature_value.to_lowercase() == "tem".to_string() {
                        result = 3;
                    }
                    if feature.ud_feature_value.to_lowercase() == "ter".to_string() {
                        result = 4;
                    }

                    break;
                }
            }
        }
    }

    Ok(result)
}

/// Checks if a triplet is virtual or real.
/// Returns a tuple, (is_virtual, mood), where mood is verb mood.
/// The parameter, allowed_tags exempts any triplet being
/// marked as virtual based on the presence of the allowed tag
/// in its "input_features.ud_feature_value" tags.
fn is_virtual(
    conn: &Connection,
    triplet: &InputTriplet,
    // Tags that might be exempt, such as imperative (imp).
    exempt_tags: &Vec<InputExemptFeature>,
) -> Result<(bool, String), Box<dyn std::error::Error>> {
    let mut found_virtual = false;
    let mut mood = "ind".to_string();

    let input_features: Vec<InputWordFeature> =
        select_input_features_by_triplet(&conn, &triplet.triplet_id)?;

    for feature in &input_features {
        // Return true for all items not marked "indicative" (mood).
        let relation_tag: String = feature.ud_relation.to_lowercase();
        let feature_tag: String = feature.ud_feature_value.to_lowercase();

        if relation_tag == "mood" && feature_tag != "ind" {
            for exemption in exempt_tags {
                if exemption.ud_feature.to_lowercase() != "mood" {
                    continue;
                }
                if exemption.ud_feature_value.to_lowercase() == feature_tag {
                    mood = String::from(&feature_tag);
                    break;
                }
            }

            found_virtual = true;
            break;
        }
    }
    Ok((found_virtual, mood))
}

/// Creates senses for batches of words that do not yet exist in the meaning grid.
/// Where a word sense does not exist on the meaning grid, it can be defined
/// using word senses that do.
///
/// The definition must be succinct and must also be a hyponym of an existing
/// sense which must also double as the subject.
/// The words used in the definition must be less complex than the word being
/// defined - a great way to ensure this is to use words whose (x, y) coordinates
/// are both much less than the (x, y) coordinates of the hypernym, another
/// would be to use the most simple and general words as possible (especially for
/// any words used in the definition that are not on the meaning grid).
///
/// After adding all the vectors in the definition
/// (with the exception of the new words hypernym), the resultant is then
/// added to the position vector of the hypernym, yielding a new resultant whose
/// end coordinate is represents the sense coordinate of the new word.
#[no_mangle]
pub extern "C" fn batch_define_new_word_sense(
    conn: &Connection,
    discourse_id: i32,
    agrees_to_the_creed: &bool,
) -> Result<HashMap<i32, (f64, f64)>, Box<dyn std::error::Error>> {
    // Check that the user has agreed to The Creed.
    // Stop processing with an error if they haven't.
    // The user interface must present a choice whose value we use here.
    if !agrees_to_the_creed {
        Err("You must agree to The Creed to continue".to_string())?
    }

    // HashMap<new_word_id, Point2D>
    let mut new_coordinates: HashMap<i32, Point2D> = HashMap::new();

    // The values in health_checks can be used by the network
    // to flag the health of new definitions for acceptance
    // or for further scrutiny.
    let mut health_checks: HashMap<i32, (f64, f64)> = HashMap::new();

    let new_word_vec: Vec<InputNewWordDef> = select_input_new_word_defs(&conn, discourse_id)?;

    for new_word in &new_word_vec {
        let start_word_id = new_word.hypernym_synset_id;
        let isa_xy: Point2D = select_new_def_isa(&start_word_id)?;

        let definition: Vec<InputWord> =
            select_input_words_new_def(&conn, &discourse_id, new_word.new_word_id)?;

        let mut resultant = Vector2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 0.0, y: 0.0 },
        };

        for def_word in &definition {
            if def_word.lexeme.to_lowercase() != new_word.lexeme.to_lowercase() {
                let position_vector = Vector2D {
                    start: Point2D { x: 0.0, y: 0.0 },
                    end: Point2D {
                        x: def_word.x,
                        y: def_word.y,
                    },
                };

                resultant = vector_addition_2d(&resultant, &position_vector);
            }
        }

        // Create a new vector from the isa_xy and
        // add it to the resultant.
        let isa_position_vector = Vector2D {
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D {
                x: isa_xy.x,
                y: isa_xy.y,
            },
        };

        // Nature differentiates "is a" relations by the 80-20
        // rule (properly called, the Pareto Distribution),
        // so we can apply this to flag that the definition
        // is outlier or falls within expectation.
        // Presently, we just send coordinate ratios that we
        // can check to be within a threshold of 0.80.
        // In future, it would be best to apply the Pareto
        // distribution formula and experiment with varied values
        // for the shape parameter until a best fit is obtained.
        // Naive implementation follows for x, the axis
        // for functional variety:
        let total_x_space: f64 = isa_position_vector.end.x / 0.8;
        let new_property_x_space: f64 = total_x_space - isa_position_vector.end.x;
        // adjust x to fit in new_property_x_space.
        let adjusted_x = resultant.end.x * (new_property_x_space / total_x_space);
        resultant.end.x = adjusted_x;

        // Do the addition.
        resultant = vector_addition_2d(&resultant, &isa_position_vector);

        health_checks.insert(
            new_word.new_word_id,
            (isa_xy.x / resultant.end.x, isa_xy.y / resultant.end.y),
        );

        // Add end point of resultant to result.
        new_coordinates.insert(new_word.new_word_id, resultant.end);
    }

    // Set the coordinates for all new definitions in this discourse.
    for item in &new_coordinates {
        update_input_new_word_def(&conn, &item.1.x, &item.1.y, &item.0)?;
    }

    Ok(health_checks)
}

/// Toy. Gets a vector collection of sets of math vectors that correspond
/// to the concracting set of convex hulls.
/// As with the convex_hull_sets, this is a gradient, with the first set containing
/// the most influencial vectors and the last set, the least.
pub extern "C" fn representative_vector_sets(
    convex_hull_sets: &Vec<Vec<Point2D>>,
    agrees_to_the_creed: &bool,
) -> Result<Vec<Vec<Vector2D>>, Box<dyn std::error::Error>> {
    // Check that the user has agreed to The Creed.
    // Stop processing with an error if they haven't.
    // The user interface must present a choice whose value we use here.
    if !agrees_to_the_creed {
        Err("You must agree to The Creed to continue".to_string())?
    }

    let mut result: Vec<Vec<Vector2D>> = Vec::new();

    for point_vec in convex_hull_sets {
        let vector_vector2d: Vec<Vector2D> = point_vec_to_position_vec_2d(point_vec);
        result.push(vector_vector2d);
    }

    Ok(result)
}

/// Toy. Gets a vector of a concracting set of convex hulls that
/// correspond to sets of entities ordered by influence on the discourse.
/// This creates a gradient, with the first set of convex hulls containing
/// the most influencial entities and the last set, the least.
/// The parameter, translate, can be any of the translate functions
/// in the math module that is paired with a corresponding input_vectors parameter.
pub extern "C" fn convex_hull_sets(
    input_vectors: &Vec<Vector2D>,
    //translate: &Fn(&Vec<Vector2D>) -> Vec<Vector2D>,
    agrees_to_the_creed: &bool,
) -> Result<Vec<Vec<Point2D>>, Box<dyn std::error::Error>> {
    // Check that the user has agreed to The Creed.
    // Stop processing with an error if they haven't.
    // The user interface must present a choice whose value we use here.
    if !agrees_to_the_creed {
        Err("You must agree to The Creed to continue".to_string())?
    }

    // Perform the desired translation on the vectors.
    // let vector_vector: Vec<Vector2D> = translate(input_vectors).to_vec();
    // Commented out, because it might be best to do the separation of dimensions in SQL.

    //// Extract a vector of points.
    //let mut point_vector: Vec<Point2D> = position_vec_to_point_vec_2d(&vector_vector);
    // Commented out, because it might be best to do the separation of dimensions in SQL.

    // Extract a vector of points.
    let mut point_vector: Vec<Point2D> = position_vec_to_point_vec_2d(input_vectors);

    let mut convex_hull_vector: Vec<Vec<Point2D>> = Vec::new();

    // Gets concentric convex hulls.
    // for _i in 1..3 {
    loop {
        let convex_hull: Vec<Point2D> = calculate_convex_hull(&point_vector);

        // Eject the points in the current convex hull from point_vector.
        let mut temp: Vec<Point2D> = Vec::new();
        for point in &point_vector {
            if !convex_hull.contains(point) {
                temp.push(Point2D {
                    x: point.x,
                    y: point.y,
                });
            }
        }
        // At least 3 elements are needed for a convex hull, so;
        if temp.len() < 3 {
            break;
        }
        // Update point_vector.
        point_vector = temp;

        convex_hull_vector.push(convex_hull);
    }

    Ok(convex_hull_vector)
}

/// Gets the most essential items (useful useful for extracting a pithy definition)
/// in a long tailed distribution by getting items at or before the sharpest point of the bend.
pub extern "C" fn top_contributors(hash_item_vec: &Vec<HashItem>) -> Vec<HashItem> {
    // Note that calculations are done with the direction set as "max x --> origin".
    let mut result: Vec<HashItem> = Vec::new();
    // Copy the vector.
    let mut input: Vec<HashItem> = hash_item_vec.to_vec();
    // Sort input in descending order by excited_radius.
    input.sort_by(|a, b| {
        b.excited_radius
            .partial_cmp(&a.excited_radius)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Get the side of the triangle between the
    // upper and lower bounds of the input vector.
    let point_min = Point2D {
        x: 0.0,
        y: input[0].excited_radius,
    };
    let point_max = Point2D {
        x: (&input.len() - 1) as f64,
        y: input[input.len() - 1].excited_radius,
    };
    let span = distance(&point_min, &point_max);

    // Angle and index trackers for the min angle and its index.
    let mut min_angle = std::f64::consts::PI;
    let mut bend_index: usize = 0;

    for i in 0..&input.len() - 1 {
        // Compute the lengths of the remain two sides of the triangle.
        let point = Point2D {
            x: i as f64,
            y: input[i].excited_radius,
        };
        let span_left: f64 = distance(&point_min, &point);
        let span_right: f64 = distance(&point_max, &point);

        // Use cosine law to find angle between span_left and span_right.
        let cos = (span_left.powi(2) + span_right.powi(2) - span.powi(2))
            / (2.0 * span_left * span_right);
        let angle = cos.acos();

        // Seek the minimum angle to get the sharpest point of bend.
        if angle < min_angle {
            bend_index = i;
            min_angle = angle;
        }
    }

    // Collect only items at of before bend_index.
    for i in 0..&input.len() - 1 {
        result.push(input[i].clone());
        if i == bend_index {
            break;
        }
    }

    result
}

#[no_mangle]
pub extern "C" fn version() -> &'static str {
    let current_version = "19.07.23 (Codename Ama)";
    current_version
}

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

use crate::math::{Point2D, Vector2D};

/// Calculates the area of an ordered set of points using the shoelace formula.
pub(crate) fn polygon_area(points: &Vec<Point2D>) -> f64 {
    // A triangle is the minimal shape that can have an area,
    // so ensure that the shape has at least 3 points.
    if points.len() < 3 {
        return 0.0;
    }

    let mut area: f64 = 0.0;
    let mut points2: Vec<Point2D> = Vec::new();
    // Copy the original array.
    points2.extend(points.iter().cloned());
    // Push the first point in points2 to make it the last point.
    points2.push(Point2D {
        x: points2[0].x,
        y: points[0].y,
    });

    let mut i: usize = 0_usize;

    loop {
        area += (points2[i + 1].x - points2[i].x) * (points2[i + 1].y + points2[i].y);
        i = i + 1;
        if i == points2.len() {
            break;
        }
    }

    (area * 0.5).abs()
}

/// Calculates convex hull from list of points (f64, f64)
/// This is a slight modification of existing online code, courtesy https://rosettacode.org/wiki/Convex_hull#Rust
pub(crate) fn calculate_convex_hull(points: &Vec<Point2D>) -> Vec<Point2D> {
    //There must be at least 3 points
    if points.len() < 3 {
        return points.clone().to_vec();
    }

    let mut hull = vec![];

    /*//Find the left most point in the polygon
    let (left_most_idx, _) = points.iter()
        .enumerate()
        .min_by(|lhs, rhs| lhs.1.x.partial_cmp(&rhs.1.x).unwrap())
        .expect("No left most point");*/

    // Find the left most point in the polygon.
    let mut left_most_idx = 0_usize;
    let mut min_x = points[0].x;
    for (i, item) in points.iter().enumerate() {
        if item.x < min_x {
            min_x = item.x;
            left_most_idx = i;
        };
    }

    let mut p = left_most_idx;
    let mut q = 0_usize;

    loop {
        //The left most point must be part of the hull
        //hull.push(points[p].clone());
        hull.push(Point2D {
            x: points[p].x,
            y: points[p].y,
        });

        q = (p + 1) % points.len();

        for i in 0..points.len() {
            if orientation(&points[p], &points[i], &points[q]) == 2 {
                q = i;
            }
        }

        p = q;

        //Break from loop once we reach the first point again
        if p == left_most_idx {
            break;
        }
    }

    return hull;
}

//Calculate orientation for 3 points
//0 -> Straight line
//1 -> Clockwise
//2 -> Counterclockwise
fn orientation(p: &Point2D, q: &Point2D, r: &Point2D) -> usize {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    if val == 0. {
        return 0;
    };
    if val > 0. {
        return 1;
    } else {
        return 2;
    }
}

/// Gets the perimeter length of a convex hull.
pub(crate) fn convex_hull_perimeter(convex_hull: &Vec<Point2D>) -> f64 {
    let mut result: f64 = 0_f64;

    // Add up the distances between neighbouring points.
    for i in 0..convex_hull.len() - 1 {
        result += distance(&convex_hull[i], &convex_hull[i + 1]);
    }

    // Add the distance between the first and last points of the convex_hull.
    result += distance(&convex_hull[0], &convex_hull[convex_hull.len() - 1]);

    result
}

/// Calculates the distance between two points.
pub(crate) fn distance(one: &Point2D, two: &Point2D) -> f64 {
    //let distance_squared: f64 =
    //    ((one.x - two.x) * (one.x - two.x)) + ((one.y - two.y) * (one.y - two.y));
    //distance_squared.sqrt()

    (one.x - two.x).hypot(one.y - two.y)
}

/// Converts a vector collection of points to a vector collection of math vectors.
pub(crate) fn point_vec_to_position_vec_2d(vector: &Vec<Point2D>) -> Vec<Vector2D> {
    let mut result: Vec<Vector2D> = Vec::new();

    for item in vector {
        let start_point = Point2D { x: 0.0, y: 0.0 };

        let end_point = Point2D {
            x: item.x,
            y: item.y,
        };

        result.push(Vector2D {
            start: start_point,
            end: end_point,
        });
    }

    result
}

/// Converts a vector collection of math vectors to a vector collection of points.
pub(crate) fn position_vec_to_point_vec_2d(vector: &Vec<Vector2D>) -> Vec<Point2D> {
    let mut result: Vec<Point2D> = Vec::new();

    for item in vector {
        result.push(Point2D {
            x: item.end.x,
            y: item.end.y,
        });
    }

    result
}

/// Computes the number to add to a Wordnet offset to
/// convert it to Wordnet SQL format, given a Wordnet
/// style part of speech parameter.
pub(crate) fn pos_num_to_subtract(pos: &str) -> Option<i32> {
    let mut num: i32 = 0;

    if pos == "n" {
        num = 100000000;
    }

    if pos == "v" {
        num = 200000000;
    }

    if pos == "s" {
        num = 300000000
    }

    if pos == "a" {
        num = 300000000;
    }

    if pos == "r" {
        num = 400000000;
    }

    Some(num)
}

/// Retrieves a Wordnet style part of speech letter,
/// given a Wordnet SQL format synset_id.
pub(crate) fn pos_letter(synset_id: &i32) -> Option<String> {
    let mut pos: String = "".to_string();

    if synset_id < &200000000 {
        pos.push_str("n");
    }

    if synset_id > &200000000 && synset_id < &300000000 {
        pos.push_str("v");
    }

    if synset_id > &300000000 && synset_id < &400000000 {
        pos.push_str("a");
    }

    if synset_id > &400000000 {
        pos.push_str("r");
    }

    Some(pos)
}

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

use crate::TOLERANCE;

/// A 2D point.
#[derive(Debug, Clone, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// A 3D point.
#[derive(Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A 2D vector.
#[derive(Debug, Clone)]
pub struct Vector2D {
    pub start: Point2D,
    pub end: Point2D,
}

/// A 3D vector.
#[derive(Debug)]
pub struct Vector3D {
    pub start: Point3D,
    pub end: Point3D,
}

/// Adds two 2D vectors.
pub fn vector_addition_2d(vector1: &Vector2D, vector2: &Vector2D) -> Vector2D {
    let xi: f64 = vector1.start.x + vector2.start.x;
    let yi: f64 = vector1.start.y + vector2.start.y;
    let point1 = Point2D { x: xi, y: yi };

    let xj: f64 = vector1.end.x + vector2.end.x;
    let yj: f64 = vector1.end.y + vector2.end.y;
    let point2 = Point2D { x: xj, y: yj };

    let vector: Vector2D = Vector2D {
        start: point1,
        end: point2,
    };

    vector
}

/// Adds two 3D vectors.
pub fn vector_addition_3d(vector1: &Vector3D, vector3: &Vector3D) -> Vector3D {
    let xi: f64 = vector1.start.x + vector3.start.x;
    let yi: f64 = vector1.start.y + vector3.start.y;
    let zi: f64 = vector1.start.z + vector3.start.z;
    let point1 = Point3D {
        x: xi,
        y: yi,
        z: zi,
    };

    let xj: f64 = vector1.end.x + vector3.end.x;
    let yj: f64 = vector1.end.y + vector3.end.y;
    let zj: f64 = vector1.end.z + vector3.end.z;
    let point3 = Point3D {
        x: xj,
        y: yj,
        z: zj,
    };

    let vector: Vector3D = Vector3D {
        start: point1,
        end: point3,
    };

    vector
}

/// Translate a 2D vector in the positive x direction of a reference frame of maximum x dimension, max_x.
pub fn vector_translate_right_2d(
    vector: &Vector2D,
    max_x: &f64,
) -> Result<Vector2D, Box<dyn std::error::Error>> {
    let xi: f64 = vector.start.x + max_x;
    let point1 = Point2D {
        x: xi,
        ..vector.start
    };

    let xj: f64 = vector.end.x + max_x;
    let point2 = Point2D {
        x: xj,
        ..vector.end
    };

    let vector: Vector2D = Vector2D {
        start: point1,
        end: point2,
    };

    Ok(vector)
}

/// Translate a 2D vector in the negative x direction of a reference frame of maximum x dimension, max_x.
pub fn vector_translate_left_2d(
    vector: &Vector2D,
    max_x: &f64,
) -> Result<Vector2D, Box<dyn std::error::Error>> {
    let xi: f64 = vector.start.x - max_x;
    let point1 = Point2D {
        x: xi,
        ..vector.start
    };

    let xj: f64 = vector.end.x - max_x;
    let point2 = Point2D {
        x: xj,
        ..vector.end
    };

    let vector: Vector2D = Vector2D {
        start: point1,
        end: point2,
    };

    Ok(vector)
}

/// Translate a 2D vector in the positive y direction of a reference frame of maximum y dimension, max_y.
pub fn vector_translate_upward_2d(
    vector: &Vector2D,
    max_y: &f64,
) -> Result<Vector2D, Box<dyn std::error::Error>> {
    let yi: f64 = vector.start.y + max_y;
    let point1 = Point2D {
        y: yi,
        ..vector.start
    };

    let yj: f64 = vector.end.y + max_y;
    let point2 = Point2D {
        y: yj,
        ..vector.end
    };

    let vector: Vector2D = Vector2D {
        start: point1,
        end: point2,
    };

    Ok(vector)
}

/// Translate a 2D vector in the negative y direction of a reference frame of maximum y dimension, max_y.
pub fn vector_translate_downward_2d(
    vector: &Vector2D,
    max_y: &f64,
) -> Result<Vector2D, Box<dyn std::error::Error>> {
    let yi: f64 = vector.start.y - max_y;
    let point1 = Point2D {
        y: yi,
        ..vector.start
    };

    let yj: f64 = vector.end.y - max_y;
    let point2 = Point2D {
        y: yj,
        ..vector.end
    };

    let vector: Vector2D = Vector2D {
        start: point1,
        end: point2,
    };

    Ok(vector)
}

/// Translate a 3D vector in the positive x direction of a reference frame of maximum x dimension, max_x.
pub fn vector_translate_right_3d(
    vector: &Vector3D,
    max_x: &f64,
) -> Result<Vector3D, Box<dyn std::error::Error>> {
    let xi: f64 = vector.start.x + max_x;
    let point1 = Point3D {
        x: xi,
        ..vector.start
    };

    let xj: f64 = vector.end.x + max_x;
    let point3 = Point3D {
        x: xj,
        ..vector.end
    };

    let vector: Vector3D = Vector3D {
        start: point1,
        end: point3,
    };

    Ok(vector)
}

/// Translate a 3D vector in the negative x direction of a reference frame of maximum x dimension, max_x.
pub fn vector_translate_left_3d(
    vector: &Vector3D,
    max_x: &f64,
) -> Result<Vector3D, Box<dyn std::error::Error>> {
    let xi: f64 = vector.start.x - max_x;
    let point1 = Point3D {
        x: xi,
        ..vector.start
    };

    let xj: f64 = vector.end.x - max_x;
    let point3 = Point3D {
        x: xj,
        ..vector.end
    };

    let vector: Vector3D = Vector3D {
        start: point1,
        end: point3,
    };

    Ok(vector)
}

/// Translate a 3D vector in the positive y direction of a reference frame of maximum y dimension, max_y.
pub fn vector_translate_upward_3d(
    vector: &Vector3D,
    max_y: &f64,
) -> Result<Vector3D, Box<dyn std::error::Error>> {
    let yi: f64 = vector.start.y + max_y;
    let point1 = Point3D {
        y: yi,
        ..vector.start
    };

    let yj: f64 = vector.end.y + max_y;
    let point3 = Point3D {
        y: yj,
        ..vector.end
    };

    let vector: Vector3D = Vector3D {
        start: point1,
        end: point3,
    };

    Ok(vector)
}

/// Translate a 3D vector in the negative y direction of a reference frame of maximum y dimension, max_y.
pub fn vector_translate_downward_3d(
    vector: &Vector3D,
    max_y: &f64,
) -> Result<Vector3D, Box<dyn std::error::Error>> {
    let yi: f64 = vector.start.y - max_y;
    let point1 = Point3D {
        y: yi,
        ..vector.start
    };

    let yj: f64 = vector.end.y - max_y;
    let point3 = Point3D {
        y: yj,
        ..vector.end
    };

    let vector: Vector3D = Vector3D {
        start: point1,
        end: point3,
    };

    Ok(vector)
}

/// Translate a 3D vector in the positive z direction of a reference frame of maximum z dimension, max_z.
pub fn vector_translate_forward_3d(
    vector: &Vector3D,
    max_z: &f64,
) -> Result<Vector3D, Box<dyn std::error::Error>> {
    let zi: f64 = vector.start.z + max_z;
    let point1 = Point3D {
        z: zi,
        ..vector.start
    };

    let zj: f64 = vector.end.z + max_z;
    let point3 = Point3D {
        z: zj,
        ..vector.end
    };

    let vector: Vector3D = Vector3D {
        start: point1,
        end: point3,
    };

    Ok(vector)
}

/// Translate a 3D vector in the negative z direction of a reference frame of maximum z dimension, max_z.
pub fn vector_translate_backward_3d(vector: &Vector3D, max_z: &f64) -> Vector3D {
    let zi: f64 = vector.start.z - max_z;
    let point1 = Point3D {
        z: zi,
        ..vector.start
    };

    let zj: f64 = vector.end.z - max_z;
    let point3 = Point3D {
        z: zj,
        ..vector.end
    };

    let vector: Vector3D = Vector3D {
        start: point1,
        end: point3,
    };

    vector
}

/// Returns the magnitude of a 2D vector.
pub fn vector_magnitude_2d(vector: &Vector2D) -> f64 {
    let xi: f64 = vector.start.x * vector.start.x;
    let yi: f64 = vector.start.y * vector.start.y;

    let magnitude_squared = xi + yi;

    let magnitude = magnitude_squared.sqrt();

    magnitude
}

/// Returns the magnitude of a 3D vector.
pub fn vector_magnitude_3d(vector1: &Vector3D) -> f64 {
    let xi: f64 = vector1.start.x * vector1.start.x;
    let yi: f64 = vector1.start.y * vector1.start.y;
    let zi: f64 = vector1.start.z * vector1.start.z;

    let magnitude_squared = xi + yi + zi;

    let magnitude = magnitude_squared.sqrt();

    magnitude
}

/// Converts a 2D Cartesian coordinate to Polar.
pub(crate) fn cartesian_to_polar(point: &Point2D) -> (f64, f64) {
    let x_squared = point.x * point.x;
    let y_squared = point.y * point.y;

    let radius = (x_squared + y_squared).sqrt();
    let angle = (point.y / point.x).atan();

    let result = (radius, angle);

    result
}

/// Calculates the degree to which properties are shared by
/// the rule of thumb, according to which the degree to which
/// functions are shared between two meanings is proportional
/// to the area of the shared triangle between both meanings
/// coordinates on the meaning grid.
pub(crate) fn areal_jaccard(point1: &Point2D, point2: &Point2D) -> f64 {
    if (point1.x - point2.x).abs() <= TOLERANCE && (point1.x - point2.x).abs() <= TOLERANCE {
        return 1.0;
    }

    // These are areas close to the origin.
    // Scope needs to be worked out by
    // convex hull and polygon area.
    // To Do.
    let general_area: f64 = 0.0;

    let mut jaccard: f64 = 0.0;

    let grad1 = point1.y / point1.x;
    let grad2 = point2.y / point2.x;

    let area1 = point1.y * point1.x * 0.5;
    let area2 = point2.y * point2.x * 0.5;
    let total_area = area1 + area2 - general_area;
    let mut shared_area = std::f64::MIN;

    if point1.x <= point2.x {
        if grad1 < grad2 {
            shared_area = point1.x * grad1 * point1.x * 0.5;
        } else {
            shared_area = point2.x * grad2 * point1.x * 0.5;
        }
    } else {
        if grad1 < grad2 {
            shared_area = point1.x * grad1 * point2.x * 0.5;
        } else {
            shared_area = point2.x * grad2 * point2.x * 0.5;
        }
    }

    jaccard = (shared_area - general_area) / total_area;

    jaccard
}

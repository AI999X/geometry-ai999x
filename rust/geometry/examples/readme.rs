//! Geometry: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p geometry-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&geometry_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "convex_polygon_intersection", "inputs": {"p": [0, 0, 2, 0, 2, 2, 0, 2], "q": [1, 1, 3, 1, 3, 3, 1, 3]}}));
    println!();
    show(json!({"key": "conic_classify", "inputs": {"A": "1", "B": "0", "C": "4", "D": "0", "E": "0", "F": "-4"}}));
    println!();
    show(json!({"key": "angle_between_lines", "inputs": {"a1": "-1", "b1": "1", "c1": "-1", "a2": "1", "b2": "1", "c2": "-3"}}));
    println!();
    show(json!({"key": "parabola_arc_length", "inputs": {"a": "1", "b": "0", "c": "0", "x0": "0", "x1": "1"}}));
    println!();
    show(json!({"key": "triangle_area_base_height", "inputs": {"base": "10", "height": "6"}}));
    println!();
    show(json!({"key": "polygon_centroid", "inputs": {"points": [0, 0, 4, 0, 4, 2, 0, 2]}}));
    println!();
    show(json!({"key": "platonic_solid", "inputs": {"solid": "icosahedron", "edge": "2"}}));
    println!();
    show(json!({"key": "similar_figures", "inputs": {"measure": "lengths", "m1": "4", "m2": "6"}}));
    println!();
    show(json!({"key": "angle_between_planes", "inputs": {"a1": "0", "b1": "0", "c1": "1", "a2": "1", "b2": "0", "c2": "1"}}));
    println!();
    show(json!({"key": "solid_angle_cone", "inputs": {"half_angle": "60"}}));
    println!();
    show(json!({"key": "cone_measures", "inputs": {"radius": "3", "height": "4"}}));
    println!();
    show(json!({"key": "spherical_triangle_area", "inputs": {"radius": "1", "points": [0, 0, 0, 90, 90, 0]}}));
    println!();
    show(json!({"key": "torus_curvature", "inputs": {"major": "3", "minor": "1", "v": "0"}}));
    println!();
    show(json!({"key": "dilate_point", "inputs": {"k": "3", "px": "2", "py": "3"}}));
    println!();
    show(json!({"key": "closest_pair", "inputs": {"points": [0, 0, 5, 5, 1, 1, 9, 2, 6, 6.5]}}));
}

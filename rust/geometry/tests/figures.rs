//! Every Geometry capability's own live figures (src/figures.rs): its worked example gives each published figure
//! kind, a changed input changes the drawing, and no drawing holds a non-finite number or text outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use geometry_ai999x::pack;

/// The published figure kinds of every capability drawn by the model's own figures
/// (publish/website/models/geometry-ai999x/figures, except the generic "curve" and the kinds the generic figures
/// already draw), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("annulus_area", &["field"]),
    ("barycentric_coordinates", &["points"]),
    ("circular_segment", &["field"]),
    ("closest_pair", &["points"]),
    ("cone_measures", &["field"]),
    ("conic_classify", &["decision"]),
    ("convex_hull", &["points", "shape"]),
    ("convex_polygon_intersection", &["points"]),
    ("cylinder_measures", &["field"]),
    ("delaunay_voronoi_counts", &["points"]),
    ("farthest_pair", &["points"]),
    ("hausdorff_distance", &["matrix"]),
    ("helix_measures", &["field"]),
    ("min_area_rectangle", &["points"]),
    ("min_enclosing_circle", &["points"]),
    ("nearest_neighbor", &["points"]),
    ("pick_lattice_points", &["points"]),
    ("plane_through_points", &["matrix"]),
    ("point_in_polygon", &["points"]),
    ("polygon_area", &["points"]),
    ("polygon_centroid", &["points", "shape"]),
    ("polygon_interior_angles", &["points"]),
    ("polygon_is_convex", &["points"]),
    ("polyline_length", &["points"]),
    ("rectangle_measures", &["field"]),
    ("regular_polygon_angles", &["stems"]),
    ("regular_polygon_measures", &["field"]),
    ("rhombus_measures", &["field"]),
    ("rotate_point", &["shape"]),
    ("sector_measures", &["field"]),
    ("segment_intersection", &["points"]),
    ("skew_lines_distance", &["geometry"]),
    ("solid_angle_triangle", &["matrix"]),
    ("sphere_four_points", &["matrix"]),
    ("spherical_cap", &["field"]),
    ("spherical_triangle_area", &["points"]),
    ("tangent_from_point", &["shape"]),
    ("tetrahedron_measures", &["matrix"]),
    ("torus_measures", &["field"]),
    ("triangle_area_3d", &["matrix"]),
    ("triangle_area_base_height", &["field"]),
    ("triangle_circumcircle", &["points", "shape"]),
    ("triangle_coordinates", &["points"]),
    ("triangle_incircle", &["points"]),
    ("triangle_orthocenter", &["points"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("regular_polygon_angles", "n", "9"),
    ("regular_polygon_measures", "n", "8"),
    ("pick_lattice_points", "points", "0 0, 5 0, 5 2, 1 4"),
    ("point_in_polygon", "px", "4"),
    ("polygon_is_convex", "points", "0 0, 5 0, 5 4, 3 2, 0 4"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn kind_of(f: &Figure) -> &str {
    &f.kind
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    // ("infinite" is a word of some titles: check_figure looks for inf as a number)
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        // the other example: a listed replacement, else each number input scaled until the model solves it
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // inputs the model refuses (three collinear points have no circumcircle; an inner radius above the outer one):
    // no figure at all, never a wrong one
    let p = pack();
    for (key, given) in [("triangle_circumcircle", vec![("points", "0 0, 1 1, 2 2")]),
                         ("annulus_area", vec![("outer", "2"), ("inner", "3")]),
                         ("tangent_from_point", vec![("cx", "0"), ("cy", "0"), ("r", "3"), ("px", "1"), ("py", "0")])] {
        let given: Vec<(String, Input)> = given.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
        let a = ax_engine::solve(p, key, &given, None);
        let figs = figures(p, key, &given, None, None);
        if !a.solved() {
            assert!(figs.is_empty(), "{key}: a refused input drew a figure");
        } else {
            for f in &figs {
                check_texts(f);
            }
        }
    }
}

#[test]
fn own_figures_time() {
    let p = pack();
    for (key, _) in KINDS {
        let given = example(key);
        let t = std::time::Instant::now();
        let _ = figures(p, key, &given, None, None);
        eprintln!("TIME {key}: {} ms", t.elapsed().as_millis());
    }
}

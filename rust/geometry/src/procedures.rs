//! geometry: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/geometry/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "angle_between_lines",
    "angle_between_planes",
    "annulus_area",
    "barycentric_coordinates",
    "circle_circle_intersection",
    "circle_line_intersection",
    "circle_measures",
    "circular_segment",
    "closest_pair",
    "cone_measures",
    "conic_classify",
    "convex_hull",
    "convex_polygon_intersection",
    "cuboid_measures",
    "curvature_of_graph",
    "cylinder_measures",
    "delaunay_voronoi_counts",
    "dilate_point",
    "ellipse_curvature",
    "ellipse_perimeter",
    "farthest_pair",
    "frustum_measures",
    "great_circle_distance",
    "hausdorff_distance",
    "helix_measures",
    "initial_bearing",
    "line_in_space",
    "line_intersection",
    "line_plane_intersection",
    "min_area_rectangle",
    "min_enclosing_circle",
    "nearest_neighbor",
    "parabola_arc_length",
    "pick_lattice_points",
    "plane_through_points",
    "platonic_solid",
    "point_in_polygon",
    "point_plane_distance",
    "point_segment_distance",
    "polygon_area",
    "polygon_centroid",
    "polygon_interior_angles",
    "polygon_is_convex",
    "polyline_length",
    "pyramid_measures",
    "rectangle_measures",
    "reflect_point_line",
    "regular_polygon_angles",
    "regular_polygon_measures",
    "regular_prism",
    "rhombus_measures",
    "rotate_point",
    "rotate_point_3d",
    "sector_measures",
    "segment_intersection",
    "similar_figures",
    "skew_lines_distance",
    "solid_angle_cone",
    "solid_angle_triangle",
    "sphere_four_points",
    "sphere_measures",
    "spherical_cap",
    "spherical_triangle_area",
    "spheroid_measures",
    "surface_curvature",
    "tangent_from_point",
    "tetrahedron_measures",
    "torus_curvature",
    "torus_measures",
    "trapezoid_measures",
    "triangle_area_3d",
    "triangle_area_base_height",
    "triangle_cevians",
    "triangle_circumcircle",
    "triangle_classify",
    "triangle_coordinates",
    "triangle_incircle",
    "triangle_orthocenter",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "angle_between_lines" => {
            let r = s_1(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a1", "b1", "c1", "a2", "b2", "c2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_16(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "angle_between_planes" => {
            let r = s_31(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a1", "b1", "c1", "a2", "b2", "c2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_44(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "annulus_area" => {
            let r = s_56(Py::tuple(vec![]), { let mut d = vec![]; for k in ["outer", "inner"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_67(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "barycentric_coordinates" => {
            let r = s_89(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "circle_circle_intersection" => {
            let r = s_123(Py::tuple(vec![]), { let mut d = vec![]; for k in ["x1", "y1", "r1", "x2", "y2", "r2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_151(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "circle_line_intersection" => {
            let r = s_187(Py::tuple(vec![]), { let mut d = vec![]; for k in ["cx", "cy", "r", "a", "b", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_214(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "circle_measures" => {
            let r = s_245(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "circular_segment" => {
            let r = s_277(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius", "angle"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_291(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "closest_pair" => {
            let r = s_315(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_337(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cone_measures" => {
            let r = s_352(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_361(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "conic_classify" => {
            let r = s_385(Py::tuple(vec![]), { let mut d = vec![]; for k in ["A", "B", "C", "D", "E", "F"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_416(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "convex_hull" => {
            let r = s_451(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_483(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "convex_polygon_intersection" => {
            let r = s_526(Py::tuple(vec![]), { let mut d = vec![]; for k in ["p", "q"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_638(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cuboid_measures" => {
            let r = s_652(Py::tuple(vec![]), { let mut d = vec![]; for k in ["length", "width", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_661(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "curvature_of_graph" => {
            let r = s_685(Py::tuple(vec![]), { let mut d = vec![]; for k in ["coefficients", "x0"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_700(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cylinder_measures" => {
            let r = s_726(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_735(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "delaunay_voronoi_counts" => {
            let r = s_759(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_799(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dilate_point" => {
            let r = s_824(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "k", "cx", "cy"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_834(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ellipse_curvature" => {
            let r = s_846(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_855(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ellipse_perimeter" => {
            let r = s_879(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_897(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "farthest_pair" => {
            let r = s_922(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_952(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "frustum_measures" => {
            let r = s_965(Py::tuple(vec![]), { let mut d = vec![]; for k in ["r1", "r2", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_974(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "great_circle_distance" => {
            let r = s_998(Py::tuple(vec![]), { let mut d = vec![]; for k in ["lat1", "lon1", "lat2", "lon2", "radius"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1017(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hausdorff_distance" => {
            let r = s_1038(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1056(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "helix_measures" => {
            let r = s_1084(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius", "pitch"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1093(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "initial_bearing" => {
            let r = s_1117(Py::tuple(vec![]), { let mut d = vec![]; for k in ["lat1", "lon1", "lat2", "lon2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1143(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_in_space" => {
            let r = s_1159(Py::tuple(vec![]), { let mut d = vec![]; for k in ["point", "second_point", "direction", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1223(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_intersection" => {
            let r = s_1260(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a1", "b1", "c1", "a2", "b2", "c2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1278(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_plane_intersection" => {
            let r = s_1290(Py::tuple(vec![]), { let mut d = vec![]; for k in ["x0", "y0", "z0", "dx", "dy", "dz", "a", "b", "c", "d"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1313(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "min_area_rectangle" => {
            let r = s_1328(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1369(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "min_enclosing_circle" => {
            let r = s_1398(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1428(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nearest_neighbor" => {
            let r = s_1455(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1470(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parabola_arc_length" => {
            let r = s_1503(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b", "x0", "x1", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1515(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pick_lattice_points" => {
            let r = s_1543(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1637(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "plane_through_points" => {
            let r = s_1674(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1695(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "platonic_solid" => {
            let r = s_1715(Py::tuple(vec![]), { let mut d = vec![]; for k in ["solid", "edge"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1726(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "point_in_polygon" => {
            let r = s_1752(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1853(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "point_plane_distance" => {
            let r = s_1887(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "pz", "a", "b", "c", "d"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1905(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "point_segment_distance" => {
            let r = s_1918(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "x1", "y1", "x2", "y2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1933(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_area" => {
            let r = s_1953(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2042(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_centroid" => {
            let r = s_2070(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2164(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_interior_angles" => {
            let r = s_2182(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2276(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_is_convex" => {
            let r = s_2314(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2407(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polyline_length" => {
            let r = s_2432(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2448(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pyramid_measures" => {
            let r = s_2474(Py::tuple(vec![]), { let mut d = vec![]; for k in ["length", "width", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2483(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rectangle_measures" => {
            let r = s_2507(Py::tuple(vec![]), { let mut d = vec![]; for k in ["length", "width"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2516(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reflect_point_line" => {
            let r = s_2540(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "a", "b", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2556(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "regular_polygon_angles" => {
            let r = s_2570(Py::tuple(vec![]), { let mut d = vec![]; for k in ["n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2581(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "regular_polygon_measures" => {
            let r = s_2605(Py::tuple(vec![]), { let mut d = vec![]; for k in ["n", "side"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2621(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "regular_prism" => {
            let r = s_2645(Py::tuple(vec![]), { let mut d = vec![]; for k in ["n", "side", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2661(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rhombus_measures" => {
            let r = s_2685(Py::tuple(vec![]), { let mut d = vec![]; for k in ["d1", "d2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2694(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotate_point" => {
            let r = s_2718(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "angle", "cx", "cy"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2730(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotate_point_3d" => {
            let r = s_2742(Py::tuple(vec![]), { let mut d = vec![]; for k in ["point", "axis", "angle"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2759(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sector_measures" => {
            let r = s_2776(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius", "angle"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2788(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "segment_intersection" => {
            let r = s_2812(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2884(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "similar_figures" => {
            let r = s_2924(Py::tuple(vec![]), { let mut d = vec![]; for k in ["measure", "m1", "m2", "other_length", "other_area", "other_volume"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2948(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "skew_lines_distance" => {
            let r = s_2969(Py::tuple(vec![]), { let mut d = vec![]; for k in ["p1", "d1", "p2", "d2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2987(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solid_angle_cone" => {
            let r = s_3007(Py::tuple(vec![]), { let mut d = vec![]; for k in ["half_angle"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3016(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solid_angle_triangle" => {
            let r = s_3040(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3057(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sphere_four_points" => {
            let r = s_3081(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3103(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sphere_measures" => {
            let r = s_3116(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3124(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spherical_cap" => {
            let r = s_3148(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3159(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spherical_triangle_area" => {
            let r = s_3183(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points", "radius"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3217(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spheroid_measures" => {
            let r = s_3243(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3260(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "surface_curvature" => {
            let r = s_3285(Py::tuple(vec![]), { let mut d = vec![]; for k in ["A", "B", "C", "D", "E", "x0", "y0", "F"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3295(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tangent_from_point" => {
            let r = s_3311(Py::tuple(vec![]), { let mut d = vec![]; for k in ["cx", "cy", "r", "px", "py"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3326(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tetrahedron_measures" => {
            let r = s_3339(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3355(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "torus_curvature" => {
            let r = s_3389(Py::tuple(vec![]), { let mut d = vec![]; for k in ["major", "minor", "v"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3401(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "torus_measures" => {
            let r = s_3425(Py::tuple(vec![]), { let mut d = vec![]; for k in ["major", "minor"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3436(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "trapezoid_measures" => {
            let r = s_3460(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3469(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_area_3d" => {
            let r = s_3493(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3509(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_area_base_height" => {
            let r = s_3533(Py::tuple(vec![]), { let mut d = vec![]; for k in ["base", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3542(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_cevians" => {
            let r = s_3564(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3578(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_circumcircle" => {
            let r = s_3607(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3632(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_classify" => {
            let r = s_3644(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3669(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_coordinates" => {
            let r = s_3690(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3712(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_incircle" => {
            let r = s_3737(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3763(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_orthocenter" => {
            let r = s_3779(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3806(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__PLATONIC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("tetrahedron"), Py::tuple(vec![Py::Int(4i64), Py::Int(6i64), Py::Int(4i64), Py::Float(0.1178511301977579), Py::Float(1.7320508075688772), Py::Float(0.6123724356957945), Py::Float(0.20412414523193148)])), (Py::str("cube"), Py::tuple(vec![Py::Int(8i64), Py::Int(12i64), Py::Int(6i64), Py::Float(1.0), Py::Float(6.0), Py::Float(0.8660254037844386), Py::Float(0.5)])), (Py::str("octahedron"), Py::tuple(vec![Py::Int(6i64), Py::Int(12i64), Py::Int(8i64), Py::Float(0.47140452079103173), Py::Float(3.4641016151377544), Py::Float(0.7071067811865475), Py::Float(0.4082482904638631)])), (Py::str("dodecahedron"), Py::tuple(vec![Py::Int(20i64), Py::Int(30i64), Py::Int(12i64), Py::Float(7.663118960624632), Py::Float(20.645728807067602), Py::Float(1.4012585384440734), Py::Float(1.1135163644116068)])), (Py::str("icosahedron"), Py::tuple(vec![Py::Int(12i64), Py::Int(30i64), Py::Int(20i64), Py::Float(2.1816949906249126), Py::Float(8.660254037844386), Py::Float(0.9510565162951535), Py::Float(0.7557613140761706)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SIM_MEASURES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("lengths"), Py::str("areas"), Py::str("volumes")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SIM_POWER() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("lengths"), Py::Int(1i64)), (Py::str("areas"), Py::Int(2i64)), (Py::str("volumes"), Py::Int(3i64))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ccw_convex(mut v_pts: Py, mut v_name: Py) -> R {
    let _ = f__simple_polygon(v_pts.clone(), v_name.clone())?;
    if ({ let t598 = py::getitem(&f_polygon_is_convex(f__flat(v_pts.clone())?)?, &Py::str("convex"))?; Py::Bool({ let t599 = Py::str("yes"); if py::ne(&t598, &t599) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": the polygon is not convex".to_string()].concat()))));
    }
    return Ok((if ({ let t620 = f__shoelace(v_pts.clone())?; Py::Bool({ let t621 = Py::Int(0i64); if py::gt(&t620, &t621)? { true } else { false } }) }).truthy() { v_pts.clone() } else { py::slice(&v_pts.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__central(mut v_p1: Py, mut v_p2: Py) -> R {
    let mut v_f1: Py = Py::None;
    let mut v_f2: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let t3210 = (Py::tuple(vec![v_p1.clone(), v_p2.clone()])).items()?;
    if t3210.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t3211 = (t3210[0].clone()).items()?;
    if t3211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f1 = t3211[0].clone();
    v_l1 = t3211[1].clone();
    let t3212 = (t3210[1].clone()).items()?;
    if t3212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f2 = t3212[0].clone();
    v_l2 = t3212[1].clone();
    v_h = py::add(&py::pow(&py::math1("sin", &py::truediv(&py::sub(&v_f2.clone(), &v_f1.clone())?, &Py::Int(2i64))?)?, &Py::Int(2i64))?, &py::mul(&py::mul(&py::math1("cos", &v_f1.clone())?, &py::math1("cos", &v_f2.clone())?)?, &py::pow(&py::math1("sin", &py::truediv(&py::sub(&v_l2.clone(), &v_l1.clone())?, &Py::Int(2i64))?)?, &Py::Int(2i64))?)?)?;
    v_h = py::minmax(&[Py::Float(1.0), py::minmax(&[Py::Float(0.0), v_h.clone()], true)?], false)?;
    return Ok(py::mul(&Py::Int(2i64), &py::atan2(&py::math1("sqrt", &v_h.clone())?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &v_h.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cevians(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_bis: Py = Py::None;
    let mut v_med: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3573 = (f__tri_check(v_a.clone(), v_b.clone(), v_c.clone())?).items()?;
    if t3573.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3573[0].clone();
    v_b = t3573[1].clone();
    v_c = t3573[2].clone();
    let t3574 = (py::sorted(&Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]), (Py::Bool(true)).truthy())?).items()?;
    if t3574.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3574[0].clone();
    v_y = t3574[1].clone();
    v_z = t3574[2].clone();
    v_K = py::mul(&Py::Float(0.25), &py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::add(&v_x.clone(), &py::add(&v_y.clone(), &v_z.clone())?)?, &py::sub(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_x.clone(), &py::sub(&v_y.clone(), &v_z.clone())?)?)?)?)?;
    v_med = { let v_K = v_K.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_o: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_p: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_q: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&Py::Float(0.5), &py::math1("sqrt", &py::sub(&py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_p.clone())?, &v_p.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_q.clone())?, &v_q.clone())?)?, &py::mul(&v_o.clone(), &v_o.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_bis = { let v_K = v_K.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_med = v_med.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_o: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_p: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_q: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::math1("sqrt", &py::mul(&py::mul(&py::mul(&v_p.clone(), &v_q.clone())?, &py::add(&py::add(&v_p.clone(), &v_q.clone())?, &v_o.clone())?)?, &py::sub(&py::add(&v_p.clone(), &v_q.clone())?, &v_o.clone())?)?)?, &py::add(&v_p.clone(), &v_q.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("median_a"), py::call(&v_med.clone(), &[v_a.clone(), v_b.clone(), v_c.clone()])?), (Py::str("median_b"), py::call(&v_med.clone(), &[v_b.clone(), v_a.clone(), v_c.clone()])?), (Py::str("median_c"), py::call(&v_med.clone(), &[v_c.clone(), v_a.clone(), v_b.clone()])?), (Py::str("altitude_a"), py::truediv(&py::mul(&Py::Int(2i64), &v_K.clone())?, &v_a.clone())?), (Py::str("altitude_b"), py::truediv(&py::mul(&Py::Int(2i64), &v_K.clone())?, &v_b.clone())?), (Py::str("altitude_c"), py::truediv(&py::mul(&Py::Int(2i64), &v_K.clone())?, &v_c.clone())?), (Py::str("bisector_a"), py::call(&v_bis.clone(), &[v_a.clone(), v_b.clone(), v_c.clone()])?), (Py::str("bisector_b"), py::call(&v_bis.clone(), &[v_b.clone(), v_a.clone(), v_c.clone()])?), (Py::str("bisector_c"), py::call(&v_bis.clone(), &[v_c.clone(), v_a.clone(), v_b.clone()])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__circ3(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_bx: Py = Py::None;
    let mut v_by: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ux: Py = Py::None;
    let mut v_uy: Py = Py::None;
    let t1418 = (Py::tuple(vec![py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?])).items()?;
    if t1418.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t1418[0].clone();
    v_by = t1418[1].clone();
    v_cx = t1418[2].clone();
    v_cy = t1418[3].clone();
    v_d = py::mul(&Py::Int(2i64), &py::sub(&py::mul(&v_bx.clone(), &v_cy.clone())?, &py::mul(&v_by.clone(), &v_cx.clone())?)?)?;
    if ({ let t1419 = v_d.clone(); Py::Bool({ let t1420 = Py::Int(0i64); if py::eq(&t1419, &t1420) { true } else { false } }) }).truthy() {
        return Ok(Py::None);
    }
    v_ux = py::truediv(&py::sub(&py::mul(&v_cy.clone(), &py::add(&py::mul(&v_bx.clone(), &v_bx.clone())?, &py::mul(&v_by.clone(), &v_by.clone())?)?)?, &py::mul(&v_by.clone(), &py::add(&py::mul(&v_cx.clone(), &v_cx.clone())?, &py::mul(&v_cy.clone(), &v_cy.clone())?)?)?)?, &v_d.clone())?;
    v_uy = py::truediv(&py::sub(&py::mul(&v_bx.clone(), &py::add(&py::mul(&v_cx.clone(), &v_cx.clone())?, &py::mul(&v_cy.clone(), &v_cy.clone())?)?)?, &py::mul(&v_cx.clone(), &py::add(&py::mul(&v_bx.clone(), &v_bx.clone())?, &py::mul(&v_by.clone(), &v_by.clone())?)?)?)?, &v_d.clone())?;
    return Ok(Py::tuple(vec![py::add(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &v_ux.clone())?, py::add(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &v_uy.clone())?, py::hypot(&[v_ux.clone(), v_uy.clone()])?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__clip(mut v_subject: Py, mut v_clipper: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_inp: Py = Py::None;
    let mut v_ip: Py = Py::None;
    let mut v_iq: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    v_out = v_subject.clone();
    v_n = py::len(&v_clipper.clone())?;
    for t622 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t622;
        let t623 = (Py::tuple(vec![py::getitem(&v_clipper.clone(), &v_k.clone())?, py::getitem(&v_clipper.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t623[0].clone();
        v_b = t623[1].clone();
        let t624 = (Py::tuple(vec![v_out.clone(), Py::list(vec![])])).items()?;
        if t624.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_inp = t624[0].clone();
        v_out = t624[1].clone();
        if (py::not(&v_inp.clone())).truthy() {
            break;
        }
        for t625 in py::range_iter(&[(py::len(&v_inp.clone())?).i()?]) {
            v_j = t625;
            let t626 = (Py::tuple(vec![py::getitem(&v_inp.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?, py::getitem(&v_inp.clone(), &v_j.clone())?])).items()?;
            if t626.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_p = t626[0].clone();
            v_q = t626[1].clone();
            let t631 = (Py::tuple(vec![{ let t627 = f__cross(v_a.clone(), v_b.clone(), v_p.clone())?; Py::Bool({ let t628 = Py::Int(0i64); if py::ge(&t627, &t628)? { true } else { false } }) }, { let t629 = f__cross(v_a.clone(), v_b.clone(), v_q.clone())?; Py::Bool({ let t630 = Py::Int(0i64); if py::ge(&t629, &t630)? { true } else { false } }) }])).items()?;
            if t631.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_ip = t631[0].clone();
            v_iq = t631[1].clone();
            if (v_iq.clone()).truthy() {
                if (py::not(&v_ip.clone())).truthy() {
                    let _ = py::method(&v_out.clone(), "append", &[f__seg_line(v_p.clone(), v_q.clone(), v_a.clone(), v_b.clone())?])?;
                }
                let _ = py::method(&v_out.clone(), "append", &[v_q.clone()])?;
            } else {
                if (v_ip.clone()).truthy() {
                    let _ = py::method(&v_out.clone(), "append", &[f__seg_line(v_p.clone(), v_q.clone(), v_a.clone(), v_b.clone())?])?;
                }
            }
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__conic_type(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_D: Py, mut v_E: Py, mut v_F: Py) -> R {
    let mut v_big: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let t391 = ({ let mut out = vec![]; for t390 in py::iter(&(Py::tuple(vec![v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone(), v_E.clone(), v_F.clone()])))? { let mut v_v: Py; 
v_v = t390;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t391.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t391[0].clone();
    v_B = t391[1].clone();
    v_C = t391[2].clone();
    v_D = t391[3].clone();
    v_E = t391[4].clone();
    v_F = t391[5].clone();
    if ({ let t399 = { let t392 = v_A.clone(); Py::Bool({ let t393 = Py::Int(0i64); if py::eq(&t392, &t393) { true } else { false } }) }; if !t399.truthy() { t399 } else { { let t398 = { let t394 = v_B.clone(); Py::Bool({ let t395 = Py::Int(0i64); if py::eq(&t394, &t395) { true } else { false } }) }; if !t398.truthy() { t398 } else { { let t396 = v_C.clone(); Py::Bool({ let t397 = Py::Int(0i64); if py::eq(&t396, &t397) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A, B and C are all 0: the equation is not quadratic"))));
    }
    v_delta = py::sub(&py::mul(&v_A.clone(), &v_C.clone())?, &py::truediv(&py::mul(&v_B.clone(), &v_B.clone())?, &Py::Int(4i64))?)?;
    v_big = py::add(&py::sub(&py::mul(&v_A.clone(), &py::sub(&py::mul(&v_C.clone(), &v_F.clone())?, &py::truediv(&py::mul(&v_E.clone(), &v_E.clone())?, &Py::Int(4i64))?)?)?, &py::mul(&py::truediv(&v_B.clone(), &Py::Int(2i64))?, &py::sub(&py::mul(&py::truediv(&v_B.clone(), &Py::Int(2i64))?, &v_F.clone())?, &py::truediv(&py::mul(&v_E.clone(), &v_D.clone())?, &Py::Int(4i64))?)?)?)?, &py::mul(&py::truediv(&v_D.clone(), &Py::Int(2i64))?, &py::sub(&py::truediv(&py::mul(&py::truediv(&v_B.clone(), &Py::Int(2i64))?, &v_E.clone())?, &Py::Int(2i64))?, &py::truediv(&py::mul(&v_C.clone(), &v_D.clone())?, &Py::Int(2i64))?)?)?)?;
    if ({ let t400 = v_big.clone(); Py::Bool({ let t401 = Py::Int(0i64); if py::eq(&t400, &t401) { true } else { false } }) }).truthy() {
        return Ok(Py::str("degenerate"));
    }
    if ({ let t402 = v_delta.clone(); Py::Bool({ let t403 = Py::Int(0i64); if py::gt(&t402, &t403)? { true } else { false } }) }).truthy() {
        if ({ let t404 = py::mul(&py::add(&v_A.clone(), &v_C.clone())?, &v_big.clone())?; Py::Bool({ let t405 = Py::Int(0i64); if py::gt(&t404, &t405)? { true } else { false } }) }).truthy() {
            return Ok(Py::str("empty"));
        }
        return Ok((if ({ let t410 = { let t406 = v_A.clone(); Py::Bool({ let t407 = v_C.clone(); if py::eq(&t406, &t407) { true } else { false } }) }; if !t410.truthy() { t410 } else { { let t408 = v_B.clone(); Py::Bool({ let t409 = Py::Int(0i64); if py::eq(&t408, &t409) { true } else { false } }) } } }).truthy() { Py::str("circle") } else { Py::str("ellipse") }));
    }
    return Ok((if ({ let t411 = v_delta.clone(); Py::Bool({ let t412 = Py::Int(0i64); if py::eq(&t411, &t412) { true } else { false } }) }).truthy() { Py::str("parabola") } else { Py::str("hyperbola") }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cross(mut v_o: Py, mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::sub(&py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__deriv(mut v_co: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?;
    return Ok({ let t693 = { let mut out = vec![]; for t691 in py::iter(&(py::enumerate(&py::slice(&v_co.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t692 = (t691).items()?;
if t692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t692[0].clone();
v_c = t692[1].clone();
out.push(py::mul(&v_c.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?); } Py::list(out) }; if t693.truthy() { t693 } else { Py::list(vec![Py::Float(0.0)]) } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t3782 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3782.truthy() { t3782 } else { { let t3781 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3781.truthy() { t3781 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__flat(mut v_pts: Py) -> R {
    return Ok({ let mut out = vec![]; for t618 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t618;
for t619 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t619;
out.push(v_c.clone()); }} Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fr(mut v_v: Py) -> R {
    return Ok((if (Py::Bool(matches!(v_v.clone(), Py::Float(_)))).truthy() { py::fraction(&[Py::str(&py::repr(&py::float(&v_v.clone())?))])? } else { py::fraction(&[v_v.clone()])? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__horner(mut v_co: Py, mut v_x: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = Py::Float(0.0);
    for t695 in py::iter(&(v_co.clone()))? {
        v_c = t695;
        v_v = py::add(&py::mul(&v_v.clone(), &v_x.clone())?, &v_c.clone())?;
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hull(mut v_pts: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_half: Py = Py::None;
    let mut v_lower: Py = Py::None;
    let mut v_upper: Py = Py::None;
    v_P = py::sorted(&py::set(&v_pts.clone())?, (Py::Bool(false)).truthy())?;
    if ({ let t2413 = py::len(&v_P.clone())?; Py::Bool({ let t2414 = Py::Int(3i64); if py::lt(&t2413, &t2414)? { true } else { false } }) }).truthy() {
        return Ok(v_P.clone());
    }
    v_half = { let v_P = v_P.clone(); let v_half = v_half.clone(); let v_pts = v_pts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_seq: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_h: Py = Py::None;
            let mut v_p: Py = Py::None;
            v_h = Py::list(vec![]);
            for t2415 in py::iter(&(v_seq.clone()))? {
                v_p = t2415;
                while ({ let t2420 = { let t2416 = py::len(&v_h.clone())?; Py::Bool({ let t2417 = Py::Int(2i64); if py::ge(&t2416, &t2417)? { true } else { false } }) }; if !t2420.truthy() { t2420 } else { { let t2418 = f__xcross(py::getitem(&v_h.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_h.clone(), &py::neg(&Py::Int(1i64))?)?, v_p.clone())?; Py::Bool({ let t2419 = Py::Int(0i64); if py::le(&t2418, &t2419)? { true } else { false } }) } } }).truthy() {
                    let _ = py::method(&v_h.clone(), "pop", &[])?;
                }
                let _ = py::method(&v_h.clone(), "append", &[v_p.clone()])?;
            }
            return Ok(v_h.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2421 = (Py::tuple(vec![py::call(&v_half.clone(), &[v_P.clone()])?, py::call(&v_half.clone(), &[py::reversed(&v_P.clone())?])?])).items()?;
    if t2421.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lower = t2421[0].clone();
    v_upper = t2421[1].clone();
    return Ok(py::add(&py::slice(&v_lower.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, &py::slice(&v_upper.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__in_circ(mut v_c: Py, mut v_p: Py) -> R {
    return Ok({ let t1413 = py::hypot(&[py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_c.clone(), &Py::Int(1i64))?)?])?; Py::Bool({ let t1414 = py::add(&py::mul(&py::getitem(&v_c.clone(), &Py::Int(2i64))?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?, &Py::Float(1e-12))?; if py::le(&t1413, &t1414)? { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__l3_num(mut v_v: Py) -> R {
    return Ok(Py::str(&[py::format(&py::add(&v_v.clone(), &Py::Float(0.0))?, ".12g")?].concat()));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__l3_texts(mut v_P: Py, mut v_D: Py) -> R {
    let mut v_chain: Py = Py::None;
    let mut v_coef: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_fixed: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_par: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_vec: Py = Py::None;
    v_names = Py::list(vec![Py::str("x"), Py::str("y"), Py::str("z")]);
    v_par = Py::list(vec![]);
    v_chain = Py::list(vec![]);
    v_fixed = Py::list(vec![]);
    for t1195 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v_k = t1195;
        let t1196 = (Py::tuple(vec![py::getitem(&v_P.clone(), &v_k.clone())?, py::getitem(&v_D.clone(), &v_k.clone())?])).items()?;
        if t1196.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t1196[0].clone();
        v_d = t1196[1].clone();
        v_coef = (if ({ let t1197 = py::abs(&v_d.clone())?; Py::Bool({ let t1198 = Py::Int(1i64); if py::eq(&t1197, &t1198) { true } else { false } }) }).truthy() { Py::str("") } else { f__l3_num(py::abs(&v_d.clone())?)? });
        if ({ let t1199 = v_d.clone(); Py::Bool({ let t1200 = Py::Int(0i64); if py::eq(&t1199, &t1200) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_par.clone(), "append", &[py::add(&py::add(&py::getitem(&v_names.clone(), &v_k.clone())?, &Py::str(" = "))?, &f__l3_num(v_p.clone())?)?])?;
            let _ = py::method(&v_fixed.clone(), "append", &[py::add(&py::add(&py::getitem(&v_names.clone(), &v_k.clone())?, &Py::str(" = "))?, &f__l3_num(v_p.clone())?)?])?;
            continue;
        }
        if ({ let t1201 = v_p.clone(); Py::Bool({ let t1202 = Py::Int(0i64); if py::eq(&t1201, &t1202) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_par.clone(), "append", &[py::add(&py::add(&py::add(&py::add(&py::getitem(&v_names.clone(), &v_k.clone())?, &Py::str(" = "))?, &(if ({ let t1203 = v_d.clone(); Py::Bool({ let t1204 = Py::Int(0i64); if py::lt(&t1203, &t1204)? { true } else { false } }) }).truthy() { Py::str("-") } else { Py::str("") }))?, &v_coef.clone())?, &Py::str("t"))?])?;
            v_num = py::getitem(&v_names.clone(), &v_k.clone())?;
        } else {
            let _ = py::method(&v_par.clone(), "append", &[py::add(&py::add(&py::add(&py::add(&py::add(&py::getitem(&v_names.clone(), &v_k.clone())?, &Py::str(" = "))?, &f__l3_num(v_p.clone())?)?, &(if ({ let t1205 = v_d.clone(); Py::Bool({ let t1206 = Py::Int(0i64); if py::lt(&t1205, &t1206)? { true } else { false } }) }).truthy() { Py::str(" - ") } else { Py::str(" + ") }))?, &v_coef.clone())?, &Py::str("t"))?])?;
            v_num = py::add(&py::add(&py::add(&Py::str("("), &py::getitem(&v_names.clone(), &v_k.clone())?)?, &(if ({ let t1207 = v_p.clone(); Py::Bool({ let t1208 = Py::Int(0i64); if py::gt(&t1207, &t1208)? { true } else { false } }) }).truthy() { py::add(&Py::str(" - "), &f__l3_num(v_p.clone())?)? } else { py::add(&Py::str(" + "), &f__l3_num(py::neg(&v_p.clone())?)?)? }))?, &Py::str(")"))?;
        }
        if ({ let t1209 = v_d.clone(); Py::Bool({ let t1210 = Py::Int(1i64); if py::eq(&t1209, &t1210) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_chain.clone(), "append", &[(if ({ let t1211 = py::getitem(&v_num.clone(), &Py::Int(0i64))?; Py::Bool({ let t1212 = Py::str("("); if py::eq(&t1211, &t1212) { true } else { false } }) }).truthy() { py::slice(&v_num.clone(), Some((Py::Int(1i64)).i()?), Some((py::neg(&Py::Int(1i64))?).i()?), None)? } else { v_num.clone() })])?;
        } else {
            if ({ let t1213 = v_d.clone(); Py::Bool({ let t1214 = Py::Int(0i64); if py::lt(&t1213, &t1214)? { true } else { false } }) }).truthy() {
                let _ = py::method(&v_chain.clone(), "append", &[py::add(&py::add(&py::add(&v_num.clone(), &Py::str("/("))?, &f__l3_num(v_d.clone())?)?, &Py::str(")"))?])?;
            } else {
                let _ = py::method(&v_chain.clone(), "append", &[py::add(&py::add(&v_num.clone(), &Py::str("/"))?, &f__l3_num(v_d.clone())?)?])?;
            }
        }
    }
    v_parts = Py::list(vec![]);
    if ({ let t1215 = py::len(&v_chain.clone())?; Py::Bool({ let t1216 = Py::Int(2i64); if py::ge(&t1215, &t1216)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_parts.clone(), "append", &[py::method(&Py::str(" = "), "join", &[v_chain.clone()])?])?;
    }
    for t1217 in py::iter(&(v_fixed.clone()))? {
        v_f = t1217;
        let _ = py::method(&v_parts.clone(), "append", &[v_f.clone()])?;
    }
    v_vec = py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&Py::str("r = ("), &f__l3_num(py::getitem(&v_P.clone(), &Py::Int(0i64))?)?)?, &Py::str(", "))?, &f__l3_num(py::getitem(&v_P.clone(), &Py::Int(1i64))?)?)?, &Py::str(", "))?, &f__l3_num(py::getitem(&v_P.clone(), &Py::Int(2i64))?)?)?, &Py::str(") + t("))?, &f__l3_num(py::getitem(&v_D.clone(), &Py::Int(0i64))?)?)?, &Py::str(", "))?, &f__l3_num(py::getitem(&v_D.clone(), &Py::Int(1i64))?)?)?, &Py::str(", "))?, &f__l3_num(py::getitem(&v_D.clone(), &Py::Int(2i64))?)?)?, &Py::str(")"))?;
    return Ok(Py::tuple(vec![v_vec.clone(), py::method(&Py::str(", "), "join", &[v_par.clone()])?, py::method(&Py::str("; "), "join", &[v_parts.clone()])?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__latlon(mut v_lat: Py, mut v_lon: Py, mut v_name: Py) -> R {
    let t3198 = (Py::tuple(vec![f__fin(v_lat.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " latitude".to_string()].concat()))?, f__fin(v_lon.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " longitude".to_string()].concat()))?])).items()?;
    if t3198.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lat = t3198[0].clone();
    v_lon = t3198[1].clone();
    if (py::not(&{ let t3199 = py::neg(&Py::Int(90i64))?; Py::Bool({ let t3200 = v_lat.clone(); if py::le(&t3199, &t3200)? { { let t3201 = Py::Int(90i64); if py::le(&t3200, &t3201)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": latitude must be between -90 and 90 degrees".to_string()].concat()))));
    }
    if (py::not(&{ let t3202 = py::neg(&Py::Int(360i64))?; Py::Bool({ let t3203 = v_lon.clone(); if py::le(&t3202, &t3203)? { { let t3204 = Py::Int(360i64); if py::le(&t3203, &t3204)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": longitude must be between -360 and 360 degrees".to_string()].concat()))));
    }
    return Ok(Py::tuple(vec![py::math1("radians", &v_lat.clone())?, py::math1("radians", &v_lon.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__line_check(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_name: Py) -> R {
    let t2544 = (Py::tuple(vec![f__fin(v_a.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " a".to_string()].concat()))?, f__fin(v_b.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " b".to_string()].concat()))?, f__fin(v_c.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " c".to_string()].concat()))?])).items()?;
    if t2544.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2544[0].clone();
    v_b = t2544[1].clone();
    v_c = t2544[2].clone();
    if ({ let t2549 = { let t2545 = v_a.clone(); Py::Bool({ let t2546 = Py::Int(0i64); if py::eq(&t2545, &t2546) { true } else { false } }) }; if !t2549.truthy() { t2549 } else { { let t2547 = v_b.clone(); Py::Bool({ let t2548 = Py::Int(0i64); if py::eq(&t2547, &t2548) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": a and b must not both be 0 (a*x + b*y + c = 0 is not a line)".to_string()].concat()))));
    }
    return Ok(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lres(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_x: Py, mut v_y: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::add(&py::add(&py::mul(&v_a.clone(), &v_x.clone())?, &py::mul(&v_b.clone(), &v_y.clone())?)?, &v_c.clone())?)?, &py::minmax(&[py::add(&py::add(&py::abs(&py::mul(&v_a.clone(), &v_x.clone())?)?, &py::abs(&py::mul(&v_b.clone(), &v_y.clone())?)?)?, &py::abs(&v_c.clone())?)?, Py::Float(1e-300)], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__on_segment_exact(mut v_p: Py, mut v_a: Py, mut v_b: Py) -> R {
    if ({ let t2851 = f__xcross(v_a.clone(), v_b.clone(), v_p.clone())?; Py::Bool({ let t2852 = Py::Int(0i64); if py::ne(&t2851, &t2852) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t2859 = { let t2853 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(0i64))?, py::getitem(&v_b.clone(), &Py::Int(0i64))?], false)?; Py::Bool({ let t2854 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; if py::le(&t2853, &t2854)? { { let t2855 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(0i64))?, py::getitem(&v_b.clone(), &Py::Int(0i64))?], true)?; if py::le(&t2854, &t2855)? { true } else { false } } } else { false } }) }; if !t2859.truthy() { t2859 } else { { let t2856 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(1i64))?, py::getitem(&v_b.clone(), &Py::Int(1i64))?], false)?; Py::Bool({ let t2857 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; if py::le(&t2856, &t2857)? { { let t2858 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(1i64))?, py::getitem(&v_b.clone(), &Py::Int(1i64))?], true)?; if py::le(&t2857, &t2858)? { true } else { false } } } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pairs(mut v_xs: Py, mut v_name: Py, mut v_n_min: Py, mut v_n_max: Py) -> R {
    let mut v_pts: Py = Py::None;
    v_xs = { let mut out = vec![]; for t3780 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t3780;
out.push(f__fin(py::float(&v_x.clone())?, v_name.clone())?); } Py::list(out) };
    if (py::modulo(&py::len(&v_xs.clone())?, &Py::Int(2i64))?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": give an x and a y for every point (an even count of numbers)".to_string()].concat()))));
    }
    v_pts = { let mut out = vec![]; for t3783 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::len(&v_xs.clone())?).i()?, (Py::Int(2i64)).i()?]) { let mut v_i: Py; 
v_i = t3783;
out.push(Py::tuple(vec![py::getitem(&v_xs.clone(), &v_i.clone())?, py::getitem(&v_xs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])); } Py::list(out) };
    if ({ let t3784 = py::len(&v_pts.clone())?; Py::Bool({ let t3785 = v_n_min.clone(); if py::lt(&t3784, &t3785)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": needs at least ".to_string(), py::format(&v_n_min.clone(), "")?, " points".to_string()].concat()))));
    }
    if ({ let t3790 = { let t3786 = v_n_max.clone(); Py::Bool({ let t3787 = Py::None; if !py::is(&t3786, &t3787) { true } else { false } }) }; if !t3790.truthy() { t3790 } else { { let t3788 = py::len(&v_pts.clone())?; Py::Bool({ let t3789 = v_n_max.clone(); if py::gt(&t3788, &t3789)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": allows at most ".to_string(), py::format(&v_n_max.clone(), "")?, " points".to_string()].concat()))));
    }
    return Ok(v_pts.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__perimeter(mut v_pts: Py, mut v_closed: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_pts.clone())?;
    v_m = (if (v_closed.clone()).truthy() { v_n.clone() } else { py::sub(&v_n.clone(), &Py::Int(1i64))? });
    return Ok(py::fsum(&{ let mut out = vec![]; for t2444 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t2444;
out.push(py::dist(&py::getitem(&v_pts.clone(), &v_i.clone())?, &py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__plane_check(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_name: Py) -> R {
    let t1892 = ({ let mut out = vec![]; for t1891 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()])))? { let mut v_v: Py; 
v_v = t1891;
out.push(f__fin(v_v.clone(), v_name.clone())?); } Py::list(out) }).items()?;
    if t1892.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1892[0].clone();
    v_b = t1892[1].clone();
    v_c = t1892[2].clone();
    v_d = t1892[3].clone();
    if ({ let t1900 = { let t1893 = v_a.clone(); Py::Bool({ let t1894 = Py::Int(0i64); if py::eq(&t1893, &t1894) { true } else { false } }) }; if !t1900.truthy() { t1900 } else { { let t1899 = { let t1895 = v_b.clone(); Py::Bool({ let t1896 = Py::Int(0i64); if py::eq(&t1895, &t1896) { true } else { false } }) }; if !t1899.truthy() { t1899 } else { { let t1897 = v_c.clone(); Py::Bool({ let t1898 = Py::Int(0i64); if py::eq(&t1897, &t1898) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": a, b, c must not all be 0".to_string()].concat()))));
    }
    return Ok(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__fin(v_v.clone(), v_name.clone())?;
    if ({ let t3647 = v_v.clone(); Py::Bool({ let t3648 = Py::Int(0i64); if py::le(&t3647, &t3648)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_x: Py) -> R {
    if ({ let t3805 = Py::Bool(matches!(v_x.clone(), Py::Float(_))); if !t3805.truthy() { t3805 } else { py::math1("isfinite", &v_x.clone())? } }).truthy() {
        return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    if ({ let t3721 = { let t3717 = v_a.clone(); Py::Bool({ let t3718 = Py::None; if py::is(&t3717, &t3718) { true } else { false } }) }; if t3721.truthy() { t3721 } else { { let t3719 = v_b.clone(); Py::Bool({ let t3720 = Py::None; if py::is(&t3719, &t3720) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t3726 = { let t3722 = v_a.clone(); Py::Bool({ let t3723 = Py::None; if py::is(&t3722, &t3723) { true } else { false } }) }; if !t3726.truthy() { t3726 } else { { let t3724 = v_b.clone(); Py::Bool({ let t3725 = Py::None; if py::is(&t3724, &t3725) { true } else { false } }) } } }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    return Ok(py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__res(mut v_pairs: Py) -> R {
    return Ok({ let t = { let mut out = vec![]; for t3715 in py::iter(&(v_pairs.clone()))? { let mut v_a: Py; let mut v_b: Py; 
let t3716 = (t3715).items()?;
if t3716.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3716[0].clone();
v_b = t3716[1].clone();
out.push(f__rel(v_a.clone(), v_b.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__scale_of(mut v_pts: Py) -> R {
    return Ok(py::minmax(&[{ let mut out = vec![]; for t3799 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t3799;
out.push(py::minmax(&[{ let mut out = vec![]; for t3800 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t3800;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }], true)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__seg_line(mut v_p: Py, mut v_q: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t632 = (Py::tuple(vec![f__cross(v_a.clone(), v_b.clone(), v_p.clone())?, f__cross(v_a.clone(), v_b.clone(), v_q.clone())?])).items()?;
    if t632.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t632[0].clone();
    v_d2 = t632[1].clone();
    v_t = py::truediv(&v_d1.clone(), &py::sub(&v_d1.clone(), &v_d2.clone())?)?;
    return Ok(Py::tuple(vec![py::add(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_q.clone(), &Py::Int(0i64))?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?)?)?, py::add(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_q.clone(), &Py::Int(1i64))?, &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__segments_meet_exact(mut v_p1: Py, mut v_p2: Py, mut v_p3: Py, mut v_p4: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_d3: Py = Py::None;
    let mut v_d4: Py = Py::None;
    let t2832 = (Py::tuple(vec![f__xcross(v_p3.clone(), v_p4.clone(), v_p1.clone())?, f__xcross(v_p3.clone(), v_p4.clone(), v_p2.clone())?])).items()?;
    if t2832.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t2832[0].clone();
    v_d2 = t2832[1].clone();
    let t2833 = (Py::tuple(vec![f__xcross(v_p1.clone(), v_p2.clone(), v_p3.clone())?, f__xcross(v_p1.clone(), v_p2.clone(), v_p4.clone())?])).items()?;
    if t2833.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d3 = t2833[0].clone();
    v_d4 = t2833[1].clone();
    if ({ let t2848 = { let t2840 = { let t2834 = v_d1.clone(); Py::Bool({ let t2835 = Py::Int(0i64); if py::gt(&t2834, &t2835)? { { let t2836 = v_d2.clone(); if py::gt(&t2835, &t2836)? { true } else { false } } } else { false } }) }; if t2840.truthy() { t2840 } else { { let t2837 = v_d1.clone(); Py::Bool({ let t2838 = Py::Int(0i64); if py::lt(&t2837, &t2838)? { { let t2839 = v_d2.clone(); if py::lt(&t2838, &t2839)? { true } else { false } } } else { false } }) } } }; if !t2848.truthy() { t2848 } else { { let t2847 = { let t2841 = v_d3.clone(); Py::Bool({ let t2842 = Py::Int(0i64); if py::gt(&t2841, &t2842)? { { let t2843 = v_d4.clone(); if py::gt(&t2842, &t2843)? { true } else { false } } } else { false } }) }; if t2847.truthy() { t2847 } else { { let t2844 = v_d3.clone(); Py::Bool({ let t2845 = Py::Int(0i64); if py::lt(&t2844, &t2845)? { { let t2846 = v_d4.clone(); if py::lt(&t2845, &t2846)? { true } else { false } } } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(true));
    }
    return Ok({ let t2872 = { let t2860 = { let t2849 = v_d1.clone(); Py::Bool({ let t2850 = Py::Int(0i64); if py::eq(&t2849, &t2850) { true } else { false } }) }; if !t2860.truthy() { t2860 } else { f__on_segment_exact(v_p1.clone(), v_p3.clone(), v_p4.clone())? } }; if t2872.truthy() { t2872 } else { { let t2871 = { let t2863 = { let t2861 = v_d2.clone(); Py::Bool({ let t2862 = Py::Int(0i64); if py::eq(&t2861, &t2862) { true } else { false } }) }; if !t2863.truthy() { t2863 } else { f__on_segment_exact(v_p2.clone(), v_p3.clone(), v_p4.clone())? } }; if t2871.truthy() { t2871 } else { { let t2870 = { let t2866 = { let t2864 = v_d3.clone(); Py::Bool({ let t2865 = Py::Int(0i64); if py::eq(&t2864, &t2865) { true } else { false } }) }; if !t2866.truthy() { t2866 } else { f__on_segment_exact(v_p3.clone(), v_p1.clone(), v_p2.clone())? } }; if t2870.truthy() { t2870 } else { { let t2869 = { let t2867 = v_d4.clone(); Py::Bool({ let t2868 = Py::Int(0i64); if py::eq(&t2867, &t2868) { true } else { false } }) }; if !t2869.truthy() { t2869 } else { f__on_segment_exact(v_p4.clone(), v_p1.clone(), v_p2.clone())? } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__shoelace(mut v_pts: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::fsum(&{ let mut out = vec![]; for t3755 in py::range_iter(&[(py::len(&v_pts.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3755;
out.push(py::sub(&py::mul(&py::getitem(&py::getitem(&v_pts.clone(), &v_i.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &py::len(&v_pts.clone())?)?)?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &py::len(&v_pts.clone())?)?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_i.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) })?;
    return Ok(py::truediv(&v_s.clone(), &Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sim_rel(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?, Py::Float(1e-300)], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__simple_polygon(mut v_pts: Py, mut v_name: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_pts.clone())?;
    if ({ let t2326 = v_n.clone(); Py::Bool({ let t2327 = Py::Int(3i64); if py::lt(&t2326, &t2327)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": a polygon needs at least 3 vertices".to_string()].concat()))));
    }
    if ({ let t2328 = py::len(&py::set(&v_pts.clone())?)?; Py::Bool({ let t2329 = v_n.clone(); if py::ne(&t2328, &t2329) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": a vertex is repeated".to_string()].concat()))));
    }
    if ({ let t2330 = v_n.clone(); Py::Bool({ let t2331 = Py::Int(400i64); if py::gt(&t2330, &t2331)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": at most 400 vertices".to_string()].concat()))));
    }
    for t2332 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2332;
        let t2333 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &v_i.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2333.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2333[0].clone();
        v_b = t2333[1].clone();
        for t2334 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t2334;
            if ({ let t2342 = { let t2335 = v_j.clone(); Py::Bool({ let t2336 = v_i.clone(); if py::eq(&t2335, &t2336) { true } else { false } }) }; if t2342.truthy() { t2342 } else { { let t2341 = { let t2337 = py::modulo(&py::add(&v_j.clone(), &Py::Int(1i64))?, &v_n.clone())?; Py::Bool({ let t2338 = v_i.clone(); if py::eq(&t2337, &t2338) { true } else { false } }) }; if t2341.truthy() { t2341 } else { { let t2339 = v_j.clone(); Py::Bool({ let t2340 = py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?; if py::eq(&t2339, &t2340) { true } else { false } }) } } } } }).truthy() {
                continue;
            }
            let t2343 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &v_j.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_j.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
            if t2343.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t2343[0].clone();
            v_d = t2343[1].clone();
            if (f__segments_meet_exact(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone())?).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": the polygon is not simple (edges ".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?, " and ".to_string(), py::format(&py::add(&v_j.clone(), &Py::Int(1i64))?, "")?, " meet)".to_string()].concat()))));
            }
        }
    }
    for t2387 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2387;
        let t2388 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, py::getitem(&v_pts.clone(), &v_i.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2388.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2388[0].clone();
        v_b = t2388[1].clone();
        v_c = t2388[2].clone();
        if ({ let t2393 = { let t2389 = f__xcross(v_a.clone(), v_b.clone(), v_c.clone())?; Py::Bool({ let t2390 = Py::Int(0i64); if py::eq(&t2389, &t2390) { true } else { false } }) }; if !t2393.truthy() { t2393 } else { { let t2391 = py::add(&py::mul(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?)?)?; Py::Bool({ let t2392 = Py::Int(0i64); if py::lt(&t2391, &t2392)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": the polygon is not simple (an edge doubles back)".to_string()].concat()))));
        }
    }
    if ({ let t2394 = f__shoelace(v_pts.clone())?; Py::Bool({ let t2395 = Py::Int(0i64); if py::eq(&t2394, &t2395) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": the polygon has zero area".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_v: Py, mut v_scale: Py) -> R {
    return Ok((if ({ let t3801 = py::abs(&v_v.clone())?; Py::Bool({ let t3802 = py::mul(&Py::Float(1e-13), &py::minmax(&[Py::Float(1.0), v_scale.clone()], true)?)?; if py::le(&t3801, &t3802)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::add(&v_v.clone(), &Py::Float(0.0))? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sort_pts(mut v_pts: Py) -> R {
    return Ok(py::sorted_key(&v_pts.clone(), &{ let v_pts = v_pts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::round(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, Some((Py::Int(9i64)).i()?))?, py::round(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, Some((Py::Int(9i64)).i()?))?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tri_check(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3649 = (Py::tuple(vec![f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?, f__pos(v_c.clone(), Py::str("c"))?])).items()?;
    if t3649.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3649[0].clone();
    v_b = t3649[1].clone();
    v_c = t3649[2].clone();
    let t3650 = (py::sorted(&Py::tuple(vec![f__fr(v_a.clone())?, f__fr(v_b.clone())?, f__fr(v_c.clone())?]), (Py::Bool(false)).truthy())?).items()?;
    if t3650.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3650[0].clone();
    v_y = t3650[1].clone();
    v_z = t3650[2].clone();
    if ({ let t3651 = py::add(&v_x.clone(), &v_y.clone())?; Py::Bool({ let t3652 = v_z.clone(); if py::le(&t3651, &t3652)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the sides violate the triangle inequality (each side must be shorter than the sum of the other two)"))));
    }
    return Ok(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tri_pts(mut v_points: Py) -> R {
    let mut v_pts: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::Int(3i64))?;
    if ({ let t3791 = { let mut v: Vec<Py> = vec![]; v.extend((v_pts.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "o") { Some(x) => x, None => { return Err(PyErr::Type("_xcross() missing argument o".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("_xcross() missing argument a".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("_xcross() missing argument b".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("_xcross() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_xcross() got an unexpected keyword argument".into())) } f__xcross(a0, a1, a2)? }; Py::Bool({ let t3792 = Py::Int(0i64); if py::eq(&t3791, &t3792) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the three points are collinear: they do not form a triangle"))));
    }
    return Ok(v_pts.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__triples(mut v_xs: Py, mut v_name: Py, mut v_n: Py) -> R {
    v_xs = { let mut out = vec![]; for t3494 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t3494;
out.push(f__fin(py::float(&v_x.clone())?, v_name.clone())?); } Py::list(out) };
    if ({ let t3497 = py::len(&v_xs.clone())?; Py::Bool({ let t3498 = py::mul(&Py::Int(3i64), &v_n.clone())?; if py::ne(&t3497, &t3498) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": give exactly ".to_string(), py::format(&v_n.clone(), "")?, " points of three coordinates".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t3499 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3499;
out.push(Py::tuple((py::slice(&v_xs.clone(), Some((py::mul(&Py::Int(3i64), &v_i.clone())?).i()?), Some((py::add(&py::mul(&Py::Int(3i64), &v_i.clone())?, &Py::Int(3i64))?).i()?), None)?).items()?)); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__two_pts_out(mut v_pts: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_pts = f__sort_pts(v_pts.clone())?;
    v_out = Py::dict(vec![(Py::str("count"), py::len(&v_pts.clone())?)]);
    for t204 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)])))? {
        v_n = t204;
        v_p = (if ({ let t205 = py::len(&v_pts.clone())?; Py::Bool({ let t206 = v_n.clone(); if py::ge(&t205, &t206)? { true } else { false } }) }).truthy() { py::getitem(&v_pts.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)? } else { Py::tuple(vec![Py::None, Py::None]) });
        let t207 = (v_p.clone()).items()?;
        if t207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_out.clone(), &Py::str(&["x".to_string(), py::format(&v_n.clone(), "")?].concat()), t207[0].clone())?;
        py::setitem(&v_out.clone(), &Py::str(&["y".to_string(), py::format(&v_n.clone(), "")?].concat()), t207[1].clone())?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__unit(mut v_lat_r: Py, mut v_lon_r: Py) -> R {
    return Ok(Py::tuple(vec![py::mul(&py::math1("cos", &v_lat_r.clone())?, &py::math1("cos", &v_lon_r.clone())?)?, py::mul(&py::math1("cos", &v_lat_r.clone())?, &py::math1("sin", &v_lon_r.clone())?)?, py::math1("sin", &v_lat_r.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vcross(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(Py::tuple(vec![py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?)?, py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?)?, py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vdot(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::add(&py::add(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_angle_lines(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cr: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t17 = (Py::tuple(vec![py::hypot(&[py::getitem(&v_i.clone(), &Py::str("a1"))?, py::getitem(&v_i.clone(), &Py::str("b1"))?])?, py::hypot(&[py::getitem(&v_i.clone(), &Py::str("a2"))?, py::getitem(&v_i.clone(), &Py::str("b2"))?])?])).items()?;
    if t17.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t17[0].clone();
    v_n2 = t17[1].clone();
    v_cr = py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a1"))?, &py::getitem(&v_i.clone(), &Py::str("b2"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("a2"))?, &py::getitem(&v_i.clone(), &Py::str("b1"))?)?)?)?, &py::mul(&v_n1.clone(), &v_n2.clone())?)?;
    v_dt = py::truediv(&py::abs(&py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a1"))?, &py::getitem(&v_i.clone(), &Py::str("a2"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("b1"))?, &py::getitem(&v_i.clone(), &Py::str("b2"))?)?)?)?, &py::mul(&v_n1.clone(), &v_n2.clone())?)?;
    v_t = py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("angle"))?)?;
    return Ok(py::minmax(&[py::abs(&py::sub(&py::math1("sin", &v_t.clone())?, &v_cr.clone())?)?, py::abs(&py::sub(&py::math1("cos", &v_t.clone())?, &v_dt.clone())?)?, (if ({ let t18 = Py::Int(0i64); Py::Bool({ let t19 = py::getitem(&v_o.clone(), &Py::str("angle"))?; if py::le(&t18, &t19)? { { let t20 = Py::Int(90i64); if py::le(&t19, &t20)? { true } else { false } } } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_bary(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t110 = (f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(3i64))?).items()?;
    if t110.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t110[0].clone();
    v_B = t110[1].clone();
    v_C = t110[2].clone();
    v_sc = py::minmax(&[Py::Float(1.0), f__scale_of(Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()]))?, py::abs(&py::getitem(&v_i.clone(), &Py::str("px"))?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("py"))?)?], true)?;
    v_x = py::add(&py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("l1"))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("l2"))?, &py::getitem(&v_B.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("l3"))?, &py::getitem(&v_C.clone(), &Py::Int(0i64))?)?)?;
    v_y = py::add(&py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("l1"))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("l2"))?, &py::getitem(&v_B.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("l3"))?, &py::getitem(&v_C.clone(), &Py::Int(1i64))?)?)?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::add(&py::add(&py::getitem(&v_o.clone(), &Py::str("l1"))?, &py::getitem(&v_o.clone(), &Py::str("l2"))?)?, &py::getitem(&v_o.clone(), &Py::str("l3"))?)?, &Py::Int(1i64))?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::add(&py::abs(&py::getitem(&v_o.clone(), &Py::str("l1"))?)?, &py::abs(&py::getitem(&v_o.clone(), &Py::str("l2"))?)?)?, &py::abs(&py::getitem(&v_o.clone(), &Py::str("l3"))?)?)?], true)?)?, py::truediv(&py::abs(&py::sub(&v_x.clone(), &py::getitem(&v_i.clone(), &Py::str("px"))?)?)?, &v_sc.clone())?, py::truediv(&py::abs(&py::sub(&v_y.clone(), &py::getitem(&v_i.clone(), &Py::str("py"))?)?)?, &v_sc.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_bearing(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_east: Py = Py::None;
    let mut v_f1: Py = Py::None;
    let mut v_f2: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gn: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_north: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tdir: Py = Py::None;
    let t1145 = ({ let mut out = vec![]; for t1144 in py::iter(&(Py::tuple(vec![Py::str("lat1"), Py::str("lon1"), Py::str("lat2"), Py::str("lon2")])))? { let mut v_k: Py; 
v_k = t1144;
out.push(py::math1("radians", &py::getitem(&v_i.clone(), &v_k.clone())?)?); } Py::list(out) }).items()?;
    if t1145.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_f1 = t1145[0].clone();
    v_l1 = t1145[1].clone();
    v_f2 = t1145[2].clone();
    v_l2 = t1145[3].clone();
    v_b = py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("bearing"))?)?;
    v_a = f__unit(v_f1.clone(), v_l1.clone())?;
    let t1146 = (Py::tuple(vec![Py::tuple(vec![py::neg(&py::math1("sin", &v_l1.clone())?)?, py::math1("cos", &v_l1.clone())?, Py::Float(0.0)]), Py::tuple(vec![py::mul(&py::neg(&py::math1("sin", &v_f1.clone())?)?, &py::math1("cos", &v_l1.clone())?)?, py::mul(&py::neg(&py::math1("sin", &v_f1.clone())?)?, &py::math1("sin", &v_l1.clone())?)?, py::math1("cos", &v_f1.clone())?])])).items()?;
    if t1146.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_east = t1146[0].clone();
    v_north = t1146[1].clone();
    v_tdir = Py::tuple(({ let mut out = vec![]; for t1147 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t1147;
out.push(py::add(&py::mul(&py::math1("sin", &v_b.clone())?, &py::getitem(&v_east.clone(), &v_j.clone())?)?, &py::mul(&py::math1("cos", &v_b.clone())?, &py::getitem(&v_north.clone(), &v_j.clone())?)?)?); } Py::list(out) }).items()?);
    v_t = f__unit(v_f2.clone(), v_l2.clone())?;
    v_g = Py::tuple(({ let mut out = vec![]; for t1148 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t1148;
out.push(py::sub(&py::getitem(&v_t.clone(), &v_j.clone())?, &py::mul(&f__vdot(v_a.clone(), v_t.clone())?, &py::getitem(&v_a.clone(), &v_j.clone())?)?)?); } Py::list(out) }).items()?);
    v_gn = f__vnorm(v_g.clone())?;
    return Ok(py::add(&py::abs(&py::sub(&py::truediv(&f__vdot(v_tdir.clone(), v_g.clone())?, &v_gn.clone())?, &Py::Int(1i64))?)?, &py::truediv(&f__vnorm(f__vcross(v_tdir.clone(), v_g.clone())?)?, &v_gn.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C1: Py = Py::None;
    let mut v_C2: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t152 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x1"))?, py::getitem(&v_i.clone(), &Py::str("y1"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x2"))?, py::getitem(&v_i.clone(), &Py::str("y2"))?])])).items()?;
    if t152.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C1 = t152[0].clone();
    v_C2 = t152[1].clone();
    v_pts = { let mut out = vec![]; for t153 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)])))? { let mut v_n: Py; 
v_n = t153;
if ({ let t154 = py::getitem(&v_o.clone(), &Py::str(&["x".to_string(), py::format(&v_n.clone(), "")?].concat()))?; Py::Bool({ let t155 = Py::None; if !py::is(&t154, &t155) { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str(&["x".to_string(), py::format(&v_n.clone(), "")?].concat()))?, py::getitem(&v_o.clone(), &Py::str(&["y".to_string(), py::format(&v_n.clone(), "")?].concat()))?])); }} Py::list(out) };
    if ({ let t156 = py::len(&v_pts.clone())?; Py::Bool({ let t157 = py::getitem(&v_o.clone(), &Py::str("count"))?; if py::ne(&t156, &t157) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_d = py::dist(&v_C1.clone(), &v_C2.clone())?;
    if ({ let t158 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t159 = Py::Int(0i64); if py::eq(&t158, &t159) { true } else { false } }) }).truthy() {
        return Ok((if ({ let t170 = { let t160 = v_d.clone(); Py::Bool({ let t161 = py::add(&py::getitem(&v_i.clone(), &Py::str("r1"))?, &py::getitem(&v_i.clone(), &Py::str("r2"))?)?; if py::gt(&t160, &t161)? { true } else { false } }) }; if t170.truthy() { t170 } else { { let t169 = { let t162 = v_d.clone(); Py::Bool({ let t163 = py::abs(&py::sub(&py::getitem(&v_i.clone(), &Py::str("r1"))?, &py::getitem(&v_i.clone(), &Py::str("r2"))?)?)?; if py::lt(&t162, &t163)? { true } else { false } }) }; if t169.truthy() { t169 } else { { let t168 = { let t164 = v_d.clone(); Py::Bool({ let t165 = Py::Int(0i64); if py::eq(&t164, &t165) { true } else { false } }) }; if !t168.truthy() { t168 } else { { let t166 = py::getitem(&v_i.clone(), &Py::str("r1"))?; Py::Bool({ let t167 = py::getitem(&v_i.clone(), &Py::str("r2"))?; if py::ne(&t166, &t167) { true } else { false } }) } } } } } } }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    v_worst = py::minmax(&[{ let mut out = vec![]; for t171 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t171;
out.push(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::dist(&v_p.clone(), &v_C1.clone())?, &py::getitem(&v_i.clone(), &Py::str("r1"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("r1"))?)?, py::truediv(&py::abs(&py::sub(&py::dist(&v_p.clone(), &v_C2.clone())?, &py::getitem(&v_i.clone(), &Py::str("r2"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("r2"))?)?], true)?); } Py::list(out) }], true)?;
    if ({ let t176 = { let t172 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t173 = Py::Int(2i64); if py::eq(&t172, &t173) { true } else { false } }) }; if !t176.truthy() { t176 } else { { let t174 = { let a: &[Py] = &({ let mut v: Vec<Py> = vec![]; v.extend((v_pts.clone()).items()?); v })[..]; if a.len() != 2 { return Err(PyErr::Type("dist expected 2 arguments".into())); } py::dist(&a[0], &a[1])? }; Py::Bool({ let t175 = Py::Int(0i64); if py::eq(&t174, &t175) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_centroid(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_area: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_xa: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let mut v_ya: Py = Py::None;
    let mut v_yb: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::None)?;
    let t2165 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("cx"))?, py::getitem(&v_o.clone(), &Py::str("cy"))?])).items()?;
    if t2165.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t2165[0].clone();
    v_cy = t2165[1].clone();
    v_n = py::len(&v_pts.clone())?;
    let t2166 = Py::Float(0.0);
    v_mx = t2166.clone();
    v_my = t2166.clone();
    v_area = t2166.clone();
    v_scale = py::minmax(&[{ let mut out = vec![]; for t2167 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2167;
out.push(py::minmax(&[py::abs(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &v_cx.clone())?)?, py::abs(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_cy.clone())?)?], true)?); } Py::list(out) }], true)?;
    for t2168 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2168;
        let t2169 = (Py::tuple(vec![Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?, &v_cx.clone())?, py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?, &v_cy.clone())?]), Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &v_cx.clone())?, py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?, &v_cy.clone())?])])).items()?;
        if t2169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t2170 = (t2169[0].clone()).items()?;
        if t2170.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xa = t2170[0].clone();
        v_ya = t2170[1].clone();
        let t2171 = (t2169[1].clone()).items()?;
        if t2171.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xb = t2171[0].clone();
        v_yb = t2171[1].clone();
        v_t = py::truediv(&py::sub(&py::mul(&v_xa.clone(), &v_yb.clone())?, &py::mul(&v_xb.clone(), &v_ya.clone())?)?, &Py::Int(2i64))?;
        v_area = py::iadd(&v_area, &v_t.clone())?;
        v_mx = py::iadd(&v_mx, &py::truediv(&py::mul(&v_t.clone(), &py::add(&v_xa.clone(), &v_xb.clone())?)?, &Py::Int(3i64))?)?;
        v_my = py::iadd(&v_my, &py::truediv(&py::mul(&v_t.clone(), &py::add(&v_ya.clone(), &v_yb.clone())?)?, &Py::Int(3i64))?)?;
    }
    return Ok(py::truediv(&py::minmax(&[py::abs(&v_mx.clone())?, py::abs(&v_my.clone())?], true)?, &py::mul(&py::abs(&v_area.clone())?, &py::minmax(&[v_scale.clone(), Py::Float(1.0)], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cevians(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cosC: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mm: Py = Py::None;
    let mut v_nn: Py = Py::None;
    let mut v_o_: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t3579 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t3579.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3579[0].clone();
    v_b = t3579[1].clone();
    v_c = t3579[2].clone();
    v_out = Py::list(vec![]);
    for t3580 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), Py::str("a")]), Py::tuple(vec![v_b.clone(), v_a.clone(), v_c.clone(), Py::str("b")]), Py::tuple(vec![v_c.clone(), v_a.clone(), v_b.clone(), Py::str("c")])])))? {
        let t3582 = (t3580).items()?;
        if t3582.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_o_ = t3582[0].clone();
        v_p = t3582[1].clone();
        v_q = t3582[2].clone();
        v_s = t3582[3].clone();
        v_m = py::getitem(&v_o.clone(), &Py::str(&["median_".to_string(), py::format(&v_s.clone(), "")?].concat()))?;
        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![py::add(&py::truediv(&py::mul(&py::mul(&v_p.clone(), &v_p.clone())?, &v_o_.clone())?, &Py::Int(2i64))?, &py::truediv(&py::mul(&py::mul(&v_q.clone(), &v_q.clone())?, &v_o_.clone())?, &Py::Int(2i64))?)?, py::mul(&v_o_.clone(), &py::add(&py::mul(&v_m.clone(), &v_m.clone())?, &py::truediv(&py::mul(&v_o_.clone(), &v_o_.clone())?, &Py::Int(4i64))?)?)?])])?;
        let t3581 = (Py::tuple(vec![py::truediv(&py::mul(&v_o_.clone(), &v_q.clone())?, &py::add(&v_p.clone(), &v_q.clone())?)?, py::truediv(&py::mul(&v_o_.clone(), &v_p.clone())?, &py::add(&v_p.clone(), &v_q.clone())?)?])).items()?;
        if t3581.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_mm = t3581[0].clone();
        v_nn = t3581[1].clone();
        v_t = py::getitem(&v_o.clone(), &Py::str(&["bisector_".to_string(), py::format(&v_s.clone(), "")?].concat()))?;
        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![py::add(&py::mul(&py::mul(&v_p.clone(), &v_p.clone())?, &v_mm.clone())?, &py::mul(&py::mul(&v_q.clone(), &v_q.clone())?, &v_nn.clone())?)?, py::mul(&v_o_.clone(), &py::add(&py::mul(&v_t.clone(), &v_t.clone())?, &py::mul(&v_mm.clone(), &v_nn.clone())?)?)?])])?;
    }
    v_cosC = py::truediv(&py::sub(&py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?)?;
    v_K = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_a.clone())?, &v_b.clone())?, &py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::sub(&Py::Int(1i64), &py::mul(&v_cosC.clone(), &v_cosC.clone())?)?], true)?)?)?;
    v_out = py::iadd(&v_out, &Py::list(vec![Py::tuple(vec![py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("altitude_a"))?)?, py::mul(&Py::Int(2i64), &v_K.clone())?]), Py::tuple(vec![py::mul(&v_b.clone(), &py::getitem(&v_o.clone(), &Py::str("altitude_b"))?)?, py::mul(&Py::Int(2i64), &v_K.clone())?]), Py::tuple(vec![py::mul(&v_c.clone(), &py::getitem(&v_o.clone(), &Py::str("altitude_c"))?)?, py::mul(&Py::Int(2i64), &v_K.clone())?])]))?;
    return Ok((if (Py::Bool(({ let mut out = vec![]; for t3583 in py::iter(&(v_out.clone()))? { let mut v__: Py; let mut v_u: Py; 
let t3584 = (t3583).items()?;
if t3584.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t3584[0].clone();
v__ = t3584[1].clone();
out.push(py::math1("isfinite", &v_u.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() { py::minmax(&[{ let mut out = vec![]; for t3585 in py::iter(&(v_out.clone()))? { let mut v_u: Py; let mut v_v: Py; 
let t3586 = (t3585).items()?;
if t3586.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t3586[0].clone();
v_v = t3586[1].clone();
out.push(f__rel(v_u.clone(), v_v.clone())?); } Py::list(out) }], true)? } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cint(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_sc: Py = Py::None;
    v_P = f__ccw_convex(f__pairs(py::getitem(&v_i.clone(), &Py::str("p"))?, Py::str("p"), Py::Int(3i64), Py::None)?, Py::str("p"))?;
    v_Q = f__ccw_convex(f__pairs(py::getitem(&v_i.clone(), &Py::str("q"))?, Py::str("q"), Py::Int(3i64), Py::None)?, Py::str("q"))?;
    v_R = f__clip(v_P.clone(), v_Q.clone())?;
    v_a = (if ({ let t639 = py::len(&v_R.clone())?; Py::Bool({ let t640 = Py::Int(3i64); if py::ge(&t639, &t640)? { true } else { false } }) }).truthy() { py::abs(&f__shoelace(v_R.clone())?)? } else { Py::Float(0.0) });
    v_sc = py::minmax(&[py::abs(&f__shoelace(v_P.clone())?)?, py::abs(&f__shoelace(v_Q.clone())?)?], true)?;
    return Ok((if ({ let t641 = v_a.clone(); if t641.truthy() { t641 } else { py::getitem(&v_o.clone(), &Py::str("area"))? } }).truthy() { py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("area"))?, &v_a.clone())?)?, &py::minmax(&[v_a.clone(), py::mul(&Py::Float(1e-12), &v_sc.clone())?, Py::Float(1e-300)], true)?)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_circle_line(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t216 = ({ let mut out = vec![]; for t215 in py::iter(&(Py::tuple(vec![Py::str("cx"), Py::str("cy"), Py::str("r"), Py::str("a"), Py::str("b"), Py::str("c")])))? { let mut v_k: Py; 
v_k = t215;
out.push(py::getitem(&v_i.clone(), &v_k.clone())?); } Py::list(out) }).items()?;
    if t216.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t216[0].clone();
    v_cy = t216[1].clone();
    v_r = t216[2].clone();
    v_a = t216[3].clone();
    v_b = t216[4].clone();
    v_c = t216[5].clone();
    v_d = py::truediv(&py::abs(&py::add(&py::add(&py::mul(&v_a.clone(), &v_cx.clone())?, &py::mul(&v_b.clone(), &v_cy.clone())?)?, &v_c.clone())?)?, &py::hypot(&[v_a.clone(), v_b.clone()])?)?;
    v_pts = { let mut out = vec![]; for t217 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)])))? { let mut v_n: Py; 
v_n = t217;
if ({ let t218 = py::getitem(&v_o.clone(), &Py::str(&["x".to_string(), py::format(&v_n.clone(), "")?].concat()))?; Py::Bool({ let t219 = Py::None; if !py::is(&t218, &t219) { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str(&["x".to_string(), py::format(&v_n.clone(), "")?].concat()))?, py::getitem(&v_o.clone(), &Py::str(&["y".to_string(), py::format(&v_n.clone(), "")?].concat()))?])); }} Py::list(out) };
    if ({ let t225 = { let t220 = py::len(&v_pts.clone())?; Py::Bool({ let t221 = py::getitem(&v_o.clone(), &Py::str("count"))?; if py::ne(&t220, &t221) { true } else { false } }) }; if t225.truthy() { t225 } else { Py::Bool(({ let mut out = vec![]; for t222 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::getitem(&v_o.clone(), &Py::str("count"))?, &Py::Int(1i64))?).i()?]) { let mut v_n: Py; 
v_n = t222;
out.push({ let t223 = py::getitem(&v_o.clone(), &Py::str(&["x".to_string(), py::format(&v_n.clone(), "")?].concat()))?; Py::Bool({ let t224 = Py::None; if py::is(&t223, &t224) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t226 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t227 = Py::Int(0i64); if py::eq(&t226, &t227) { true } else { false } }) }).truthy() {
        return Ok((if ({ let t228 = v_d.clone(); Py::Bool({ let t229 = py::mul(&v_r.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t228, &t229)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    v_worst = py::minmax(&[{ let mut out = vec![]; for t230 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t230;
out.push(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::dist(&v_p.clone(), &Py::tuple(vec![v_cx.clone(), v_cy.clone()]))?, &v_r.clone())?)?, &v_r.clone())?, py::truediv(&py::abs(&py::add(&py::add(&py::mul(&v_a.clone(), &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?)?, &v_c.clone())?)?, &py::mul(&py::hypot(&[v_a.clone(), v_b.clone()])?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, py::abs(&py::getitem(&v_p.clone(), &Py::Int(1i64))?)?, v_r.clone()], true)?)?)?], true)?); } Py::list(out) }], true)?;
    if ({ let t231 = py::getitem(&v_o.clone(), &Py::str("count"))?; Py::Bool({ let t232 = Py::Int(1i64); if py::eq(&t231, &t232) { true } else { false } }) }).truthy() {
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_d.clone(), &v_r.clone())?)?, &v_r.clone())?], true)?;
    } else {
        if ({ let t233 = { let a: &[Py] = &({ let mut v: Vec<Py> = vec![]; v.extend((v_pts.clone()).items()?); v })[..]; if a.len() != 2 { return Err(PyErr::Type("dist expected 2 arguments".into())); } py::dist(&a[0], &a[1])? }; Py::Bool({ let t234 = Py::Int(0i64); if py::eq(&t233, &t234) { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_circum(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_pts: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(3i64))?;
    return Ok(py::minmax(&[{ let mut out = vec![]; for t3633 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t3633;
out.push(py::truediv(&py::abs(&py::sub(&py::dist(&Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("cx"))?, py::getitem(&v_o.clone(), &Py::str("cy"))?]), &v_p.clone())?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_classify(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_eq: Py = Py::None;
    let mut v_largest_cos: Py = Py::None;
    let mut v_want: Py = Py::None;
    let t3670 = (py::sorted(&Py::tuple(vec![f__fr(py::getitem(&v_i.clone(), &Py::str("a"))?)?, f__fr(py::getitem(&v_i.clone(), &Py::str("b"))?)?, f__fr(py::getitem(&v_i.clone(), &Py::str("c"))?)?]), (Py::Bool(false)).truthy())?).items()?;
    if t3670.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3670[0].clone();
    v_b = t3670[1].clone();
    v_c = t3670[2].clone();
    v_largest_cos = py::truediv(&py::sub(&py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?)?;
    v_want = (if ({ let t3671 = v_largest_cos.clone(); Py::Bool({ let t3672 = Py::Int(0i64); if py::eq(&t3671, &t3672) { true } else { false } }) }).truthy() { Py::str("right") } else { (if ({ let t3673 = v_largest_cos.clone(); Py::Bool({ let t3674 = Py::Int(0i64); if py::lt(&t3673, &t3674)? { true } else { false } }) }).truthy() { Py::str("obtuse") } else { Py::str("acute") }) });
    v_eq = py::len(&py::set(&Py::list(vec![v_a.clone(), v_b.clone(), v_c.clone()]))?)?;
    return Ok({ let t3679 = { let t3675 = py::getitem(&v_o.clone(), &Py::str("by_angles"))?; Py::Bool({ let t3676 = v_want.clone(); if py::eq(&t3675, &t3676) { true } else { false } }) }; if !t3679.truthy() { t3679 } else { { let t3677 = py::getitem(&v_o.clone(), &Py::str("by_sides"))?; Py::Bool({ let t3678 = py::getitem(&Py::dict(vec![(Py::Int(1i64), Py::str("equilateral")), (Py::Int(2i64), Py::str("isosceles")), (Py::Int(3i64), Py::str("scalene"))]), &v_eq.clone())?; if py::eq(&t3677, &t3678) { true } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_closest(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d2: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(2i64), Py::Int(2000i64))?;
    v_d2 = py::minmax(&[{ let mut out = vec![]; for t338 in py::iter(&(py::itertools("combinations", &[v_pts.clone(), Py::Int(2i64)])?))? { let mut v_a: Py; let mut v_b: Py; 
let t339 = (t338).items()?;
if t339.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t339[0].clone();
v_b = t339[1].clone();
out.push(py::add(&py::pow(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?); } Py::list(out) }], false)?;
    return Ok((if (v_d2.clone()).truthy() { py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("distance"))?, &py::math1("sqrt", &v_d2.clone())?)?)?, &py::minmax(&[py::math1("sqrt", &v_d2.clone())?, Py::Float(1e-300)], true)?)? } else { py::float(&{ let t340 = py::getitem(&v_o.clone(), &Py::str("distance"))?; Py::Bool({ let t341 = Py::Int(0i64); if py::ne(&t340, &t341) { true } else { false } }) })? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_conic(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f0: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_y0: Py = Py::None;
    let t418 = ({ let mut out = vec![]; for t417 in py::iter(&(Py::str("ABCDEF")))? { let mut v_k: Py; 
v_k = t417;
out.push(f__fr(py::getitem(&v_i.clone(), &v_k.clone())?)?); } Py::list(out) }).items()?;
    if t418.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t418[0].clone();
    v_B = t418[1].clone();
    v_C = t418[2].clone();
    v_D = t418[3].clone();
    v_E = t418[4].clone();
    v_F = t418[5].clone();
    v_disc = py::sub(&py::mul(&py::mul(&Py::Int(4i64), &v_A.clone())?, &v_C.clone())?, &py::mul(&v_B.clone(), &v_B.clone())?)?;
    if ({ let t419 = v_disc.clone(); Py::Bool({ let t420 = Py::Int(0i64); if py::ne(&t419, &t420) { true } else { false } }) }).truthy() {
        v_x0 = py::truediv(&py::sub(&py::mul(&v_B.clone(), &v_E.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_C.clone())?, &v_D.clone())?)?, &v_disc.clone())?;
        v_y0 = py::truediv(&py::sub(&py::mul(&v_B.clone(), &v_D.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_E.clone())?)?, &v_disc.clone())?;
        v_f0 = py::add(&py::add(&py::add(&py::add(&py::add(&py::mul(&py::mul(&v_A.clone(), &v_x0.clone())?, &v_x0.clone())?, &py::mul(&py::mul(&v_B.clone(), &v_x0.clone())?, &v_y0.clone())?)?, &py::mul(&py::mul(&v_C.clone(), &v_y0.clone())?, &v_y0.clone())?)?, &py::mul(&v_D.clone(), &v_x0.clone())?)?, &py::mul(&v_E.clone(), &v_y0.clone())?)?, &v_F.clone())?;
        if ({ let t421 = v_f0.clone(); Py::Bool({ let t422 = Py::Int(0i64); if py::eq(&t421, &t422) { true } else { false } }) }).truthy() {
            v_want = Py::str("degenerate");
        } else {
            if ({ let t423 = v_disc.clone(); Py::Bool({ let t424 = Py::Int(0i64); if py::gt(&t423, &t424)? { true } else { false } }) }).truthy() {
                v_want = (if ({ let t425 = { let t427 = v_f0.clone(); Py::Bool({ let t428 = Py::Int(0i64); if py::gt(&t427, &t428)? { true } else { false } }) }; Py::Bool({ let t426 = { let t429 = v_A.clone(); Py::Bool({ let t430 = Py::Int(0i64); if py::gt(&t429, &t430)? { true } else { false } }) }; if py::eq(&t425, &t426) { true } else { false } }) }).truthy() { Py::str("empty") } else { (if ({ let t435 = { let t431 = v_A.clone(); Py::Bool({ let t432 = v_C.clone(); if py::eq(&t431, &t432) { true } else { false } }) }; if !t435.truthy() { t435 } else { { let t433 = v_B.clone(); Py::Bool({ let t434 = Py::Int(0i64); if py::eq(&t433, &t434) { true } else { false } }) } } }).truthy() { Py::str("circle") } else { Py::str("ellipse") }) });
            } else {
                v_want = Py::str("hyperbola");
            }
        }
    } else {
        let t436 = (Py::tuple(vec![py::truediv(&v_B.clone(), &Py::Int(2i64))?, py::truediv(&v_D.clone(), &Py::Int(2i64))?, py::truediv(&v_E.clone(), &Py::Int(2i64))?])).items()?;
        if t436.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_b = t436[0].clone();
        v_d = t436[1].clone();
        v_e = t436[2].clone();
        v_det = py::add(&py::sub(&py::mul(&v_A.clone(), &py::sub(&py::mul(&v_C.clone(), &v_F.clone())?, &py::mul(&v_e.clone(), &v_e.clone())?)?)?, &py::mul(&v_b.clone(), &py::sub(&py::mul(&v_b.clone(), &v_F.clone())?, &py::mul(&v_e.clone(), &v_d.clone())?)?)?)?, &py::mul(&v_d.clone(), &py::sub(&py::mul(&v_b.clone(), &v_e.clone())?, &py::mul(&v_C.clone(), &v_d.clone())?)?)?)?;
        v_want = (if ({ let t437 = v_det.clone(); Py::Bool({ let t438 = Py::Int(0i64); if py::eq(&t437, &t438) { true } else { false } }) }).truthy() { Py::str("degenerate") } else { Py::str("parabola") });
    }
    return Ok({ let t439 = py::getitem(&v_o.clone(), &Py::str("type"))?; Py::Bool({ let t440 = v_want.clone(); if py::eq(&t439, &t440) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_convex(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_exact: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::None)?;
    v_exact = { let v_i = v_i.clone(); let v_o = v_o.clone(); let v_pts = v_pts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_P: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::sum(&{ let mut out = vec![]; for t2408 in py::range_iter(&[(py::len(&v_P.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2408;
out.push(py::sub(&py::mul(&f__fr(py::getitem(&py::getitem(&v_P.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &f__fr(py::getitem(&py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &py::len(&v_P.clone())?)?)?, &Py::Int(1i64))?)?)?, &py::mul(&f__fr(py::getitem(&py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &py::len(&v_P.clone())?)?)?, &Py::Int(0i64))?)?, &f__fr(py::getitem(&py::getitem(&v_P.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?)?); } Py::list(out) }, None)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok({ let t2409 = py::getitem(&v_o.clone(), &Py::str("convex"))?; Py::Bool({ let t2410 = (if ({ let t2411 = py::call(&v_exact.clone(), &[v_pts.clone()])?; Py::Bool({ let t2412 = py::call(&v_exact.clone(), &[f__hull(v_pts.clone())?])?; if py::eq(&t2411, &t2412) { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }); if py::eq(&t2409, &t2410) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_curv(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k_fd: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_yp: Py = Py::None;
    let mut v_ypp: Py = Py::None;
    let mut v_ypp_fd: Py = Py::None;
    v_co = { let mut out = vec![]; for t701 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("coefficients"))?))? { let mut v_c: Py; 
v_c = t701;
out.push(py::float(&v_c.clone())?); } Py::list(out) };
    v_x0 = py::getitem(&v_i.clone(), &Py::str("x0"))?;
    v_h = py::mul(&Py::Float(0.001), &py::minmax(&[Py::Float(1.0), py::abs(&v_x0.clone())?], true)?)?;
    v_yp = f__horner(f__deriv(v_co.clone())?, v_x0.clone())?;
    v_f = { let v_co = v_co.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_x0 = v_x0.clone(); let v_yp = v_yp.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__horner(v_co.clone(), v_x.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_ypp_fd = py::truediv(&py::sub(&py::add(&py::sub(&py::add(&py::neg(&py::call(&v_f.clone(), &[py::add(&v_x0.clone(), &py::mul(&Py::Int(2i64), &v_h.clone())?)?])?)?, &py::mul(&Py::Int(16i64), &py::call(&v_f.clone(), &[py::add(&v_x0.clone(), &v_h.clone())?])?)?)?, &py::mul(&Py::Int(30i64), &py::call(&v_f.clone(), &[v_x0.clone()])?)?)?, &py::mul(&Py::Int(16i64), &py::call(&v_f.clone(), &[py::sub(&v_x0.clone(), &v_h.clone())?])?)?)?, &py::call(&v_f.clone(), &[py::sub(&v_x0.clone(), &py::mul(&Py::Int(2i64), &v_h.clone())?)?])?)?, &py::mul(&py::mul(&Py::Int(12i64), &v_h.clone())?, &v_h.clone())?)?;
    v_k_fd = py::truediv(&py::abs(&v_ypp_fd.clone())?, &py::pow(&py::add(&Py::Int(1i64), &py::mul(&v_yp.clone(), &v_yp.clone())?)?, &Py::Float(1.5))?)?;
    if ({ let t702 = py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("curvature"))?, &v_k_fd.clone())?)?; Py::Bool({ let t703 = py::mul(&Py::Float(0.0001), &py::minmax(&[Py::Float(1.0), v_k_fd.clone()], true)?)?; if py::gt(&t702, &t703)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_ypp = f__horner(f__deriv(f__deriv(v_co.clone())?)?, v_x0.clone())?;
    v_r = f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("slope"))?, v_yp.clone()]), Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("curvature"))?, &py::pow(&py::add(&Py::Int(1i64), &py::mul(&v_yp.clone(), &v_yp.clone())?)?, &Py::Float(1.5))?)?, py::abs(&v_ypp.clone())?])]))?;
    if (py::getitem(&v_o.clone(), &Py::str("curvature"))?).truthy() {
        v_r = py::minmax(&[v_r.clone(), f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("radius_of_curvature"))?, &py::getitem(&v_o.clone(), &Py::str("curvature"))?)?, Py::Float(1.0))?], true)?;
    }
    return Ok(v_r.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_delaunay(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::floordiv(&py::len(&py::getitem(&v_i.clone(), &Py::str("points"))?)?, &Py::Int(2i64))?;
    v_h = py::getitem(&v_o.clone(), &Py::str("hull_vertices"))?;
    return Ok({ let t813 = { let t800 = py::add(&py::sub(&v_n.clone(), &py::getitem(&v_o.clone(), &Py::str("edges"))?)?, &py::add(&py::getitem(&v_o.clone(), &Py::str("triangles"))?, &Py::Int(1i64))?)?; Py::Bool({ let t801 = Py::Int(2i64); if py::eq(&t800, &t801) { true } else { false } }) }; if !t813.truthy() { t813 } else { { let t812 = { let t802 = py::add(&py::mul(&Py::Int(3i64), &py::getitem(&v_o.clone(), &Py::str("triangles"))?)?, &v_h.clone())?; Py::Bool({ let t803 = py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("edges"))?)?; if py::eq(&t802, &t803) { true } else { false } }) }; if !t812.truthy() { t812 } else { { let t811 = { let t804 = py::getitem(&v_o.clone(), &Py::str("voronoi_vertices"))?; Py::Bool({ let t805 = py::getitem(&v_o.clone(), &Py::str("triangles"))?; if py::eq(&t804, &t805) { true } else { false } }) }; if !t811.truthy() { t811 } else { { let t810 = { let t806 = py::getitem(&v_o.clone(), &Py::str("unbounded_cells"))?; Py::Bool({ let t807 = v_h.clone(); if py::eq(&t806, &t807) { true } else { false } }) }; if !t810.truthy() { t810 } else { { let t808 = v_h.clone(); Py::Bool({ let t809 = py::len(&f__hull(f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(25i64))?)?)?; if py::eq(&t808, &t809) { true } else { false } }) } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dilate(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t835 = (Py::tuple(vec![py::method(&v_i.clone(), "get", &[Py::str("cx"), Py::Float(0.0)])?, py::method(&v_i.clone(), "get", &[Py::str("cy"), Py::Float(0.0)])?])).items()?;
    if t835.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t835[0].clone();
    v_cy = t835[1].clone();
    v_sc = py::add(&py::add(&py::mul(&py::minmax(&[Py::Float(1.0), py::abs(&py::sub(&py::getitem(&v_i.clone(), &Py::str("px"))?, &v_cx.clone())?)?, py::abs(&py::sub(&py::getitem(&v_i.clone(), &Py::str("py"))?, &v_cy.clone())?)?], true)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("k"))?)?], true)?)?, &py::abs(&v_cx.clone())?)?, &py::abs(&v_cy.clone())?)?;
    return Ok(py::truediv(&py::minmax(&[py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("x"))?, &v_cx.clone())?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("px"))?, &v_cx.clone())?)?)?)?, py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("y"))?, &v_cy.clone())?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("py"))?, &v_cy.clone())?)?)?)?], true)?, &v_sc.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ellipse_perimeter(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t898 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])).items()?;
    if t898.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t898[0].clone();
    v_b = t898[1].clone();
    v_n = Py::Int(4096i64);
    v_s = py::fsum(&{ let mut out = vec![]; for t899 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t899;
out.push(py::math1("sqrt", &py::add(&py::pow(&py::mul(&v_a.clone(), &py::math1("sin", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_n.clone())?)?)?, &Py::Int(2i64))?, &py::pow(&py::mul(&v_b.clone(), &py::math1("cos", &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_n.clone())?)?)?, &Py::Int(2i64))?)?)?); } Py::list(out) })?;
    v_P = py::truediv(&py::mul(&py::mul(&v_s.clone(), &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?, &v_n.clone())?;
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, v_P.clone()]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_a.clone())?, &v_b.clone())?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_farthest(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(2i64), Py::Int(5000i64))?;
    v_d = py::math1("sqrt", &py::minmax(&[{ let mut out = vec![]; for t953 in py::iter(&(py::itertools("combinations", &[v_pts.clone(), Py::Int(2i64)])?))? { let mut v_a: Py; let mut v_b: Py; 
let t954 = (t953).items()?;
if t954.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t954[0].clone();
v_b = t954[1].clone();
out.push(py::add(&py::pow(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?); } Py::list(out) }], true)?)?;
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("distance"))?, &v_d.clone())?)?, &py::minmax(&[v_d.clone(), Py::Float(1e-300)], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_gc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_a = f__unit(py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("lat1"))?)?, py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("lon1"))?)?)?;
    v_b = f__unit(py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("lat2"))?)?, py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("lon2"))?)?)?;
    v_t = py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("central_angle"))?)?;
    return Ok(py::minmax(&[py::abs(&py::sub(&py::math1("cos", &v_t.clone())?, &f__vdot(v_a.clone(), v_b.clone())?)?)?, py::abs(&py::sub(&py::math1("sin", &v_t.clone())?, &f__vnorm(f__vcross(v_a.clone(), v_b.clone())?)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("distance"))?, py::mul(&py::method(&v_i.clone(), "get", &[Py::str("radius"), Py::Float(6371.0)])?, &v_t.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_haus(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_dA: Py = Py::None;
    let mut v_dB: Py = Py::None;
    let t1057 = (Py::tuple(vec![f__pairs(py::getitem(&v_i.clone(), &Py::str("a"))?, Py::str("a"), Py::Int(1i64), Py::Int(1000i64))?, f__pairs(py::getitem(&v_i.clone(), &Py::str("b"))?, Py::str("b"), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t1057.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1057[0].clone();
    v_B = t1057[1].clone();
    v_dA = py::math1("sqrt", &py::minmax(&[{ let mut out = vec![]; for t1058 in py::iter(&(v_A.clone()))? { let mut v_s: Py; 
v_s = t1058;
out.push(py::minmax(&[{ let mut out = vec![]; for t1059 in py::iter(&(v_B.clone()))? { let mut v_t: Py; 
v_t = t1059;
out.push(py::add(&py::pow(&py::sub(&py::getitem(&v_s.clone(), &Py::Int(0i64))?, &py::getitem(&v_t.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&v_s.clone(), &Py::Int(1i64))?, &py::getitem(&v_t.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?); } Py::list(out) }], false)?); } Py::list(out) }], true)?)?;
    v_dB = py::math1("sqrt", &py::minmax(&[{ let mut out = vec![]; for t1060 in py::iter(&(v_B.clone()))? { let mut v_s: Py; 
v_s = t1060;
out.push(py::minmax(&[{ let mut out = vec![]; for t1061 in py::iter(&(v_A.clone()))? { let mut v_t: Py; 
v_t = t1061;
out.push(py::add(&py::pow(&py::sub(&py::getitem(&v_s.clone(), &Py::Int(0i64))?, &py::getitem(&v_t.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&v_s.clone(), &Py::Int(1i64))?, &py::getitem(&v_t.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?); } Py::list(out) }], false)?); } Py::list(out) }], true)?)?;
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("from_a"))?, v_dA.clone()]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("from_b"))?, v_dB.clone()]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("distance"))?, py::minmax(&[v_dA.clone(), v_dB.clone()], true)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_hull(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(5000i64))?;
    v_h = { let mut out = vec![]; for t484 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("hull"))?))? { let mut v_p: Py; 
v_p = t484;
out.push(Py::tuple((v_p.clone()).items()?)); } Py::list(out) };
    v_S = py::set(&v_pts.clone())?;
    if ({ let t496 = { let t485 = py::len(&v_h.clone())?; Py::Bool({ let t486 = py::getitem(&v_o.clone(), &Py::str("n_vertices"))?; if py::ne(&t485, &t486) { true } else { false } }) }; if t496.truthy() { t496 } else { { let t495 = { let t487 = py::len(&v_h.clone())?; Py::Bool({ let t488 = Py::Int(3i64); if py::lt(&t487, &t488)? { true } else { false } }) }; if t495.truthy() { t495 } else { { let t494 = Py::Bool(({ let mut out = vec![]; for t489 in py::iter(&(v_h.clone()))? { let mut v_p: Py; 
v_p = t489;
out.push({ let t490 = v_p.clone(); Py::Bool({ let t491 = v_S.clone(); if !py::contains(&t491, &t490)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t494.truthy() { t494 } else { { let t492 = py::getitem(&v_h.clone(), &Py::Int(0i64))?; Py::Bool({ let t493 = py::minmax(&[v_h.clone()], false)?; if py::ne(&t492, &t493) { true } else { false } }) } } } } } } }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_n = py::len(&v_h.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t497 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t497;
out.push({ let t498 = f__xcross(py::getitem(&v_h.clone(), &v_k.clone())?, py::getitem(&v_h.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, py::getitem(&v_h.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(2i64))?, &v_n.clone())?)?)?; Py::Bool({ let t499 = Py::Int(0i64); if py::le(&t498, &t499)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Bool(false));
    }
    for t500 in py::iter(&(v_S.clone()))? {
        v_p = t500;
        if (Py::Bool(({ let mut out = vec![]; for t501 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t501;
out.push({ let t502 = f__xcross(py::getitem(&v_h.clone(), &v_k.clone())?, py::getitem(&v_h.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, v_p.clone())?; Py::Bool({ let t503 = Py::Int(0i64); if py::lt(&t502, &t503)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
            return Ok(Py::Bool(false));
        }
    }
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::abs(&f__shoelace(v_h.clone())?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, f__perimeter(v_h.clone(), Py::Bool(true))?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_incircle(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_sgn: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3764 = (f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(3i64))?).items()?;
    if t3764.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3764[0].clone();
    v_B = t3764[1].clone();
    v_C = t3764[2].clone();
    v_I = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("ix"))?, py::getitem(&v_o.clone(), &Py::str("iy"))?]);
    v_sgn = (if ({ let t3765 = f__shoelace(Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()]))?; Py::Bool({ let t3766 = Py::Int(0i64); if py::gt(&t3765, &t3766)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    v_worst = Py::Float(0.0);
    for t3767 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_A.clone(), v_B.clone()]), Py::tuple(vec![v_B.clone(), v_C.clone()]), Py::tuple(vec![v_C.clone(), v_A.clone()])])))? {
        let t3768 = (t3767).items()?;
        if t3768.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_P = t3768[0].clone();
        v_Q = t3768[1].clone();
        v_dist = py::truediv(&py::mul(&v_sgn.clone(), &f__cross(v_P.clone(), v_Q.clone(), v_I.clone())?)?, &py::dist(&v_P.clone(), &v_Q.clone())?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_dist.clone(), &py::getitem(&v_o.clone(), &Py::str("radius"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_line3(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_par: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_sym: Py = Py::None;
    let mut v_vec: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_P = { let mut out = vec![]; for t1224 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("point"))?))? { let mut v_v: Py; 
v_v = t1224;
out.push(py::float(&v_v.clone())?); } Py::list(out) };
    v_D = Py::list(vec![py::getitem(&v_o.clone(), &Py::str("dx"))?, py::getitem(&v_o.clone(), &Py::str("dy"))?, py::getitem(&v_o.clone(), &Py::str("dz"))?]);
    if ({ let t1225 = py::method(&v_i.clone(), "get", &[Py::str("second_point")])?; Py::Bool({ let t1226 = Py::None; if !py::is(&t1225, &t1226) { true } else { false } }) }).truthy() {
        v_want = { let mut out = vec![]; for t1227 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t1227;
out.push(py::sub(&py::float(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("second_point"))?, &v_k.clone())?)?, &py::getitem(&v_P.clone(), &v_k.clone())?)?); } Py::list(out) };
    } else {
        v_want = { let mut out = vec![]; for t1228 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("direction"))?))? { let mut v_v: Py; 
v_v = t1228;
out.push(py::float(&v_v.clone())?); } Py::list(out) };
    }
    v_sc = py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_want.clone(), &Py::Int(0i64))?)?, py::abs(&py::getitem(&v_want.clone(), &Py::Int(1i64))?)?, py::abs(&py::getitem(&v_want.clone(), &Py::Int(2i64))?)?], true)?;
    v_n = py::math1("sqrt", &py::add(&py::add(&py::mul(&py::getitem(&v_want.clone(), &Py::Int(0i64))?, &py::getitem(&v_want.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_want.clone(), &Py::Int(1i64))?, &py::getitem(&v_want.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_want.clone(), &Py::Int(2i64))?, &py::getitem(&v_want.clone(), &Py::Int(2i64))?)?)?)?;
    v_worst = Py::Float(0.0);
    for t1229 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v_k = t1229;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_D.clone(), &v_k.clone())?, &py::getitem(&v_want.clone(), &v_k.clone())?)?)?, &v_sc.clone())?, py::truediv(&py::abs(&py::sub(&py::getitem(&Py::list(vec![py::getitem(&v_o.clone(), &Py::str("x0"))?, py::getitem(&v_o.clone(), &Py::str("y0"))?, py::getitem(&v_o.clone(), &Py::str("z0"))?]), &v_k.clone())?, &py::getitem(&v_P.clone(), &v_k.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_P.clone(), &v_k.clone())?)?], true)?)?, py::abs(&py::sub(&py::getitem(&Py::list(vec![py::getitem(&v_o.clone(), &Py::str("ux"))?, py::getitem(&v_o.clone(), &Py::str("uy"))?, py::getitem(&v_o.clone(), &Py::str("uz"))?]), &v_k.clone())?, &py::truediv(&py::getitem(&v_want.clone(), &v_k.clone())?, &v_n.clone())?)?)?], true)?;
    }
    if ({ let t1230 = py::method(&v_i.clone(), "get", &[Py::str("t")])?; Py::Bool({ let t1231 = Py::None; if py::is(&t1230, &t1231) { true } else { false } }) }).truthy() {
        if ({ let t1239 = { let t1232 = py::getitem(&v_o.clone(), &Py::str("xt"))?; Py::Bool({ let t1233 = Py::None; if !py::is(&t1232, &t1233) { true } else { false } }) }; if t1239.truthy() { t1239 } else { { let t1238 = { let t1234 = py::getitem(&v_o.clone(), &Py::str("yt"))?; Py::Bool({ let t1235 = Py::None; if !py::is(&t1234, &t1235) { true } else { false } }) }; if t1238.truthy() { t1238 } else { { let t1236 = py::getitem(&v_o.clone(), &Py::str("zt"))?; Py::Bool({ let t1237 = Py::None; if !py::is(&t1236, &t1237) { true } else { false } }) } } } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
    } else {
        v_X = Py::list(vec![py::getitem(&v_o.clone(), &Py::str("xt"))?, py::getitem(&v_o.clone(), &Py::str("yt"))?, py::getitem(&v_o.clone(), &Py::str("zt"))?]);
        for t1240 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v_k = t1240;
            v_w = py::add(&py::getitem(&v_P.clone(), &v_k.clone())?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("t"))?, &py::getitem(&v_want.clone(), &v_k.clone())?)?)?;
            v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_X.clone(), &v_k.clone())?, &v_w.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_w.clone())?, py::abs(&py::getitem(&v_P.clone(), &v_k.clone())?)?], true)?)?], true)?;
        }
    }
    let t1241 = (f__l3_texts(Py::list(vec![py::getitem(&v_o.clone(), &Py::str("x0"))?, py::getitem(&v_o.clone(), &Py::str("y0"))?, py::getitem(&v_o.clone(), &Py::str("z0"))?]), v_D.clone())?).items()?;
    if t1241.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_vec = t1241[0].clone();
    v_par = t1241[1].clone();
    v_sym = t1241[2].clone();
    if ({ let t1249 = { let t1242 = py::getitem(&v_o.clone(), &Py::str("vector_form"))?; Py::Bool({ let t1243 = v_vec.clone(); if py::ne(&t1242, &t1243) { true } else { false } }) }; if t1249.truthy() { t1249 } else { { let t1248 = { let t1244 = py::getitem(&v_o.clone(), &Py::str("parametric"))?; Py::Bool({ let t1245 = v_par.clone(); if py::ne(&t1244, &t1245) { true } else { false } }) }; if t1248.truthy() { t1248 } else { { let t1246 = py::getitem(&v_o.clone(), &Py::str("symmetric"))?; Py::Bool({ let t1247 = v_sym.clone(); if py::ne(&t1246, &t1247) { true } else { false } }) } } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_lpi(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_on_line: Py = Py::None;
    let mut v_on_plane: Py = Py::None;
    let mut v_sc: Py = Py::None;
    v_X = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?, py::getitem(&v_o.clone(), &Py::str("z"))?]);
    v_n = Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?]);
    let t1314 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x0"))?, py::getitem(&v_i.clone(), &Py::str("y0"))?, py::getitem(&v_i.clone(), &Py::str("z0"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("dx"))?, py::getitem(&v_i.clone(), &Py::str("dy"))?, py::getitem(&v_i.clone(), &Py::str("dz"))?])])).items()?;
    if t1314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1314[0].clone();
    v_D = t1314[1].clone();
    v_sc = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1315 in py::iter(&(v_X.clone()))? { let mut v_v: Py; 
v_v = t1315;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?, py::minmax(&[{ let mut out = vec![]; for t1316 in py::iter(&(v_P.clone()))? { let mut v_v: Py; 
v_v = t1316;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?;
    v_on_plane = py::truediv(&py::abs(&py::add(&f__vdot(v_n.clone(), v_X.clone())?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?)?, &py::mul(&f__vnorm(v_n.clone())?, &v_sc.clone())?)?;
    v_on_line = py::truediv(&f__vnorm(f__vsub(v_X.clone(), Py::tuple(({ let mut out = vec![]; for t1317 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t1317;
out.push(py::add(&py::getitem(&v_P.clone(), &v_k.clone())?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("t"))?, &py::getitem(&v_D.clone(), &v_k.clone())?)?)?); } Py::list(out) }).items()?))?)?, &v_sc.clone())?;
    return Ok(py::minmax(&[v_on_plane.clone(), v_on_line.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mar(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_us: Py = Py::None;
    let mut v_vs: Py = Py::None;
    v_h = f__hull(f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(5000i64))?)?;
    v_n = py::len(&v_h.clone())?;
    v_best = Py::Float(f64::INFINITY);
    for t1370 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1370;
        let t1371 = (Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_h.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_h.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(1i64))?)?])).items()?;
        if t1371.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ex = t1371[0].clone();
        v_ey = t1371[1].clone();
        v_L = py::hypot(&[v_ex.clone(), v_ey.clone()])?;
        v_us = { let mut out = vec![]; for t1372 in py::iter(&(v_h.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1373 = (t1372).items()?;
if t1373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1373[0].clone();
v_y = t1373[1].clone();
out.push(py::truediv(&py::add(&py::mul(&py::sub(&v_x.clone(), &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &v_ex.clone())?, &py::mul(&py::sub(&v_y.clone(), &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(1i64))?)?, &v_ey.clone())?)?, &v_L.clone())?); } Py::list(out) };
        v_vs = { let mut out = vec![]; for t1374 in py::iter(&(v_h.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1375 = (t1374).items()?;
if t1375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1375[0].clone();
v_y = t1375[1].clone();
out.push(py::truediv(&py::sub(&py::mul(&py::sub(&v_y.clone(), &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(1i64))?)?, &v_ex.clone())?, &py::mul(&py::sub(&v_x.clone(), &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &v_ey.clone())?)?, &v_L.clone())?); } Py::list(out) };
        v_best = py::minmax(&[v_best.clone(), py::mul(&py::sub(&py::minmax(&[v_us.clone()], true)?, &py::minmax(&[v_us.clone()], false)?)?, &py::sub(&py::minmax(&[v_vs.clone()], true)?, &py::minmax(&[v_vs.clone()], false)?)?)?], false)?;
    }
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, v_best.clone()]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("width"))?, &py::getitem(&v_o.clone(), &Py::str("length"))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mec(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_gaps: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(1i64), Py::Int(2000i64))?;
    let t1429 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("cx"))?, py::getitem(&v_o.clone(), &Py::str("cy"))?]), py::getitem(&v_o.clone(), &Py::str("radius"))?])).items()?;
    if t1429.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C = t1429[0].clone();
    v_r = t1429[1].clone();
    if ({ let t1430 = v_r.clone(); Py::Bool({ let t1431 = Py::Int(0i64); if py::eq(&t1430, &t1431) { true } else { false } }) }).truthy() {
        return Ok({ let t1432 = py::len(&py::set(&v_pts.clone())?)?; Py::Bool({ let t1433 = Py::Int(1i64); if py::eq(&t1432, &t1433) { true } else { false } }) });
    }
    if (Py::Bool(({ let mut out = vec![]; for t1434 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1434;
out.push({ let t1435 = py::dist(&v_C.clone(), &v_p.clone())?; Py::Bool({ let t1436 = py::mul(&v_r.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::gt(&t1435, &t1436)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Bool(false));
    }
    v_on = { let mut out = vec![]; for t1437 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1437;
if ({ let t1438 = py::abs(&py::sub(&py::dist(&v_C.clone(), &v_p.clone())?, &v_r.clone())?)?; Py::Bool({ let t1439 = py::mul(&Py::Float(1e-09), &v_r.clone())?; if py::le(&t1438, &t1439)? { true } else { false } }) }).truthy() { out.push(py::atan2(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_C.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_C.clone(), &Py::Int(0i64))?)?)?); }} Py::list(out) };
    if ({ let t1440 = py::len(&v_on.clone())?; Py::Bool({ let t1441 = Py::Int(2i64); if py::lt(&t1440, &t1441)? { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    let _ = py::method_sort(&v_on.clone(), None, false)?;
    v_gaps = py::add(&{ let mut out = vec![]; for t1442 in py::range_iter(&[(py::sub(&py::len(&v_on.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1442;
out.push(py::sub(&py::getitem(&v_on.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_on.clone(), &v_k.clone())?)?); } Py::list(out) }, &Py::list(vec![py::add(&py::sub(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_on.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::getitem(&v_on.clone(), &Py::Int(0i64))?)?]))?;
    return Ok({ let t1443 = py::minmax(&[v_gaps.clone()], true)?; Py::Bool({ let t1444 = py::mul(&Py::Float(std::f64::consts::PI), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::le(&t1443, &t1444)? { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_nn(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_P = Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("px"))?, py::getitem(&v_i.clone(), &Py::str("py"))?]);
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(1i64), Py::Int(100000i64))?;
    v_k = py::sub(&py::getitem(&v_o.clone(), &Py::str("index"))?, &Py::Int(1i64))?;
    if ({ let t1476 = py::not(&{ let t1471 = Py::Int(0i64); Py::Bool({ let t1472 = v_k.clone(); if py::le(&t1471, &t1472)? { { let t1473 = py::len(&v_pts.clone())?; if py::lt(&t1472, &t1473)? { true } else { false } } } else { false } }) }); if t1476.truthy() { t1476 } else { { let t1474 = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("nx"))?, py::getitem(&v_o.clone(), &Py::str("ny"))?]); Py::Bool({ let t1475 = py::getitem(&v_pts.clone(), &v_k.clone())?; if py::ne(&t1474, &t1475) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_d2 = { let mut out = vec![]; for t1477 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1477;
out.push(py::add(&py::pow(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_P.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?); } Py::list(out) };
    if ({ let t1482 = { let t1478 = py::getitem(&v_d2.clone(), &v_k.clone())?; Py::Bool({ let t1479 = py::minmax(&[v_d2.clone()], false)?; if py::ne(&t1478, &t1479) { true } else { false } }) }; if t1482.truthy() { t1482 } else { { let t1480 = py::method(&v_d2.clone(), "index", &[py::minmax(&[v_d2.clone()], false)?])?; Py::Bool({ let t1481 = v_k.clone(); if py::ne(&t1480, &t1481) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("distance"))?, py::math1("sqrt", &py::getitem(&v_d2.clone(), &v_k.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ortho(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_R_: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3807 = (f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(3i64))?).items()?;
    if t3807.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3807[0].clone();
    v_B = t3807[1].clone();
    v_C = t3807[2].clone();
    v_H = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("hx"))?, py::getitem(&v_o.clone(), &Py::str("hy"))?]);
    v_L = py::minmax(&[py::dist(&v_A.clone(), &v_B.clone())?, py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?], true)?;
    v_worst = Py::Float(0.0);
    for t3808 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_A.clone(), v_B.clone(), v_C.clone()]), Py::tuple(vec![v_B.clone(), v_C.clone(), v_A.clone()]), Py::tuple(vec![v_C.clone(), v_A.clone(), v_B.clone()])])))? {
        let t3809 = (t3808).items()?;
        if t3809.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_P = t3809[0].clone();
        v_Q = t3809[1].clone();
        v_R_ = t3809[2].clone();
        v_d = py::add(&py::mul(&py::sub(&py::getitem(&v_H.clone(), &Py::Int(0i64))?, &py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_R_.clone(), &Py::Int(0i64))?, &py::getitem(&v_Q.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_H.clone(), &Py::Int(1i64))?, &py::getitem(&v_P.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_R_.clone(), &Py::Int(1i64))?, &py::getitem(&v_Q.clone(), &Py::Int(1i64))?)?)?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&v_d.clone())?, &py::mul(&v_L.clone(), &py::minmax(&[v_L.clone(), py::dist(&v_H.clone(), &v_P.clone())?], true)?)?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pangles(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_sgn: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::None)?;
    v_n = py::len(&v_pts.clone())?;
    if ({ let t2284 = { let t2277 = py::len(&py::getitem(&v_o.clone(), &Py::str("angles"))?)?; Py::Bool({ let t2278 = v_n.clone(); if py::ne(&t2277, &t2278) { true } else { false } }) }; if t2284.truthy() { t2284 } else { { let t2279 = py::getitem(&v_o.clone(), &Py::str("reflex_vertices"))?; Py::Bool({ let t2280 = py::sum(&{ let mut out = vec![]; for t2281 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("angles"))?))? { let mut v_x: Py; 
v_x = t2281;
if ({ let t2282 = v_x.clone(); Py::Bool({ let t2283 = Py::Int(180i64); if py::gt(&t2282, &t2283)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::ne(&t2279, &t2280) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = py::add(&f__rel(py::sum(&py::getitem(&v_o.clone(), &Py::str("angles"))?, None)?, py::mul(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &Py::Float(180.0))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("angle_sum"))?, py::mul(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &Py::Float(180.0))?)?)?;
    v_sgn = (if ({ let t2295 = f__shoelace(v_pts.clone())?; Py::Bool({ let t2296 = Py::Int(0i64); if py::gt(&t2295, &t2296)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    for t2297 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2297;
        let t2298 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2298.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2298[0].clone();
        v_b = t2298[1].clone();
        v_c = t2298[2].clone();
        let t2299 = (Py::tuple(vec![Py::tuple(vec![py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?]), Py::tuple(vec![py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?])])).items()?;
        if t2299.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2299[0].clone();
        v_v = t2299[1].clone();
        v_x = py::math1("degrees", &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::add(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::hypot(&({ let mut v: Vec<Py> = vec![]; v.extend((v_u.clone()).items()?); v })[..])?, &py::hypot(&({ let mut v: Vec<Py> = vec![]; v.extend((v_v.clone()).items()?); v })[..])?)?)?], false)?], true)?)?)?;
        if ({ let t2300 = py::mul(&v_sgn.clone(), &f__cross(v_a.clone(), v_b.clone(), v_c.clone())?)?; Py::Bool({ let t2301 = Py::Int(0i64); if py::lt(&t2300, &t2301)? { true } else { false } }) }).truthy() {
            v_x = py::sub(&Py::Int(360i64), &v_x.clone())?;
        }
        v_worst = py::minmax(&[v_worst.clone(), (if ({ let t2302 = py::abs(&py::sub(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("angles"))?, &v_k.clone())?, &v_x.clone())?)?; Py::Bool({ let t2303 = Py::Float(1e-07); if py::gt(&t2302, &t2303)? { true } else { false } }) }).truthy() { py::truediv(&py::abs(&py::sub(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("angles"))?, &v_k.clone())?, &v_x.clone())?)?, &Py::Int(360i64))? } else { Py::Float(0.0) })], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_parab(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_x1: Py = Py::None;
    let t1516 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("x0"))?, py::getitem(&v_i.clone(), &Py::str("x1"))?])).items()?;
    if t1516.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1516[0].clone();
    v_b = t1516[1].clone();
    v_x0 = t1516[2].clone();
    v_x1 = t1516[3].clone();
    v_nodes = Py::list(vec![Py::tuple(vec![py::neg(&Py::Float(0.9739065285171717))?, Py::Float(0.0666713443086881)]), Py::tuple(vec![py::neg(&Py::Float(0.8650633666889845))?, Py::Float(0.1494513491505806)]), Py::tuple(vec![py::neg(&Py::Float(0.6794095682990244))?, Py::Float(0.219086362515982)]), Py::tuple(vec![py::neg(&Py::Float(0.4333953941292472))?, Py::Float(0.2692667193099963)]), Py::tuple(vec![py::neg(&Py::Float(0.1488743389816312))?, Py::Float(0.2955242247147529)])]);
    v_nodes = py::iadd(&v_nodes, &{ let mut out = vec![]; for t1517 in py::iter(&(v_nodes.clone()))? { let mut v_w: Py; let mut v_x: Py; 
let t1518 = (t1517).items()?;
if t1518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1518[0].clone();
v_w = t1518[1].clone();
out.push(Py::tuple(vec![py::neg(&v_x.clone())?, v_w.clone()])); } Py::list(out) })?;
    let t1519 = (Py::tuple(vec![py::minmax(&[v_x0.clone(), v_x1.clone()], false)?, py::minmax(&[v_x0.clone(), v_x1.clone()], true)?])).items()?;
    if t1519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1519[0].clone();
    v_hi = t1519[1].clone();
    v_n = Py::Int(64i64);
    v_h = py::truediv(&py::sub(&v_hi.clone(), &v_lo.clone())?, &v_n.clone())?;
    v_s = Py::Float(0.0);
    for t1520 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1520;
        v_m = py::add(&v_lo.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Float(0.5))?, &v_h.clone())?)?;
        v_s = py::iadd(&v_s, &py::truediv(&py::mul(&py::sum(&{ let mut out = vec![]; for t1521 in py::iter(&(v_nodes.clone()))? { let mut v_w: Py; let mut v_x: Py; 
let t1522 = (t1521).items()?;
if t1522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1522[0].clone();
v_w = t1522[1].clone();
out.push(py::mul(&v_w.clone(), &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::pow(&py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &py::add(&v_m.clone(), &py::truediv(&py::mul(&v_x.clone(), &v_h.clone())?, &Py::Int(2i64))?)?)?, &v_b.clone())?, &Py::Int(2i64))?)?)?)?); } Py::list(out) }, None)?, &v_h.clone())?, &Py::Int(2i64))?)?;
    }
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("length"))?, v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pick(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_area: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_steps: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_x1: Py = Py::None;
    let mut v_y0: Py = Py::None;
    let mut v_y1: Py = Py::None;
    v_pts = { let mut out = vec![]; for t1638 in py::iter(&(f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::None)?))? { let mut v_x: Py; let mut v_y: Py; 
let t1639 = (t1638).items()?;
if t1639.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1639[0].clone();
v_y = t1639[1].clone();
out.push(Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])); } Py::list(out) };
    v_n = py::len(&v_pts.clone())?;
    v_area = py::truediv(&py::abs(&py::sum(&{ let mut out = vec![]; for t1640 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1640;
out.push(py::mul(&py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &py::add(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?)?, &Py::Int(2i64))?;
    v_b = Py::Int(0i64);
    for t1641 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1641;
        let t1642 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t1642.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t1643 = (t1642[0].clone()).items()?;
        if t1643.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x0 = t1643[0].clone();
        v_y0 = t1643[1].clone();
        let t1644 = (t1642[1].clone()).items()?;
        if t1644.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x1 = t1644[0].clone();
        v_y1 = t1644[1].clone();
        let t1645 = (Py::tuple(vec![py::sub(&v_x1.clone(), &v_x0.clone())?, py::sub(&v_y1.clone(), &v_y0.clone())?])).items()?;
        if t1645.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_dx = t1645[0].clone();
        v_dy = t1645[1].clone();
        v_steps = py::minmax(&[py::abs(&v_dx.clone())?, py::abs(&v_dy.clone())?], true)?;
        v_b = py::iadd(&v_b, &py::sum(&{ let mut out = vec![]; for t1646 in py::range_iter(&[(v_steps.clone()).i()?]) { let mut v_s: Py; 
v_s = t1646;
if ({ let t1651 = { let t1647 = py::modulo(&py::mul(&v_s.clone(), &v_dx.clone())?, &v_steps.clone())?; Py::Bool({ let t1648 = Py::Int(0i64); if py::eq(&t1647, &t1648) { true } else { false } }) }; if !t1651.truthy() { t1651 } else { { let t1649 = py::modulo(&py::mul(&v_s.clone(), &v_dy.clone())?, &v_steps.clone())?; Py::Bool({ let t1650 = Py::Int(0i64); if py::eq(&t1649, &t1650) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?;
    }
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, v_area.clone()]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("boundary_points"))?, v_b.clone()]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("interior_points"))?, py::add(&py::sub(&v_area.clone(), &py::truediv(&v_b.clone(), &Py::Int(2i64))?)?, &Py::Int(1i64))?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pip(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_ax: Py = Py::None;
    let mut v_ay: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_by: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_wn: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::None)?;
    let t1854 = (Py::tuple(vec![f__fr(py::getitem(&v_i.clone(), &Py::str("px"))?)?, f__fr(py::getitem(&v_i.clone(), &Py::str("py"))?)?])).items()?;
    if t1854.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t1854[0].clone();
    v_Y = t1854[1].clone();
    v_n = py::len(&v_pts.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t1855 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1855;
out.push(f__on_segment_exact(Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("px"))?, py::getitem(&v_i.clone(), &Py::str("py"))?]), py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?)?); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok({ let t1856 = py::getitem(&v_o.clone(), &Py::str("location"))?; Py::Bool({ let t1857 = Py::str("boundary"); if py::eq(&t1856, &t1857) { true } else { false } }) });
    }
    v_wn = Py::Int(0i64);
    for t1858 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1858;
        let t1860 = ({ let mut out = vec![]; for t1859 in py::iter(&(Py::tuple(vec![py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])))? { let mut v_c: Py; 
v_c = t1859;
out.push(Py::tuple(vec![py::sub(&f__fr(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, &v_X.clone())?, py::sub(&f__fr(py::getitem(&v_c.clone(), &Py::Int(1i64))?)?, &v_Y.clone())?])); } Py::list(out) }).items()?;
        if t1860.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t1861 = (t1860[0].clone()).items()?;
        if t1861.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ax = t1861[0].clone();
        v_ay = t1861[1].clone();
        let t1862 = (t1860[1].clone()).items()?;
        if t1862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_bx = t1862[0].clone();
        v_by = t1862[1].clone();
        if ({ let t1868 = { let t1863 = v_ay.clone(); Py::Bool({ let t1864 = Py::Int(0i64); if py::le(&t1863, &t1864)? { { let t1865 = v_by.clone(); if py::lt(&t1864, &t1865)? { true } else { false } } } else { false } }) }; if !t1868.truthy() { t1868 } else { { let t1866 = py::sub(&py::mul(&v_ax.clone(), &v_by.clone())?, &py::mul(&v_bx.clone(), &v_ay.clone())?)?; Py::Bool({ let t1867 = Py::Int(0i64); if py::gt(&t1866, &t1867)? { true } else { false } }) } } }).truthy() {
            v_wn = py::iadd(&v_wn, &Py::Int(1i64))?;
        } else {
            if ({ let t1874 = { let t1869 = v_by.clone(); Py::Bool({ let t1870 = Py::Int(0i64); if py::le(&t1869, &t1870)? { { let t1871 = v_ay.clone(); if py::lt(&t1870, &t1871)? { true } else { false } } } else { false } }) }; if !t1874.truthy() { t1874 } else { { let t1872 = py::sub(&py::mul(&v_ax.clone(), &v_by.clone())?, &py::mul(&v_bx.clone(), &v_ay.clone())?)?; Py::Bool({ let t1873 = Py::Int(0i64); if py::lt(&t1872, &t1873)? { true } else { false } }) } } }).truthy() {
                v_wn = py::sub(&v_wn, &Py::Int(1i64))?;
            }
        }
    }
    return Ok({ let t1875 = py::getitem(&v_o.clone(), &Py::str("location"))?; Py::Bool({ let t1876 = (if (v_wn.clone()).truthy() { Py::str("inside") } else { Py::str("outside") }); if py::eq(&t1875, &t1876) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_plane3(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sc: Py = Py::None;
    v_P = f__triples(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64))?;
    v_n = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("a"))?, py::getitem(&v_o.clone(), &Py::str("b"))?, py::getitem(&v_o.clone(), &Py::str("c"))?]);
    v_first = (match ('g1696: { let mut out = vec![]; for t1697 in py::iter(&(v_n.clone()))? { let mut v_c: Py; 
v_c = t1697;
if ({ let t1698 = py::abs(&v_c.clone())?; Py::Bool({ let t1699 = Py::Float(1e-12); if py::gt(&t1698, &t1699)? { true } else { false } }) }).truthy() { out.push(v_c.clone()); break 'g1696 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::Float(0.0) });
    v_sc = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1700 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t1700;
for t1701 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t1701;
out.push(py::abs(&v_c.clone())?); }} Py::list(out) }], true)?], true)?;
    return Ok(py::minmax(&[py::add(&py::add(&Py::list(vec![py::abs(&py::sub(&f__vnorm(v_n.clone())?, &Py::Int(1i64))?)?]), &{ let mut out = vec![]; for t1702 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t1702;
out.push(py::truediv(&py::abs(&py::add(&f__vdot(v_n.clone(), v_p.clone())?, &py::getitem(&v_o.clone(), &Py::str("d"))?)?)?, &v_sc.clone())?); } Py::list(out) })?, &Py::list(vec![(if ({ let t1703 = v_first.clone(); Py::Bool({ let t1704 = Py::Int(0i64); if py::gt(&t1703, &t1704)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_platonic(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_kR: Py = Py::None;
    let mut v_kr: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_kv: Py = Py::None;
    let t1727 = (py::getitem(&c__PLATONIC()?, &py::getitem(&v_i.clone(), &Py::str("solid"))?)?).items()?;
    if t1727.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1727[0].clone();
    v_E = t1727[1].clone();
    v_F = t1727[2].clone();
    v_kv = t1727[3].clone();
    v_ks = t1727[4].clone();
    v_kR = t1727[5].clone();
    v_kr = t1727[6].clone();
    v_a = py::getitem(&v_i.clone(), &Py::str("edge"))?;
    if ({ let t1728 = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("vertices"))?, py::getitem(&v_o.clone(), &Py::str("edges"))?, py::getitem(&v_o.clone(), &Py::str("faces"))?, py::getitem(&v_o.clone(), &Py::str("euler_characteristic"))?]); Py::Bool({ let t1729 = Py::tuple(vec![v_V.clone(), v_E.clone(), v_F.clone(), Py::Int(2i64)]); if py::ne(&t1728, &t1729) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&Py::Int(3i64), &py::getitem(&v_o.clone(), &Py::str("volume"))?)?, py::mul(&py::getitem(&v_o.clone(), &Py::str("inradius"))?, &py::getitem(&v_o.clone(), &Py::str("surface_area"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("circumradius"))?, py::mul(&v_kR.clone(), &v_a.clone())?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("surface_area"))?, py::mul(&py::mul(&v_ks.clone(), &v_a.clone())?, &v_a.clone())?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_polygon_area(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_trap: Py = Py::None;
    v_pts = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::None)?;
    v_n = py::len(&v_pts.clone())?;
    v_trap = py::truediv(&py::neg(&py::fsum(&{ let mut out = vec![]; for t2043 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2043;
out.push(py::mul(&py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &py::add(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?)?)?); } Py::list(out) })?)?, &Py::Int(2i64))?;
    v_scale = py::truediv(&py::pow(&py::fsum(&{ let mut out = vec![]; for t2044 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2044;
out.push(py::add(&py::abs(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &py::abs(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) })?, &Py::Int(2i64))?, &v_n.clone())?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("area"))?, &py::abs(&v_trap.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_trap.clone())?, py::mul(&v_scale.clone(), &Py::Float(1e-06))?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::fsum(&{ let mut out = vec![]; for t2055 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2055;
out.push(py::hypot(&[py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?)?])?); } Py::list(out) })?)?, (if ({ let t2056 = py::getitem(&v_o.clone(), &Py::str("orientation"))?; Py::Bool({ let t2057 = (if ({ let t2058 = v_trap.clone(); Py::Bool({ let t2059 = Py::Int(0i64); if py::gt(&t2058, &t2059)? { true } else { false } }) }).truthy() { Py::str("counterclockwise") } else { Py::str("clockwise") }); if py::eq(&t2056, &t2057) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ppd(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Fp: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_par: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t1906 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("px"))?, py::getitem(&v_i.clone(), &Py::str("py"))?, py::getitem(&v_i.clone(), &Py::str("pz"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])])).items()?;
    if t1906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1906[0].clone();
    v_n = t1906[1].clone();
    v_Fp = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("fx"))?, py::getitem(&v_o.clone(), &Py::str("fy"))?, py::getitem(&v_o.clone(), &Py::str("fz"))?]);
    v_sc = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1907 in py::iter(&(v_P.clone()))? { let mut v_v: Py; 
v_v = t1907;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?, py::truediv(&py::abs(&py::getitem(&v_i.clone(), &Py::str("d"))?)?, &f__vnorm(v_n.clone())?)?], true)?;
    v_on = py::truediv(&py::abs(&py::add(&f__vdot(v_n.clone(), v_Fp.clone())?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?)?, &py::mul(&f__vnorm(v_n.clone())?, &v_sc.clone())?)?;
    v_par = py::truediv(&f__vnorm(f__vcross(f__vsub(v_P.clone(), v_Fp.clone())?, v_n.clone())?)?, &py::mul(&f__vnorm(v_n.clone())?, &v_sc.clone())?)?;
    return Ok(py::minmax(&[v_on.clone(), v_par.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("distance"))?, &py::dist(&v_P.clone(), &v_Fp.clone())?)?)?, &v_sc.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_psd(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_d_end: Py = Py::None;
    let mut v_far: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1934 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("px"))?, py::getitem(&v_i.clone(), &Py::str("py"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x1"))?, py::getitem(&v_i.clone(), &Py::str("y1"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x2"))?, py::getitem(&v_i.clone(), &Py::str("y2"))?])])).items()?;
    if t1934.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1934[0].clone();
    v_A = t1934[1].clone();
    v_B = t1934[2].clone();
    v_Q = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("qx"))?, py::getitem(&v_o.clone(), &Py::str("qy"))?]);
    v_L = py::dist(&v_A.clone(), &v_B.clone())?;
    v_sc = py::minmax(&[Py::Float(1.0), v_L.clone(), py::dist(&v_P.clone(), &v_A.clone())?], true)?;
    v_on = py::add(&py::truediv(&py::abs(&f__cross(v_A.clone(), v_B.clone(), v_Q.clone())?)?, &py::mul(&v_L.clone(), &v_sc.clone())?)?, &py::truediv(&py::minmax(&[Py::Float(0.0), py::sub(&py::add(&py::dist(&v_A.clone(), &v_Q.clone())?, &py::dist(&v_Q.clone(), &v_B.clone())?)?, &v_L.clone())?], true)?, &v_sc.clone())?)?;
    v_worst = py::minmax(&[v_on.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("distance"))?, &py::dist(&v_P.clone(), &v_Q.clone())?)?)?, &v_sc.clone())?], true)?;
    v_d_end = py::minmax(&[py::dist(&v_P.clone(), &v_A.clone())?, py::dist(&v_P.clone(), &v_B.clone())?], false)?;
    if ({ let t1935 = py::getitem(&v_o.clone(), &Py::str("distance"))?; Py::Bool({ let t1936 = py::add(&py::mul(&v_d_end.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-10))?)?, &Py::Float(1e-300))?; if py::gt(&t1935, &t1936)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t1937 = v_Q.clone(); Py::Bool({ let t1938 = Py::tuple(vec![v_A.clone(), v_B.clone()]); if !py::contains(&t1938, &t1937)? { true } else { false } }) }).truthy() {
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::add(&py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::getitem(&v_Q.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &py::getitem(&v_Q.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?)?)?)?, &py::mul(&v_L.clone(), &v_sc.clone())?)?], true)?;
    } else {
        v_far = (if ({ let t1939 = v_Q.clone(); Py::Bool({ let t1940 = v_A.clone(); if py::eq(&t1939, &t1940) { true } else { false } }) }).truthy() { v_B.clone() } else { v_A.clone() });
        if ({ let t1941 = py::add(&py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::getitem(&v_Q.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_far.clone(), &Py::Int(0i64))?, &py::getitem(&v_Q.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &py::getitem(&v_Q.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_far.clone(), &Py::Int(1i64))?, &py::getitem(&v_Q.clone(), &Py::Int(1i64))?)?)?)?; Py::Bool({ let t1942 = py::mul(&py::mul(&Py::Float(1e-10), &v_L.clone())?, &v_sc.clone())?; if py::gt(&t1941, &t1942)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(1.0));
        }
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_reflect(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t2557 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t2557.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2557[0].clone();
    v_b = t2557[1].clone();
    v_c = t2557[2].clone();
    let t2558 = (Py::tuple(vec![py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("px"))?, &py::getitem(&v_o.clone(), &Py::str("x"))?)?, &Py::Int(2i64))?, py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("py"))?, &py::getitem(&v_o.clone(), &Py::str("y"))?)?, &Py::Int(2i64))?])).items()?;
    if t2558.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2558[0].clone();
    v_my = t2558[1].clone();
    let t2559 = (Py::tuple(vec![py::sub(&py::getitem(&v_o.clone(), &Py::str("x"))?, &py::getitem(&v_i.clone(), &Py::str("px"))?)?, py::sub(&py::getitem(&v_o.clone(), &Py::str("y"))?, &py::getitem(&v_i.clone(), &Py::str("py"))?)?])).items()?;
    if t2559.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dx = t2559[0].clone();
    v_dy = t2559[1].clone();
    v_n = py::hypot(&[v_a.clone(), v_b.clone()])?;
    v_sc = py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("px"))?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("py"))?)?, py::abs(&py::getitem(&v_o.clone(), &Py::str("x"))?)?, py::abs(&py::getitem(&v_o.clone(), &Py::str("y"))?)?], true)?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::add(&py::add(&py::mul(&v_a.clone(), &v_mx.clone())?, &py::mul(&v_b.clone(), &v_my.clone())?)?, &v_c.clone())?)?, &py::mul(&v_n.clone(), &v_sc.clone())?)?, py::truediv(&py::abs(&py::sub(&py::mul(&v_dx.clone(), &v_b.clone())?, &py::mul(&v_dy.clone(), &v_a.clone())?)?)?, &py::mul(&v_n.clone(), &v_sc.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_rot3(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nk: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_xp: Py = Py::None;
    v_P = Py::tuple(({ let mut out = vec![]; for t2760 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("point"))?))? { let mut v_v: Py; 
v_v = t2760;
out.push(py::float(&v_v.clone())?); } Py::list(out) }).items()?);
    v_k = Py::tuple(({ let mut out = vec![]; for t2761 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("axis"))?))? { let mut v_v: Py; 
v_v = t2761;
out.push(py::float(&v_v.clone())?); } Py::list(out) }).items()?);
    v_X = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?, py::getitem(&v_o.clone(), &Py::str("z"))?]);
    v_nk = f__vnorm(v_k.clone())?;
    v_u = Py::tuple(({ let mut out = vec![]; for t2762 in py::iter(&(v_k.clone()))? { let mut v_c: Py; 
v_c = t2762;
out.push(py::truediv(&v_c.clone(), &v_nk.clone())?); } Py::list(out) }).items()?);
    v_sc = py::minmax(&[Py::Float(1.0), f__vnorm(v_P.clone())?], true)?;
    v_pp = Py::tuple(({ let mut out = vec![]; for t2763 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2763;
out.push(py::sub(&py::getitem(&v_P.clone(), &v_j.clone())?, &py::mul(&py::getitem(&v_u.clone(), &v_j.clone())?, &f__vdot(v_u.clone(), v_P.clone())?)?)?); } Py::list(out) }).items()?);
    v_xp = Py::tuple(({ let mut out = vec![]; for t2764 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2764;
out.push(py::sub(&py::getitem(&v_X.clone(), &v_j.clone())?, &py::mul(&py::getitem(&v_u.clone(), &v_j.clone())?, &f__vdot(v_u.clone(), v_X.clone())?)?)?); } Py::list(out) }).items()?);
    v_t = py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("angle"))?)?;
    v_want = Py::tuple(({ let mut out = vec![]; for t2765 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2765;
out.push(py::add(&py::mul(&py::getitem(&v_pp.clone(), &v_j.clone())?, &py::math1("cos", &v_t.clone())?)?, &py::mul(&py::getitem(&f__vcross(v_u.clone(), v_pp.clone())?, &v_j.clone())?, &py::math1("sin", &v_t.clone())?)?)?); } Py::list(out) }).items()?);
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&f__vdot(v_u.clone(), v_X.clone())?, &f__vdot(v_u.clone(), v_P.clone())?)?)?, &v_sc.clone())?, py::truediv(&f__vnorm(f__vsub(v_xp.clone(), v_want.clone())?)?, &v_sc.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_rotate(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t2731 = (Py::tuple(vec![py::method(&v_i.clone(), "get", &[Py::str("cx"), Py::Float(0.0)])?, py::method(&v_i.clone(), "get", &[Py::str("cy"), Py::Float(0.0)])?])).items()?;
    if t2731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t2731[0].clone();
    v_cy = t2731[1].clone();
    v_z = py::complex(&[py::sub(&py::getitem(&v_i.clone(), &Py::str("px"))?, &v_cx.clone())?, py::sub(&py::getitem(&v_i.clone(), &Py::str("py"))?, &v_cy.clone())?])?;
    v_w = py::complex(&[py::sub(&py::getitem(&v_o.clone(), &Py::str("x"))?, &v_cx.clone())?, py::sub(&py::getitem(&v_o.clone(), &Py::str("y"))?, &v_cy.clone())?])?;
    v_t = py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("angle"))?)?;
    return Ok(py::truediv(&py::abs(&py::sub(&v_w.clone(), &py::mul(&v_z.clone(), &py::complex(&[py::math1("cos", &v_t.clone())?, py::math1("sin", &v_t.clone())?])?)?)?)?, &py::minmax(&[py::abs(&v_z.clone())?, Py::Float(1.0), py::abs(&v_cx.clone())?, py::abs(&v_cy.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_seg(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_collinear: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_on: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let mut v_rx: Py = Py::None;
    let mut v_ry: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t2885 = (f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(4i64), Py::Int(4i64))?).items()?;
    if t2885.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2885[0].clone();
    v_B = t2885[1].clone();
    v_C = t2885[2].clone();
    v_D = t2885[3].clone();
    let t2886 = (Py::tuple(vec![py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_D.clone(), &Py::Int(0i64))?, &py::getitem(&v_C.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_D.clone(), &Py::Int(1i64))?, &py::getitem(&v_C.clone(), &Py::Int(1i64))?)?])).items()?;
    if t2886.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rx = t2886[0].clone();
    v_ry = t2886[1].clone();
    v_sx = t2886[2].clone();
    v_sy = t2886[3].clone();
    v_den = py::sub(&py::mul(&v_rx.clone(), &v_sy.clone())?, &py::mul(&v_ry.clone(), &v_sx.clone())?)?;
    v_sc = py::minmax(&[py::hypot(&[v_rx.clone(), v_ry.clone()])?, py::hypot(&[v_sx.clone(), v_sy.clone()])?], true)?;
    if ({ let t2887 = py::getitem(&v_o.clone(), &Py::str("status"))?; Py::Bool({ let t2888 = Py::str("point"); if py::eq(&t2887, &t2888) { true } else { false } }) }).truthy() {
        v_P = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?]);
        v_on = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_D = v_D.clone(); let v_P = v_P.clone(); let v_den = v_den.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_rx = v_rx.clone(); let v_ry = v_ry.clone(); let v_sc = v_sc.clone(); let v_sx = v_sx.clone(); let v_sy = v_sy.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_U: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_V: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::truediv(&py::abs(&f__cross(v_U.clone(), v_V.clone(), v_P.clone())?)?, &py::mul(&py::dist(&v_U.clone(), &v_V.clone())?, &py::minmax(&[Py::Float(1.0), v_sc.clone()], true)?)?)?, &py::truediv(&py::minmax(&[Py::Float(0.0), py::sub(&py::add(&py::dist(&v_U.clone(), &v_P.clone())?, &py::dist(&v_P.clone(), &v_V.clone())?)?, &py::dist(&v_U.clone(), &v_V.clone())?)?], true)?, &py::minmax(&[Py::Float(1.0), v_sc.clone()], true)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        return Ok(py::minmax(&[py::call(&v_on.clone(), &[v_A.clone(), v_B.clone()])?, py::call(&v_on.clone(), &[v_C.clone(), v_D.clone()])?], true)?);
    }
    v_collinear = { let t2893 = { let t2889 = v_den.clone(); Py::Bool({ let t2890 = Py::Int(0i64); if py::eq(&t2889, &t2890) { true } else { false } }) }; if !t2893.truthy() { t2893 } else { { let t2891 = py::abs(&f__cross(v_A.clone(), v_B.clone(), v_C.clone())?)?; Py::Bool({ let t2892 = py::mul(&py::mul(&Py::Float(1e-12), &v_sc.clone())?, &v_sc.clone())?; if py::le(&t2891, &t2892)? { true } else { false } }) } } };
    if ({ let t2894 = py::getitem(&v_o.clone(), &Py::str("status"))?; Py::Bool({ let t2895 = Py::str("overlap"); if py::eq(&t2894, &t2895) { true } else { false } }) }).truthy() {
        return Ok(v_collinear.clone());
    }
    if ({ let t2896 = v_den.clone(); Py::Bool({ let t2897 = Py::Int(0i64); if py::eq(&t2896, &t2897) { true } else { false } }) }).truthy() {
        if (py::not(&v_collinear.clone())).truthy() {
            return Ok(Py::Bool(true));
        }
        v_rr = py::add(&py::mul(&v_rx.clone(), &v_rx.clone())?, &py::mul(&v_ry.clone(), &v_ry.clone())?)?;
        v_t0 = py::truediv(&py::add(&py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_rx.clone())?, &py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_ry.clone())?)?, &v_rr.clone())?;
        v_t1 = py::truediv(&py::add(&py::mul(&py::sub(&py::getitem(&v_D.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_rx.clone())?, &py::mul(&py::sub(&py::getitem(&v_D.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_ry.clone())?)?, &v_rr.clone())?;
        return Ok({ let t2902 = { let t2898 = py::minmax(&[v_t0.clone(), v_t1.clone()], true)?; Py::Bool({ let t2899 = Py::Int(0i64); if py::lt(&t2898, &t2899)? { true } else { false } }) }; if t2902.truthy() { t2902 } else { { let t2900 = py::minmax(&[v_t0.clone(), v_t1.clone()], false)?; Py::Bool({ let t2901 = Py::Int(1i64); if py::gt(&t2900, &t2901)? { true } else { false } }) } } });
    }
    v_t = py::truediv(&py::sub(&py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_sy.clone())?, &py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_sx.clone())?)?, &v_den.clone())?;
    v_u = py::truediv(&py::sub(&py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_ry.clone())?, &py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_rx.clone())?)?, &v_den.clone())?;
    v_eps = Py::Float(1e-09);
    return Ok({ let t2913 = { let t2903 = v_t.clone(); Py::Bool({ let t2904 = py::neg(&v_eps.clone())?; if py::lt(&t2903, &t2904)? { true } else { false } }) }; if t2913.truthy() { t2913 } else { { let t2912 = { let t2905 = v_t.clone(); Py::Bool({ let t2906 = py::add(&Py::Int(1i64), &v_eps.clone())?; if py::gt(&t2905, &t2906)? { true } else { false } }) }; if t2912.truthy() { t2912 } else { { let t2911 = { let t2907 = v_u.clone(); Py::Bool({ let t2908 = py::neg(&v_eps.clone())?; if py::lt(&t2907, &t2908)? { true } else { false } }) }; if t2911.truthy() { t2911 } else { { let t2909 = v_u.clone(); Py::Bool({ let t2910 = py::add(&Py::Int(1i64), &v_eps.clone())?; if py::gt(&t2909, &t2910)? { true } else { false } }) } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_similar(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_other: Py = Py::None;
    let mut v_power: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_k = py::getitem(&v_o.clone(), &Py::str("scale_factor"))?;
    if (py::not(&{ let t2949 = v_k.clone(); Py::Bool({ let t2950 = Py::Int(0i64); if py::gt(&t2949, &t2950)? { true } else { false } }) })).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = py::minmax(&[f__sim_rel(py::mul(&py::getitem(&v_i.clone(), &Py::str("m1"))?, &py::pow(&v_k.clone(), &py::getitem(&c__SIM_POWER()?, &py::getitem(&v_i.clone(), &Py::str("measure"))?)?)?)?, py::getitem(&v_i.clone(), &Py::str("m2"))?)?, f__sim_rel(py::getitem(&v_o.clone(), &Py::str("area_ratio"))?, py::mul(&v_k.clone(), &v_k.clone())?)?, f__sim_rel(py::getitem(&v_o.clone(), &Py::str("volume_ratio"))?, py::mul(&py::mul(&v_k.clone(), &v_k.clone())?, &v_k.clone())?)?], true)?;
    for t2951 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("length2"), Py::str("other_length"), Py::Int(1i64)]), Py::tuple(vec![Py::str("area2"), Py::str("other_area"), Py::Int(2i64)]), Py::tuple(vec![Py::str("volume2"), Py::str("other_volume"), Py::Int(3i64)])])))? {
        let t2958 = (t2951).items()?;
        if t2958.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_name = t2958[0].clone();
        v_other = t2958[1].clone();
        v_power = t2958[2].clone();
        if ({ let t2952 = py::method(&v_i.clone(), "get", &[v_other.clone()])?; Py::Bool({ let t2953 = Py::None; if py::is(&t2952, &t2953) { true } else { false } }) }).truthy() {
            if ({ let t2954 = py::getitem(&v_o.clone(), &v_name.clone())?; Py::Bool({ let t2955 = Py::None; if !py::is(&t2954, &t2955) { true } else { false } }) }).truthy() {
                return Ok(Py::Float(1.0));
            }
        } else {
            if ({ let t2956 = py::getitem(&v_o.clone(), &v_name.clone())?; Py::Bool({ let t2957 = Py::None; if py::is(&t2956, &t2957) { true } else { false } }) }).truthy() {
                return Ok(Py::Float(1.0));
            }
            v_worst = py::minmax(&[v_worst.clone(), f__sim_rel(py::getitem(&v_o.clone(), &v_name.clone())?, py::mul(&py::getitem(&v_i.clone(), &v_other.clone())?, &py::pow(&v_k.clone(), &v_power.clone())?)?)?], true)?;
        }
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_skew(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_D1: Py = Py::None;
    let mut v_D2: Py = Py::None;
    let mut v_P1: Py = Py::None;
    let mut v_P2: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_gap: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2989 = ({ let mut out = vec![]; for t2988 in py::iter(&(Py::tuple(vec![Py::str("p1"), Py::str("d1"), Py::str("p2"), Py::str("d2")])))? { let mut v_k: Py; 
v_k = t2988;
out.push(Py::tuple((py::getitem(&v_i.clone(), &v_k.clone())?).items()?)); } Py::list(out) }).items()?;
    if t2989.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P1 = t2989[0].clone();
    v_D1 = t2989[1].clone();
    v_P2 = t2989[2].clone();
    v_D2 = t2989[3].clone();
    v_w = f__vsub(v_P1.clone(), v_P2.clone())?;
    let t2990 = (Py::tuple(vec![f__vdot(v_D1.clone(), v_D1.clone())?, f__vdot(v_D1.clone(), v_D2.clone())?, f__vdot(v_D2.clone(), v_D2.clone())?])).items()?;
    if t2990.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2990[0].clone();
    v_b = t2990[1].clone();
    v_c = t2990[2].clone();
    let t2991 = (Py::tuple(vec![f__vdot(v_D1.clone(), v_w.clone())?, f__vdot(v_D2.clone(), v_w.clone())?])).items()?;
    if t2991.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t2991[0].clone();
    v_e = t2991[1].clone();
    v_den = py::sub(&py::mul(&v_a.clone(), &v_c.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?;
    if ({ let t2992 = py::getitem(&v_o.clone(), &Py::str("parallel"))?; Py::Bool({ let t2993 = Py::str("yes"); if py::eq(&t2992, &t2993) { true } else { false } }) }).truthy() {
        let t2994 = (Py::tuple(vec![Py::Float(0.0), py::truediv(&v_e.clone(), &v_c.clone())?])).items()?;
        if t2994.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2994[0].clone();
        v_t = t2994[1].clone();
    } else {
        let t2995 = (Py::tuple(vec![py::truediv(&py::sub(&py::mul(&v_b.clone(), &v_e.clone())?, &py::mul(&v_c.clone(), &v_d.clone())?)?, &v_den.clone())?, py::truediv(&py::sub(&py::mul(&v_a.clone(), &v_e.clone())?, &py::mul(&v_b.clone(), &v_d.clone())?)?, &v_den.clone())?])).items()?;
        if t2995.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2995[0].clone();
        v_t = t2995[1].clone();
    }
    v_gap = f__vnorm(Py::tuple(({ let mut out = vec![]; for t2996 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t2996;
out.push(py::sub(&py::add(&py::getitem(&v_w.clone(), &v_k.clone())?, &py::mul(&v_s.clone(), &py::getitem(&v_D1.clone(), &v_k.clone())?)?)?, &py::mul(&v_t.clone(), &py::getitem(&v_D2.clone(), &v_k.clone())?)?)?); } Py::list(out) }).items()?))?;
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("distance"))?, &v_gap.clone())?)?, &py::minmax(&[Py::Float(1.0), f__vnorm(v_w.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_solid_tri(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_arc: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_P = f__triples(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64))?;
    v_U = { let mut out = vec![]; for t3058 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t3058;
out.push(Py::tuple(({ let mut out = vec![]; for t3059 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t3059;
out.push(py::truediv(&v_c.clone(), &f__vnorm(v_p.clone())?)?); } Py::list(out) }).items()?)); } Py::list(out) };
    v_arc = { let v_P = v_P.clone(); let v_U = v_U.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_v: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::atan2(&f__vnorm(f__vcross(v_u.clone(), v_v.clone())?)?, &f__vdot(v_u.clone(), v_v.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t3060 = (Py::tuple(vec![py::call(&v_arc.clone(), &[py::getitem(&v_U.clone(), &Py::Int(1i64))?, py::getitem(&v_U.clone(), &Py::Int(2i64))?])?, py::call(&v_arc.clone(), &[py::getitem(&v_U.clone(), &Py::Int(0i64))?, py::getitem(&v_U.clone(), &Py::Int(2i64))?])?, py::call(&v_arc.clone(), &[py::getitem(&v_U.clone(), &Py::Int(0i64))?, py::getitem(&v_U.clone(), &Py::Int(1i64))?])?])).items()?;
    if t3060.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3060[0].clone();
    v_b = t3060[1].clone();
    v_c = t3060[2].clone();
    v_s = py::truediv(&py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?, &Py::Int(2i64))?;
    v_E = py::mul(&Py::Int(4i64), &py::math1("atan", &py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::mul(&py::mul(&py::mul(&py::math1("tan", &py::truediv(&v_s.clone(), &Py::Int(2i64))?)?, &py::math1("tan", &py::truediv(&py::sub(&v_s.clone(), &v_a.clone())?, &Py::Int(2i64))?)?)?, &py::math1("tan", &py::truediv(&py::sub(&v_s.clone(), &v_b.clone())?, &Py::Int(2i64))?)?)?, &py::math1("tan", &py::truediv(&py::sub(&v_s.clone(), &v_c.clone())?, &Py::Int(2i64))?)?)?], true)?)?)?)?;
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("solid_angle"))?, v_E.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sph(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_num: Py = Py::None;
    v_P = f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(3i64))?;
    let t3220 = ({ let mut out = vec![]; for t3218 in py::iter(&(v_P.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t3219 = (t3218).items()?;
if t3219.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3219[0].clone();
v_y = t3219[1].clone();
out.push(f__unit(py::math1("radians", &v_x.clone())?, py::math1("radians", &v_y.clone())?)?); } Py::list(out) }).items()?;
    if t3220.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3220[0].clone();
    v_b = t3220[1].clone();
    v_c = t3220[2].clone();
    v_num = py::abs(&f__vdot(v_a.clone(), f__vcross(v_b.clone(), v_c.clone())?)?)?;
    v_den = py::add(&py::add(&py::add(&Py::Int(1i64), &f__vdot(v_a.clone(), v_b.clone())?)?, &f__vdot(v_b.clone(), v_c.clone())?)?, &f__vdot(v_a.clone(), v_c.clone())?)?;
    v_E = py::mul(&Py::Int(2i64), &py::atan2(&v_num.clone(), &v_den.clone())?)?;
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("excess"))?)?, v_E.clone()]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::mul(&v_E.clone(), &py::pow(&py::method(&v_i.clone(), "get", &[Py::str("radius"), Py::Float(1.0)])?, &Py::Int(2i64))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_spheroid(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3261 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t3261.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3261[0].clone();
    v_c = t3261[1].clone();
    v_n = Py::Int(4096i64);
    v_h = py::truediv(&Py::Float(std::f64::consts::PI), &v_n.clone())?;
    v_f = { let v_a = v_a.clone(); let v_c = v_c.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_n = v_n.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::math1("cos", &v_t.clone())?, &py::math1("sqrt", &py::add(&py::pow(&py::mul(&v_a.clone(), &py::math1("sin", &v_t.clone())?)?, &Py::Int(2i64))?, &py::pow(&py::mul(&v_c.clone(), &py::math1("cos", &v_t.clone())?)?, &Py::Int(2i64))?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_s = py::fsum(&{ let mut out = vec![]; for t3262 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t3262;
out.push(py::mul(&(if ({ let t3263 = v_k.clone(); Py::Bool({ let t3264 = Py::tuple(vec![Py::Int(0i64), v_n.clone()]); if py::contains(&t3264, &t3263)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { (if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() { Py::Int(4i64) } else { Py::Int(2i64) }) }), &py::call(&v_f.clone(), &[py::add(&py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?, &py::mul(&v_k.clone(), &v_h.clone())?)?])?)?); } Py::list(out) })?;
    v_S = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_s.clone())?, &v_h.clone())?, &Py::Int(3i64))?;
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("surface_area"))?, v_S.clone())?, f__rel(py::getitem(&v_o.clone(), &Py::str("volume"))?, py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_a.clone())?, &v_c.clone())?, &Py::Int(3i64))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_surf(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_EE: Py = Py::None;
    let mut v_FF: Py = Py::None;
    let mut v_GG: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_nn: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_y0: Py = Py::None;
    let t3297 = ({ let mut out = vec![]; for t3296 in py::iter(&(Py::tuple(vec![Py::str("A"), Py::str("B"), Py::str("C"), Py::str("D"), Py::str("E"), Py::str("x0"), Py::str("y0")])))? { let mut v_k: Py; 
v_k = t3296;
out.push(py::getitem(&v_i.clone(), &v_k.clone())?); } Py::list(out) }).items()?;
    if t3297.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3297[0].clone();
    v_B = t3297[1].clone();
    v_C = t3297[2].clone();
    v_D = t3297[3].clone();
    v_E = t3297[4].clone();
    v_x0 = t3297[5].clone();
    v_y0 = t3297[6].clone();
    let t3298 = (Py::tuple(vec![py::add(&py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_x0.clone())?, &py::mul(&v_B.clone(), &v_y0.clone())?)?, &v_D.clone())?, py::add(&py::add(&py::mul(&v_B.clone(), &v_x0.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_C.clone())?, &v_y0.clone())?)?, &v_E.clone())?])).items()?;
    if t3298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3298[0].clone();
    v_q = t3298[1].clone();
    let t3299 = (Py::tuple(vec![py::add(&Py::Int(1i64), &py::mul(&v_p.clone(), &v_p.clone())?)?, py::mul(&v_p.clone(), &v_q.clone())?, py::add(&Py::Int(1i64), &py::mul(&v_q.clone(), &v_q.clone())?)?])).items()?;
    if t3299.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_EE = t3299[0].clone();
    v_FF = t3299[1].clone();
    v_GG = t3299[2].clone();
    v_nn = py::math1("sqrt", &py::add(&py::add(&Py::Int(1i64), &py::mul(&v_p.clone(), &v_p.clone())?)?, &py::mul(&v_q.clone(), &v_q.clone())?)?)?;
    let t3300 = (Py::tuple(vec![py::truediv(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_nn.clone())?, py::truediv(&v_B.clone(), &v_nn.clone())?, py::truediv(&py::mul(&Py::Int(2i64), &v_C.clone())?, &v_nn.clone())?])).items()?;
    if t3300.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_L = t3300[0].clone();
    v_M = t3300[1].clone();
    v_N = t3300[2].clone();
    v_det = py::sub(&py::mul(&v_EE.clone(), &v_GG.clone())?, &py::mul(&v_FF.clone(), &v_FF.clone())?)?;
    v_K = py::truediv(&py::sub(&py::mul(&v_L.clone(), &v_N.clone())?, &py::mul(&v_M.clone(), &v_M.clone())?)?, &v_det.clone())?;
    v_H = py::truediv(&py::add(&py::sub(&py::mul(&v_EE.clone(), &v_N.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_FF.clone())?, &v_M.clone())?)?, &py::mul(&v_GG.clone(), &v_L.clone())?)?, &py::mul(&Py::Int(2i64), &v_det.clone())?)?;
    v_sc = py::minmax(&[Py::Float(1.0), py::abs(&v_A.clone())?, py::abs(&v_B.clone())?, py::abs(&v_C.clone())?], true)?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("gaussian"))?, &v_K.clone())?)?, &py::minmax(&[py::abs(&v_K.clone())?, py::mul(&py::mul(&v_sc.clone(), &v_sc.clone())?, &Py::Float(1e-06))?], true)?)?, py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("mean"))?, &v_H.clone())?)?, &py::minmax(&[py::abs(&v_H.clone())?, py::mul(&v_sc.clone(), &Py::Float(1e-06))?], true)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tangent(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_dot: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3327 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("cx"))?, py::getitem(&v_i.clone(), &Py::str("cy"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("px"))?, py::getitem(&v_i.clone(), &Py::str("py"))?]), py::getitem(&v_i.clone(), &Py::str("r"))?])).items()?;
    if t3327.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_C = t3327[0].clone();
    v_P = t3327[1].clone();
    v_r = t3327[2].clone();
    v_worst = py::truediv(&py::abs(&py::sub(&py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("length"))?, &Py::Int(2i64))?, &py::mul(&v_r.clone(), &v_r.clone())?)?, &py::pow(&py::dist(&v_C.clone(), &v_P.clone())?, &Py::Int(2i64))?)?)?, &py::pow(&py::dist(&v_C.clone(), &v_P.clone())?, &Py::Int(2i64))?)?;
    for t3328 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)])))? {
        v_n = t3328;
        v_T = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str(&["x".to_string(), py::format(&v_n.clone(), "")?].concat()))?, py::getitem(&v_o.clone(), &Py::str(&["y".to_string(), py::format(&v_n.clone(), "")?].concat()))?]);
        v_dot = py::add(&py::mul(&py::sub(&py::getitem(&v_T.clone(), &Py::Int(0i64))?, &py::getitem(&v_C.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_T.clone(), &Py::Int(0i64))?, &py::getitem(&v_P.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_T.clone(), &Py::Int(1i64))?, &py::getitem(&v_C.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_T.clone(), &Py::Int(1i64))?, &py::getitem(&v_P.clone(), &Py::Int(1i64))?)?)?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::dist(&v_T.clone(), &v_C.clone())?, &v_r.clone())?)?, &v_r.clone())?, py::truediv(&py::abs(&v_dot.clone())?, &py::mul(&v_r.clone(), &py::minmax(&[py::getitem(&v_o.clone(), &Py::str("length"))?, v_r.clone()], true)?)?)?, py::truediv(&py::abs(&py::sub(&py::dist(&v_T.clone(), &v_P.clone())?, &py::getitem(&v_o.clone(), &Py::str("length"))?)?)?, &py::minmax(&[py::getitem(&v_o.clone(), &Py::str("length"))?, v_r.clone()], true)?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tetra(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_V2: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_P = f__triples(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(4i64))?;
    v_d = { let mut out = vec![]; for t3356 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_a: Py; 
v_a = t3356;
out.push({ let mut out = vec![]; for t3357 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_b: Py; 
v_b = t3357;
out.push(py::pow(&py::dist(&py::getitem(&v_P.clone(), &v_a.clone())?, &py::getitem(&v_P.clone(), &v_b.clone())?)?, &Py::Int(2i64))?); } Py::list(out) }); } Py::list(out) };
    v_M = py::add(&Py::list(vec![Py::list(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(1i64), Py::Int(1i64), Py::Int(1i64)])]), &{ let mut out = vec![]; for t3358 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_a: Py; 
v_a = t3358;
out.push(py::add(&Py::list(vec![Py::Int(1i64)]), &py::getitem(&v_d.clone(), &v_a.clone())?)?); } Py::list(out) })?;
    let rec3359 = py::rec_cell();
    v_det = { let v_M = v_M.clone(); let v_P = v_P.clone(); let v_d = v_d.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let rec3359 = rec3359.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_det: Py = py::rec_get(&rec3359)?;
            if ({ let t3360 = py::len(&v_m.clone())?; Py::Bool({ let t3361 = Py::Int(1i64); if py::eq(&t3360, &t3361) { true } else { false } }) }).truthy() {
                return Ok(py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?);
            }
            return Ok(py::sum(&{ let mut out = vec![]; for t3362 in py::range_iter(&[(py::len(&v_m.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3362;
if (py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &v_j.clone())?).truthy() { out.push(py::mul(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_j.clone())?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &v_j.clone())?)?, &py::call(&v_det.clone(), &[{ let mut out = vec![]; for t3363 in py::iter(&(py::slice(&v_m.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_r: Py; 
v_r = t3363;
out.push(py::add(&py::slice(&v_r.clone(), None, Some((v_j.clone()).i()?), None)?, &py::slice(&v_r.clone(), Some((py::add(&v_j.clone(), &Py::Int(1i64))?).i()?), None, None)?)?); } Py::list(out) }])?)?); }} Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec3359, &v_det);
    v_V2 = py::truediv(&py::call(&v_det.clone(), &[v_M.clone()])?, &Py::Int(288i64))?;
    v_S = Py::Float(0.0);
    for t3364 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(2i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(3i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(2i64), Py::Int(3i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)])])))? {
        let t3366 = (t3364).items()?;
        if t3366.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3366[0].clone();
        v_b = t3366[1].clone();
        v_c = t3366[2].clone();
        let t3365 = (py::sorted(&Py::tuple(vec![py::dist(&py::getitem(&v_P.clone(), &v_b.clone())?, &py::getitem(&v_P.clone(), &v_c.clone())?)?, py::dist(&py::getitem(&v_P.clone(), &v_a.clone())?, &py::getitem(&v_P.clone(), &v_c.clone())?)?, py::dist(&py::getitem(&v_P.clone(), &v_a.clone())?, &py::getitem(&v_P.clone(), &v_b.clone())?)?]), (Py::Bool(true)).truthy())?).items()?;
        if t3365.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_x = t3365[0].clone();
        v_y = t3365[1].clone();
        v_z = t3365[2].clone();
        v_S = py::iadd(&v_S, &py::mul(&Py::Float(0.25), &py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::mul(&py::mul(&py::mul(&py::add(&v_x.clone(), &py::add(&v_y.clone(), &v_z.clone())?)?, &py::sub(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_x.clone(), &py::sub(&v_y.clone(), &v_z.clone())?)?)?], true)?)?)?)?;
    }
    v_scale = py::pow(&py::minmax(&[py::minmax(&[{ let mut out = vec![]; for t3367 in py::iter(&(v_d.clone()))? { let mut v_row: Py; 
v_row = t3367;
for t3368 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t3368;
out.push(py::abs(&v_v.clone())?); }} Py::list(out) }], true)?, Py::Float(1.0)], true)?, &Py::Float(1.5))?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("volume"))?, &Py::Int(2i64))?, &v_V2.clone())?)?, &py::minmax(&[py::pow(&py::getitem(&v_o.clone(), &Py::str("volume"))?, &Py::Int(2i64))?, py::mul(&Py::Float(1e-06), &py::pow(&v_scale.clone(), &Py::Int(2i64))?)?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("surface_area"))?, v_S.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tri_coords(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let t3713 = (f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(3i64))?).items()?;
    if t3713.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3713[0].clone();
    v_B = t3713[1].clone();
    v_C = t3713[2].clone();
    let t3714 = (Py::tuple(vec![py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_B.clone())?])).items()?;
    if t3714.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3714[0].clone();
    v_b = t3714[1].clone();
    v_c = t3714[2].clone();
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::add(&py::add(&py::getitem(&v_o.clone(), &Py::str("angle_A"))?, &py::getitem(&v_o.clone(), &Py::str("angle_B"))?)?, &py::getitem(&v_o.clone(), &Py::str("angle_C"))?)?, Py::Float(180.0)]), Py::tuple(vec![py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("area"))?)?, py::mul(&py::mul(&v_b.clone(), &v_c.clone())?, &py::math1("sin", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("angle_A"))?)?)?)?]), Py::tuple(vec![py::mul(&v_a.clone(), &py::math1("sin", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("angle_B"))?)?)?)?, py::mul(&v_b.clone(), &py::math1("sin", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("angle_A"))?)?)?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?]), Py::tuple(vec![py::mul(&Py::Int(3i64), &py::getitem(&v_o.clone(), &Py::str("gx"))?)?, py::add(&py::add(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &py::getitem(&v_B.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_C.clone(), &Py::Int(0i64))?)?]), Py::tuple(vec![py::mul(&Py::Int(3i64), &py::getitem(&v_o.clone(), &Py::str("gy"))?)?, py::add(&py::add(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &py::getitem(&v_B.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_C.clone(), &Py::Int(1i64))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vnorm(mut v_a: Py) -> R {
    return Ok(py::math1("sqrt", &f__vdot(v_a.clone(), v_a.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vsub(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(Py::tuple(vec![py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__xcross(mut v_o: Py, mut v_a: Py, mut v_b: Py) -> R {
    let t3794 = ({ let mut out = vec![]; for t3793 in py::iter(&(Py::tuple(vec![v_o.clone(), v_a.clone(), v_b.clone()])))? { let mut v_p: Py; 
v_p = t3793;
out.push(Py::tuple(vec![f__fr(py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, f__fr(py::getitem(&v_p.clone(), &Py::Int(1i64))?)?])); } Py::list(out) }).items()?;
    if t3794.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_o = t3794[0].clone();
    v_a = t3794[1].clone();
    v_b = t3794[2].clone();
    return Ok(py::sub(&py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__xvec(mut v_p: Py) -> R {
    return Ok(Py::tuple(({ let mut out = vec![]; for t3503 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t3503;
out.push(f__fr(v_c.clone())?); } Py::list(out) }).items()?));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angle_between_lines(mut v_a1: Py, mut v_b1: Py, mut v_c1: Py, mut v_a2: Py, mut v_b2: Py, mut v_c2: Py) -> R {
    let mut v_cr: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let t10 = (f__line_check(v_a1.clone(), v_b1.clone(), v_c1.clone(), Py::str("line 1"))?).items()?;
    if t10.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t10[0].clone();
    v_b1 = t10[1].clone();
    v_c1 = t10[2].clone();
    let t11 = (f__line_check(v_a2.clone(), v_b2.clone(), v_c2.clone(), Py::str("line 2"))?).items()?;
    if t11.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t11[0].clone();
    v_b2 = t11[1].clone();
    v_c2 = t11[2].clone();
    let t12 = (Py::tuple(vec![py::sub(&py::mul(&v_a1.clone(), &v_b2.clone())?, &py::mul(&v_a2.clone(), &v_b1.clone())?)?, py::add(&py::mul(&v_a1.clone(), &v_a2.clone())?, &py::mul(&v_b1.clone(), &v_b2.clone())?)?])).items()?;
    if t12.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cr = t12[0].clone();
    v_dt = t12[1].clone();
    return Ok(Py::dict(vec![(Py::str("angle"), py::math1("degrees", &py::atan2(&py::abs(&v_cr.clone())?, &py::abs(&v_dt.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angle_between_planes(mut v_a1: Py, mut v_b1: Py, mut v_c1: Py, mut v_a2: Py, mut v_b2: Py, mut v_c2: Py) -> R {
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    v_n1 = Py::tuple(({ let mut out = vec![]; for t32 in py::iter(&(Py::tuple(vec![v_a1.clone(), v_b1.clone(), v_c1.clone()])))? { let mut v_v: Py; 
v_v = t32;
out.push(f__fin(v_v.clone(), Py::str("plane 1"))?); } Py::list(out) }).items()?);
    v_n2 = Py::tuple(({ let mut out = vec![]; for t35 in py::iter(&(Py::tuple(vec![v_a2.clone(), v_b2.clone(), v_c2.clone()])))? { let mut v_v: Py; 
v_v = t35;
out.push(f__fin(v_v.clone(), Py::str("plane 2"))?); } Py::list(out) }).items()?);
    if ({ let t40 = { let t36 = v_n1.clone(); Py::Bool({ let t37 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t36, &t37) { true } else { false } }) }; if t40.truthy() { t40 } else { { let t38 = v_n2.clone(); Py::Bool({ let t39 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t38, &t39) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a plane's normal (a, b, c) must not be zero"))));
    }
    return Ok(Py::dict(vec![(Py::str("angle"), py::math1("degrees", &py::atan2(&f__vnorm(f__vcross(v_n1.clone(), v_n2.clone())?)?, &py::abs(&f__vdot(v_n1.clone(), v_n2.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_annulus_area(mut v_outer: Py, mut v_inner: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t61 = (Py::tuple(vec![f__pos(v_outer.clone(), Py::str("outer radius"))?, f__pos(v_inner.clone(), Py::str("inner radius"))?])).items()?;
    if t61.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t61[0].clone();
    v_r = t61[1].clone();
    if ({ let t62 = v_r.clone(); Py::Bool({ let t63 = v_R.clone(); if py::ge(&t62, &t63)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inner radius must be smaller than the outer radius"))));
    }
    return Ok(Py::dict(vec![(Py::str("area"), py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::sub(&v_R.clone(), &v_r.clone())?)?, &py::add(&v_R.clone(), &v_r.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_barycentric(mut v_px: Py, mut v_py: Py, mut v_points: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_T: Py = Py::None;
    let t105 = (f__tri_pts(v_points.clone())?).items()?;
    if t105.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t105[0].clone();
    v_B = t105[1].clone();
    v_C = t105[2].clone();
    v_P = Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?]);
    v_T = f__cross(v_A.clone(), v_B.clone(), v_C.clone())?;
    return Ok(Py::dict(vec![(Py::str("l1"), py::add(&py::truediv(&f__cross(v_P.clone(), v_B.clone(), v_C.clone())?, &v_T.clone())?, &Py::Float(0.0))?), (Py::str("l2"), py::add(&py::truediv(&f__cross(v_A.clone(), v_P.clone(), v_C.clone())?, &v_T.clone())?, &Py::Float(0.0))?), (Py::str("l3"), py::add(&py::truediv(&f__cross(v_A.clone(), v_B.clone(), v_P.clone())?, &v_T.clone())?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_circle_circle_intersection(mut v_x1: Py, mut v_y1: Py, mut v_r1: Py, mut v_x2: Py, mut v_y2: Py, mut v_r2: Py) -> R {
    let mut v_R1: Py = Py::None;
    let mut v_R2: Py = Py::None;
    let mut v_X1: Py = Py::None;
    let mut v_X2: Py = Py::None;
    let mut v_Y1: Py = Py::None;
    let mut v_Y2: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_a_: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_h2: Py = Py::None;
    let mut v_ux: Py = Py::None;
    let mut v_uy: Py = Py::None;
    let t128 = (Py::tuple(vec![f__fin(v_x1.clone(), Py::str("x1"))?, f__fin(v_y1.clone(), Py::str("y1"))?, f__pos(v_r1.clone(), Py::str("r1"))?, f__fin(v_x2.clone(), Py::str("x2"))?, f__fin(v_y2.clone(), Py::str("y2"))?, f__pos(v_r2.clone(), Py::str("r2"))?])).items()?;
    if t128.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_x1 = t128[0].clone();
    v_y1 = t128[1].clone();
    v_r1 = t128[2].clone();
    v_x2 = t128[3].clone();
    v_y2 = t128[4].clone();
    v_r2 = t128[5].clone();
    let t130 = ({ let mut out = vec![]; for t129 in py::iter(&(Py::tuple(vec![v_x1.clone(), v_y1.clone(), v_r1.clone(), v_x2.clone(), v_y2.clone(), v_r2.clone()])))? { let mut v_v: Py; 
v_v = t129;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t130.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_X1 = t130[0].clone();
    v_Y1 = t130[1].clone();
    v_R1 = t130[2].clone();
    v_X2 = t130[3].clone();
    v_Y2 = t130[4].clone();
    v_R2 = t130[5].clone();
    v_d2 = py::add(&py::pow(&py::sub(&v_X2.clone(), &v_X1.clone())?, &Py::Int(2i64))?, &py::pow(&py::sub(&v_Y2.clone(), &v_Y1.clone())?, &Py::Int(2i64))?)?;
    if ({ let t131 = v_d2.clone(); Py::Bool({ let t132 = Py::Int(0i64); if py::eq(&t131, &t132) { true } else { false } }) }).truthy() {
        if ({ let t133 = v_R1.clone(); Py::Bool({ let t134 = v_R2.clone(); if py::eq(&t133, &t134) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the circles coincide: they share infinitely many points"))));
        }
        return Ok(f__two_pts_out(Py::list(vec![]))?);
    }
    if ({ let t143 = { let t139 = v_d2.clone(); Py::Bool({ let t140 = py::pow(&py::add(&v_R1.clone(), &v_R2.clone())?, &Py::Int(2i64))?; if py::gt(&t139, &t140)? { true } else { false } }) }; if t143.truthy() { t143 } else { { let t141 = v_d2.clone(); Py::Bool({ let t142 = py::pow(&py::sub(&v_R1.clone(), &v_R2.clone())?, &Py::Int(2i64))?; if py::lt(&t141, &t142)? { true } else { false } }) } } }).truthy() {
        return Ok(f__two_pts_out(Py::list(vec![]))?);
    }
    v_a_ = py::truediv(&py::sub(&py::add(&v_d2.clone(), &py::mul(&v_R1.clone(), &v_R1.clone())?)?, &py::mul(&v_R2.clone(), &v_R2.clone())?)?, &Py::Int(2i64))?;
    v_h2 = py::sub(&py::mul(&v_R1.clone(), &v_R1.clone())?, &py::truediv(&py::mul(&v_a_.clone(), &v_a_.clone())?, &v_d2.clone())?)?;
    v_d = py::math1("sqrt", &py::float(&v_d2.clone())?)?;
    let t144 = (Py::tuple(vec![py::truediv(&py::sub(&v_x2.clone(), &v_x1.clone())?, &v_d.clone())?, py::truediv(&py::sub(&v_y2.clone(), &v_y1.clone())?, &v_d.clone())?])).items()?;
    if t144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ux = t144[0].clone();
    v_uy = t144[1].clone();
    v_a = py::truediv(&py::float(&v_a_.clone())?, &v_d.clone())?;
    let t145 = (Py::tuple(vec![py::add(&v_x1.clone(), &py::mul(&v_a.clone(), &v_ux.clone())?)?, py::add(&v_y1.clone(), &py::mul(&v_a.clone(), &v_uy.clone())?)?])).items()?;
    if t145.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t145[0].clone();
    v_fy = t145[1].clone();
    if ({ let t146 = v_h2.clone(); Py::Bool({ let t147 = Py::Int(0i64); if py::eq(&t146, &t147) { true } else { false } }) }).truthy() {
        return Ok(f__two_pts_out(Py::list(vec![Py::tuple(vec![v_fx.clone(), v_fy.clone()])]))?);
    }
    v_h = py::math1("sqrt", &py::float(&v_h2.clone())?)?;
    return Ok(f__two_pts_out(Py::list(vec![Py::tuple(vec![py::sub(&v_fx.clone(), &py::mul(&v_h.clone(), &v_uy.clone())?)?, py::add(&v_fy.clone(), &py::mul(&v_h.clone(), &v_ux.clone())?)?]), Py::tuple(vec![py::add(&v_fx.clone(), &py::mul(&v_h.clone(), &v_uy.clone())?)?, py::sub(&v_fy.clone(), &py::mul(&v_h.clone(), &v_ux.clone())?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_circle_line_intersection(mut v_cx: Py, mut v_cy: Py, mut v_r: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_Cc: Py = Py::None;
    let mut v_Rr: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_gap: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_num: Py = Py::None;
    let t192 = (Py::tuple(vec![f__fin(v_cx.clone(), Py::str("cx"))?, f__fin(v_cy.clone(), Py::str("cy"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t192.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t192[0].clone();
    v_cy = t192[1].clone();
    v_r = t192[2].clone();
    let t199 = (f__line_check(v_a.clone(), v_b.clone(), v_c.clone(), Py::str("line"))?).items()?;
    if t199.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t199[0].clone();
    v_b = t199[1].clone();
    v_c = t199[2].clone();
    let t201 = ({ let mut out = vec![]; for t200 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_cx.clone(), v_cy.clone(), v_r.clone()])))? { let mut v_v: Py; 
v_v = t200;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t201.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A = t201[0].clone();
    v_B = t201[1].clone();
    v_Cc = t201[2].clone();
    v_X = t201[3].clone();
    v_Y = t201[4].clone();
    v_Rr = t201[5].clone();
    v_num = py::pow(&py::add(&py::add(&py::mul(&v_A.clone(), &v_X.clone())?, &py::mul(&v_B.clone(), &v_Y.clone())?)?, &v_Cc.clone())?, &Py::Int(2i64))?;
    v_n2 = py::add(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&v_B.clone(), &v_B.clone())?)?;
    v_gap = py::sub(&py::mul(&py::mul(&v_Rr.clone(), &v_Rr.clone())?, &v_n2.clone())?, &v_num.clone())?;
    if ({ let t202 = v_gap.clone(); Py::Bool({ let t203 = Py::Int(0i64); if py::lt(&t202, &t203)? { true } else { false } }) }).truthy() {
        return Ok(f__two_pts_out(Py::list(vec![]))?);
    }
    v_k = py::truediv(&py::add(&py::add(&py::mul(&v_a.clone(), &v_cx.clone())?, &py::mul(&v_b.clone(), &v_cy.clone())?)?, &v_c.clone())?, &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?)?;
    let t208 = (Py::tuple(vec![py::sub(&v_cx.clone(), &py::mul(&v_a.clone(), &v_k.clone())?)?, py::sub(&v_cy.clone(), &py::mul(&v_b.clone(), &v_k.clone())?)?])).items()?;
    if t208.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t208[0].clone();
    v_fy = t208[1].clone();
    if ({ let t209 = v_gap.clone(); Py::Bool({ let t210 = Py::Int(0i64); if py::eq(&t209, &t210) { true } else { false } }) }).truthy() {
        return Ok(f__two_pts_out(Py::list(vec![Py::tuple(vec![v_fx.clone(), v_fy.clone()])]))?);
    }
    v_h = py::truediv(&py::math1("sqrt", &py::float(&py::truediv(&v_gap.clone(), &v_n2.clone())?)?)?, &py::math1("sqrt", &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?)?)?;
    return Ok(f__two_pts_out(Py::list(vec![Py::tuple(vec![py::sub(&v_fx.clone(), &py::mul(&v_b.clone(), &v_h.clone())?)?, py::add(&v_fy.clone(), &py::mul(&v_a.clone(), &v_h.clone())?)?]), Py::tuple(vec![py::add(&v_fx.clone(), &py::mul(&v_b.clone(), &v_h.clone())?)?, py::sub(&v_fy.clone(), &py::mul(&v_a.clone(), &v_h.clone())?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_circle_measures(mut v_radius: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = f__pos(v_radius.clone(), Py::str("radius"))?;
    return Ok(Py::dict(vec![(Py::str("area"), py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &v_r.clone())?), (Py::str("circumference"), py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?), (Py::str("diameter"), py::mul(&Py::Int(2i64), &v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_circular_segment(mut v_radius: Py, mut v_angle: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tms: Py = Py::None;
    let t282 = (Py::tuple(vec![f__pos(v_radius.clone(), Py::str("radius"))?, f__fin(v_angle.clone(), Py::str("angle"))?])).items()?;
    if t282.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t282[0].clone();
    v_a = t282[1].clone();
    if (py::not(&{ let t283 = Py::Int(0i64); Py::Bool({ let t284 = v_a.clone(); if py::lt(&t283, &t284)? { { let t285 = Py::Int(360i64); if py::lt(&t284, &t285)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the central angle must be between 0 and 360 degrees (exclusive)"))));
    }
    v_t = py::math1("radians", &v_a.clone())?;
    if ({ let t286 = v_t.clone(); Py::Bool({ let t287 = Py::Float(0.1); if py::lt(&t286, &t287)? { true } else { false } }) }).truthy() {
        v_tms = py::add(&py::sub(&py::add(&py::sub(&py::truediv(&py::pow(&v_t.clone(), &Py::Int(3i64))?, &Py::Int(6i64))?, &py::truediv(&py::pow(&v_t.clone(), &Py::Int(5i64))?, &Py::Int(120i64))?)?, &py::truediv(&py::pow(&v_t.clone(), &Py::Int(7i64))?, &Py::Int(5040i64))?)?, &py::truediv(&py::pow(&v_t.clone(), &Py::Int(9i64))?, &Py::Int(362880i64))?)?, &py::truediv(&py::pow(&v_t.clone(), &Py::Int(11i64))?, &Py::Int(39916800i64))?)?;
    } else {
        v_tms = py::sub(&v_t.clone(), &py::math1("sin", &v_t.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("area"), py::truediv(&py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &v_tms.clone())?, &Py::Int(2i64))?), (Py::str("chord"), py::mul(&py::mul(&Py::Int(2i64), &v_r.clone())?, &py::math1("sin", &py::truediv(&v_t.clone(), &Py::Int(2i64))?)?)?), (Py::str("height"), py::mul(&py::mul(&Py::Int(2i64), &v_r.clone())?, &py::pow(&py::math1("sin", &py::truediv(&v_t.clone(), &Py::Int(4i64))?)?, &Py::Int(2i64))?)?), (Py::str("arc_length"), py::mul(&v_r.clone(), &v_t.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_closest_pair(mut v_points: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_y0: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(2i64), Py::Int(2000i64))?;
    v_P = py::sorted(&v_pts.clone(), (Py::Bool(false)).truthy())?;
    v_best = Py::Float(f64::INFINITY);
    for t327 in py::range_iter(&[(py::len(&v_P.clone())?).i()?]) {
        v_k = t327;
        let t328 = (py::getitem(&v_P.clone(), &v_k.clone())?).items()?;
        if t328.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x0 = t328[0].clone();
        v_y0 = t328[1].clone();
        for t329 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (py::len(&v_P.clone())?).i()?]) {
            v_j = t329;
            v_dx = py::sub(&py::getitem(&py::getitem(&v_P.clone(), &v_j.clone())?, &Py::Int(0i64))?, &v_x0.clone())?;
            if ({ let t330 = v_dx.clone(); Py::Bool({ let t331 = v_best.clone(); if py::ge(&t330, &t331)? { true } else { false } }) }).truthy() {
                break;
            }
            v_d = py::hypot(&[v_dx.clone(), py::sub(&py::getitem(&py::getitem(&v_P.clone(), &v_j.clone())?, &Py::Int(1i64))?, &v_y0.clone())?])?;
            if ({ let t332 = v_d.clone(); Py::Bool({ let t333 = v_best.clone(); if py::lt(&t332, &t333)? { true } else { false } }) }).truthy() {
                v_best = v_d.clone();
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("distance"), v_best.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cone_measures(mut v_radius: Py, mut v_height: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t357 = (Py::tuple(vec![f__pos(v_radius.clone(), Py::str("radius"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t357[0].clone();
    v_h = t357[1].clone();
    v_s = py::hypot(&[v_r.clone(), v_h.clone()])?;
    return Ok(Py::dict(vec![(Py::str("volume"), py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &v_r.clone())?, &v_h.clone())?, &Py::Int(3i64))?), (Py::str("slant_height"), v_s.clone()), (Py::str("lateral_area"), py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &v_s.clone())?), (Py::str("total_area"), py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &py::add(&v_r.clone(), &v_s.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_conic_classify(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_D: Py, mut v_E: Py, mut v_F: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t386 in py::iter(&(py::zip(&[Py::str("ABCDEF"), Py::tuple(vec![v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone(), v_E.clone(), v_F.clone()])])?))? {
        let t389 = (t386).items()?;
        if t389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t389[0].clone();
        v_v = t389[1].clone();
        let _ = f__fin(v_v.clone(), v_n.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("type"), f__conic_type(v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone(), v_E.clone(), v_F.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_convex_hull(mut v_points: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::Int(5000i64))?;
    v_h = f__hull(v_pts.clone())?;
    if ({ let t474 = py::len(&v_h.clone())?; Py::Bool({ let t475 = Py::Int(3i64); if py::lt(&t474, &t475)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all points are collinear (or fewer than 3 distinct): the hull has no area"))));
    }
    return Ok(Py::dict(vec![(Py::str("hull"), { let mut out = vec![]; for t476 in py::iter(&(v_h.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t477 = (t476).items()?;
if t477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t477[0].clone();
v_y = t477[1].clone();
out.push(Py::list(vec![v_x.clone(), v_y.clone()])); } Py::list(out) }), (Py::str("n_vertices"), py::len(&v_h.clone())?), (Py::str("area"), py::abs(&f__shoelace(v_h.clone())?)?), (Py::str("perimeter"), f__perimeter(v_h.clone(), Py::Bool(true))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_convex_intersection(mut v_p: Py, mut v_q: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_R: Py = Py::None;
    v_P = f__ccw_convex(f__pairs(v_p.clone(), Py::str("p"), Py::Int(3i64), Py::None)?, Py::str("p"))?;
    v_Q = f__ccw_convex(f__pairs(v_q.clone(), Py::str("q"), Py::Int(3i64), Py::None)?, Py::str("q"))?;
    v_R = f__clip(v_Q.clone(), v_P.clone())?;
    return Ok(Py::dict(vec![(Py::str("area"), (if ({ let t633 = py::len(&v_R.clone())?; Py::Bool({ let t634 = Py::Int(3i64); if py::ge(&t633, &t634)? { true } else { false } }) }).truthy() { py::abs(&f__shoelace(v_R.clone())?)? } else { Py::Float(0.0) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cuboid_measures(mut v_length: Py, mut v_width: Py, mut v_height: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t657 = (Py::tuple(vec![f__pos(v_length.clone(), Py::str("length"))?, f__pos(v_width.clone(), Py::str("width"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t657.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_l = t657[0].clone();
    v_w = t657[1].clone();
    v_h = t657[2].clone();
    return Ok(Py::dict(vec![(Py::str("volume"), py::mul(&py::mul(&v_l.clone(), &v_w.clone())?, &v_h.clone())?), (Py::str("surface_area"), py::mul(&Py::Int(2i64), &py::add(&py::add(&py::mul(&v_l.clone(), &v_w.clone())?, &py::mul(&v_l.clone(), &v_h.clone())?)?, &py::mul(&v_w.clone(), &v_h.clone())?)?)?), (Py::str("space_diagonal"), py::math1("sqrt", &py::add(&py::add(&py::mul(&v_l.clone(), &v_l.clone())?, &py::mul(&v_w.clone(), &v_w.clone())?)?, &py::mul(&v_h.clone(), &v_h.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_curvature_graph(mut v_coefficients: Py, mut v_x0: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_yp: Py = Py::None;
    let mut v_ypp: Py = Py::None;
    v_co = { let mut out = vec![]; for t686 in py::iter(&(v_coefficients.clone()))? { let mut v_c: Py; 
v_c = t686;
out.push(f__fin(py::float(&v_c.clone())?, Py::str("coefficients"))?); } Py::list(out) };
    v_x0 = f__fin(v_x0.clone(), Py::str("x0"))?;
    if ({ let t689 = py::len(&v_co.clone())?; Py::Bool({ let t690 = Py::Int(11i64); if py::gt(&t689, &t690)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("polynomials up to degree 10"))));
    }
    let t694 = (Py::tuple(vec![f__deriv(v_co.clone())?, f__deriv(f__deriv(v_co.clone())?)?])).items()?;
    if t694.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t694[0].clone();
    v_d2 = t694[1].clone();
    let t696 = (Py::tuple(vec![f__horner(v_d1.clone(), v_x0.clone())?, f__horner(v_d2.clone(), v_x0.clone())?])).items()?;
    if t696.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_yp = t696[0].clone();
    v_ypp = t696[1].clone();
    v_k = py::truediv(&py::abs(&v_ypp.clone())?, &py::pow(&py::add(&Py::Int(1i64), &py::mul(&v_yp.clone(), &v_yp.clone())?)?, &Py::Float(1.5))?)?;
    return Ok(Py::dict(vec![(Py::str("curvature"), v_k.clone()), (Py::str("radius_of_curvature"), (if (v_k.clone()).truthy() { py::truediv(&Py::Int(1i64), &v_k.clone())? } else { Py::None })), (Py::str("slope"), v_yp.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cylinder_measures(mut v_radius: Py, mut v_height: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t731 = (Py::tuple(vec![f__pos(v_radius.clone(), Py::str("radius"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t731[0].clone();
    v_h = t731[1].clone();
    return Ok(Py::dict(vec![(Py::str("volume"), py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &v_r.clone())?, &v_h.clone())?), (Py::str("lateral_area"), py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?, &v_h.clone())?), (Py::str("total_area"), py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?, &py::add(&v_r.clone(), &v_h.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_delaunay_counts(mut v_points: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_rows: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::Int(25i64))?;
    if ({ let t771 = py::len(&py::set(&v_pts.clone())?)?; Py::Bool({ let t772 = py::len(&v_pts.clone())?; if py::ne(&t771, &t772) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a point is repeated"))));
    }
    v_X = { let mut out = vec![]; for t773 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t774 = (t773).items()?;
if t774.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t774[0].clone();
v_y = t774[1].clone();
out.push(Py::tuple(vec![f__fr(v_x.clone())?, f__fr(v_y.clone())?])); } Py::list(out) };
    for t775 in py::iter(&(py::itertools("combinations", &[v_X.clone(), Py::Int(3i64)])?))? {
        let t778 = (t775).items()?;
        if t778.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t778[0].clone();
        v_b = t778[1].clone();
        v_c = t778[2].clone();
        if ({ let t776 = py::sub(&py::mul(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?)?)?; Py::Bool({ let t777 = Py::Int(0i64); if py::eq(&t776, &t777) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("three points are collinear: not in general position"))));
        }
    }
    for t779 in py::iter(&(py::itertools("combinations", &[v_X.clone(), Py::Int(4i64)])?))? {
        let t783 = (t779).items()?;
        if t783.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_a = t783[0].clone();
        v_b = t783[1].clone();
        v_c = t783[2].clone();
        v_d = t783[3].clone();
        v_rows = { let mut out = vec![]; for t780 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()])))? { let mut v_p: Py; 
v_p = t780;
out.push(Py::tuple(vec![py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?, py::add(&py::pow(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_d.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_d.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?])); } Py::list(out) };
        v_det = py::add(&py::sub(&py::mul(&py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::sub(&py::mul(&py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?)?, &py::mul(&py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::sub(&py::mul(&py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?)?)?, &py::mul(&py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::sub(&py::mul(&py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_rows.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?)?)?;
        if ({ let t781 = v_det.clone(); Py::Bool({ let t782 = Py::Int(0i64); if py::eq(&t781, &t782) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("four points are concyclic: the Delaunay triangulation is not unique"))));
        }
    }
    let t795 = (Py::tuple(vec![py::len(&v_pts.clone())?, py::len(&f__hull(v_pts.clone())?)?])).items()?;
    if t795.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t795[0].clone();
    v_h = t795[1].clone();
    return Ok(Py::dict(vec![(Py::str("triangles"), py::sub(&py::sub(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(2i64))?, &v_h.clone())?), (Py::str("edges"), py::sub(&py::sub(&py::mul(&Py::Int(3i64), &v_n.clone())?, &Py::Int(3i64))?, &v_h.clone())?), (Py::str("hull_vertices"), v_h.clone()), (Py::str("voronoi_vertices"), py::sub(&py::sub(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(2i64))?, &v_h.clone())?), (Py::str("unbounded_cells"), v_h.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dilate_point(mut v_px: Py, mut v_py: Py, mut v_k: Py, mut v_cx: Py, mut v_cy: Py) -> R {
    let t827 = (Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?, f__fin(v_k.clone(), Py::str("k"))?])).items()?;
    if t827.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_px = t827[0].clone();
    v_py = t827[1].clone();
    v_k = t827[2].clone();
    let t828 = (Py::tuple(vec![f__fin(v_cx.clone(), Py::str("cx"))?, f__fin(v_cy.clone(), Py::str("cy"))?])).items()?;
    if t828.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t828[0].clone();
    v_cy = t828[1].clone();
    if ({ let t829 = v_k.clone(); Py::Bool({ let t830 = Py::Int(0i64); if py::eq(&t829, &t830) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the scale factor must not be 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("x"), py::add(&py::add(&v_cx.clone(), &py::mul(&v_k.clone(), &py::sub(&v_px.clone(), &v_cx.clone())?)?)?, &Py::Float(0.0))?), (Py::str("y"), py::add(&py::add(&v_cy.clone(), &py::mul(&v_k.clone(), &py::sub(&v_py.clone(), &v_cy.clone())?)?)?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ellipse_curvature(mut v_a: Py, mut v_b: Py, mut v_t: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let t851 = (Py::tuple(vec![f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?, f__fin(v_t.clone(), Py::str("t"))?])).items()?;
    if t851.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t851[0].clone();
    v_b = t851[1].clone();
    v_t = t851[2].clone();
    v_tr = py::math1("radians", &v_t.clone())?;
    v_q = py::add(&py::pow(&py::mul(&v_a.clone(), &py::math1("sin", &v_tr.clone())?)?, &Py::Int(2i64))?, &py::pow(&py::mul(&v_b.clone(), &py::math1("cos", &v_tr.clone())?)?, &Py::Int(2i64))?)?;
    v_k = py::truediv(&py::mul(&v_a.clone(), &v_b.clone())?, &py::pow(&v_q.clone(), &Py::Float(1.5))?)?;
    return Ok(Py::dict(vec![(Py::str("curvature"), v_k.clone()), (Py::str("radius_of_curvature"), py::truediv(&Py::Int(1i64), &v_k.clone())?), (Py::str("x"), py::add(&py::mul(&v_a.clone(), &py::math1("cos", &v_tr.clone())?)?, &Py::Float(0.0))?), (Py::str("y"), py::add(&py::mul(&v_b.clone(), &py::math1("sin", &v_tr.clone())?)?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ellipse_perimeter(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_cn: Py = Py::None;
    let mut v_gn: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t884 = (Py::tuple(vec![f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?])).items()?;
    if t884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t884[0].clone();
    v_b = t884[1].clone();
    let t885 = (Py::tuple(vec![py::minmax(&[v_a.clone(), v_b.clone()], true)?, py::minmax(&[v_a.clone(), v_b.clone()], false)?])).items()?;
    if t885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t885[0].clone();
    v_B = t885[1].clone();
    if ({ let t886 = py::truediv(&v_B.clone(), &v_A.clone())?; Py::Bool({ let t887 = Py::Float(0.001); if py::lt(&t886, &t887)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the axis ratio must be at least 0.001"))));
    }
    v_m = py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_B.clone(), &v_A.clone())?, &Py::Int(2i64))?)?;
    let t888 = (Py::tuple(vec![Py::Float(1.0), py::math1("sqrt", &py::sub(&Py::Int(1i64), &v_m.clone())?)?])).items()?;
    if t888.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_an = t888[0].clone();
    v_gn = t888[1].clone();
    let t889 = (Py::tuple(vec![py::truediv(&v_m.clone(), &Py::Int(2i64))?, Py::Float(0.5)])).items()?;
    if t889.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t889[0].clone();
    v_p = t889[1].clone();
    for t890 in py::range_iter(&[(Py::Int(40i64)).i()?]) {
        v__ = t890;
        v_cn = py::truediv(&py::sub(&v_an.clone(), &v_gn.clone())?, &Py::Int(2i64))?;
        let t891 = (Py::tuple(vec![py::truediv(&py::add(&v_an.clone(), &v_gn.clone())?, &Py::Int(2i64))?, py::math1("sqrt", &py::mul(&v_an.clone(), &v_gn.clone())?)?])).items()?;
        if t891.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_an = t891[0].clone();
        v_gn = t891[1].clone();
        v_p = py::mul(&v_p, &Py::Int(2i64))?;
        v_s = py::iadd(&v_s, &py::mul(&py::mul(&v_p.clone(), &v_cn.clone())?, &v_cn.clone())?)?;
        if ({ let t892 = v_cn.clone(); Py::Bool({ let t893 = Py::Float(1e-17); if py::lt(&t892, &t893)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_E = py::mul(&py::truediv(&Py::Float(std::f64::consts::PI), &py::mul(&Py::Int(2i64), &v_an.clone())?)?, &py::sub(&Py::Int(1i64), &v_s.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("perimeter"), py::mul(&py::mul(&Py::Int(4i64), &v_A.clone())?, &v_E.clone())?), (Py::str("area"), py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_a.clone())?, &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_farthest_pair(mut v_points: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(2i64), Py::Int(5000i64))?;
    v_h = (if ({ let t934 = py::len(&py::set(&v_pts.clone())?)?; Py::Bool({ let t935 = Py::Int(3i64); if py::ge(&t934, &t935)? { true } else { false } }) }).truthy() { f__hull(v_pts.clone())? } else { py::sorted(&py::set(&v_pts.clone())?, (Py::Bool(false)).truthy())? });
    v_best = { let t = { let mut out = vec![]; for t947 in py::iter(&(py::itertools("combinations", &[v_h.clone(), Py::Int(2i64)])?))? { let mut v_a: Py; let mut v_b: Py; 
let t948 = (t947).items()?;
if t948.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t948[0].clone();
v_b = t948[1].clone();
out.push(py::dist(&v_a.clone(), &v_b.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    return Ok(Py::dict(vec![(Py::str("distance"), v_best.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_frustum_measures(mut v_r1: Py, mut v_r2: Py, mut v_height: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t970 = (Py::tuple(vec![f__pos(v_r1.clone(), Py::str("r1"))?, f__pos(v_r2.clone(), Py::str("r2"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t970.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_R = t970[0].clone();
    v_r = t970[1].clone();
    v_h = t970[2].clone();
    v_s = py::hypot(&[v_h.clone(), py::sub(&v_R.clone(), &v_r.clone())?])?;
    return Ok(Py::dict(vec![(Py::str("volume"), py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_h.clone())?, &py::add(&py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_R.clone(), &v_r.clone())?)?, &py::mul(&v_r.clone(), &v_r.clone())?)?)?, &Py::Int(3i64))?), (Py::str("slant_height"), v_s.clone()), (Py::str("lateral_area"), py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::add(&v_R.clone(), &v_r.clone())?)?, &v_s.clone())?), (Py::str("total_area"), py::mul(&Py::Float(std::f64::consts::PI), &py::add(&py::add(&py::mul(&py::add(&v_R.clone(), &v_r.clone())?, &v_s.clone())?, &py::mul(&v_R.clone(), &v_R.clone())?)?, &py::mul(&v_r.clone(), &v_r.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_great_circle(mut v_lat1: Py, mut v_lon1: Py, mut v_lat2: Py, mut v_lon2: Py, mut v_radius: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    let t1008 = (Py::tuple(vec![f__latlon(v_lat1.clone(), v_lon1.clone(), Py::str("point 1"))?, f__latlon(v_lat2.clone(), v_lon2.clone(), Py::str("point 2"))?])).items()?;
    if t1008.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t1008[0].clone();
    v_p2 = t1008[1].clone();
    v_R = f__pos(v_radius.clone(), Py::str("radius"))?;
    v_c = f__central(v_p1.clone(), v_p2.clone())?;
    return Ok(Py::dict(vec![(Py::str("central_angle"), py::math1("degrees", &v_c.clone())?), (Py::str("distance"), py::mul(&v_R.clone(), &v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hausdorff_distance(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t1050 = (Py::tuple(vec![f__pairs(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(1000i64))?, f__pairs(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t1050.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1050[0].clone();
    v_B = t1050[1].clone();
    v_d = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_S: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_T: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[{ let mut out = vec![]; for t1051 in py::iter(&(v_S.clone()))? { let mut v_s: Py; 
v_s = t1051;
out.push(py::minmax(&[{ let mut out = vec![]; for t1052 in py::iter(&(v_T.clone()))? { let mut v_t: Py; 
v_t = t1052;
out.push(py::dist(&v_s.clone(), &v_t.clone())?); } Py::list(out) }], false)?); } Py::list(out) }], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("distance"), py::minmax(&[py::call(&v_d.clone(), &[v_A.clone(), v_B.clone()])?, py::call(&v_d.clone(), &[v_B.clone(), v_A.clone()])?], true)?), (Py::str("from_a"), py::call(&v_d.clone(), &[v_A.clone(), v_B.clone()])?), (Py::str("from_b"), py::call(&v_d.clone(), &[v_B.clone(), v_A.clone()])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_helix_measures(mut v_radius: Py, mut v_pitch: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1089 = (Py::tuple(vec![f__pos(v_radius.clone(), Py::str("radius"))?, f__fin(v_pitch.clone(), Py::str("pitch"))?])).items()?;
    if t1089.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t1089[0].clone();
    v_p = t1089[1].clone();
    v_c = py::truediv(&v_p.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
    v_den = py::add(&py::mul(&v_r.clone(), &v_r.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("curvature"), py::truediv(&v_r.clone(), &v_den.clone())?), (Py::str("torsion"), py::truediv(&v_c.clone(), &v_den.clone())?), (Py::str("length_per_turn"), py::hypot(&[py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?, v_p.clone()])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_initial_bearing(mut v_lat1: Py, mut v_lon1: Py, mut v_lat2: Py, mut v_lon2: Py) -> R {
    let mut v_dl: Py = Py::None;
    let mut v_f1: Py = Py::None;
    let mut v_f2: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t1127 = (Py::tuple(vec![f__latlon(v_lat1.clone(), v_lon1.clone(), Py::str("point 1"))?, f__latlon(v_lat2.clone(), v_lon2.clone(), Py::str("point 2"))?])).items()?;
    if t1127.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t1128 = (t1127[0].clone()).items()?;
    if t1128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f1 = t1128[0].clone();
    v_l1 = t1128[1].clone();
    let t1129 = (t1127[1].clone()).items()?;
    if t1129.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f2 = t1129[0].clone();
    v_l2 = t1129[1].clone();
    if ({ let t1130 = py::abs(&v_lat1.clone())?; Py::Bool({ let t1131 = Py::Int(90i64); if py::eq(&t1130, &t1131) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the start is at a pole: every direction is south (or north); the bearing is undefined"))));
    }
    if ({ let t1139 = { let t1132 = f__central(Py::tuple(vec![v_f1.clone(), v_l1.clone()]), Py::tuple(vec![v_f2.clone(), v_l2.clone()]))?; Py::Bool({ let t1133 = Py::Float(1e-12); if py::lt(&t1132, &t1133)? { true } else { false } }) }; if t1139.truthy() { t1139 } else { { let t1137 = f__central(Py::tuple(vec![v_f1.clone(), v_l1.clone()]), Py::tuple(vec![v_f2.clone(), v_l2.clone()]))?; Py::Bool({ let t1138 = py::sub(&Py::Float(std::f64::consts::PI), &Py::Float(1e-09))?; if py::gt(&t1137, &t1138)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the points coincide or are antipodal: the initial bearing is not unique"))));
    }
    v_dl = py::sub(&v_l2.clone(), &v_l1.clone())?;
    v_t = py::atan2(&py::mul(&py::math1("sin", &v_dl.clone())?, &py::math1("cos", &v_f2.clone())?)?, &py::sub(&py::mul(&py::math1("cos", &v_f1.clone())?, &py::math1("sin", &v_f2.clone())?)?, &py::mul(&py::mul(&py::math1("sin", &v_f1.clone())?, &py::math1("cos", &v_f2.clone())?)?, &py::math1("cos", &v_dl.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("bearing"), py::modulo(&py::add(&py::math1("degrees", &v_t.clone())?, &Py::Float(360.0))?, &Py::Float(360.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_in_space(mut v_point: Py, mut v_second_point: Py, mut v_direction: Py, mut v_t: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_par: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sym: Py = Py::None;
    let mut v_vec: Py = Py::None;
    if ({ let t1160 = py::len(&v_point.clone())?; Py::Bool({ let t1161 = Py::Int(3i64); if py::ne(&t1160, &t1161) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a point in space has three coordinates"))));
    }
    v_P = Py::list(vec![f__fin(py::float(&py::getitem(&v_point.clone(), &Py::Int(0i64))?)?, Py::str("point"))?, f__fin(py::float(&py::getitem(&v_point.clone(), &Py::Int(1i64))?)?, Py::str("point"))?, f__fin(py::float(&py::getitem(&v_point.clone(), &Py::Int(2i64))?)?, Py::str("point"))?]);
    if ({ let t1164 = { let t1166 = v_second_point.clone(); Py::Bool({ let t1167 = Py::None; if py::is(&t1166, &t1167) { true } else { false } }) }; Py::Bool({ let t1165 = { let t1168 = v_direction.clone(); Py::Bool({ let t1169 = Py::None; if py::is(&t1168, &t1169) { true } else { false } }) }; if py::eq(&t1164, &t1165) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give either a second point or a direction vector (exactly one of them)"))));
    }
    if ({ let t1170 = v_second_point.clone(); Py::Bool({ let t1171 = Py::None; if !py::is(&t1170, &t1171) { true } else { false } }) }).truthy() {
        if ({ let t1172 = py::len(&v_second_point.clone())?; Py::Bool({ let t1173 = Py::Int(3i64); if py::ne(&t1172, &t1173) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a point in space has three coordinates"))));
        }
        v_D = { let mut out = vec![]; for t1174 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t1174;
out.push(py::add(&py::sub(&f__fin(py::float(&py::getitem(&v_second_point.clone(), &v_k.clone())?)?, Py::str("second_point"))?, &py::getitem(&v_P.clone(), &v_k.clone())?)?, &Py::Float(0.0))?); } Py::list(out) };
        if ({ let t1182 = { let t1175 = py::getitem(&v_D.clone(), &Py::Int(0i64))?; Py::Bool({ let t1176 = Py::Int(0i64); if py::eq(&t1175, &t1176) { true } else { false } }) }; if !t1182.truthy() { t1182 } else { { let t1181 = { let t1177 = py::getitem(&v_D.clone(), &Py::Int(1i64))?; Py::Bool({ let t1178 = Py::Int(0i64); if py::eq(&t1177, &t1178) { true } else { false } }) }; if !t1181.truthy() { t1181 } else { { let t1179 = py::getitem(&v_D.clone(), &Py::Int(2i64))?; Py::Bool({ let t1180 = Py::Int(0i64); if py::eq(&t1179, &t1180) { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the two points coincide: they do not determine a line"))));
        }
    } else {
        if ({ let t1183 = py::len(&v_direction.clone())?; Py::Bool({ let t1184 = Py::Int(3i64); if py::ne(&t1183, &t1184) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a direction in space has three components"))));
        }
        v_D = { let mut out = vec![]; for t1185 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t1185;
out.push(py::add(&f__fin(py::float(&py::getitem(&v_direction.clone(), &v_k.clone())?)?, Py::str("direction"))?, &Py::Float(0.0))?); } Py::list(out) };
        if ({ let t1193 = { let t1186 = py::getitem(&v_D.clone(), &Py::Int(0i64))?; Py::Bool({ let t1187 = Py::Int(0i64); if py::eq(&t1186, &t1187) { true } else { false } }) }; if !t1193.truthy() { t1193 } else { { let t1192 = { let t1188 = py::getitem(&v_D.clone(), &Py::Int(1i64))?; Py::Bool({ let t1189 = Py::Int(0i64); if py::eq(&t1188, &t1189) { true } else { false } }) }; if !t1192.truthy() { t1192 } else { { let t1190 = py::getitem(&v_D.clone(), &Py::Int(2i64))?; Py::Bool({ let t1191 = Py::Int(0i64); if py::eq(&t1190, &t1191) { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the direction vector must not be zero"))));
        }
    }
    v_P = Py::list(vec![f__r12(py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, f__r12(py::getitem(&v_P.clone(), &Py::Int(1i64))?)?, f__r12(py::getitem(&v_P.clone(), &Py::Int(2i64))?)?]);
    v_D = Py::list(vec![f__r12(py::getitem(&v_D.clone(), &Py::Int(0i64))?)?, f__r12(py::getitem(&v_D.clone(), &Py::Int(1i64))?)?, f__r12(py::getitem(&v_D.clone(), &Py::Int(2i64))?)?]);
    v_n = py::math1("sqrt", &py::add(&py::add(&py::mul(&py::getitem(&v_D.clone(), &Py::Int(0i64))?, &py::getitem(&v_D.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_D.clone(), &Py::Int(1i64))?, &py::getitem(&v_D.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_D.clone(), &Py::Int(2i64))?, &py::getitem(&v_D.clone(), &Py::Int(2i64))?)?)?)?;
    let t1218 = (f__l3_texts(v_P.clone(), v_D.clone())?).items()?;
    if t1218.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_vec = t1218[0].clone();
    v_par = t1218[1].clone();
    v_sym = t1218[2].clone();
    v_out = Py::dict(vec![(Py::str("vector_form"), v_vec.clone()), (Py::str("parametric"), v_par.clone()), (Py::str("symmetric"), v_sym.clone()), (Py::str("x0"), py::getitem(&v_P.clone(), &Py::Int(0i64))?), (Py::str("y0"), py::getitem(&v_P.clone(), &Py::Int(1i64))?), (Py::str("z0"), py::getitem(&v_P.clone(), &Py::Int(2i64))?), (Py::str("dx"), py::getitem(&v_D.clone(), &Py::Int(0i64))?), (Py::str("dy"), py::getitem(&v_D.clone(), &Py::Int(1i64))?), (Py::str("dz"), py::getitem(&v_D.clone(), &Py::Int(2i64))?), (Py::str("ux"), py::add(&py::truediv(&py::getitem(&v_D.clone(), &Py::Int(0i64))?, &v_n.clone())?, &Py::Float(0.0))?), (Py::str("uy"), py::add(&py::truediv(&py::getitem(&v_D.clone(), &Py::Int(1i64))?, &v_n.clone())?, &Py::Float(0.0))?), (Py::str("uz"), py::add(&py::truediv(&py::getitem(&v_D.clone(), &Py::Int(2i64))?, &v_n.clone())?, &Py::Float(0.0))?), (Py::str("xt"), Py::None), (Py::str("yt"), Py::None), (Py::str("zt"), Py::None)]);
    if ({ let t1219 = v_t.clone(); Py::Bool({ let t1220 = Py::None; if !py::is(&t1219, &t1220) { true } else { false } }) }).truthy() {
        v_s = f__fin(v_t.clone(), Py::str("t"))?;
        py::setitem(&v_out.clone(), &Py::str("xt"), py::add(&py::add(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::mul(&v_s.clone(), &py::getitem(&v_D.clone(), &Py::Int(0i64))?)?)?, &Py::Float(0.0))?)?;
        py::setitem(&v_out.clone(), &Py::str("yt"), py::add(&py::add(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &py::mul(&v_s.clone(), &py::getitem(&v_D.clone(), &Py::Int(1i64))?)?)?, &Py::Float(0.0))?)?;
        py::setitem(&v_out.clone(), &Py::str("zt"), py::add(&py::add(&py::getitem(&v_P.clone(), &Py::Int(2i64))?, &py::mul(&v_s.clone(), &py::getitem(&v_D.clone(), &Py::Int(2i64))?)?)?, &Py::Float(0.0))?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_intersection(mut v_a1: Py, mut v_b1: Py, mut v_c1: Py, mut v_a2: Py, mut v_b2: Py, mut v_c2: Py) -> R {
    let mut v_det: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t1269 = (f__line_check(v_a1.clone(), v_b1.clone(), v_c1.clone(), Py::str("line 1"))?).items()?;
    if t1269.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t1269[0].clone();
    v_b1 = t1269[1].clone();
    v_c1 = t1269[2].clone();
    let t1270 = (f__line_check(v_a2.clone(), v_b2.clone(), v_c2.clone(), Py::str("line 2"))?).items()?;
    if t1270.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t1270[0].clone();
    v_b2 = t1270[1].clone();
    v_c2 = t1270[2].clone();
    if ({ let t1271 = py::sub(&py::mul(&f__fr(v_a1.clone())?, &f__fr(v_b2.clone())?)?, &py::mul(&f__fr(v_a2.clone())?, &f__fr(v_b1.clone())?)?)?; Py::Bool({ let t1272 = Py::Int(0i64); if py::eq(&t1271, &t1272) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the lines are parallel (or the same line): there is no single intersection point"))));
    }
    v_det = py::sub(&py::mul(&v_a1.clone(), &v_b2.clone())?, &py::mul(&v_a2.clone(), &v_b1.clone())?)?;
    v_s = py::minmax(&[py::abs(&v_c1.clone())?, py::abs(&v_c2.clone())?, Py::Float(1.0)], true)?;
    return Ok(Py::dict(vec![(Py::str("x"), f__snap(py::truediv(&py::sub(&py::mul(&v_b1.clone(), &v_c2.clone())?, &py::mul(&v_b2.clone(), &v_c1.clone())?)?, &v_det.clone())?, Py::Float(0.0))?), (Py::str("y"), f__snap(py::truediv(&py::sub(&py::mul(&v_c1.clone(), &v_a2.clone())?, &py::mul(&v_c2.clone(), &v_a1.clone())?)?, &v_det.clone())?, Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_plane_intersection(mut v_x0: Py, mut v_y0: Py, mut v_z0: Py, mut v_dx: Py, mut v_dy: Py, mut v_dz: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_P = Py::tuple(({ let mut out = vec![]; for t1291 in py::iter(&(Py::tuple(vec![v_x0.clone(), v_y0.clone(), v_z0.clone()])))? { let mut v_v: Py; 
v_v = t1291;
out.push(f__fin(v_v.clone(), Py::str("point"))?); } Py::list(out) }).items()?);
    v_D = Py::tuple(({ let mut out = vec![]; for t1294 in py::iter(&(Py::tuple(vec![v_dx.clone(), v_dy.clone(), v_dz.clone()])))? { let mut v_v: Py; 
v_v = t1294;
out.push(f__fin(v_v.clone(), Py::str("direction"))?); } Py::list(out) }).items()?);
    let t1305 = (f__plane_check(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone(), Py::str("plane"))?).items()?;
    if t1305.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1305[0].clone();
    v_b = t1305[1].clone();
    v_c = t1305[2].clone();
    v_d = t1305[3].clone();
    if ({ let t1306 = v_D.clone(); Py::Bool({ let t1307 = Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]); if py::eq(&t1306, &t1307) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the direction vector must not be zero"))));
    }
    v_den = py::add(&py::add(&py::mul(&f__fr(v_a.clone())?, &f__fr(v_dx.clone())?)?, &py::mul(&f__fr(v_b.clone())?, &f__fr(v_dy.clone())?)?)?, &py::mul(&f__fr(v_c.clone())?, &f__fr(v_dz.clone())?)?)?;
    if ({ let t1308 = v_den.clone(); Py::Bool({ let t1309 = Py::Int(0i64); if py::eq(&t1308, &t1309) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the line is parallel to the plane (or lies in it): no single intersection point"))));
    }
    v_t = py::truediv(&py::add(&py::add(&py::add(&py::mul(&f__fr(v_a.clone())?, &f__fr(v_x0.clone())?)?, &py::mul(&f__fr(v_b.clone())?, &f__fr(v_y0.clone())?)?)?, &py::mul(&f__fr(v_c.clone())?, &f__fr(v_z0.clone())?)?)?, &f__fr(v_d.clone())?)?, &py::neg(&v_den.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("x"), py::add(&py::float(&py::add(&f__fr(v_x0.clone())?, &py::mul(&v_t.clone(), &f__fr(v_dx.clone())?)?)?)?, &Py::Float(0.0))?), (Py::str("y"), py::add(&py::float(&py::add(&f__fr(v_y0.clone())?, &py::mul(&v_t.clone(), &f__fr(v_dy.clone())?)?)?)?, &Py::Float(0.0))?), (Py::str("z"), py::add(&py::float(&py::add(&f__fr(v_z0.clone())?, &py::mul(&v_t.clone(), &f__fr(v_dz.clone())?)?)?)?, &Py::Float(0.0))?), (Py::str("t"), py::add(&py::float(&v_t.clone())?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_min_area_rectangle(mut v_points: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_us: Py = Py::None;
    let mut v_ux: Py = Py::None;
    let mut v_uy: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::Int(5000i64))?;
    v_h = f__hull(v_pts.clone())?;
    if ({ let t1351 = py::len(&v_h.clone())?; Py::Bool({ let t1352 = Py::Int(3i64); if py::lt(&t1351, &t1352)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the points are collinear: the enclosing rectangle has no area"))));
    }
    v_best = Py::None;
    v_n = py::len(&v_h.clone())?;
    for t1353 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1353;
        let t1354 = (Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_h.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_h.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(1i64))?)?])).items()?;
        if t1354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ex = t1354[0].clone();
        v_ey = t1354[1].clone();
        v_L = py::hypot(&[v_ex.clone(), v_ey.clone()])?;
        let t1355 = (Py::tuple(vec![py::truediv(&v_ex.clone(), &v_L.clone())?, py::truediv(&v_ey.clone(), &v_L.clone())?])).items()?;
        if t1355.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ux = t1355[0].clone();
        v_uy = t1355[1].clone();
        v_us = { let mut out = vec![]; for t1356 in py::iter(&(v_h.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1357 = (t1356).items()?;
if t1357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1357[0].clone();
v_y = t1357[1].clone();
out.push(py::add(&py::mul(&v_x.clone(), &v_ux.clone())?, &py::mul(&v_y.clone(), &v_uy.clone())?)?); } Py::list(out) };
        v_vs = { let mut out = vec![]; for t1358 in py::iter(&(v_h.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1359 = (t1358).items()?;
if t1359.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1359[0].clone();
v_y = t1359[1].clone();
out.push(py::add(&py::mul(&py::neg(&v_x.clone())?, &v_uy.clone())?, &py::mul(&v_y.clone(), &v_ux.clone())?)?); } Py::list(out) };
        let t1360 = (Py::tuple(vec![py::sub(&py::minmax(&[v_us.clone()], true)?, &py::minmax(&[v_us.clone()], false)?)?, py::sub(&py::minmax(&[v_vs.clone()], true)?, &py::minmax(&[v_vs.clone()], false)?)?])).items()?;
        if t1360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_w = t1360[0].clone();
        v_t = t1360[1].clone();
        if ({ let t1365 = { let t1361 = v_best.clone(); Py::Bool({ let t1362 = Py::None; if py::is(&t1361, &t1362) { true } else { false } }) }; if t1365.truthy() { t1365 } else { { let t1363 = py::mul(&v_w.clone(), &v_t.clone())?; Py::Bool({ let t1364 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t1363, &t1364)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![py::mul(&v_w.clone(), &v_t.clone())?, py::minmax(&[v_w.clone(), v_t.clone()], false)?, py::minmax(&[v_w.clone(), v_t.clone()], true)?]);
        }
    }
    return Ok(Py::dict(vec![(Py::str("area"), py::getitem(&v_best.clone(), &Py::Int(0i64))?), (Py::str("width"), py::getitem(&v_best.clone(), &Py::Int(1i64))?), (Py::str("length"), py::getitem(&v_best.clone(), &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_min_enclosing_circle(mut v_points: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cc: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_pts = Py::list((py::dict_fromkeys(&f__pairs(v_points.clone(), Py::str("points"), Py::Int(1i64), Py::Int(2000i64))?, Py::None)?).items()?);
    let _ = { py::random_shuffle(&Py::Int(20261022i64), &v_pts.clone())?; Py::None };
    v_c = Py::None;
    for t1410 in py::iter(&(py::enumerate(&v_pts.clone(), 0)?))? {
        let t1424 = (t1410).items()?;
        if t1424.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t1424[0].clone();
        v_p = t1424[1].clone();
        if ({ let t1415 = { let t1411 = v_c.clone(); Py::Bool({ let t1412 = Py::None; if !py::is(&t1411, &t1412) { true } else { false } }) }; if !t1415.truthy() { t1415 } else { f__in_circ(v_c.clone(), v_p.clone())? } }).truthy() {
            continue;
        }
        v_c = Py::tuple(vec![py::getitem(&v_p.clone(), &Py::Int(0i64))?, py::getitem(&v_p.clone(), &Py::Int(1i64))?, Py::Float(0.0)]);
        for t1416 in py::range_iter(&[(v_i.clone()).i()?]) {
            v_j = t1416;
            v_q = py::getitem(&v_pts.clone(), &v_j.clone())?;
            if (f__in_circ(v_c.clone(), v_q.clone())?).truthy() {
                continue;
            }
            v_c = Py::tuple(vec![py::truediv(&py::add(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_q.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, py::truediv(&py::add(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_q.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?, py::truediv(&py::dist(&v_p.clone(), &v_q.clone())?, &Py::Int(2i64))?]);
            for t1417 in py::range_iter(&[(v_j.clone()).i()?]) {
                v_k = t1417;
                v_r = py::getitem(&v_pts.clone(), &v_k.clone())?;
                if (f__in_circ(v_c.clone(), v_r.clone())?).truthy() {
                    continue;
                }
                v_cc = f__circ3(v_p.clone(), v_q.clone(), v_r.clone())?;
                if ({ let t1421 = v_cc.clone(); Py::Bool({ let t1422 = Py::None; if py::is(&t1421, &t1422) { true } else { false } }) }).truthy() {
                    let t1423 = (py::minmax_key(&[Py::tuple(vec![Py::tuple(vec![v_p.clone(), v_q.clone()]), Py::tuple(vec![v_p.clone(), v_r.clone()]), Py::tuple(vec![v_q.clone(), v_r.clone()])])], &{ let v_c = v_c.clone(); let v_cc = v_cc.clone(); let v_i = v_i.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_p = v_p.clone(); let v_points = v_points.clone(); let v_pts = v_pts.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let a: &[Py] = &({ let mut v: Vec<Py> = vec![]; v.extend((v_e.clone()).items()?); v })[..]; if a.len() != 2 { return Err(PyErr::Type("dist expected 2 arguments".into())); } py::dist(&a[0], &a[1])? });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?).items()?;
                    if t1423.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_a = t1423[0].clone();
                    v_b = t1423[1].clone();
                    v_cc = Py::tuple(vec![py::truediv(&py::add(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, py::truediv(&py::add(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?, py::truediv(&py::dist(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?]);
                }
                v_c = v_cc.clone();
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("cx"), py::add(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &Py::Float(0.0))?), (Py::str("cy"), py::add(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &Py::Float(0.0))?), (Py::str("radius"), py::getitem(&v_c.clone(), &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nearest_neighbor(mut v_px: Py, mut v_py: Py, mut v_points: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_P = Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?]);
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(1i64), Py::Int(100000i64))?;
    v_best = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_pts.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_P = v_P.clone(); let v_points = v_points.clone(); let v_pts = v_pts.clone(); let v_px = v_px.clone(); let v_py = v_py.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::add(&py::pow(&py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?, &py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::getitem(&v_P.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?, v_k.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
    return Ok(Py::dict(vec![(Py::str("index"), py::add(&v_best.clone(), &Py::Int(1i64))?), (Py::str("distance"), py::dist(&v_P.clone(), &py::getitem(&v_pts.clone(), &v_best.clone())?)?), (Py::str("nx"), py::getitem(&py::getitem(&v_pts.clone(), &v_best.clone())?, &Py::Int(0i64))?), (Py::str("ny"), py::getitem(&py::getitem(&v_pts.clone(), &v_best.clone())?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parabola_arc_length(mut v_a: Py, mut v_b: Py, mut v_x0: Py, mut v_x1: Py, mut v_c: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_u0: Py = Py::None;
    let mut v_u1: Py = Py::None;
    let t1506 = (Py::tuple(vec![f__fin(v_a.clone(), Py::str("a"))?, f__fin(v_b.clone(), Py::str("b"))?, f__fin(v_x0.clone(), Py::str("x0"))?, f__fin(v_x1.clone(), Py::str("x1"))?, f__fin(v_c.clone(), Py::str("c"))?])).items()?;
    if t1506.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1506[0].clone();
    v_b = t1506[1].clone();
    v_x0 = t1506[2].clone();
    v_x1 = t1506[3].clone();
    v_c = t1506[4].clone();
    if ({ let t1507 = v_x0.clone(); Py::Bool({ let t1508 = v_x1.clone(); if py::eq(&t1507, &t1508) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval is empty (x0 = x1)"))));
    }
    if ({ let t1509 = v_a.clone(); Py::Bool({ let t1510 = Py::Int(0i64); if py::eq(&t1509, &t1510) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("length"), py::mul(&py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_b.clone(), &v_b.clone())?)?)?, &py::abs(&py::sub(&v_x1.clone(), &v_x0.clone())?)?)?)]));
    }
    v_F = { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_x0 = v_x0.clone(); let v_x1 = v_x1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::add(&py::mul(&v_u.clone(), &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_u.clone(), &v_u.clone())?)?)?)?, &py::math1("asinh", &v_u.clone())?)?, &Py::Int(2i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t1511 = (Py::tuple(vec![py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_x0.clone())?, &v_b.clone())?, py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_x1.clone())?, &v_b.clone())?])).items()?;
    if t1511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u0 = t1511[0].clone();
    v_u1 = t1511[1].clone();
    return Ok(Py::dict(vec![(Py::str("length"), py::abs(&py::truediv(&py::sub(&py::call(&v_F.clone(), &[v_u1.clone()])?, &py::call(&v_F.clone(), &[v_u0.clone()])?)?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pick_counts(mut v_points: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_twice: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::None)?;
    if (Py::Bool(({ let mut out = vec![]; for t1555 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1555;
for t1556 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t1556;
out.push({ let t1557 = v_v.clone(); Py::Bool({ let t1558 = py::int(&v_v.clone())?; if py::ne(&t1557, &t1558) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Pick's theorem needs integer (lattice) coordinates"))));
    }
    v_pts = { let mut out = vec![]; for t1559 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1560 = (t1559).items()?;
if t1560.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1560[0].clone();
v_y = t1560[1].clone();
out.push(Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])); } Py::list(out) };
    let _ = f__simple_polygon(v_pts.clone(), Py::str("points"))?;
    v_n = py::len(&v_pts.clone())?;
    v_twice = py::abs(&py::sum(&{ let mut out = vec![]; for t1632 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1632;
out.push(py::sub(&py::mul(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?)?;
    v_b = py::sum(&{ let mut out = vec![]; for t1633 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1633;
out.push(py::gcd(&py::abs(&py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?)?)?, &py::abs(&py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?)?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("area"), py::truediv(&v_twice.clone(), &Py::Int(2i64))?), (Py::str("boundary_points"), v_b.clone()), (Py::str("interior_points"), py::floordiv(&py::add(&py::sub(&v_twice.clone(), &v_b.clone())?, &Py::Int(2i64))?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plane_through_points(mut v_points: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nn: Py = Py::None;
    v_P = f__triples(v_points.clone(), Py::str("points"), Py::Int(3i64))?;
    v_X = { let mut out = vec![]; for t1681 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t1681;
out.push(f__xvec(v_p.clone())?); } Py::list(out) };
    v_n = f__vcross(f__vsub(py::getitem(&v_X.clone(), &Py::Int(1i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?, f__vsub(py::getitem(&v_X.clone(), &Py::Int(2i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?)?;
    if ({ let t1683 = v_n.clone(); Py::Bool({ let t1684 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t1683, &t1684) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the three points are collinear: they do not determine a plane"))));
    }
    if ({ let t1685 = (match ('g1687: { let mut out = vec![]; for t1688 in py::iter(&(v_n.clone()))? { let mut v_c: Py; 
v_c = t1688;
if ({ let t1689 = v_c.clone(); Py::Bool({ let t1690 = Py::Int(0i64); if py::ne(&t1689, &t1690) { true } else { false } }) }).truthy() { out.push(v_c.clone()); break 'g1687 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) }); Py::Bool({ let t1686 = Py::Int(0i64); if py::lt(&t1685, &t1686)? { true } else { false } }) }).truthy() {
        v_n = Py::tuple(({ let mut out = vec![]; for t1691 in py::iter(&(v_n.clone()))? { let mut v_c: Py; 
v_c = t1691;
out.push(py::neg(&v_c.clone())?); } Py::list(out) }).items()?);
    }
    v_nn = py::math1("sqrt", &py::float(&py::add(&py::add(&py::pow(&py::getitem(&v_n.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_n.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?)?)?;
    v_d = py::neg(&py::add(&py::add(&py::mul(&py::getitem(&v_n.clone(), &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_X.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_n.clone(), &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_X.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_n.clone(), &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_X.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("a"), py::add(&py::truediv(&py::float(&py::getitem(&v_n.clone(), &Py::Int(0i64))?)?, &v_nn.clone())?, &Py::Float(0.0))?), (Py::str("b"), py::add(&py::truediv(&py::float(&py::getitem(&v_n.clone(), &Py::Int(1i64))?)?, &v_nn.clone())?, &Py::Float(0.0))?), (Py::str("c"), py::add(&py::truediv(&py::float(&py::getitem(&v_n.clone(), &Py::Int(2i64))?)?, &v_nn.clone())?, &Py::Float(0.0))?), (Py::str("d"), py::add(&py::truediv(&py::float(&v_d.clone())?, &v_nn.clone())?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_platonic_solid(mut v_solid: Py, mut v_edge: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_kR: Py = Py::None;
    let mut v_kr: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_kv: Py = Py::None;
    if ({ let t1716 = v_solid.clone(); Py::Bool({ let t1717 = c__PLATONIC()?; if !py::contains(&t1717, &t1716)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("solid must be one of "), &py::method(&Py::str(", "), "join", &[c__PLATONIC()?])?)?)));
    }
    v_a = f__pos(v_edge.clone(), Py::str("edge"))?;
    let t1722 = (py::getitem(&c__PLATONIC()?, &v_solid.clone())?).items()?;
    if t1722.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1722[0].clone();
    v_E = t1722[1].clone();
    v_F = t1722[2].clone();
    v_kv = t1722[3].clone();
    v_ks = t1722[4].clone();
    v_kR = t1722[5].clone();
    v_kr = t1722[6].clone();
    return Ok(Py::dict(vec![(Py::str("vertices"), v_V.clone()), (Py::str("edges"), v_E.clone()), (Py::str("faces"), v_F.clone()), (Py::str("euler_characteristic"), py::add(&py::sub(&v_V.clone(), &v_E.clone())?, &v_F.clone())?), (Py::str("volume"), py::mul(&v_kv.clone(), &py::pow(&v_a.clone(), &Py::Int(3i64))?)?), (Py::str("surface_area"), py::mul(&py::mul(&v_ks.clone(), &v_a.clone())?, &v_a.clone())?), (Py::str("circumradius"), py::mul(&v_kR.clone(), &v_a.clone())?), (Py::str("inradius"), py::mul(&v_kr.clone(), &v_a.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_point_in_polygon(mut v_px: Py, mut v_py: Py, mut v_points: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_ax: Py = Py::None;
    let mut v_ay: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_by: Py = Py::None;
    let mut v_inside: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_xi: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::None)?;
    let _ = f__simple_polygon(v_pts.clone(), Py::str("points"))?;
    v_P = Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?]);
    v_n = py::len(&v_pts.clone())?;
    for t1835 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1835;
        if (f__on_segment_exact(v_P.clone(), py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?)?).truthy() {
            return Ok(Py::dict(vec![(Py::str("location"), Py::str("boundary"))]));
        }
    }
    let t1836 = (Py::tuple(vec![f__fr(py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, f__fr(py::getitem(&v_P.clone(), &Py::Int(1i64))?)?])).items()?;
    if t1836.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t1836[0].clone();
    v_Y = t1836[1].clone();
    v_inside = Py::Bool(false);
    for t1837 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1837;
        let t1839 = ({ let mut out = vec![]; for t1838 in py::iter(&(Py::tuple(vec![py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])))? { let mut v_c: Py; 
v_c = t1838;
out.push(Py::tuple(vec![f__fr(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, f__fr(py::getitem(&v_c.clone(), &Py::Int(1i64))?)?])); } Py::list(out) }).items()?;
        if t1839.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t1840 = (t1839[0].clone()).items()?;
        if t1840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ax = t1840[0].clone();
        v_ay = t1840[1].clone();
        let t1841 = (t1839[1].clone()).items()?;
        if t1841.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_bx = t1841[0].clone();
        v_by = t1841[1].clone();
        if ({ let t1842 = { let t1844 = v_ay.clone(); Py::Bool({ let t1845 = v_Y.clone(); if py::gt(&t1844, &t1845)? { true } else { false } }) }; Py::Bool({ let t1843 = { let t1846 = v_by.clone(); Py::Bool({ let t1847 = v_Y.clone(); if py::gt(&t1846, &t1847)? { true } else { false } }) }; if py::ne(&t1842, &t1843) { true } else { false } }) }).truthy() {
            v_xi = py::add(&v_ax.clone(), &py::truediv(&py::mul(&py::sub(&v_Y.clone(), &v_ay.clone())?, &py::sub(&v_bx.clone(), &v_ax.clone())?)?, &py::sub(&v_by.clone(), &v_ay.clone())?)?)?;
            if ({ let t1848 = v_X.clone(); Py::Bool({ let t1849 = v_xi.clone(); if py::lt(&t1848, &t1849)? { true } else { false } }) }).truthy() {
                v_inside = py::not(&v_inside.clone());
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("location"), (if (v_inside.clone()).truthy() { Py::str("inside") } else { Py::str("outside") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_point_plane_distance(mut v_px: Py, mut v_py: Py, mut v_pz: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v_n2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t1890 = (Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?, f__fin(v_pz.clone(), Py::str("pz"))?])).items()?;
    if t1890.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_px = t1890[0].clone();
    v_py = t1890[1].clone();
    v_pz = t1890[2].clone();
    let t1901 = (f__plane_check(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone(), Py::str("plane"))?).items()?;
    if t1901.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1901[0].clone();
    v_b = t1901[1].clone();
    v_c = t1901[2].clone();
    v_d = t1901[3].clone();
    v_n2 = py::add(&py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_c.clone(), &v_c.clone())?)?;
    v_s = py::add(&py::add(&py::add(&py::mul(&v_a.clone(), &v_px.clone())?, &py::mul(&v_b.clone(), &v_py.clone())?)?, &py::mul(&v_c.clone(), &v_pz.clone())?)?, &v_d.clone())?;
    return Ok(Py::dict(vec![(Py::str("distance"), py::truediv(&py::abs(&v_s.clone())?, &py::math1("sqrt", &v_n2.clone())?)?), (Py::str("fx"), py::add(&py::sub(&v_px.clone(), &py::truediv(&py::mul(&v_a.clone(), &v_s.clone())?, &v_n2.clone())?)?, &Py::Float(0.0))?), (Py::str("fy"), py::add(&py::sub(&v_py.clone(), &py::truediv(&py::mul(&v_b.clone(), &v_s.clone())?, &v_n2.clone())?)?, &Py::Float(0.0))?), (Py::str("fz"), py::add(&py::sub(&v_pz.clone(), &py::truediv(&py::mul(&v_c.clone(), &v_s.clone())?, &v_n2.clone())?)?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_point_segment_distance(mut v_px: Py, mut v_py: Py, mut v_x1: Py, mut v_y1: Py, mut v_x2: Py, mut v_y2: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_rx: Py = Py::None;
    let mut v_ry: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t1921 = (Py::tuple(vec![Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?]), Py::tuple(vec![f__fin(v_x1.clone(), Py::str("x1"))?, f__fin(v_y1.clone(), Py::str("y1"))?]), Py::tuple(vec![f__fin(v_x2.clone(), Py::str("x2"))?, f__fin(v_y2.clone(), Py::str("y2"))?])])).items()?;
    if t1921.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1921[0].clone();
    v_A = t1921[1].clone();
    v_B = t1921[2].clone();
    if ({ let t1922 = v_A.clone(); Py::Bool({ let t1923 = v_B.clone(); if py::eq(&t1922, &t1923) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the segment's endpoints coincide"))));
    }
    let t1924 = (Py::tuple(vec![py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?])).items()?;
    if t1924.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rx = t1924[0].clone();
    v_ry = t1924[1].clone();
    v_t = py::truediv(&py::add(&py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_rx.clone())?, &py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_ry.clone())?)?, &py::add(&py::mul(&v_rx.clone(), &v_rx.clone())?, &py::mul(&v_ry.clone(), &v_ry.clone())?)?)?;
    v_t = py::minmax(&[Py::Float(1.0), py::minmax(&[Py::Float(0.0), v_t.clone()], true)?], false)?;
    v_Q = (if ({ let t1925 = Py::Int(0i64); Py::Bool({ let t1926 = v_t.clone(); if py::lt(&t1925, &t1926)? { { let t1927 = Py::Int(1i64); if py::lt(&t1926, &t1927)? { true } else { false } } } else { false } }) }).truthy() { Py::tuple(vec![py::add(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &py::mul(&v_t.clone(), &v_rx.clone())?)?, py::add(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &py::mul(&v_t.clone(), &v_ry.clone())?)?]) } else { (if ({ let t1928 = v_t.clone(); Py::Bool({ let t1929 = Py::Int(0i64); if py::eq(&t1928, &t1929) { true } else { false } }) }).truthy() { v_A.clone() } else { v_B.clone() }) });
    return Ok(Py::dict(vec![(Py::str("distance"), py::dist(&v_P.clone(), &v_Q.clone())?), (Py::str("qx"), py::add(&py::getitem(&v_Q.clone(), &Py::Int(0i64))?, &Py::Float(0.0))?), (Py::str("qy"), py::add(&py::getitem(&v_Q.clone(), &Py::Int(1i64))?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polygon_angles(mut v_points: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_ang: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_sgn: Py = Py::None;
    let mut v_turn: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::None)?;
    let _ = f__simple_polygon(v_pts.clone(), Py::str("points"))?;
    v_n = py::len(&v_pts.clone())?;
    v_sgn = (if ({ let t2265 = f__shoelace(v_pts.clone())?; Py::Bool({ let t2266 = Py::Int(0i64); if py::gt(&t2265, &t2266)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    v_ang = Py::list(vec![]);
    for t2267 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2267;
        let t2268 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2268.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2268[0].clone();
        v_b = t2268[1].clone();
        v_c = t2268[2].clone();
        let t2269 = (Py::tuple(vec![Py::tuple(vec![py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?]), Py::tuple(vec![py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?])])).items()?;
        if t2269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2269[0].clone();
        v_v = t2269[1].clone();
        v_turn = py::math1("degrees", &py::atan2(&py::mul(&v_sgn.clone(), &py::sub(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?)?)?, &py::add(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?)?)?)?;
        let _ = py::method(&v_ang.clone(), "append", &[f__r12(py::sub(&Py::Float(180.0), &v_turn.clone())?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("angles"), v_ang.clone()), (Py::str("angle_sum"), py::float(&py::mul(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(180i64))?)?), (Py::str("reflex_vertices"), py::sum(&{ let mut out = vec![]; for t2271 in py::iter(&(v_ang.clone()))? { let mut v_x: Py; 
v_x = t2271;
if ({ let t2272 = v_x.clone(); Py::Bool({ let t2273 = Py::Int(180i64); if py::gt(&t2272, &t2273)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polygon_area(mut v_points: Py) -> R {
    let mut v_pts: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::None)?;
    let _ = f__simple_polygon(v_pts.clone(), Py::str("points"))?;
    v_s = f__shoelace(v_pts.clone())?;
    return Ok(Py::dict(vec![(Py::str("area"), py::abs(&v_s.clone())?), (Py::str("perimeter"), f__perimeter(v_pts.clone(), Py::Bool(true))?), (Py::str("orientation"), (if ({ let t2037 = v_s.clone(); Py::Bool({ let t2038 = Py::Int(0i64); if py::gt(&t2037, &t2038)? { true } else { false } }) }).truthy() { Py::str("counterclockwise") } else { Py::str("clockwise") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polygon_centroid(mut v_points: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_xa: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let mut v_y0: Py = Py::None;
    let mut v_ya: Py = Py::None;
    let mut v_yb: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::None)?;
    let _ = f__simple_polygon(v_pts.clone(), Py::str("points"))?;
    let t2153 = (Py::tuple(vec![py::len(&v_pts.clone())?, Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t2153.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2153[0].clone();
    v_a = t2153[1].clone();
    v_cx = t2153[2].clone();
    v_cy = t2153[3].clone();
    let t2154 = (py::getitem(&v_pts.clone(), &Py::Int(0i64))?).items()?;
    if t2154.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t2154[0].clone();
    v_y0 = t2154[1].clone();
    v_q = { let mut out = vec![]; for t2155 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t2156 = (t2155).items()?;
if t2156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2156[0].clone();
v_y = t2156[1].clone();
out.push(Py::tuple(vec![py::sub(&v_x.clone(), &v_x0.clone())?, py::sub(&v_y.clone(), &v_y0.clone())?])); } Py::list(out) };
    for t2157 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2157;
        let t2158 = (Py::tuple(vec![py::getitem(&v_q.clone(), &v_k.clone())?, py::getitem(&v_q.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t2159 = (t2158[0].clone()).items()?;
        if t2159.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xa = t2159[0].clone();
        v_ya = t2159[1].clone();
        let t2160 = (t2158[1].clone()).items()?;
        if t2160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xb = t2160[0].clone();
        v_yb = t2160[1].clone();
        v_w = py::sub(&py::mul(&v_xa.clone(), &v_yb.clone())?, &py::mul(&v_xb.clone(), &v_ya.clone())?)?;
        v_a = py::iadd(&v_a, &v_w.clone())?;
        v_cx = py::iadd(&v_cx, &py::mul(&py::add(&v_xa.clone(), &v_xb.clone())?, &v_w.clone())?)?;
        v_cy = py::iadd(&v_cy, &py::mul(&py::add(&v_ya.clone(), &v_yb.clone())?, &v_w.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("cx"), py::add(&py::truediv(&v_cx.clone(), &py::mul(&Py::Int(3i64), &v_a.clone())?)?, &v_x0.clone())?), (Py::str("cy"), py::add(&py::truediv(&v_cy.clone(), &py::mul(&Py::Int(3i64), &v_a.clone())?)?, &v_y0.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polygon_is_convex(mut v_points: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_signs: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::None)?;
    let _ = f__simple_polygon(v_pts.clone(), Py::str("points"))?;
    v_n = py::len(&v_pts.clone())?;
    v_signs = py::set(&{ let mut out = vec![]; for t2397 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2397;
out.push(py::sub(&{ let t2398 = f__xcross(py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(2i64))?, &v_n.clone())?)?)?; Py::Bool({ let t2399 = Py::Int(0i64); if py::gt(&t2398, &t2399)? { true } else { false } }) }, &{ let t2400 = f__xcross(py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(2i64))?, &v_n.clone())?)?)?; Py::Bool({ let t2401 = Py::Int(0i64); if py::lt(&t2400, &t2401)? { true } else { false } }) })?); } Py::list(out) })?;
    let _ = py::method(&v_signs.clone(), "discard", &[Py::Int(0i64)])?;
    return Ok(Py::dict(vec![(Py::str("convex"), (if ({ let t2402 = py::len(&v_signs.clone())?; Py::Bool({ let t2403 = Py::Int(1i64); if py::le(&t2402, &t2403)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polyline_length(mut v_points: Py) -> R {
    let mut v_pts: Py = Py::None;
    v_pts = f__pairs(v_points.clone(), Py::str("points"), Py::Int(2i64), Py::None)?;
    return Ok(Py::dict(vec![(Py::str("length"), f__perimeter(v_pts.clone(), Py::Bool(false))?), (Py::str("segments"), py::sub(&py::len(&v_pts.clone())?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pyramid_measures(mut v_length: Py, mut v_width: Py, mut v_height: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_lat: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2479 = (Py::tuple(vec![f__pos(v_length.clone(), Py::str("length"))?, f__pos(v_width.clone(), Py::str("width"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t2479.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_l = t2479[0].clone();
    v_w = t2479[1].clone();
    v_h = t2479[2].clone();
    v_lat = py::add(&py::mul(&v_l.clone(), &py::hypot(&[v_h.clone(), py::truediv(&v_w.clone(), &Py::Int(2i64))?])?)?, &py::mul(&v_w.clone(), &py::hypot(&[v_h.clone(), py::truediv(&v_l.clone(), &Py::Int(2i64))?])?)?)?;
    return Ok(Py::dict(vec![(Py::str("volume"), py::truediv(&py::mul(&py::mul(&v_l.clone(), &v_w.clone())?, &v_h.clone())?, &Py::Int(3i64))?), (Py::str("lateral_area"), v_lat.clone()), (Py::str("total_area"), py::add(&v_lat.clone(), &py::mul(&v_l.clone(), &v_w.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rectangle_measures(mut v_length: Py, mut v_width: Py) -> R {
    let mut v_l: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2512 = (Py::tuple(vec![f__pos(v_length.clone(), Py::str("length"))?, f__pos(v_width.clone(), Py::str("width"))?])).items()?;
    if t2512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l = t2512[0].clone();
    v_w = t2512[1].clone();
    return Ok(Py::dict(vec![(Py::str("area"), py::mul(&v_l.clone(), &v_w.clone())?), (Py::str("perimeter"), py::mul(&Py::Int(2i64), &py::add(&v_l.clone(), &v_w.clone())?)?), (Py::str("diagonal"), py::hypot(&[v_l.clone(), v_w.clone()])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reflect_point_line(mut v_px: Py, mut v_py: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t2543 = (Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?])).items()?;
    if t2543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_px = t2543[0].clone();
    v_py = t2543[1].clone();
    let t2550 = (f__line_check(v_a.clone(), v_b.clone(), v_c.clone(), Py::str("line"))?).items()?;
    if t2550.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2550[0].clone();
    v_b = t2550[1].clone();
    v_c = t2550[2].clone();
    v_k = py::truediv(&py::add(&py::add(&py::mul(&v_a.clone(), &v_px.clone())?, &py::mul(&v_b.clone(), &v_py.clone())?)?, &v_c.clone())?, &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?)?;
    v_s = py::minmax(&[py::abs(&v_px.clone())?, py::abs(&v_py.clone())?], true)?;
    return Ok(Py::dict(vec![(Py::str("x"), f__snap(py::sub(&v_px.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_k.clone())?)?, v_s.clone())?), (Py::str("y"), f__snap(py::sub(&v_py.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_b.clone())?, &v_k.clone())?)?, v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_regular_polygon_angles(mut v_n: Py) -> R {
    if (Py::Bool(matches!(v_n.clone(), Py::Float(_)))).truthy() {
        if ({ let t2571 = v_n.clone(); Py::Bool({ let t2572 = py::int(&v_n.clone())?; if py::ne(&t2571, &t2572) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("n must be a whole number"))));
        }
        v_n = py::int(&v_n.clone())?;
    }
    if ({ let t2577 = { let t2573 = v_n.clone(); Py::Bool({ let t2574 = Py::Int(3i64); if py::lt(&t2573, &t2574)? { true } else { false } }) }; if t2577.truthy() { t2577 } else { { let t2575 = v_n.clone(); Py::Bool({ let t2576 = Py::Int(1000i64); if py::gt(&t2575, &t2576)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a polygon has between 3 and 1000 sides here"))));
    }
    return Ok(Py::dict(vec![(Py::str("interior_angle"), py::truediv(&py::mul(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(180i64))?, &v_n.clone())?), (Py::str("exterior_angle"), py::truediv(&Py::Int(360i64), &v_n.clone())?), (Py::str("interior_sum"), py::float(&py::mul(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(180i64))?)?), (Py::str("diagonals"), py::floordiv(&py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Int(3i64))?)?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_regular_polygon_measures(mut v_n: Py, mut v_side: Py) -> R {
    let mut v_apo: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_s: Py = Py::None;
    if (Py::Bool(matches!(v_n.clone(), Py::Float(_)))).truthy() {
        if ({ let t2606 = v_n.clone(); Py::Bool({ let t2607 = py::int(&v_n.clone())?; if py::ne(&t2606, &t2607) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("n must be a whole number"))));
        }
        v_n = py::int(&v_n.clone())?;
    }
    if ({ let t2612 = { let t2608 = v_n.clone(); Py::Bool({ let t2609 = Py::Int(3i64); if py::lt(&t2608, &t2609)? { true } else { false } }) }; if t2612.truthy() { t2612 } else { { let t2610 = v_n.clone(); Py::Bool({ let t2611 = Py::Int(200i64); if py::gt(&t2610, &t2611)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a regular polygon has between 3 and 200 sides here"))));
    }
    v_s = f__pos(v_side.clone(), Py::str("side"))?;
    let t2617 = (Py::tuple(vec![py::truediv(&v_s.clone(), &py::mul(&Py::Int(2i64), &py::math1("tan", &py::truediv(&Py::Float(std::f64::consts::PI), &v_n.clone())?)?)?)?, py::truediv(&v_s.clone(), &py::mul(&Py::Int(2i64), &py::math1("sin", &py::truediv(&Py::Float(std::f64::consts::PI), &v_n.clone())?)?)?)?])).items()?;
    if t2617.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_apo = t2617[0].clone();
    v_cr = t2617[1].clone();
    return Ok(Py::dict(vec![(Py::str("area"), py::truediv(&py::mul(&py::mul(&v_n.clone(), &v_s.clone())?, &v_apo.clone())?, &Py::Int(2i64))?), (Py::str("perimeter"), py::mul(&v_n.clone(), &v_s.clone())?), (Py::str("apothem"), v_apo.clone()), (Py::str("circumradius"), v_cr.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_regular_prism(mut v_n: Py, mut v_side: Py, mut v_height: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_s: Py = Py::None;
    if (Py::Bool(matches!(v_n.clone(), Py::Float(_)))).truthy() {
        if ({ let t2646 = v_n.clone(); Py::Bool({ let t2647 = py::int(&v_n.clone())?; if py::ne(&t2646, &t2647) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("n must be a whole number"))));
        }
        v_n = py::int(&v_n.clone())?;
    }
    if ({ let t2652 = { let t2648 = v_n.clone(); Py::Bool({ let t2649 = Py::Int(3i64); if py::lt(&t2648, &t2649)? { true } else { false } }) }; if t2652.truthy() { t2652 } else { { let t2650 = v_n.clone(); Py::Bool({ let t2651 = Py::Int(200i64); if py::gt(&t2650, &t2651)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base has between 3 and 200 sides here"))));
    }
    let t2657 = (Py::tuple(vec![f__pos(v_side.clone(), Py::str("side"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t2657.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2657[0].clone();
    v_h = t2657[1].clone();
    v_B = py::truediv(&py::mul(&py::mul(&v_n.clone(), &v_s.clone())?, &v_s.clone())?, &py::mul(&Py::Int(4i64), &py::math1("tan", &py::truediv(&Py::Float(std::f64::consts::PI), &v_n.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("volume"), py::mul(&v_B.clone(), &v_h.clone())?), (Py::str("lateral_area"), py::mul(&py::mul(&v_n.clone(), &v_s.clone())?, &v_h.clone())?), (Py::str("total_area"), py::add(&py::mul(&py::mul(&v_n.clone(), &v_s.clone())?, &v_h.clone())?, &py::mul(&Py::Int(2i64), &v_B.clone())?)?), (Py::str("base_area"), v_B.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rhombus_measures(mut v_d1: Py, mut v_d2: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t2690 = (Py::tuple(vec![f__pos(v_d1.clone(), Py::str("d1"))?, f__pos(v_d2.clone(), Py::str("d2"))?])).items()?;
    if t2690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2690[0].clone();
    v_q = t2690[1].clone();
    v_s = py::truediv(&py::hypot(&[v_p.clone(), v_q.clone()])?, &Py::Int(2i64))?;
    return Ok(Py::dict(vec![(Py::str("area"), py::truediv(&py::mul(&v_p.clone(), &v_q.clone())?, &Py::Int(2i64))?), (Py::str("side"), v_s.clone()), (Py::str("perimeter"), py::mul(&Py::Int(4i64), &v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rotate_point(mut v_px: Py, mut v_py: Py, mut v_angle: Py, mut v_cx: Py, mut v_cy: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_si: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t2721 = (Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?, f__fin(v_angle.clone(), Py::str("angle"))?])).items()?;
    if t2721.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_px = t2721[0].clone();
    v_py = t2721[1].clone();
    v_t = t2721[2].clone();
    let t2722 = (Py::tuple(vec![f__fin(v_cx.clone(), Py::str("cx"))?, f__fin(v_cy.clone(), Py::str("cy"))?])).items()?;
    if t2722.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t2722[0].clone();
    v_cy = t2722[1].clone();
    v_r = py::modulo(&v_t.clone(), &Py::Float(360.0))?;
    v_exact = Py::dict(vec![(Py::Float(0.0), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::Float(90.0), Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)])), (Py::Float(180.0), Py::tuple(vec![py::neg(&Py::Float(1.0))?, Py::Float(0.0)])), (Py::Float(270.0), Py::tuple(vec![Py::Float(0.0), py::neg(&Py::Float(1.0))?]))]);
    let t2723 = (py::method(&v_exact.clone(), "get", &[v_r.clone(), Py::tuple(vec![py::math1("cos", &py::math1("radians", &v_t.clone())?)?, py::math1("sin", &py::math1("radians", &v_t.clone())?)?])])?).items()?;
    if t2723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_co = t2723[0].clone();
    v_si = t2723[1].clone();
    let t2724 = (Py::tuple(vec![py::sub(&v_px.clone(), &v_cx.clone())?, py::sub(&v_py.clone(), &v_cy.clone())?])).items()?;
    if t2724.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dx = t2724[0].clone();
    v_dy = t2724[1].clone();
    v_s = py::minmax(&[py::abs(&v_px.clone())?, py::abs(&v_py.clone())?, py::abs(&v_cx.clone())?, py::abs(&v_cy.clone())?], true)?;
    return Ok(Py::dict(vec![(Py::str("x"), f__snap(py::sub(&py::add(&v_cx.clone(), &py::mul(&v_dx.clone(), &v_co.clone())?)?, &py::mul(&v_dy.clone(), &v_si.clone())?)?, v_s.clone())?), (Py::str("y"), f__snap(py::add(&py::add(&v_cy.clone(), &py::mul(&v_dx.clone(), &v_si.clone())?)?, &py::mul(&v_dy.clone(), &v_co.clone())?)?, v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rotate_point_3d(mut v_point: Py, mut v_axis: Py, mut v_angle: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kp: Py = Py::None;
    let mut v_kxp: Py = Py::None;
    let mut v_nk: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_si: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_P = Py::tuple(({ let mut out = vec![]; for t2743 in py::iter(&(v_point.clone()))? { let mut v_v: Py; 
v_v = t2743;
out.push(f__fin(py::float(&v_v.clone())?, Py::str("point"))?); } Py::list(out) }).items()?);
    v_k = Py::tuple(({ let mut out = vec![]; for t2746 in py::iter(&(v_axis.clone()))? { let mut v_v: Py; 
v_v = t2746;
out.push(f__fin(py::float(&v_v.clone())?, Py::str("axis"))?); } Py::list(out) }).items()?);
    v_t = f__fin(v_angle.clone(), Py::str("angle"))?;
    v_nk = f__vnorm(v_k.clone())?;
    if ({ let t2747 = v_nk.clone(); Py::Bool({ let t2748 = Py::Int(0i64); if py::eq(&t2747, &t2748) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the axis must not be the zero vector"))));
    }
    v_k = Py::tuple(({ let mut out = vec![]; for t2749 in py::iter(&(v_k.clone()))? { let mut v_c: Py; 
v_c = t2749;
out.push(py::truediv(&v_c.clone(), &v_nk.clone())?); } Py::list(out) }).items()?);
    v_r = py::modulo(&v_t.clone(), &Py::Float(360.0))?;
    let t2750 = (py::method(&Py::dict(vec![(Py::Float(0.0), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::Float(90.0), Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)])), (Py::Float(180.0), Py::tuple(vec![py::neg(&Py::Float(1.0))?, Py::Float(0.0)])), (Py::Float(270.0), Py::tuple(vec![Py::Float(0.0), py::neg(&Py::Float(1.0))?]))]), "get", &[v_r.clone(), Py::tuple(vec![py::math1("cos", &py::math1("radians", &v_t.clone())?)?, py::math1("sin", &py::math1("radians", &v_t.clone())?)?])])?).items()?;
    if t2750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_co = t2750[0].clone();
    v_si = t2750[1].clone();
    let t2751 = (Py::tuple(vec![f__vcross(v_k.clone(), v_P.clone())?, f__vdot(v_k.clone(), v_P.clone())?])).items()?;
    if t2751.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kxp = t2751[0].clone();
    v_kp = t2751[1].clone();
    v_out = Py::tuple(({ let mut out = vec![]; for t2752 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2752;
out.push(py::add(&py::add(&py::mul(&py::getitem(&v_P.clone(), &v_j.clone())?, &v_co.clone())?, &py::mul(&py::getitem(&v_kxp.clone(), &v_j.clone())?, &v_si.clone())?)?, &py::mul(&py::mul(&py::getitem(&v_k.clone(), &v_j.clone())?, &v_kp.clone())?, &py::sub(&Py::Int(1i64), &v_co.clone())?)?)?); } Py::list(out) }).items()?);
    v_s = py::minmax(&[{ let mut out = vec![]; for t2753 in py::iter(&(v_P.clone()))? { let mut v_c: Py; 
v_c = t2753;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("x"), f__snap(py::getitem(&v_out.clone(), &Py::Int(0i64))?, v_s.clone())?), (Py::str("y"), f__snap(py::getitem(&v_out.clone(), &Py::Int(1i64))?, v_s.clone())?), (Py::str("z"), f__snap(py::getitem(&v_out.clone(), &Py::Int(2i64))?, v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sector_measures(mut v_radius: Py, mut v_angle: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t2781 = (Py::tuple(vec![f__pos(v_radius.clone(), Py::str("radius"))?, f__fin(v_angle.clone(), Py::str("angle"))?])).items()?;
    if t2781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t2781[0].clone();
    v_a = t2781[1].clone();
    if (py::not(&{ let t2782 = Py::Int(0i64); Py::Bool({ let t2783 = v_a.clone(); if py::lt(&t2782, &t2783)? { { let t2784 = Py::Int(360i64); if py::le(&t2783, &t2784)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the central angle must be between 0 and 360 degrees"))));
    }
    v_t = py::math1("radians", &v_a.clone())?;
    return Ok(Py::dict(vec![(Py::str("arc_length"), py::mul(&v_r.clone(), &v_t.clone())?), (Py::str("sector_area"), py::truediv(&py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &v_t.clone())?, &Py::Int(2i64))?), (Py::str("chord"), py::mul(&py::mul(&Py::Int(2i64), &v_r.clone())?, &py::math1("sin", &py::truediv(&v_t.clone(), &Py::Int(2i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_segment_intersection(mut v_points: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_ax: Py = Py::None;
    let mut v_ay: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_by: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_none: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let mut v_rx: Py = Py::None;
    let mut v_ry: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let t2824 = (f__pairs(v_points.clone(), Py::str("points"), Py::Int(4i64), Py::Int(4i64))?).items()?;
    if t2824.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2824[0].clone();
    v_B = t2824[1].clone();
    v_C = t2824[2].clone();
    v_D = t2824[3].clone();
    if ({ let t2829 = { let t2825 = v_A.clone(); Py::Bool({ let t2826 = v_B.clone(); if py::eq(&t2825, &t2826) { true } else { false } }) }; if t2829.truthy() { t2829 } else { { let t2827 = v_C.clone(); Py::Bool({ let t2828 = v_D.clone(); if py::eq(&t2827, &t2828) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a segment has two equal endpoints"))));
    }
    v_none = Py::dict(vec![(Py::str("status"), Py::str("none")), (Py::str("x"), Py::None), (Py::str("y"), Py::None)]);
    if (py::not(&f__segments_meet_exact(v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone())?)).truthy() {
        return Ok(v_none.clone());
    }
    let t2874 = ({ let mut out = vec![]; for t2873 in py::iter(&({ let mut v: Vec<Py> = vec![]; v.extend((v_A.clone()).items()?); v.extend((v_B.clone()).items()?); v.extend((v_C.clone()).items()?); v.extend((v_D.clone()).items()?); Py::tuple(v) }))? { let mut v_v: Py; 
v_v = t2873;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t2874.len() != 8 { return Err(PyErr::Value("unpack".into())); }
    v_ax = t2874[0].clone();
    v_ay = t2874[1].clone();
    v_bx = t2874[2].clone();
    v_by = t2874[3].clone();
    v_cx = t2874[4].clone();
    v_cy = t2874[5].clone();
    v_dx = t2874[6].clone();
    v_dy = t2874[7].clone();
    let t2875 = (Py::tuple(vec![py::sub(&v_bx.clone(), &v_ax.clone())?, py::sub(&v_by.clone(), &v_ay.clone())?, py::sub(&v_dx.clone(), &v_cx.clone())?, py::sub(&v_dy.clone(), &v_cy.clone())?])).items()?;
    if t2875.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rx = t2875[0].clone();
    v_ry = t2875[1].clone();
    v_sx = t2875[2].clone();
    v_sy = t2875[3].clone();
    v_den = py::sub(&py::mul(&v_rx.clone(), &v_sy.clone())?, &py::mul(&v_ry.clone(), &v_sx.clone())?)?;
    if ({ let t2876 = v_den.clone(); Py::Bool({ let t2877 = Py::Int(0i64); if py::ne(&t2876, &t2877) { true } else { false } }) }).truthy() {
        v_t = py::truediv(&py::sub(&py::mul(&py::sub(&v_cx.clone(), &v_ax.clone())?, &v_sy.clone())?, &py::mul(&py::sub(&v_cy.clone(), &v_ay.clone())?, &v_sx.clone())?)?, &v_den.clone())?;
        return Ok(Py::dict(vec![(Py::str("status"), Py::str("point")), (Py::str("x"), py::add(&py::float(&py::add(&v_ax.clone(), &py::mul(&v_t.clone(), &v_rx.clone())?)?)?, &Py::Float(0.0))?), (Py::str("y"), py::add(&py::float(&py::add(&v_ay.clone(), &py::mul(&v_t.clone(), &v_ry.clone())?)?)?, &Py::Float(0.0))?)]));
    }
    v_rr = py::add(&py::mul(&v_rx.clone(), &v_rx.clone())?, &py::mul(&v_ry.clone(), &v_ry.clone())?)?;
    v_t0 = py::truediv(&py::add(&py::mul(&py::sub(&v_cx.clone(), &v_ax.clone())?, &v_rx.clone())?, &py::mul(&py::sub(&v_cy.clone(), &v_ay.clone())?, &v_ry.clone())?)?, &v_rr.clone())?;
    v_t1 = py::truediv(&py::add(&py::mul(&py::sub(&v_dx.clone(), &v_ax.clone())?, &v_rx.clone())?, &py::mul(&py::sub(&v_dy.clone(), &v_ay.clone())?, &v_ry.clone())?)?, &v_rr.clone())?;
    let t2878 = (Py::tuple(vec![py::minmax(&[py::fraction(&[Py::Int(0i64)])?, py::minmax(&[v_t0.clone(), v_t1.clone()], false)?], true)?, py::minmax(&[py::fraction(&[Py::Int(1i64)])?, py::minmax(&[v_t0.clone(), v_t1.clone()], true)?], false)?])).items()?;
    if t2878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2878[0].clone();
    v_hi = t2878[1].clone();
    if ({ let t2879 = v_lo.clone(); Py::Bool({ let t2880 = v_hi.clone(); if py::eq(&t2879, &t2880) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("status"), Py::str("point")), (Py::str("x"), py::add(&py::float(&py::add(&v_ax.clone(), &py::mul(&v_lo.clone(), &v_rx.clone())?)?)?, &Py::Float(0.0))?), (Py::str("y"), py::add(&py::float(&py::add(&v_ay.clone(), &py::mul(&v_lo.clone(), &v_ry.clone())?)?)?, &Py::Float(0.0))?)]));
    }
    return Ok(Py::dict(vec![(Py::str("status"), Py::str("overlap")), (Py::str("x"), Py::None), (Py::str("y"), Py::None)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_similar_figures(mut v_measure: Py, mut v_m1: Py, mut v_m2: Py, mut v_other_length: Py, mut v_other_area: Py, mut v_other_volume: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_ar: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_vr: Py = Py::None;
    if ({ let t2925 = v_measure.clone(); Py::Bool({ let t2926 = c__SIM_MEASURES()?; if !py::contains(&t2926, &t2925)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the measure compared is lengths, areas or volumes"))));
    }
    let t2931 = (Py::tuple(vec![f__pos(v_m1.clone(), Py::str("m1"))?, f__pos(v_m2.clone(), Py::str("m2"))?])).items()?;
    if t2931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2931[0].clone();
    v_b = t2931[1].clone();
    v_r = py::truediv(&v_b.clone(), &v_a.clone())?;
    if ({ let t2932 = v_measure.clone(); Py::Bool({ let t2933 = Py::str("lengths"); if py::eq(&t2932, &t2933) { true } else { false } }) }).truthy() {
        let t2934 = (Py::tuple(vec![v_r.clone(), py::mul(&v_r.clone(), &v_r.clone())?, py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &v_r.clone())?])).items()?;
        if t2934.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2934[0].clone();
        v_ar = t2934[1].clone();
        v_vr = t2934[2].clone();
    } else {
        if ({ let t2935 = v_measure.clone(); Py::Bool({ let t2936 = Py::str("areas"); if py::eq(&t2935, &t2936) { true } else { false } }) }).truthy() {
            v_k = py::math1("sqrt", &v_r.clone())?;
            let t2937 = (Py::tuple(vec![v_r.clone(), py::mul(&v_r.clone(), &v_k.clone())?])).items()?;
            if t2937.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_ar = t2937[0].clone();
            v_vr = t2937[1].clone();
        } else {
            v_k = py::pow(&v_r.clone(), &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?;
            let t2938 = (Py::tuple(vec![py::mul(&v_k.clone(), &v_k.clone())?, v_r.clone()])).items()?;
            if t2938.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_ar = t2938[0].clone();
            v_vr = t2938[1].clone();
        }
    }
    v_out = Py::dict(vec![(Py::str("scale_factor"), v_k.clone()), (Py::str("area_ratio"), v_ar.clone()), (Py::str("volume_ratio"), v_vr.clone()), (Py::str("length2"), Py::None), (Py::str("area2"), Py::None), (Py::str("volume2"), Py::None)]);
    if ({ let t2939 = v_other_length.clone(); Py::Bool({ let t2940 = Py::None; if !py::is(&t2939, &t2940) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("length2"), py::mul(&v_k.clone(), &f__pos(v_other_length.clone(), Py::str("other_length"))?)?)?;
    }
    if ({ let t2941 = v_other_area.clone(); Py::Bool({ let t2942 = Py::None; if !py::is(&t2941, &t2942) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("area2"), py::mul(&v_ar.clone(), &f__pos(v_other_area.clone(), Py::str("other_area"))?)?)?;
    }
    if ({ let t2943 = v_other_volume.clone(); Py::Bool({ let t2944 = Py::None; if !py::is(&t2943, &t2944) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("volume2"), py::mul(&v_vr.clone(), &f__pos(v_other_volume.clone(), Py::str("other_volume"))?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_skew_lines_distance(mut v_p1: Py, mut v_d1: Py, mut v_p2: Py, mut v_d2: Py) -> R {
    let mut v_D1: Py = Py::None;
    let mut v_D2: Py = Py::None;
    let mut v_P1: Py = Py::None;
    let mut v_P2: Py = Py::None;
    let mut v_dist: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nx: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2975 = ({ let mut out = vec![]; for t2970 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_p1.clone(), Py::str("p1")]), Py::tuple(vec![v_d1.clone(), Py::str("d1")]), Py::tuple(vec![v_p2.clone(), Py::str("p2")]), Py::tuple(vec![v_d2.clone(), Py::str("d2")])])))? { let mut v_n: Py; let mut v_vec: Py; 
let t2971 = (t2970).items()?;
if t2971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_vec = t2971[0].clone();
v_n = t2971[1].clone();
out.push(Py::tuple(({ let mut out = vec![]; for t2972 in py::iter(&(v_vec.clone()))? { let mut v_v: Py; 
v_v = t2972;
out.push(f__fin(py::float(&v_v.clone())?, v_n.clone())?); } Py::list(out) }).items()?)); } Py::list(out) }).items()?;
    if t2975.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P1 = t2975[0].clone();
    v_D1 = t2975[1].clone();
    v_P2 = t2975[2].clone();
    v_D2 = t2975[3].clone();
    if ({ let t2980 = { let t2976 = v_D1.clone(); Py::Bool({ let t2977 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t2976, &t2977) { true } else { false } }) }; if t2980.truthy() { t2980 } else { { let t2978 = v_D2.clone(); Py::Bool({ let t2979 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t2978, &t2979) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a direction vector is zero"))));
    }
    v_w = f__vsub(v_P2.clone(), v_P1.clone())?;
    v_nx = f__vcross(f__xvec(v_D1.clone())?, f__xvec(v_D2.clone())?)?;
    if ({ let t2982 = v_nx.clone(); Py::Bool({ let t2983 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t2982, &t2983) { true } else { false } }) }).truthy() {
        v_dist = py::truediv(&f__vnorm(f__vcross(v_w.clone(), v_D1.clone())?)?, &f__vnorm(v_D1.clone())?)?;
        return Ok(Py::dict(vec![(Py::str("distance"), v_dist.clone()), (Py::str("parallel"), Py::str("yes"))]));
    }
    v_n = f__vcross(v_D1.clone(), v_D2.clone())?;
    return Ok(Py::dict(vec![(Py::str("distance"), py::truediv(&py::abs(&f__vdot(v_w.clone(), v_n.clone())?)?, &f__vnorm(v_n.clone())?)?), (Py::str("parallel"), Py::str("no"))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solid_angle_cone(mut v_half_angle: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_om: Py = Py::None;
    v_a = f__fin(v_half_angle.clone(), Py::str("half_angle"))?;
    if (py::not(&{ let t3010 = Py::Int(0i64); Py::Bool({ let t3011 = v_a.clone(); if py::lt(&t3010, &t3011)? { { let t3012 = Py::Int(180i64); if py::le(&t3011, &t3012)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the half-angle must be between 0 and 180 degrees"))));
    }
    v_om = py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&py::math1("sin", &py::truediv(&py::math1("radians", &v_a.clone())?, &Py::Int(2i64))?)?, &Py::Int(2i64))?)?;
    return Ok(Py::dict(vec![(Py::str("solid_angle"), v_om.clone()), (Py::str("fraction_of_sphere"), py::truediv(&v_om.clone(), &py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solid_angle_triangle(mut v_points: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_la: Py = Py::None;
    let mut v_lb: Py = Py::None;
    let mut v_lc: Py = Py::None;
    let mut v_num: Py = Py::None;
    let t3047 = (f__triples(v_points.clone(), Py::str("points"), Py::Int(3i64))?).items()?;
    if t3047.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3047[0].clone();
    v_b = t3047[1].clone();
    v_c = t3047[2].clone();
    if ({ let t3048 = Py::Float(0.0); Py::Bool({ let t3049 = Py::tuple(vec![f__vnorm(v_a.clone())?, f__vnorm(v_b.clone())?, f__vnorm(v_c.clone())?]); if py::contains(&t3049, &t3048)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a vertex is at the origin (the observer)"))));
    }
    if ({ let t3050 = f__vdot(f__xvec(v_a.clone())?, f__vcross(f__xvec(v_b.clone())?, f__xvec(v_c.clone())?)?)?; Py::Bool({ let t3051 = Py::Int(0i64); if py::eq(&t3050, &t3051) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the triangle's plane passes through the origin: it subtends no solid angle"))));
    }
    let t3053 = (Py::tuple(vec![f__vnorm(v_a.clone())?, f__vnorm(v_b.clone())?, f__vnorm(v_c.clone())?])).items()?;
    if t3053.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_la = t3053[0].clone();
    v_lb = t3053[1].clone();
    v_lc = t3053[2].clone();
    v_num = py::abs(&f__vdot(v_a.clone(), f__vcross(v_b.clone(), v_c.clone())?)?)?;
    v_den = py::add(&py::add(&py::add(&py::mul(&py::mul(&v_la.clone(), &v_lb.clone())?, &v_lc.clone())?, &py::mul(&f__vdot(v_a.clone(), v_b.clone())?, &v_lc.clone())?)?, &py::mul(&f__vdot(v_a.clone(), v_c.clone())?, &v_lb.clone())?)?, &py::mul(&f__vdot(v_b.clone(), v_c.clone())?, &v_la.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("solid_angle"), py::mul(&Py::Int(2i64), &py::atan2(&v_num.clone(), &v_den.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sphere_four_points(mut v_points: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_det3: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_rows: Py = Py::None;
    v_P = f__triples(v_points.clone(), Py::str("points"), Py::Int(4i64))?;
    v_X = { let mut out = vec![]; for t3088 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t3088;
out.push(f__xvec(v_p.clone())?); } Py::list(out) };
    v_rows = { let mut out = vec![]; for t3090 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)])))? { let mut v_k: Py; 
v_k = t3090;
out.push({ let mut out = vec![]; for t3091 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t3091;
out.push(py::mul(&Py::Int(2i64), &py::sub(&py::getitem(&py::getitem(&v_X.clone(), &v_k.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &Py::Int(0i64))?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    v_rhs = { let mut out = vec![]; for t3092 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)])))? { let mut v_k: Py; 
v_k = t3092;
out.push(py::sum(&{ let mut out = vec![]; for t3093 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t3093;
out.push(py::sub(&py::pow(&py::getitem(&py::getitem(&v_X.clone(), &v_k.clone())?, &v_j.clone())?, &Py::Int(2i64))?, &py::pow(&py::getitem(&py::getitem(&v_X.clone(), &Py::Int(0i64))?, &v_j.clone())?, &Py::Int(2i64))?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_det3 = { let v_P = v_P.clone(); let v_X = v_X.clone(); let v_points = v_points.clone(); let v_rhs = v_rhs.clone(); let v_rows = v_rows.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?)?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::sub(&py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_m.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_D = py::call(&v_det3.clone(), &[v_rows.clone()])?;
    if ({ let t3094 = v_D.clone(); Py::Bool({ let t3095 = Py::Int(0i64); if py::eq(&t3094, &t3095) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the four points are coplanar: no unique sphere passes through them"))));
    }
    v_c = Py::list(vec![]);
    for t3096 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v_j = t3096;
        v_m = { let mut out = vec![]; for t3097 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t3097;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) };
        for t3098 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v_k = t3098;
            py::setitem(&py::getitem(&v_m.clone(), &v_k.clone())?, &v_j.clone(), py::getitem(&v_rhs.clone(), &v_k.clone())?)?;
        }
        let _ = py::method(&v_c.clone(), "append", &[py::truediv(&py::call(&v_det3.clone(), &[v_m.clone()])?, &v_D.clone())?])?;
    }
    v_r2 = py::sum(&{ let mut out = vec![]; for t3099 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t3099;
out.push(py::pow(&py::sub(&py::getitem(&py::getitem(&v_X.clone(), &Py::Int(0i64))?, &v_j.clone())?, &py::getitem(&v_c.clone(), &v_j.clone())?)?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("cx"), py::add(&py::float(&py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, &Py::Float(0.0))?), (Py::str("cy"), py::add(&py::float(&py::getitem(&v_c.clone(), &Py::Int(1i64))?)?, &Py::Float(0.0))?), (Py::str("cz"), py::add(&py::float(&py::getitem(&v_c.clone(), &Py::Int(2i64))?)?, &Py::Float(0.0))?), (Py::str("radius"), py::math1("sqrt", &py::float(&v_r2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sphere_measures(mut v_radius: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = f__pos(v_radius.clone(), Py::str("radius"))?;
    return Ok(Py::dict(vec![(Py::str("volume"), py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&v_r.clone(), &Py::Int(3i64))?)?, &Py::Int(3i64))?), (Py::str("surface_area"), py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?, &v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spherical_cap(mut v_radius: Py, mut v_height: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_h: Py = Py::None;
    let t3153 = (Py::tuple(vec![f__pos(v_radius.clone(), Py::str("radius"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t3153.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t3153[0].clone();
    v_h = t3153[1].clone();
    if ({ let t3154 = v_h.clone(); Py::Bool({ let t3155 = py::mul(&Py::Int(2i64), &v_R.clone())?; if py::gt(&t3154, &t3155)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cap height cannot exceed the sphere's diameter"))));
    }
    return Ok(Py::dict(vec![(Py::str("volume"), py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_h.clone())?, &v_h.clone())?, &py::sub(&py::mul(&Py::Int(3i64), &v_R.clone())?, &v_h.clone())?)?, &Py::Int(3i64))?), (Py::str("cap_area"), py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_R.clone())?, &v_h.clone())?), (Py::str("base_radius"), py::math1("sqrt", &py::mul(&v_h.clone(), &py::sub(&py::mul(&Py::Int(2i64), &v_R.clone())?, &v_h.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spherical_triangle(mut v_points: Py, mut v_radius: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_P = f__pairs(v_points.clone(), Py::str("points"), Py::Int(3i64), Py::Int(3i64))?;
    v_v = { let mut out = vec![]; for t3195 in py::iter(&(py::enumerate(&v_P.clone(), 0)?))? { let mut v_a: Py; let mut v_b: Py; let mut v_k: Py; 
let t3196 = (t3195).items()?;
if t3196.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3196[0].clone();
let t3197 = (t3196[1].clone()).items()?;
if t3197.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3197[0].clone();
v_b = t3197[1].clone();
out.push(f__latlon(v_a.clone(), v_b.clone(), Py::str(&["vertex ".to_string(), py::format(&py::add(&v_k.clone(), &Py::Int(1i64))?, "")?].concat()))?); } Py::list(out) };
    v_R = f__pos(v_radius.clone(), Py::str("radius"))?;
    v_U = { let mut out = vec![]; for t3207 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3207;
out.push({ let mut v: Vec<Py> = vec![]; v.extend((v_x.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "lat_r") { Some(x) => x, None => { return Err(PyErr::Type("_unit() missing argument lat_r".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "lon_r") { Some(x) => x, None => { return Err(PyErr::Type("_unit() missing argument lon_r".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("_unit() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_unit() got an unexpected keyword argument".into())) } f__unit(a0, a1)? }); } Py::list(out) };
    if ({ let t3208 = py::abs(&f__vdot(py::getitem(&v_U.clone(), &Py::Int(0i64))?, f__vcross(py::getitem(&v_U.clone(), &Py::Int(1i64))?, py::getitem(&v_U.clone(), &Py::Int(2i64))?)?)?)?; Py::Bool({ let t3209 = Py::Float(1e-12); if py::lt(&t3208, &t3209)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the vertices lie on one great circle (or two coincide): no spherical triangle"))));
    }
    let t3213 = (Py::tuple(vec![f__central(py::getitem(&v_v.clone(), &Py::Int(1i64))?, py::getitem(&v_v.clone(), &Py::Int(2i64))?)?, f__central(py::getitem(&v_v.clone(), &Py::Int(0i64))?, py::getitem(&v_v.clone(), &Py::Int(2i64))?)?, f__central(py::getitem(&v_v.clone(), &Py::Int(0i64))?, py::getitem(&v_v.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3213.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3213[0].clone();
    v_b = t3213[1].clone();
    v_c = t3213[2].clone();
    v_s = py::truediv(&py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?, &Py::Int(2i64))?;
    v_t = py::mul(&py::mul(&py::mul(&py::math1("tan", &py::truediv(&v_s.clone(), &Py::Int(2i64))?)?, &py::math1("tan", &py::truediv(&py::sub(&v_s.clone(), &v_a.clone())?, &Py::Int(2i64))?)?)?, &py::math1("tan", &py::truediv(&py::sub(&v_s.clone(), &v_b.clone())?, &Py::Int(2i64))?)?)?, &py::math1("tan", &py::truediv(&py::sub(&v_s.clone(), &v_c.clone())?, &Py::Int(2i64))?)?)?;
    v_E = py::mul(&Py::Int(4i64), &py::math1("atan", &py::math1("sqrt", &py::minmax(&[Py::Float(0.0), v_t.clone()], true)?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("excess"), py::math1("degrees", &v_E.clone())?), (Py::str("area"), py::mul(&py::mul(&v_E.clone(), &v_R.clone())?, &v_R.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spheroid_measures(mut v_a: Py, mut v_c: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_e: Py = Py::None;
    let t3248 = (Py::tuple(vec![f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_c.clone(), Py::str("c"))?])).items()?;
    if t3248.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3248[0].clone();
    v_c = t3248[1].clone();
    if ({ let t3249 = v_a.clone(); Py::Bool({ let t3250 = v_c.clone(); if py::eq(&t3249, &t3250) { true } else { false } }) }).truthy() {
        v_S = py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_a.clone())?;
    } else {
        if ({ let t3251 = v_c.clone(); Py::Bool({ let t3252 = v_a.clone(); if py::lt(&t3251, &t3252)? { true } else { false } }) }).truthy() {
            v_e = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_c.clone(), &v_a.clone())?, &Py::Int(2i64))?)?)?;
            v_S = py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_a.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::truediv(&py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &v_e.clone())?)?, &v_e.clone())?, &py::math1("atanh", &v_e.clone())?)?)?)?;
        } else {
            v_e = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_a.clone(), &v_c.clone())?, &Py::Int(2i64))?)?)?;
            v_S = py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_a.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::truediv(&v_c.clone(), &py::mul(&v_a.clone(), &v_e.clone())?)?, &py::math1("asin", &v_e.clone())?)?)?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("surface_area"), v_S.clone()), (Py::str("volume"), py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_a.clone())?, &v_c.clone())?, &Py::Int(3i64))?), (Py::str("shape"), (if ({ let t3253 = v_a.clone(); Py::Bool({ let t3254 = v_c.clone(); if py::eq(&t3253, &t3254) { true } else { false } }) }).truthy() { Py::str("sphere") } else { (if ({ let t3255 = v_c.clone(); Py::Bool({ let t3256 = v_a.clone(); if py::lt(&t3255, &t3256)? { true } else { false } }) }).truthy() { Py::str("oblate") } else { Py::str("prolate") }) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_surface_curvature(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_D: Py, mut v_E: Py, mut v_x0: Py, mut v_y0: Py, mut v_F: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    for t3286 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("A"), v_A.clone()]), Py::tuple(vec![Py::str("B"), v_B.clone()]), Py::tuple(vec![Py::str("C"), v_C.clone()]), Py::tuple(vec![Py::str("D"), v_D.clone()]), Py::tuple(vec![Py::str("E"), v_E.clone()]), Py::tuple(vec![Py::str("x0"), v_x0.clone()]), Py::tuple(vec![Py::str("y0"), v_y0.clone()]), Py::tuple(vec![Py::str("F"), v_F.clone()])])))? {
        let t3289 = (t3286).items()?;
        if t3289.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t3289[0].clone();
        v_v = t3289[1].clone();
        let _ = f__fin(v_v.clone(), v_n.clone())?;
    }
    let t3290 = (Py::tuple(vec![py::add(&py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_x0.clone())?, &py::mul(&v_B.clone(), &v_y0.clone())?)?, &v_D.clone())?, py::add(&py::add(&py::mul(&v_B.clone(), &v_x0.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_C.clone())?, &v_y0.clone())?)?, &v_E.clone())?])).items()?;
    if t3290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3290[0].clone();
    v_q = t3290[1].clone();
    let t3291 = (Py::tuple(vec![py::mul(&Py::Int(2i64), &v_A.clone())?, v_B.clone(), py::mul(&Py::Int(2i64), &v_C.clone())?])).items()?;
    if t3291.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r = t3291[0].clone();
    v_s = t3291[1].clone();
    v_t = t3291[2].clone();
    v_w = py::add(&py::add(&Py::Int(1i64), &py::mul(&v_p.clone(), &v_p.clone())?)?, &py::mul(&v_q.clone(), &v_q.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("gaussian"), py::add(&py::truediv(&py::sub(&py::mul(&v_r.clone(), &v_t.clone())?, &py::mul(&v_s.clone(), &v_s.clone())?)?, &py::mul(&v_w.clone(), &v_w.clone())?)?, &Py::Float(0.0))?), (Py::str("mean"), py::add(&py::truediv(&py::add(&py::sub(&py::mul(&py::add(&Py::Int(1i64), &py::mul(&v_q.clone(), &v_q.clone())?)?, &v_r.clone())?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_p.clone())?, &v_q.clone())?, &v_s.clone())?)?, &py::mul(&py::add(&Py::Int(1i64), &py::mul(&v_p.clone(), &v_p.clone())?)?, &v_t.clone())?)?, &py::mul(&Py::Int(2i64), &py::pow(&v_w.clone(), &Py::Float(1.5))?)?)?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tangent_from_point(mut v_cx: Py, mut v_cy: Py, mut v_r: Py, mut v_px: Py, mut v_py: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_d2x: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gg: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_vx: Py = Py::None;
    let mut v_vy: Py = Py::None;
    let t3316 = (Py::tuple(vec![f__fin(v_cx.clone(), Py::str("cx"))?, f__fin(v_cy.clone(), Py::str("cy"))?, f__pos(v_r.clone(), Py::str("r"))?, f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?])).items()?;
    if t3316.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t3316[0].clone();
    v_cy = t3316[1].clone();
    v_r = t3316[2].clone();
    v_px = t3316[3].clone();
    v_py = t3316[4].clone();
    v_d2x = py::add(&py::pow(&py::sub(&f__fr(v_px.clone())?, &f__fr(v_cx.clone())?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&f__fr(v_py.clone())?, &f__fr(v_cy.clone())?)?, &Py::Int(2i64))?)?;
    v_g = py::sub(&v_d2x.clone(), &py::pow(&f__fr(v_r.clone())?, &Py::Int(2i64))?)?;
    if ({ let t3317 = v_g.clone(); Py::Bool({ let t3318 = Py::Int(0i64); if py::lt(&t3317, &t3318)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point is inside the circle: no tangent passes through it"))));
    }
    if ({ let t3319 = v_g.clone(); Py::Bool({ let t3320 = Py::Int(0i64); if py::eq(&t3319, &t3320) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("length"), Py::Float(0.0)), (Py::str("x1"), v_px.clone()), (Py::str("y1"), v_py.clone()), (Py::str("x2"), v_px.clone()), (Py::str("y2"), v_py.clone())]));
    }
    let t3321 = (Py::tuple(vec![py::sub(&v_px.clone(), &v_cx.clone())?, py::sub(&v_py.clone(), &v_cy.clone())?])).items()?;
    if t3321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vx = t3321[0].clone();
    v_vy = t3321[1].clone();
    v_d2 = py::float(&v_d2x.clone())?;
    v_L = py::math1("sqrt", &py::float(&v_g.clone())?)?;
    let t3322 = (Py::tuple(vec![py::truediv(&py::mul(&v_r.clone(), &v_r.clone())?, &v_d2.clone())?, py::truediv(&py::mul(&v_r.clone(), &v_L.clone())?, &v_d2.clone())?])).items()?;
    if t3322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t3322[0].clone();
    v_gg = t3322[1].clone();
    v_pts = f__sort_pts(Py::list(vec![Py::tuple(vec![py::sub(&py::add(&v_cx.clone(), &py::mul(&v_f.clone(), &v_vx.clone())?)?, &py::mul(&v_gg.clone(), &v_vy.clone())?)?, py::add(&py::add(&v_cy.clone(), &py::mul(&v_f.clone(), &v_vy.clone())?)?, &py::mul(&v_gg.clone(), &v_vx.clone())?)?]), Py::tuple(vec![py::add(&py::add(&v_cx.clone(), &py::mul(&v_f.clone(), &v_vx.clone())?)?, &py::mul(&v_gg.clone(), &v_vy.clone())?)?, py::sub(&py::add(&v_cy.clone(), &py::mul(&v_f.clone(), &v_vy.clone())?)?, &py::mul(&v_gg.clone(), &v_vx.clone())?)?])]))?;
    return Ok(Py::dict(vec![(Py::str("length"), v_L.clone()), (Py::str("x1"), py::getitem(&py::getitem(&v_pts.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?), (Py::str("y1"), py::getitem(&py::getitem(&v_pts.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?), (Py::str("x2"), py::getitem(&py::getitem(&v_pts.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?), (Py::str("y2"), py::getitem(&py::getitem(&v_pts.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tetrahedron_measures(mut v_points: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_det: Py = Py::None;
    v_P = f__triples(v_points.clone(), Py::str("points"), Py::Int(4i64))?;
    v_X = { let mut out = vec![]; for t3346 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t3346;
out.push(f__xvec(v_p.clone())?); } Py::list(out) };
    if ({ let t3348 = f__vdot(f__vsub(py::getitem(&v_X.clone(), &Py::Int(1i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?, f__vcross(f__vsub(py::getitem(&v_X.clone(), &Py::Int(2i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?, f__vsub(py::getitem(&v_X.clone(), &Py::Int(3i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?)?)?; Py::Bool({ let t3349 = Py::Int(0i64); if py::eq(&t3348, &t3349) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the four points are coplanar: they do not form a tetrahedron"))));
    }
    v_det = f__vdot(f__vsub(py::getitem(&v_P.clone(), &Py::Int(1i64))?, py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, f__vcross(f__vsub(py::getitem(&v_P.clone(), &Py::Int(2i64))?, py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, f__vsub(py::getitem(&v_P.clone(), &Py::Int(3i64))?, py::getitem(&v_P.clone(), &Py::Int(0i64))?)?)?)?;
    v_S = py::sum(&{ let mut out = vec![]; for t3350 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(2i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(3i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(2i64), Py::Int(3i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)])])))? { let mut v_i: Py; let mut v_j: Py; let mut v_k: Py; 
let t3351 = (t3350).items()?;
if t3351.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t3351[0].clone();
v_j = t3351[1].clone();
v_k = t3351[2].clone();
out.push(py::truediv(&f__vnorm(f__vcross(f__vsub(py::getitem(&v_P.clone(), &v_j.clone())?, py::getitem(&v_P.clone(), &v_i.clone())?)?, f__vsub(py::getitem(&v_P.clone(), &v_k.clone())?, py::getitem(&v_P.clone(), &v_i.clone())?)?)?)?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("volume"), py::truediv(&py::abs(&v_det.clone())?, &Py::Int(6i64))?), (Py::str("surface_area"), v_S.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_torus_curvature(mut v_major: Py, mut v_minor: Py, mut v_v: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t3394 = (Py::tuple(vec![f__pos(v_major.clone(), Py::str("major radius"))?, f__pos(v_minor.clone(), Py::str("minor radius"))?, f__fin(v_v.clone(), Py::str("v"))?])).items()?;
    if t3394.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_R = t3394[0].clone();
    v_r = t3394[1].clone();
    v_v = t3394[2].clone();
    if ({ let t3395 = v_r.clone(); Py::Bool({ let t3396 = v_R.clone(); if py::ge(&t3395, &t3396)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the tube radius must be smaller than the major radius (a ring torus)"))));
    }
    v_cv = py::math1("cos", &py::math1("radians", &v_v.clone())?)?;
    let t3397 = (Py::tuple(vec![py::truediv(&Py::Int(1i64), &v_r.clone())?, py::truediv(&v_cv.clone(), &py::add(&v_R.clone(), &py::mul(&v_r.clone(), &v_cv.clone())?)?)?])).items()?;
    if t3397.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k1 = t3397[0].clone();
    v_k2 = t3397[1].clone();
    return Ok(Py::dict(vec![(Py::str("gaussian"), py::add(&py::mul(&v_k1.clone(), &v_k2.clone())?, &Py::Float(0.0))?), (Py::str("mean"), py::truediv(&py::add(&v_k1.clone(), &v_k2.clone())?, &Py::Int(2i64))?), (Py::str("k1"), v_k1.clone()), (Py::str("k2"), py::add(&v_k2.clone(), &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_torus_measures(mut v_major: Py, mut v_minor: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t3430 = (Py::tuple(vec![f__pos(v_major.clone(), Py::str("major radius"))?, f__pos(v_minor.clone(), Py::str("minor radius"))?])).items()?;
    if t3430.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t3430[0].clone();
    v_r = t3430[1].clone();
    if ({ let t3431 = v_r.clone(); Py::Bool({ let t3432 = v_R.clone(); if py::ge(&t3431, &t3432)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the tube radius must be smaller than the major radius (a ring torus)"))));
    }
    return Ok(Py::dict(vec![(Py::str("volume"), py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &v_R.clone())?, &v_r.clone())?, &v_r.clone())?), (Py::str("surface_area"), py::mul(&py::mul(&py::mul(&Py::Int(4i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &v_R.clone())?, &v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_trapezoid_measures(mut v_a: Py, mut v_b: Py, mut v_height: Py) -> R {
    let mut v_h: Py = Py::None;
    let t3465 = (Py::tuple(vec![f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t3465.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3465[0].clone();
    v_b = t3465[1].clone();
    v_h = t3465[2].clone();
    return Ok(Py::dict(vec![(Py::str("area"), py::truediv(&py::mul(&py::add(&v_a.clone(), &v_b.clone())?, &v_h.clone())?, &Py::Int(2i64))?), (Py::str("midsegment"), py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triangle_area_3d(mut v_points: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3500 = (f__triples(v_points.clone(), Py::str("points"), Py::Int(3i64))?).items()?;
    if t3500.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3500[0].clone();
    v_B = t3500[1].clone();
    v_C = t3500[2].clone();
    if ({ let t3501 = f__vcross(f__vsub(f__xvec(v_B.clone())?, f__xvec(v_A.clone())?)?, f__vsub(f__xvec(v_C.clone())?, f__xvec(v_A.clone())?)?)?; Py::Bool({ let t3502 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t3501, &t3502) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the three points are collinear"))));
    }
    let t3504 = (Py::tuple(vec![py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_B.clone())?])).items()?;
    if t3504.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3504[0].clone();
    v_b = t3504[1].clone();
    v_c = t3504[2].clone();
    let t3505 = (py::sorted(&Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]), (Py::Bool(true)).truthy())?).items()?;
    if t3505.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3505[0].clone();
    v_y = t3505[1].clone();
    v_z = t3505[2].clone();
    return Ok(Py::dict(vec![(Py::str("area"), py::mul(&Py::Float(0.25), &py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::add(&v_x.clone(), &py::add(&v_y.clone(), &v_z.clone())?)?, &py::sub(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_x.clone(), &py::sub(&v_y.clone(), &v_z.clone())?)?)?)?)?), (Py::str("perimeter"), py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triangle_base_height(mut v_base: Py, mut v_height: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_h: Py = Py::None;
    let t3538 = (Py::tuple(vec![f__pos(v_base.clone(), Py::str("base"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t3538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t3538[0].clone();
    v_h = t3538[1].clone();
    return Ok(Py::dict(vec![(Py::str("area"), py::truediv(&py::mul(&v_b.clone(), &v_h.clone())?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triangle_circumcircle(mut v_points: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_by: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_ux: Py = Py::None;
    let mut v_uy: Py = Py::None;
    let t3623 = (f__tri_pts(v_points.clone())?).items()?;
    if t3623.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3623[0].clone();
    v_B = t3623[1].clone();
    v_C = t3623[2].clone();
    let t3624 = (Py::tuple(vec![py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3624.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t3624[0].clone();
    v_by = t3624[1].clone();
    v_cx = t3624[2].clone();
    v_cy = t3624[3].clone();
    v_d = py::mul(&Py::Int(2i64), &py::sub(&py::mul(&v_bx.clone(), &v_cy.clone())?, &py::mul(&v_by.clone(), &v_cx.clone())?)?)?;
    v_ux = py::truediv(&py::sub(&py::mul(&v_cy.clone(), &py::add(&py::mul(&v_bx.clone(), &v_bx.clone())?, &py::mul(&v_by.clone(), &v_by.clone())?)?)?, &py::mul(&v_by.clone(), &py::add(&py::mul(&v_cx.clone(), &v_cx.clone())?, &py::mul(&v_cy.clone(), &v_cy.clone())?)?)?)?, &v_d.clone())?;
    v_uy = py::truediv(&py::sub(&py::mul(&v_bx.clone(), &py::add(&py::mul(&v_cx.clone(), &v_cx.clone())?, &py::mul(&v_cy.clone(), &v_cy.clone())?)?)?, &py::mul(&v_cx.clone(), &py::add(&py::mul(&v_bx.clone(), &v_bx.clone())?, &py::mul(&v_by.clone(), &v_by.clone())?)?)?)?, &v_d.clone())?;
    v_s = f__scale_of(Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()]))?;
    return Ok(Py::dict(vec![(Py::str("cx"), f__snap(py::add(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &v_ux.clone())?, v_s.clone())?), (Py::str("cy"), f__snap(py::add(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &v_uy.clone())?, v_s.clone())?), (Py::str("radius"), py::hypot(&[v_ux.clone(), v_uy.clone()])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triangle_classify(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_angles: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_sides: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3653 = (f__tri_check(v_a.clone(), v_b.clone(), v_c.clone())?).items()?;
    if t3653.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3653[0].clone();
    v_b = t3653[1].clone();
    v_c = t3653[2].clone();
    let t3654 = (py::sorted(&Py::tuple(vec![f__fr(v_a.clone())?, f__fr(v_b.clone())?, f__fr(v_c.clone())?]), (Py::Bool(false)).truthy())?).items()?;
    if t3654.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3654[0].clone();
    v_y = t3654[1].clone();
    v_z = t3654[2].clone();
    v_sides = (if ({ let t3655 = v_x.clone(); Py::Bool({ let t3656 = v_z.clone(); if py::eq(&t3655, &t3656) { true } else { false } }) }).truthy() { Py::str("equilateral") } else { (if ({ let t3661 = { let t3657 = v_x.clone(); Py::Bool({ let t3658 = v_y.clone(); if py::eq(&t3657, &t3658) { true } else { false } }) }; if t3661.truthy() { t3661 } else { { let t3659 = v_y.clone(); Py::Bool({ let t3660 = v_z.clone(); if py::eq(&t3659, &t3660) { true } else { false } }) } } }).truthy() { Py::str("isosceles") } else { Py::str("scalene") }) });
    v_d = py::sub(&py::add(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&v_y.clone(), &v_y.clone())?)?, &py::mul(&v_z.clone(), &v_z.clone())?)?;
    v_angles = (if ({ let t3662 = v_d.clone(); Py::Bool({ let t3663 = Py::Int(0i64); if py::eq(&t3662, &t3663) { true } else { false } }) }).truthy() { Py::str("right") } else { (if ({ let t3664 = v_d.clone(); Py::Bool({ let t3665 = Py::Int(0i64); if py::gt(&t3664, &t3665)? { true } else { false } }) }).truthy() { Py::str("acute") } else { Py::str("obtuse") }) });
    return Ok(Py::dict(vec![(Py::str("by_sides"), v_sides.clone()), (Py::str("by_angles"), v_angles.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triangle_coordinates(mut v_points: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_ang: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3706 = (f__tri_pts(v_points.clone())?).items()?;
    if t3706.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3706[0].clone();
    v_B = t3706[1].clone();
    v_C = t3706[2].clone();
    v_s = f__shoelace(Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()]))?;
    let t3708 = (Py::tuple(vec![py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_B.clone())?])).items()?;
    if t3708.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3708[0].clone();
    v_b = t3708[1].clone();
    v_c = t3708[2].clone();
    v_k = py::abs(&v_s.clone())?;
    v_ang = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_k = v_k.clone(); let v_points = v_points.clone(); let v_s = v_s.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_opp: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s1: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_s2: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::math1("degrees", &py::atan2(&py::mul(&Py::Int(4i64), &v_k.clone())?, &py::sub(&py::add(&py::mul(&v_s1.clone(), &v_s1.clone())?, &py::mul(&v_s2.clone(), &v_s2.clone())?)?, &py::mul(&v_opp.clone(), &v_opp.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("area"), v_k.clone()), (Py::str("perimeter"), py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?), (Py::str("gx"), py::truediv(&py::add(&py::add(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &py::getitem(&v_B.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_C.clone(), &Py::Int(0i64))?)?, &Py::Int(3i64))?), (Py::str("gy"), py::truediv(&py::add(&py::add(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &py::getitem(&v_B.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_C.clone(), &Py::Int(1i64))?)?, &Py::Int(3i64))?), (Py::str("angle_A"), py::call(&v_ang.clone(), &[v_a.clone(), v_b.clone(), v_c.clone()])?), (Py::str("angle_B"), py::call(&v_ang.clone(), &[v_b.clone(), v_a.clone(), v_c.clone()])?), (Py::str("angle_C"), py::call(&v_ang.clone(), &[v_c.clone(), v_a.clone(), v_b.clone()])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triangle_incircle(mut v_points: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3753 = (f__tri_pts(v_points.clone())?).items()?;
    if t3753.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3753[0].clone();
    v_B = t3753[1].clone();
    v_C = t3753[2].clone();
    let t3754 = (Py::tuple(vec![py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_B.clone())?])).items()?;
    if t3754.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3754[0].clone();
    v_b = t3754[1].clone();
    v_c = t3754[2].clone();
    v_p = py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?;
    v_k = py::abs(&f__shoelace(Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()]))?)?;
    v_s = f__scale_of(Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()]))?;
    return Ok(Py::dict(vec![(Py::str("ix"), f__snap(py::truediv(&py::add(&py::add(&py::mul(&v_a.clone(), &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_B.clone(), &Py::Int(0i64))?)?)?, &py::mul(&v_c.clone(), &py::getitem(&v_C.clone(), &Py::Int(0i64))?)?)?, &v_p.clone())?, v_s.clone())?), (Py::str("iy"), f__snap(py::truediv(&py::add(&py::add(&py::mul(&v_a.clone(), &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_B.clone(), &Py::Int(1i64))?)?)?, &py::mul(&v_c.clone(), &py::getitem(&v_C.clone(), &Py::Int(1i64))?)?)?, &v_p.clone())?, v_s.clone())?), (Py::str("radius"), py::truediv(&py::mul(&Py::Int(2i64), &v_k.clone())?, &v_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triangle_orthocenter(mut v_points: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_by: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_cy: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_hx: Py = Py::None;
    let mut v_hy: Py = Py::None;
    let mut v_m11: Py = Py::None;
    let mut v_m12: Py = Py::None;
    let mut v_m21: Py = Py::None;
    let mut v_m22: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3795 = (f__tri_pts(v_points.clone())?).items()?;
    if t3795.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3795[0].clone();
    v_B = t3795[1].clone();
    v_C = t3795[2].clone();
    let t3796 = (Py::tuple(vec![py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3796.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t3796[0].clone();
    v_by = t3796[1].clone();
    v_cx = t3796[2].clone();
    v_cy = t3796[3].clone();
    let t3797 = (Py::tuple(vec![py::sub(&v_cx.clone(), &v_bx.clone())?, py::sub(&v_cy.clone(), &v_by.clone())?, v_cx.clone(), v_cy.clone()])).items()?;
    if t3797.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_m11 = t3797[0].clone();
    v_m12 = t3797[1].clone();
    v_m21 = t3797[2].clone();
    v_m22 = t3797[3].clone();
    v_r2 = py::add(&py::mul(&v_bx.clone(), &v_cx.clone())?, &py::mul(&v_by.clone(), &v_cy.clone())?)?;
    v_det = py::sub(&py::mul(&v_m11.clone(), &v_m22.clone())?, &py::mul(&v_m12.clone(), &v_m21.clone())?)?;
    let t3798 = (Py::tuple(vec![py::truediv(&py::mul(&py::neg(&v_m12.clone())?, &v_r2.clone())?, &v_det.clone())?, py::truediv(&py::mul(&v_m11.clone(), &v_r2.clone())?, &v_det.clone())?])).items()?;
    if t3798.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_hx = t3798[0].clone();
    v_hy = t3798[1].clone();
    v_s = f__scale_of(Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()]))?;
    return Ok(Py::dict(vec![(Py::str("hx"), f__snap(py::add(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &v_hx.clone())?, v_s.clone())?), (Py::str("hy"), f__snap(py::add(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &v_hy.clone())?, v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "c1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c1".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "a2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a2".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "b2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b2".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "c2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c2".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_angle_between_lines(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t13 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t14 = (t13).items()?;
if t14.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t14[0].clone();
v_v = t14[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1017(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1028: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_gc(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1028 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1029 = v_r.clone(); Py::Bool({ let t1030 = Py::Bool(true); if py::is(&t1029, &t1030) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1035 = { let t1031 = v_r.clone(); Py::Bool({ let t1032 = Py::Bool(false); if py::is(&t1031, &t1032) { true } else { false } }) }; if t1035.truthy() { t1035 } else { { let t1033 = v_r.clone(); Py::Bool({ let t1034 = Py::None; if py::is(&t1033, &t1034) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1036 = v_r.clone(); Py::Bool({ let t1037 = v_r.clone(); if py::eq(&t1036, &t1037) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1038(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_hausdorff_distance(a0, a1)? };
    return Ok({ let mut out = vec![]; for t1053 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1054 = (t1053).items()?;
if t1054.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1054[0].clone();
v_v = t1054[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1056(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1074: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_haus(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1074 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1075 = v_r.clone(); Py::Bool({ let t1076 = Py::Bool(true); if py::is(&t1075, &t1076) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1081 = { let t1077 = v_r.clone(); Py::Bool({ let t1078 = Py::Bool(false); if py::is(&t1077, &t1078) { true } else { false } }) }; if t1081.truthy() { t1081 } else { { let t1079 = v_r.clone(); Py::Bool({ let t1080 = Py::None; if py::is(&t1079, &t1080) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1082 = v_r.clone(); Py::Bool({ let t1083 = v_r.clone(); if py::eq(&t1082, &t1083) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1084(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "pitch") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument pitch".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_helix_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t1090 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1091 = (t1090).items()?;
if t1091.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1091[0].clone();
v_v = t1091[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t113: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_bary(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t113 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t114 = v_r.clone(); Py::Bool({ let t115 = Py::Bool(true); if py::is(&t114, &t115) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t120 = { let t116 = v_r.clone(); Py::Bool({ let t117 = Py::Bool(false); if py::is(&t116, &t117) { true } else { false } }) }; if t120.truthy() { t120 } else { { let t118 = v_r.clone(); Py::Bool({ let t119 = Py::None; if py::is(&t118, &t119) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t121 = v_r.clone(); Py::Bool({ let t122 = v_r.clone(); if py::eq(&t121, &t122) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1093(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1107: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1094(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1107 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1108 = v_r.clone(); Py::Bool({ let t1109 = Py::Bool(true); if py::is(&t1108, &t1109) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1114 = { let t1110 = v_r.clone(); Py::Bool({ let t1111 = Py::Bool(false); if py::is(&t1110, &t1111) { true } else { false } }) }; if t1114.truthy() { t1114 } else { { let t1112 = v_r.clone(); Py::Bool({ let t1113 = Py::None; if py::is(&t1112, &t1113) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1115 = v_r.clone(); Py::Bool({ let t1116 = v_r.clone(); if py::eq(&t1115, &t1116) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1094(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("curvature"))?, &py::add(&py::mul(&v_r.clone(), &v_r.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?, v_r.clone()]), Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("torsion"))?, &py::add(&py::mul(&v_r.clone(), &v_r.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?, v_c.clone()]), Py::tuple(vec![py::pow(&py::getitem(&v_o.clone(), &Py::str("length_per_turn"))?, &Py::Int(2i64))?, py::mul(&py::mul(&Py::Int(4i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &py::add(&py::mul(&v_r.clone(), &v_r.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&v_i.clone(), &Py::str("radius"))?, py::truediv(&py::getitem(&v_i.clone(), &Py::str("pitch"))?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1117(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "lat1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument lat1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "lon1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument lon1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "lat2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument lat2".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "lon2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument lon2".into())) } } }; if v.len() > 4 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_initial_bearing(a0, a1, a2, a3)? };
    return Ok({ let mut out = vec![]; for t1140 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1141 = (t1140).items()?;
if t1141.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1141[0].clone();
v_v = t1141[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1143(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1149: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_bearing(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1149 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1150 = v_r.clone(); Py::Bool({ let t1151 = Py::Bool(true); if py::is(&t1150, &t1151) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1156 = { let t1152 = v_r.clone(); Py::Bool({ let t1153 = Py::Bool(false); if py::is(&t1152, &t1153) { true } else { false } }) }; if t1156.truthy() { t1156 } else { { let t1154 = v_r.clone(); Py::Bool({ let t1155 = Py::None; if py::is(&t1154, &t1155) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1157 = v_r.clone(); Py::Bool({ let t1158 = v_r.clone(); if py::eq(&t1157, &t1158) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1159(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "point") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument point".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "second_point") { Some(x) => x, None => Py::None } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "direction") { Some(x) => x, None => Py::None } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "t") { Some(x) => x, None => Py::None } }; if v.len() > 4 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_line_in_space(a0, a1, a2, a3)? };
    return Ok({ let mut out = vec![]; for t1221 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1222 = (t1221).items()?;
if t1222.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1222[0].clone();
v_v = t1222[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1223(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1250: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_line3(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1250 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1251 = v_r.clone(); Py::Bool({ let t1252 = Py::Bool(true); if py::is(&t1251, &t1252) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1257 = { let t1253 = v_r.clone(); Py::Bool({ let t1254 = Py::Bool(false); if py::is(&t1253, &t1254) { true } else { false } }) }; if t1257.truthy() { t1257 } else { { let t1255 = v_r.clone(); Py::Bool({ let t1256 = Py::None; if py::is(&t1255, &t1256) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1258 = v_r.clone(); Py::Bool({ let t1259 = v_r.clone(); if py::eq(&t1258, &t1259) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_123(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "x1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "y1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "r1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument r1".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "x2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x2".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "y2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y2".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "r2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument r2".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_circle_circle_intersection(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t148 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t149 = (t148).items()?;
if t149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t149[0].clone();
v_v = t149[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1260(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "c1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c1".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "a2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a2".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "b2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b2".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "c2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c2".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_line_intersection(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t1275 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1276 = (t1275).items()?;
if t1276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1276[0].clone();
v_v = t1276[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1278(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1280: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1279(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1280 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1281 = v_r.clone(); Py::Bool({ let t1282 = Py::Bool(true); if py::is(&t1281, &t1282) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1287 = { let t1283 = v_r.clone(); Py::Bool({ let t1284 = Py::Bool(false); if py::is(&t1283, &t1284) { true } else { false } }) }; if t1287.truthy() { t1287 } else { { let t1285 = v_r.clone(); Py::Bool({ let t1286 = Py::None; if py::is(&t1285, &t1286) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1288 = v_r.clone(); Py::Bool({ let t1289 = v_r.clone(); if py::eq(&t1288, &t1289) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1279(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__lres(py::getitem(&v_i.clone(), &Py::str("a1"))?, py::getitem(&v_i.clone(), &Py::str("b1"))?, py::getitem(&v_i.clone(), &Py::str("c1"))?, py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?)?, f__lres(py::getitem(&v_i.clone(), &Py::str("a2"))?, py::getitem(&v_i.clone(), &Py::str("b2"))?, py::getitem(&v_i.clone(), &Py::str("c2"))?, py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1290(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "x0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x0".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "y0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y0".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "z0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument z0".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "dx") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument dx".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "dy") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument dy".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "dz") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument dz".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a7 = if 7 < v.len() { v[7].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a8 = if 8 < v.len() { v[8].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; let a9 = if 9 < v.len() { v[9].clone() } else { match py::kw_take(&mut d, "d") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d".into())) } } }; if v.len() > 10 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_line_plane_intersection(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9)? };
    return Ok({ let mut out = vec![]; for t1310 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1311 = (t1310).items()?;
if t1311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1311[0].clone();
v_v = t1311[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1313(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1318: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_lpi(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1318 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1319 = v_r.clone(); Py::Bool({ let t1320 = Py::Bool(true); if py::is(&t1319, &t1320) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1325 = { let t1321 = v_r.clone(); Py::Bool({ let t1322 = Py::Bool(false); if py::is(&t1321, &t1322) { true } else { false } }) }; if t1325.truthy() { t1325 } else { { let t1323 = v_r.clone(); Py::Bool({ let t1324 = Py::None; if py::is(&t1323, &t1324) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1326 = v_r.clone(); Py::Bool({ let t1327 = v_r.clone(); if py::eq(&t1326, &t1327) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1328(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_min_area_rectangle(a0)? };
    return Ok({ let mut out = vec![]; for t1366 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1367 = (t1366).items()?;
if t1367.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1367[0].clone();
v_v = t1367[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1369(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1388: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_mar(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1388 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1389 = v_r.clone(); Py::Bool({ let t1390 = Py::Bool(true); if py::is(&t1389, &t1390) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1395 = { let t1391 = v_r.clone(); Py::Bool({ let t1392 = Py::Bool(false); if py::is(&t1391, &t1392) { true } else { false } }) }; if t1395.truthy() { t1395 } else { { let t1393 = v_r.clone(); Py::Bool({ let t1394 = Py::None; if py::is(&t1393, &t1394) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1396 = v_r.clone(); Py::Bool({ let t1397 = v_r.clone(); if py::eq(&t1396, &t1397) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1398(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_min_enclosing_circle(a0)? };
    return Ok({ let mut out = vec![]; for t1425 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1426 = (t1425).items()?;
if t1426.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1426[0].clone();
v_v = t1426[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1428(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1445: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_mec(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1445 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1446 = v_r.clone(); Py::Bool({ let t1447 = Py::Bool(true); if py::is(&t1446, &t1447) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1452 = { let t1448 = v_r.clone(); Py::Bool({ let t1449 = Py::Bool(false); if py::is(&t1448, &t1449) { true } else { false } }) }; if t1452.truthy() { t1452 } else { { let t1450 = v_r.clone(); Py::Bool({ let t1451 = Py::None; if py::is(&t1450, &t1451) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1453 = v_r.clone(); Py::Bool({ let t1454 = v_r.clone(); if py::eq(&t1453, &t1454) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1455(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_nearest_neighbor(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t1467 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1468 = (t1467).items()?;
if t1468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1468[0].clone();
v_v = t1468[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1470(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1493: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_nn(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1493 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1494 = v_r.clone(); Py::Bool({ let t1495 = Py::Bool(true); if py::is(&t1494, &t1495) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1500 = { let t1496 = v_r.clone(); Py::Bool({ let t1497 = Py::Bool(false); if py::is(&t1496, &t1497) { true } else { false } }) }; if t1500.truthy() { t1500 } else { { let t1498 = v_r.clone(); Py::Bool({ let t1499 = Py::None; if py::is(&t1498, &t1499) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1501 = v_r.clone(); Py::Bool({ let t1502 = v_r.clone(); if py::eq(&t1501, &t1502) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1503(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "x0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x0".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "x1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x1".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => Py::Float(0.0) } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_parabola_arc_length(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t1512 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1513 = (t1512).items()?;
if t1513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1513[0].clone();
v_v = t1513[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_151(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t177: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_cc(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t177 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t178 = v_r.clone(); Py::Bool({ let t179 = Py::Bool(true); if py::is(&t178, &t179) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t184 = { let t180 = v_r.clone(); Py::Bool({ let t181 = Py::Bool(false); if py::is(&t180, &t181) { true } else { false } }) }; if t184.truthy() { t184 } else { { let t182 = v_r.clone(); Py::Bool({ let t183 = Py::None; if py::is(&t182, &t183) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t185 = v_r.clone(); Py::Bool({ let t186 = v_r.clone(); if py::eq(&t185, &t186) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1515(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1533: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_parab(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1533 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1534 = v_r.clone(); Py::Bool({ let t1535 = Py::Bool(true); if py::is(&t1534, &t1535) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1540 = { let t1536 = v_r.clone(); Py::Bool({ let t1537 = Py::Bool(false); if py::is(&t1536, &t1537) { true } else { false } }) }; if t1540.truthy() { t1540 } else { { let t1538 = v_r.clone(); Py::Bool({ let t1539 = Py::None; if py::is(&t1538, &t1539) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1541 = v_r.clone(); Py::Bool({ let t1542 = v_r.clone(); if py::eq(&t1541, &t1542) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1543(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_pick_counts(a0)? };
    return Ok({ let mut out = vec![]; for t1634 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1635 = (t1634).items()?;
if t1635.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1635[0].clone();
v_v = t1635[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_16(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t21: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_angle_lines(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t21 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t22 = v_r.clone(); Py::Bool({ let t23 = Py::Bool(true); if py::is(&t22, &t23) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t28 = { let t24 = v_r.clone(); Py::Bool({ let t25 = Py::Bool(false); if py::is(&t24, &t25) { true } else { false } }) }; if t28.truthy() { t28 } else { { let t26 = v_r.clone(); Py::Bool({ let t27 = Py::None; if py::is(&t26, &t27) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t29 = v_r.clone(); Py::Bool({ let t30 = v_r.clone(); if py::eq(&t29, &t30) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1637(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1664: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_pick(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1664 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1665 = v_r.clone(); Py::Bool({ let t1666 = Py::Bool(true); if py::is(&t1665, &t1666) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1671 = { let t1667 = v_r.clone(); Py::Bool({ let t1668 = Py::Bool(false); if py::is(&t1667, &t1668) { true } else { false } }) }; if t1671.truthy() { t1671 } else { { let t1669 = v_r.clone(); Py::Bool({ let t1670 = Py::None; if py::is(&t1669, &t1670) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1672 = v_r.clone(); Py::Bool({ let t1673 = v_r.clone(); if py::eq(&t1672, &t1673) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1674(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_plane_through_points(a0)? };
    return Ok({ let mut out = vec![]; for t1692 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1693 = (t1692).items()?;
if t1693.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1693[0].clone();
v_v = t1693[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1695(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1705: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_plane3(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1705 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1706 = v_r.clone(); Py::Bool({ let t1707 = Py::Bool(true); if py::is(&t1706, &t1707) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1712 = { let t1708 = v_r.clone(); Py::Bool({ let t1709 = Py::Bool(false); if py::is(&t1708, &t1709) { true } else { false } }) }; if t1712.truthy() { t1712 } else { { let t1710 = v_r.clone(); Py::Bool({ let t1711 = Py::None; if py::is(&t1710, &t1711) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1713 = v_r.clone(); Py::Bool({ let t1714 = v_r.clone(); if py::eq(&t1713, &t1714) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1715(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "solid") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument solid".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "edge") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument edge".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_platonic_solid(a0, a1)? };
    return Ok({ let mut out = vec![]; for t1723 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1724 = (t1723).items()?;
if t1724.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1724[0].clone();
v_v = t1724[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1726(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1742: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_platonic(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1742 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1743 = v_r.clone(); Py::Bool({ let t1744 = Py::Bool(true); if py::is(&t1743, &t1744) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1749 = { let t1745 = v_r.clone(); Py::Bool({ let t1746 = Py::Bool(false); if py::is(&t1745, &t1746) { true } else { false } }) }; if t1749.truthy() { t1749 } else { { let t1747 = v_r.clone(); Py::Bool({ let t1748 = Py::None; if py::is(&t1747, &t1748) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1750 = v_r.clone(); Py::Bool({ let t1751 = v_r.clone(); if py::eq(&t1750, &t1751) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1752(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_point_in_polygon(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t1850 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1851 = (t1850).items()?;
if t1851.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1851[0].clone();
v_v = t1851[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1853(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1877: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_pip(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1877 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1878 = v_r.clone(); Py::Bool({ let t1879 = Py::Bool(true); if py::is(&t1878, &t1879) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1884 = { let t1880 = v_r.clone(); Py::Bool({ let t1881 = Py::Bool(false); if py::is(&t1880, &t1881) { true } else { false } }) }; if t1884.truthy() { t1884 } else { { let t1882 = v_r.clone(); Py::Bool({ let t1883 = Py::None; if py::is(&t1882, &t1883) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1885 = v_r.clone(); Py::Bool({ let t1886 = v_r.clone(); if py::eq(&t1885, &t1886) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_187(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "cx") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument cx".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "cy") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument cy".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "r") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument r".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_circle_line_intersection(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t211 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t212 = (t211).items()?;
if t212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t212[0].clone();
v_v = t212[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1887(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "pz") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument pz".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "d") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d".into())) } } }; if v.len() > 7 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_point_plane_distance(a0, a1, a2, a3, a4, a5, a6)? };
    return Ok({ let mut out = vec![]; for t1902 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1903 = (t1902).items()?;
if t1903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1903[0].clone();
v_v = t1903[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1905(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1908: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_ppd(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1908 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1909 = v_r.clone(); Py::Bool({ let t1910 = Py::Bool(true); if py::is(&t1909, &t1910) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1915 = { let t1911 = v_r.clone(); Py::Bool({ let t1912 = Py::Bool(false); if py::is(&t1911, &t1912) { true } else { false } }) }; if t1915.truthy() { t1915 } else { { let t1913 = v_r.clone(); Py::Bool({ let t1914 = Py::None; if py::is(&t1913, &t1914) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1916 = v_r.clone(); Py::Bool({ let t1917 = v_r.clone(); if py::eq(&t1916, &t1917) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1918(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "x1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x1".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "y1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y1".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "x2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x2".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "y2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y2".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_point_segment_distance(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t1930 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1931 = (t1930).items()?;
if t1931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1931[0].clone();
v_v = t1931[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1933(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1943: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_psd(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1943 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1944 = v_r.clone(); Py::Bool({ let t1945 = Py::Bool(true); if py::is(&t1944, &t1945) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1950 = { let t1946 = v_r.clone(); Py::Bool({ let t1947 = Py::Bool(false); if py::is(&t1946, &t1947) { true } else { false } }) }; if t1950.truthy() { t1950 } else { { let t1948 = v_r.clone(); Py::Bool({ let t1949 = Py::None; if py::is(&t1948, &t1949) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1951 = v_r.clone(); Py::Bool({ let t1952 = v_r.clone(); if py::eq(&t1951, &t1952) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1953(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polygon_area(a0)? };
    return Ok({ let mut out = vec![]; for t2039 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2040 = (t2039).items()?;
if t2040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2040[0].clone();
v_v = t2040[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2042(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2060: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_polygon_area(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2060 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2061 = v_r.clone(); Py::Bool({ let t2062 = Py::Bool(true); if py::is(&t2061, &t2062) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2067 = { let t2063 = v_r.clone(); Py::Bool({ let t2064 = Py::Bool(false); if py::is(&t2063, &t2064) { true } else { false } }) }; if t2067.truthy() { t2067 } else { { let t2065 = v_r.clone(); Py::Bool({ let t2066 = Py::None; if py::is(&t2065, &t2066) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2068 = v_r.clone(); Py::Bool({ let t2069 = v_r.clone(); if py::eq(&t2068, &t2069) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2070(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polygon_centroid(a0)? };
    return Ok({ let mut out = vec![]; for t2161 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2162 = (t2161).items()?;
if t2162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2162[0].clone();
v_v = t2162[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_214(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t235: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_circle_line(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t235 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t236 = v_r.clone(); Py::Bool({ let t237 = Py::Bool(true); if py::is(&t236, &t237) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t242 = { let t238 = v_r.clone(); Py::Bool({ let t239 = Py::Bool(false); if py::is(&t238, &t239) { true } else { false } }) }; if t242.truthy() { t242 } else { { let t240 = v_r.clone(); Py::Bool({ let t241 = Py::None; if py::is(&t240, &t241) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t243 = v_r.clone(); Py::Bool({ let t244 = v_r.clone(); if py::eq(&t243, &t244) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2164(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2172: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_centroid(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2172 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2173 = v_r.clone(); Py::Bool({ let t2174 = Py::Bool(true); if py::is(&t2173, &t2174) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2179 = { let t2175 = v_r.clone(); Py::Bool({ let t2176 = Py::Bool(false); if py::is(&t2175, &t2176) { true } else { false } }) }; if t2179.truthy() { t2179 } else { { let t2177 = v_r.clone(); Py::Bool({ let t2178 = Py::None; if py::is(&t2177, &t2178) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2180 = v_r.clone(); Py::Bool({ let t2181 = v_r.clone(); if py::eq(&t2180, &t2181) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2182(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polygon_angles(a0)? };
    return Ok({ let mut out = vec![]; for t2274 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2275 = (t2274).items()?;
if t2275.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2275[0].clone();
v_v = t2275[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2276(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2304: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_pangles(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2304 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2305 = v_r.clone(); Py::Bool({ let t2306 = Py::Bool(true); if py::is(&t2305, &t2306) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2311 = { let t2307 = v_r.clone(); Py::Bool({ let t2308 = Py::Bool(false); if py::is(&t2307, &t2308) { true } else { false } }) }; if t2311.truthy() { t2311 } else { { let t2309 = v_r.clone(); Py::Bool({ let t2310 = Py::None; if py::is(&t2309, &t2310) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2312 = v_r.clone(); Py::Bool({ let t2313 = v_r.clone(); if py::eq(&t2312, &t2313) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2314(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polygon_is_convex(a0)? };
    return Ok({ let mut out = vec![]; for t2404 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2405 = (t2404).items()?;
if t2405.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2405[0].clone();
v_v = t2405[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2407(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2422: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_convex(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2422 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2423 = v_r.clone(); Py::Bool({ let t2424 = Py::Bool(true); if py::is(&t2423, &t2424) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2429 = { let t2425 = v_r.clone(); Py::Bool({ let t2426 = Py::Bool(false); if py::is(&t2425, &t2426) { true } else { false } }) }; if t2429.truthy() { t2429 } else { { let t2427 = v_r.clone(); Py::Bool({ let t2428 = Py::None; if py::is(&t2427, &t2428) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2430 = v_r.clone(); Py::Bool({ let t2431 = v_r.clone(); if py::eq(&t2430, &t2431) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2432(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polyline_length(a0)? };
    return Ok({ let mut out = vec![]; for t2445 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2446 = (t2445).items()?;
if t2446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2446[0].clone();
v_v = t2446[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2448(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2464: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2449(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2464 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2465 = v_r.clone(); Py::Bool({ let t2466 = Py::Bool(true); if py::is(&t2465, &t2466) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2471 = { let t2467 = v_r.clone(); Py::Bool({ let t2468 = Py::Bool(false); if py::is(&t2467, &t2468) { true } else { false } }) }; if t2471.truthy() { t2471 } else { { let t2469 = v_r.clone(); Py::Bool({ let t2470 = Py::None; if py::is(&t2469, &t2470) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2472 = v_r.clone(); Py::Bool({ let t2473 = v_r.clone(); if py::eq(&t2472, &t2473) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2449(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("length"))?, py::sum(&{ let mut out = vec![]; for t2462 in py::iter(&(py::zip(&[v_p.clone(), py::slice(&v_p.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2463 = (t2462).items()?;
if t2463.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2463[0].clone();
v_b = t2463[1].clone();
out.push(py::math1("sqrt", &py::add(&py::pow(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?)?); } Py::list(out) }, None)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("segments"))?, py::sub(&py::len(&v_p.clone())?, &Py::Int(1i64))?])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(2i64), Py::None)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_245(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_circle_measures(a0)? };
    return Ok({ let mut out = vec![]; for t250 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t251 = (t250).items()?;
if t251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t251[0].clone();
v_v = t251[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2474(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "length") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument length".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "width") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument width".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_pyramid_measures(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t2480 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2481 = (t2480).items()?;
if t2481.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2481[0].clone();
v_v = t2481[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2483(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2497: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2484(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2497 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2498 = v_r.clone(); Py::Bool({ let t2499 = Py::Bool(true); if py::is(&t2498, &t2499) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2504 = { let t2500 = v_r.clone(); Py::Bool({ let t2501 = Py::Bool(false); if py::is(&t2500, &t2501) { true } else { false } }) }; if t2504.truthy() { t2504 } else { { let t2502 = v_r.clone(); Py::Bool({ let t2503 = Py::None; if py::is(&t2502, &t2503) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2505 = v_r.clone(); Py::Bool({ let t2506 = v_r.clone(); if py::eq(&t2505, &t2506) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2484(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_l: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_w: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_h: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&Py::Int(3i64), &py::getitem(&v_o.clone(), &Py::str("volume"))?)?, py::mul(&py::mul(&v_l.clone(), &v_w.clone())?, &v_h.clone())?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("total_area"))?, py::add(&py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, &py::mul(&v_l.clone(), &v_w.clone())?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, py::add(&py::mul(&Py::Int(2i64), &py::mul(&py::mul(&Py::Float(0.5), &v_l.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::truediv(&py::mul(&v_w.clone(), &v_w.clone())?, &Py::Int(4i64))?)?)?)?)?, &py::mul(&Py::Int(2i64), &py::mul(&py::mul(&Py::Float(0.5), &v_w.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::truediv(&py::mul(&v_l.clone(), &v_l.clone())?, &Py::Int(4i64))?)?)?)?)?)?])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&v_i.clone(), &Py::str("length"))?, py::getitem(&v_i.clone(), &Py::str("width"))?, py::getitem(&v_i.clone(), &Py::str("height"))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2507(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "length") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument length".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "width") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument width".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_rectangle_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t2513 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2514 = (t2513).items()?;
if t2514.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2514[0].clone();
v_v = t2514[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2516(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2530: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2517(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2530 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2531 = v_r.clone(); Py::Bool({ let t2532 = Py::Bool(true); if py::is(&t2531, &t2532) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2537 = { let t2533 = v_r.clone(); Py::Bool({ let t2534 = Py::Bool(false); if py::is(&t2533, &t2534) { true } else { false } }) }; if t2537.truthy() { t2537 } else { { let t2535 = v_r.clone(); Py::Bool({ let t2536 = Py::None; if py::is(&t2535, &t2536) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2538 = v_r.clone(); Py::Bool({ let t2539 = v_r.clone(); if py::eq(&t2538, &t2539) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2517(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("length"))?, &py::getitem(&v_i.clone(), &Py::str("width"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::mul(&Py::Int(2i64), &py::add(&py::getitem(&v_i.clone(), &Py::str("length"))?, &py::getitem(&v_i.clone(), &Py::str("width"))?)?)?]), Py::tuple(vec![py::pow(&py::getitem(&v_o.clone(), &Py::str("diagonal"))?, &Py::Int(2i64))?, py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("length"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("width"))?, &Py::Int(2i64))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_253(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t267: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_254(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t267 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t268 = v_r.clone(); Py::Bool({ let t269 = Py::Bool(true); if py::is(&t268, &t269) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t274 = { let t270 = v_r.clone(); Py::Bool({ let t271 = Py::Bool(false); if py::is(&t270, &t271) { true } else { false } }) }; if t274.truthy() { t274 } else { { let t272 = v_r.clone(); Py::Bool({ let t273 = Py::None; if py::is(&t272, &t273) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t275 = v_r.clone(); Py::Bool({ let t276 = v_r.clone(); if py::eq(&t275, &t276) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_254(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("circumference"))?, py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_i.clone(), &Py::str("radius"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("diameter"))?, py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("radius"))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2540(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_reflect_point_line(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t2553 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2554 = (t2553).items()?;
if t2554.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2554[0].clone();
v_v = t2554[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2556(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2560: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_reflect(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2560 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2561 = v_r.clone(); Py::Bool({ let t2562 = Py::Bool(true); if py::is(&t2561, &t2562) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2567 = { let t2563 = v_r.clone(); Py::Bool({ let t2564 = Py::Bool(false); if py::is(&t2563, &t2564) { true } else { false } }) }; if t2567.truthy() { t2567 } else { { let t2565 = v_r.clone(); Py::Bool({ let t2566 = Py::None; if py::is(&t2565, &t2566) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2568 = v_r.clone(); Py::Bool({ let t2569 = v_r.clone(); if py::eq(&t2568, &t2569) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2570(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "n") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument n".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_regular_polygon_angles(a0)? };
    return Ok({ let mut out = vec![]; for t2578 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2579 = (t2578).items()?;
if t2579.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2579[0].clone();
v_v = t2579[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2581(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2595: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2582(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2595 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2596 = v_r.clone(); Py::Bool({ let t2597 = Py::Bool(true); if py::is(&t2596, &t2597) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2602 = { let t2598 = v_r.clone(); Py::Bool({ let t2599 = Py::Bool(false); if py::is(&t2598, &t2599) { true } else { false } }) }; if t2602.truthy() { t2602 } else { { let t2600 = v_r.clone(); Py::Bool({ let t2601 = Py::None; if py::is(&t2600, &t2601) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2603 = v_r.clone(); Py::Bool({ let t2604 = v_r.clone(); if py::eq(&t2603, &t2604) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2582(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::add(&py::getitem(&v_o.clone(), &Py::str("interior_angle"))?, &py::getitem(&v_o.clone(), &Py::str("exterior_angle"))?)?, Py::Float(180.0)]), Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("exterior_angle"))?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?, Py::Float(360.0)]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("interior_sum"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("interior_angle"))?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("diagonals"))?, py::sub(&py::comb(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &Py::Int(2i64))?, &py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2605(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "n") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument n".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "side") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument side".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_regular_polygon_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t2618 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2619 = (t2618).items()?;
if t2619.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2619[0].clone();
v_v = t2619[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2621(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2635: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2622(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2635 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2636 = v_r.clone(); Py::Bool({ let t2637 = Py::Bool(true); if py::is(&t2636, &t2637) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2642 = { let t2638 = v_r.clone(); Py::Bool({ let t2639 = Py::Bool(false); if py::is(&t2638, &t2639) { true } else { false } }) }; if t2642.truthy() { t2642 } else { { let t2640 = v_r.clone(); Py::Bool({ let t2641 = Py::None; if py::is(&t2640, &t2641) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2643 = v_r.clone(); Py::Bool({ let t2644 = v_r.clone(); if py::eq(&t2643, &t2644) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2622(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("apothem"))?, &Py::Int(2i64))?, &py::pow(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("side"))?, &Py::Int(2i64))?, &Py::Int(2i64))?)?, py::pow(&py::getitem(&v_o.clone(), &Py::str("circumradius"))?, &Py::Int(2i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("perimeter"))?, &py::getitem(&v_o.clone(), &Py::str("apothem"))?)?, &Py::Int(2i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::getitem(&v_i.clone(), &Py::str("side"))?)?]), Py::tuple(vec![py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("circumradius"))?, &py::math1("sin", &py::truediv(&Py::Float(std::f64::consts::PI), &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?)?, &Py::Int(2i64))?, py::getitem(&v_i.clone(), &Py::str("side"))?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2645(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "n") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument n".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "side") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument side".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_regular_prism(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t2658 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2659 = (t2658).items()?;
if t2659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2659[0].clone();
v_v = t2659[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2661(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2675: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2662(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2675 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2676 = v_r.clone(); Py::Bool({ let t2677 = Py::Bool(true); if py::is(&t2676, &t2677) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2682 = { let t2678 = v_r.clone(); Py::Bool({ let t2679 = Py::Bool(false); if py::is(&t2678, &t2679) { true } else { false } }) }; if t2682.truthy() { t2682 } else { { let t2680 = v_r.clone(); Py::Bool({ let t2681 = Py::None; if py::is(&t2680, &t2681) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2683 = v_r.clone(); Py::Bool({ let t2684 = v_r.clone(); if py::eq(&t2683, &t2684) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2662(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("base_area"))?, &Py::Int(4i64))?, &py::math1("tan", &py::truediv(&Py::Float(std::f64::consts::PI), &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("side"))?, &Py::Int(2i64))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("base_area"))?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::getitem(&v_i.clone(), &Py::str("side"))?)?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("total_area"))?, py::add(&py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, &py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("base_area"))?)?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2685(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "d1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "d2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d2".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_rhombus_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t2691 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2692 = (t2691).items()?;
if t2692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2692[0].clone();
v_v = t2692[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2694(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2708: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2695(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2708 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2709 = v_r.clone(); Py::Bool({ let t2710 = Py::Bool(true); if py::is(&t2709, &t2710) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2715 = { let t2711 = v_r.clone(); Py::Bool({ let t2712 = Py::Bool(false); if py::is(&t2711, &t2712) { true } else { false } }) }; if t2715.truthy() { t2715 } else { { let t2713 = v_r.clone(); Py::Bool({ let t2714 = Py::None; if py::is(&t2713, &t2714) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2716 = v_r.clone(); Py::Bool({ let t2717 = v_r.clone(); if py::eq(&t2716, &t2717) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2695(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("d1"))?, &py::getitem(&v_i.clone(), &Py::str("d2"))?)?, &Py::Int(2i64))?]), Py::tuple(vec![py::mul(&Py::Int(4i64), &py::pow(&py::getitem(&v_o.clone(), &Py::str("side"))?, &Py::Int(2i64))?)?, py::mul(&py::truediv(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("d1"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("d2"))?, &Py::Int(2i64))?)?, &Py::Int(4i64))?, &Py::Int(4i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::mul(&Py::Int(4i64), &py::getitem(&v_o.clone(), &Py::str("side"))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2718(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "angle") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument angle".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "cx") { Some(x) => x, None => Py::Float(0.0) } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "cy") { Some(x) => x, None => Py::Float(0.0) } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_rotate_point(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t2727 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2728 = (t2727).items()?;
if t2728.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2728[0].clone();
v_v = t2728[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2730(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2732: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_rotate(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2732 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2733 = v_r.clone(); Py::Bool({ let t2734 = Py::Bool(true); if py::is(&t2733, &t2734) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2739 = { let t2735 = v_r.clone(); Py::Bool({ let t2736 = Py::Bool(false); if py::is(&t2735, &t2736) { true } else { false } }) }; if t2739.truthy() { t2739 } else { { let t2737 = v_r.clone(); Py::Bool({ let t2738 = Py::None; if py::is(&t2737, &t2738) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2740 = v_r.clone(); Py::Bool({ let t2741 = v_r.clone(); if py::eq(&t2740, &t2741) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2742(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "point") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument point".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "axis") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument axis".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "angle") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument angle".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_rotate_point_3d(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t2756 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2757 = (t2756).items()?;
if t2757.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2757[0].clone();
v_v = t2757[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2759(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2766: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_rot3(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2766 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2767 = v_r.clone(); Py::Bool({ let t2768 = Py::Bool(true); if py::is(&t2767, &t2768) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2773 = { let t2769 = v_r.clone(); Py::Bool({ let t2770 = Py::Bool(false); if py::is(&t2769, &t2770) { true } else { false } }) }; if t2773.truthy() { t2773 } else { { let t2771 = v_r.clone(); Py::Bool({ let t2772 = Py::None; if py::is(&t2771, &t2772) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2774 = v_r.clone(); Py::Bool({ let t2775 = v_r.clone(); if py::eq(&t2774, &t2775) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_277(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "angle") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument angle".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_circular_segment(a0, a1)? };
    return Ok({ let mut out = vec![]; for t288 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t289 = (t288).items()?;
if t289.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t289[0].clone();
v_v = t289[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2776(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "angle") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument angle".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_sector_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t2785 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2786 = (t2785).items()?;
if t2786.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2786[0].clone();
v_v = t2786[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2788(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2802: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2789(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2802 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2803 = v_r.clone(); Py::Bool({ let t2804 = Py::Bool(true); if py::is(&t2803, &t2804) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2809 = { let t2805 = v_r.clone(); Py::Bool({ let t2806 = Py::Bool(false); if py::is(&t2805, &t2806) { true } else { false } }) }; if t2809.truthy() { t2809 } else { { let t2807 = v_r.clone(); Py::Bool({ let t2808 = Py::None; if py::is(&t2807, &t2808) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2810 = v_r.clone(); Py::Bool({ let t2811 = v_r.clone(); if py::eq(&t2810, &t2811) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2789(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("arc_length"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("angle"))?)?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("sector_area"))?, py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("arc_length"))?, &py::getitem(&v_i.clone(), &Py::str("radius"))?)?, &Py::Int(2i64))?]), Py::tuple(vec![py::pow(&py::getitem(&v_o.clone(), &Py::str("chord"))?, &Py::Int(2i64))?, py::mul(&py::mul(&Py::Int(2i64), &py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?, &py::sub(&Py::Int(1i64), &py::math1("cos", &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("angle"))?)?)?)?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2812(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_segment_intersection(a0)? };
    return Ok({ let mut out = vec![]; for t2881 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2882 = (t2881).items()?;
if t2882.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2882[0].clone();
v_v = t2882[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2884(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2914: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_seg(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2914 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2915 = v_r.clone(); Py::Bool({ let t2916 = Py::Bool(true); if py::is(&t2915, &t2916) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2921 = { let t2917 = v_r.clone(); Py::Bool({ let t2918 = Py::Bool(false); if py::is(&t2917, &t2918) { true } else { false } }) }; if t2921.truthy() { t2921 } else { { let t2919 = v_r.clone(); Py::Bool({ let t2920 = Py::None; if py::is(&t2919, &t2920) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2922 = v_r.clone(); Py::Bool({ let t2923 = v_r.clone(); if py::eq(&t2922, &t2923) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_291(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t305: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_292(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t305 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t306 = v_r.clone(); Py::Bool({ let t307 = Py::Bool(true); if py::is(&t306, &t307) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t312 = { let t308 = v_r.clone(); Py::Bool({ let t309 = Py::Bool(false); if py::is(&t308, &t309) { true } else { false } }) }; if t312.truthy() { t312 } else { { let t310 = v_r.clone(); Py::Bool({ let t311 = Py::None; if py::is(&t310, &t311) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t313 = v_r.clone(); Py::Bool({ let t314 = v_r.clone(); if py::eq(&t313, &t314) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_292(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_t: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::add(&py::getitem(&v_o.clone(), &Py::str("area"))?, &py::truediv(&py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &py::math1("sin", &v_t.clone())?)?, &Py::Int(2i64))?)?, py::truediv(&py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &v_t.clone())?, &Py::Int(2i64))?]), Py::tuple(vec![py::pow(&py::getitem(&v_o.clone(), &Py::str("chord"))?, &Py::Int(2i64))?, py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_r.clone())?, &v_r.clone())?, &py::sub(&Py::Int(1i64), &py::math1("cos", &v_t.clone())?)?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("height"))?, py::sub(&v_r.clone(), &py::mul(&v_r.clone(), &py::math1("cos", &py::truediv(&v_t.clone(), &Py::Int(2i64))?)?)?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("arc_length"))?, py::mul(&v_r.clone(), &v_t.clone())?])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&v_i.clone(), &Py::str("radius"))?, py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("angle"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2924(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "measure") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument measure".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "m1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument m1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "m2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument m2".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "other_length") { Some(x) => x, None => Py::None } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "other_area") { Some(x) => x, None => Py::None } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "other_volume") { Some(x) => x, None => Py::None } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_similar_figures(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t2945 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2946 = (t2945).items()?;
if t2946.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2946[0].clone();
v_v = t2946[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2948(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2959: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_similar(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2959 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2960 = v_r.clone(); Py::Bool({ let t2961 = Py::Bool(true); if py::is(&t2960, &t2961) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2966 = { let t2962 = v_r.clone(); Py::Bool({ let t2963 = Py::Bool(false); if py::is(&t2962, &t2963) { true } else { false } }) }; if t2966.truthy() { t2966 } else { { let t2964 = v_r.clone(); Py::Bool({ let t2965 = Py::None; if py::is(&t2964, &t2965) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2967 = v_r.clone(); Py::Bool({ let t2968 = v_r.clone(); if py::eq(&t2967, &t2968) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2969(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "p1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument p1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "d1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "p2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument p2".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "d2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d2".into())) } } }; if v.len() > 4 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_skew_lines_distance(a0, a1, a2, a3)? };
    return Ok({ let mut out = vec![]; for t2984 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2985 = (t2984).items()?;
if t2985.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2985[0].clone();
v_v = t2985[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2987(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2997: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_skew(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2997 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2998 = v_r.clone(); Py::Bool({ let t2999 = Py::Bool(true); if py::is(&t2998, &t2999) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3004 = { let t3000 = v_r.clone(); Py::Bool({ let t3001 = Py::Bool(false); if py::is(&t3000, &t3001) { true } else { false } }) }; if t3004.truthy() { t3004 } else { { let t3002 = v_r.clone(); Py::Bool({ let t3003 = Py::None; if py::is(&t3002, &t3003) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3005 = v_r.clone(); Py::Bool({ let t3006 = v_r.clone(); if py::eq(&t3005, &t3006) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3007(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "half_angle") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument half_angle".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_solid_angle_cone(a0)? };
    return Ok({ let mut out = vec![]; for t3013 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3014 = (t3013).items()?;
if t3014.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3014[0].clone();
v_v = t3014[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3016(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3030: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3017(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3030 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3031 = v_r.clone(); Py::Bool({ let t3032 = Py::Bool(true); if py::is(&t3031, &t3032) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3037 = { let t3033 = v_r.clone(); Py::Bool({ let t3034 = Py::Bool(false); if py::is(&t3033, &t3034) { true } else { false } }) }; if t3037.truthy() { t3037 } else { { let t3035 = v_r.clone(); Py::Bool({ let t3036 = Py::None; if py::is(&t3035, &t3036) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3038 = v_r.clone(); Py::Bool({ let t3039 = v_r.clone(); if py::eq(&t3038, &t3039) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3017(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("solid_angle"))?, py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::sub(&Py::Int(1i64), &py::math1("cos", &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("half_angle"))?)?)?)?)?]), Py::tuple(vec![py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("fraction_of_sphere"))?, &Py::Int(4i64))?, &Py::Float(std::f64::consts::PI))?, py::getitem(&v_o.clone(), &Py::str("solid_angle"))?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3040(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_solid_angle_triangle(a0)? };
    return Ok({ let mut out = vec![]; for t3054 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3055 = (t3054).items()?;
if t3055.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3055[0].clone();
v_v = t3055[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3057(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3071: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_solid_tri(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3071 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3072 = v_r.clone(); Py::Bool({ let t3073 = Py::Bool(true); if py::is(&t3072, &t3073) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3078 = { let t3074 = v_r.clone(); Py::Bool({ let t3075 = Py::Bool(false); if py::is(&t3074, &t3075) { true } else { false } }) }; if t3078.truthy() { t3078 } else { { let t3076 = v_r.clone(); Py::Bool({ let t3077 = Py::None; if py::is(&t3076, &t3077) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3079 = v_r.clone(); Py::Bool({ let t3080 = v_r.clone(); if py::eq(&t3079, &t3080) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3081(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_sphere_four_points(a0)? };
    return Ok({ let mut out = vec![]; for t3100 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3101 = (t3100).items()?;
if t3101.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3101[0].clone();
v_v = t3101[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_31(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "c1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c1".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "a2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a2".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "b2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b2".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "c2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c2".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_angle_between_planes(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t41 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t42 = (t41).items()?;
if t42.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t42[0].clone();
v_v = t42[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3103(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3106: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3104(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3106 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3107 = v_r.clone(); Py::Bool({ let t3108 = Py::Bool(true); if py::is(&t3107, &t3108) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3113 = { let t3109 = v_r.clone(); Py::Bool({ let t3110 = Py::Bool(false); if py::is(&t3109, &t3110) { true } else { false } }) }; if t3113.truthy() { t3113 } else { { let t3111 = v_r.clone(); Py::Bool({ let t3112 = Py::None; if py::is(&t3111, &t3112) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3114 = v_r.clone(); Py::Bool({ let t3115 = v_r.clone(); if py::eq(&t3114, &t3115) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3104(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_P: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[{ let mut out = vec![]; for t3105 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t3105;
out.push(py::truediv(&py::abs(&py::sub(&py::dist(&Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("cx"))?, py::getitem(&v_o.clone(), &Py::str("cy"))?, py::getitem(&v_o.clone(), &Py::str("cz"))?]), &v_p.clone())?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?); } Py::list(out) }], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__triples(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(4i64))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3116(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_sphere_measures(a0)? };
    return Ok({ let mut out = vec![]; for t3121 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3122 = (t3121).items()?;
if t3122.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3122[0].clone();
v_v = t3122[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3124(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3138: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3125(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3138 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3139 = v_r.clone(); Py::Bool({ let t3140 = Py::Bool(true); if py::is(&t3139, &t3140) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3145 = { let t3141 = v_r.clone(); Py::Bool({ let t3142 = Py::Bool(false); if py::is(&t3141, &t3142) { true } else { false } }) }; if t3145.truthy() { t3145 } else { { let t3143 = v_r.clone(); Py::Bool({ let t3144 = Py::None; if py::is(&t3143, &t3144) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3146 = v_r.clone(); Py::Bool({ let t3147 = v_r.clone(); if py::eq(&t3146, &t3147) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3125(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("volume"))?, py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(3i64))?)?, &Py::Int(3i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("surface_area"))?, py::truediv(&py::mul(&Py::Int(3i64), &py::getitem(&v_o.clone(), &Py::str("volume"))?)?, &py::getitem(&v_i.clone(), &Py::str("radius"))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3148(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_spherical_cap(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3156 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3157 = (t3156).items()?;
if t3157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3157[0].clone();
v_v = t3157[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_315(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_closest_pair(a0)? };
    return Ok({ let mut out = vec![]; for t334 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t335 = (t334).items()?;
if t335.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t335[0].clone();
v_v = t335[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3159(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3173: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3160(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3173 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3174 = v_r.clone(); Py::Bool({ let t3175 = Py::Bool(true); if py::is(&t3174, &t3175) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3180 = { let t3176 = v_r.clone(); Py::Bool({ let t3177 = Py::Bool(false); if py::is(&t3176, &t3177) { true } else { false } }) }; if t3180.truthy() { t3180 } else { { let t3178 = v_r.clone(); Py::Bool({ let t3179 = Py::None; if py::is(&t3178, &t3179) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3181 = v_r.clone(); Py::Bool({ let t3182 = v_r.clone(); if py::eq(&t3181, &t3182) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3160(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_R: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_h: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&Py::Int(3i64), &py::getitem(&v_o.clone(), &Py::str("volume"))?)?, py::mul(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_h.clone())?, &v_h.clone())?, &py::sub(&py::mul(&Py::Int(3i64), &v_R.clone())?, &v_h.clone())?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("cap_area"))?, py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_R.clone())?, &v_h.clone())?]), Py::tuple(vec![py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("base_radius"))?, &Py::Int(2i64))?, &py::pow(&py::sub(&v_R.clone(), &v_h.clone())?, &Py::Int(2i64))?)?, py::mul(&v_R.clone(), &v_R.clone())?])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&v_i.clone(), &Py::str("radius"))?, py::getitem(&v_i.clone(), &Py::str("height"))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3183(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => Py::Float(1.0) } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_spherical_triangle(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3214 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3215 = (t3214).items()?;
if t3215.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3215[0].clone();
v_v = t3215[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3217(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3233: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_sph(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3233 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3234 = v_r.clone(); Py::Bool({ let t3235 = Py::Bool(true); if py::is(&t3234, &t3235) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3240 = { let t3236 = v_r.clone(); Py::Bool({ let t3237 = Py::Bool(false); if py::is(&t3236, &t3237) { true } else { false } }) }; if t3240.truthy() { t3240 } else { { let t3238 = v_r.clone(); Py::Bool({ let t3239 = Py::None; if py::is(&t3238, &t3239) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3241 = v_r.clone(); Py::Bool({ let t3242 = v_r.clone(); if py::eq(&t3241, &t3242) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3243(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_spheroid_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3257 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3258 = (t3257).items()?;
if t3258.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3258[0].clone();
v_v = t3258[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3260(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3275: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_spheroid(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3275 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3276 = v_r.clone(); Py::Bool({ let t3277 = Py::Bool(true); if py::is(&t3276, &t3277) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3282 = { let t3278 = v_r.clone(); Py::Bool({ let t3279 = Py::Bool(false); if py::is(&t3278, &t3279) { true } else { false } }) }; if t3282.truthy() { t3282 } else { { let t3280 = v_r.clone(); Py::Bool({ let t3281 = Py::None; if py::is(&t3280, &t3281) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3283 = v_r.clone(); Py::Bool({ let t3284 = v_r.clone(); if py::eq(&t3283, &t3284) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3285(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument A".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "B") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument B".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "C") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument C".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "D") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument D".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "E") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument E".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "x0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x0".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "y0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y0".into())) } } }; let a7 = if 7 < v.len() { v[7].clone() } else { match py::kw_take(&mut d, "F") { Some(x) => x, None => Py::Float(0.0) } }; if v.len() > 8 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_surface_curvature(a0, a1, a2, a3, a4, a5, a6, a7)? };
    return Ok({ let mut out = vec![]; for t3292 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3293 = (t3292).items()?;
if t3293.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3293[0].clone();
v_v = t3293[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3295(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3301: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_surf(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3301 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3302 = v_r.clone(); Py::Bool({ let t3303 = Py::Bool(true); if py::is(&t3302, &t3303) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3308 = { let t3304 = v_r.clone(); Py::Bool({ let t3305 = Py::Bool(false); if py::is(&t3304, &t3305) { true } else { false } }) }; if t3308.truthy() { t3308 } else { { let t3306 = v_r.clone(); Py::Bool({ let t3307 = Py::None; if py::is(&t3306, &t3307) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3309 = v_r.clone(); Py::Bool({ let t3310 = v_r.clone(); if py::eq(&t3309, &t3310) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3311(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "cx") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument cx".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "cy") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument cy".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "r") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument r".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_tangent_from_point(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t3323 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3324 = (t3323).items()?;
if t3324.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3324[0].clone();
v_v = t3324[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3326(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3329: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_tangent(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3329 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3330 = v_r.clone(); Py::Bool({ let t3331 = Py::Bool(true); if py::is(&t3330, &t3331) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3336 = { let t3332 = v_r.clone(); Py::Bool({ let t3333 = Py::Bool(false); if py::is(&t3332, &t3333) { true } else { false } }) }; if t3336.truthy() { t3336 } else { { let t3334 = v_r.clone(); Py::Bool({ let t3335 = Py::None; if py::is(&t3334, &t3335) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3337 = v_r.clone(); Py::Bool({ let t3338 = v_r.clone(); if py::eq(&t3337, &t3338) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3339(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_tetrahedron_measures(a0)? };
    return Ok({ let mut out = vec![]; for t3352 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3353 = (t3352).items()?;
if t3353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3353[0].clone();
v_v = t3353[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3355(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3379: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_tetra(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3379 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3380 = v_r.clone(); Py::Bool({ let t3381 = Py::Bool(true); if py::is(&t3380, &t3381) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3386 = { let t3382 = v_r.clone(); Py::Bool({ let t3383 = Py::Bool(false); if py::is(&t3382, &t3383) { true } else { false } }) }; if t3386.truthy() { t3386 } else { { let t3384 = v_r.clone(); Py::Bool({ let t3385 = Py::None; if py::is(&t3384, &t3385) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3387 = v_r.clone(); Py::Bool({ let t3388 = v_r.clone(); if py::eq(&t3387, &t3388) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_337(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t342: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_closest(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t342 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t343 = v_r.clone(); Py::Bool({ let t344 = Py::Bool(true); if py::is(&t343, &t344) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t349 = { let t345 = v_r.clone(); Py::Bool({ let t346 = Py::Bool(false); if py::is(&t345, &t346) { true } else { false } }) }; if t349.truthy() { t349 } else { { let t347 = v_r.clone(); Py::Bool({ let t348 = Py::None; if py::is(&t347, &t348) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t350 = v_r.clone(); Py::Bool({ let t351 = v_r.clone(); if py::eq(&t350, &t351) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3389(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "major") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument major".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "minor") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument minor".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "v") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument v".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_torus_curvature(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t3398 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3399 = (t3398).items()?;
if t3399.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3399[0].clone();
v_v = t3399[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3401(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3415: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3402(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3415 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3416 = v_r.clone(); Py::Bool({ let t3417 = Py::Bool(true); if py::is(&t3416, &t3417) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3422 = { let t3418 = v_r.clone(); Py::Bool({ let t3419 = Py::Bool(false); if py::is(&t3418, &t3419) { true } else { false } }) }; if t3422.truthy() { t3422 } else { { let t3420 = v_r.clone(); Py::Bool({ let t3421 = Py::None; if py::is(&t3420, &t3421) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3423 = v_r.clone(); Py::Bool({ let t3424 = v_r.clone(); if py::eq(&t3423, &t3424) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3402(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_R: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_r: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("gaussian"))?, &v_r.clone())?, &py::add(&v_R.clone(), &py::mul(&v_r.clone(), &v_c.clone())?)?)?, v_c.clone()]), Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("k1"))?, &v_r.clone())?, Py::Float(1.0)]), Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("k2"))?, &py::add(&v_R.clone(), &py::mul(&v_r.clone(), &v_c.clone())?)?)?, v_c.clone()]), Py::tuple(vec![py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("mean"))?)?, py::add(&py::getitem(&v_o.clone(), &Py::str("k1"))?, &py::getitem(&v_o.clone(), &Py::str("k2"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("gaussian"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("k1"))?, &py::getitem(&v_o.clone(), &Py::str("k2"))?)?])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&v_i.clone(), &Py::str("major"))?, py::getitem(&v_i.clone(), &Py::str("minor"))?, py::math1("cos", &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("v"))?)?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3425(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "major") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument major".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "minor") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument minor".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_torus_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3433 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3434 = (t3433).items()?;
if t3434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3434[0].clone();
v_v = t3434[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3436(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3450: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3437(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3450 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3451 = v_r.clone(); Py::Bool({ let t3452 = Py::Bool(true); if py::is(&t3451, &t3452) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3457 = { let t3453 = v_r.clone(); Py::Bool({ let t3454 = Py::Bool(false); if py::is(&t3453, &t3454) { true } else { false } }) }; if t3457.truthy() { t3457 } else { { let t3455 = v_r.clone(); Py::Bool({ let t3456 = Py::None; if py::is(&t3455, &t3456) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3458 = v_r.clone(); Py::Bool({ let t3459 = v_r.clone(); if py::eq(&t3458, &t3459) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3437(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::getitem(&v_i.clone(), &Py::str("minor"))?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_i.clone(), &Py::str("major"))?)?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("surface_area"))?, py::truediv(&py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("volume"))?)?, &py::getitem(&v_i.clone(), &Py::str("minor"))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3460(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_trapezoid_measures(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t3466 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3467 = (t3466).items()?;
if t3467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3467[0].clone();
v_v = t3467[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3469(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3483: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3470(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3483 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3484 = v_r.clone(); Py::Bool({ let t3485 = Py::Bool(true); if py::is(&t3484, &t3485) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3490 = { let t3486 = v_r.clone(); Py::Bool({ let t3487 = Py::Bool(false); if py::is(&t3486, &t3487) { true } else { false } }) }; if t3490.truthy() { t3490 } else { { let t3488 = v_r.clone(); Py::Bool({ let t3489 = Py::None; if py::is(&t3488, &t3489) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3491 = v_r.clone(); Py::Bool({ let t3492 = v_r.clone(); if py::eq(&t3491, &t3492) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3470(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::truediv(&py::mul(&py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?, &Py::Int(2i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("midsegment"))?, py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &Py::Int(2i64))?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3493(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_area_3d(a0)? };
    return Ok({ let mut out = vec![]; for t3506 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3507 = (t3506).items()?;
if t3507.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3507[0].clone();
v_v = t3507[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3509(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3523: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3510(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3523 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3524 = v_r.clone(); Py::Bool({ let t3525 = Py::Bool(true); if py::is(&t3524, &t3525) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3530 = { let t3526 = v_r.clone(); Py::Bool({ let t3527 = Py::Bool(false); if py::is(&t3526, &t3527) { true } else { false } }) }; if t3530.truthy() { t3530 } else { { let t3528 = v_r.clone(); Py::Bool({ let t3529 = Py::None; if py::is(&t3528, &t3529) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3531 = v_r.clone(); Py::Bool({ let t3532 = v_r.clone(); if py::eq(&t3531, &t3532) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3510(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_P: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("area"))?)?, f__vnorm(f__vcross(f__vsub(py::getitem(&v_P.clone(), &Py::Int(1i64))?, py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, f__vsub(py::getitem(&v_P.clone(), &Py::Int(2i64))?, py::getitem(&v_P.clone(), &Py::Int(0i64))?)?)?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::add(&py::add(&py::dist(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::getitem(&v_P.clone(), &Py::Int(1i64))?)?, &py::dist(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &py::getitem(&v_P.clone(), &Py::Int(2i64))?)?)?, &py::dist(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::getitem(&v_P.clone(), &Py::Int(2i64))?)?)?])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__triples(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_352(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_cone_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t358 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t359 = (t358).items()?;
if t359.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t359[0].clone();
v_v = t359[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3533(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "base") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument base".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_base_height(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3539 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3540 = (t3539).items()?;
if t3540.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3540[0].clone();
v_v = t3540[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3542(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3554: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3543(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3554 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3555 = v_r.clone(); Py::Bool({ let t3556 = Py::Bool(true); if py::is(&t3555, &t3556) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3561 = { let t3557 = v_r.clone(); Py::Bool({ let t3558 = Py::Bool(false); if py::is(&t3557, &t3558) { true } else { false } }) }; if t3561.truthy() { t3561 } else { { let t3559 = v_r.clone(); Py::Bool({ let t3560 = Py::None; if py::is(&t3559, &t3560) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3562 = v_r.clone(); Py::Bool({ let t3563 = v_r.clone(); if py::eq(&t3562, &t3563) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3543(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("area"))?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("base"))?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?, &Py::Int(2i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3564(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f__cevians(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t3575 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3576 = (t3575).items()?;
if t3576.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3576[0].clone();
v_v = t3576[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3578(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3597: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_cevians(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3597 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3598 = v_r.clone(); Py::Bool({ let t3599 = Py::Bool(true); if py::is(&t3598, &t3599) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3604 = { let t3600 = v_r.clone(); Py::Bool({ let t3601 = Py::Bool(false); if py::is(&t3600, &t3601) { true } else { false } }) }; if t3604.truthy() { t3604 } else { { let t3602 = v_r.clone(); Py::Bool({ let t3603 = Py::None; if py::is(&t3602, &t3603) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3605 = v_r.clone(); Py::Bool({ let t3606 = v_r.clone(); if py::eq(&t3605, &t3606) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3607(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_circumcircle(a0)? };
    return Ok({ let mut out = vec![]; for t3629 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3630 = (t3629).items()?;
if t3630.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3630[0].clone();
v_v = t3630[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_361(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t375: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_362(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t375 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t376 = v_r.clone(); Py::Bool({ let t377 = Py::Bool(true); if py::is(&t376, &t377) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t382 = { let t378 = v_r.clone(); Py::Bool({ let t379 = Py::Bool(false); if py::is(&t378, &t379) { true } else { false } }) }; if t382.truthy() { t382 } else { { let t380 = v_r.clone(); Py::Bool({ let t381 = Py::None; if py::is(&t380, &t381) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t383 = v_r.clone(); Py::Bool({ let t384 = v_r.clone(); if py::eq(&t383, &t384) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_362(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("volume"))?, &Py::Int(3i64))?, py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?]), Py::tuple(vec![py::pow(&py::getitem(&v_o.clone(), &Py::str("slant_height"))?, &Py::Int(2i64))?, py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("height"))?, &Py::Int(2i64))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::getitem(&v_i.clone(), &Py::str("radius"))?)?, &py::getitem(&v_o.clone(), &Py::str("slant_height"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("total_area"))?, py::add(&py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, &py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3632(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3634: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_circum(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3634 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3635 = v_r.clone(); Py::Bool({ let t3636 = Py::Bool(true); if py::is(&t3635, &t3636) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3641 = { let t3637 = v_r.clone(); Py::Bool({ let t3638 = Py::Bool(false); if py::is(&t3637, &t3638) { true } else { false } }) }; if t3641.truthy() { t3641 } else { { let t3639 = v_r.clone(); Py::Bool({ let t3640 = Py::None; if py::is(&t3639, &t3640) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3642 = v_r.clone(); Py::Bool({ let t3643 = v_r.clone(); if py::eq(&t3642, &t3643) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3644(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_classify(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t3666 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3667 = (t3666).items()?;
if t3667.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3667[0].clone();
v_v = t3667[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3669(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3680: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_classify(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3680 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3681 = v_r.clone(); Py::Bool({ let t3682 = Py::Bool(true); if py::is(&t3681, &t3682) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3687 = { let t3683 = v_r.clone(); Py::Bool({ let t3684 = Py::Bool(false); if py::is(&t3683, &t3684) { true } else { false } }) }; if t3687.truthy() { t3687 } else { { let t3685 = v_r.clone(); Py::Bool({ let t3686 = Py::None; if py::is(&t3685, &t3686) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3688 = v_r.clone(); Py::Bool({ let t3689 = v_r.clone(); if py::eq(&t3688, &t3689) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3690(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_coordinates(a0)? };
    return Ok({ let mut out = vec![]; for t3709 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3710 = (t3709).items()?;
if t3710.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3710[0].clone();
v_v = t3710[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3712(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3727: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_tri_coords(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3727 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3728 = v_r.clone(); Py::Bool({ let t3729 = Py::Bool(true); if py::is(&t3728, &t3729) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3734 = { let t3730 = v_r.clone(); Py::Bool({ let t3731 = Py::Bool(false); if py::is(&t3730, &t3731) { true } else { false } }) }; if t3734.truthy() { t3734 } else { { let t3732 = v_r.clone(); Py::Bool({ let t3733 = Py::None; if py::is(&t3732, &t3733) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3735 = v_r.clone(); Py::Bool({ let t3736 = v_r.clone(); if py::eq(&t3735, &t3736) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3737(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_incircle(a0)? };
    return Ok({ let mut out = vec![]; for t3760 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3761 = (t3760).items()?;
if t3761.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3761[0].clone();
v_v = t3761[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3763(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3769: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_incircle(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3769 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3770 = v_r.clone(); Py::Bool({ let t3771 = Py::Bool(true); if py::is(&t3770, &t3771) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3776 = { let t3772 = v_r.clone(); Py::Bool({ let t3773 = Py::Bool(false); if py::is(&t3772, &t3773) { true } else { false } }) }; if t3776.truthy() { t3776 } else { { let t3774 = v_r.clone(); Py::Bool({ let t3775 = Py::None; if py::is(&t3774, &t3775) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3777 = v_r.clone(); Py::Bool({ let t3778 = v_r.clone(); if py::eq(&t3777, &t3778) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3779(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_orthocenter(a0)? };
    return Ok({ let mut out = vec![]; for t3803 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3804 = (t3803).items()?;
if t3804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3804[0].clone();
v_v = t3804[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3806(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3810: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_ortho(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3810 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3811 = v_r.clone(); Py::Bool({ let t3812 = Py::Bool(true); if py::is(&t3811, &t3812) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3817 = { let t3813 = v_r.clone(); Py::Bool({ let t3814 = Py::Bool(false); if py::is(&t3813, &t3814) { true } else { false } }) }; if t3817.truthy() { t3817 } else { { let t3815 = v_r.clone(); Py::Bool({ let t3816 = Py::None; if py::is(&t3815, &t3816) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3818 = v_r.clone(); Py::Bool({ let t3819 = v_r.clone(); if py::eq(&t3818, &t3819) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_385(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument A".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "B") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument B".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "C") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument C".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "D") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument D".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "E") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument E".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "F") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument F".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_conic_classify(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t413 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t414 = (t413).items()?;
if t414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t414[0].clone();
v_v = t414[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_416(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t441: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_conic(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t441 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t442 = v_r.clone(); Py::Bool({ let t443 = Py::Bool(true); if py::is(&t442, &t443) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t448 = { let t444 = v_r.clone(); Py::Bool({ let t445 = Py::Bool(false); if py::is(&t444, &t445) { true } else { false } }) }; if t448.truthy() { t448 } else { { let t446 = v_r.clone(); Py::Bool({ let t447 = Py::None; if py::is(&t446, &t447) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t449 = v_r.clone(); Py::Bool({ let t450 = v_r.clone(); if py::eq(&t449, &t450) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_44(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t46: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_45(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t46 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t47 = v_r.clone(); Py::Bool({ let t48 = Py::Bool(true); if py::is(&t47, &t48) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t53 = { let t49 = v_r.clone(); Py::Bool({ let t50 = Py::Bool(false); if py::is(&t49, &t50) { true } else { false } }) }; if t53.truthy() { t53 } else { { let t51 = v_r.clone(); Py::Bool({ let t52 = Py::None; if py::is(&t51, &t52) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t54 = v_r.clone(); Py::Bool({ let t55 = v_r.clone(); if py::eq(&t54, &t55) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_45(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n1: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_n2: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[py::abs(&py::sub(&py::math1("cos", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("angle"))?)?)?, &py::truediv(&py::abs(&f__vdot(v_n1.clone(), v_n2.clone())?)?, &py::mul(&f__vnorm(v_n1.clone())?, &f__vnorm(v_n2.clone())?)?)?)?)?, py::abs(&py::sub(&py::math1("sin", &py::math1("radians", &py::getitem(&v_o.clone(), &Py::str("angle"))?)?)?, &py::truediv(&f__vnorm(f__vcross(v_n1.clone(), v_n2.clone())?)?, &py::mul(&f__vnorm(v_n1.clone())?, &f__vnorm(v_n2.clone())?)?)?)?)?], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a1"))?, py::getitem(&v_i.clone(), &Py::str("b1"))?, py::getitem(&v_i.clone(), &Py::str("c1"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a2"))?, py::getitem(&v_i.clone(), &Py::str("b2"))?, py::getitem(&v_i.clone(), &Py::str("c2"))?])])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_451(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_convex_hull(a0)? };
    return Ok({ let mut out = vec![]; for t480 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t481 = (t480).items()?;
if t481.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t481[0].clone();
v_v = t481[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_483(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t516: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_hull(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t516 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t517 = v_r.clone(); Py::Bool({ let t518 = Py::Bool(true); if py::is(&t517, &t518) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t523 = { let t519 = v_r.clone(); Py::Bool({ let t520 = Py::Bool(false); if py::is(&t519, &t520) { true } else { false } }) }; if t523.truthy() { t523 } else { { let t521 = v_r.clone(); Py::Bool({ let t522 = Py::None; if py::is(&t521, &t522) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t524 = v_r.clone(); Py::Bool({ let t525 = v_r.clone(); if py::eq(&t524, &t525) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_526(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "p") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument p".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "q") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument q".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_convex_intersection(a0, a1)? };
    return Ok({ let mut out = vec![]; for t635 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t636 = (t635).items()?;
if t636.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t636[0].clone();
v_v = t636[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_56(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "outer") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument outer".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "inner") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument inner".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_annulus_area(a0, a1)? };
    return Ok({ let mut out = vec![]; for t64 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t65 = (t64).items()?;
if t65.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t65[0].clone();
v_v = t65[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_638(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t642: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_cint(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t642 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t643 = v_r.clone(); Py::Bool({ let t644 = Py::Bool(true); if py::is(&t643, &t644) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t649 = { let t645 = v_r.clone(); Py::Bool({ let t646 = Py::Bool(false); if py::is(&t645, &t646) { true } else { false } }) }; if t649.truthy() { t649 } else { { let t647 = v_r.clone(); Py::Bool({ let t648 = Py::None; if py::is(&t647, &t648) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t650 = v_r.clone(); Py::Bool({ let t651 = v_r.clone(); if py::eq(&t650, &t651) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_652(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "length") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument length".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "width") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument width".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_cuboid_measures(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t658 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t659 = (t658).items()?;
if t659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t659[0].clone();
v_v = t659[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_661(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t675: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_662(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t675 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t676 = v_r.clone(); Py::Bool({ let t677 = Py::Bool(true); if py::is(&t676, &t677) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t682 = { let t678 = v_r.clone(); Py::Bool({ let t679 = Py::Bool(false); if py::is(&t678, &t679) { true } else { false } }) }; if t682.truthy() { t682 } else { { let t680 = v_r.clone(); Py::Bool({ let t681 = Py::None; if py::is(&t680, &t681) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t683 = v_r.clone(); Py::Bool({ let t684 = v_r.clone(); if py::eq(&t683, &t684) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_662(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("length"))?, &py::getitem(&v_i.clone(), &Py::str("width"))?)?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("surface_area"))?, py::mul(&Py::Int(2i64), &py::add(&py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("length"))?, &py::getitem(&v_i.clone(), &Py::str("width"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("length"))?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("width"))?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?)?)?]), Py::tuple(vec![py::pow(&py::getitem(&v_o.clone(), &Py::str("space_diagonal"))?, &Py::Int(2i64))?, py::add(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("length"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("width"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("height"))?, &Py::Int(2i64))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_67(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t79: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_68(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t79 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t80 = v_r.clone(); Py::Bool({ let t81 = Py::Bool(true); if py::is(&t80, &t81) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t86 = { let t82 = v_r.clone(); Py::Bool({ let t83 = Py::Bool(false); if py::is(&t82, &t83) { true } else { false } }) }; if t86.truthy() { t86 } else { { let t84 = v_r.clone(); Py::Bool({ let t85 = Py::None; if py::is(&t84, &t85) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t87 = v_r.clone(); Py::Bool({ let t88 = v_r.clone(); if py::eq(&t87, &t88) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_68(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("area"))?, py::sub(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::getitem(&v_i.clone(), &Py::str("outer"))?, &Py::Int(2i64))?)?, &py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::getitem(&v_i.clone(), &Py::str("inner"))?, &Py::Int(2i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_685(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "coefficients") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument coefficients".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "x0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x0".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_curvature_graph(a0, a1)? };
    return Ok({ let mut out = vec![]; for t697 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t698 = (t697).items()?;
if t698.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t698[0].clone();
v_v = t698[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_700(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t716: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_curv(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t716 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t717 = v_r.clone(); Py::Bool({ let t718 = Py::Bool(true); if py::is(&t717, &t718) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t723 = { let t719 = v_r.clone(); Py::Bool({ let t720 = Py::Bool(false); if py::is(&t719, &t720) { true } else { false } }) }; if t723.truthy() { t723 } else { { let t721 = v_r.clone(); Py::Bool({ let t722 = Py::None; if py::is(&t721, &t722) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t724 = v_r.clone(); Py::Bool({ let t725 = v_r.clone(); if py::eq(&t724, &t725) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_726(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_cylinder_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t732 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t733 = (t732).items()?;
if t733.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t733[0].clone();
v_v = t733[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_735(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t749: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_736(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t749 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t750 = v_r.clone(); Py::Bool({ let t751 = Py::Bool(true); if py::is(&t750, &t751) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t756 = { let t752 = v_r.clone(); Py::Bool({ let t753 = Py::Bool(false); if py::is(&t752, &t753) { true } else { false } }) }; if t756.truthy() { t756 } else { { let t754 = v_r.clone(); Py::Bool({ let t755 = Py::None; if py::is(&t754, &t755) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t757 = v_r.clone(); Py::Bool({ let t758 = v_r.clone(); if py::eq(&t757, &t758) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_736(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_i.clone(), &Py::str("radius"))?)?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("total_area"))?, py::add(&py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_759(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_delaunay_counts(a0)? };
    return Ok({ let mut out = vec![]; for t796 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t797 = (t796).items()?;
if t797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t797[0].clone();
v_v = t797[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_799(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t814: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_delaunay(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t814 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t815 = v_r.clone(); Py::Bool({ let t816 = Py::Bool(true); if py::is(&t815, &t816) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t821 = { let t817 = v_r.clone(); Py::Bool({ let t818 = Py::Bool(false); if py::is(&t817, &t818) { true } else { false } }) }; if t821.truthy() { t821 } else { { let t819 = v_r.clone(); Py::Bool({ let t820 = Py::None; if py::is(&t819, &t820) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t822 = v_r.clone(); Py::Bool({ let t823 = v_r.clone(); if py::eq(&t822, &t823) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_824(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "k") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument k".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "cx") { Some(x) => x, None => Py::Float(0.0) } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "cy") { Some(x) => x, None => Py::Float(0.0) } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_dilate_point(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t831 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t832 = (t831).items()?;
if t832.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t832[0].clone();
v_v = t832[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_834(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t836: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_dilate(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t836 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t837 = v_r.clone(); Py::Bool({ let t838 = Py::Bool(true); if py::is(&t837, &t838) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t843 = { let t839 = v_r.clone(); Py::Bool({ let t840 = Py::Bool(false); if py::is(&t839, &t840) { true } else { false } }) }; if t843.truthy() { t843 } else { { let t841 = v_r.clone(); Py::Bool({ let t842 = Py::None; if py::is(&t841, &t842) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t844 = v_r.clone(); Py::Bool({ let t845 = v_r.clone(); if py::eq(&t844, &t845) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_846(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "t") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument t".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_ellipse_curvature(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t852 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t853 = (t852).items()?;
if t853.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t853[0].clone();
v_v = t853[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_855(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t869: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_856(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t869 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t870 = v_r.clone(); Py::Bool({ let t871 = Py::Bool(true); if py::is(&t870, &t871) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t876 = { let t872 = v_r.clone(); Py::Bool({ let t873 = Py::Bool(false); if py::is(&t872, &t873) { true } else { false } }) }; if t876.truthy() { t876 } else { { let t874 = v_r.clone(); Py::Bool({ let t875 = Py::None; if py::is(&t874, &t875) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t877 = v_r.clone(); Py::Bool({ let t878 = v_r.clone(); if py::eq(&t877, &t878) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_856(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_t: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("curvature"))?, &py::pow(&py::add(&py::pow(&py::mul(&v_a.clone(), &py::math1("sin", &v_t.clone())?)?, &Py::Int(2i64))?, &py::pow(&py::mul(&v_b.clone(), &py::math1("cos", &v_t.clone())?)?, &Py::Int(2i64))?)?, &Py::Float(1.5))?)?, py::mul(&v_a.clone(), &v_b.clone())?]), Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("radius_of_curvature"))?, &py::getitem(&v_o.clone(), &Py::str("curvature"))?)?, Py::Float(1.0)]), Py::tuple(vec![py::add(&py::pow(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("x"))?, &v_a.clone())?, &Py::Int(2i64))?, &py::pow(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("y"))?, &v_b.clone())?, &Py::Int(2i64))?)?, Py::Float(1.0)])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("t"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_879(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_ellipse_perimeter(a0, a1)? };
    return Ok({ let mut out = vec![]; for t894 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t895 = (t894).items()?;
if t895.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t895[0].clone();
v_v = t895[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_89(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_barycentric(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t106 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t107 = (t106).items()?;
if t107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t107[0].clone();
v_v = t107[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_897(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t912: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_ellipse_perimeter(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t912 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t913 = v_r.clone(); Py::Bool({ let t914 = Py::Bool(true); if py::is(&t913, &t914) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t919 = { let t915 = v_r.clone(); Py::Bool({ let t916 = Py::Bool(false); if py::is(&t915, &t916) { true } else { false } }) }; if t919.truthy() { t919 } else { { let t917 = v_r.clone(); Py::Bool({ let t918 = Py::None; if py::is(&t917, &t918) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t920 = v_r.clone(); Py::Bool({ let t921 = v_r.clone(); if py::eq(&t920, &t921) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_922(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_farthest_pair(a0)? };
    return Ok({ let mut out = vec![]; for t949 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t950 = (t949).items()?;
if t950.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t950[0].clone();
v_v = t950[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_952(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t955: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_farthest(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t955 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t956 = v_r.clone(); Py::Bool({ let t957 = Py::Bool(true); if py::is(&t956, &t957) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t962 = { let t958 = v_r.clone(); Py::Bool({ let t959 = Py::Bool(false); if py::is(&t958, &t959) { true } else { false } }) }; if t962.truthy() { t962 } else { { let t960 = v_r.clone(); Py::Bool({ let t961 = Py::None; if py::is(&t960, &t961) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t963 = v_r.clone(); Py::Bool({ let t964 = v_r.clone(); if py::eq(&t963, &t964) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_965(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "r1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument r1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "r2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument r2".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_frustum_measures(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t971 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t972 = (t971).items()?;
if t972.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t972[0].clone();
v_v = t972[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_974(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t988: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_975(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t988 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t989 = v_r.clone(); Py::Bool({ let t990 = Py::Bool(true); if py::is(&t989, &t990) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t995 = { let t991 = v_r.clone(); Py::Bool({ let t992 = Py::Bool(false); if py::is(&t991, &t992) { true } else { false } }) }; if t995.truthy() { t995 } else { { let t993 = v_r.clone(); Py::Bool({ let t994 = Py::None; if py::is(&t993, &t994) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t996 = v_r.clone(); Py::Bool({ let t997 = v_r.clone(); if py::eq(&t996, &t997) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_975(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_R: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_r: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_h: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&Py::Int(3i64), &py::getitem(&v_o.clone(), &Py::str("volume"))?)?, py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_h.clone())?, &py::add(&py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_R.clone(), &v_r.clone())?)?, &py::mul(&v_r.clone(), &v_r.clone())?)?)?]), Py::tuple(vec![py::pow(&py::getitem(&v_o.clone(), &Py::str("slant_height"))?, &Py::Int(2i64))?, py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::pow(&py::sub(&v_R.clone(), &v_r.clone())?, &Py::Int(2i64))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::add(&v_R.clone(), &v_r.clone())?)?, &py::getitem(&v_o.clone(), &Py::str("slant_height"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("total_area"))?, py::add(&py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, &py::mul(&Py::Float(std::f64::consts::PI), &py::add(&py::mul(&v_R.clone(), &v_R.clone())?, &py::mul(&v_r.clone(), &v_r.clone())?)?)?)?])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&v_i.clone(), &Py::str("r1"))?, py::getitem(&v_i.clone(), &Py::str("r2"))?, py::getitem(&v_i.clone(), &Py::str("height"))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_998(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "lat1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument lat1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "lon1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument lon1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "lat2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument lat2".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "lon2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument lon2".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => Py::Float(6371.0) } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_great_circle(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t1014 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1015 = (t1014).items()?;
if t1015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1015[0].clone();
v_v = t1015[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

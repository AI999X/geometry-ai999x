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
        "line_intersection" => {
            let r = s_1159(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a1", "b1", "c1", "a2", "b2", "c2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1177(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_plane_intersection" => {
            let r = s_1189(Py::tuple(vec![]), { let mut d = vec![]; for k in ["x0", "y0", "z0", "dx", "dy", "dz", "a", "b", "c", "d"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1212(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "min_area_rectangle" => {
            let r = s_1227(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1268(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "min_enclosing_circle" => {
            let r = s_1297(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1327(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nearest_neighbor" => {
            let r = s_1354(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1369(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parabola_arc_length" => {
            let r = s_1402(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b", "x0", "x1", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1414(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pick_lattice_points" => {
            let r = s_1442(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1536(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "plane_through_points" => {
            let r = s_1573(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1594(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "platonic_solid" => {
            let r = s_1614(Py::tuple(vec![]), { let mut d = vec![]; for k in ["solid", "edge"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1625(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "point_in_polygon" => {
            let r = s_1651(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1752(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "point_plane_distance" => {
            let r = s_1786(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "pz", "a", "b", "c", "d"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1804(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "point_segment_distance" => {
            let r = s_1817(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "x1", "y1", "x2", "y2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1832(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_area" => {
            let r = s_1852(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1941(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_centroid" => {
            let r = s_1969(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2063(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_interior_angles" => {
            let r = s_2081(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2175(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polygon_is_convex" => {
            let r = s_2213(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2306(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polyline_length" => {
            let r = s_2331(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2347(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pyramid_measures" => {
            let r = s_2373(Py::tuple(vec![]), { let mut d = vec![]; for k in ["length", "width", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2382(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rectangle_measures" => {
            let r = s_2406(Py::tuple(vec![]), { let mut d = vec![]; for k in ["length", "width"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2415(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reflect_point_line" => {
            let r = s_2439(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "a", "b", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2455(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "regular_polygon_angles" => {
            let r = s_2469(Py::tuple(vec![]), { let mut d = vec![]; for k in ["n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2480(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "regular_polygon_measures" => {
            let r = s_2504(Py::tuple(vec![]), { let mut d = vec![]; for k in ["n", "side"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2520(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "regular_prism" => {
            let r = s_2544(Py::tuple(vec![]), { let mut d = vec![]; for k in ["n", "side", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2560(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rhombus_measures" => {
            let r = s_2584(Py::tuple(vec![]), { let mut d = vec![]; for k in ["d1", "d2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotate_point" => {
            let r = s_2617(Py::tuple(vec![]), { let mut d = vec![]; for k in ["px", "py", "angle", "cx", "cy"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2629(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotate_point_3d" => {
            let r = s_2641(Py::tuple(vec![]), { let mut d = vec![]; for k in ["point", "axis", "angle"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2658(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sector_measures" => {
            let r = s_2675(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius", "angle"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2687(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "segment_intersection" => {
            let r = s_2711(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2783(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "skew_lines_distance" => {
            let r = s_2823(Py::tuple(vec![]), { let mut d = vec![]; for k in ["p1", "d1", "p2", "d2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2841(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solid_angle_cone" => {
            let r = s_2861(Py::tuple(vec![]), { let mut d = vec![]; for k in ["half_angle"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2870(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solid_angle_triangle" => {
            let r = s_2894(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2911(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sphere_four_points" => {
            let r = s_2935(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2957(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sphere_measures" => {
            let r = s_2970(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2978(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spherical_cap" => {
            let r = s_3002(Py::tuple(vec![]), { let mut d = vec![]; for k in ["radius", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3013(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spherical_triangle_area" => {
            let r = s_3037(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points", "radius"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3071(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "spheroid_measures" => {
            let r = s_3097(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3114(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "surface_curvature" => {
            let r = s_3139(Py::tuple(vec![]), { let mut d = vec![]; for k in ["A", "B", "C", "D", "E", "x0", "y0", "F"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3149(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tangent_from_point" => {
            let r = s_3165(Py::tuple(vec![]), { let mut d = vec![]; for k in ["cx", "cy", "r", "px", "py"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3180(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tetrahedron_measures" => {
            let r = s_3193(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3209(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "torus_curvature" => {
            let r = s_3243(Py::tuple(vec![]), { let mut d = vec![]; for k in ["major", "minor", "v"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3255(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "torus_measures" => {
            let r = s_3279(Py::tuple(vec![]), { let mut d = vec![]; for k in ["major", "minor"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3290(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "trapezoid_measures" => {
            let r = s_3314(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3323(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_area_3d" => {
            let r = s_3347(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3363(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_area_base_height" => {
            let r = s_3387(Py::tuple(vec![]), { let mut d = vec![]; for k in ["base", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3396(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_cevians" => {
            let r = s_3418(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3432(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_circumcircle" => {
            let r = s_3461(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3486(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_classify" => {
            let r = s_3498(Py::tuple(vec![]), { let mut d = vec![]; for k in ["a", "b", "c"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3523(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_coordinates" => {
            let r = s_3544(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3566(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_incircle" => {
            let r = s_3591(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3617(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triangle_orthocenter" => {
            let r = s_3633(Py::tuple(vec![]), { let mut d = vec![]; for k in ["points"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3660(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__PLATONIC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("tetrahedron"), Py::tuple(vec![Py::Int(4i64), Py::Int(6i64), Py::Int(4i64), Py::Float(0.1178511301977579), Py::Float(1.7320508075688772), Py::Float(0.6123724356957945), Py::Float(0.20412414523193148)])), (Py::str("cube"), Py::tuple(vec![Py::Int(8i64), Py::Int(12i64), Py::Int(6i64), Py::Float(1.0), Py::Float(6.0), Py::Float(0.8660254037844386), Py::Float(0.5)])), (Py::str("octahedron"), Py::tuple(vec![Py::Int(6i64), Py::Int(12i64), Py::Int(8i64), Py::Float(0.47140452079103173), Py::Float(3.4641016151377544), Py::Float(0.7071067811865475), Py::Float(0.4082482904638631)])), (Py::str("dodecahedron"), Py::tuple(vec![Py::Int(20i64), Py::Int(30i64), Py::Int(12i64), Py::Float(7.663118960624632), Py::Float(20.645728807067602), Py::Float(1.4012585384440734), Py::Float(1.1135163644116068)])), (Py::str("icosahedron"), Py::tuple(vec![Py::Int(12i64), Py::Int(30i64), Py::Int(20i64), Py::Float(2.1816949906249126), Py::Float(8.660254037844386), Py::Float(0.9510565162951535), Py::Float(0.7557613140761706)]))])) })());
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
    let t3064 = (Py::tuple(vec![v_p1.clone(), v_p2.clone()])).items()?;
    if t3064.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t3065 = (t3064[0].clone()).items()?;
    if t3065.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f1 = t3065[0].clone();
    v_l1 = t3065[1].clone();
    let t3066 = (t3064[1].clone()).items()?;
    if t3066.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f2 = t3066[0].clone();
    v_l2 = t3066[1].clone();
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
    let t3427 = (f__tri_check(v_a.clone(), v_b.clone(), v_c.clone())?).items()?;
    if t3427.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3427[0].clone();
    v_b = t3427[1].clone();
    v_c = t3427[2].clone();
    let t3428 = (py::sorted(&Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]), (Py::Bool(true)).truthy())?).items()?;
    if t3428.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3428[0].clone();
    v_y = t3428[1].clone();
    v_z = t3428[2].clone();
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
    let t1317 = (Py::tuple(vec![py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?])).items()?;
    if t1317.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t1317[0].clone();
    v_by = t1317[1].clone();
    v_cx = t1317[2].clone();
    v_cy = t1317[3].clone();
    v_d = py::mul(&Py::Int(2i64), &py::sub(&py::mul(&v_bx.clone(), &v_cy.clone())?, &py::mul(&v_by.clone(), &v_cx.clone())?)?)?;
    if ({ let t1318 = v_d.clone(); Py::Bool({ let t1319 = Py::Int(0i64); if py::eq(&t1318, &t1319) { true } else { false } }) }).truthy() {
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
    if ({ let t3636 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3636.truthy() { t3636 } else { { let t3635 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3635.truthy() { t3635 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
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
    if ({ let t2312 = py::len(&v_P.clone())?; Py::Bool({ let t2313 = Py::Int(3i64); if py::lt(&t2312, &t2313)? { true } else { false } }) }).truthy() {
        return Ok(v_P.clone());
    }
    v_half = { let v_P = v_P.clone(); let v_half = v_half.clone(); let v_pts = v_pts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_seq: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_h: Py = Py::None;
            let mut v_p: Py = Py::None;
            v_h = Py::list(vec![]);
            for t2314 in py::iter(&(v_seq.clone()))? {
                v_p = t2314;
                while ({ let t2319 = { let t2315 = py::len(&v_h.clone())?; Py::Bool({ let t2316 = Py::Int(2i64); if py::ge(&t2315, &t2316)? { true } else { false } }) }; if !t2319.truthy() { t2319 } else { { let t2317 = f__xcross(py::getitem(&v_h.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_h.clone(), &py::neg(&Py::Int(1i64))?)?, v_p.clone())?; Py::Bool({ let t2318 = Py::Int(0i64); if py::le(&t2317, &t2318)? { true } else { false } }) } } }).truthy() {
                    let _ = py::method(&v_h.clone(), "pop", &[])?;
                }
                let _ = py::method(&v_h.clone(), "append", &[v_p.clone()])?;
            }
            return Ok(v_h.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2320 = (Py::tuple(vec![py::call(&v_half.clone(), &[v_P.clone()])?, py::call(&v_half.clone(), &[py::reversed(&v_P.clone())?])?])).items()?;
    if t2320.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lower = t2320[0].clone();
    v_upper = t2320[1].clone();
    return Ok(py::add(&py::slice(&v_lower.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, &py::slice(&v_upper.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__in_circ(mut v_c: Py, mut v_p: Py) -> R {
    return Ok({ let t1312 = py::hypot(&[py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_c.clone(), &Py::Int(1i64))?)?])?; Py::Bool({ let t1313 = py::add(&py::mul(&py::getitem(&v_c.clone(), &Py::Int(2i64))?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?, &Py::Float(1e-12))?; if py::le(&t1312, &t1313)? { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__latlon(mut v_lat: Py, mut v_lon: Py, mut v_name: Py) -> R {
    let t3052 = (Py::tuple(vec![f__fin(v_lat.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " latitude".to_string()].concat()))?, f__fin(v_lon.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " longitude".to_string()].concat()))?])).items()?;
    if t3052.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lat = t3052[0].clone();
    v_lon = t3052[1].clone();
    if (py::not(&{ let t3053 = py::neg(&Py::Int(90i64))?; Py::Bool({ let t3054 = v_lat.clone(); if py::le(&t3053, &t3054)? { { let t3055 = Py::Int(90i64); if py::le(&t3054, &t3055)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": latitude must be between -90 and 90 degrees".to_string()].concat()))));
    }
    if (py::not(&{ let t3056 = py::neg(&Py::Int(360i64))?; Py::Bool({ let t3057 = v_lon.clone(); if py::le(&t3056, &t3057)? { { let t3058 = Py::Int(360i64); if py::le(&t3057, &t3058)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": longitude must be between -360 and 360 degrees".to_string()].concat()))));
    }
    return Ok(Py::tuple(vec![py::math1("radians", &v_lat.clone())?, py::math1("radians", &v_lon.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__line_check(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_name: Py) -> R {
    let t2443 = (Py::tuple(vec![f__fin(v_a.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " a".to_string()].concat()))?, f__fin(v_b.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " b".to_string()].concat()))?, f__fin(v_c.clone(), Py::str(&[py::format(&v_name.clone(), "")?, " c".to_string()].concat()))?])).items()?;
    if t2443.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2443[0].clone();
    v_b = t2443[1].clone();
    v_c = t2443[2].clone();
    if ({ let t2448 = { let t2444 = v_a.clone(); Py::Bool({ let t2445 = Py::Int(0i64); if py::eq(&t2444, &t2445) { true } else { false } }) }; if !t2448.truthy() { t2448 } else { { let t2446 = v_b.clone(); Py::Bool({ let t2447 = Py::Int(0i64); if py::eq(&t2446, &t2447) { true } else { false } }) } } }).truthy() {
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
    if ({ let t2750 = f__xcross(v_a.clone(), v_b.clone(), v_p.clone())?; Py::Bool({ let t2751 = Py::Int(0i64); if py::ne(&t2750, &t2751) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok({ let t2758 = { let t2752 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(0i64))?, py::getitem(&v_b.clone(), &Py::Int(0i64))?], false)?; Py::Bool({ let t2753 = py::getitem(&v_p.clone(), &Py::Int(0i64))?; if py::le(&t2752, &t2753)? { { let t2754 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(0i64))?, py::getitem(&v_b.clone(), &Py::Int(0i64))?], true)?; if py::le(&t2753, &t2754)? { true } else { false } } } else { false } }) }; if !t2758.truthy() { t2758 } else { { let t2755 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(1i64))?, py::getitem(&v_b.clone(), &Py::Int(1i64))?], false)?; Py::Bool({ let t2756 = py::getitem(&v_p.clone(), &Py::Int(1i64))?; if py::le(&t2755, &t2756)? { { let t2757 = py::minmax(&[py::getitem(&v_a.clone(), &Py::Int(1i64))?, py::getitem(&v_b.clone(), &Py::Int(1i64))?], true)?; if py::le(&t2756, &t2757)? { true } else { false } } } else { false } }) } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pairs(mut v_xs: Py, mut v_name: Py, mut v_n_min: Py, mut v_n_max: Py) -> R {
    let mut v_pts: Py = Py::None;
    v_xs = { let mut out = vec![]; for t3634 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t3634;
out.push(f__fin(py::float(&v_x.clone())?, v_name.clone())?); } Py::list(out) };
    if (py::modulo(&py::len(&v_xs.clone())?, &Py::Int(2i64))?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": give an x and a y for every point (an even count of numbers)".to_string()].concat()))));
    }
    v_pts = { let mut out = vec![]; for t3637 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::len(&v_xs.clone())?).i()?, (Py::Int(2i64)).i()?]) { let mut v_i: Py; 
v_i = t3637;
out.push(Py::tuple(vec![py::getitem(&v_xs.clone(), &v_i.clone())?, py::getitem(&v_xs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])); } Py::list(out) };
    if ({ let t3638 = py::len(&v_pts.clone())?; Py::Bool({ let t3639 = v_n_min.clone(); if py::lt(&t3638, &t3639)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": needs at least ".to_string(), py::format(&v_n_min.clone(), "")?, " points".to_string()].concat()))));
    }
    if ({ let t3644 = { let t3640 = v_n_max.clone(); Py::Bool({ let t3641 = Py::None; if !py::is(&t3640, &t3641) { true } else { false } }) }; if !t3644.truthy() { t3644 } else { { let t3642 = py::len(&v_pts.clone())?; Py::Bool({ let t3643 = v_n_max.clone(); if py::gt(&t3642, &t3643)? { true } else { false } }) } } }).truthy() {
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
    return Ok(py::fsum(&{ let mut out = vec![]; for t2343 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t2343;
out.push(py::dist(&py::getitem(&v_pts.clone(), &v_i.clone())?, &py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__plane_check(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_name: Py) -> R {
    let t1791 = ({ let mut out = vec![]; for t1790 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()])))? { let mut v_v: Py; 
v_v = t1790;
out.push(f__fin(v_v.clone(), v_name.clone())?); } Py::list(out) }).items()?;
    if t1791.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1791[0].clone();
    v_b = t1791[1].clone();
    v_c = t1791[2].clone();
    v_d = t1791[3].clone();
    if ({ let t1799 = { let t1792 = v_a.clone(); Py::Bool({ let t1793 = Py::Int(0i64); if py::eq(&t1792, &t1793) { true } else { false } }) }; if !t1799.truthy() { t1799 } else { { let t1798 = { let t1794 = v_b.clone(); Py::Bool({ let t1795 = Py::Int(0i64); if py::eq(&t1794, &t1795) { true } else { false } }) }; if !t1798.truthy() { t1798 } else { { let t1796 = v_c.clone(); Py::Bool({ let t1797 = Py::Int(0i64); if py::eq(&t1796, &t1797) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": a, b, c must not all be 0".to_string()].concat()))));
    }
    return Ok(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__fin(v_v.clone(), v_name.clone())?;
    if ({ let t3501 = v_v.clone(); Py::Bool({ let t3502 = Py::Int(0i64); if py::le(&t3501, &t3502)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_x: Py) -> R {
    if ({ let t3659 = Py::Bool(matches!(v_x.clone(), Py::Float(_))); if !t3659.truthy() { t3659 } else { py::math1("isfinite", &v_x.clone())? } }).truthy() {
        return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    if ({ let t3575 = { let t3571 = v_a.clone(); Py::Bool({ let t3572 = Py::None; if py::is(&t3571, &t3572) { true } else { false } }) }; if t3575.truthy() { t3575 } else { { let t3573 = v_b.clone(); Py::Bool({ let t3574 = Py::None; if py::is(&t3573, &t3574) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t3580 = { let t3576 = v_a.clone(); Py::Bool({ let t3577 = Py::None; if py::is(&t3576, &t3577) { true } else { false } }) }; if !t3580.truthy() { t3580 } else { { let t3578 = v_b.clone(); Py::Bool({ let t3579 = Py::None; if py::is(&t3578, &t3579) { true } else { false } }) } } }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    return Ok(py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__res(mut v_pairs: Py) -> R {
    return Ok({ let t = { let mut out = vec![]; for t3569 in py::iter(&(v_pairs.clone()))? { let mut v_a: Py; let mut v_b: Py; 
let t3570 = (t3569).items()?;
if t3570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3570[0].clone();
v_b = t3570[1].clone();
out.push(f__rel(v_a.clone(), v_b.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__scale_of(mut v_pts: Py) -> R {
    return Ok(py::minmax(&[{ let mut out = vec![]; for t3653 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t3653;
out.push(py::minmax(&[{ let mut out = vec![]; for t3654 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t3654;
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
    let t2731 = (Py::tuple(vec![f__xcross(v_p3.clone(), v_p4.clone(), v_p1.clone())?, f__xcross(v_p3.clone(), v_p4.clone(), v_p2.clone())?])).items()?;
    if t2731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t2731[0].clone();
    v_d2 = t2731[1].clone();
    let t2732 = (Py::tuple(vec![f__xcross(v_p1.clone(), v_p2.clone(), v_p3.clone())?, f__xcross(v_p1.clone(), v_p2.clone(), v_p4.clone())?])).items()?;
    if t2732.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d3 = t2732[0].clone();
    v_d4 = t2732[1].clone();
    if ({ let t2747 = { let t2739 = { let t2733 = v_d1.clone(); Py::Bool({ let t2734 = Py::Int(0i64); if py::gt(&t2733, &t2734)? { { let t2735 = v_d2.clone(); if py::gt(&t2734, &t2735)? { true } else { false } } } else { false } }) }; if t2739.truthy() { t2739 } else { { let t2736 = v_d1.clone(); Py::Bool({ let t2737 = Py::Int(0i64); if py::lt(&t2736, &t2737)? { { let t2738 = v_d2.clone(); if py::lt(&t2737, &t2738)? { true } else { false } } } else { false } }) } } }; if !t2747.truthy() { t2747 } else { { let t2746 = { let t2740 = v_d3.clone(); Py::Bool({ let t2741 = Py::Int(0i64); if py::gt(&t2740, &t2741)? { { let t2742 = v_d4.clone(); if py::gt(&t2741, &t2742)? { true } else { false } } } else { false } }) }; if t2746.truthy() { t2746 } else { { let t2743 = v_d3.clone(); Py::Bool({ let t2744 = Py::Int(0i64); if py::lt(&t2743, &t2744)? { { let t2745 = v_d4.clone(); if py::lt(&t2744, &t2745)? { true } else { false } } } else { false } }) } } } } }).truthy() {
        return Ok(Py::Bool(true));
    }
    return Ok({ let t2771 = { let t2759 = { let t2748 = v_d1.clone(); Py::Bool({ let t2749 = Py::Int(0i64); if py::eq(&t2748, &t2749) { true } else { false } }) }; if !t2759.truthy() { t2759 } else { f__on_segment_exact(v_p1.clone(), v_p3.clone(), v_p4.clone())? } }; if t2771.truthy() { t2771 } else { { let t2770 = { let t2762 = { let t2760 = v_d2.clone(); Py::Bool({ let t2761 = Py::Int(0i64); if py::eq(&t2760, &t2761) { true } else { false } }) }; if !t2762.truthy() { t2762 } else { f__on_segment_exact(v_p2.clone(), v_p3.clone(), v_p4.clone())? } }; if t2770.truthy() { t2770 } else { { let t2769 = { let t2765 = { let t2763 = v_d3.clone(); Py::Bool({ let t2764 = Py::Int(0i64); if py::eq(&t2763, &t2764) { true } else { false } }) }; if !t2765.truthy() { t2765 } else { f__on_segment_exact(v_p3.clone(), v_p1.clone(), v_p2.clone())? } }; if t2769.truthy() { t2769 } else { { let t2768 = { let t2766 = v_d4.clone(); Py::Bool({ let t2767 = Py::Int(0i64); if py::eq(&t2766, &t2767) { true } else { false } }) }; if !t2768.truthy() { t2768 } else { f__on_segment_exact(v_p4.clone(), v_p1.clone(), v_p2.clone())? } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__shoelace(mut v_pts: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::fsum(&{ let mut out = vec![]; for t3609 in py::range_iter(&[(py::len(&v_pts.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3609;
out.push(py::sub(&py::mul(&py::getitem(&py::getitem(&v_pts.clone(), &v_i.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &py::len(&v_pts.clone())?)?)?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &py::len(&v_pts.clone())?)?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_i.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) })?;
    return Ok(py::truediv(&v_s.clone(), &Py::Int(2i64))?);
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
    if ({ let t2225 = v_n.clone(); Py::Bool({ let t2226 = Py::Int(3i64); if py::lt(&t2225, &t2226)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": a polygon needs at least 3 vertices".to_string()].concat()))));
    }
    if ({ let t2227 = py::len(&py::set(&v_pts.clone())?)?; Py::Bool({ let t2228 = v_n.clone(); if py::ne(&t2227, &t2228) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": a vertex is repeated".to_string()].concat()))));
    }
    if ({ let t2229 = v_n.clone(); Py::Bool({ let t2230 = Py::Int(400i64); if py::gt(&t2229, &t2230)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": at most 400 vertices".to_string()].concat()))));
    }
    for t2231 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2231;
        let t2232 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &v_i.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2232[0].clone();
        v_b = t2232[1].clone();
        for t2233 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t2233;
            if ({ let t2241 = { let t2234 = v_j.clone(); Py::Bool({ let t2235 = v_i.clone(); if py::eq(&t2234, &t2235) { true } else { false } }) }; if t2241.truthy() { t2241 } else { { let t2240 = { let t2236 = py::modulo(&py::add(&v_j.clone(), &Py::Int(1i64))?, &v_n.clone())?; Py::Bool({ let t2237 = v_i.clone(); if py::eq(&t2236, &t2237) { true } else { false } }) }; if t2240.truthy() { t2240 } else { { let t2238 = v_j.clone(); Py::Bool({ let t2239 = py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?; if py::eq(&t2238, &t2239) { true } else { false } }) } } } } }).truthy() {
                continue;
            }
            let t2242 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &v_j.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_j.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
            if t2242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t2242[0].clone();
            v_d = t2242[1].clone();
            if (f__segments_meet_exact(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone())?).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": the polygon is not simple (edges ".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?, " and ".to_string(), py::format(&py::add(&v_j.clone(), &Py::Int(1i64))?, "")?, " meet)".to_string()].concat()))));
            }
        }
    }
    for t2286 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2286;
        let t2287 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, py::getitem(&v_pts.clone(), &v_i.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2287.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2287[0].clone();
        v_b = t2287[1].clone();
        v_c = t2287[2].clone();
        if ({ let t2292 = { let t2288 = f__xcross(v_a.clone(), v_b.clone(), v_c.clone())?; Py::Bool({ let t2289 = Py::Int(0i64); if py::eq(&t2288, &t2289) { true } else { false } }) }; if !t2292.truthy() { t2292 } else { { let t2290 = py::add(&py::mul(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?)?)?; Py::Bool({ let t2291 = Py::Int(0i64); if py::lt(&t2290, &t2291)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": the polygon is not simple (an edge doubles back)".to_string()].concat()))));
        }
    }
    if ({ let t2293 = f__shoelace(v_pts.clone())?; Py::Bool({ let t2294 = Py::Int(0i64); if py::eq(&t2293, &t2294) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": the polygon has zero area".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_v: Py, mut v_scale: Py) -> R {
    return Ok((if ({ let t3655 = py::abs(&v_v.clone())?; Py::Bool({ let t3656 = py::mul(&Py::Float(1e-13), &py::minmax(&[Py::Float(1.0), v_scale.clone()], true)?)?; if py::le(&t3655, &t3656)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::add(&v_v.clone(), &Py::Float(0.0))? }));
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
    let t3503 = (Py::tuple(vec![f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?, f__pos(v_c.clone(), Py::str("c"))?])).items()?;
    if t3503.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3503[0].clone();
    v_b = t3503[1].clone();
    v_c = t3503[2].clone();
    let t3504 = (py::sorted(&Py::tuple(vec![f__fr(v_a.clone())?, f__fr(v_b.clone())?, f__fr(v_c.clone())?]), (Py::Bool(false)).truthy())?).items()?;
    if t3504.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3504[0].clone();
    v_y = t3504[1].clone();
    v_z = t3504[2].clone();
    if ({ let t3505 = py::add(&v_x.clone(), &v_y.clone())?; Py::Bool({ let t3506 = v_z.clone(); if py::le(&t3505, &t3506)? { true } else { false } }) }).truthy() {
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
    if ({ let t3645 = { let mut v: Vec<Py> = vec![]; v.extend((v_pts.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "o") { Some(x) => x, None => { return Err(PyErr::Type("_xcross() missing argument o".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("_xcross() missing argument a".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("_xcross() missing argument b".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("_xcross() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_xcross() got an unexpected keyword argument".into())) } f__xcross(a0, a1, a2)? }; Py::Bool({ let t3646 = Py::Int(0i64); if py::eq(&t3645, &t3646) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the three points are collinear: they do not form a triangle"))));
    }
    return Ok(v_pts.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__triples(mut v_xs: Py, mut v_name: Py, mut v_n: Py) -> R {
    v_xs = { let mut out = vec![]; for t3348 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t3348;
out.push(f__fin(py::float(&v_x.clone())?, v_name.clone())?); } Py::list(out) };
    if ({ let t3351 = py::len(&v_xs.clone())?; Py::Bool({ let t3352 = py::mul(&Py::Int(3i64), &v_n.clone())?; if py::ne(&t3351, &t3352) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, ": give exactly ".to_string(), py::format(&v_n.clone(), "")?, " points of three coordinates".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t3353 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3353;
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
    let t2064 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("cx"))?, py::getitem(&v_o.clone(), &Py::str("cy"))?])).items()?;
    if t2064.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t2064[0].clone();
    v_cy = t2064[1].clone();
    v_n = py::len(&v_pts.clone())?;
    let t2065 = Py::Float(0.0);
    v_mx = t2065.clone();
    v_my = t2065.clone();
    v_area = t2065.clone();
    v_scale = py::minmax(&[{ let mut out = vec![]; for t2066 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2066;
out.push(py::minmax(&[py::abs(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &v_cx.clone())?)?, py::abs(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &v_cy.clone())?)?], true)?); } Py::list(out) }], true)?;
    for t2067 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2067;
        let t2068 = (Py::tuple(vec![Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?, &v_cx.clone())?, py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?, &v_cy.clone())?]), Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &v_cx.clone())?, py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?, &v_cy.clone())?])])).items()?;
        if t2068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t2069 = (t2068[0].clone()).items()?;
        if t2069.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xa = t2069[0].clone();
        v_ya = t2069[1].clone();
        let t2070 = (t2068[1].clone()).items()?;
        if t2070.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xb = t2070[0].clone();
        v_yb = t2070[1].clone();
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
    let t3433 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t3433.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3433[0].clone();
    v_b = t3433[1].clone();
    v_c = t3433[2].clone();
    v_out = Py::list(vec![]);
    for t3434 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), Py::str("a")]), Py::tuple(vec![v_b.clone(), v_a.clone(), v_c.clone(), Py::str("b")]), Py::tuple(vec![v_c.clone(), v_a.clone(), v_b.clone(), Py::str("c")])])))? {
        let t3436 = (t3434).items()?;
        if t3436.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_o_ = t3436[0].clone();
        v_p = t3436[1].clone();
        v_q = t3436[2].clone();
        v_s = t3436[3].clone();
        v_m = py::getitem(&v_o.clone(), &Py::str(&["median_".to_string(), py::format(&v_s.clone(), "")?].concat()))?;
        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![py::add(&py::truediv(&py::mul(&py::mul(&v_p.clone(), &v_p.clone())?, &v_o_.clone())?, &Py::Int(2i64))?, &py::truediv(&py::mul(&py::mul(&v_q.clone(), &v_q.clone())?, &v_o_.clone())?, &Py::Int(2i64))?)?, py::mul(&v_o_.clone(), &py::add(&py::mul(&v_m.clone(), &v_m.clone())?, &py::truediv(&py::mul(&v_o_.clone(), &v_o_.clone())?, &Py::Int(4i64))?)?)?])])?;
        let t3435 = (Py::tuple(vec![py::truediv(&py::mul(&v_o_.clone(), &v_q.clone())?, &py::add(&v_p.clone(), &v_q.clone())?)?, py::truediv(&py::mul(&v_o_.clone(), &v_p.clone())?, &py::add(&v_p.clone(), &v_q.clone())?)?])).items()?;
        if t3435.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_mm = t3435[0].clone();
        v_nn = t3435[1].clone();
        v_t = py::getitem(&v_o.clone(), &Py::str(&["bisector_".to_string(), py::format(&v_s.clone(), "")?].concat()))?;
        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![py::add(&py::mul(&py::mul(&v_p.clone(), &v_p.clone())?, &v_mm.clone())?, &py::mul(&py::mul(&v_q.clone(), &v_q.clone())?, &v_nn.clone())?)?, py::mul(&v_o_.clone(), &py::add(&py::mul(&v_t.clone(), &v_t.clone())?, &py::mul(&v_mm.clone(), &v_nn.clone())?)?)?])])?;
    }
    v_cosC = py::truediv(&py::sub(&py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?)?;
    v_K = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_a.clone())?, &v_b.clone())?, &py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::sub(&Py::Int(1i64), &py::mul(&v_cosC.clone(), &v_cosC.clone())?)?], true)?)?)?;
    v_out = py::iadd(&v_out, &Py::list(vec![Py::tuple(vec![py::mul(&v_a.clone(), &py::getitem(&v_o.clone(), &Py::str("altitude_a"))?)?, py::mul(&Py::Int(2i64), &v_K.clone())?]), Py::tuple(vec![py::mul(&v_b.clone(), &py::getitem(&v_o.clone(), &Py::str("altitude_b"))?)?, py::mul(&Py::Int(2i64), &v_K.clone())?]), Py::tuple(vec![py::mul(&v_c.clone(), &py::getitem(&v_o.clone(), &Py::str("altitude_c"))?)?, py::mul(&Py::Int(2i64), &v_K.clone())?])]))?;
    return Ok((if (Py::Bool(({ let mut out = vec![]; for t3437 in py::iter(&(v_out.clone()))? { let mut v__: Py; let mut v_u: Py; 
let t3438 = (t3437).items()?;
if t3438.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t3438[0].clone();
v__ = t3438[1].clone();
out.push(py::math1("isfinite", &v_u.clone())?); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() { py::minmax(&[{ let mut out = vec![]; for t3439 in py::iter(&(v_out.clone()))? { let mut v_u: Py; let mut v_v: Py; 
let t3440 = (t3439).items()?;
if t3440.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t3440[0].clone();
v_v = t3440[1].clone();
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
    return Ok(py::minmax(&[{ let mut out = vec![]; for t3487 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t3487;
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
    let t3524 = (py::sorted(&Py::tuple(vec![f__fr(py::getitem(&v_i.clone(), &Py::str("a"))?)?, f__fr(py::getitem(&v_i.clone(), &Py::str("b"))?)?, f__fr(py::getitem(&v_i.clone(), &Py::str("c"))?)?]), (Py::Bool(false)).truthy())?).items()?;
    if t3524.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3524[0].clone();
    v_b = t3524[1].clone();
    v_c = t3524[2].clone();
    v_largest_cos = py::truediv(&py::sub(&py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?, &py::mul(&v_c.clone(), &v_c.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?)?;
    v_want = (if ({ let t3525 = v_largest_cos.clone(); Py::Bool({ let t3526 = Py::Int(0i64); if py::eq(&t3525, &t3526) { true } else { false } }) }).truthy() { Py::str("right") } else { (if ({ let t3527 = v_largest_cos.clone(); Py::Bool({ let t3528 = Py::Int(0i64); if py::lt(&t3527, &t3528)? { true } else { false } }) }).truthy() { Py::str("obtuse") } else { Py::str("acute") }) });
    v_eq = py::len(&py::set(&Py::list(vec![v_a.clone(), v_b.clone(), v_c.clone()]))?)?;
    return Ok({ let t3533 = { let t3529 = py::getitem(&v_o.clone(), &Py::str("by_angles"))?; Py::Bool({ let t3530 = v_want.clone(); if py::eq(&t3529, &t3530) { true } else { false } }) }; if !t3533.truthy() { t3533 } else { { let t3531 = py::getitem(&v_o.clone(), &Py::str("by_sides"))?; Py::Bool({ let t3532 = py::getitem(&Py::dict(vec![(Py::Int(1i64), Py::str("equilateral")), (Py::Int(2i64), Py::str("isosceles")), (Py::Int(3i64), Py::str("scalene"))]), &v_eq.clone())?; if py::eq(&t3531, &t3532) { true } else { false } }) } } });
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
            return Ok(py::abs(&py::sum(&{ let mut out = vec![]; for t2307 in py::range_iter(&[(py::len(&v_P.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2307;
out.push(py::sub(&py::mul(&f__fr(py::getitem(&py::getitem(&v_P.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &f__fr(py::getitem(&py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &py::len(&v_P.clone())?)?)?, &Py::Int(1i64))?)?)?, &py::mul(&f__fr(py::getitem(&py::getitem(&v_P.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &py::len(&v_P.clone())?)?)?, &Py::Int(0i64))?)?, &f__fr(py::getitem(&py::getitem(&v_P.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?)?); } Py::list(out) }, None)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok({ let t2308 = py::getitem(&v_o.clone(), &Py::str("convex"))?; Py::Bool({ let t2309 = (if ({ let t2310 = py::call(&v_exact.clone(), &[v_pts.clone()])?; Py::Bool({ let t2311 = py::call(&v_exact.clone(), &[f__hull(v_pts.clone())?])?; if py::eq(&t2310, &t2311) { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }); if py::eq(&t2308, &t2309) { true } else { false } }) });
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
    let t3618 = (f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(3i64))?).items()?;
    if t3618.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3618[0].clone();
    v_B = t3618[1].clone();
    v_C = t3618[2].clone();
    v_I = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("ix"))?, py::getitem(&v_o.clone(), &Py::str("iy"))?]);
    v_sgn = (if ({ let t3619 = f__shoelace(Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()]))?; Py::Bool({ let t3620 = Py::Int(0i64); if py::gt(&t3619, &t3620)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    v_worst = Py::Float(0.0);
    for t3621 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_A.clone(), v_B.clone()]), Py::tuple(vec![v_B.clone(), v_C.clone()]), Py::tuple(vec![v_C.clone(), v_A.clone()])])))? {
        let t3622 = (t3621).items()?;
        if t3622.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_P = t3622[0].clone();
        v_Q = t3622[1].clone();
        v_dist = py::truediv(&py::mul(&v_sgn.clone(), &f__cross(v_P.clone(), v_Q.clone(), v_I.clone())?)?, &py::dist(&v_P.clone(), &v_Q.clone())?)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_dist.clone(), &py::getitem(&v_o.clone(), &Py::str("radius"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?], true)?;
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
    let t1213 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x0"))?, py::getitem(&v_i.clone(), &Py::str("y0"))?, py::getitem(&v_i.clone(), &Py::str("z0"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("dx"))?, py::getitem(&v_i.clone(), &Py::str("dy"))?, py::getitem(&v_i.clone(), &Py::str("dz"))?])])).items()?;
    if t1213.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1213[0].clone();
    v_D = t1213[1].clone();
    v_sc = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1214 in py::iter(&(v_X.clone()))? { let mut v_v: Py; 
v_v = t1214;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?, py::minmax(&[{ let mut out = vec![]; for t1215 in py::iter(&(v_P.clone()))? { let mut v_v: Py; 
v_v = t1215;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?;
    v_on_plane = py::truediv(&py::abs(&py::add(&f__vdot(v_n.clone(), v_X.clone())?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?)?, &py::mul(&f__vnorm(v_n.clone())?, &v_sc.clone())?)?;
    v_on_line = py::truediv(&f__vnorm(f__vsub(v_X.clone(), Py::tuple(({ let mut out = vec![]; for t1216 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t1216;
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
    for t1269 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1269;
        let t1270 = (Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_h.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_h.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(1i64))?)?])).items()?;
        if t1270.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ex = t1270[0].clone();
        v_ey = t1270[1].clone();
        v_L = py::hypot(&[v_ex.clone(), v_ey.clone()])?;
        v_us = { let mut out = vec![]; for t1271 in py::iter(&(v_h.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1272 = (t1271).items()?;
if t1272.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1272[0].clone();
v_y = t1272[1].clone();
out.push(py::truediv(&py::add(&py::mul(&py::sub(&v_x.clone(), &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &v_ex.clone())?, &py::mul(&py::sub(&v_y.clone(), &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(1i64))?)?, &v_ey.clone())?)?, &v_L.clone())?); } Py::list(out) };
        v_vs = { let mut out = vec![]; for t1273 in py::iter(&(v_h.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1274 = (t1273).items()?;
if t1274.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1274[0].clone();
v_y = t1274[1].clone();
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
    let t1328 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("cx"))?, py::getitem(&v_o.clone(), &Py::str("cy"))?]), py::getitem(&v_o.clone(), &Py::str("radius"))?])).items()?;
    if t1328.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C = t1328[0].clone();
    v_r = t1328[1].clone();
    if ({ let t1329 = v_r.clone(); Py::Bool({ let t1330 = Py::Int(0i64); if py::eq(&t1329, &t1330) { true } else { false } }) }).truthy() {
        return Ok({ let t1331 = py::len(&py::set(&v_pts.clone())?)?; Py::Bool({ let t1332 = Py::Int(1i64); if py::eq(&t1331, &t1332) { true } else { false } }) });
    }
    if (Py::Bool(({ let mut out = vec![]; for t1333 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1333;
out.push({ let t1334 = py::dist(&v_C.clone(), &v_p.clone())?; Py::Bool({ let t1335 = py::mul(&v_r.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::gt(&t1334, &t1335)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Bool(false));
    }
    v_on = { let mut out = vec![]; for t1336 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1336;
if ({ let t1337 = py::abs(&py::sub(&py::dist(&v_C.clone(), &v_p.clone())?, &v_r.clone())?)?; Py::Bool({ let t1338 = py::mul(&Py::Float(1e-09), &v_r.clone())?; if py::le(&t1337, &t1338)? { true } else { false } }) }).truthy() { out.push(py::atan2(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_C.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_C.clone(), &Py::Int(0i64))?)?)?); }} Py::list(out) };
    if ({ let t1339 = py::len(&v_on.clone())?; Py::Bool({ let t1340 = Py::Int(2i64); if py::lt(&t1339, &t1340)? { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    let _ = py::method_sort(&v_on.clone(), None, false)?;
    v_gaps = py::add(&{ let mut out = vec![]; for t1341 in py::range_iter(&[(py::sub(&py::len(&v_on.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1341;
out.push(py::sub(&py::getitem(&v_on.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_on.clone(), &v_k.clone())?)?); } Py::list(out) }, &Py::list(vec![py::add(&py::sub(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_on.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::getitem(&v_on.clone(), &Py::Int(0i64))?)?]))?;
    return Ok({ let t1342 = py::minmax(&[v_gaps.clone()], true)?; Py::Bool({ let t1343 = py::mul(&Py::Float(std::f64::consts::PI), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::le(&t1342, &t1343)? { true } else { false } }) });
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
    if ({ let t1375 = py::not(&{ let t1370 = Py::Int(0i64); Py::Bool({ let t1371 = v_k.clone(); if py::le(&t1370, &t1371)? { { let t1372 = py::len(&v_pts.clone())?; if py::lt(&t1371, &t1372)? { true } else { false } } } else { false } }) }); if t1375.truthy() { t1375 } else { { let t1373 = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("nx"))?, py::getitem(&v_o.clone(), &Py::str("ny"))?]); Py::Bool({ let t1374 = py::getitem(&v_pts.clone(), &v_k.clone())?; if py::ne(&t1373, &t1374) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_d2 = { let mut out = vec![]; for t1376 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1376;
out.push(py::add(&py::pow(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_P.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?); } Py::list(out) };
    if ({ let t1381 = { let t1377 = py::getitem(&v_d2.clone(), &v_k.clone())?; Py::Bool({ let t1378 = py::minmax(&[v_d2.clone()], false)?; if py::ne(&t1377, &t1378) { true } else { false } }) }; if t1381.truthy() { t1381 } else { { let t1379 = py::method(&v_d2.clone(), "index", &[py::minmax(&[v_d2.clone()], false)?])?; Py::Bool({ let t1380 = v_k.clone(); if py::ne(&t1379, &t1380) { true } else { false } }) } } }).truthy() {
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
    let t3661 = (f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(3i64))?).items()?;
    if t3661.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3661[0].clone();
    v_B = t3661[1].clone();
    v_C = t3661[2].clone();
    v_H = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("hx"))?, py::getitem(&v_o.clone(), &Py::str("hy"))?]);
    v_L = py::minmax(&[py::dist(&v_A.clone(), &v_B.clone())?, py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?], true)?;
    v_worst = Py::Float(0.0);
    for t3662 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_A.clone(), v_B.clone(), v_C.clone()]), Py::tuple(vec![v_B.clone(), v_C.clone(), v_A.clone()]), Py::tuple(vec![v_C.clone(), v_A.clone(), v_B.clone()])])))? {
        let t3663 = (t3662).items()?;
        if t3663.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_P = t3663[0].clone();
        v_Q = t3663[1].clone();
        v_R_ = t3663[2].clone();
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
    if ({ let t2183 = { let t2176 = py::len(&py::getitem(&v_o.clone(), &Py::str("angles"))?)?; Py::Bool({ let t2177 = v_n.clone(); if py::ne(&t2176, &t2177) { true } else { false } }) }; if t2183.truthy() { t2183 } else { { let t2178 = py::getitem(&v_o.clone(), &Py::str("reflex_vertices"))?; Py::Bool({ let t2179 = py::sum(&{ let mut out = vec![]; for t2180 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("angles"))?))? { let mut v_x: Py; 
v_x = t2180;
if ({ let t2181 = v_x.clone(); Py::Bool({ let t2182 = Py::Int(180i64); if py::gt(&t2181, &t2182)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?; if py::ne(&t2178, &t2179) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_worst = py::add(&f__rel(py::sum(&py::getitem(&v_o.clone(), &Py::str("angles"))?, None)?, py::mul(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &Py::Float(180.0))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("angle_sum"))?, py::mul(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &Py::Float(180.0))?)?)?;
    v_sgn = (if ({ let t2194 = f__shoelace(v_pts.clone())?; Py::Bool({ let t2195 = Py::Int(0i64); if py::gt(&t2194, &t2195)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    for t2196 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2196;
        let t2197 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2197.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2197[0].clone();
        v_b = t2197[1].clone();
        v_c = t2197[2].clone();
        let t2198 = (Py::tuple(vec![Py::tuple(vec![py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?]), Py::tuple(vec![py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?])])).items()?;
        if t2198.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2198[0].clone();
        v_v = t2198[1].clone();
        v_x = py::math1("degrees", &py::math1("acos", &py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::add(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::hypot(&({ let mut v: Vec<Py> = vec![]; v.extend((v_u.clone()).items()?); v })[..])?, &py::hypot(&({ let mut v: Vec<Py> = vec![]; v.extend((v_v.clone()).items()?); v })[..])?)?)?], false)?], true)?)?)?;
        if ({ let t2199 = py::mul(&v_sgn.clone(), &f__cross(v_a.clone(), v_b.clone(), v_c.clone())?)?; Py::Bool({ let t2200 = Py::Int(0i64); if py::lt(&t2199, &t2200)? { true } else { false } }) }).truthy() {
            v_x = py::sub(&Py::Int(360i64), &v_x.clone())?;
        }
        v_worst = py::minmax(&[v_worst.clone(), (if ({ let t2201 = py::abs(&py::sub(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("angles"))?, &v_k.clone())?, &v_x.clone())?)?; Py::Bool({ let t2202 = Py::Float(1e-07); if py::gt(&t2201, &t2202)? { true } else { false } }) }).truthy() { py::truediv(&py::abs(&py::sub(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("angles"))?, &v_k.clone())?, &v_x.clone())?)?, &Py::Int(360i64))? } else { Py::Float(0.0) })], true)?;
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
    let t1415 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("x0"))?, py::getitem(&v_i.clone(), &Py::str("x1"))?])).items()?;
    if t1415.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1415[0].clone();
    v_b = t1415[1].clone();
    v_x0 = t1415[2].clone();
    v_x1 = t1415[3].clone();
    v_nodes = Py::list(vec![Py::tuple(vec![py::neg(&Py::Float(0.9739065285171717))?, Py::Float(0.0666713443086881)]), Py::tuple(vec![py::neg(&Py::Float(0.8650633666889845))?, Py::Float(0.1494513491505806)]), Py::tuple(vec![py::neg(&Py::Float(0.6794095682990244))?, Py::Float(0.219086362515982)]), Py::tuple(vec![py::neg(&Py::Float(0.4333953941292472))?, Py::Float(0.2692667193099963)]), Py::tuple(vec![py::neg(&Py::Float(0.1488743389816312))?, Py::Float(0.2955242247147529)])]);
    v_nodes = py::iadd(&v_nodes, &{ let mut out = vec![]; for t1416 in py::iter(&(v_nodes.clone()))? { let mut v_w: Py; let mut v_x: Py; 
let t1417 = (t1416).items()?;
if t1417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1417[0].clone();
v_w = t1417[1].clone();
out.push(Py::tuple(vec![py::neg(&v_x.clone())?, v_w.clone()])); } Py::list(out) })?;
    let t1418 = (Py::tuple(vec![py::minmax(&[v_x0.clone(), v_x1.clone()], false)?, py::minmax(&[v_x0.clone(), v_x1.clone()], true)?])).items()?;
    if t1418.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1418[0].clone();
    v_hi = t1418[1].clone();
    v_n = Py::Int(64i64);
    v_h = py::truediv(&py::sub(&v_hi.clone(), &v_lo.clone())?, &v_n.clone())?;
    v_s = Py::Float(0.0);
    for t1419 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1419;
        v_m = py::add(&v_lo.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Float(0.5))?, &v_h.clone())?)?;
        v_s = py::iadd(&v_s, &py::truediv(&py::mul(&py::sum(&{ let mut out = vec![]; for t1420 in py::iter(&(v_nodes.clone()))? { let mut v_w: Py; let mut v_x: Py; 
let t1421 = (t1420).items()?;
if t1421.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1421[0].clone();
v_w = t1421[1].clone();
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
    v_pts = { let mut out = vec![]; for t1537 in py::iter(&(f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::None)?))? { let mut v_x: Py; let mut v_y: Py; 
let t1538 = (t1537).items()?;
if t1538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1538[0].clone();
v_y = t1538[1].clone();
out.push(Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])); } Py::list(out) };
    v_n = py::len(&v_pts.clone())?;
    v_area = py::truediv(&py::abs(&py::sum(&{ let mut out = vec![]; for t1539 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1539;
out.push(py::mul(&py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &py::add(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?)?, &Py::Int(2i64))?;
    v_b = Py::Int(0i64);
    for t1540 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1540;
        let t1541 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t1541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t1542 = (t1541[0].clone()).items()?;
        if t1542.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x0 = t1542[0].clone();
        v_y0 = t1542[1].clone();
        let t1543 = (t1541[1].clone()).items()?;
        if t1543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x1 = t1543[0].clone();
        v_y1 = t1543[1].clone();
        let t1544 = (Py::tuple(vec![py::sub(&v_x1.clone(), &v_x0.clone())?, py::sub(&v_y1.clone(), &v_y0.clone())?])).items()?;
        if t1544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_dx = t1544[0].clone();
        v_dy = t1544[1].clone();
        v_steps = py::minmax(&[py::abs(&v_dx.clone())?, py::abs(&v_dy.clone())?], true)?;
        v_b = py::iadd(&v_b, &py::sum(&{ let mut out = vec![]; for t1545 in py::range_iter(&[(v_steps.clone()).i()?]) { let mut v_s: Py; 
v_s = t1545;
if ({ let t1550 = { let t1546 = py::modulo(&py::mul(&v_s.clone(), &v_dx.clone())?, &v_steps.clone())?; Py::Bool({ let t1547 = Py::Int(0i64); if py::eq(&t1546, &t1547) { true } else { false } }) }; if !t1550.truthy() { t1550 } else { { let t1548 = py::modulo(&py::mul(&v_s.clone(), &v_dy.clone())?, &v_steps.clone())?; Py::Bool({ let t1549 = Py::Int(0i64); if py::eq(&t1548, &t1549) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?;
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
    let t1753 = (Py::tuple(vec![f__fr(py::getitem(&v_i.clone(), &Py::str("px"))?)?, f__fr(py::getitem(&v_i.clone(), &Py::str("py"))?)?])).items()?;
    if t1753.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t1753[0].clone();
    v_Y = t1753[1].clone();
    v_n = py::len(&v_pts.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t1754 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1754;
out.push(f__on_segment_exact(Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("px"))?, py::getitem(&v_i.clone(), &Py::str("py"))?]), py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?)?); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok({ let t1755 = py::getitem(&v_o.clone(), &Py::str("location"))?; Py::Bool({ let t1756 = Py::str("boundary"); if py::eq(&t1755, &t1756) { true } else { false } }) });
    }
    v_wn = Py::Int(0i64);
    for t1757 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1757;
        let t1759 = ({ let mut out = vec![]; for t1758 in py::iter(&(Py::tuple(vec![py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])))? { let mut v_c: Py; 
v_c = t1758;
out.push(Py::tuple(vec![py::sub(&f__fr(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, &v_X.clone())?, py::sub(&f__fr(py::getitem(&v_c.clone(), &Py::Int(1i64))?)?, &v_Y.clone())?])); } Py::list(out) }).items()?;
        if t1759.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t1760 = (t1759[0].clone()).items()?;
        if t1760.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ax = t1760[0].clone();
        v_ay = t1760[1].clone();
        let t1761 = (t1759[1].clone()).items()?;
        if t1761.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_bx = t1761[0].clone();
        v_by = t1761[1].clone();
        if ({ let t1767 = { let t1762 = v_ay.clone(); Py::Bool({ let t1763 = Py::Int(0i64); if py::le(&t1762, &t1763)? { { let t1764 = v_by.clone(); if py::lt(&t1763, &t1764)? { true } else { false } } } else { false } }) }; if !t1767.truthy() { t1767 } else { { let t1765 = py::sub(&py::mul(&v_ax.clone(), &v_by.clone())?, &py::mul(&v_bx.clone(), &v_ay.clone())?)?; Py::Bool({ let t1766 = Py::Int(0i64); if py::gt(&t1765, &t1766)? { true } else { false } }) } } }).truthy() {
            v_wn = py::iadd(&v_wn, &Py::Int(1i64))?;
        } else {
            if ({ let t1773 = { let t1768 = v_by.clone(); Py::Bool({ let t1769 = Py::Int(0i64); if py::le(&t1768, &t1769)? { { let t1770 = v_ay.clone(); if py::lt(&t1769, &t1770)? { true } else { false } } } else { false } }) }; if !t1773.truthy() { t1773 } else { { let t1771 = py::sub(&py::mul(&v_ax.clone(), &v_by.clone())?, &py::mul(&v_bx.clone(), &v_ay.clone())?)?; Py::Bool({ let t1772 = Py::Int(0i64); if py::lt(&t1771, &t1772)? { true } else { false } }) } } }).truthy() {
                v_wn = py::sub(&v_wn, &Py::Int(1i64))?;
            }
        }
    }
    return Ok({ let t1774 = py::getitem(&v_o.clone(), &Py::str("location"))?; Py::Bool({ let t1775 = (if (v_wn.clone()).truthy() { Py::str("inside") } else { Py::str("outside") }); if py::eq(&t1774, &t1775) { true } else { false } }) });
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
    v_first = (match ('g1595: { let mut out = vec![]; for t1596 in py::iter(&(v_n.clone()))? { let mut v_c: Py; 
v_c = t1596;
if ({ let t1597 = py::abs(&v_c.clone())?; Py::Bool({ let t1598 = Py::Float(1e-12); if py::gt(&t1597, &t1598)? { true } else { false } }) }).truthy() { out.push(v_c.clone()); break 'g1595 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::Float(0.0) });
    v_sc = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1599 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t1599;
for t1600 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t1600;
out.push(py::abs(&v_c.clone())?); }} Py::list(out) }], true)?], true)?;
    return Ok(py::minmax(&[py::add(&py::add(&Py::list(vec![py::abs(&py::sub(&f__vnorm(v_n.clone())?, &Py::Int(1i64))?)?]), &{ let mut out = vec![]; for t1601 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t1601;
out.push(py::truediv(&py::abs(&py::add(&f__vdot(v_n.clone(), v_p.clone())?, &py::getitem(&v_o.clone(), &Py::str("d"))?)?)?, &v_sc.clone())?); } Py::list(out) })?, &Py::list(vec![(if ({ let t1602 = v_first.clone(); Py::Bool({ let t1603 = Py::Int(0i64); if py::gt(&t1602, &t1603)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
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
    let t1626 = (py::getitem(&c__PLATONIC()?, &py::getitem(&v_i.clone(), &Py::str("solid"))?)?).items()?;
    if t1626.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1626[0].clone();
    v_E = t1626[1].clone();
    v_F = t1626[2].clone();
    v_kv = t1626[3].clone();
    v_ks = t1626[4].clone();
    v_kR = t1626[5].clone();
    v_kr = t1626[6].clone();
    v_a = py::getitem(&v_i.clone(), &Py::str("edge"))?;
    if ({ let t1627 = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("vertices"))?, py::getitem(&v_o.clone(), &Py::str("edges"))?, py::getitem(&v_o.clone(), &Py::str("faces"))?, py::getitem(&v_o.clone(), &Py::str("euler_characteristic"))?]); Py::Bool({ let t1628 = Py::tuple(vec![v_V.clone(), v_E.clone(), v_F.clone(), Py::Int(2i64)]); if py::ne(&t1627, &t1628) { true } else { false } }) }).truthy() {
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
    v_trap = py::truediv(&py::neg(&py::fsum(&{ let mut out = vec![]; for t1942 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1942;
out.push(py::mul(&py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &py::add(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?)?)?); } Py::list(out) })?)?, &Py::Int(2i64))?;
    v_scale = py::truediv(&py::pow(&py::fsum(&{ let mut out = vec![]; for t1943 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1943;
out.push(py::add(&py::abs(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, &py::abs(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) })?, &Py::Int(2i64))?, &v_n.clone())?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("area"))?, &py::abs(&v_trap.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_trap.clone())?, py::mul(&v_scale.clone(), &Py::Float(1e-06))?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::fsum(&{ let mut out = vec![]; for t1954 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1954;
out.push(py::hypot(&[py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?)?])?); } Py::list(out) })?)?, (if ({ let t1955 = py::getitem(&v_o.clone(), &Py::str("orientation"))?; Py::Bool({ let t1956 = (if ({ let t1957 = v_trap.clone(); Py::Bool({ let t1958 = Py::Int(0i64); if py::gt(&t1957, &t1958)? { true } else { false } }) }).truthy() { Py::str("counterclockwise") } else { Py::str("clockwise") }); if py::eq(&t1955, &t1956) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })], true)?);
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
    let t1805 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("px"))?, py::getitem(&v_i.clone(), &Py::str("py"))?, py::getitem(&v_i.clone(), &Py::str("pz"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])])).items()?;
    if t1805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1805[0].clone();
    v_n = t1805[1].clone();
    v_Fp = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("fx"))?, py::getitem(&v_o.clone(), &Py::str("fy"))?, py::getitem(&v_o.clone(), &Py::str("fz"))?]);
    v_sc = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1806 in py::iter(&(v_P.clone()))? { let mut v_v: Py; 
v_v = t1806;
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
    let t1833 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("px"))?, py::getitem(&v_i.clone(), &Py::str("py"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x1"))?, py::getitem(&v_i.clone(), &Py::str("y1"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x2"))?, py::getitem(&v_i.clone(), &Py::str("y2"))?])])).items()?;
    if t1833.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1833[0].clone();
    v_A = t1833[1].clone();
    v_B = t1833[2].clone();
    v_Q = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("qx"))?, py::getitem(&v_o.clone(), &Py::str("qy"))?]);
    v_L = py::dist(&v_A.clone(), &v_B.clone())?;
    v_sc = py::minmax(&[Py::Float(1.0), v_L.clone(), py::dist(&v_P.clone(), &v_A.clone())?], true)?;
    v_on = py::add(&py::truediv(&py::abs(&f__cross(v_A.clone(), v_B.clone(), v_Q.clone())?)?, &py::mul(&v_L.clone(), &v_sc.clone())?)?, &py::truediv(&py::minmax(&[Py::Float(0.0), py::sub(&py::add(&py::dist(&v_A.clone(), &v_Q.clone())?, &py::dist(&v_Q.clone(), &v_B.clone())?)?, &v_L.clone())?], true)?, &v_sc.clone())?)?;
    v_worst = py::minmax(&[v_on.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("distance"))?, &py::dist(&v_P.clone(), &v_Q.clone())?)?)?, &v_sc.clone())?], true)?;
    v_d_end = py::minmax(&[py::dist(&v_P.clone(), &v_A.clone())?, py::dist(&v_P.clone(), &v_B.clone())?], false)?;
    if ({ let t1834 = py::getitem(&v_o.clone(), &Py::str("distance"))?; Py::Bool({ let t1835 = py::add(&py::mul(&v_d_end.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-10))?)?, &Py::Float(1e-300))?; if py::gt(&t1834, &t1835)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t1836 = v_Q.clone(); Py::Bool({ let t1837 = Py::tuple(vec![v_A.clone(), v_B.clone()]); if !py::contains(&t1837, &t1836)? { true } else { false } }) }).truthy() {
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::add(&py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::getitem(&v_Q.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &py::getitem(&v_Q.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?)?)?)?, &py::mul(&v_L.clone(), &v_sc.clone())?)?], true)?;
    } else {
        v_far = (if ({ let t1838 = v_Q.clone(); Py::Bool({ let t1839 = v_A.clone(); if py::eq(&t1838, &t1839) { true } else { false } }) }).truthy() { v_B.clone() } else { v_A.clone() });
        if ({ let t1840 = py::add(&py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::getitem(&v_Q.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_far.clone(), &Py::Int(0i64))?, &py::getitem(&v_Q.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &py::getitem(&v_Q.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_far.clone(), &Py::Int(1i64))?, &py::getitem(&v_Q.clone(), &Py::Int(1i64))?)?)?)?; Py::Bool({ let t1841 = py::mul(&py::mul(&Py::Float(1e-10), &v_L.clone())?, &v_sc.clone())?; if py::gt(&t1840, &t1841)? { true } else { false } }) }).truthy() {
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
    let t2456 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t2456.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2456[0].clone();
    v_b = t2456[1].clone();
    v_c = t2456[2].clone();
    let t2457 = (Py::tuple(vec![py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("px"))?, &py::getitem(&v_o.clone(), &Py::str("x"))?)?, &Py::Int(2i64))?, py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("py"))?, &py::getitem(&v_o.clone(), &Py::str("y"))?)?, &Py::Int(2i64))?])).items()?;
    if t2457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2457[0].clone();
    v_my = t2457[1].clone();
    let t2458 = (Py::tuple(vec![py::sub(&py::getitem(&v_o.clone(), &Py::str("x"))?, &py::getitem(&v_i.clone(), &Py::str("px"))?)?, py::sub(&py::getitem(&v_o.clone(), &Py::str("y"))?, &py::getitem(&v_i.clone(), &Py::str("py"))?)?])).items()?;
    if t2458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dx = t2458[0].clone();
    v_dy = t2458[1].clone();
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
    v_P = Py::tuple(({ let mut out = vec![]; for t2659 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("point"))?))? { let mut v_v: Py; 
v_v = t2659;
out.push(py::float(&v_v.clone())?); } Py::list(out) }).items()?);
    v_k = Py::tuple(({ let mut out = vec![]; for t2660 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("axis"))?))? { let mut v_v: Py; 
v_v = t2660;
out.push(py::float(&v_v.clone())?); } Py::list(out) }).items()?);
    v_X = Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?, py::getitem(&v_o.clone(), &Py::str("z"))?]);
    v_nk = f__vnorm(v_k.clone())?;
    v_u = Py::tuple(({ let mut out = vec![]; for t2661 in py::iter(&(v_k.clone()))? { let mut v_c: Py; 
v_c = t2661;
out.push(py::truediv(&v_c.clone(), &v_nk.clone())?); } Py::list(out) }).items()?);
    v_sc = py::minmax(&[Py::Float(1.0), f__vnorm(v_P.clone())?], true)?;
    v_pp = Py::tuple(({ let mut out = vec![]; for t2662 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2662;
out.push(py::sub(&py::getitem(&v_P.clone(), &v_j.clone())?, &py::mul(&py::getitem(&v_u.clone(), &v_j.clone())?, &f__vdot(v_u.clone(), v_P.clone())?)?)?); } Py::list(out) }).items()?);
    v_xp = Py::tuple(({ let mut out = vec![]; for t2663 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2663;
out.push(py::sub(&py::getitem(&v_X.clone(), &v_j.clone())?, &py::mul(&py::getitem(&v_u.clone(), &v_j.clone())?, &f__vdot(v_u.clone(), v_X.clone())?)?)?); } Py::list(out) }).items()?);
    v_t = py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("angle"))?)?;
    v_want = Py::tuple(({ let mut out = vec![]; for t2664 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2664;
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
    let t2630 = (Py::tuple(vec![py::method(&v_i.clone(), "get", &[Py::str("cx"), Py::Float(0.0)])?, py::method(&v_i.clone(), "get", &[Py::str("cy"), Py::Float(0.0)])?])).items()?;
    if t2630.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t2630[0].clone();
    v_cy = t2630[1].clone();
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
    let t2784 = (f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(4i64), Py::Int(4i64))?).items()?;
    if t2784.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2784[0].clone();
    v_B = t2784[1].clone();
    v_C = t2784[2].clone();
    v_D = t2784[3].clone();
    let t2785 = (Py::tuple(vec![py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_D.clone(), &Py::Int(0i64))?, &py::getitem(&v_C.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_D.clone(), &Py::Int(1i64))?, &py::getitem(&v_C.clone(), &Py::Int(1i64))?)?])).items()?;
    if t2785.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rx = t2785[0].clone();
    v_ry = t2785[1].clone();
    v_sx = t2785[2].clone();
    v_sy = t2785[3].clone();
    v_den = py::sub(&py::mul(&v_rx.clone(), &v_sy.clone())?, &py::mul(&v_ry.clone(), &v_sx.clone())?)?;
    v_sc = py::minmax(&[py::hypot(&[v_rx.clone(), v_ry.clone()])?, py::hypot(&[v_sx.clone(), v_sy.clone()])?], true)?;
    if ({ let t2786 = py::getitem(&v_o.clone(), &Py::str("status"))?; Py::Bool({ let t2787 = Py::str("point"); if py::eq(&t2786, &t2787) { true } else { false } }) }).truthy() {
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
    v_collinear = { let t2792 = { let t2788 = v_den.clone(); Py::Bool({ let t2789 = Py::Int(0i64); if py::eq(&t2788, &t2789) { true } else { false } }) }; if !t2792.truthy() { t2792 } else { { let t2790 = py::abs(&f__cross(v_A.clone(), v_B.clone(), v_C.clone())?)?; Py::Bool({ let t2791 = py::mul(&py::mul(&Py::Float(1e-12), &v_sc.clone())?, &v_sc.clone())?; if py::le(&t2790, &t2791)? { true } else { false } }) } } };
    if ({ let t2793 = py::getitem(&v_o.clone(), &Py::str("status"))?; Py::Bool({ let t2794 = Py::str("overlap"); if py::eq(&t2793, &t2794) { true } else { false } }) }).truthy() {
        return Ok(v_collinear.clone());
    }
    if ({ let t2795 = v_den.clone(); Py::Bool({ let t2796 = Py::Int(0i64); if py::eq(&t2795, &t2796) { true } else { false } }) }).truthy() {
        if (py::not(&v_collinear.clone())).truthy() {
            return Ok(Py::Bool(true));
        }
        v_rr = py::add(&py::mul(&v_rx.clone(), &v_rx.clone())?, &py::mul(&v_ry.clone(), &v_ry.clone())?)?;
        v_t0 = py::truediv(&py::add(&py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_rx.clone())?, &py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_ry.clone())?)?, &v_rr.clone())?;
        v_t1 = py::truediv(&py::add(&py::mul(&py::sub(&py::getitem(&v_D.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_rx.clone())?, &py::mul(&py::sub(&py::getitem(&v_D.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_ry.clone())?)?, &v_rr.clone())?;
        return Ok({ let t2801 = { let t2797 = py::minmax(&[v_t0.clone(), v_t1.clone()], true)?; Py::Bool({ let t2798 = Py::Int(0i64); if py::lt(&t2797, &t2798)? { true } else { false } }) }; if t2801.truthy() { t2801 } else { { let t2799 = py::minmax(&[v_t0.clone(), v_t1.clone()], false)?; Py::Bool({ let t2800 = Py::Int(1i64); if py::gt(&t2799, &t2800)? { true } else { false } }) } } });
    }
    v_t = py::truediv(&py::sub(&py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_sy.clone())?, &py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_sx.clone())?)?, &v_den.clone())?;
    v_u = py::truediv(&py::sub(&py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_ry.clone())?, &py::mul(&py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_rx.clone())?)?, &v_den.clone())?;
    v_eps = Py::Float(1e-09);
    return Ok({ let t2812 = { let t2802 = v_t.clone(); Py::Bool({ let t2803 = py::neg(&v_eps.clone())?; if py::lt(&t2802, &t2803)? { true } else { false } }) }; if t2812.truthy() { t2812 } else { { let t2811 = { let t2804 = v_t.clone(); Py::Bool({ let t2805 = py::add(&Py::Int(1i64), &v_eps.clone())?; if py::gt(&t2804, &t2805)? { true } else { false } }) }; if t2811.truthy() { t2811 } else { { let t2810 = { let t2806 = v_u.clone(); Py::Bool({ let t2807 = py::neg(&v_eps.clone())?; if py::lt(&t2806, &t2807)? { true } else { false } }) }; if t2810.truthy() { t2810 } else { { let t2808 = v_u.clone(); Py::Bool({ let t2809 = py::add(&Py::Int(1i64), &v_eps.clone())?; if py::gt(&t2808, &t2809)? { true } else { false } }) } } } } } } });
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
    let t2843 = ({ let mut out = vec![]; for t2842 in py::iter(&(Py::tuple(vec![Py::str("p1"), Py::str("d1"), Py::str("p2"), Py::str("d2")])))? { let mut v_k: Py; 
v_k = t2842;
out.push(Py::tuple((py::getitem(&v_i.clone(), &v_k.clone())?).items()?)); } Py::list(out) }).items()?;
    if t2843.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P1 = t2843[0].clone();
    v_D1 = t2843[1].clone();
    v_P2 = t2843[2].clone();
    v_D2 = t2843[3].clone();
    v_w = f__vsub(v_P1.clone(), v_P2.clone())?;
    let t2844 = (Py::tuple(vec![f__vdot(v_D1.clone(), v_D1.clone())?, f__vdot(v_D1.clone(), v_D2.clone())?, f__vdot(v_D2.clone(), v_D2.clone())?])).items()?;
    if t2844.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2844[0].clone();
    v_b = t2844[1].clone();
    v_c = t2844[2].clone();
    let t2845 = (Py::tuple(vec![f__vdot(v_D1.clone(), v_w.clone())?, f__vdot(v_D2.clone(), v_w.clone())?])).items()?;
    if t2845.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t2845[0].clone();
    v_e = t2845[1].clone();
    v_den = py::sub(&py::mul(&v_a.clone(), &v_c.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?;
    if ({ let t2846 = py::getitem(&v_o.clone(), &Py::str("parallel"))?; Py::Bool({ let t2847 = Py::str("yes"); if py::eq(&t2846, &t2847) { true } else { false } }) }).truthy() {
        let t2848 = (Py::tuple(vec![Py::Float(0.0), py::truediv(&v_e.clone(), &v_c.clone())?])).items()?;
        if t2848.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2848[0].clone();
        v_t = t2848[1].clone();
    } else {
        let t2849 = (Py::tuple(vec![py::truediv(&py::sub(&py::mul(&v_b.clone(), &v_e.clone())?, &py::mul(&v_c.clone(), &v_d.clone())?)?, &v_den.clone())?, py::truediv(&py::sub(&py::mul(&v_a.clone(), &v_e.clone())?, &py::mul(&v_b.clone(), &v_d.clone())?)?, &v_den.clone())?])).items()?;
        if t2849.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2849[0].clone();
        v_t = t2849[1].clone();
    }
    v_gap = f__vnorm(Py::tuple(({ let mut out = vec![]; for t2850 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t2850;
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
    v_U = { let mut out = vec![]; for t2912 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t2912;
out.push(Py::tuple(({ let mut out = vec![]; for t2913 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t2913;
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
    let t2914 = (Py::tuple(vec![py::call(&v_arc.clone(), &[py::getitem(&v_U.clone(), &Py::Int(1i64))?, py::getitem(&v_U.clone(), &Py::Int(2i64))?])?, py::call(&v_arc.clone(), &[py::getitem(&v_U.clone(), &Py::Int(0i64))?, py::getitem(&v_U.clone(), &Py::Int(2i64))?])?, py::call(&v_arc.clone(), &[py::getitem(&v_U.clone(), &Py::Int(0i64))?, py::getitem(&v_U.clone(), &Py::Int(1i64))?])?])).items()?;
    if t2914.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2914[0].clone();
    v_b = t2914[1].clone();
    v_c = t2914[2].clone();
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
    let t3074 = ({ let mut out = vec![]; for t3072 in py::iter(&(v_P.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t3073 = (t3072).items()?;
if t3073.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3073[0].clone();
v_y = t3073[1].clone();
out.push(f__unit(py::math1("radians", &v_x.clone())?, py::math1("radians", &v_y.clone())?)?); } Py::list(out) }).items()?;
    if t3074.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3074[0].clone();
    v_b = t3074[1].clone();
    v_c = t3074[2].clone();
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
    let t3115 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t3115.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3115[0].clone();
    v_c = t3115[1].clone();
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
    v_s = py::fsum(&{ let mut out = vec![]; for t3116 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t3116;
out.push(py::mul(&(if ({ let t3117 = v_k.clone(); Py::Bool({ let t3118 = Py::tuple(vec![Py::Int(0i64), v_n.clone()]); if py::contains(&t3118, &t3117)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { (if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() { Py::Int(4i64) } else { Py::Int(2i64) }) }), &py::call(&v_f.clone(), &[py::add(&py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?, &py::mul(&v_k.clone(), &v_h.clone())?)?])?)?); } Py::list(out) })?;
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
    let t3151 = ({ let mut out = vec![]; for t3150 in py::iter(&(Py::tuple(vec![Py::str("A"), Py::str("B"), Py::str("C"), Py::str("D"), Py::str("E"), Py::str("x0"), Py::str("y0")])))? { let mut v_k: Py; 
v_k = t3150;
out.push(py::getitem(&v_i.clone(), &v_k.clone())?); } Py::list(out) }).items()?;
    if t3151.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3151[0].clone();
    v_B = t3151[1].clone();
    v_C = t3151[2].clone();
    v_D = t3151[3].clone();
    v_E = t3151[4].clone();
    v_x0 = t3151[5].clone();
    v_y0 = t3151[6].clone();
    let t3152 = (Py::tuple(vec![py::add(&py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_x0.clone())?, &py::mul(&v_B.clone(), &v_y0.clone())?)?, &v_D.clone())?, py::add(&py::add(&py::mul(&v_B.clone(), &v_x0.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_C.clone())?, &v_y0.clone())?)?, &v_E.clone())?])).items()?;
    if t3152.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3152[0].clone();
    v_q = t3152[1].clone();
    let t3153 = (Py::tuple(vec![py::add(&Py::Int(1i64), &py::mul(&v_p.clone(), &v_p.clone())?)?, py::mul(&v_p.clone(), &v_q.clone())?, py::add(&Py::Int(1i64), &py::mul(&v_q.clone(), &v_q.clone())?)?])).items()?;
    if t3153.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_EE = t3153[0].clone();
    v_FF = t3153[1].clone();
    v_GG = t3153[2].clone();
    v_nn = py::math1("sqrt", &py::add(&py::add(&Py::Int(1i64), &py::mul(&v_p.clone(), &v_p.clone())?)?, &py::mul(&v_q.clone(), &v_q.clone())?)?)?;
    let t3154 = (Py::tuple(vec![py::truediv(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_nn.clone())?, py::truediv(&v_B.clone(), &v_nn.clone())?, py::truediv(&py::mul(&Py::Int(2i64), &v_C.clone())?, &v_nn.clone())?])).items()?;
    if t3154.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_L = t3154[0].clone();
    v_M = t3154[1].clone();
    v_N = t3154[2].clone();
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
    let t3181 = (Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("cx"))?, py::getitem(&v_i.clone(), &Py::str("cy"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("px"))?, py::getitem(&v_i.clone(), &Py::str("py"))?]), py::getitem(&v_i.clone(), &Py::str("r"))?])).items()?;
    if t3181.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_C = t3181[0].clone();
    v_P = t3181[1].clone();
    v_r = t3181[2].clone();
    v_worst = py::truediv(&py::abs(&py::sub(&py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("length"))?, &Py::Int(2i64))?, &py::mul(&v_r.clone(), &v_r.clone())?)?, &py::pow(&py::dist(&v_C.clone(), &v_P.clone())?, &Py::Int(2i64))?)?)?, &py::pow(&py::dist(&v_C.clone(), &v_P.clone())?, &Py::Int(2i64))?)?;
    for t3182 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64)])))? {
        v_n = t3182;
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
    v_d = { let mut out = vec![]; for t3210 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_a: Py; 
v_a = t3210;
out.push({ let mut out = vec![]; for t3211 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_b: Py; 
v_b = t3211;
out.push(py::pow(&py::dist(&py::getitem(&v_P.clone(), &v_a.clone())?, &py::getitem(&v_P.clone(), &v_b.clone())?)?, &Py::Int(2i64))?); } Py::list(out) }); } Py::list(out) };
    v_M = py::add(&Py::list(vec![Py::list(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(1i64), Py::Int(1i64), Py::Int(1i64)])]), &{ let mut out = vec![]; for t3212 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_a: Py; 
v_a = t3212;
out.push(py::add(&Py::list(vec![Py::Int(1i64)]), &py::getitem(&v_d.clone(), &v_a.clone())?)?); } Py::list(out) })?;
    let rec3213 = py::rec_cell();
    v_det = { let v_M = v_M.clone(); let v_P = v_P.clone(); let v_d = v_d.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let rec3213 = rec3213.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_m: Py = a.get(0).cloned().unwrap_or(Py::None);
            let v_det: Py = py::rec_get(&rec3213)?;
            if ({ let t3214 = py::len(&v_m.clone())?; Py::Bool({ let t3215 = Py::Int(1i64); if py::eq(&t3214, &t3215) { true } else { false } }) }).truthy() {
                return Ok(py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?);
            }
            return Ok(py::sum(&{ let mut out = vec![]; for t3216 in py::range_iter(&[(py::len(&v_m.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3216;
if (py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &v_j.clone())?).truthy() { out.push(py::mul(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_j.clone())?, &py::getitem(&py::getitem(&v_m.clone(), &Py::Int(0i64))?, &v_j.clone())?)?, &py::call(&v_det.clone(), &[{ let mut out = vec![]; for t3217 in py::iter(&(py::slice(&v_m.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_r: Py; 
v_r = t3217;
out.push(py::add(&py::slice(&v_r.clone(), None, Some((v_j.clone()).i()?), None)?, &py::slice(&v_r.clone(), Some((py::add(&v_j.clone(), &Py::Int(1i64))?).i()?), None, None)?)?); } Py::list(out) }])?)?); }} Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec3213, &v_det);
    v_V2 = py::truediv(&py::call(&v_det.clone(), &[v_M.clone()])?, &Py::Int(288i64))?;
    v_S = Py::Float(0.0);
    for t3218 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(2i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(3i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(2i64), Py::Int(3i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)])])))? {
        let t3220 = (t3218).items()?;
        if t3220.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3220[0].clone();
        v_b = t3220[1].clone();
        v_c = t3220[2].clone();
        let t3219 = (py::sorted(&Py::tuple(vec![py::dist(&py::getitem(&v_P.clone(), &v_b.clone())?, &py::getitem(&v_P.clone(), &v_c.clone())?)?, py::dist(&py::getitem(&v_P.clone(), &v_a.clone())?, &py::getitem(&v_P.clone(), &v_c.clone())?)?, py::dist(&py::getitem(&v_P.clone(), &v_a.clone())?, &py::getitem(&v_P.clone(), &v_b.clone())?)?]), (Py::Bool(true)).truthy())?).items()?;
        if t3219.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_x = t3219[0].clone();
        v_y = t3219[1].clone();
        v_z = t3219[2].clone();
        v_S = py::iadd(&v_S, &py::mul(&Py::Float(0.25), &py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::mul(&py::mul(&py::mul(&py::add(&v_x.clone(), &py::add(&v_y.clone(), &v_z.clone())?)?, &py::sub(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_x.clone(), &py::sub(&v_y.clone(), &v_z.clone())?)?)?], true)?)?)?)?;
    }
    v_scale = py::pow(&py::minmax(&[py::minmax(&[{ let mut out = vec![]; for t3221 in py::iter(&(v_d.clone()))? { let mut v_row: Py; 
v_row = t3221;
for t3222 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t3222;
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
    let t3567 = (f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(3i64), Py::Int(3i64))?).items()?;
    if t3567.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3567[0].clone();
    v_B = t3567[1].clone();
    v_C = t3567[2].clone();
    let t3568 = (Py::tuple(vec![py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_B.clone())?])).items()?;
    if t3568.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3568[0].clone();
    v_b = t3568[1].clone();
    v_c = t3568[2].clone();
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
    let t3648 = ({ let mut out = vec![]; for t3647 in py::iter(&(Py::tuple(vec![v_o.clone(), v_a.clone(), v_b.clone()])))? { let mut v_p: Py; 
v_p = t3647;
out.push(Py::tuple(vec![f__fr(py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, f__fr(py::getitem(&v_p.clone(), &Py::Int(1i64))?)?])); } Py::list(out) }).items()?;
    if t3648.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_o = t3648[0].clone();
    v_a = t3648[1].clone();
    v_b = t3648[2].clone();
    return Ok(py::sub(&py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__xvec(mut v_p: Py) -> R {
    return Ok(Py::tuple(({ let mut out = vec![]; for t3357 in py::iter(&(v_p.clone()))? { let mut v_c: Py; 
v_c = t3357;
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
fn f_line_intersection(mut v_a1: Py, mut v_b1: Py, mut v_c1: Py, mut v_a2: Py, mut v_b2: Py, mut v_c2: Py) -> R {
    let mut v_det: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t1168 = (f__line_check(v_a1.clone(), v_b1.clone(), v_c1.clone(), Py::str("line 1"))?).items()?;
    if t1168.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t1168[0].clone();
    v_b1 = t1168[1].clone();
    v_c1 = t1168[2].clone();
    let t1169 = (f__line_check(v_a2.clone(), v_b2.clone(), v_c2.clone(), Py::str("line 2"))?).items()?;
    if t1169.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t1169[0].clone();
    v_b2 = t1169[1].clone();
    v_c2 = t1169[2].clone();
    if ({ let t1170 = py::sub(&py::mul(&f__fr(v_a1.clone())?, &f__fr(v_b2.clone())?)?, &py::mul(&f__fr(v_a2.clone())?, &f__fr(v_b1.clone())?)?)?; Py::Bool({ let t1171 = Py::Int(0i64); if py::eq(&t1170, &t1171) { true } else { false } }) }).truthy() {
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
    v_P = Py::tuple(({ let mut out = vec![]; for t1190 in py::iter(&(Py::tuple(vec![v_x0.clone(), v_y0.clone(), v_z0.clone()])))? { let mut v_v: Py; 
v_v = t1190;
out.push(f__fin(v_v.clone(), Py::str("point"))?); } Py::list(out) }).items()?);
    v_D = Py::tuple(({ let mut out = vec![]; for t1193 in py::iter(&(Py::tuple(vec![v_dx.clone(), v_dy.clone(), v_dz.clone()])))? { let mut v_v: Py; 
v_v = t1193;
out.push(f__fin(v_v.clone(), Py::str("direction"))?); } Py::list(out) }).items()?);
    let t1204 = (f__plane_check(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone(), Py::str("plane"))?).items()?;
    if t1204.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1204[0].clone();
    v_b = t1204[1].clone();
    v_c = t1204[2].clone();
    v_d = t1204[3].clone();
    if ({ let t1205 = v_D.clone(); Py::Bool({ let t1206 = Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]); if py::eq(&t1205, &t1206) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the direction vector must not be zero"))));
    }
    v_den = py::add(&py::add(&py::mul(&f__fr(v_a.clone())?, &f__fr(v_dx.clone())?)?, &py::mul(&f__fr(v_b.clone())?, &f__fr(v_dy.clone())?)?)?, &py::mul(&f__fr(v_c.clone())?, &f__fr(v_dz.clone())?)?)?;
    if ({ let t1207 = v_den.clone(); Py::Bool({ let t1208 = Py::Int(0i64); if py::eq(&t1207, &t1208) { true } else { false } }) }).truthy() {
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
    if ({ let t1250 = py::len(&v_h.clone())?; Py::Bool({ let t1251 = Py::Int(3i64); if py::lt(&t1250, &t1251)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the points are collinear: the enclosing rectangle has no area"))));
    }
    v_best = Py::None;
    v_n = py::len(&v_h.clone())?;
    for t1252 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1252;
        let t1253 = (Py::tuple(vec![py::sub(&py::getitem(&py::getitem(&v_h.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(0i64))?)?, py::sub(&py::getitem(&py::getitem(&v_h.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_h.clone(), &v_k.clone())?, &Py::Int(1i64))?)?])).items()?;
        if t1253.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ex = t1253[0].clone();
        v_ey = t1253[1].clone();
        v_L = py::hypot(&[v_ex.clone(), v_ey.clone()])?;
        let t1254 = (Py::tuple(vec![py::truediv(&v_ex.clone(), &v_L.clone())?, py::truediv(&v_ey.clone(), &v_L.clone())?])).items()?;
        if t1254.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ux = t1254[0].clone();
        v_uy = t1254[1].clone();
        v_us = { let mut out = vec![]; for t1255 in py::iter(&(v_h.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1256 = (t1255).items()?;
if t1256.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1256[0].clone();
v_y = t1256[1].clone();
out.push(py::add(&py::mul(&v_x.clone(), &v_ux.clone())?, &py::mul(&v_y.clone(), &v_uy.clone())?)?); } Py::list(out) };
        v_vs = { let mut out = vec![]; for t1257 in py::iter(&(v_h.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1258 = (t1257).items()?;
if t1258.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1258[0].clone();
v_y = t1258[1].clone();
out.push(py::add(&py::mul(&py::neg(&v_x.clone())?, &v_uy.clone())?, &py::mul(&v_y.clone(), &v_ux.clone())?)?); } Py::list(out) };
        let t1259 = (Py::tuple(vec![py::sub(&py::minmax(&[v_us.clone()], true)?, &py::minmax(&[v_us.clone()], false)?)?, py::sub(&py::minmax(&[v_vs.clone()], true)?, &py::minmax(&[v_vs.clone()], false)?)?])).items()?;
        if t1259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_w = t1259[0].clone();
        v_t = t1259[1].clone();
        if ({ let t1264 = { let t1260 = v_best.clone(); Py::Bool({ let t1261 = Py::None; if py::is(&t1260, &t1261) { true } else { false } }) }; if t1264.truthy() { t1264 } else { { let t1262 = py::mul(&v_w.clone(), &v_t.clone())?; Py::Bool({ let t1263 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t1262, &t1263)? { true } else { false } }) } } }).truthy() {
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
    for t1309 in py::iter(&(py::enumerate(&v_pts.clone(), 0)?))? {
        let t1323 = (t1309).items()?;
        if t1323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t1323[0].clone();
        v_p = t1323[1].clone();
        if ({ let t1314 = { let t1310 = v_c.clone(); Py::Bool({ let t1311 = Py::None; if !py::is(&t1310, &t1311) { true } else { false } }) }; if !t1314.truthy() { t1314 } else { f__in_circ(v_c.clone(), v_p.clone())? } }).truthy() {
            continue;
        }
        v_c = Py::tuple(vec![py::getitem(&v_p.clone(), &Py::Int(0i64))?, py::getitem(&v_p.clone(), &Py::Int(1i64))?, Py::Float(0.0)]);
        for t1315 in py::range_iter(&[(v_i.clone()).i()?]) {
            v_j = t1315;
            v_q = py::getitem(&v_pts.clone(), &v_j.clone())?;
            if (f__in_circ(v_c.clone(), v_q.clone())?).truthy() {
                continue;
            }
            v_c = Py::tuple(vec![py::truediv(&py::add(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, &py::getitem(&v_q.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, py::truediv(&py::add(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, &py::getitem(&v_q.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?, py::truediv(&py::dist(&v_p.clone(), &v_q.clone())?, &Py::Int(2i64))?]);
            for t1316 in py::range_iter(&[(v_j.clone()).i()?]) {
                v_k = t1316;
                v_r = py::getitem(&v_pts.clone(), &v_k.clone())?;
                if (f__in_circ(v_c.clone(), v_r.clone())?).truthy() {
                    continue;
                }
                v_cc = f__circ3(v_p.clone(), v_q.clone(), v_r.clone())?;
                if ({ let t1320 = v_cc.clone(); Py::Bool({ let t1321 = Py::None; if py::is(&t1320, &t1321) { true } else { false } }) }).truthy() {
                    let t1322 = (py::minmax_key(&[Py::tuple(vec![Py::tuple(vec![v_p.clone(), v_q.clone()]), Py::tuple(vec![v_p.clone(), v_r.clone()]), Py::tuple(vec![v_q.clone(), v_r.clone()])])], &{ let v_c = v_c.clone(); let v_cc = v_cc.clone(); let v_i = v_i.clone(); let v_j = v_j.clone(); let v_k = v_k.clone(); let v_p = v_p.clone(); let v_points = v_points.clone(); let v_pts = v_pts.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let a: &[Py] = &({ let mut v: Vec<Py> = vec![]; v.extend((v_e.clone()).items()?); v })[..]; if a.len() != 2 { return Err(PyErr::Type("dist expected 2 arguments".into())); } py::dist(&a[0], &a[1])? });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?).items()?;
                    if t1322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_a = t1322[0].clone();
                    v_b = t1322[1].clone();
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
    let t1405 = (Py::tuple(vec![f__fin(v_a.clone(), Py::str("a"))?, f__fin(v_b.clone(), Py::str("b"))?, f__fin(v_x0.clone(), Py::str("x0"))?, f__fin(v_x1.clone(), Py::str("x1"))?, f__fin(v_c.clone(), Py::str("c"))?])).items()?;
    if t1405.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1405[0].clone();
    v_b = t1405[1].clone();
    v_x0 = t1405[2].clone();
    v_x1 = t1405[3].clone();
    v_c = t1405[4].clone();
    if ({ let t1406 = v_x0.clone(); Py::Bool({ let t1407 = v_x1.clone(); if py::eq(&t1406, &t1407) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval is empty (x0 = x1)"))));
    }
    if ({ let t1408 = v_a.clone(); Py::Bool({ let t1409 = Py::Int(0i64); if py::eq(&t1408, &t1409) { true } else { false } }) }).truthy() {
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
    let t1410 = (Py::tuple(vec![py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_x0.clone())?, &v_b.clone())?, py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_x1.clone())?, &v_b.clone())?])).items()?;
    if t1410.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u0 = t1410[0].clone();
    v_u1 = t1410[1].clone();
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
    if (Py::Bool(({ let mut out = vec![]; for t1454 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1454;
for t1455 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t1455;
out.push({ let t1456 = v_v.clone(); Py::Bool({ let t1457 = py::int(&v_v.clone())?; if py::ne(&t1456, &t1457) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Pick's theorem needs integer (lattice) coordinates"))));
    }
    v_pts = { let mut out = vec![]; for t1458 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t1459 = (t1458).items()?;
if t1459.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1459[0].clone();
v_y = t1459[1].clone();
out.push(Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])); } Py::list(out) };
    let _ = f__simple_polygon(v_pts.clone(), Py::str("points"))?;
    v_n = py::len(&v_pts.clone())?;
    v_twice = py::abs(&py::sum(&{ let mut out = vec![]; for t1531 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1531;
out.push(py::sub(&py::mul(&py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_pts.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?)?;
    v_b = py::sum(&{ let mut out = vec![]; for t1532 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1532;
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
    v_X = { let mut out = vec![]; for t1580 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t1580;
out.push(f__xvec(v_p.clone())?); } Py::list(out) };
    v_n = f__vcross(f__vsub(py::getitem(&v_X.clone(), &Py::Int(1i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?, f__vsub(py::getitem(&v_X.clone(), &Py::Int(2i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?)?;
    if ({ let t1582 = v_n.clone(); Py::Bool({ let t1583 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t1582, &t1583) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the three points are collinear: they do not determine a plane"))));
    }
    if ({ let t1584 = (match ('g1586: { let mut out = vec![]; for t1587 in py::iter(&(v_n.clone()))? { let mut v_c: Py; 
v_c = t1587;
if ({ let t1588 = v_c.clone(); Py::Bool({ let t1589 = Py::Int(0i64); if py::ne(&t1588, &t1589) { true } else { false } }) }).truthy() { out.push(v_c.clone()); break 'g1586 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) }); Py::Bool({ let t1585 = Py::Int(0i64); if py::lt(&t1584, &t1585)? { true } else { false } }) }).truthy() {
        v_n = Py::tuple(({ let mut out = vec![]; for t1590 in py::iter(&(v_n.clone()))? { let mut v_c: Py; 
v_c = t1590;
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
    if ({ let t1615 = v_solid.clone(); Py::Bool({ let t1616 = c__PLATONIC()?; if !py::contains(&t1616, &t1615)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("solid must be one of "), &py::method(&Py::str(", "), "join", &[c__PLATONIC()?])?)?)));
    }
    v_a = f__pos(v_edge.clone(), Py::str("edge"))?;
    let t1621 = (py::getitem(&c__PLATONIC()?, &v_solid.clone())?).items()?;
    if t1621.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1621[0].clone();
    v_E = t1621[1].clone();
    v_F = t1621[2].clone();
    v_kv = t1621[3].clone();
    v_ks = t1621[4].clone();
    v_kR = t1621[5].clone();
    v_kr = t1621[6].clone();
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
    for t1734 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1734;
        if (f__on_segment_exact(v_P.clone(), py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?)?).truthy() {
            return Ok(Py::dict(vec![(Py::str("location"), Py::str("boundary"))]));
        }
    }
    let t1735 = (Py::tuple(vec![f__fr(py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, f__fr(py::getitem(&v_P.clone(), &Py::Int(1i64))?)?])).items()?;
    if t1735.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t1735[0].clone();
    v_Y = t1735[1].clone();
    v_inside = Py::Bool(false);
    for t1736 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1736;
        let t1738 = ({ let mut out = vec![]; for t1737 in py::iter(&(Py::tuple(vec![py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])))? { let mut v_c: Py; 
v_c = t1737;
out.push(Py::tuple(vec![f__fr(py::getitem(&v_c.clone(), &Py::Int(0i64))?)?, f__fr(py::getitem(&v_c.clone(), &Py::Int(1i64))?)?])); } Py::list(out) }).items()?;
        if t1738.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t1739 = (t1738[0].clone()).items()?;
        if t1739.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ax = t1739[0].clone();
        v_ay = t1739[1].clone();
        let t1740 = (t1738[1].clone()).items()?;
        if t1740.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_bx = t1740[0].clone();
        v_by = t1740[1].clone();
        if ({ let t1741 = { let t1743 = v_ay.clone(); Py::Bool({ let t1744 = v_Y.clone(); if py::gt(&t1743, &t1744)? { true } else { false } }) }; Py::Bool({ let t1742 = { let t1745 = v_by.clone(); Py::Bool({ let t1746 = v_Y.clone(); if py::gt(&t1745, &t1746)? { true } else { false } }) }; if py::ne(&t1741, &t1742) { true } else { false } }) }).truthy() {
            v_xi = py::add(&v_ax.clone(), &py::truediv(&py::mul(&py::sub(&v_Y.clone(), &v_ay.clone())?, &py::sub(&v_bx.clone(), &v_ax.clone())?)?, &py::sub(&v_by.clone(), &v_ay.clone())?)?)?;
            if ({ let t1747 = v_X.clone(); Py::Bool({ let t1748 = v_xi.clone(); if py::lt(&t1747, &t1748)? { true } else { false } }) }).truthy() {
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
    let t1789 = (Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?, f__fin(v_pz.clone(), Py::str("pz"))?])).items()?;
    if t1789.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_px = t1789[0].clone();
    v_py = t1789[1].clone();
    v_pz = t1789[2].clone();
    let t1800 = (f__plane_check(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone(), Py::str("plane"))?).items()?;
    if t1800.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1800[0].clone();
    v_b = t1800[1].clone();
    v_c = t1800[2].clone();
    v_d = t1800[3].clone();
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
    let t1820 = (Py::tuple(vec![Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?]), Py::tuple(vec![f__fin(v_x1.clone(), Py::str("x1"))?, f__fin(v_y1.clone(), Py::str("y1"))?]), Py::tuple(vec![f__fin(v_x2.clone(), Py::str("x2"))?, f__fin(v_y2.clone(), Py::str("y2"))?])])).items()?;
    if t1820.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1820[0].clone();
    v_A = t1820[1].clone();
    v_B = t1820[2].clone();
    if ({ let t1821 = v_A.clone(); Py::Bool({ let t1822 = v_B.clone(); if py::eq(&t1821, &t1822) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the segment's endpoints coincide"))));
    }
    let t1823 = (Py::tuple(vec![py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?])).items()?;
    if t1823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rx = t1823[0].clone();
    v_ry = t1823[1].clone();
    v_t = py::truediv(&py::add(&py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, &v_rx.clone())?, &py::mul(&py::sub(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, &v_ry.clone())?)?, &py::add(&py::mul(&v_rx.clone(), &v_rx.clone())?, &py::mul(&v_ry.clone(), &v_ry.clone())?)?)?;
    v_t = py::minmax(&[Py::Float(1.0), py::minmax(&[Py::Float(0.0), v_t.clone()], true)?], false)?;
    v_Q = (if ({ let t1824 = Py::Int(0i64); Py::Bool({ let t1825 = v_t.clone(); if py::lt(&t1824, &t1825)? { { let t1826 = Py::Int(1i64); if py::lt(&t1825, &t1826)? { true } else { false } } } else { false } }) }).truthy() { Py::tuple(vec![py::add(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &py::mul(&v_t.clone(), &v_rx.clone())?)?, py::add(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &py::mul(&v_t.clone(), &v_ry.clone())?)?]) } else { (if ({ let t1827 = v_t.clone(); Py::Bool({ let t1828 = Py::Int(0i64); if py::eq(&t1827, &t1828) { true } else { false } }) }).truthy() { v_A.clone() } else { v_B.clone() }) });
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
    v_sgn = (if ({ let t2164 = f__shoelace(v_pts.clone())?; Py::Bool({ let t2165 = Py::Int(0i64); if py::gt(&t2164, &t2165)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
    v_ang = Py::list(vec![]);
    for t2166 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2166;
        let t2167 = (Py::tuple(vec![py::getitem(&v_pts.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2167.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2167[0].clone();
        v_b = t2167[1].clone();
        v_c = t2167[2].clone();
        let t2168 = (Py::tuple(vec![Py::tuple(vec![py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?]), Py::tuple(vec![py::sub(&py::getitem(&v_c.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_c.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?])])).items()?;
        if t2168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2168[0].clone();
        v_v = t2168[1].clone();
        v_turn = py::math1("degrees", &py::atan2(&py::mul(&v_sgn.clone(), &py::sub(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?)?)?, &py::add(&py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_u.clone(), &Py::Int(1i64))?, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?)?)?)?;
        let _ = py::method(&v_ang.clone(), "append", &[f__r12(py::sub(&Py::Float(180.0), &v_turn.clone())?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("angles"), v_ang.clone()), (Py::str("angle_sum"), py::float(&py::mul(&py::sub(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(180i64))?)?), (Py::str("reflex_vertices"), py::sum(&{ let mut out = vec![]; for t2170 in py::iter(&(v_ang.clone()))? { let mut v_x: Py; 
v_x = t2170;
if ({ let t2171 = v_x.clone(); Py::Bool({ let t2172 = Py::Int(180i64); if py::gt(&t2171, &t2172)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)]));
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
    return Ok(Py::dict(vec![(Py::str("area"), py::abs(&v_s.clone())?), (Py::str("perimeter"), f__perimeter(v_pts.clone(), Py::Bool(true))?), (Py::str("orientation"), (if ({ let t1936 = v_s.clone(); Py::Bool({ let t1937 = Py::Int(0i64); if py::gt(&t1936, &t1937)? { true } else { false } }) }).truthy() { Py::str("counterclockwise") } else { Py::str("clockwise") }))]));
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
    let t2052 = (Py::tuple(vec![py::len(&v_pts.clone())?, Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t2052.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2052[0].clone();
    v_a = t2052[1].clone();
    v_cx = t2052[2].clone();
    v_cy = t2052[3].clone();
    let t2053 = (py::getitem(&v_pts.clone(), &Py::Int(0i64))?).items()?;
    if t2053.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t2053[0].clone();
    v_y0 = t2053[1].clone();
    v_q = { let mut out = vec![]; for t2054 in py::iter(&(v_pts.clone()))? { let mut v_x: Py; let mut v_y: Py; 
let t2055 = (t2054).items()?;
if t2055.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2055[0].clone();
v_y = t2055[1].clone();
out.push(Py::tuple(vec![py::sub(&v_x.clone(), &v_x0.clone())?, py::sub(&v_y.clone(), &v_y0.clone())?])); } Py::list(out) };
    for t2056 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t2056;
        let t2057 = (Py::tuple(vec![py::getitem(&v_q.clone(), &v_k.clone())?, py::getitem(&v_q.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2057.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t2058 = (t2057[0].clone()).items()?;
        if t2058.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xa = t2058[0].clone();
        v_ya = t2058[1].clone();
        let t2059 = (t2057[1].clone()).items()?;
        if t2059.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xb = t2059[0].clone();
        v_yb = t2059[1].clone();
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
    v_signs = py::set(&{ let mut out = vec![]; for t2296 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2296;
out.push(py::sub(&{ let t2297 = f__xcross(py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(2i64))?, &v_n.clone())?)?)?; Py::Bool({ let t2298 = Py::Int(0i64); if py::gt(&t2297, &t2298)? { true } else { false } }) }, &{ let t2299 = f__xcross(py::getitem(&v_pts.clone(), &v_k.clone())?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, py::getitem(&v_pts.clone(), &py::modulo(&py::add(&v_k.clone(), &Py::Int(2i64))?, &v_n.clone())?)?)?; Py::Bool({ let t2300 = Py::Int(0i64); if py::lt(&t2299, &t2300)? { true } else { false } }) })?); } Py::list(out) })?;
    let _ = py::method(&v_signs.clone(), "discard", &[Py::Int(0i64)])?;
    return Ok(Py::dict(vec![(Py::str("convex"), (if ({ let t2301 = py::len(&v_signs.clone())?; Py::Bool({ let t2302 = Py::Int(1i64); if py::le(&t2301, &t2302)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") }))]));
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
    let t2378 = (Py::tuple(vec![f__pos(v_length.clone(), Py::str("length"))?, f__pos(v_width.clone(), Py::str("width"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t2378.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_l = t2378[0].clone();
    v_w = t2378[1].clone();
    v_h = t2378[2].clone();
    v_lat = py::add(&py::mul(&v_l.clone(), &py::hypot(&[v_h.clone(), py::truediv(&v_w.clone(), &Py::Int(2i64))?])?)?, &py::mul(&v_w.clone(), &py::hypot(&[v_h.clone(), py::truediv(&v_l.clone(), &Py::Int(2i64))?])?)?)?;
    return Ok(Py::dict(vec![(Py::str("volume"), py::truediv(&py::mul(&py::mul(&v_l.clone(), &v_w.clone())?, &v_h.clone())?, &Py::Int(3i64))?), (Py::str("lateral_area"), v_lat.clone()), (Py::str("total_area"), py::add(&v_lat.clone(), &py::mul(&v_l.clone(), &v_w.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rectangle_measures(mut v_length: Py, mut v_width: Py) -> R {
    let mut v_l: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2411 = (Py::tuple(vec![f__pos(v_length.clone(), Py::str("length"))?, f__pos(v_width.clone(), Py::str("width"))?])).items()?;
    if t2411.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l = t2411[0].clone();
    v_w = t2411[1].clone();
    return Ok(Py::dict(vec![(Py::str("area"), py::mul(&v_l.clone(), &v_w.clone())?), (Py::str("perimeter"), py::mul(&Py::Int(2i64), &py::add(&v_l.clone(), &v_w.clone())?)?), (Py::str("diagonal"), py::hypot(&[v_l.clone(), v_w.clone()])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reflect_point_line(mut v_px: Py, mut v_py: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t2442 = (Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?])).items()?;
    if t2442.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_px = t2442[0].clone();
    v_py = t2442[1].clone();
    let t2449 = (f__line_check(v_a.clone(), v_b.clone(), v_c.clone(), Py::str("line"))?).items()?;
    if t2449.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2449[0].clone();
    v_b = t2449[1].clone();
    v_c = t2449[2].clone();
    v_k = py::truediv(&py::add(&py::add(&py::mul(&v_a.clone(), &v_px.clone())?, &py::mul(&v_b.clone(), &v_py.clone())?)?, &v_c.clone())?, &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::mul(&v_b.clone(), &v_b.clone())?)?)?;
    v_s = py::minmax(&[py::abs(&v_px.clone())?, py::abs(&v_py.clone())?], true)?;
    return Ok(Py::dict(vec![(Py::str("x"), f__snap(py::sub(&v_px.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_k.clone())?)?, v_s.clone())?), (Py::str("y"), f__snap(py::sub(&v_py.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_b.clone())?, &v_k.clone())?)?, v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_regular_polygon_angles(mut v_n: Py) -> R {
    if (Py::Bool(matches!(v_n.clone(), Py::Float(_)))).truthy() {
        if ({ let t2470 = v_n.clone(); Py::Bool({ let t2471 = py::int(&v_n.clone())?; if py::ne(&t2470, &t2471) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("n must be a whole number"))));
        }
        v_n = py::int(&v_n.clone())?;
    }
    if ({ let t2476 = { let t2472 = v_n.clone(); Py::Bool({ let t2473 = Py::Int(3i64); if py::lt(&t2472, &t2473)? { true } else { false } }) }; if t2476.truthy() { t2476 } else { { let t2474 = v_n.clone(); Py::Bool({ let t2475 = Py::Int(1000i64); if py::gt(&t2474, &t2475)? { true } else { false } }) } } }).truthy() {
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
        if ({ let t2505 = v_n.clone(); Py::Bool({ let t2506 = py::int(&v_n.clone())?; if py::ne(&t2505, &t2506) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("n must be a whole number"))));
        }
        v_n = py::int(&v_n.clone())?;
    }
    if ({ let t2511 = { let t2507 = v_n.clone(); Py::Bool({ let t2508 = Py::Int(3i64); if py::lt(&t2507, &t2508)? { true } else { false } }) }; if t2511.truthy() { t2511 } else { { let t2509 = v_n.clone(); Py::Bool({ let t2510 = Py::Int(200i64); if py::gt(&t2509, &t2510)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a regular polygon has between 3 and 200 sides here"))));
    }
    v_s = f__pos(v_side.clone(), Py::str("side"))?;
    let t2516 = (Py::tuple(vec![py::truediv(&v_s.clone(), &py::mul(&Py::Int(2i64), &py::math1("tan", &py::truediv(&Py::Float(std::f64::consts::PI), &v_n.clone())?)?)?)?, py::truediv(&v_s.clone(), &py::mul(&Py::Int(2i64), &py::math1("sin", &py::truediv(&Py::Float(std::f64::consts::PI), &v_n.clone())?)?)?)?])).items()?;
    if t2516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_apo = t2516[0].clone();
    v_cr = t2516[1].clone();
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
        if ({ let t2545 = v_n.clone(); Py::Bool({ let t2546 = py::int(&v_n.clone())?; if py::ne(&t2545, &t2546) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("n must be a whole number"))));
        }
        v_n = py::int(&v_n.clone())?;
    }
    if ({ let t2551 = { let t2547 = v_n.clone(); Py::Bool({ let t2548 = Py::Int(3i64); if py::lt(&t2547, &t2548)? { true } else { false } }) }; if t2551.truthy() { t2551 } else { { let t2549 = v_n.clone(); Py::Bool({ let t2550 = Py::Int(200i64); if py::gt(&t2549, &t2550)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the base has between 3 and 200 sides here"))));
    }
    let t2556 = (Py::tuple(vec![f__pos(v_side.clone(), Py::str("side"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t2556.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2556[0].clone();
    v_h = t2556[1].clone();
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
    let t2589 = (Py::tuple(vec![f__pos(v_d1.clone(), Py::str("d1"))?, f__pos(v_d2.clone(), Py::str("d2"))?])).items()?;
    if t2589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2589[0].clone();
    v_q = t2589[1].clone();
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
    let t2620 = (Py::tuple(vec![f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?, f__fin(v_angle.clone(), Py::str("angle"))?])).items()?;
    if t2620.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_px = t2620[0].clone();
    v_py = t2620[1].clone();
    v_t = t2620[2].clone();
    let t2621 = (Py::tuple(vec![f__fin(v_cx.clone(), Py::str("cx"))?, f__fin(v_cy.clone(), Py::str("cy"))?])).items()?;
    if t2621.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t2621[0].clone();
    v_cy = t2621[1].clone();
    v_r = py::modulo(&v_t.clone(), &Py::Float(360.0))?;
    v_exact = Py::dict(vec![(Py::Float(0.0), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::Float(90.0), Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)])), (Py::Float(180.0), Py::tuple(vec![py::neg(&Py::Float(1.0))?, Py::Float(0.0)])), (Py::Float(270.0), Py::tuple(vec![Py::Float(0.0), py::neg(&Py::Float(1.0))?]))]);
    let t2622 = (py::method(&v_exact.clone(), "get", &[v_r.clone(), Py::tuple(vec![py::math1("cos", &py::math1("radians", &v_t.clone())?)?, py::math1("sin", &py::math1("radians", &v_t.clone())?)?])])?).items()?;
    if t2622.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_co = t2622[0].clone();
    v_si = t2622[1].clone();
    let t2623 = (Py::tuple(vec![py::sub(&v_px.clone(), &v_cx.clone())?, py::sub(&v_py.clone(), &v_cy.clone())?])).items()?;
    if t2623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dx = t2623[0].clone();
    v_dy = t2623[1].clone();
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
    v_P = Py::tuple(({ let mut out = vec![]; for t2642 in py::iter(&(v_point.clone()))? { let mut v_v: Py; 
v_v = t2642;
out.push(f__fin(py::float(&v_v.clone())?, Py::str("point"))?); } Py::list(out) }).items()?);
    v_k = Py::tuple(({ let mut out = vec![]; for t2645 in py::iter(&(v_axis.clone()))? { let mut v_v: Py; 
v_v = t2645;
out.push(f__fin(py::float(&v_v.clone())?, Py::str("axis"))?); } Py::list(out) }).items()?);
    v_t = f__fin(v_angle.clone(), Py::str("angle"))?;
    v_nk = f__vnorm(v_k.clone())?;
    if ({ let t2646 = v_nk.clone(); Py::Bool({ let t2647 = Py::Int(0i64); if py::eq(&t2646, &t2647) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the axis must not be the zero vector"))));
    }
    v_k = Py::tuple(({ let mut out = vec![]; for t2648 in py::iter(&(v_k.clone()))? { let mut v_c: Py; 
v_c = t2648;
out.push(py::truediv(&v_c.clone(), &v_nk.clone())?); } Py::list(out) }).items()?);
    v_r = py::modulo(&v_t.clone(), &Py::Float(360.0))?;
    let t2649 = (py::method(&Py::dict(vec![(Py::Float(0.0), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::Float(90.0), Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)])), (Py::Float(180.0), Py::tuple(vec![py::neg(&Py::Float(1.0))?, Py::Float(0.0)])), (Py::Float(270.0), Py::tuple(vec![Py::Float(0.0), py::neg(&Py::Float(1.0))?]))]), "get", &[v_r.clone(), Py::tuple(vec![py::math1("cos", &py::math1("radians", &v_t.clone())?)?, py::math1("sin", &py::math1("radians", &v_t.clone())?)?])])?).items()?;
    if t2649.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_co = t2649[0].clone();
    v_si = t2649[1].clone();
    let t2650 = (Py::tuple(vec![f__vcross(v_k.clone(), v_P.clone())?, f__vdot(v_k.clone(), v_P.clone())?])).items()?;
    if t2650.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kxp = t2650[0].clone();
    v_kp = t2650[1].clone();
    v_out = Py::tuple(({ let mut out = vec![]; for t2651 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2651;
out.push(py::add(&py::add(&py::mul(&py::getitem(&v_P.clone(), &v_j.clone())?, &v_co.clone())?, &py::mul(&py::getitem(&v_kxp.clone(), &v_j.clone())?, &v_si.clone())?)?, &py::mul(&py::mul(&py::getitem(&v_k.clone(), &v_j.clone())?, &v_kp.clone())?, &py::sub(&Py::Int(1i64), &v_co.clone())?)?)?); } Py::list(out) }).items()?);
    v_s = py::minmax(&[{ let mut out = vec![]; for t2652 in py::iter(&(v_P.clone()))? { let mut v_c: Py; 
v_c = t2652;
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
    let t2680 = (Py::tuple(vec![f__pos(v_radius.clone(), Py::str("radius"))?, f__fin(v_angle.clone(), Py::str("angle"))?])).items()?;
    if t2680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t2680[0].clone();
    v_a = t2680[1].clone();
    if (py::not(&{ let t2681 = Py::Int(0i64); Py::Bool({ let t2682 = v_a.clone(); if py::lt(&t2681, &t2682)? { { let t2683 = Py::Int(360i64); if py::le(&t2682, &t2683)? { true } else { false } } } else { false } }) })).truthy() {
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
    let t2723 = (f__pairs(v_points.clone(), Py::str("points"), Py::Int(4i64), Py::Int(4i64))?).items()?;
    if t2723.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2723[0].clone();
    v_B = t2723[1].clone();
    v_C = t2723[2].clone();
    v_D = t2723[3].clone();
    if ({ let t2728 = { let t2724 = v_A.clone(); Py::Bool({ let t2725 = v_B.clone(); if py::eq(&t2724, &t2725) { true } else { false } }) }; if t2728.truthy() { t2728 } else { { let t2726 = v_C.clone(); Py::Bool({ let t2727 = v_D.clone(); if py::eq(&t2726, &t2727) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a segment has two equal endpoints"))));
    }
    v_none = Py::dict(vec![(Py::str("status"), Py::str("none")), (Py::str("x"), Py::None), (Py::str("y"), Py::None)]);
    if (py::not(&f__segments_meet_exact(v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone())?)).truthy() {
        return Ok(v_none.clone());
    }
    let t2773 = ({ let mut out = vec![]; for t2772 in py::iter(&({ let mut v: Vec<Py> = vec![]; v.extend((v_A.clone()).items()?); v.extend((v_B.clone()).items()?); v.extend((v_C.clone()).items()?); v.extend((v_D.clone()).items()?); Py::tuple(v) }))? { let mut v_v: Py; 
v_v = t2772;
out.push(f__fr(v_v.clone())?); } Py::list(out) }).items()?;
    if t2773.len() != 8 { return Err(PyErr::Value("unpack".into())); }
    v_ax = t2773[0].clone();
    v_ay = t2773[1].clone();
    v_bx = t2773[2].clone();
    v_by = t2773[3].clone();
    v_cx = t2773[4].clone();
    v_cy = t2773[5].clone();
    v_dx = t2773[6].clone();
    v_dy = t2773[7].clone();
    let t2774 = (Py::tuple(vec![py::sub(&v_bx.clone(), &v_ax.clone())?, py::sub(&v_by.clone(), &v_ay.clone())?, py::sub(&v_dx.clone(), &v_cx.clone())?, py::sub(&v_dy.clone(), &v_cy.clone())?])).items()?;
    if t2774.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rx = t2774[0].clone();
    v_ry = t2774[1].clone();
    v_sx = t2774[2].clone();
    v_sy = t2774[3].clone();
    v_den = py::sub(&py::mul(&v_rx.clone(), &v_sy.clone())?, &py::mul(&v_ry.clone(), &v_sx.clone())?)?;
    if ({ let t2775 = v_den.clone(); Py::Bool({ let t2776 = Py::Int(0i64); if py::ne(&t2775, &t2776) { true } else { false } }) }).truthy() {
        v_t = py::truediv(&py::sub(&py::mul(&py::sub(&v_cx.clone(), &v_ax.clone())?, &v_sy.clone())?, &py::mul(&py::sub(&v_cy.clone(), &v_ay.clone())?, &v_sx.clone())?)?, &v_den.clone())?;
        return Ok(Py::dict(vec![(Py::str("status"), Py::str("point")), (Py::str("x"), py::add(&py::float(&py::add(&v_ax.clone(), &py::mul(&v_t.clone(), &v_rx.clone())?)?)?, &Py::Float(0.0))?), (Py::str("y"), py::add(&py::float(&py::add(&v_ay.clone(), &py::mul(&v_t.clone(), &v_ry.clone())?)?)?, &Py::Float(0.0))?)]));
    }
    v_rr = py::add(&py::mul(&v_rx.clone(), &v_rx.clone())?, &py::mul(&v_ry.clone(), &v_ry.clone())?)?;
    v_t0 = py::truediv(&py::add(&py::mul(&py::sub(&v_cx.clone(), &v_ax.clone())?, &v_rx.clone())?, &py::mul(&py::sub(&v_cy.clone(), &v_ay.clone())?, &v_ry.clone())?)?, &v_rr.clone())?;
    v_t1 = py::truediv(&py::add(&py::mul(&py::sub(&v_dx.clone(), &v_ax.clone())?, &v_rx.clone())?, &py::mul(&py::sub(&v_dy.clone(), &v_ay.clone())?, &v_ry.clone())?)?, &v_rr.clone())?;
    let t2777 = (Py::tuple(vec![py::minmax(&[py::fraction(&[Py::Int(0i64)])?, py::minmax(&[v_t0.clone(), v_t1.clone()], false)?], true)?, py::minmax(&[py::fraction(&[Py::Int(1i64)])?, py::minmax(&[v_t0.clone(), v_t1.clone()], true)?], false)?])).items()?;
    if t2777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2777[0].clone();
    v_hi = t2777[1].clone();
    if ({ let t2778 = v_lo.clone(); Py::Bool({ let t2779 = v_hi.clone(); if py::eq(&t2778, &t2779) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("status"), Py::str("point")), (Py::str("x"), py::add(&py::float(&py::add(&v_ax.clone(), &py::mul(&v_lo.clone(), &v_rx.clone())?)?)?, &Py::Float(0.0))?), (Py::str("y"), py::add(&py::float(&py::add(&v_ay.clone(), &py::mul(&v_lo.clone(), &v_ry.clone())?)?)?, &Py::Float(0.0))?)]));
    }
    return Ok(Py::dict(vec![(Py::str("status"), Py::str("overlap")), (Py::str("x"), Py::None), (Py::str("y"), Py::None)]));
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
    let t2829 = ({ let mut out = vec![]; for t2824 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_p1.clone(), Py::str("p1")]), Py::tuple(vec![v_d1.clone(), Py::str("d1")]), Py::tuple(vec![v_p2.clone(), Py::str("p2")]), Py::tuple(vec![v_d2.clone(), Py::str("d2")])])))? { let mut v_n: Py; let mut v_vec: Py; 
let t2825 = (t2824).items()?;
if t2825.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_vec = t2825[0].clone();
v_n = t2825[1].clone();
out.push(Py::tuple(({ let mut out = vec![]; for t2826 in py::iter(&(v_vec.clone()))? { let mut v_v: Py; 
v_v = t2826;
out.push(f__fin(py::float(&v_v.clone())?, v_n.clone())?); } Py::list(out) }).items()?)); } Py::list(out) }).items()?;
    if t2829.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_P1 = t2829[0].clone();
    v_D1 = t2829[1].clone();
    v_P2 = t2829[2].clone();
    v_D2 = t2829[3].clone();
    if ({ let t2834 = { let t2830 = v_D1.clone(); Py::Bool({ let t2831 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t2830, &t2831) { true } else { false } }) }; if t2834.truthy() { t2834 } else { { let t2832 = v_D2.clone(); Py::Bool({ let t2833 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t2832, &t2833) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a direction vector is zero"))));
    }
    v_w = f__vsub(v_P2.clone(), v_P1.clone())?;
    v_nx = f__vcross(f__xvec(v_D1.clone())?, f__xvec(v_D2.clone())?)?;
    if ({ let t2836 = v_nx.clone(); Py::Bool({ let t2837 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t2836, &t2837) { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t2864 = Py::Int(0i64); Py::Bool({ let t2865 = v_a.clone(); if py::lt(&t2864, &t2865)? { { let t2866 = Py::Int(180i64); if py::le(&t2865, &t2866)? { true } else { false } } } else { false } }) })).truthy() {
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
    let t2901 = (f__triples(v_points.clone(), Py::str("points"), Py::Int(3i64))?).items()?;
    if t2901.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2901[0].clone();
    v_b = t2901[1].clone();
    v_c = t2901[2].clone();
    if ({ let t2902 = Py::Float(0.0); Py::Bool({ let t2903 = Py::tuple(vec![f__vnorm(v_a.clone())?, f__vnorm(v_b.clone())?, f__vnorm(v_c.clone())?]); if py::contains(&t2903, &t2902)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a vertex is at the origin (the observer)"))));
    }
    if ({ let t2904 = f__vdot(f__xvec(v_a.clone())?, f__vcross(f__xvec(v_b.clone())?, f__xvec(v_c.clone())?)?)?; Py::Bool({ let t2905 = Py::Int(0i64); if py::eq(&t2904, &t2905) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the triangle's plane passes through the origin: it subtends no solid angle"))));
    }
    let t2907 = (Py::tuple(vec![f__vnorm(v_a.clone())?, f__vnorm(v_b.clone())?, f__vnorm(v_c.clone())?])).items()?;
    if t2907.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_la = t2907[0].clone();
    v_lb = t2907[1].clone();
    v_lc = t2907[2].clone();
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
    v_X = { let mut out = vec![]; for t2942 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t2942;
out.push(f__xvec(v_p.clone())?); } Py::list(out) };
    v_rows = { let mut out = vec![]; for t2944 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)])))? { let mut v_k: Py; 
v_k = t2944;
out.push({ let mut out = vec![]; for t2945 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2945;
out.push(py::mul(&Py::Int(2i64), &py::sub(&py::getitem(&py::getitem(&v_X.clone(), &v_k.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &Py::Int(0i64))?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    v_rhs = { let mut out = vec![]; for t2946 in py::iter(&(Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)])))? { let mut v_k: Py; 
v_k = t2946;
out.push(py::sum(&{ let mut out = vec![]; for t2947 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2947;
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
    if ({ let t2948 = v_D.clone(); Py::Bool({ let t2949 = Py::Int(0i64); if py::eq(&t2948, &t2949) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the four points are coplanar: no unique sphere passes through them"))));
    }
    v_c = Py::list(vec![]);
    for t2950 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v_j = t2950;
        v_m = { let mut out = vec![]; for t2951 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t2951;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) };
        for t2952 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v_k = t2952;
            py::setitem(&py::getitem(&v_m.clone(), &v_k.clone())?, &v_j.clone(), py::getitem(&v_rhs.clone(), &v_k.clone())?)?;
        }
        let _ = py::method(&v_c.clone(), "append", &[py::truediv(&py::call(&v_det3.clone(), &[v_m.clone()])?, &v_D.clone())?])?;
    }
    v_r2 = py::sum(&{ let mut out = vec![]; for t2953 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t2953;
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
    let t3007 = (Py::tuple(vec![f__pos(v_radius.clone(), Py::str("radius"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t3007.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t3007[0].clone();
    v_h = t3007[1].clone();
    if ({ let t3008 = v_h.clone(); Py::Bool({ let t3009 = py::mul(&Py::Int(2i64), &v_R.clone())?; if py::gt(&t3008, &t3009)? { true } else { false } }) }).truthy() {
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
    v_v = { let mut out = vec![]; for t3049 in py::iter(&(py::enumerate(&v_P.clone(), 0)?))? { let mut v_a: Py; let mut v_b: Py; let mut v_k: Py; 
let t3050 = (t3049).items()?;
if t3050.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3050[0].clone();
let t3051 = (t3050[1].clone()).items()?;
if t3051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3051[0].clone();
v_b = t3051[1].clone();
out.push(f__latlon(v_a.clone(), v_b.clone(), Py::str(&["vertex ".to_string(), py::format(&py::add(&v_k.clone(), &Py::Int(1i64))?, "")?].concat()))?); } Py::list(out) };
    v_R = f__pos(v_radius.clone(), Py::str("radius"))?;
    v_U = { let mut out = vec![]; for t3061 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3061;
out.push({ let mut v: Vec<Py> = vec![]; v.extend((v_x.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "lat_r") { Some(x) => x, None => { return Err(PyErr::Type("_unit() missing argument lat_r".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "lon_r") { Some(x) => x, None => { return Err(PyErr::Type("_unit() missing argument lon_r".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("_unit() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_unit() got an unexpected keyword argument".into())) } f__unit(a0, a1)? }); } Py::list(out) };
    if ({ let t3062 = py::abs(&f__vdot(py::getitem(&v_U.clone(), &Py::Int(0i64))?, f__vcross(py::getitem(&v_U.clone(), &Py::Int(1i64))?, py::getitem(&v_U.clone(), &Py::Int(2i64))?)?)?)?; Py::Bool({ let t3063 = Py::Float(1e-12); if py::lt(&t3062, &t3063)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the vertices lie on one great circle (or two coincide): no spherical triangle"))));
    }
    let t3067 = (Py::tuple(vec![f__central(py::getitem(&v_v.clone(), &Py::Int(1i64))?, py::getitem(&v_v.clone(), &Py::Int(2i64))?)?, f__central(py::getitem(&v_v.clone(), &Py::Int(0i64))?, py::getitem(&v_v.clone(), &Py::Int(2i64))?)?, f__central(py::getitem(&v_v.clone(), &Py::Int(0i64))?, py::getitem(&v_v.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3067.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3067[0].clone();
    v_b = t3067[1].clone();
    v_c = t3067[2].clone();
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
    let t3102 = (Py::tuple(vec![f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_c.clone(), Py::str("c"))?])).items()?;
    if t3102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3102[0].clone();
    v_c = t3102[1].clone();
    if ({ let t3103 = v_a.clone(); Py::Bool({ let t3104 = v_c.clone(); if py::eq(&t3103, &t3104) { true } else { false } }) }).truthy() {
        v_S = py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_a.clone())?;
    } else {
        if ({ let t3105 = v_c.clone(); Py::Bool({ let t3106 = v_a.clone(); if py::lt(&t3105, &t3106)? { true } else { false } }) }).truthy() {
            v_e = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_c.clone(), &v_a.clone())?, &Py::Int(2i64))?)?)?;
            v_S = py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_a.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::truediv(&py::sub(&Py::Int(1i64), &py::mul(&v_e.clone(), &v_e.clone())?)?, &v_e.clone())?, &py::math1("atanh", &v_e.clone())?)?)?)?;
        } else {
            v_e = py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&v_a.clone(), &v_c.clone())?, &Py::Int(2i64))?)?)?;
            v_S = py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_a.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::truediv(&v_c.clone(), &py::mul(&v_a.clone(), &v_e.clone())?)?, &py::math1("asin", &v_e.clone())?)?)?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("surface_area"), v_S.clone()), (Py::str("volume"), py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?, &v_a.clone())?, &v_c.clone())?, &Py::Int(3i64))?), (Py::str("shape"), (if ({ let t3107 = v_a.clone(); Py::Bool({ let t3108 = v_c.clone(); if py::eq(&t3107, &t3108) { true } else { false } }) }).truthy() { Py::str("sphere") } else { (if ({ let t3109 = v_c.clone(); Py::Bool({ let t3110 = v_a.clone(); if py::lt(&t3109, &t3110)? { true } else { false } }) }).truthy() { Py::str("oblate") } else { Py::str("prolate") }) }))]));
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
    for t3140 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("A"), v_A.clone()]), Py::tuple(vec![Py::str("B"), v_B.clone()]), Py::tuple(vec![Py::str("C"), v_C.clone()]), Py::tuple(vec![Py::str("D"), v_D.clone()]), Py::tuple(vec![Py::str("E"), v_E.clone()]), Py::tuple(vec![Py::str("x0"), v_x0.clone()]), Py::tuple(vec![Py::str("y0"), v_y0.clone()]), Py::tuple(vec![Py::str("F"), v_F.clone()])])))? {
        let t3143 = (t3140).items()?;
        if t3143.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t3143[0].clone();
        v_v = t3143[1].clone();
        let _ = f__fin(v_v.clone(), v_n.clone())?;
    }
    let t3144 = (Py::tuple(vec![py::add(&py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_A.clone())?, &v_x0.clone())?, &py::mul(&v_B.clone(), &v_y0.clone())?)?, &v_D.clone())?, py::add(&py::add(&py::mul(&v_B.clone(), &v_x0.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_C.clone())?, &v_y0.clone())?)?, &v_E.clone())?])).items()?;
    if t3144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t3144[0].clone();
    v_q = t3144[1].clone();
    let t3145 = (Py::tuple(vec![py::mul(&Py::Int(2i64), &v_A.clone())?, v_B.clone(), py::mul(&Py::Int(2i64), &v_C.clone())?])).items()?;
    if t3145.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r = t3145[0].clone();
    v_s = t3145[1].clone();
    v_t = t3145[2].clone();
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
    let t3170 = (Py::tuple(vec![f__fin(v_cx.clone(), Py::str("cx"))?, f__fin(v_cy.clone(), Py::str("cy"))?, f__pos(v_r.clone(), Py::str("r"))?, f__fin(v_px.clone(), Py::str("px"))?, f__fin(v_py.clone(), Py::str("py"))?])).items()?;
    if t3170.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_cx = t3170[0].clone();
    v_cy = t3170[1].clone();
    v_r = t3170[2].clone();
    v_px = t3170[3].clone();
    v_py = t3170[4].clone();
    v_d2x = py::add(&py::pow(&py::sub(&f__fr(v_px.clone())?, &f__fr(v_cx.clone())?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&f__fr(v_py.clone())?, &f__fr(v_cy.clone())?)?, &Py::Int(2i64))?)?;
    v_g = py::sub(&v_d2x.clone(), &py::pow(&f__fr(v_r.clone())?, &Py::Int(2i64))?)?;
    if ({ let t3171 = v_g.clone(); Py::Bool({ let t3172 = Py::Int(0i64); if py::lt(&t3171, &t3172)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point is inside the circle: no tangent passes through it"))));
    }
    if ({ let t3173 = v_g.clone(); Py::Bool({ let t3174 = Py::Int(0i64); if py::eq(&t3173, &t3174) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("length"), Py::Float(0.0)), (Py::str("x1"), v_px.clone()), (Py::str("y1"), v_py.clone()), (Py::str("x2"), v_px.clone()), (Py::str("y2"), v_py.clone())]));
    }
    let t3175 = (Py::tuple(vec![py::sub(&v_px.clone(), &v_cx.clone())?, py::sub(&v_py.clone(), &v_cy.clone())?])).items()?;
    if t3175.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vx = t3175[0].clone();
    v_vy = t3175[1].clone();
    v_d2 = py::float(&v_d2x.clone())?;
    v_L = py::math1("sqrt", &py::float(&v_g.clone())?)?;
    let t3176 = (Py::tuple(vec![py::truediv(&py::mul(&v_r.clone(), &v_r.clone())?, &v_d2.clone())?, py::truediv(&py::mul(&v_r.clone(), &v_L.clone())?, &v_d2.clone())?])).items()?;
    if t3176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t3176[0].clone();
    v_gg = t3176[1].clone();
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
    v_X = { let mut out = vec![]; for t3200 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t3200;
out.push(f__xvec(v_p.clone())?); } Py::list(out) };
    if ({ let t3202 = f__vdot(f__vsub(py::getitem(&v_X.clone(), &Py::Int(1i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?, f__vcross(f__vsub(py::getitem(&v_X.clone(), &Py::Int(2i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?, f__vsub(py::getitem(&v_X.clone(), &Py::Int(3i64))?, py::getitem(&v_X.clone(), &Py::Int(0i64))?)?)?)?; Py::Bool({ let t3203 = Py::Int(0i64); if py::eq(&t3202, &t3203) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the four points are coplanar: they do not form a tetrahedron"))));
    }
    v_det = f__vdot(f__vsub(py::getitem(&v_P.clone(), &Py::Int(1i64))?, py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, f__vcross(f__vsub(py::getitem(&v_P.clone(), &Py::Int(2i64))?, py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, f__vsub(py::getitem(&v_P.clone(), &Py::Int(3i64))?, py::getitem(&v_P.clone(), &Py::Int(0i64))?)?)?)?;
    v_S = py::sum(&{ let mut out = vec![]; for t3204 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(2i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(3i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(2i64), Py::Int(3i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)])])))? { let mut v_i: Py; let mut v_j: Py; let mut v_k: Py; 
let t3205 = (t3204).items()?;
if t3205.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_i = t3205[0].clone();
v_j = t3205[1].clone();
v_k = t3205[2].clone();
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
    let t3248 = (Py::tuple(vec![f__pos(v_major.clone(), Py::str("major radius"))?, f__pos(v_minor.clone(), Py::str("minor radius"))?, f__fin(v_v.clone(), Py::str("v"))?])).items()?;
    if t3248.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_R = t3248[0].clone();
    v_r = t3248[1].clone();
    v_v = t3248[2].clone();
    if ({ let t3249 = v_r.clone(); Py::Bool({ let t3250 = v_R.clone(); if py::ge(&t3249, &t3250)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the tube radius must be smaller than the major radius (a ring torus)"))));
    }
    v_cv = py::math1("cos", &py::math1("radians", &v_v.clone())?)?;
    let t3251 = (Py::tuple(vec![py::truediv(&Py::Int(1i64), &v_r.clone())?, py::truediv(&v_cv.clone(), &py::add(&v_R.clone(), &py::mul(&v_r.clone(), &v_cv.clone())?)?)?])).items()?;
    if t3251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k1 = t3251[0].clone();
    v_k2 = t3251[1].clone();
    return Ok(Py::dict(vec![(Py::str("gaussian"), py::add(&py::mul(&v_k1.clone(), &v_k2.clone())?, &Py::Float(0.0))?), (Py::str("mean"), py::truediv(&py::add(&v_k1.clone(), &v_k2.clone())?, &Py::Int(2i64))?), (Py::str("k1"), v_k1.clone()), (Py::str("k2"), py::add(&v_k2.clone(), &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_torus_measures(mut v_major: Py, mut v_minor: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t3284 = (Py::tuple(vec![f__pos(v_major.clone(), Py::str("major radius"))?, f__pos(v_minor.clone(), Py::str("minor radius"))?])).items()?;
    if t3284.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t3284[0].clone();
    v_r = t3284[1].clone();
    if ({ let t3285 = v_r.clone(); Py::Bool({ let t3286 = v_R.clone(); if py::ge(&t3285, &t3286)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the tube radius must be smaller than the major radius (a ring torus)"))));
    }
    return Ok(Py::dict(vec![(Py::str("volume"), py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &v_R.clone())?, &v_r.clone())?, &v_r.clone())?), (Py::str("surface_area"), py::mul(&py::mul(&py::mul(&Py::Int(4i64), &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &v_R.clone())?, &v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_trapezoid_measures(mut v_a: Py, mut v_b: Py, mut v_height: Py) -> R {
    let mut v_h: Py = Py::None;
    let t3319 = (Py::tuple(vec![f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t3319.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3319[0].clone();
    v_b = t3319[1].clone();
    v_h = t3319[2].clone();
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
    let t3354 = (f__triples(v_points.clone(), Py::str("points"), Py::Int(3i64))?).items()?;
    if t3354.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3354[0].clone();
    v_B = t3354[1].clone();
    v_C = t3354[2].clone();
    if ({ let t3355 = f__vcross(f__vsub(f__xvec(v_B.clone())?, f__xvec(v_A.clone())?)?, f__vsub(f__xvec(v_C.clone())?, f__xvec(v_A.clone())?)?)?; Py::Bool({ let t3356 = Py::tuple(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(0i64)]); if py::eq(&t3355, &t3356) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the three points are collinear"))));
    }
    let t3358 = (Py::tuple(vec![py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_B.clone())?])).items()?;
    if t3358.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3358[0].clone();
    v_b = t3358[1].clone();
    v_c = t3358[2].clone();
    let t3359 = (py::sorted(&Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]), (Py::Bool(true)).truthy())?).items()?;
    if t3359.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3359[0].clone();
    v_y = t3359[1].clone();
    v_z = t3359[2].clone();
    return Ok(Py::dict(vec![(Py::str("area"), py::mul(&Py::Float(0.25), &py::math1("sqrt", &py::mul(&py::mul(&py::mul(&py::add(&v_x.clone(), &py::add(&v_y.clone(), &v_z.clone())?)?, &py::sub(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_z.clone(), &py::sub(&v_x.clone(), &v_y.clone())?)?)?, &py::add(&v_x.clone(), &py::sub(&v_y.clone(), &v_z.clone())?)?)?)?)?), (Py::str("perimeter"), py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triangle_base_height(mut v_base: Py, mut v_height: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_h: Py = Py::None;
    let t3392 = (Py::tuple(vec![f__pos(v_base.clone(), Py::str("base"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t3392.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t3392[0].clone();
    v_h = t3392[1].clone();
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
    let t3477 = (f__tri_pts(v_points.clone())?).items()?;
    if t3477.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3477[0].clone();
    v_B = t3477[1].clone();
    v_C = t3477[2].clone();
    let t3478 = (Py::tuple(vec![py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3478.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t3478[0].clone();
    v_by = t3478[1].clone();
    v_cx = t3478[2].clone();
    v_cy = t3478[3].clone();
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
    let t3507 = (f__tri_check(v_a.clone(), v_b.clone(), v_c.clone())?).items()?;
    if t3507.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3507[0].clone();
    v_b = t3507[1].clone();
    v_c = t3507[2].clone();
    let t3508 = (py::sorted(&Py::tuple(vec![f__fr(v_a.clone())?, f__fr(v_b.clone())?, f__fr(v_c.clone())?]), (Py::Bool(false)).truthy())?).items()?;
    if t3508.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3508[0].clone();
    v_y = t3508[1].clone();
    v_z = t3508[2].clone();
    v_sides = (if ({ let t3509 = v_x.clone(); Py::Bool({ let t3510 = v_z.clone(); if py::eq(&t3509, &t3510) { true } else { false } }) }).truthy() { Py::str("equilateral") } else { (if ({ let t3515 = { let t3511 = v_x.clone(); Py::Bool({ let t3512 = v_y.clone(); if py::eq(&t3511, &t3512) { true } else { false } }) }; if t3515.truthy() { t3515 } else { { let t3513 = v_y.clone(); Py::Bool({ let t3514 = v_z.clone(); if py::eq(&t3513, &t3514) { true } else { false } }) } } }).truthy() { Py::str("isosceles") } else { Py::str("scalene") }) });
    v_d = py::sub(&py::add(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&v_y.clone(), &v_y.clone())?)?, &py::mul(&v_z.clone(), &v_z.clone())?)?;
    v_angles = (if ({ let t3516 = v_d.clone(); Py::Bool({ let t3517 = Py::Int(0i64); if py::eq(&t3516, &t3517) { true } else { false } }) }).truthy() { Py::str("right") } else { (if ({ let t3518 = v_d.clone(); Py::Bool({ let t3519 = Py::Int(0i64); if py::gt(&t3518, &t3519)? { true } else { false } }) }).truthy() { Py::str("acute") } else { Py::str("obtuse") }) });
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
    let t3560 = (f__tri_pts(v_points.clone())?).items()?;
    if t3560.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3560[0].clone();
    v_B = t3560[1].clone();
    v_C = t3560[2].clone();
    v_s = f__shoelace(Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()]))?;
    let t3562 = (Py::tuple(vec![py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_B.clone())?])).items()?;
    if t3562.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3562[0].clone();
    v_b = t3562[1].clone();
    v_c = t3562[2].clone();
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
    let t3607 = (f__tri_pts(v_points.clone())?).items()?;
    if t3607.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3607[0].clone();
    v_B = t3607[1].clone();
    v_C = t3607[2].clone();
    let t3608 = (Py::tuple(vec![py::dist(&v_B.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_C.clone())?, py::dist(&v_A.clone(), &v_B.clone())?])).items()?;
    if t3608.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3608[0].clone();
    v_b = t3608[1].clone();
    v_c = t3608[2].clone();
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
    let t3649 = (f__tri_pts(v_points.clone())?).items()?;
    if t3649.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3649[0].clone();
    v_B = t3649[1].clone();
    v_C = t3649[2].clone();
    let t3650 = (Py::tuple(vec![py::sub(&py::getitem(&v_B.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_B.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_C.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_C.clone(), &Py::Int(1i64))?, &py::getitem(&v_A.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3650.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t3650[0].clone();
    v_by = t3650[1].clone();
    v_cx = t3650[2].clone();
    v_cy = t3650[3].clone();
    let t3651 = (Py::tuple(vec![py::sub(&v_cx.clone(), &v_bx.clone())?, py::sub(&v_cy.clone(), &v_by.clone())?, v_cx.clone(), v_cy.clone()])).items()?;
    if t3651.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_m11 = t3651[0].clone();
    v_m12 = t3651[1].clone();
    v_m21 = t3651[2].clone();
    v_m22 = t3651[3].clone();
    v_r2 = py::add(&py::mul(&v_bx.clone(), &v_cx.clone())?, &py::mul(&v_by.clone(), &v_cy.clone())?)?;
    v_det = py::sub(&py::mul(&v_m11.clone(), &v_m22.clone())?, &py::mul(&v_m12.clone(), &v_m21.clone())?)?;
    let t3652 = (Py::tuple(vec![py::truediv(&py::mul(&py::neg(&v_m12.clone())?, &v_r2.clone())?, &v_det.clone())?, py::truediv(&py::mul(&v_m11.clone(), &v_r2.clone())?, &v_det.clone())?])).items()?;
    if t3652.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_hx = t3652[0].clone();
    v_hy = t3652[1].clone();
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
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "c1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c1".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "a2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a2".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "b2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b2".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "c2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c2".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_line_intersection(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t1174 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1175 = (t1174).items()?;
if t1175.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1175[0].clone();
v_v = t1175[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1177(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1179: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_1178(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1179 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1180 = v_r.clone(); Py::Bool({ let t1181 = Py::Bool(true); if py::is(&t1180, &t1181) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1186 = { let t1182 = v_r.clone(); Py::Bool({ let t1183 = Py::Bool(false); if py::is(&t1182, &t1183) { true } else { false } }) }; if t1186.truthy() { t1186 } else { { let t1184 = v_r.clone(); Py::Bool({ let t1185 = Py::None; if py::is(&t1184, &t1185) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1187 = v_r.clone(); Py::Bool({ let t1188 = v_r.clone(); if py::eq(&t1187, &t1188) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1178(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__lres(py::getitem(&v_i.clone(), &Py::str("a1"))?, py::getitem(&v_i.clone(), &Py::str("b1"))?, py::getitem(&v_i.clone(), &Py::str("c1"))?, py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?)?, f__lres(py::getitem(&v_i.clone(), &Py::str("a2"))?, py::getitem(&v_i.clone(), &Py::str("b2"))?, py::getitem(&v_i.clone(), &Py::str("c2"))?, py::getitem(&v_o.clone(), &Py::str("x"))?, py::getitem(&v_o.clone(), &Py::str("y"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1189(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "x0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x0".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "y0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y0".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "z0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument z0".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "dx") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument dx".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "dy") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument dy".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "dz") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument dz".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a7 = if 7 < v.len() { v[7].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a8 = if 8 < v.len() { v[8].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; let a9 = if 9 < v.len() { v[9].clone() } else { match py::kw_take(&mut d, "d") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d".into())) } } }; if v.len() > 10 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_line_plane_intersection(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9)? };
    return Ok({ let mut out = vec![]; for t1209 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1210 = (t1209).items()?;
if t1210.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1210[0].clone();
v_v = t1210[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1212(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1217: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_lpi(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1217 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1218 = v_r.clone(); Py::Bool({ let t1219 = Py::Bool(true); if py::is(&t1218, &t1219) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1224 = { let t1220 = v_r.clone(); Py::Bool({ let t1221 = Py::Bool(false); if py::is(&t1220, &t1221) { true } else { false } }) }; if t1224.truthy() { t1224 } else { { let t1222 = v_r.clone(); Py::Bool({ let t1223 = Py::None; if py::is(&t1222, &t1223) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1225 = v_r.clone(); Py::Bool({ let t1226 = v_r.clone(); if py::eq(&t1225, &t1226) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1227(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_min_area_rectangle(a0)? };
    return Ok({ let mut out = vec![]; for t1265 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1266 = (t1265).items()?;
if t1266.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1266[0].clone();
v_v = t1266[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
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
fn s_1268(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1287: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_mar(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1287 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1288 = v_r.clone(); Py::Bool({ let t1289 = Py::Bool(true); if py::is(&t1288, &t1289) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1294 = { let t1290 = v_r.clone(); Py::Bool({ let t1291 = Py::Bool(false); if py::is(&t1290, &t1291) { true } else { false } }) }; if t1294.truthy() { t1294 } else { { let t1292 = v_r.clone(); Py::Bool({ let t1293 = Py::None; if py::is(&t1292, &t1293) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1295 = v_r.clone(); Py::Bool({ let t1296 = v_r.clone(); if py::eq(&t1295, &t1296) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1297(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_min_enclosing_circle(a0)? };
    return Ok({ let mut out = vec![]; for t1324 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1325 = (t1324).items()?;
if t1325.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1325[0].clone();
v_v = t1325[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1327(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1344: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_mec(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1344 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1345 = v_r.clone(); Py::Bool({ let t1346 = Py::Bool(true); if py::is(&t1345, &t1346) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1351 = { let t1347 = v_r.clone(); Py::Bool({ let t1348 = Py::Bool(false); if py::is(&t1347, &t1348) { true } else { false } }) }; if t1351.truthy() { t1351 } else { { let t1349 = v_r.clone(); Py::Bool({ let t1350 = Py::None; if py::is(&t1349, &t1350) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1352 = v_r.clone(); Py::Bool({ let t1353 = v_r.clone(); if py::eq(&t1352, &t1353) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1354(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_nearest_neighbor(a0, a1, a2)? };
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
    let t1392: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_nn(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1392 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1393 = v_r.clone(); Py::Bool({ let t1394 = Py::Bool(true); if py::is(&t1393, &t1394) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1399 = { let t1395 = v_r.clone(); Py::Bool({ let t1396 = Py::Bool(false); if py::is(&t1395, &t1396) { true } else { false } }) }; if t1399.truthy() { t1399 } else { { let t1397 = v_r.clone(); Py::Bool({ let t1398 = Py::None; if py::is(&t1397, &t1398) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1400 = v_r.clone(); Py::Bool({ let t1401 = v_r.clone(); if py::eq(&t1400, &t1401) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1402(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "x0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x0".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "x1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x1".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => Py::Float(0.0) } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_parabola_arc_length(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t1411 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1412 = (t1411).items()?;
if t1412.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1412[0].clone();
v_v = t1412[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1414(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1432: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_parab(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1432 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1433 = v_r.clone(); Py::Bool({ let t1434 = Py::Bool(true); if py::is(&t1433, &t1434) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1439 = { let t1435 = v_r.clone(); Py::Bool({ let t1436 = Py::Bool(false); if py::is(&t1435, &t1436) { true } else { false } }) }; if t1439.truthy() { t1439 } else { { let t1437 = v_r.clone(); Py::Bool({ let t1438 = Py::None; if py::is(&t1437, &t1438) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1440 = v_r.clone(); Py::Bool({ let t1441 = v_r.clone(); if py::eq(&t1440, &t1441) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1442(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_pick_counts(a0)? };
    return Ok({ let mut out = vec![]; for t1533 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1534 = (t1533).items()?;
if t1534.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1534[0].clone();
v_v = t1534[1].clone();
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
fn s_1536(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1563: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_pick(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1563 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1564 = v_r.clone(); Py::Bool({ let t1565 = Py::Bool(true); if py::is(&t1564, &t1565) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1570 = { let t1566 = v_r.clone(); Py::Bool({ let t1567 = Py::Bool(false); if py::is(&t1566, &t1567) { true } else { false } }) }; if t1570.truthy() { t1570 } else { { let t1568 = v_r.clone(); Py::Bool({ let t1569 = Py::None; if py::is(&t1568, &t1569) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1571 = v_r.clone(); Py::Bool({ let t1572 = v_r.clone(); if py::eq(&t1571, &t1572) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1573(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_plane_through_points(a0)? };
    return Ok({ let mut out = vec![]; for t1591 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1592 = (t1591).items()?;
if t1592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1592[0].clone();
v_v = t1592[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1594(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1604: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_plane3(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1604 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1605 = v_r.clone(); Py::Bool({ let t1606 = Py::Bool(true); if py::is(&t1605, &t1606) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1611 = { let t1607 = v_r.clone(); Py::Bool({ let t1608 = Py::Bool(false); if py::is(&t1607, &t1608) { true } else { false } }) }; if t1611.truthy() { t1611 } else { { let t1609 = v_r.clone(); Py::Bool({ let t1610 = Py::None; if py::is(&t1609, &t1610) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1612 = v_r.clone(); Py::Bool({ let t1613 = v_r.clone(); if py::eq(&t1612, &t1613) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
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
fn s_1614(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "solid") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument solid".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "edge") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument edge".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_platonic_solid(a0, a1)? };
    return Ok({ let mut out = vec![]; for t1622 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1623 = (t1622).items()?;
if t1623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1623[0].clone();
v_v = t1623[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1625(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1641: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_platonic(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1641 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1642 = v_r.clone(); Py::Bool({ let t1643 = Py::Bool(true); if py::is(&t1642, &t1643) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1648 = { let t1644 = v_r.clone(); Py::Bool({ let t1645 = Py::Bool(false); if py::is(&t1644, &t1645) { true } else { false } }) }; if t1648.truthy() { t1648 } else { { let t1646 = v_r.clone(); Py::Bool({ let t1647 = Py::None; if py::is(&t1646, &t1647) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1649 = v_r.clone(); Py::Bool({ let t1650 = v_r.clone(); if py::eq(&t1649, &t1650) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1651(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_point_in_polygon(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t1749 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1750 = (t1749).items()?;
if t1750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1750[0].clone();
v_v = t1750[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1752(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1776: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_pip(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1776 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1777 = v_r.clone(); Py::Bool({ let t1778 = Py::Bool(true); if py::is(&t1777, &t1778) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1783 = { let t1779 = v_r.clone(); Py::Bool({ let t1780 = Py::Bool(false); if py::is(&t1779, &t1780) { true } else { false } }) }; if t1783.truthy() { t1783 } else { { let t1781 = v_r.clone(); Py::Bool({ let t1782 = Py::None; if py::is(&t1781, &t1782) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1784 = v_r.clone(); Py::Bool({ let t1785 = v_r.clone(); if py::eq(&t1784, &t1785) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1786(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "pz") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument pz".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "d") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d".into())) } } }; if v.len() > 7 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_point_plane_distance(a0, a1, a2, a3, a4, a5, a6)? };
    return Ok({ let mut out = vec![]; for t1801 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1802 = (t1801).items()?;
if t1802.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1802[0].clone();
v_v = t1802[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1804(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1807: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_ppd(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1807 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1808 = v_r.clone(); Py::Bool({ let t1809 = Py::Bool(true); if py::is(&t1808, &t1809) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1814 = { let t1810 = v_r.clone(); Py::Bool({ let t1811 = Py::Bool(false); if py::is(&t1810, &t1811) { true } else { false } }) }; if t1814.truthy() { t1814 } else { { let t1812 = v_r.clone(); Py::Bool({ let t1813 = Py::None; if py::is(&t1812, &t1813) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1815 = v_r.clone(); Py::Bool({ let t1816 = v_r.clone(); if py::eq(&t1815, &t1816) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1817(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "x1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x1".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "y1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y1".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "x2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x2".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "y2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y2".into())) } } }; if v.len() > 6 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_point_segment_distance(a0, a1, a2, a3, a4, a5)? };
    return Ok({ let mut out = vec![]; for t1829 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1830 = (t1829).items()?;
if t1830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1830[0].clone();
v_v = t1830[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1832(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1842: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_psd(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1842 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1843 = v_r.clone(); Py::Bool({ let t1844 = Py::Bool(true); if py::is(&t1843, &t1844) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1849 = { let t1845 = v_r.clone(); Py::Bool({ let t1846 = Py::Bool(false); if py::is(&t1845, &t1846) { true } else { false } }) }; if t1849.truthy() { t1849 } else { { let t1847 = v_r.clone(); Py::Bool({ let t1848 = Py::None; if py::is(&t1847, &t1848) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1850 = v_r.clone(); Py::Bool({ let t1851 = v_r.clone(); if py::eq(&t1850, &t1851) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1852(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polygon_area(a0)? };
    return Ok({ let mut out = vec![]; for t1938 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t1939 = (t1938).items()?;
if t1939.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t1939[0].clone();
v_v = t1939[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
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
fn s_1941(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1959: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_polygon_area(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1959 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1960 = v_r.clone(); Py::Bool({ let t1961 = Py::Bool(true); if py::is(&t1960, &t1961) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t1966 = { let t1962 = v_r.clone(); Py::Bool({ let t1963 = Py::Bool(false); if py::is(&t1962, &t1963) { true } else { false } }) }; if t1966.truthy() { t1966 } else { { let t1964 = v_r.clone(); Py::Bool({ let t1965 = Py::None; if py::is(&t1964, &t1965) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t1967 = v_r.clone(); Py::Bool({ let t1968 = v_r.clone(); if py::eq(&t1967, &t1968) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1969(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polygon_centroid(a0)? };
    return Ok({ let mut out = vec![]; for t2060 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2061 = (t2060).items()?;
if t2061.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2061[0].clone();
v_v = t2061[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2063(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2071: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_centroid(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2071 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2072 = v_r.clone(); Py::Bool({ let t2073 = Py::Bool(true); if py::is(&t2072, &t2073) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2078 = { let t2074 = v_r.clone(); Py::Bool({ let t2075 = Py::Bool(false); if py::is(&t2074, &t2075) { true } else { false } }) }; if t2078.truthy() { t2078 } else { { let t2076 = v_r.clone(); Py::Bool({ let t2077 = Py::None; if py::is(&t2076, &t2077) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2079 = v_r.clone(); Py::Bool({ let t2080 = v_r.clone(); if py::eq(&t2079, &t2080) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2081(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polygon_angles(a0)? };
    return Ok({ let mut out = vec![]; for t2173 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2174 = (t2173).items()?;
if t2174.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2174[0].clone();
v_v = t2174[1].clone();
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
fn s_2175(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2203: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_pangles(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2203 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2204 = v_r.clone(); Py::Bool({ let t2205 = Py::Bool(true); if py::is(&t2204, &t2205) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2210 = { let t2206 = v_r.clone(); Py::Bool({ let t2207 = Py::Bool(false); if py::is(&t2206, &t2207) { true } else { false } }) }; if t2210.truthy() { t2210 } else { { let t2208 = v_r.clone(); Py::Bool({ let t2209 = Py::None; if py::is(&t2208, &t2209) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2211 = v_r.clone(); Py::Bool({ let t2212 = v_r.clone(); if py::eq(&t2211, &t2212) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2213(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polygon_is_convex(a0)? };
    return Ok({ let mut out = vec![]; for t2303 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2304 = (t2303).items()?;
if t2304.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2304[0].clone();
v_v = t2304[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2306(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2321: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_convex(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2321 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2322 = v_r.clone(); Py::Bool({ let t2323 = Py::Bool(true); if py::is(&t2322, &t2323) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2328 = { let t2324 = v_r.clone(); Py::Bool({ let t2325 = Py::Bool(false); if py::is(&t2324, &t2325) { true } else { false } }) }; if t2328.truthy() { t2328 } else { { let t2326 = v_r.clone(); Py::Bool({ let t2327 = Py::None; if py::is(&t2326, &t2327) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2329 = v_r.clone(); Py::Bool({ let t2330 = v_r.clone(); if py::eq(&t2329, &t2330) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2331(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_polyline_length(a0)? };
    return Ok({ let mut out = vec![]; for t2344 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2345 = (t2344).items()?;
if t2345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2345[0].clone();
v_v = t2345[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2347(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2363: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2348(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2363 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2364 = v_r.clone(); Py::Bool({ let t2365 = Py::Bool(true); if py::is(&t2364, &t2365) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2370 = { let t2366 = v_r.clone(); Py::Bool({ let t2367 = Py::Bool(false); if py::is(&t2366, &t2367) { true } else { false } }) }; if t2370.truthy() { t2370 } else { { let t2368 = v_r.clone(); Py::Bool({ let t2369 = Py::None; if py::is(&t2368, &t2369) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2371 = v_r.clone(); Py::Bool({ let t2372 = v_r.clone(); if py::eq(&t2371, &t2372) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2348(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("length"))?, py::sum(&{ let mut out = vec![]; for t2361 in py::iter(&(py::zip(&[v_p.clone(), py::slice(&v_p.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2362 = (t2361).items()?;
if t2362.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2362[0].clone();
v_b = t2362[1].clone();
out.push(py::math1("sqrt", &py::add(&py::pow(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?, &Py::Int(2i64))?, &py::pow(&py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?)?); } Py::list(out) }, None)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("segments"))?, py::sub(&py::len(&v_p.clone())?, &Py::Int(1i64))?])]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__pairs(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(2i64), Py::None)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2373(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "length") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument length".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "width") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument width".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_pyramid_measures(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t2379 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2380 = (t2379).items()?;
if t2380.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2380[0].clone();
v_v = t2380[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2382(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2396: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2383(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2396 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2397 = v_r.clone(); Py::Bool({ let t2398 = Py::Bool(true); if py::is(&t2397, &t2398) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2403 = { let t2399 = v_r.clone(); Py::Bool({ let t2400 = Py::Bool(false); if py::is(&t2399, &t2400) { true } else { false } }) }; if t2403.truthy() { t2403 } else { { let t2401 = v_r.clone(); Py::Bool({ let t2402 = Py::None; if py::is(&t2401, &t2402) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2404 = v_r.clone(); Py::Bool({ let t2405 = v_r.clone(); if py::eq(&t2404, &t2405) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2383(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_2406(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "length") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument length".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "width") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument width".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_rectangle_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t2412 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2413 = (t2412).items()?;
if t2413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2413[0].clone();
v_v = t2413[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2415(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2429: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2416(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2429 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2430 = v_r.clone(); Py::Bool({ let t2431 = Py::Bool(true); if py::is(&t2430, &t2431) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2436 = { let t2432 = v_r.clone(); Py::Bool({ let t2433 = Py::Bool(false); if py::is(&t2432, &t2433) { true } else { false } }) }; if t2436.truthy() { t2436 } else { { let t2434 = v_r.clone(); Py::Bool({ let t2435 = Py::None; if py::is(&t2434, &t2435) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2437 = v_r.clone(); Py::Bool({ let t2438 = v_r.clone(); if py::eq(&t2437, &t2438) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2416(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("length"))?, &py::getitem(&v_i.clone(), &Py::str("width"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::mul(&Py::Int(2i64), &py::add(&py::getitem(&v_i.clone(), &Py::str("length"))?, &py::getitem(&v_i.clone(), &Py::str("width"))?)?)?]), Py::tuple(vec![py::pow(&py::getitem(&v_o.clone(), &Py::str("diagonal"))?, &Py::Int(2i64))?, py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("length"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("width"))?, &Py::Int(2i64))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2439(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_reflect_point_line(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t2452 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2453 = (t2452).items()?;
if t2453.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2453[0].clone();
v_v = t2453[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
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
fn s_2455(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2459: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_reflect(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2459 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2460 = v_r.clone(); Py::Bool({ let t2461 = Py::Bool(true); if py::is(&t2460, &t2461) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2466 = { let t2462 = v_r.clone(); Py::Bool({ let t2463 = Py::Bool(false); if py::is(&t2462, &t2463) { true } else { false } }) }; if t2466.truthy() { t2466 } else { { let t2464 = v_r.clone(); Py::Bool({ let t2465 = Py::None; if py::is(&t2464, &t2465) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2467 = v_r.clone(); Py::Bool({ let t2468 = v_r.clone(); if py::eq(&t2467, &t2468) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2469(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "n") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument n".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_regular_polygon_angles(a0)? };
    return Ok({ let mut out = vec![]; for t2477 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2478 = (t2477).items()?;
if t2478.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2478[0].clone();
v_v = t2478[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2480(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2494: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2481(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2494 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2495 = v_r.clone(); Py::Bool({ let t2496 = Py::Bool(true); if py::is(&t2495, &t2496) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2501 = { let t2497 = v_r.clone(); Py::Bool({ let t2498 = Py::Bool(false); if py::is(&t2497, &t2498) { true } else { false } }) }; if t2501.truthy() { t2501 } else { { let t2499 = v_r.clone(); Py::Bool({ let t2500 = Py::None; if py::is(&t2499, &t2500) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2502 = v_r.clone(); Py::Bool({ let t2503 = v_r.clone(); if py::eq(&t2502, &t2503) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2481(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::add(&py::getitem(&v_o.clone(), &Py::str("interior_angle"))?, &py::getitem(&v_o.clone(), &Py::str("exterior_angle"))?)?, Py::Float(180.0)]), Py::tuple(vec![py::mul(&py::getitem(&v_o.clone(), &Py::str("exterior_angle"))?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?, Py::Float(360.0)]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("interior_sum"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("interior_angle"))?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("diagonals"))?, py::sub(&py::comb(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &Py::Int(2i64))?, &py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2504(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "n") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument n".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "side") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument side".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_regular_polygon_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t2517 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2518 = (t2517).items()?;
if t2518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2518[0].clone();
v_v = t2518[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2520(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2534: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2521(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2534 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2535 = v_r.clone(); Py::Bool({ let t2536 = Py::Bool(true); if py::is(&t2535, &t2536) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2541 = { let t2537 = v_r.clone(); Py::Bool({ let t2538 = Py::Bool(false); if py::is(&t2537, &t2538) { true } else { false } }) }; if t2541.truthy() { t2541 } else { { let t2539 = v_r.clone(); Py::Bool({ let t2540 = Py::None; if py::is(&t2539, &t2540) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2542 = v_r.clone(); Py::Bool({ let t2543 = v_r.clone(); if py::eq(&t2542, &t2543) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2521(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::add(&py::pow(&py::getitem(&v_o.clone(), &Py::str("apothem"))?, &Py::Int(2i64))?, &py::pow(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("side"))?, &Py::Int(2i64))?, &Py::Int(2i64))?)?, py::pow(&py::getitem(&v_o.clone(), &Py::str("circumradius"))?, &Py::Int(2i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("perimeter"))?, &py::getitem(&v_o.clone(), &Py::str("apothem"))?)?, &Py::Int(2i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::getitem(&v_i.clone(), &Py::str("side"))?)?]), Py::tuple(vec![py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("circumradius"))?, &py::math1("sin", &py::truediv(&Py::Float(std::f64::consts::PI), &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?)?, &Py::Int(2i64))?, py::getitem(&v_i.clone(), &Py::str("side"))?])]))?);
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
fn s_2544(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "n") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument n".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "side") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument side".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_regular_prism(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t2557 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2558 = (t2557).items()?;
if t2558.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2558[0].clone();
v_v = t2558[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2560(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2574: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2561(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2574 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2575 = v_r.clone(); Py::Bool({ let t2576 = Py::Bool(true); if py::is(&t2575, &t2576) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2581 = { let t2577 = v_r.clone(); Py::Bool({ let t2578 = Py::Bool(false); if py::is(&t2577, &t2578) { true } else { false } }) }; if t2581.truthy() { t2581 } else { { let t2579 = v_r.clone(); Py::Bool({ let t2580 = Py::None; if py::is(&t2579, &t2580) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2582 = v_r.clone(); Py::Bool({ let t2583 = v_r.clone(); if py::eq(&t2582, &t2583) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2561(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("base_area"))?, &Py::Int(4i64))?, &py::math1("tan", &py::truediv(&Py::Float(std::f64::consts::PI), &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("side"))?, &Py::Int(2i64))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&py::getitem(&v_o.clone(), &Py::str("base_area"))?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::getitem(&v_i.clone(), &Py::str("side"))?)?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("total_area"))?, py::add(&py::getitem(&v_o.clone(), &Py::str("lateral_area"))?, &py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("base_area"))?)?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2584(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "d1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "d2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d2".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_rhombus_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t2590 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2591 = (t2590).items()?;
if t2591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2591[0].clone();
v_v = t2591[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2593(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2607: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2594(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2607 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2608 = v_r.clone(); Py::Bool({ let t2609 = Py::Bool(true); if py::is(&t2608, &t2609) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2614 = { let t2610 = v_r.clone(); Py::Bool({ let t2611 = Py::Bool(false); if py::is(&t2610, &t2611) { true } else { false } }) }; if t2614.truthy() { t2614 } else { { let t2612 = v_r.clone(); Py::Bool({ let t2613 = Py::None; if py::is(&t2612, &t2613) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2615 = v_r.clone(); Py::Bool({ let t2616 = v_r.clone(); if py::eq(&t2615, &t2616) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2594(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("d1"))?, &py::getitem(&v_i.clone(), &Py::str("d2"))?)?, &Py::Int(2i64))?]), Py::tuple(vec![py::mul(&Py::Int(4i64), &py::pow(&py::getitem(&v_o.clone(), &Py::str("side"))?, &Py::Int(2i64))?)?, py::mul(&py::truediv(&py::add(&py::pow(&py::getitem(&v_i.clone(), &Py::str("d1"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("d2"))?, &Py::Int(2i64))?)?, &Py::Int(4i64))?, &Py::Int(4i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("perimeter"))?, py::mul(&Py::Int(4i64), &py::getitem(&v_o.clone(), &Py::str("side"))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2617(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "angle") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument angle".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "cx") { Some(x) => x, None => Py::Float(0.0) } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "cy") { Some(x) => x, None => Py::Float(0.0) } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_rotate_point(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t2626 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2627 = (t2626).items()?;
if t2627.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2627[0].clone();
v_v = t2627[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2629(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2631: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_rotate(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2631 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2632 = v_r.clone(); Py::Bool({ let t2633 = Py::Bool(true); if py::is(&t2632, &t2633) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2638 = { let t2634 = v_r.clone(); Py::Bool({ let t2635 = Py::Bool(false); if py::is(&t2634, &t2635) { true } else { false } }) }; if t2638.truthy() { t2638 } else { { let t2636 = v_r.clone(); Py::Bool({ let t2637 = Py::None; if py::is(&t2636, &t2637) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2639 = v_r.clone(); Py::Bool({ let t2640 = v_r.clone(); if py::eq(&t2639, &t2640) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2641(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "point") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument point".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "axis") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument axis".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "angle") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument angle".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_rotate_point_3d(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t2655 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2656 = (t2655).items()?;
if t2656.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2656[0].clone();
v_v = t2656[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2658(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2665: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_rot3(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2665 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2666 = v_r.clone(); Py::Bool({ let t2667 = Py::Bool(true); if py::is(&t2666, &t2667) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2672 = { let t2668 = v_r.clone(); Py::Bool({ let t2669 = Py::Bool(false); if py::is(&t2668, &t2669) { true } else { false } }) }; if t2672.truthy() { t2672 } else { { let t2670 = v_r.clone(); Py::Bool({ let t2671 = Py::None; if py::is(&t2670, &t2671) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2673 = v_r.clone(); Py::Bool({ let t2674 = v_r.clone(); if py::eq(&t2673, &t2674) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2675(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "angle") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument angle".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_sector_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t2684 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2685 = (t2684).items()?;
if t2685.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2685[0].clone();
v_v = t2685[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2687(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2701: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2688(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2701 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2702 = v_r.clone(); Py::Bool({ let t2703 = Py::Bool(true); if py::is(&t2702, &t2703) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2708 = { let t2704 = v_r.clone(); Py::Bool({ let t2705 = Py::Bool(false); if py::is(&t2704, &t2705) { true } else { false } }) }; if t2708.truthy() { t2708 } else { { let t2706 = v_r.clone(); Py::Bool({ let t2707 = Py::None; if py::is(&t2706, &t2707) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2709 = v_r.clone(); Py::Bool({ let t2710 = v_r.clone(); if py::eq(&t2709, &t2710) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2688(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("arc_length"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("angle"))?)?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("sector_area"))?, py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("arc_length"))?, &py::getitem(&v_i.clone(), &Py::str("radius"))?)?, &Py::Int(2i64))?]), Py::tuple(vec![py::pow(&py::getitem(&v_o.clone(), &Py::str("chord"))?, &Py::Int(2i64))?, py::mul(&py::mul(&Py::Int(2i64), &py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(2i64))?)?, &py::sub(&Py::Int(1i64), &py::math1("cos", &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("angle"))?)?)?)?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2711(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_segment_intersection(a0)? };
    return Ok({ let mut out = vec![]; for t2780 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2781 = (t2780).items()?;
if t2781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2781[0].clone();
v_v = t2781[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
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
fn s_2783(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2813: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_seg(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2813 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2814 = v_r.clone(); Py::Bool({ let t2815 = Py::Bool(true); if py::is(&t2814, &t2815) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2820 = { let t2816 = v_r.clone(); Py::Bool({ let t2817 = Py::Bool(false); if py::is(&t2816, &t2817) { true } else { false } }) }; if t2820.truthy() { t2820 } else { { let t2818 = v_r.clone(); Py::Bool({ let t2819 = Py::None; if py::is(&t2818, &t2819) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2821 = v_r.clone(); Py::Bool({ let t2822 = v_r.clone(); if py::eq(&t2821, &t2822) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2823(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "p1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument p1".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "d1") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d1".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "p2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument p2".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "d2") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument d2".into())) } } }; if v.len() > 4 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_skew_lines_distance(a0, a1, a2, a3)? };
    return Ok({ let mut out = vec![]; for t2838 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2839 = (t2838).items()?;
if t2839.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2839[0].clone();
v_v = t2839[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2841(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2851: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_skew(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2851 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2852 = v_r.clone(); Py::Bool({ let t2853 = Py::Bool(true); if py::is(&t2852, &t2853) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2858 = { let t2854 = v_r.clone(); Py::Bool({ let t2855 = Py::Bool(false); if py::is(&t2854, &t2855) { true } else { false } }) }; if t2858.truthy() { t2858 } else { { let t2856 = v_r.clone(); Py::Bool({ let t2857 = Py::None; if py::is(&t2856, &t2857) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2859 = v_r.clone(); Py::Bool({ let t2860 = v_r.clone(); if py::eq(&t2859, &t2860) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2861(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "half_angle") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument half_angle".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_solid_angle_cone(a0)? };
    return Ok({ let mut out = vec![]; for t2867 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2868 = (t2867).items()?;
if t2868.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2868[0].clone();
v_v = t2868[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2870(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2884: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2871(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2884 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2885 = v_r.clone(); Py::Bool({ let t2886 = Py::Bool(true); if py::is(&t2885, &t2886) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2891 = { let t2887 = v_r.clone(); Py::Bool({ let t2888 = Py::Bool(false); if py::is(&t2887, &t2888) { true } else { false } }) }; if t2891.truthy() { t2891 } else { { let t2889 = v_r.clone(); Py::Bool({ let t2890 = Py::None; if py::is(&t2889, &t2890) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2892 = v_r.clone(); Py::Bool({ let t2893 = v_r.clone(); if py::eq(&t2892, &t2893) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2871(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("solid_angle"))?, py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::sub(&Py::Int(1i64), &py::math1("cos", &py::math1("radians", &py::getitem(&v_i.clone(), &Py::str("half_angle"))?)?)?)?)?]), Py::tuple(vec![py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("fraction_of_sphere"))?, &Py::Int(4i64))?, &Py::Float(std::f64::consts::PI))?, py::getitem(&v_o.clone(), &Py::str("solid_angle"))?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2894(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_solid_angle_triangle(a0)? };
    return Ok({ let mut out = vec![]; for t2908 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2909 = (t2908).items()?;
if t2909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2909[0].clone();
v_v = t2909[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
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
fn s_2911(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2925: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_solid_tri(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2925 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2926 = v_r.clone(); Py::Bool({ let t2927 = Py::Bool(true); if py::is(&t2926, &t2927) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2932 = { let t2928 = v_r.clone(); Py::Bool({ let t2929 = Py::Bool(false); if py::is(&t2928, &t2929) { true } else { false } }) }; if t2932.truthy() { t2932 } else { { let t2930 = v_r.clone(); Py::Bool({ let t2931 = Py::None; if py::is(&t2930, &t2931) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2933 = v_r.clone(); Py::Bool({ let t2934 = v_r.clone(); if py::eq(&t2933, &t2934) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
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
fn s_2935(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_sphere_four_points(a0)? };
    return Ok({ let mut out = vec![]; for t2954 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2955 = (t2954).items()?;
if t2955.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2955[0].clone();
v_v = t2955[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2957(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2960: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2958(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2960 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2961 = v_r.clone(); Py::Bool({ let t2962 = Py::Bool(true); if py::is(&t2961, &t2962) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2967 = { let t2963 = v_r.clone(); Py::Bool({ let t2964 = Py::Bool(false); if py::is(&t2963, &t2964) { true } else { false } }) }; if t2967.truthy() { t2967 } else { { let t2965 = v_r.clone(); Py::Bool({ let t2966 = Py::None; if py::is(&t2965, &t2966) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t2968 = v_r.clone(); Py::Bool({ let t2969 = v_r.clone(); if py::eq(&t2968, &t2969) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2958(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_P: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[{ let mut out = vec![]; for t2959 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t2959;
out.push(py::truediv(&py::abs(&py::sub(&py::dist(&Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("cx"))?, py::getitem(&v_o.clone(), &Py::str("cy"))?, py::getitem(&v_o.clone(), &Py::str("cz"))?]), &v_p.clone())?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("radius"))?)?); } Py::list(out) }], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__triples(py::getitem(&v_i.clone(), &Py::str("points"))?, Py::str("points"), Py::Int(4i64))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2970(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_sphere_measures(a0)? };
    return Ok({ let mut out = vec![]; for t2975 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t2976 = (t2975).items()?;
if t2976.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t2976[0].clone();
v_v = t2976[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2978(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2992: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_2979(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2992 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2993 = v_r.clone(); Py::Bool({ let t2994 = Py::Bool(true); if py::is(&t2993, &t2994) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2999 = { let t2995 = v_r.clone(); Py::Bool({ let t2996 = Py::Bool(false); if py::is(&t2995, &t2996) { true } else { false } }) }; if t2999.truthy() { t2999 } else { { let t2997 = v_r.clone(); Py::Bool({ let t2998 = Py::None; if py::is(&t2997, &t2998) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3000 = v_r.clone(); Py::Bool({ let t3001 = v_r.clone(); if py::eq(&t3000, &t3001) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2979(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("volume"))?, py::truediv(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("radius"))?, &Py::Int(3i64))?)?, &Py::Int(3i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("surface_area"))?, py::truediv(&py::mul(&Py::Int(3i64), &py::getitem(&v_o.clone(), &Py::str("volume"))?)?, &py::getitem(&v_i.clone(), &Py::str("radius"))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3002(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument radius".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_spherical_cap(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3010 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3011 = (t3010).items()?;
if t3011.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3011[0].clone();
v_v = t3011[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3013(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3027: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3014(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3027 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3028 = v_r.clone(); Py::Bool({ let t3029 = Py::Bool(true); if py::is(&t3028, &t3029) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3034 = { let t3030 = v_r.clone(); Py::Bool({ let t3031 = Py::Bool(false); if py::is(&t3030, &t3031) { true } else { false } }) }; if t3034.truthy() { t3034 } else { { let t3032 = v_r.clone(); Py::Bool({ let t3033 = Py::None; if py::is(&t3032, &t3033) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3035 = v_r.clone(); Py::Bool({ let t3036 = v_r.clone(); if py::eq(&t3035, &t3036) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3014(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_3037(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "radius") { Some(x) => x, None => Py::Float(1.0) } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_spherical_triangle(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3068 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3069 = (t3068).items()?;
if t3069.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3069[0].clone();
v_v = t3069[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3071(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3087: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_sph(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3087 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3088 = v_r.clone(); Py::Bool({ let t3089 = Py::Bool(true); if py::is(&t3088, &t3089) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3094 = { let t3090 = v_r.clone(); Py::Bool({ let t3091 = Py::Bool(false); if py::is(&t3090, &t3091) { true } else { false } }) }; if t3094.truthy() { t3094 } else { { let t3092 = v_r.clone(); Py::Bool({ let t3093 = Py::None; if py::is(&t3092, &t3093) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3095 = v_r.clone(); Py::Bool({ let t3096 = v_r.clone(); if py::eq(&t3095, &t3096) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3097(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_spheroid_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3111 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3112 = (t3111).items()?;
if t3112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3112[0].clone();
v_v = t3112[1].clone();
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
fn s_3114(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3129: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_spheroid(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3129 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3130 = v_r.clone(); Py::Bool({ let t3131 = Py::Bool(true); if py::is(&t3130, &t3131) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3136 = { let t3132 = v_r.clone(); Py::Bool({ let t3133 = Py::Bool(false); if py::is(&t3132, &t3133) { true } else { false } }) }; if t3136.truthy() { t3136 } else { { let t3134 = v_r.clone(); Py::Bool({ let t3135 = Py::None; if py::is(&t3134, &t3135) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3137 = v_r.clone(); Py::Bool({ let t3138 = v_r.clone(); if py::eq(&t3137, &t3138) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3139(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument A".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "B") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument B".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "C") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument C".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "D") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument D".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "E") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument E".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "x0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument x0".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "y0") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument y0".into())) } } }; let a7 = if 7 < v.len() { v[7].clone() } else { match py::kw_take(&mut d, "F") { Some(x) => x, None => Py::Float(0.0) } }; if v.len() > 8 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_surface_curvature(a0, a1, a2, a3, a4, a5, a6, a7)? };
    return Ok({ let mut out = vec![]; for t3146 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3147 = (t3146).items()?;
if t3147.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3147[0].clone();
v_v = t3147[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3149(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3155: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_surf(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3155 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3156 = v_r.clone(); Py::Bool({ let t3157 = Py::Bool(true); if py::is(&t3156, &t3157) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3162 = { let t3158 = v_r.clone(); Py::Bool({ let t3159 = Py::Bool(false); if py::is(&t3158, &t3159) { true } else { false } }) }; if t3162.truthy() { t3162 } else { { let t3160 = v_r.clone(); Py::Bool({ let t3161 = Py::None; if py::is(&t3160, &t3161) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3163 = v_r.clone(); Py::Bool({ let t3164 = v_r.clone(); if py::eq(&t3163, &t3164) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
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
fn s_3165(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "cx") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument cx".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "cy") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument cy".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "r") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument r".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "px") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument px".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "py") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument py".into())) } } }; if v.len() > 5 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_tangent_from_point(a0, a1, a2, a3, a4)? };
    return Ok({ let mut out = vec![]; for t3177 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3178 = (t3177).items()?;
if t3178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3178[0].clone();
v_v = t3178[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3180(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3183: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_tangent(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3183 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3184 = v_r.clone(); Py::Bool({ let t3185 = Py::Bool(true); if py::is(&t3184, &t3185) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3190 = { let t3186 = v_r.clone(); Py::Bool({ let t3187 = Py::Bool(false); if py::is(&t3186, &t3187) { true } else { false } }) }; if t3190.truthy() { t3190 } else { { let t3188 = v_r.clone(); Py::Bool({ let t3189 = Py::None; if py::is(&t3188, &t3189) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3191 = v_r.clone(); Py::Bool({ let t3192 = v_r.clone(); if py::eq(&t3191, &t3192) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3193(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_tetrahedron_measures(a0)? };
    return Ok({ let mut out = vec![]; for t3206 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3207 = (t3206).items()?;
if t3207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3207[0].clone();
v_v = t3207[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3209(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3233: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_tetra(v_inputs.clone(), v_outputs.clone())?;
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
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "major") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument major".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "minor") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument minor".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "v") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument v".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_torus_curvature(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t3252 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3253 = (t3252).items()?;
if t3253.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3253[0].clone();
v_v = t3253[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3255(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3269: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3256(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3269 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3270 = v_r.clone(); Py::Bool({ let t3271 = Py::Bool(true); if py::is(&t3270, &t3271) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3276 = { let t3272 = v_r.clone(); Py::Bool({ let t3273 = Py::Bool(false); if py::is(&t3272, &t3273) { true } else { false } }) }; if t3276.truthy() { t3276 } else { { let t3274 = v_r.clone(); Py::Bool({ let t3275 = Py::None; if py::is(&t3274, &t3275) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3277 = v_r.clone(); Py::Bool({ let t3278 = v_r.clone(); if py::eq(&t3277, &t3278) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3256(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_3279(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "major") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument major".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "minor") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument minor".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_torus_measures(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3287 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3288 = (t3287).items()?;
if t3288.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3288[0].clone();
v_v = t3288[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3290(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3304: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3291(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3304 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3305 = v_r.clone(); Py::Bool({ let t3306 = Py::Bool(true); if py::is(&t3305, &t3306) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3311 = { let t3307 = v_r.clone(); Py::Bool({ let t3308 = Py::Bool(false); if py::is(&t3307, &t3308) { true } else { false } }) }; if t3311.truthy() { t3311 } else { { let t3309 = v_r.clone(); Py::Bool({ let t3310 = Py::None; if py::is(&t3309, &t3310) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3312 = v_r.clone(); Py::Bool({ let t3313 = v_r.clone(); if py::eq(&t3312, &t3313) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3291(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&py::getitem(&v_i.clone(), &Py::str("minor"))?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::getitem(&v_i.clone(), &Py::str("major"))?)?)?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("surface_area"))?, py::truediv(&py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("volume"))?)?, &py::getitem(&v_i.clone(), &Py::str("minor"))?)?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3314(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_trapezoid_measures(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t3320 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3321 = (t3320).items()?;
if t3321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3321[0].clone();
v_v = t3321[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3323(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3337: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3324(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3337 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3338 = v_r.clone(); Py::Bool({ let t3339 = Py::Bool(true); if py::is(&t3338, &t3339) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3344 = { let t3340 = v_r.clone(); Py::Bool({ let t3341 = Py::Bool(false); if py::is(&t3340, &t3341) { true } else { false } }) }; if t3344.truthy() { t3344 } else { { let t3342 = v_r.clone(); Py::Bool({ let t3343 = Py::None; if py::is(&t3342, &t3343) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3345 = v_r.clone(); Py::Bool({ let t3346 = v_r.clone(); if py::eq(&t3345, &t3346) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3324(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__res(Py::tuple(vec![Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("area"))?, py::truediv(&py::mul(&py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?, &Py::Int(2i64))?]), Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("midsegment"))?, py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &Py::Int(2i64))?])]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3347(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_area_3d(a0)? };
    return Ok({ let mut out = vec![]; for t3360 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3361 = (t3360).items()?;
if t3361.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3361[0].clone();
v_v = t3361[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3363(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3377: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3364(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3377 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3378 = v_r.clone(); Py::Bool({ let t3379 = Py::Bool(true); if py::is(&t3378, &t3379) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3384 = { let t3380 = v_r.clone(); Py::Bool({ let t3381 = Py::Bool(false); if py::is(&t3380, &t3381) { true } else { false } }) }; if t3384.truthy() { t3384 } else { { let t3382 = v_r.clone(); Py::Bool({ let t3383 = Py::None; if py::is(&t3382, &t3383) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3385 = v_r.clone(); Py::Bool({ let t3386 = v_r.clone(); if py::eq(&t3385, &t3386) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3364(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_3387(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "base") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument base".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "height") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument height".into())) } } }; if v.len() > 2 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_base_height(a0, a1)? };
    return Ok({ let mut out = vec![]; for t3393 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3394 = (t3393).items()?;
if t3394.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3394[0].clone();
v_v = t3394[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3396(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3408: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = s_3397(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3408 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3409 = v_r.clone(); Py::Bool({ let t3410 = Py::Bool(true); if py::is(&t3409, &t3410) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3415 = { let t3411 = v_r.clone(); Py::Bool({ let t3412 = Py::Bool(false); if py::is(&t3411, &t3412) { true } else { false } }) }; if t3415.truthy() { t3415 } else { { let t3413 = v_r.clone(); Py::Bool({ let t3414 = Py::None; if py::is(&t3413, &t3414) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3416 = v_r.clone(); Py::Bool({ let t3417 = v_r.clone(); if py::eq(&t3416, &t3417) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3397(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("area"))?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("base"))?, &py::getitem(&v_i.clone(), &Py::str("height"))?)?, &Py::Int(2i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3418(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f__cevians(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t3429 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3430 = (t3429).items()?;
if t3430.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3430[0].clone();
v_v = t3430[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3432(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3451: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_cevians(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3451 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3452 = v_r.clone(); Py::Bool({ let t3453 = Py::Bool(true); if py::is(&t3452, &t3453) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3458 = { let t3454 = v_r.clone(); Py::Bool({ let t3455 = Py::Bool(false); if py::is(&t3454, &t3455) { true } else { false } }) }; if t3458.truthy() { t3458 } else { { let t3456 = v_r.clone(); Py::Bool({ let t3457 = Py::None; if py::is(&t3456, &t3457) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3459 = v_r.clone(); Py::Bool({ let t3460 = v_r.clone(); if py::eq(&t3459, &t3460) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3461(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_circumcircle(a0)? };
    return Ok({ let mut out = vec![]; for t3483 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3484 = (t3483).items()?;
if t3484.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3484[0].clone();
v_v = t3484[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3486(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3488: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_circum(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3488 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3489 = v_r.clone(); Py::Bool({ let t3490 = Py::Bool(true); if py::is(&t3489, &t3490) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3495 = { let t3491 = v_r.clone(); Py::Bool({ let t3492 = Py::Bool(false); if py::is(&t3491, &t3492) { true } else { false } }) }; if t3495.truthy() { t3495 } else { { let t3493 = v_r.clone(); Py::Bool({ let t3494 = Py::None; if py::is(&t3493, &t3494) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3496 = v_r.clone(); Py::Bool({ let t3497 = v_r.clone(); if py::eq(&t3496, &t3497) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3498(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument a".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument b".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "c") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument c".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_classify(a0, a1, a2)? };
    return Ok({ let mut out = vec![]; for t3520 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3521 = (t3520).items()?;
if t3521.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3521[0].clone();
v_v = t3521[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
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
fn s_3523(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3534: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_classify(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3534 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3535 = v_r.clone(); Py::Bool({ let t3536 = Py::Bool(true); if py::is(&t3535, &t3536) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3541 = { let t3537 = v_r.clone(); Py::Bool({ let t3538 = Py::Bool(false); if py::is(&t3537, &t3538) { true } else { false } }) }; if t3541.truthy() { t3541 } else { { let t3539 = v_r.clone(); Py::Bool({ let t3540 = Py::None; if py::is(&t3539, &t3540) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3542 = v_r.clone(); Py::Bool({ let t3543 = v_r.clone(); if py::eq(&t3542, &t3543) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3544(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_coordinates(a0)? };
    return Ok({ let mut out = vec![]; for t3563 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3564 = (t3563).items()?;
if t3564.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3564[0].clone();
v_v = t3564[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3566(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3581: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_tri_coords(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3581 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3582 = v_r.clone(); Py::Bool({ let t3583 = Py::Bool(true); if py::is(&t3582, &t3583) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3588 = { let t3584 = v_r.clone(); Py::Bool({ let t3585 = Py::Bool(false); if py::is(&t3584, &t3585) { true } else { false } }) }; if t3588.truthy() { t3588 } else { { let t3586 = v_r.clone(); Py::Bool({ let t3587 = Py::None; if py::is(&t3586, &t3587) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3589 = v_r.clone(); Py::Bool({ let t3590 = v_r.clone(); if py::eq(&t3589, &t3590) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3591(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_incircle(a0)? };
    return Ok({ let mut out = vec![]; for t3614 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3615 = (t3614).items()?;
if t3615.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3615[0].clone();
v_v = t3615[1].clone();
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
fn s_3617(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3623: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_incircle(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3623 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3624 = v_r.clone(); Py::Bool({ let t3625 = Py::Bool(true); if py::is(&t3624, &t3625) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3630 = { let t3626 = v_r.clone(); Py::Bool({ let t3627 = Py::Bool(false); if py::is(&t3626, &t3627) { true } else { false } }) }; if t3630.truthy() { t3630 } else { { let t3628 = v_r.clone(); Py::Bool({ let t3629 = Py::None; if py::is(&t3628, &t3629) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3631 = v_r.clone(); Py::Bool({ let t3632 = v_r.clone(); if py::eq(&t3631, &t3632) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
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
fn s_3633(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_out: Py = Py::None;
    v_out = { let mut v: Vec<Py> = vec![]; v.extend((v_a.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; py::kw_extend(&mut d, &v_k.clone())?; let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "points") { Some(x) => x, None => { return Err(PyErr::Type("fn() missing argument points".into())) } } }; if v.len() > 1 { return Err(PyErr::Type("fn() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("fn() got an unexpected keyword argument".into())) } f_triangle_orthocenter(a0)? };
    return Ok({ let mut out = vec![]; for t3657 in py::iter(&(py::method(&v_out.clone(), "items", &[])?))? { let mut v_key: Py; let mut v_v: Py; 
let t3658 = (t3657).items()?;
if t3658.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_key = t3658[0].clone();
v_v = t3658[1].clone();
out.push((v_key.clone(), f__r12(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3660(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3664: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = f__verify_ortho(v_inputs.clone(), v_outputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3664 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t3665 = v_r.clone(); Py::Bool({ let t3666 = Py::Bool(true); if py::is(&t3665, &t3666) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t3671 = { let t3667 = v_r.clone(); Py::Bool({ let t3668 = Py::Bool(false); if py::is(&t3667, &t3668) { true } else { false } }) }; if t3671.truthy() { t3671 } else { { let t3669 = v_r.clone(); Py::Bool({ let t3670 = Py::None; if py::is(&t3669, &t3670) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_r = py::float(&v_r.clone())?;
    return Ok((if ({ let t3672 = v_r.clone(); Py::Bool({ let t3673 = v_r.clone(); if py::eq(&t3672, &t3673) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
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

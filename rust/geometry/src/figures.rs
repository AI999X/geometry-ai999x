//! Geometry's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model computes:
//! the points and polygons as read, the returned centres, circles, hulls and intersection points, and further runs
//! of the model's own capabilities (each lattice point tested by its point-in-polygon test, each pair of points
//! measured by its closest-pair capability, a vertex moved by its rotation). A figure the model cannot support for
//! these inputs is left out, never drawn wrong (docs/FIGURE_PARITY.md).

use ax_engine::figkit::{self as k, num, num_of, DPlot, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

#[path = "figures_shape.rs"]
mod shape;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // the answer over a grid of two inputs
        "annulus_area" => vec![field2(s, "outer", "inner", "area", (false, false), "Area of an annulus")],
        "circular_segment" => vec![field2(s, "radius", "angle", "area", (false, false), "Area of a circular segment")],
        "cone_measures" => vec![field2(s, "radius", "height", "volume", (false, false), "Volume of a cone")],
        "cylinder_measures" => vec![field2(s, "radius", "height", "volume", (false, false), "Volume of a cylinder")],
        "helix_measures" => vec![field2(s, "pitch", "radius", "curvature", (true, true), "Curvature of a helix")],
        "rectangle_measures" => vec![field2(s, "length", "width", "area", (false, false), "Area of a rectangle")],
        "regular_polygon_measures" => vec![field2(s, "side", "n", "area", (false, false), "Area of a regular polygon")],
        "rhombus_measures" => vec![field2(s, "d1", "d2", "area", (false, false), "Area of a rhombus")],
        "sector_measures" => vec![field2(s, "radius", "angle", "arc_length", (false, false), "Arc length of a sector")],
        "spherical_cap" => vec![field2(s, "radius", "height", "volume", (false, false), "Volume of a spherical cap")],
        "torus_measures" => vec![field2(s, "major", "minor", "volume", (false, false), "Volume of a torus")],
        "triangle_area_base_height" => vec![field2(s, "base", "height", "area", (false, false), "Area of a triangle")],
        // the points as read, with what the model returned drawn on them
        "barycentric_coordinates" => vec![points_fig(s)],
        "closest_pair" => vec![points_fig(s)],
        "convex_hull" => vec![points_fig(s), shape::hull_circle(s)],
        "convex_polygon_intersection" => vec![points_fig(s)],
        "delaunay_voronoi_counts" => vec![points_fig(s)],
        "farthest_pair" => vec![points_fig(s)],
        "min_area_rectangle" => vec![points_fig(s)],
        "min_enclosing_circle" => vec![points_fig(s)],
        "nearest_neighbor" => vec![points_fig(s)],
        "pick_lattice_points" => vec![points_fig(s)],
        "point_in_polygon" => vec![points_fig(s)],
        "polygon_area" => vec![points_fig(s)],
        "polygon_centroid" => vec![points_fig(s), shape::polygon_shape(s)],
        "polygon_interior_angles" => vec![points_fig(s)],
        "polygon_is_convex" => vec![points_fig(s)],
        "polyline_length" => vec![points_fig(s)],
        "segment_intersection" => vec![points_fig(s)],
        "spherical_triangle_area" => vec![points_fig(s)],
        "triangle_circumcircle" => vec![points_fig(s), shape::triangle_centres(s)],
        "triangle_coordinates" => vec![points_fig(s)],
        "triangle_incircle" => vec![points_fig(s)],
        "triangle_orthocenter" => vec![points_fig(s)],
        // points in space as a table of coordinates
        "hausdorff_distance" => vec![matrix_fig(s)],
        "plane_through_points" => vec![matrix_fig(s)],
        "solid_angle_triangle" => vec![matrix_fig(s)],
        "sphere_four_points" => vec![matrix_fig(s)],
        "tetrahedron_measures" => vec![matrix_fig(s)],
        "triangle_area_3d" => vec![matrix_fig(s)],
        // drawings of their own
        "rotate_point" => vec![shape::transforms(s)],
        "tangent_from_point" => vec![shape::tangents(s)],
        "regular_polygon_angles" => vec![shape::polygon_angles(s)],
        "conic_classify" => vec![shape::conic_map(s)],
        "skew_lines_distance" => vec![shape::skew_lines(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ shared helpers
pub(crate) fn tx(t: &str) -> Input {
    Input::Text(t.to_string())
}

/// A list input read as points of `dim` coordinates (None when its length is not a multiple of dim).
pub(crate) fn pts_n(s: &Solved, sym: &str, dim: usize) -> Option<Vec<Vec<f64>>> {
    let v = s.vec(sym)?;
    (!v.is_empty() && v.len() % dim == 0).then(|| v.chunks(dim).map(|c| c.to_vec()).collect())
}

/// A list input read as plane points.
pub(crate) fn pts2(s: &Solved, sym: &str) -> Option<Vec<(f64, f64)>> {
    Some(pts_n(s, sym, 2)?.into_iter().map(|c| (c[0], c[1])).collect())
}

/// Plane points written as the model reads them ("x1 y1, x2 y2, ...").
pub(crate) fn ptext(p: &[(f64, f64)]) -> String {
    p.iter().map(|(x, y)| format!("{x} {y}")).collect::<Vec<_>>().join(", ")
}

/// "(x, y)" to 4 digits.
pub(crate) fn xy(p: (f64, f64)) -> String {
    format!("({}, {})", num(k::snap(p.0, 1.0)), num(k::snap(p.1, 1.0)))
}

/// A data point moved by (dx, dy) pixels on the plot (right and down positive).
pub(crate) fn off(p: &DPlot, x: f64, y: f64, dx: f64, dy: f64) -> (f64, f64) {
    let (a, b) = (p.px(0.0, 0.0), p.px(1.0, 1.0));
    let (sx, sy) = (b.0 - a.0, b.1 - a.1);
    (x + if sx != 0.0 { dx / sx } else { 0.0 }, y + if sy != 0.0 { dy / sy } else { 0.0 })
}

/// A circle (as a closed curve of 97 points).
pub(crate) fn circle(c: (f64, f64), r: f64) -> Vec<(f64, f64)> {
    (0..=96).map(|i| {
        let t = i as f64 / 96.0 * std::f64::consts::TAU;
        (c.0 + r * t.cos(), c.1 + r * t.sin())
    }).collect()
}

/// Every point that should fit the plot, with a circle's extent.
pub(crate) fn extent(pts: &[(f64, f64)], circles: &[((f64, f64), f64)]) -> (Vec<f64>, Vec<f64>) {
    let mut xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let mut ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    for (c, r) in circles {
        xs.extend([c.0 - r, c.0 + r]);
        ys.extend([c.1 - r, c.1 + r]);
    }
    (xs, ys)
}

/// Each point numbered (1, 2, ...) beside it, on its side away from the middle of the points (inside a polygon
/// stay the angles and the marks).
pub(crate) fn numbered(p: &mut DPlot, pts: &[(f64, f64)], k_: usize) {
    let n = pts.len().max(1) as f64;
    let mid = p.px(pts.iter().map(|v| v.0).sum::<f64>() / n, pts.iter().map(|v| v.1).sum::<f64>() / n);
    for (i, q) in pts.iter().enumerate() {
        p.dot(q.0, q.1, k_, 4.5);
        let a = p.px(q.0, q.1);
        let (dx, dy) = (a.0 - mid.0, a.1 - mid.1);
        let d = (dx * dx + dy * dy).sqrt();
        let (ux, uy) = if d > 1.0 { (dx / d, dy / d) } else { (0.7, -0.7) };
        let (lx, ly) = off(p, q.0, q.1, ux * 13.0, uy * 13.0 + 5.0);
        p.label(lx, ly, "middle", "axp-bt", &(i + 1).to_string());
    }
}

/// How far inside a vertex its angle label sits (pixels): further for a sharp corner, whose sides crowd it.
pub(crate) fn reach(angle_deg: f64) -> f64 {
    if angle_deg >= 80.0 { 26.0 } else { 26.0 + (80.0 - angle_deg.max(5.0)) * 0.55 }
}

fn closed(pts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut v = pts.to_vec();
    if let Some(f) = pts.first() {
        v.push(*f);
    }
    v
}

// ------------------------------------------------------------------------------------------------ the field
/// The answer shaded over a grid of two number inputs (13 x 9 runs of the model; a whole-number input steps by 1).
fn field2(s: &Solved, xsym: &str, ysym: &str, out: &str, log: (bool, bool), what: &str) -> Option<Figure> {
    // a whole-number input is read as a list of one (Solved::vec), a number in SI
    let get = |sym: &str| s.get(sym).or_else(|| s.vec(sym).and_then(|v| (v.len() == 1).then(|| v[0])));
    let (x0, y0, v0) = (get(xsym)?, get(ysym)?, s.out(out)?);
    let int = |sym: &str| sym == "n";
    let axis = |sym: &str, v: f64, lg: bool, n: usize| -> Option<Vec<f64>> {
        if int(sym) {
            let lo = (v - 4.0).max(3.0);
            return Some((0..n).map(|i| lo + i as f64).collect());
        }
        if lg {
            return (v > 0.0).then(|| k::span(v / 10.0, v * 10.0, n, true));
        }
        let (lo, hi) = if sym == "angle" {
            ((v * 0.5).max(v.min(1.0) * 0.5), (v * 1.5).min(359.0))
        } else if v != 0.0 {
            (v - v.abs() * 0.5, v + v.abs() * 0.5)
        } else {
            (-1.0, 1.0)
        };
        Some(k::span(lo, hi, n, false))
    };
    let xs = axis(xsym, x0, log.0, 13)?;
    let ys = axis(ysym, y0, log.1, 9)?;
    let input = |sym: &str, v: f64| if int(sym) { Input::Int(v.round() as i64) } else { Input::Float(v) };
    let vals: Vec<Vec<Option<f64>>> = xs.iter().map(|x| ys.iter().map(|y| {
        s.with_input(&[(xsym, input(xsym, *x)), (ysym, input(ysym, *y))]).and_then(|a| num_of(&a, out))
    }).collect()).collect();
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    if all.len() < 20 {
        return None;
    }
    let (lo, hi) = all.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    let log_shade = lo > 0.0 && hi / lo > 30.0;
    let shown: Vec<Vec<Option<f64>>> = if log_shade {
        vals.iter().map(|c| c.iter().map(|v| v.map(|x| x.log10())).collect()).collect()
    } else {
        vals.clone()
    };
    let head = if log_shade { format!("{out} = {} (shade: log10 of {out})", num(v0)) } else { format!("{out} = {}", num(v0)) };
    let body = k::field(&xs, &ys, &shown, (x0, y0), xsym, ysym, &head, log);
    if body.is_empty() {
        return None;
    }
    let refused = vals.iter().flatten().filter(|v| v.is_none()).count();
    Some(s.chart("field", &format!("{what} over {xsym} and {ysym}"),
                 &format!("{out} over {xsym} from {} to {} and {ysym} from {} to {}{}: each of the {} × {} cells is one run \
                           of the model, shaded by {} (darker is larger; values from {} to {}){}. Marked: {xsym} = {}, \
                           {ysym} = {}, {out} = {}.", num(xs[0]), num(xs[xs.len() - 1]), num(ys[0]), num(ys[ys.len() - 1]),
                          if log.0 || log.1 { " (logarithmic axes)" } else { "" }, xs.len(), ys.len(),
                          if log_shade { "the logarithm of its value" } else { "its value" }, num(lo), num(hi),
                          if refused > 0 { format!("; {refused} blank cells are inputs the model refuses") } else { String::new() },
                          num(x0), num(y0), num(v0)), &body))
}

// ------------------------------------------------------------------------------------------------ points in space
/// The points of a request in space (or two plane point sets) as a table of coordinates, shaded by value.
fn matrix_fig(s: &Solved) -> Option<Figure> {
    let (m, rows, cols, what): (Vec<Vec<f64>>, Vec<String>, Vec<String>, String) = if s.key == "hausdorff_distance" {
        let (a, b) = (pts_n(s, "a", 2)?, pts_n(s, "b", 2)?);
        let n = a.len().max(b.len());
        if n > 24 {
            return None;
        }
        let mut m = vec![];
        for i in 0..n {
            let mut r = a.get(i).cloned().unwrap_or(vec![f64::NAN, f64::NAN]);
            r.push(f64::NAN);
            r.extend(b.get(i).cloned().unwrap_or(vec![f64::NAN, f64::NAN]));
            m.push(r);
        }
        let cols = ["a: x", "a: y", "", "b: x", "b: y"].iter().map(|c| c.to_string()).collect();
        (m, (1..=n).map(|i| format!("point {i}")).collect(), cols, "the two point sets a (left) and b (right)".into())
    } else {
        let m = pts_n(s, "points", 3)?;
        if m.len() > 24 {
            return None;
        }
        let rows = (1..=m.len()).map(|i| format!("P{i}")).collect();
        (m, rows, vec!["x".into(), "y".into(), "z".into()], "the points (one row each: x, y, z)".into())
    };
    let outs: Vec<String> = s.a.values.iter().filter_map(|(n, v, _)| v.as_f64().map(|x| format!("{n} = {}", num(x)))).collect();
    if outs.is_empty() {
        return None;
    }
    let head = match s.key {
        "plane_through_points" => {
            let (a, b, c, d) = (s.out("a")?, s.out("b")?, s.out("c")?, s.out("d")?);
            format!("{}x + {}y + {}z + {} = 0", num(a), num(b), num(c), num(d)).replace("+ -", "- ")
        }
        "sphere_four_points" => format!("centre ({}, {}, {}), radius {}", num(s.out("cx")?), num(s.out("cy")?),
                                        num(s.out("cz")?), num(s.out("radius")?)),
        _ => String::new(),
    };
    // a heading too long for the canvas gives way to the first returned value
    let head = if head.is_empty() || head.chars().count() > 46 {
        k::eq(&s.a.values[0].0, s.out(&s.a.values[0].0)?, "")
    } else {
        head
    };
    let body = k::heat(&m, &rows, &cols, &head);
    let title = match s.key {
        "hausdorff_distance" => "Hausdorff distance: the two point sets",
        "plane_through_points" => "Plane through three points: the points",
        "solid_angle_triangle" => "Solid angle of a triangle: its vertices",
        "sphere_four_points" => "Sphere through four points: the points",
        "tetrahedron_measures" => "Tetrahedron: its four vertices",
        _ => "Triangle in space: its vertices",
    };
    Some(s.chart("matrix", title, &format!("The coordinates of {what} as read, each cell shaded by its size (negative \
                                            values in the second colour). Returned by the model: {}.", outs.join(", ")), &body))
}

// ------------------------------------------------------------------------------------------------ points in the plane
/// The model's own distance between two points (its closest-pair capability on the pair alone).
fn pair_dist(s: &Solved, key: &str, a: (f64, f64), b: (f64, f64)) -> Option<f64> {
    let r = s.solve_other(key, &[("points", tx(&ptext(&[a, b])))])?;
    num_of(&r, "distance")
}

/// The pair of points at the returned distance, found by measuring each pair with the model (up to 12 points).
fn pair_at(s: &Solved, key: &str, pts: &[(f64, f64)], d: f64) -> Option<(usize, usize)> {
    if pts.len() > 12 {
        return None;
    }
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            if let Some(e) = pair_dist(s, key, pts[i], pts[j]) {
                if (e - d).abs() <= 1e-9 * d.abs().max(1.0) {
                    return Some((i, j));
                }
            }
        }
    }
    None
}

fn points_fig(s: &Solved) -> Option<Figure> {
    let key = s.key;
    let sym = if key == "convex_polygon_intersection" { "p" } else { "points" };
    let pts = pts2(s, sym)?;
    // (lat, lon) vertices are drawn as a map: longitude across, latitude up (they were swapped; geometry review)
    let pts: Vec<(f64, f64)> = if key == "spherical_triangle_area" { pts.iter().map(|p| (p.1, p.0)).collect() } else { pts };
    if pts.len() < 2 || pts.len() > 60 {
        return None;
    }
    let mut extra: Vec<(f64, f64)> = vec![];
    let mut circles: Vec<((f64, f64), f64)> = vec![];
    let q = (s.inp("px"), s.inp("py"));
    if let (Some(x), Some(y)) = q {
        extra.push((x, y));
    }
    let q2 = if key == "convex_polygon_intersection" { pts2(s, "q") } else { None };
    if let Some(v) = &q2 {
        extra.extend(v.iter().copied());
    }
    match key {
        "min_enclosing_circle" => circles.push(((s.out("cx")?, s.out("cy")?), s.out("radius")?)),
        "triangle_circumcircle" => circles.push(((s.out("cx")?, s.out("cy")?), s.out("radius")?)),
        "triangle_incircle" => circles.push(((s.out("ix")?, s.out("iy")?), s.out("radius")?)),
        "triangle_orthocenter" => extra.push((s.out("hx")?, s.out("hy")?)),
        "triangle_coordinates" => extra.push((s.out("gx")?, s.out("gy")?)),
        "polygon_centroid" => extra.push((s.out("cx")?, s.out("cy")?)),
        "segment_intersection" => {
            if let (Some(x), Some(y)) = (s.out("x"), s.out("y")) {
                extra.push((x, y));
            }
        }
        _ => {}
    }
    let (mut xs, mut ys) = extent(&pts, &circles);
    xs.extend(extra.iter().map(|p| p.0));
    ys.extend(extra.iter().map(|p| p.1));
    let (xlab, ylab) = if key == "spherical_triangle_area" { ("Longitude (°)", "Latitude (°)") } else { ("x", "y") };
    // the heading: the first number returned, else the first verdict (a list such as a hull is drawn, not written)
    let main = s.a.values.iter().find(|v| v.1.is_number()).or_else(|| s.a.values.iter().find(|v| v.1.is_string()))
        .map(|v| v.0.clone()).unwrap_or_default();
    let pair = |a: &str, b: &str| -> Option<String> { Some(xy((s.out(a)?, s.out(b)?))) };
    let special = match key {
        "polygon_centroid" => pair("cx", "cy").map(|t| format!("centroid {t}")),
        "triangle_orthocenter" => pair("hx", "hy").map(|t| format!("orthocentre {t}")),
        "segment_intersection" => Some(match pair("x", "y") {
            Some(t) => format!("{}: {t}", s.out_text("status").unwrap_or_default()),
            None => format!("status: {}", s.out_text("status").unwrap_or_default()),
        }),
        "nearest_neighbor" => Some(format!("nearest: point {}, distance {}", num(s.out("index")?), num(s.out("distance")?))),
        "barycentric_coordinates" => Some(format!("l1, l2, l3 = {}, {}, {}", num(s.out("l1")?), num(s.out("l2")?),
                                                  num(s.out("l3")?))),
        "triangle_coordinates" => Some(format!("area = {}, perimeter = {}", num(s.out("area")?), num(s.out("perimeter")?))),
        "polygon_area" => Some(format!("area = {}, perimeter = {}", num(s.out("area")?), num(s.out("perimeter")?))),
        "min_area_rectangle" => Some(format!("area = {}: {} by {}", num(s.out("area")?), num(s.out("width")?),
                                             num(s.out("length")?))),
        _ => None,
    };
    let head = if let Some(h) = special {
        h
    } else if let Some(c) = circles.first() {
        format!("centre {}, radius {}", xy(c.0), num(c.1))
    } else {
        match s.out(&main) {
            Some(v) => format!("{main} = {}", num(v)),
            None => format!("{main} = {}", s.out_text(&main).unwrap_or_default()),
        }
    };
    let mut p = DPlot::equal(&xs, &ys, xlab, ylab, &head);
    let polygon = !matches!(key, "closest_pair" | "farthest_pair" | "delaunay_voronoi_counts" | "min_enclosing_circle"
                                 | "nearest_neighbor" | "polyline_length" | "segment_intersection" | "convex_hull");
    if polygon {
        p.fill(&pts, 1);
        p.outline(&pts, 3);
    }
    let mut notes: Vec<String> = vec![];
    match key {
        "barycentric_coordinates" => {
            let (x, y) = (q.0?, q.1?);
            for v in pts.iter().take(3) {
                p.seg((x, y), *v, 3, true);
            }
            p.mark(x, y, &format!("P ({}, {}, {})", num(s.out("l1")?), num(s.out("l2")?), num(s.out("l3")?)));
            notes.push(format!("the point P = {} has barycentric coordinates l1 = {}, l2 = {}, l3 = {} (weights of the \
                                vertices 1, 2, 3)", xy((x, y)), num(s.out("l1")?), num(s.out("l2")?), num(s.out("l3")?)));
        }
        "closest_pair" | "farthest_pair" => {
            let d = s.out("distance")?;
            let (i, j) = pair_at(s, key, &pts, d)?;
            p.seg(pts[i], pts[j], 0, false);
            let m = ((pts[i].0 + pts[j].0) / 2.0, (pts[i].1 + pts[j].1) / 2.0);
            let (lx, ly) = off(&p, m.0, m.1, 8.0, 16.0);
            p.label(lx, ly, "start", "axp-vt", &format!("d = {}", num(d)));
            notes.push(format!("the {} pair is points {} and {}, at distance {} (each pair measured by the model)",
                               if key == "closest_pair" { "closest" } else { "farthest" }, i + 1, j + 1, num(d)));
        }
        "convex_hull" => {
            let hull = hull_of(s)?;
            p.fill(&hull, 1);
            p.outline(&hull, 0);
            for v in &hull {
                p.dot(v.0, v.1, 0, 6.0);
            }
            notes.push(format!("the returned hull has {} vertices {}, area {} and perimeter {}", hull.len(),
                               hull.iter().map(|v| xy(*v)).collect::<Vec<_>>().join(", "), num(s.out("area")?),
                               num(s.out("perimeter")?)));
        }
        "convex_polygon_intersection" => {
            let qv = q2?;
            p.fill(&qv, 2);
            p.outline(&qv, 2);
            p.outline(&pts, 0);
            p.key("p", 0);
            p.key("q", 2);
            notes.push(format!("the polygons p and q overlap where both shades meet; the model returns the area of their \
                                intersection, {}", num(s.out("area")?)));
        }
        "delaunay_voronoi_counts" => {
            if let Some(h) = s.solve_other("convex_hull", &[("points", tx(&ptext(&pts)))]) {
                if let Some(hull) = hull_list(&h) {
                    p.outline(&hull, 3);
                }
            }
            notes.push(format!("the Delaunay triangulation has {} triangles and {} edges, {} points on the hull (grey \
                                outline: the hull from the model); the Voronoi diagram has {} vertices and {} unbounded cells",
                               num(s.out("triangles")?), num(s.out("edges")?), num(s.out("hull_vertices")?),
                               num(s.out("voronoi_vertices")?), num(s.out("unbounded_cells")?)));
        }
        "min_area_rectangle" => {
            if let Some(h) = s.solve_other("convex_hull", &[("points", tx(&ptext(&pts)))]) {
                if let Some(hull) = hull_list(&h) {
                    p.outline(&hull, 0);
                }
            }
            notes.push(format!("the smallest rectangle enclosing them has area {}, width {} and length {} (outlined: the \
                                convex hull from the model)",
                               num(s.out("area")?), num(s.out("width")?), num(s.out("length")?)));
        }
        "min_enclosing_circle" | "triangle_circumcircle" | "triangle_incircle" => {
            let (c, r) = circles[0];
            p.curve(&circle(c, r), 0);
            p.mark(c.0, c.1, "");
            notes.push(format!("the returned circle has centre {} and radius {}", xy(c), num(r)));
        }
        "nearest_neighbor" => {
            let (x, y) = (q.0?, q.1?);
            let nb = (s.out("nx")?, s.out("ny")?);
            p.seg((x, y), nb, 0, true);
            p.dot(x, y, 2, 6.0);
            let (lx, ly) = off(&p, x, y, 9.0, 20.0);
            p.label(lx, ly, "start", "axp-bt", "query");
            p.mark(nb.0, nb.1, &format!("nearest: point {}", num(s.out("index")?)));
            notes.push(format!("the nearest point to {} is point {} at {}, distance {}", xy((x, y)), num(s.out("index")?),
                               xy(nb), num(s.out("distance")?)));
        }
        "pick_lattice_points" => {
            let (bi, ii) = lattice(s, &mut p, &pts);
            notes.push(format!("area {}, {} lattice points on the boundary and {} inside (Pick: A = I + B/2 - 1); \
                                {}", num(s.out("area")?), num(s.out("boundary_points")?), num(s.out("interior_points")?),
                               if bi + ii > 0 { format!("dots: each lattice point tested by the model's point-in-polygon \
                                                         test ({bi} boundary, {ii} inside)") } else {
                                   "the lattice is too large to draw point by point".into() }));
        }
        "point_in_polygon" => {
            let (x, y) = (q.0?, q.1?);
            let loc = s.out_text("location")?;
            p.mark(x, y, &format!("P: {loc}"));
            notes.push(format!("the point {} is {loc}", xy((x, y))));
        }
        "polygon_area" => {
            for w in closed(&pts).windows(2) {
                let m = ((w[0].0 * 0.4 + w[1].0 * 0.6), (w[0].1 * 0.4 + w[1].1 * 0.6));
                let a = ((w[0].0 * 0.55 + w[1].0 * 0.45), (w[0].1 * 0.55 + w[1].1 * 0.45));
                p.arrow(a, m, 0);
            }
            notes.push(format!("area {}, perimeter {}, the vertices run {} (arrows: the order as given)",
                               num(s.out("area")?), num(s.out("perimeter")?), s.out_text("orientation")?));
        }
        "polygon_centroid" => {
            let c = (s.out("cx")?, s.out("cy")?);
            p.mark(c.0, c.1, &format!("centroid {}", xy(c)));
            notes.push(format!("the centroid is {}", xy(c)));
        }
        "polygon_interior_angles" | "polygon_is_convex" => {
            let ang = if key == "polygon_interior_angles" { s.out_list("angles")? } else {
                let a = s.solve_other("polygon_interior_angles", &[("points", tx(&ptext(&pts)))])?;
                k::list_of(&a, "angles")?
            };
            if ang.len() != pts.len() {
                return None;
            }
            let (cx, cy) = (pts.iter().map(|v| v.0).sum::<f64>() / pts.len() as f64,
                            pts.iter().map(|v| v.1).sum::<f64>() / pts.len() as f64);
            for (v, a) in pts.iter().zip(&ang) {
                let (pa, pc) = (p.px(v.0, v.1), p.px(cx, cy));
                let (dx, dy) = (pc.0 - pa.0, pc.1 - pa.1);
                let d = (dx * dx + dy * dy).sqrt().max(1e-9);
                let (lx, ly) = off(&p, v.0, v.1, dx / d * reach(*a), dy / d * reach(*a) + 5.0);
                p.label(lx, ly, "middle", if *a > 180.0 { "axp-bt" } else { "axp-vt" }, &format!("{}°", num(*a)));
                if *a > 180.0 {
                    p.dot(v.0, v.1, 0, 7.0);
                }
            }
            let reflex = ang.iter().filter(|a| **a > 180.0).count();
            if key == "polygon_is_convex" {
                notes.push(format!("convex: {}; interior angles from the model {}{}", s.out_text("convex")?,
                                   ang.iter().map(|a| format!("{}°", num(*a))).collect::<Vec<_>>().join(", "),
                                   if reflex > 0 { format!(", {reflex} reflex (over 180°, marked)") } else { String::new() }));
            } else {
                notes.push(format!("interior angles {} (sum {}°, {} reflex)",
                                   ang.iter().map(|a| format!("{}°", num(*a))).collect::<Vec<_>>().join(", "),
                                   num(s.out("angle_sum")?), num(s.out("reflex_vertices")?)));
            }
        }
        "polyline_length" => {
            p.curve(&pts, 0);
            notes.push(format!("length {} over {} segments", num(s.out("length")?), num(s.out("segments")?)));
        }
        "segment_intersection" => {
            if pts.len() != 4 {
                return None;
            }
            p.seg(pts[0], pts[1], 0, false);
            p.seg(pts[2], pts[3], 2, false);
            p.key("AB", 0);
            p.key("CD", 2);
            let st = s.out_text("status")?;
            if let (Some(x), Some(y)) = (s.out("x"), s.out("y")) {
                p.mark(x, y, &xy((x, y)));
                notes.push(format!("the segments AB (points 1, 2) and CD (points 3, 4) meet at {} ({st})", xy((x, y))));
            } else {
                notes.push(format!("the segments AB (points 1, 2) and CD (points 3, 4): {st}"));
            }
        }
        "spherical_triangle_area" => {
            notes.push(format!("the vertices as latitude and longitude in degrees (a flat map, not to scale on the \
                                sphere): spherical excess {}°, area {}", num(s.out("excess")?), num(s.out("area")?)));
        }
        "triangle_coordinates" => {
            let g = (s.out("gx")?, s.out("gy")?);
            p.mark(g.0, g.1, &format!("centroid {}", xy(g)));
            let names = ["A", "B", "C"];
            let ang = [s.out("angle_A")?, s.out("angle_B")?, s.out("angle_C")?];
            for (i, v) in pts.iter().take(3).enumerate() {
                let (pa, pc) = (p.px(v.0, v.1), p.px(g.0, g.1));
                let (dx, dy) = (pc.0 - pa.0, pc.1 - pa.1);
                let d = (dx * dx + dy * dy).sqrt().max(1e-9);
                let (lx, ly) = off(&p, v.0, v.1, dx / d * 34.0, dy / d * 30.0 + 5.0);
                p.label(lx, ly, "middle", "axp-vt", &format!("{} {}°", names[i], num(ang[i])));
            }
            notes.push(format!("area {}, perimeter {}, centroid {}, angles A = {}°, B = {}°, C = {}°",
                               num(s.out("area")?), num(s.out("perimeter")?), xy(g), num(ang[0]), num(ang[1]), num(ang[2])));
        }
        "triangle_orthocenter" => {
            let h = (s.out("hx")?, s.out("hy")?);
            for v in pts.iter().take(3) {
                p.seg(*v, h, 3, true);
            }
            p.mark(h.0, h.1, &format!("H {}", xy(h)));
            notes.push(format!("the orthocentre H = {}, where the altitudes meet (dashed: each vertex joined to H, along \
                                its altitude)", xy(h)));
        }
        _ => {}
    }
    // (a lattice polygon's vertices are boundary lattice points: drawn in the boundary colour)
    numbered(&mut p, &pts, if polygon && key != "pick_lattice_points" { 3 } else { 0 });
    let title = match key {
        "barycentric_coordinates" => "Barycentric coordinates of a point in a triangle",
        "closest_pair" => "The closest pair of points",
        "convex_hull" => "Convex hull of the points",
        "convex_polygon_intersection" => "Two convex polygons and their overlap",
        "delaunay_voronoi_counts" => "Points of a Delaunay triangulation",
        "farthest_pair" => "The farthest pair of points",
        "min_area_rectangle" => "Points and their smallest enclosing rectangle",
        "min_enclosing_circle" => "The smallest circle enclosing the points",
        "nearest_neighbor" => "Nearest neighbour of a query point",
        "pick_lattice_points" => "Lattice points of a polygon (Pick's theorem)",
        "point_in_polygon" => "Point in a polygon",
        "polygon_area" => "A polygon from its vertices",
        "polygon_centroid" => "Centroid of a polygon",
        "polygon_interior_angles" => "Interior angles of a polygon",
        "polygon_is_convex" => "Is the polygon convex?",
        "polyline_length" => "A path through points",
        "segment_intersection" => "Intersection of two segments",
        "spherical_triangle_area" => "Vertices of a spherical triangle",
        "triangle_circumcircle" => "Circumcircle of a triangle",
        "triangle_coordinates" => "A triangle from its vertices",
        "triangle_incircle" => "Incircle of a triangle",
        _ => "Orthocentre of a triangle",
    };
    Some(s.chart("points", title, &format!("The {} points as given, numbered in order{}, drawn to scale: {}.", pts.len(),
                                           if polygon { " and joined as a polygon" } else { "" }, notes.join("; ")),
                 &p.body()))
}

/// The hull returned (a list of (x, y) pairs).
pub(crate) fn hull_list(a: &ax_engine::Answer) -> Option<Vec<(f64, f64)>> {
    let v = a.values.iter().find(|x| x.0 == "hull")?.1.as_array()?;
    let out: Vec<(f64, f64)> = v.iter().filter_map(|q| {
        let q = q.as_array()?;
        Some((q.first()?.as_f64()?, q.get(1)?.as_f64()?))
    }).collect();
    (out.len() == v.len() && out.len() >= 2).then_some(out)
}

pub(crate) fn hull_of(s: &Solved) -> Option<Vec<(f64, f64)>> {
    hull_list(s.a)
}

/// The lattice points of the polygon's box, each tested by the model (boundary in the accent colour, inside green);
/// returns how many of each were drawn (none when the box holds more than 150 points).
fn lattice(s: &Solved, p: &mut DPlot, pts: &[(f64, f64)]) -> (usize, usize) {
    let (x0, x1) = pts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), v| (a.min(v.0), b.max(v.0)));
    let (y0, y1) = pts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), v| (a.min(v.1), b.max(v.1)));
    let (nx, ny) = ((x1.floor() - x0.ceil()) as i64 + 1, (y1.floor() - y0.ceil()) as i64 + 1);
    if nx < 1 || ny < 1 || nx * ny > 150 || nx * ny as i64 > s.left() as i64 - 10 {
        return (0, 0);
    }
    let text = ptext(pts);
    let (mut b, mut i) = (0, 0);
    for gx in x0.ceil() as i64..=x1.floor() as i64 {
        for gy in y0.ceil() as i64..=y1.floor() as i64 {
            let Some(a) = s.solve_other("point_in_polygon", &[("px", Input::Float(gx as f64)), ("py", Input::Float(gy as f64)),
                                                              ("points", tx(&text))]) else { continue };
            let loc = match a.values.iter().find(|v| v.0 == "location").and_then(|v| v.1.as_str()) {
                Some(l) => l.to_string(),
                None => continue,
            };
            if loc.contains("bound") || loc.contains("edge") || loc.contains("vertex") || loc == "on" {
                p.dot(gx as f64, gy as f64, 0, 4.0);
                b += 1;
            } else if loc.contains("inside") || loc.contains("interior") {
                p.dot(gx as f64, gy as f64, 1, 4.0);
                i += 1;
            } else {
                p.dot(gx as f64, gy as f64, 3, 1.8);
            }
        }
    }
    p.key("boundary", 0);
    p.key("inside", 1);
    (b, i)
}

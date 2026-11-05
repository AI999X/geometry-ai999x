//! Drawings of their own: a hull with its enclosing circle, a polygon's angles and centroid, a triangle's centres,
//! images of a triangle under the model's transformations, tangents to a circle, regular polygons' angles, a map of
//! conic types and two lines in space. Every point, angle and verdict is the model's.

use super::*;

/// A centre and radius from an answer.
fn circ(a: &ax_engine::Answer, x: &str, y: &str, r: &str) -> Option<((f64, f64), f64)> {
    Some(((num_of(a, x)?, num_of(a, y)?), num_of(a, r)?))
}

/// The convex hull and the smallest enclosing circle of the same points (the circle a second run of the model).
pub fn hull_circle(s: &Solved) -> Option<Figure> {
    let pts = pts2(s, "points")?;
    let hull = hull_of(s)?;
    let c = s.solve_other("min_enclosing_circle", &[("points", tx(&ptext(&pts)))])?;
    let (cc, r) = circ(&c, "cx", "cy", "radius")?;
    let (xs, ys) = extent(&pts, &[(cc, r)]);
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("area = {}, radius = {}", num(s.out("area")?), num(r)));
    p.fill(&hull, 1);
    p.outline(&hull, 1);
    p.curve(&circle(cc, r), 0);
    for q in &pts {
        p.dot(q.0, q.1, 3, 4.0);
    }
    for v in &hull {
        p.dot(v.0, v.1, 1, 5.5);
    }
    p.mark(cc.0, cc.1, &format!("centre {}", xy(cc)));
    p.key("hull", 1);
    p.key("enclosing circle", 0);
    Some(s.chart("shape", "Convex hull and smallest enclosing circle",
                 &format!("The {} points, their convex hull as returned ({} vertices, area {}, perimeter {}) and the \
                           smallest circle enclosing them, from a second run of the model: centre {}, radius {}.",
                          pts.len(), hull.len(), num(s.out("area")?), num(s.out("perimeter")?), xy(cc), num(r)), &p.body()))
}

/// A polygon with the interior angle at each vertex, its area and its centroid (three capabilities of the model).
pub fn polygon_shape(s: &Solved) -> Option<Figure> {
    let pts = pts2(s, "points")?;
    let c = (s.out("cx")?, s.out("cy")?);
    let text = ptext(&pts);
    let ang = k::list_of(&s.solve_other("polygon_interior_angles", &[("points", tx(&text))])?, "angles")?;
    let ar = s.solve_other("polygon_area", &[("points", tx(&text))])?;
    let (area, per) = (num_of(&ar, "area")?, num_of(&ar, "perimeter")?);
    if ang.len() != pts.len() {
        return None;
    }
    let (mut xs, mut ys) = extent(&pts, &[]);
    xs.push(c.0);
    ys.push(c.1);
    let sum: f64 = ang.iter().sum();
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("area = {}, perimeter = {}", num(area), num(per)));
    p.fill(&pts, 2);
    p.outline(&pts, 2);
    for (v, a) in pts.iter().zip(&ang) {
        let (pa, pc) = (p.px(v.0, v.1), p.px(c.0, c.1));
        let (dx, dy) = (pc.0 - pa.0, pc.1 - pa.1);
        let d = (dx * dx + dy * dy).sqrt().max(1e-9);
        let (lx, ly) = off(&p, v.0, v.1, dx / d * reach(*a), dy / d * reach(*a) + 5.0);
        p.label(lx, ly, "middle", if *a > 180.0 { "axp-bt" } else { "axp-vt" }, &format!("{}°", num(*a)));
        p.dot(v.0, v.1, if *a > 180.0 { 0 } else { 3 }, 4.5);
    }
    p.mark(c.0, c.1, &format!("centroid {}", xy(c)));
    let reflex = ang.iter().filter(|a| **a > 180.0).count();
    Some(s.chart("shape", "A polygon's angles, area and centroid",
                 &format!("The polygon {} with the interior angle at each vertex (sum {}°, {} reflex), area {} and \
                           perimeter {}; marked: the centroid {}. All computed by the model; the centroid of a \
                           non-convex shape need not lie inside it.", text, num(sum), reflex, num(area), num(per), xy(c)),
                 &p.body()))
}

/// The triangle's circumcircle, incircle, orthocentre and centroid (four capabilities of the model), and the Euler
/// line through the circumcentre and the orthocentre.
pub fn triangle_centres(s: &Solved) -> Option<Figure> {
    let pts = pts2(s, "points")?;
    if pts.len() != 3 {
        return None;
    }
    let text = ptext(&pts);
    let (o, ro) = ((s.out("cx")?, s.out("cy")?), s.out("radius")?);
    let (i, ri) = circ(&s.solve_other("triangle_incircle", &[("points", tx(&text))])?, "ix", "iy", "radius")?;
    let ha = s.solve_other("triangle_orthocenter", &[("points", tx(&text))])?;
    let h = (num_of(&ha, "hx")?, num_of(&ha, "hy")?);
    let ga = s.solve_other("triangle_coordinates", &[("points", tx(&text))])?;
    let g = (num_of(&ga, "gx")?, num_of(&ga, "gy")?);
    let (mut xs, mut ys) = extent(&pts, &[(o, ro)]);
    xs.extend([h.0, g.0]);
    ys.extend([h.1, g.1]);
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("R = {}, r = {}", num(ro), num(ri)));
    p.fill(&pts, 3);
    p.outline(&pts, 3);
    p.curve(&circle(o, ro), 0);
    p.curve(&circle(i, ri), 1);
    if (h.0 - o.0).abs() + (h.1 - o.1).abs() > 1e-9 * ro {
        let d = (h.0 - o.0, h.1 - o.1);
        p.seg((o.0 - d.0 * 3.0, o.1 - d.1 * 3.0), (h.0 + d.0 * 3.0, h.1 + d.1 * 3.0), 2, true);
    }
    // each centre's letter on its own side (they crowd together in a near-right or near-equilateral triangle)
    for (q, nm, kk, dx, dy, anchor) in [(o, "O", 0usize, 8.0, -7.0, "start"), (i, "I", 1, -8.0, -7.0, "end"),
                                         (h, "H", 2, -8.0, 17.0, "end"), (g, "G", 3, 8.0, 17.0, "start")] {
        p.dot(q.0, q.1, kk, 5.0);
        let (lx, ly) = off(&p, q.0, q.1, dx, dy);
        p.label(lx, ly, anchor, "axp-bt", nm);
    }
    p.key("circumcircle", 0);
    p.key("incircle", 1);
    p.key("Euler line", 2);
    Some(s.chart("shape", "A triangle's circles and centres",
                 &format!("The triangle {text}: circumcircle (centre O = {}, radius {}), incircle (centre I = {}, radius \
                           {}), orthocentre H = {} and centroid G = {}, each computed by the model; O, G and H lie on the \
                           Euler line (dashed).", xy(o), num(ro), xy(i), num(ri), xy(h), xy(g)), &p.body()))
}

/// A triangle with the visitor's point as a vertex, and its images: rotated by the visitor's angle about the
/// visitor's centre, reflected in the line through that centre with slope -1, and dilated about it by -0.5 (every
/// image vertex one run of the model).
pub fn transforms(s: &Solved) -> Option<Figure> {
    let (px, py, ang) = (s.inp("px")?, s.inp("py")?, s.inp("angle")?);
    let (cx, cy) = (s.inp("cx").unwrap_or(0.0), s.inp("cy").unwrap_or(0.0));
    let d = (((px - cx).powi(2) + (py - cy).powi(2)).sqrt() / 2.0).max(1.0);
    let tri = [(px, py), (px + d, py), (px + 0.4 * d, py + 0.8 * d)];
    let rot: Vec<(f64, f64)> = tri.iter().map(|v| {
        let a = s.with(&[("px", v.0), ("py", v.1)])?;
        Some((num_of(&a, "x")?, num_of(&a, "y")?))
    }).collect::<Option<_>>()?;
    let (la, lb, lc) = (1.0, 1.0, -(cx + cy));
    let refl: Vec<(f64, f64)> = tri.iter().map(|v| {
        let a = s.solve_other("reflect_point_line", &[("px", Input::Float(v.0)), ("py", Input::Float(v.1)), ("a", Input::Float(la)),
                                                      ("b", Input::Float(lb)), ("c", Input::Float(lc))])?;
        Some((num_of(&a, "x")?, num_of(&a, "y")?))
    }).collect::<Option<_>>()?;
    let dil: Vec<(f64, f64)> = tri.iter().map(|v| {
        let a = s.solve_other("dilate_point", &[("px", Input::Float(v.0)), ("py", Input::Float(v.1)), ("k", Input::Float(-0.5)),
                                                ("cx", Input::Float(cx)), ("cy", Input::Float(cy))])?;
        Some((num_of(&a, "x")?, num_of(&a, "y")?))
    }).collect::<Option<_>>()?;
    let all: Vec<(f64, f64)> = tri.iter().chain(&rot).chain(&refl).chain(&dil).copied().chain([(cx, cy)]).collect();
    let (xs, ys) = extent(&all, &[]);
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("P = {} to {}", xy((px, py)), xy(rot[0])));
    let ((x0, x1), _) = p.ranges();
    p.seg((x0, -(la * x0 + lc) / lb), (x1, -(la * x1 + lc) / lb), 3, true);
    for (shape, kk) in [(&tri.to_vec(), 3usize), (&rot, 0), (&refl, 2), (&dil, 1)] {
        p.fill(shape, kk);
        p.outline(shape, kk);
        p.dot(shape[0].0, shape[0].1, kk, 4.5);
    }
    p.dot(cx, cy, 3, 4.0);
    let (lx, ly) = off(&p, cx, cy, -7.0, 16.0);
    p.label(lx, ly, "end", "axp-lt", "centre");
    p.mark(rot[0].0, rot[0].1, &format!("P' {}", xy(rot[0])));
    p.key("given", 3);
    p.key(&format!("rotated {}°", num(ang)), 0);
    p.key("reflected", 2);
    p.key("dilated -0.5", 1);
    let list = |v: &[(f64, f64)]| v.iter().map(|q| xy(*q)).collect::<Vec<_>>().join(", ");
    Some(s.chart("shape", "Rotation, reflection and dilation of a triangle",
                 &format!("A triangle with the given point P = {} as its first vertex ({}) and three images, each vertex \
                           mapped by the model: rotated {}° about {} ({}), reflected in the line x + y = {} through the \
                           centre ({}) and dilated about the centre by -0.5 ({}). Marked: the image of P, {}.",
                          xy((px, py)), list(&tri), num(ang), xy((cx, cy)), list(&rot), num(cx + cy), list(&refl),
                          list(&dil), xy(rot[0])), &p.body()))
}

/// Tangents from an outside point to a circle, with the circle on the diameter CP meeting the circle at the same
/// touching points (that intersection a second run of the model).
pub fn tangents(s: &Solved) -> Option<Figure> {
    let (cx, cy, r, px, py) = (s.inp("cx")?, s.inp("cy")?, s.inp("r")?, s.inp("px")?, s.inp("py")?);
    let (t1, t2) = ((s.out("x1")?, s.out("y1")?), (s.out("x2")?, s.out("y2")?));
    let m = ((cx + px) / 2.0, (cy + py) / 2.0);
    let rm = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt() / 2.0;
    let ci = s.solve_other("circle_circle_intersection", &[("x1", Input::Float(cx)), ("y1", Input::Float(cy)), ("r1", Input::Float(r)),
                                                           ("x2", Input::Float(m.0)), ("y2", Input::Float(m.1)), ("r2", Input::Float(rm))]);
    let (xs, ys) = extent(&[(px, py)], &[((cx, cy), r), (m, rm)]);
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("tangent length = {}", num(s.out("length")?)));
    p.curve(&circle((cx, cy), r), 3);
    p.dashed(&circle(m, rm), 2);
    for t in [t1, t2] {
        let d = (t.0 - px, t.1 - py);
        p.seg((px, py), (px + d.0 * 1.35, py + d.1 * 1.35), 0, false);
        p.seg((cx, cy), t, 3, true);
        p.dot(t.0, t.1, 0, 5.0);
        let (lx, ly) = off(&p, t.0, t.1, 8.0, if t.1 >= cy { -8.0 } else { 18.0 });
        p.label(lx, ly, "start", "axp-vt", &xy(t));
    }
    let mut met = String::new();
    if let Some(a) = ci {
        if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (num_of(&a, "x1"), num_of(&a, "y1"), num_of(&a, "x2"), num_of(&a, "y2")) {
            p.dot(x1, y1, 2, 2.5);
            p.dot(x2, y2, 2, 2.5);
            met = format!(" The circle on the diameter CP (dashed) meets the circle at {} and {} (a second run of the model).",
                          xy((x1, y1)), xy((x2, y2)));
        }
    }
    for (q, nm) in [((cx, cy), "C"), ((px, py), "P")] {
        p.dot(q.0, q.1, 3, 4.5);
        let (lx, ly) = off(&p, q.0, q.1, 7.0, 16.0);
        p.label(lx, ly, "start", "axp-bt", nm);
    }
    Some(s.chart("shape", "Tangent lines from an outside point to a circle",
                 &format!("Tangents from P = {} to the circle of centre C = {} and radius {}: the model gives the touching \
                           points {} and {} and the tangent length {}.{met}", xy((px, py)), xy((cx, cy)), num(r), xy(t1),
                          xy(t2), num(s.out("length")?)), &p.body()))
}

/// Interior and exterior angles of the regular polygons with 3 to 20 sides (one run of the model each), as stems.
pub fn polygon_angles(s: &Solved) -> Option<Figure> {
    let n0 = s.vec("n").and_then(|v| v.first().copied()).or_else(|| s.inp("n"))?;
    let hi = (n0.max(20.0) as i64).min(60);
    let mut rows: Vec<(f64, f64, f64)> = vec![];
    for n in 3..=hi {
        if let Some(a) = s.with_input(&[("n", Input::Int(n))]) {
            if let (Some(i), Some(e)) = (num_of(&a, "interior_angle"), num_of(&a, "exterior_angle")) {
                rows.push((n as f64, i, e));
            }
        }
    }
    if rows.len() < 3 {
        return None;
    }
    let ia = s.out("interior_angle")?;
    let mut p = DPlot::new((2.0, hi as f64 + 1.0), (0.0, 180.0), "Number of sides n", "Angle (°)",
                          &format!("n = {}: interior {}°", num(n0), num(ia)));
    for (n, i, e) in &rows {
        p.seg((*n - 0.15, 0.0), (*n - 0.15, *i), 0, false);
        p.dot(*n - 0.15, *i, 0, 3.5);
        p.seg((*n + 0.15, 0.0), (*n + 0.15, *e), 2, false);
        p.dot(*n + 0.15, *e, 2, 3.5);
    }
    p.hline(180.0);
    p.mark(n0 - 0.15, ia, &format!("{}°", num(ia)));
    p.key("interior", 0);
    p.key("exterior", 2);
    let first = rows[0];
    let last = rows[rows.len() - 1];
    Some(s.chart("stems", &format!("Angles of regular polygons, 3 to {hi} sides"),
                 &format!("Interior and exterior angles of the regular polygons with 3 to {hi} sides, each computed by the \
                           model: the interior angle climbs towards 180° ({} sides {}°, {} sides {}°) while the exterior \
                           angle falls; marked: n = {}, interior angle {}°, exterior {}°.", num(first.0), num(first.1),
                          num(last.0), num(last.1), num(n0), num(ia), num(s.out("exterior_angle")?)), &p.body()))
}

/// The conic's type over its xy and y² coefficients (15 x 15 runs of the model), the other coefficients as given.
pub fn conic_map(s: &Solved) -> Option<Figure> {
    let (b0, c0) = (s.inp("B")?, s.inp("C")?);
    let w = |v: f64| v.abs().max(4.0);
    let (bw, cw) = (w(b0), w(c0));
    let xs = k::span(b0 - bw, b0 + bw, 15, false);
    let ys = k::span(c0 - cw, c0 + cw, 15, false);
    let mut names: Vec<String> = vec![];
    let mut cells = vec![];
    let mut counts: Vec<usize> = vec![];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            let Some(a) = s.with(&[("B", *x), ("C", *y)]) else { continue };
            let Some(t) = a.values.iter().find(|v| v.0 == "type").and_then(|v| v.1.as_str()).map(String::from) else { continue };
            let kk = match names.iter().position(|n| *n == t) {
                Some(kk) => kk,
                None => {
                    names.push(t.clone());
                    counts.push(0);
                    names.len() - 1
                }
            };
            counts[kk] += 1;
            cells.push((i, j, t));
        }
    }
    if cells.len() < 20 {
        return None;
    }
    let t0 = s.out_text("type")?;
    let body = k::decision(&xs, &ys, &cells, &names, (b0, c0), "B (coefficient of xy)", "C (coefficient of y²)",
                           &format!("B = {}, C = {}: {t0}", num(b0), num(c0)));
    let other: Vec<String> = ["A", "D", "E", "F"].iter().filter_map(|n| s.inp(n).map(|v| format!("{n} = {}", num(v)))).collect();
    Some(s.chart("decision", "Conic type over the xy and y² coefficients",
                 &format!("Type of the conic A x² + B xy + C y² + D x + E y + F = 0 over B from {} to {} and C from {} to {}, \
                           with {} as given. Each of the 15 × 15 cells is one run of the model, coloured by the type it \
                           returned: {}. Marked: B = {}, C = {}, {t0}.", num(xs[0]), num(xs[14]), num(ys[0]), num(ys[14]),
                          other.join(", "), names.iter().zip(&counts).map(|(n, c)| format!("{n} {c} cells"))
                              .collect::<Vec<_>>().join(", "), num(b0), num(c0)), &body))
}

/// Two lines in space in an oblique view (x across, y up, z receding at 30°), the coordinate axes, the stated points.
pub fn skew_lines(s: &Solved) -> Option<Figure> {
    let g = |n: &str| -> Option<[f64; 3]> {
        let v = s.vec(n)?;
        (v.len() == 3).then(|| [v[0], v[1], v[2]])
    };
    let (p1, d1, p2, d2) = (g("p1")?, g("d1")?, g("p2")?, g("d2")?);
    let dist = s.out("distance")?;
    let par = s.out_text("parallel")?;
    let (c, sn) = (0.5 * 30f64.to_radians().cos(), 0.5 * 30f64.to_radians().sin());
    let proj = |v: [f64; 3]| (v[0] + c * v[2], v[1] + sn * v[2]);
    let unit = |d: [f64; 3]| -> Option<[f64; 3]> {
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        (n > 0.0).then(|| [d[0] / n, d[1] / n, d[2] / n])
    };
    let scale = [p1, p2].iter().flat_map(|p| p.iter().map(|x| x.abs())).fold(1.0, f64::max).max(dist);
    let (u1, u2) = (unit(d1)?, unit(d2)?);
    let line = |p: [f64; 3], u: [f64; 3]| -> Vec<(f64, f64)> {
        [-1.5, 1.5].iter().map(|t| proj([p[0] + u[0] * t * scale, p[1] + u[1] * t * scale, p[2] + u[2] * t * scale])).collect()
    };
    let (l1, l2) = (line(p1, u1), line(p2, u2));
    let ax = scale * 1.2;
    let axes = [([ax, 0.0, 0.0], "x"), ([0.0, ax, 0.0], "y"), ([0.0, 0.0, ax], "z")];
    let mut all: Vec<(f64, f64)> = l1.iter().chain(&l2).copied().collect();
    all.push((0.0, 0.0));
    all.extend(axes.iter().map(|a| proj(a.0)));
    let (xs, ys) = extent(&all, &[]);
    let mut p = DPlot::equal(&xs, &ys, "", "", &format!("distance = {} (parallel: {par})", num(dist)));
    for (a, nm) in axes {
        p.arrow((0.0, 0.0), proj(a), 3);
        let q = proj(a);
        let (lx, ly) = off(&p, q.0, q.1, 6.0, -6.0);
        p.label(lx, ly, "start", "axp-lt", nm);
    }
    p.curve(&l1, 0);
    p.curve(&l2, 2);
    for (q, nm, kk) in [(p1, "p1", 0usize), (p2, "p2", 2)] {
        let v = proj(q);
        p.dot(v.0, v.1, kk, 5.0);
        let (lx, ly) = off(&p, v.0, v.1, 8.0, 16.0);
        p.label(lx, ly, "start", "axp-bt", nm);
    }
    p.key("line 1", 0);
    p.key("line 2", 2);
    let t = |v: [f64; 3]| format!("({} {} {})", num(v[0]), num(v[1]), num(v[2]));
    Some(s.chart("geometry", "Two lines in space and their shortest distance",
                 &format!("The two stated lines in an oblique view (z drawn receding at 30°): line 1 through {} along {}, \
                           line 2 through {} along {}. The model returned their shortest distance {} (parallel: {par}). \
                           Marked: the stated point of each line.", t(p1), t(d1), t(p2), t(d2), num(dist)), &p.body()))
}

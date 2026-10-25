"""Geometry procedures: plane and solid geometry, coordinate geometry, conic sections, polygons and polyhedra,
computational geometry, spherical geometry and curves and surfaces (domain code, loaded by the pack; enabled only
once tools/certify_procedures.py has certified them).

Every procedure states its inputs, outputs, conditions, method, LaTeX, the documented question templates it parses,
worked examples, an INDEPENDENT reference computation (a checking library -- numpy, scipy.spatial, scipy.integrate,
sympy.geometry, shapely -- imported inside the reference and used only by certification) and a deliberately wrong
variant (certification shows the checks reject it). The shipped code is plain Python (math, fractions, itertools).

Conventions (stated in each procedure's conditions):
  * lengths are pure numbers in one consistent unit; areas and volumes come out in that unit squared / cubed, and a
    question that mixes length units is refused;
  * angles are given and reported in degrees (a value written with 'rad' or 'radians' is converted);
  * points in the plane are written (x, y); a list of points is given as "x1 y1, x2 y2, ..." or "(x1, y1), (x2, y2)";
  * a line is a*x + b*y + c = 0 (questions may write y = m*x + k or any linear equation);
  * a polygon is given by its vertices in order (either orientation) and must be simple;
  * point lists in answers are counterclockwise, starting at the vertex with the smallest x (then smallest y);
  * two answer points are ordered by x, then by y;
  * yes/no decisions (collinear, tangent, on the boundary, convex, parallel) are exact on the decimals as written;
  * numeric outputs are rounded to 12 significant digits (math libraries differ in the last bit between platforms);
  * spherical positions are (latitude, longitude) in signed decimal degrees; the Earth is a sphere of radius 6371 km;
  * curvature signs: a surface z = f(x, y) uses the upward normal; the torus is positive where it bends away from the
    outward normal; a plane's unit normal has its first nonzero component positive.
"""
from __future__ import annotations

import itertools
import math
import re
from fractions import Fraction

PROCEDURES: dict = {}
LEVELS: dict = {}

_N = r"[-+]?(?:\d+(?:\.\d+)?|\.\d+)(?:e[-+]?\d+)?"


# ==================================================================================================================
# shared helpers
# ==================================================================================================================
def _safe(fn):
    """A verifier: a residual from a check that may meet malformed outputs; any error means reject."""
    def verify(inputs, outputs):
        try:
            r = fn(inputs, outputs)
        except Exception:
            return 1.0
        if r is True:
            return 0.0
        if r is False or r is None:
            return 1.0
        r = float(r)
        return r if r == r else 1.0
    return verify


def _rel(a, b) -> float:
    if a is None or b is None:
        return 0.0 if a is None and b is None else 1.0
    return abs(a - b) / max(1.0, abs(a), abs(b))


def _res(*pairs) -> float:
    return max((_rel(a, b) for a, b in pairs), default=0.0)


def _fin(v, name):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
        raise ValueError(f"{name} must be a finite number")
    return float(v)


def _pos(v, name):
    v = _fin(v, name)
    if v <= 0:
        raise ValueError(f"{name} must be positive")
    return v


def _fr(v) -> Fraction:
    """The exact value of a number as written (0.1 -> 1/10), for exact comparisons."""
    return Fraction(repr(float(v))) if isinstance(v, float) else Fraction(v)


def _pairs(xs, name, n_min=1, n_max=None):
    """A flat list x1, y1, x2, y2, ... -> [(x1, y1), ...]."""
    xs = [_fin(float(x), name) for x in xs]
    if len(xs) % 2:
        raise ValueError(f"{name}: give an x and a y for every point (an even count of numbers)")
    pts = [(xs[i], xs[i + 1]) for i in range(0, len(xs), 2)]
    if len(pts) < n_min:
        raise ValueError(f"{name}: needs at least {n_min} points")
    if n_max is not None and len(pts) > n_max:
        raise ValueError(f"{name}: allows at most {n_max} points")
    return pts


def _triples(xs, name, n):
    xs = [_fin(float(x), name) for x in xs]
    if len(xs) != 3 * n:
        raise ValueError(f"{name}: give exactly {n} points of three coordinates")
    return [tuple(xs[3 * i:3 * i + 3]) for i in range(n)]


def _flat(pts):
    return [c for p in pts for c in p]


def _cross(o, a, b):
    return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])


def _xcross(o, a, b):
    """The exact cross product (Fractions of the floats): its sign is never a rounding accident."""
    o, a, b = [(_fr(p[0]), _fr(p[1])) for p in (o, a, b)]
    return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])


def _shoelace(pts) -> float:
    s = math.fsum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))
    return s / 2


def _perimeter(pts, closed=True) -> float:
    n = len(pts)
    m = n if closed else n - 1
    return math.fsum(math.dist(pts[i], pts[(i + 1) % n]) for i in range(m))


def _on_segment_exact(p, a, b) -> bool:
    if _xcross(a, b, p) != 0:
        return False
    return min(a[0], b[0]) <= p[0] <= max(a[0], b[0]) and min(a[1], b[1]) <= p[1] <= max(a[1], b[1])


def _segments_meet_exact(p1, p2, p3, p4) -> bool:
    d1, d2 = _xcross(p3, p4, p1), _xcross(p3, p4, p2)
    d3, d4 = _xcross(p1, p2, p3), _xcross(p1, p2, p4)
    if ((d1 > 0 > d2) or (d1 < 0 < d2)) and ((d3 > 0 > d4) or (d3 < 0 < d4)):
        return True
    return (d1 == 0 and _on_segment_exact(p1, p3, p4)) or (d2 == 0 and _on_segment_exact(p2, p3, p4)) or \
        (d3 == 0 and _on_segment_exact(p3, p1, p2)) or (d4 == 0 and _on_segment_exact(p4, p1, p2))


def _simple_polygon(pts, name="points"):
    """Raise ValueError unless the closed polygon is simple (no repeated vertex, no two edges meeting except
    neighbours at their shared vertex) and has a nonzero area."""
    n = len(pts)
    if n < 3:
        raise ValueError(f"{name}: a polygon needs at least 3 vertices")
    if len(set(pts)) != n:
        raise ValueError(f"{name}: a vertex is repeated")
    if n > 400:
        raise ValueError(f"{name}: at most 400 vertices")
    for i in range(n):
        a, b = pts[i], pts[(i + 1) % n]
        for j in range(i + 1, n):
            if j == i or (j + 1) % n == i or j == (i + 1) % n:
                continue
            c, d = pts[j], pts[(j + 1) % n]
            if _segments_meet_exact(a, b, c, d):
                raise ValueError(f"{name}: the polygon is not simple (edges {i + 1} and {j + 1} meet)")
    for i in range(n):        # a neighbour edge folding back over its predecessor
        a, b, c = pts[i - 1], pts[i], pts[(i + 1) % n]
        if _xcross(a, b, c) == 0 and ((b[0] - a[0]) * (c[0] - b[0]) + (b[1] - a[1]) * (c[1] - b[1])) < 0:
            raise ValueError(f"{name}: the polygon is not simple (an edge doubles back)")
    if _shoelace(pts) == 0:
        raise ValueError(f"{name}: the polygon has zero area")


def _star_polygon(rng, n_lo=3, n_hi=12, scale=20.0):
    """A random simple polygon: vertices at sorted, well-separated angles around a centre, random radii."""
    n = rng.randint(n_lo, n_hi)
    while True:
        angs = sorted(rng.uniform(0, 2 * math.pi) for _ in range(n))
        gaps = [angs[i + 1] - angs[i] for i in range(n - 1)] + [2 * math.pi - angs[-1] + angs[0]]
        if min(gaps) > 0.25 / n and max(gaps) < math.pi * 0.9:
            break
    cx, cy = rng.uniform(-scale, scale), rng.uniform(-scale, scale)
    pts = []
    for a in angs:
        r = rng.uniform(0.3, 1.0) * scale
        pts.append((round(cx + r * math.cos(a), 2), round(cy + r * math.sin(a), 2)))
    if rng.random() < 0.5:
        pts.reverse()
    return pts


def _ccw_start(pts):
    """Rotate a counterclockwise cycle to start at the smallest (x, y)."""
    i = min(range(len(pts)), key=lambda k: (pts[k][0], pts[k][1]))
    return pts[i:] + pts[:i]


def _deg(r):
    return math.degrees(r)


def _rad(d):
    return math.radians(d)


def _sort_pts(pts):
    return sorted(pts, key=lambda p: (round(p[0], 9), round(p[1], 9)))


# --- reading questions -------------------------------------------------------------------------------------------
def _norm(question: str) -> str:
    q = question.lower()
    for a, b in (("−", "-"), ("–", "-"), ("²", "^2"), ("³", "^3"), ("**", "^"), ("×", " by "), ("π", "pi"),
                 ("°", " degrees"), ("·", "*"), ("’", "'")):
        q = q.replace(a, b)
    return q


def _after(q, words):
    """The number written right after one of the words: 'radius 5', 'radius of 5', 'radius r = 5', 'radius: 5'."""
    m = re.search(r"\b(?:" + words + r")\b(?:\s*(?:of\b|is\b|=|:|equal to\b|equals\b|measuring\b|measures\b|"
                  r"length\b|lengths\b|[a-z]\b(?!\s*[a-z]{2})))*\s*(" + _N + r")", q)
    if m:
        return float(m.group(1))
    adj = [w for w in words.split("|") if w in ("long", "wide", "tall", "high", "deep", "thick")]
    if adj:
        m = re.search(r"(" + _N + r")\s*(?:" + _UNIT[1:-3] + r")?\s*(?:" + "|".join(adj) + r")\b", q)
        if m:
            return float(m.group(1))
    return None


def _two_after(q, words):
    """Two numbers after a word: 'legs 3 and 4', 'bases of 5 cm and 9 cm', 'diagonals 6, 8'."""
    m = re.search(r"\b(?:" + words + r")\b(?:\s*(?:of\b|are\b|is\b|=|:|measuring\b|lengths?\b))*\s*(" + _N +
                  r")\s*(?:[a-z]{1,6}\b)?\s*(?:and|,|&)\s*(" + _N + r")", q)
    return (float(m.group(1)), float(m.group(2))) if m else None


_P2 = r"\(\s*(" + _N + r")\s*,\s*(" + _N + r")\s*\)"
_P3 = r"\(\s*(" + _N + r")\s*,\s*(" + _N + r")\s*,\s*(" + _N + r")\s*\)"


def _pts2(q):
    return [(float(a), float(b)) for a, b in re.findall(_P2, q)]


def _pts3(q):
    return [(float(a), float(b), float(c)) for a, b, c in re.findall(_P3, q)]


_UNIT = r"(mm|cm|dm|km|m|ft|feet|foot|inches|inch|yd|yards?|miles?)\b"
_UNORM = {"feet": "ft", "foot": "ft", "inches": "in", "inch": "in", "yard": "yd", "yards": "yd", "mile": "mi",
          "miles": "mi"}


def _mixed_units(q) -> bool:
    us = {_UNORM.get(u, u) for u in re.findall(r"\d\s*" + _UNIT, q)}
    return len(us) > 1


def _scoped(q, d):
    """Refuse a question that mixes length units (the answer would silently be wrong)."""
    if d is not None and _mixed_units(q):
        return {"_out_of_scope": "lengths are given in different units; state every length in one unit"}
    return d


def _angle_in(q, words=None):
    """An angle in degrees: '60 degrees', '60 deg', 'angle 60', '1.2 rad' (converted)."""
    m = re.search(r"(" + _N + r")\s*(?:rad|radians)\b", q)
    if m:
        return math.degrees(float(m.group(1)))
    m = re.search(r"(" + _N + r")\s*(?:degrees?|deg)\b", q)
    if m:
        return float(m.group(1))
    return _after(q, words) if words else None


def _clean(d):
    return {k: v for k, v in d.items() if v is not None}


# --- linear and quadratic equations in x and y ---------------------------------------------------------------------
_MONO = {"x^2": "xx", "y^2": "yy", "xy": "xy", "yx": "xy", "x*y": "xy", "y*x": "xy", "x": "x", "y": "y", "": "1"}


def _terms(expr: str):
    """Coefficients {'xx','xy','yy','x','y','1'} of a polynomial of degree <= 2 in x and y written term by term
    ('3x^2 - 2xy + y - 4'), or None when it is not one."""
    s = expr.replace(" ", "").replace("*x", "x").replace("*y", "y")
    if not s:
        return None
    if not s.startswith(("+", "-")):
        s = "+" + s
    terms = re.findall(r"[+-][^+-]+", s)
    if "".join(terms) != s:
        return None
    out = {k: 0.0 for k in ("xx", "xy", "yy", "x", "y", "1")}
    for t in terms:
        m = re.fullmatch(r"([+-])((?:\d+(?:\.\d+)?|\.\d+)(?:/(?:\d+(?:\.\d+)?))?)?(x\^2|y\^2|xy|yx|x|y)?", t)
        if not m or (m.group(2) is None and m.group(3) is None):
            return None
        c = float(Fraction(m.group(2))) if m.group(2) else 1.0
        out[_MONO[m.group(3) or ""]] += -c if m.group(1) == "-" else c
    return out


def _equation(text: str):
    """'lhs = rhs' -> coefficients of lhs - rhs (degree <= 2 in x, y), or None."""
    if text.count("=") != 1:
        return None
    l, r = text.split("=")
    a, b = _terms(l), _terms(r)
    if a is None or b is None:
        return None
    return {k: a[k] - b[k] for k in a}


_EQ_CHARS = r"[-+0-9.xy^*/ ]"
# an equation never starts inside a word: "represented by -4x + 0y + 20 = 0" was read with a +y from "by"
# (tools/ask_sweep.py, 2026-10-23)
_EQ_AT_WORD_START = r"(?<![A-Za-z])"


def _equations(q):
    """Every equation in x and y written in the question, as coefficient dictionaries (in order)."""
    out = []
    for m in re.finditer(_EQ_AT_WORD_START + r"(" + _EQ_CHARS + r"+)=(" + _EQ_CHARS + r"+)", q):
        lhs, rhs = m.group(1).strip(), m.group(2).strip()
        rhs = re.split(r"\s+(?=[a-z]{2})", rhs)[0].rstrip(". ")            # stop at the next word
        lhs = re.split(r"(?<=[a-z]{2})\s+", lhs)[-1]
        if not re.search(r"[xy]", lhs + rhs):
            continue
        e = _equation(lhs.strip() + "=" + rhs.strip())
        if e is not None:
            out.append(e)
    return out


def _line_of(e):
    """A linear equation's (a, b, c) for a*x + b*y + c = 0, or None."""
    if e is None or e["xx"] or e["xy"] or e["yy"] or (e["x"] == 0 and e["y"] == 0):
        return None
    return e["x"], e["y"], e["1"]


def _lines(q):
    out = []
    for e in _equations(q):
        ln = _line_of(e)
        if ln:
            out.append(ln)
    if re.search(r"\bthe x-axis\b", q):
        out.append((0.0, 1.0, 0.0))
    if re.search(r"\bthe y-axis\b", q):
        out.append((1.0, 0.0, 0.0))
    return out


def _line_check(a, b, c, name="line"):
    a, b, c = _fin(a, f"{name} a"), _fin(b, f"{name} b"), _fin(c, f"{name} c")
    if a == 0 and b == 0:
        raise ValueError(f"{name}: a and b must not both be 0 (a*x + b*y + c = 0 is not a line)")
    return a, b, c


def _ref_line(a, b, c):
    """Two exact points on a*x + b*y + c = 0 as a sympy Line (references)."""
    import sympy as sp
    A, B, C = (sp.Rational(str(float(v))) for v in (a, b, c))
    if B != 0:
        return sp.Line(sp.Point(0, -C / B), sp.Point(1, -(A + C) / B))
    return sp.Line(sp.Point(-C / A, 0), sp.Point(-C / A, 1))


def _R(v):
    import sympy as sp
    return sp.Rational(str(float(v)))


def _F(e):
    """A sympy value to a float through 40 significant digits (no cancellation loss in nested radicals)."""
    import sympy as sp
    return float(sp.N(e, 40))


def _rand_len(rng, lo=0.5, hi=60.0, nd=2):
    return round(rng.uniform(lo, hi), nd) if rng.random() < 0.7 else float(rng.randint(max(1, int(lo) + 1), int(hi)))


def _r12(x):
    """12 significant digits: the last bits of cos, sin, exp, log differ between platforms' math libraries."""
    if isinstance(x, float) and math.isfinite(x):
        return float(f"{x:.12g}") + 0.0
    return x


def _rounded(fn):
    import functools

    @functools.wraps(fn)
    def run(*a, **k):
        out = fn(*a, **k)
        return {key: _r12(v) for key, v in out.items()}
    return run


def _add(key, level, **spec):
    spec.setdefault("conditions", [])
    for need in ("name", "topic", "purpose", "inputs", "outputs", "primary", "method", "latex", "run", "verify", "parse",
                 "reference", "wrong", "random_inputs", "templates", "examples", "questions"):
        if need not in spec:
            raise KeyError(f"{key}: missing {need}")
    spec["verify"] = _safe(spec["verify"])
    spec["run"] = _rounded(spec["run"])
    PROCEDURES[key] = spec
    LEVELS[key] = level


def _I(sym, meaning, required=True, **kw):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **kw}


def _V(sym, meaning, n_min=2, n_max=None, required=True):
    d = {"symbol": sym, "kind": "number", "type": "vector", "meaning": meaning, "required": required, "min_length": n_min}
    if n_max:
        d["max_length"] = n_max
    return d


def _O(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


_EXCL_SOLID = r"\b(pyramid|prism|cuboid|box|cube|cylinder|cone|sphere|frustum|tetrahedron|octahedron|dodecahedron|icosahedron|polyhedron)\b"


# ==================================================================================================================
# plane figures (beginner)
# ==================================================================================================================
def rectangle_measures(length, width):
    l, w = _pos(length, "length"), _pos(width, "width")
    return {"area": l * w, "perimeter": 2 * (l + w), "diagonal": math.hypot(l, w)}


def parse_rectangle(q0):
    q = _norm(q0)
    if re.search(_EXCL_SOLID + r"|\(|vertices|triangle|polygon|inscribed|circumscribed", q):
        return None
    if re.search(r"\bsquare\b", q) and not re.search(r"\brectangle\b", q):
        s = _after(q, r"side|sides|side length|edge")
        if not re.search(r"\b(area|perimeter|diagonal)\b", q) or re.search(r"\b(area|perimeter|diagonal)\b\s*(?:of|is|=)?\s*" + _N, q):
            return None
        return _scoped(q, _clean({"length": s, "width": s}))
    if not re.search(r"\brectangle\b", q) or re.search(r"\b(area|perimeter|diagonal)\b\s*(?:is|=|of)\s*" + _N, q):
        return None
    m = re.search(r"(" + _N + r")\s*(?:[a-z]{1,6}\s*)?(?:by|x)\s*(" + _N + r")", q)
    if m:
        return _scoped(q, {"length": float(m.group(1)), "width": float(m.group(2))})
    return _scoped(q, _clean({"length": _after(q, r"length|long"), "width": _after(q, r"width|wide|breadth")}))


_add("rectangle_measures", "beginner",
     name="Rectangle: area, perimeter and diagonal", topic="plane figures / quadrilaterals",
     purpose="Area, perimeter and diagonal of a rectangle (a square when length = width).",
     inputs=[_I("length", "length of the rectangle"), _I("width", "width of the rectangle")],
     outputs=[_O("area", "area l*w"), _O("perimeter", "perimeter 2(l + w)"), _O("diagonal", "diagonal sqrt(l^2 + w^2)")],
     primary="area", conditions=["length > 0", "width > 0"],
     method="A = l w, P = 2(l + w), d = sqrt(l^2 + w^2) (Pythagoras)", latex=r"A = lw,\quad P = 2(l+w),\quad d = \sqrt{l^2+w^2}",
     run=rectangle_measures,
     verify=lambda i, o: _res((o["area"], i["length"] * i["width"]), (o["perimeter"], 2 * (i["length"] + i["width"])),
                              (o["diagonal"] ** 2, i["length"] ** 2 + i["width"] ** 2)),
     parse=parse_rectangle,
     reference=lambda length, width: (lambda sg, sd: {"area": sg.box(0, 0, length, width).area,
                                                       "perimeter": sg.box(0, 0, length, width).length,
                                                       "diagonal": float(sd.euclidean((0, 0), (length, width)))})(
         __import__("shapely.geometry", fromlist=["box"]), __import__("scipy.spatial.distance", fromlist=["euclidean"])),
     wrong=lambda length, width: {"area": length * width, "perimeter": length + width, "diagonal": length + width},
     random_inputs=lambda rng: {"length": _rand_len(rng), "width": _rand_len(rng)},
     templates=["Find the area and perimeter of a rectangle with length 8 and width 5.",
                "What is the diagonal of a 6 by 8 rectangle?", "Find the area of a square with side 7."],
     examples=[{"inputs": {"length": 8, "width": 5}, "outputs": {"area": 40.0, "perimeter": 26.0}},
               {"inputs": {"length": 6, "width": 8}, "outputs": {"diagonal": 10.0}},
               {"inputs": {"length": 7, "width": 7}, "outputs": {"area": 49.0, "perimeter": 28.0}}],
     questions=[{"q": "Find the area and perimeter of a rectangle with length 8 and width 5.", "outputs": {"area": 40.0, "perimeter": 26.0}},
                {"q": "What is the diagonal of a 6 by 8 rectangle?", "outputs": {"diagonal": 10.0}},
                {"q": "Find the perimeter of a square with side 2.5 cm.", "outputs": {"perimeter": 10.0}},
                {"q": "A rectangle is 12 m long and 5 m wide. What is its diagonal?", "outputs": {"diagonal": 13.0}},
                {"q": "Find the area of a rectangle with length 3 m and width 40 cm.", "outcome": "OUT_OF_SCOPE"},
                {"q": "Find the area of a rectangle with length -3 and width 4.", "outcome": "INVALID_INPUT"},
                {"q": "Find the area of a rectangle with length 3.", "outcome": "NEEDS_INPUT"}])


def triangle_base_height(base, height):
    b, h = _pos(base, "base"), _pos(height, "height")
    return {"area": b * h / 2}


def parse_triangle_bh(q0):
    q = _norm(q0)
    if not re.search(r"\btriangle\b", q) or not re.search(r"\bbase\b", q) or not re.search(r"\b(height|altitude)\b", q):
        return None
    if re.search(_EXCL_SOLID + r"|\(|vertices|\bsides\b", q) or re.search(r"\barea\b\s*(?:is|=|of)\s*" + _N, q):
        return None
    return _scoped(q, _clean({"base": _after(q, r"base"), "height": _after(q, r"height|altitude")}))


_add("triangle_area_base_height", "beginner",
     name="Area of a triangle from base and height", topic="plane figures / triangles",
     purpose="The area of a triangle from a base and the height (altitude) to that base.",
     inputs=[_I("base", "length of the base"), _I("height", "height perpendicular to the base")],
     outputs=[_O("area", "area b*h/2")], primary="area", conditions=["base > 0", "height > 0"],
     method="A = b h / 2", latex=r"A = \tfrac{1}{2} b h",
     run=triangle_base_height, verify=lambda i, o: _rel(o["area"], i["base"] * i["height"] / 2), parse=parse_triangle_bh,
     reference=lambda base, height: {"area": __import__("shapely.geometry", fromlist=["Polygon"]).Polygon(
         [(0, 0), (base, 0), (0.37 * base, height)]).area},
     wrong=lambda base, height: {"area": base * height},
     random_inputs=lambda rng: {"base": _rand_len(rng), "height": _rand_len(rng)},
     templates=["Find the area of a triangle with base 10 and height 6.",
                "A triangle has a base of 7 cm and an altitude of 4 cm. What is its area?"],
     examples=[{"inputs": {"base": 10, "height": 6}, "outputs": {"area": 30.0}},
               {"inputs": {"base": 7, "height": 4}, "outputs": {"area": 14.0}}],
     questions=[{"q": "Find the area of a triangle with base 10 and height 6.", "outputs": {"area": 30.0}},
                {"q": "A triangle has a base of 7 cm and an altitude of 4 cm. What is its area?", "outputs": {"area": 14.0}},
                {"q": "Find the area of a triangle with base 0 and height 6.", "outcome": "INVALID_INPUT"},
                {"q": "Find the area of a triangle with base 10 and height not given.", "outcome": "NEEDS_INPUT"}])


def circle_measures(radius):
    r = _pos(radius, "radius")
    return {"area": math.pi * r * r, "circumference": 2 * math.pi * r, "diameter": 2 * r}


def _ref_circle(radius):
    import sympy as sp
    x, t = sp.symbols("x t", real=True)
    r = _R(radius)
    area = sp.integrate(2 * sp.sqrt(r ** 2 - x ** 2), (x, -r, r))
    circ = sp.integrate(sp.sqrt(sp.diff(r * sp.cos(t), t) ** 2 + sp.diff(r * sp.sin(t), t) ** 2), (t, 0, 2 * sp.pi))
    return {"area": float(area), "circumference": float(sp.simplify(circ)), "diameter": float(2 * r)}


def parse_circle(q0):
    q = _norm(q0)
    if not re.search(r"\bcircle\b|\bcircular\b", q):
        return None
    if re.search(r"\b(sector|arc|segment|chord|through|equation|cent(?:er|re)|tangent|intersect\w*|line|inscribed|"
                 r"circumscribed|annulus|ring|triangle|square|polygon|points?)\b|x\^2|" + _EXCL_SOLID + r"|\(", q):
        return None
    if not re.search(r"\b(area|circumference|perimeter|diameter)\b", q):
        return None
    if re.search(r"\b(area|circumference|perimeter)\b\s*(?:is|=|of)\s*" + _N, q):
        return None
    r = _after(q, r"radius")
    if r is None:
        d = _after(q, r"diameter")
        r = d / 2 if d is not None else None
    return _scoped(q, _clean({"radius": r}))


_add("circle_measures", "beginner",
     name="Circle: area, circumference and diameter", topic="plane figures / circles",
     purpose="Area, circumference and diameter of a circle from its radius (or diameter).",
     inputs=[_I("radius", "radius of the circle")],
     outputs=[_O("area", "area pi r^2"), _O("circumference", "circumference 2 pi r"), _O("diameter", "diameter 2r")],
     primary="area", conditions=["radius > 0"],
     method="A = pi r^2, C = 2 pi r", latex=r"A = \pi r^2,\quad C = 2\pi r",
     run=circle_measures,
     verify=lambda i, o: _res((o["area"], math.pi * i["radius"] ** 2), (o["circumference"], 2 * math.pi * i["radius"]),
                              (o["diameter"], 2 * i["radius"])),
     parse=parse_circle, reference=_ref_circle,
     wrong=lambda radius: {"area": 2 * math.pi * radius, "circumference": math.pi * radius, "diameter": radius},
     random_inputs=lambda rng: {"radius": _rand_len(rng, 0.1, 100)},
     templates=["Find the area of a circle with radius 3.", "What is the circumference of a circle with diameter 10?"],
     examples=[{"inputs": {"radius": 3}, "outputs": {"area": 28.274333882308138, "circumference": 18.84955592153876}},
               {"inputs": {"radius": 5}, "outputs": {"circumference": 31.41592653589793, "diameter": 10.0}}],
     questions=[{"q": "Find the area of a circle with radius 3.", "outputs": {"area": 28.274333882308138}},
                {"q": "What is the circumference of a circle with diameter 10?", "outputs": {"circumference": 31.41592653589793}},
                {"q": "A circular garden has a radius of 1 m. Find its area.", "outputs": {"area": math.pi}},
                {"q": "Find the area of a circle with radius -2.", "outcome": "INVALID_INPUT"},
                {"q": "Find the area of a circle.", "outcome": "NEEDS_INPUT"}])


def trapezoid_measures(a, b, height):
    a, b, h = _pos(a, "a"), _pos(b, "b"), _pos(height, "height")
    return {"area": (a + b) * h / 2, "midsegment": (a + b) / 2}


def parse_trapezoid(q0):
    q = _norm(q0)
    if not re.search(r"\btrapez(?:oid|ium)\b", q) or re.search(r"\(|vertices|" + _EXCL_SOLID, q):
        return None
    t = _two_after(q, r"parallel sides|bases|base lengths")
    d = {"height": _after(q, r"height|altitude")}
    if t:
        d.update(a=t[0], b=t[1])
    return _scoped(q, _clean(d))


_add("trapezoid_measures", "beginner",
     name="Trapezoid: area and midsegment", topic="plane figures / quadrilaterals",
     purpose="Area and midsegment (median) of a trapezoid from its parallel sides and height.",
     inputs=[_I("a", "first parallel side"), _I("b", "second parallel side"), _I("height", "distance between the parallel sides")],
     outputs=[_O("area", "(a + b) h / 2"), _O("midsegment", "(a + b) / 2")], primary="area",
     conditions=["a, b, height > 0"], method="A = (a + b) h / 2", latex=r"A = \frac{(a+b)\,h}{2}",
     run=trapezoid_measures,
     verify=lambda i, o: _res((o["area"], (i["a"] + i["b"]) * i["height"] / 2), (o["midsegment"], (i["a"] + i["b"]) / 2)),
     parse=parse_trapezoid,
     reference=lambda a, b, height: {"area": __import__("shapely.geometry", fromlist=["Polygon"]).Polygon(
         [(0, 0), (a, 0), (0.3 * a + b, height), (0.3 * a, height)]).area, "midsegment": (a + b) / 2},
     wrong=lambda a, b, height: {"area": (a + b) * height, "midsegment": a + b},
     random_inputs=lambda rng: {"a": _rand_len(rng), "b": _rand_len(rng), "height": _rand_len(rng)},
     templates=["Find the area of a trapezoid with parallel sides 6 and 10 and height 4.",
                "A trapezium has bases of 8 cm and 12 cm and a height of 5 cm. What is its area?"],
     examples=[{"inputs": {"a": 6, "b": 10, "height": 4}, "outputs": {"area": 32.0, "midsegment": 8.0}},
               {"inputs": {"a": 8, "b": 12, "height": 5}, "outputs": {"area": 50.0}}],
     questions=[{"q": "Find the area of a trapezoid with parallel sides 6 and 10 and height 4.", "outputs": {"area": 32.0}},
                {"q": "A trapezium has bases of 8 cm and 12 cm and a height of 5 cm. What is its area?", "outputs": {"area": 50.0}},
                {"q": "Find the area of a trapezoid with parallel sides 6 and 10.", "outcome": "NEEDS_INPUT"}])


def rhombus_measures(d1, d2):
    p, q = _pos(d1, "d1"), _pos(d2, "d2")
    s = math.hypot(p, q) / 2
    return {"area": p * q / 2, "side": s, "perimeter": 4 * s}


def parse_rhombus(q0):
    q = _norm(q0)
    if not re.search(r"\brhombus\b", q) or re.search(r"\(|vertices", q):
        return None
    t = _two_after(q, r"diagonals")
    return _scoped(q, {"d1": t[0], "d2": t[1]} if t else {})


_add("rhombus_measures", "beginner",
     name="Rhombus from its diagonals: area, side, perimeter", topic="plane figures / quadrilaterals",
     purpose="Area, side and perimeter of a rhombus from its two diagonals.",
     inputs=[_I("d1", "first diagonal"), _I("d2", "second diagonal")],
     outputs=[_O("area", "d1 d2 / 2"), _O("side", "sqrt(d1^2 + d2^2)/2"), _O("perimeter", "4 * side")], primary="area",
     conditions=["d1, d2 > 0 (the diagonals of a rhombus bisect each other at right angles)"],
     method="A = d1 d2 / 2; the half-diagonals are the legs of a right triangle whose hypotenuse is a side",
     latex=r"A = \tfrac{1}{2} d_1 d_2,\quad s = \tfrac{1}{2}\sqrt{d_1^2+d_2^2}",
     run=rhombus_measures,
     verify=lambda i, o: _res((o["area"], i["d1"] * i["d2"] / 2), (4 * o["side"] ** 2, (i["d1"] ** 2 + i["d2"] ** 2) / 4 * 4),
                              (o["perimeter"], 4 * o["side"])),
     parse=parse_rhombus,
     reference=lambda d1, d2: (lambda P: {"area": P.area, "perimeter": P.length, "side": P.length / 4})(
         __import__("shapely.geometry", fromlist=["Polygon"]).Polygon([(d1 / 2, 0), (0, d2 / 2), (-d1 / 2, 0), (0, -d2 / 2)])),
     wrong=lambda d1, d2: {"area": d1 * d2, "side": (d1 + d2) / 2, "perimeter": 2 * (d1 + d2)},
     random_inputs=lambda rng: {"d1": _rand_len(rng), "d2": _rand_len(rng)},
     templates=["Find the area of a rhombus with diagonals 6 and 8.", "A rhombus has diagonals of 10 cm and 24 cm. Find its perimeter."],
     examples=[{"inputs": {"d1": 6, "d2": 8}, "outputs": {"area": 24.0, "side": 5.0, "perimeter": 20.0}},
               {"inputs": {"d1": 10, "d2": 24}, "outputs": {"perimeter": 52.0}}],
     questions=[{"q": "Find the area of a rhombus with diagonals 6 and 8.", "outputs": {"area": 24.0, "side": 5.0}},
                {"q": "A rhombus has diagonals of 10 cm and 24 cm. Find its perimeter.", "outputs": {"perimeter": 52.0}},
                {"q": "Find the area of a rhombus.", "outcome": "NEEDS_INPUT"}])


def sector_measures(radius, angle):
    r, a = _pos(radius, "radius"), _fin(angle, "angle")
    if not 0 < a <= 360:
        raise ValueError("the central angle must be between 0 and 360 degrees")
    t = math.radians(a)
    return {"arc_length": r * t, "sector_area": r * r * t / 2, "chord": 2 * r * math.sin(t / 2)}


def _ref_sector(radius, angle):
    from scipy.integrate import quad
    import numpy as np
    from scipy.spatial.distance import euclidean
    t1 = np.radians(angle)
    x, y = (lambda t: radius * np.cos(t)), (lambda t: radius * np.sin(t))
    dx, dy = (lambda t: -radius * np.sin(t)), (lambda t: radius * np.cos(t))
    arc = quad(lambda t: np.hypot(dx(t), dy(t)), 0, t1, epsabs=1e-13, epsrel=1e-13)[0]
    area = quad(lambda t: 0.5 * (x(t) * dy(t) - y(t) * dx(t)), 0, t1, epsabs=1e-13, epsrel=1e-13)[0]
    return {"arc_length": arc, "sector_area": area, "chord": float(euclidean((radius, 0), (x(t1), y(t1))))}


def parse_sector(q0):
    q = _norm(q0)
    if not re.search(r"\b(sector|arc)\b", q) or re.search(r"\b(segment|parabola|curve|helix|ellipse|great[- ]circle|latitude|sphere)\b|\(|\by\s*=", q):
        return None
    r = _after(q, r"radius")
    if r is None and _after(q, r"diameter") is not None:
        r = _after(q, r"diameter") / 2
    return _scoped(q, _clean({"radius": r, "angle": _angle_in(q, r"angle|central angle")}))


_add("sector_measures", "beginner",
     name="Circular sector: arc length, area and chord", topic="plane figures / circles",
     purpose="Arc length, sector area and chord of a circular sector from the radius and central angle.",
     inputs=[_I("radius", "radius"), _I("angle", "central angle in degrees")],
     outputs=[_O("arc_length", "r theta (theta in radians)"), _O("sector_area", "r^2 theta / 2"), _O("chord", "2 r sin(theta/2)")],
     primary="arc_length", conditions=["radius > 0", "0 < angle <= 360 degrees"],
     method="theta = angle * pi/180; s = r theta; A = r^2 theta / 2; chord = 2 r sin(theta/2)",
     latex=r"s = r\theta,\quad A = \tfrac{1}{2} r^2\theta,\quad c = 2r\sin\tfrac{\theta}{2}",
     run=sector_measures,
     verify=lambda i, o: _res((o["arc_length"], i["radius"] * math.radians(i["angle"])),
                              (o["sector_area"], o["arc_length"] * i["radius"] / 2),
                              (o["chord"] ** 2, 2 * i["radius"] ** 2 * (1 - math.cos(math.radians(i["angle"]))))),
     parse=parse_sector, reference=_ref_sector,
     wrong=lambda radius, angle: {"arc_length": radius * angle, "sector_area": radius ** 2 * math.radians(angle),
                                  "chord": radius * math.radians(angle)},
     random_inputs=lambda rng: {"radius": _rand_len(rng), "angle": round(rng.uniform(1, 360), 1)},
     templates=["Find the arc length and area of a sector with radius 6 and central angle 60 degrees.",
                "A sector of a circle has radius 10 cm and angle 90°. What is its area?"],
     examples=[{"inputs": {"radius": 6, "angle": 60}, "outputs": {"arc_length": 2 * math.pi, "sector_area": 6 * math.pi, "chord": 6.0}},
               {"inputs": {"radius": 10, "angle": 90}, "outputs": {"sector_area": 25 * math.pi}}],
     questions=[{"q": "Find the arc length and area of a sector with radius 6 and central angle 60 degrees.",
                 "outputs": {"arc_length": 2 * math.pi, "sector_area": 6 * math.pi}},
                {"q": "A sector of a circle has radius 10 cm and angle 90°. What is its area?", "outputs": {"sector_area": 25 * math.pi}},
                {"q": "Find the arc length of a sector with radius 2 and angle 3.14159265358979 rad.", "outputs": {"arc_length": 6.28318530717958}},
                {"q": "Find the area of a sector with radius 4 and central angle 400 degrees.", "outcome": "INVALID_INPUT"},
                {"q": "Find the area of a sector with radius 4.", "outcome": "NEEDS_INPUT"}])


_POLY_NAMES = {"triangle": 3, "equilateral triangle": 3, "square": 4, "quadrilateral": 4, "pentagon": 5, "hexagon": 6,
               "heptagon": 7, "septagon": 7, "octagon": 8, "nonagon": 9, "enneagon": 9, "decagon": 10, "hendecagon": 11,
               "undecagon": 11, "dodecagon": 12, "icosagon": 20}


def _n_sides(q):
    m = re.search(r"(\d+)\s*(?:-\s*)?(?:sided\b|sides\b|gon\b)", q)
    if m:
        return int(m.group(1))
    m = re.search(r"\bn\s*=\s*(\d+)\b", q)
    if m:
        return int(m.group(1))
    for name, n in sorted(_POLY_NAMES.items(), key=lambda kv: -len(kv[0])):
        if re.search(r"\b" + name + r"\b", q):
            return n
    return None


def regular_polygon_angles(n):
    if isinstance(n, float):
        if n != int(n):
            raise ValueError("n must be a whole number")
        n = int(n)
    if n < 3 or n > 1000:
        raise ValueError("a polygon has between 3 and 1000 sides here")
    return {"interior_angle": (n - 2) * 180 / n, "exterior_angle": 360 / n, "interior_sum": float((n - 2) * 180),
            "diagonals": n * (n - 3) // 2}


def _ref_poly_angles(n):
    import sympy as sp
    rp = sp.RegularPolygon(sp.Point(0, 0), 1, int(n))
    diag = sum(1 for i, j in itertools.combinations(range(int(n)), 2) if (j - i) not in (1, int(n) - 1))
    return {"interior_angle": float(sp.deg(rp.interior_angle)), "exterior_angle": float(sp.deg(rp.exterior_angle)),
            "interior_sum": float(sp.deg(rp.interior_angle * int(n))), "diagonals": diag}


def parse_polygon_angles(q0):
    q = _norm(q0)
    if not re.search(r"\b(angles?|diagonals?)\b", q) or re.search(r"\(|vertices|" + _EXCL_SOLID, q):
        return None
    if not re.search(r"\bpolygon\b|gon\b|\b(pentagon|hexagon|heptagon|octagon|nonagon|decagon|dodecagon)\b", q):
        return None
    if re.search(r"\bside\s*(?:length|of|=|is)\b|\bside\s+" + _N + r"|\bedge\b|\barea\b|apothem|perimeter|circumradius|inradius", q):
        return None
    if re.search(_N + r"\s*degrees|\bangle\b\s*(?:is|=|of)\s*" + _N, q):
        return None                                 # 'how many sides has a polygon whose angle is 140 degrees': inverse
    n = _n_sides(q)
    return {"n": n} if n is not None else {}


_add("regular_polygon_angles", "beginner",
     name="Polygon angles and diagonals", topic="polygons / regular polygons",
     purpose="Each interior and exterior angle of a regular n-gon, the interior angle sum of any n-gon and its number of diagonals.",
     inputs=[{"symbol": "n", "kind": "number", "type": "integer", "meaning": "number of sides", "required": True, "min": 3, "max": 1000}],
     outputs=[_O("interior_angle", "each interior angle of the regular polygon, degrees"), _O("exterior_angle", "each exterior angle, degrees"),
              _O("interior_sum", "sum of the interior angles, degrees"), _O("diagonals", "number of diagonals n(n-3)/2")],
     primary="interior_angle", conditions=["3 <= n <= 1000 (whole number)", "each single angle assumes a regular polygon"],
     method="sum = (n - 2) 180; interior = sum / n; exterior = 360 / n; diagonals = n(n - 3)/2",
     latex=r"\theta_{int} = \frac{(n-2)\,180^\circ}{n},\quad \theta_{ext} = \frac{360^\circ}{n},\quad D = \frac{n(n-3)}{2}",
     run=regular_polygon_angles,
     verify=lambda i, o: _res((o["interior_angle"] + o["exterior_angle"], 180.0), (o["exterior_angle"] * i["n"], 360.0),
                              (o["interior_sum"], o["interior_angle"] * i["n"]),
                              (o["diagonals"], math.comb(int(i["n"]), 2) - int(i["n"]))),
     parse=parse_polygon_angles, reference=_ref_poly_angles,
     wrong=lambda n: {"interior_angle": 360 / n, "exterior_angle": 180 - 360 / n, "interior_sum": (n - 1) * 180.0,
                      "diagonals": n * (n - 1) // 2},
     random_inputs=lambda rng: {"n": rng.choice([rng.randint(3, 12), rng.randint(3, 60), rng.randint(3, 400)])},
     templates=["What is each interior angle of a regular hexagon?", "Find the sum of the interior angles of a polygon with 9 sides.",
                "How many diagonals does a regular decagon have?"],
     examples=[{"inputs": {"n": 6}, "outputs": {"interior_angle": 120.0, "exterior_angle": 60.0, "interior_sum": 720.0, "diagonals": 9}},
               {"inputs": {"n": 10}, "outputs": {"interior_angle": 144.0, "diagonals": 35}},
               {"inputs": {"n": "5"}, "outputs": {"interior_angle": 108.0, "interior_sum": 540.0}}],
     questions=[{"q": "What is each interior angle of a regular hexagon?", "outputs": {"interior_angle": 120.0}},
                {"q": "Find the sum of the interior angles of a polygon with 9 sides.", "outputs": {"interior_sum": 1260.0}},
                {"q": "How many diagonals does a regular decagon have?", "outputs": {"diagonals": 35}},
                {"q": "What is the exterior angle of a regular 12-sided polygon?", "outputs": {"exterior_angle": 30.0}},
                {"q": "What is each interior angle of a regular polygon with 2 sides?", "outcome": "INVALID_INPUT"},
                {"q": "What is each interior angle of a regular polygon?", "outcome": "NEEDS_INPUT"}])


def regular_polygon_measures(n, side):
    if isinstance(n, float):
        if n != int(n):
            raise ValueError("n must be a whole number")
        n = int(n)
    if n < 3 or n > 200:
        raise ValueError("a regular polygon has between 3 and 200 sides here")
    s = _pos(side, "side")
    apo, cr = s / (2 * math.tan(math.pi / n)), s / (2 * math.sin(math.pi / n))
    return {"area": n * s * apo / 2, "perimeter": n * s, "apothem": apo, "circumradius": cr}


def _ref_poly_measures(n, side):
    """Walk the polygon edge by edge (turning the exterior angle each time), then measure the shape."""
    from shapely.geometry import Point, Polygon
    pts, x, y, h = [], 0.0, 0.0, 0.0
    for _ in range(int(n)):
        pts.append((x, y))
        x, y, h = x + side * math.cos(h), y + side * math.sin(h), h + 2 * math.pi / int(n)
    P = Polygon(pts)
    c = P.centroid
    return {"area": P.area, "perimeter": P.length, "apothem": P.exterior.distance(c), "circumradius": c.distance(Point(pts[0]))}


def parse_polygon_measures(q0):
    q = _norm(q0)
    if not re.search(r"\bregular\b", q) or re.search(r"\(|vertices|" + _EXCL_SOLID, q):
        return None
    if not re.search(r"\b(area|perimeter|apothem|circumradius|inradius)\b", q):
        return None
    n = _n_sides(q)
    s = None
    m = re.search(r"\bsides?\s+of\s+(?:length\s+)?(" + _N + r")", q) or \
        re.search(r"\b(?:side length|side|edge length|edge)\b\s*(?:of|is|=|:|equal to)?\s*(" + _N + r")", q)
    if m:
        s = float(m.group(1))
    return _scoped(q, _clean({"n": n, "side": s}))


_add("regular_polygon_measures", "intermediate",
     name="Regular polygon: area, perimeter, apothem, circumradius", topic="polygons / regular polygons",
     purpose="Area, perimeter, apothem (inradius) and circumradius of a regular n-gon from its side length.",
     inputs=[{"symbol": "n", "kind": "number", "type": "integer", "meaning": "number of sides", "required": True, "min": 3, "max": 200},
             _I("side", "side length")],
     outputs=[_O("area", "n s a / 2"), _O("perimeter", "n s"), _O("apothem", "s / (2 tan(pi/n))"), _O("circumradius", "s / (2 sin(pi/n))")],
     primary="area", conditions=["3 <= n <= 200", "side > 0"],
     method="apothem a = s/(2 tan(pi/n)), R = s/(2 sin(pi/n)), A = (perimeter * apothem)/2",
     latex=r"a = \frac{s}{2\tan(\pi/n)},\quad R = \frac{s}{2\sin(\pi/n)},\quad A = \tfrac12 n s a",
     run=regular_polygon_measures,
     verify=lambda i, o: _res((o["apothem"] ** 2 + (i["side"] / 2) ** 2, o["circumradius"] ** 2),
                              (o["area"], o["perimeter"] * o["apothem"] / 2), (o["perimeter"], i["n"] * i["side"]),
                              (o["circumradius"] * math.sin(math.pi / i["n"]) * 2, i["side"])),
     parse=parse_polygon_measures, reference=_ref_poly_measures,
     wrong=lambda n, side: (lambda R: {"area": n * side * R / 2, "perimeter": n * side, "apothem": R, "circumradius": R})(
         side / (2 * math.sin(math.pi / n))),
     random_inputs=lambda rng: {"n": rng.choice([rng.randint(3, 12), rng.randint(3, 100)]), "side": _rand_len(rng, 0.5, 30)},
     templates=["Find the area of a regular hexagon with side 4.", "What is the apothem of a regular octagon with side length 10?",
                "Find the area of a regular polygon with 7 sides of length 3."],
     examples=[{"inputs": {"n": 6, "side": 4}, "outputs": {"area": 24 * math.sqrt(3), "perimeter": 24.0, "apothem": 2 * math.sqrt(3), "circumradius": 4.0}},
               {"inputs": {"n": 4, "side": 5}, "outputs": {"area": 25.0, "apothem": 2.5}}],
     questions=[{"q": "Find the area of a regular hexagon with side 4.", "outputs": {"area": 24 * math.sqrt(3)}},
                {"q": "What is the apothem of a regular octagon with side length 10?", "outputs": {"apothem": 5 + 5 * math.sqrt(2)}},
                {"q": "Find the area of a regular polygon with 4 sides of length 5.", "outputs": {"area": 25.0}},
                {"q": "Find the area of a regular hexagon with side -4.", "outcome": "INVALID_INPUT"},
                {"q": "Find the area of a regular hexagon.", "outcome": "NEEDS_INPUT"}])


# ==================================================================================================================
# solids (beginner / intermediate)
# ==================================================================================================================
def cuboid_measures(length, width, height):
    l, w, h = _pos(length, "length"), _pos(width, "width"), _pos(height, "height")
    return {"volume": l * w * h, "surface_area": 2 * (l * w + l * h + w * h), "space_diagonal": math.sqrt(l * l + w * w + h * h)}


def _ref_cuboid(length, width, height):
    from scipy.spatial import ConvexHull
    from scipy.spatial.distance import euclidean
    pts = [(x, y, z) for x in (0, length) for y in (0, width) for z in (0, height)]
    hull = ConvexHull(pts)
    return {"volume": hull.volume, "surface_area": hull.area, "space_diagonal": float(euclidean((0, 0, 0), (length, width, height)))}


def parse_cuboid(q0):
    q = _norm(q0)
    if not re.search(r"\b(cuboid|rectangular box|rectangular prism|rectangular solid|box|cube)\b", q) or \
            re.search(r"\(|vertices|sphere|cylinder|circumradius|inradius|faces|edges|euler", q):
        return None
    if re.search(r"\b(volume|surface area|area|diagonal)\b\s*(?:is|=|of)\s*" + _N, q):
        return None
    if re.search(r"\bcube\b", q) and not re.search(r"\b(cuboid|box|prism)\b", q):
        s = _after(q, r"side|edge|side length|edge length|sides|edges")
        return _scoped(q, _clean({"length": s, "width": s, "height": s}))
    m = re.search(r"(" + _N + r")\s*(?:[a-z]{1,6}\s*)?(?:by|x)\s*(" + _N + r")\s*(?:[a-z]{1,6}\s*)?(?:by|x)\s*(" + _N + r")", q)
    if m:
        return _scoped(q, {"length": float(m.group(1)), "width": float(m.group(2)), "height": float(m.group(3))})
    return _scoped(q, _clean({"length": _after(q, r"length|long"), "width": _after(q, r"width|wide|breadth"),
                              "height": _after(q, r"height|high|tall|depth|deep")}))


_add("cuboid_measures", "beginner",
     name="Cuboid (rectangular box): volume, surface area, space diagonal", topic="solids / prisms",
     purpose="Volume, total surface area and space diagonal of a rectangular box (a cube when all edges are equal).",
     inputs=[_I("length", "length"), _I("width", "width"), _I("height", "height")],
     outputs=[_O("volume", "l w h"), _O("surface_area", "2(lw + lh + wh)"), _O("space_diagonal", "sqrt(l^2 + w^2 + h^2)")],
     primary="volume", conditions=["length, width, height > 0"],
     method="V = l w h; S = 2(lw + lh + wh); d = sqrt(l^2 + w^2 + h^2)",
     latex=r"V = lwh,\quad S = 2(lw+lh+wh),\quad d = \sqrt{l^2+w^2+h^2}",
     run=cuboid_measures,
     verify=lambda i, o: _res((o["volume"], i["length"] * i["width"] * i["height"]),
                              (o["surface_area"], 2 * (i["length"] * i["width"] + i["length"] * i["height"] + i["width"] * i["height"])),
                              (o["space_diagonal"] ** 2, i["length"] ** 2 + i["width"] ** 2 + i["height"] ** 2)),
     parse=parse_cuboid, reference=_ref_cuboid,
     wrong=lambda length, width, height: {"volume": length * width * height,
                                          "surface_area": length * width + length * height + width * height,
                                          "space_diagonal": math.hypot(length, width)},
     random_inputs=lambda rng: {"length": _rand_len(rng), "width": _rand_len(rng), "height": _rand_len(rng)},
     templates=["Find the volume and surface area of a box 3 by 4 by 12.", "Find the surface area of a cube with edge 5.",
                "A rectangular prism has length 6, width 4 and height 3. Find its volume."],
     examples=[{"inputs": {"length": 3, "width": 4, "height": 12}, "outputs": {"volume": 144.0, "surface_area": 192.0, "space_diagonal": 13.0}},
               {"inputs": {"length": 5, "width": 5, "height": 5}, "outputs": {"volume": 125.0, "surface_area": 150.0}}],
     questions=[{"q": "Find the volume and surface area of a box 3 by 4 by 12.", "outputs": {"volume": 144.0, "surface_area": 192.0}},
                {"q": "Find the surface area of a cube with edge 5.", "outputs": {"surface_area": 150.0}},
                {"q": "A rectangular prism has length 6, width 4 and height 3. Find its volume.", "outputs": {"volume": 72.0}},
                {"q": "What is the space diagonal of a box 3 by 4 by 12?", "outputs": {"space_diagonal": 13.0}},
                {"q": "A rectangular prism has length 6 and width 4. Find its volume.", "outcome": "NEEDS_INPUT"}])


def _radius_of(q):
    r = _after(q, r"radius|base radius")
    if r is None:
        d = _after(q, r"diameter")
        r = d / 2 if d is not None else None
    return r


def cylinder_measures(radius, height):
    r, h = _pos(radius, "radius"), _pos(height, "height")
    return {"volume": math.pi * r * r * h, "lateral_area": 2 * math.pi * r * h, "total_area": 2 * math.pi * r * (r + h)}


def _ref_cylinder(radius, height):
    import sympy as sp
    z, th = sp.symbols("z theta", real=True)
    r, h = _R(radius), _R(height)
    vol = sp.integrate(sp.pi * r ** 2, (z, 0, h))
    lat = sp.integrate(sp.integrate(r, (th, 0, 2 * sp.pi)), (z, 0, h))
    disk = sp.integrate(sp.integrate(sp.Symbol("rho") , (sp.Symbol("rho"), 0, r)), (th, 0, 2 * sp.pi))
    return {"volume": float(vol), "lateral_area": float(lat), "total_area": float(lat + 2 * disk)}


def parse_cylinder(q0):
    q = _norm(q0)
    if not re.search(r"\bcylind(?:er|rical)\b", q) or re.search(r"\b(cone|frustum|sphere)\b|\(", q):
        return None
    if re.search(r"\b(volume|area)\b\s*(?:is|=|of)\s*" + _N, q):
        return None
    return _scoped(q, _clean({"radius": _radius_of(q), "height": _after(q, r"height|length|tall|high")}))


_add("cylinder_measures", "beginner",
     name="Cylinder: volume and surface area", topic="solids / cylinders",
     purpose="Volume, lateral (curved) area and total surface area of a right circular cylinder.",
     inputs=[_I("radius", "radius of the base"), _I("height", "height")],
     outputs=[_O("volume", "pi r^2 h"), _O("lateral_area", "2 pi r h"), _O("total_area", "2 pi r (r + h)")], primary="volume",
     conditions=["radius, height > 0"], method="V = pi r^2 h; lateral = 2 pi r h; total = lateral + 2 pi r^2",
     latex=r"V = \pi r^2 h,\quad A = 2\pi r h + 2\pi r^2",
     run=cylinder_measures,
     verify=lambda i, o: _res((o["volume"], math.pi * i["radius"] ** 2 * i["height"]), (o["lateral_area"], 2 * math.pi * i["radius"] * i["height"]),
                              (o["total_area"], o["lateral_area"] + 2 * math.pi * i["radius"] ** 2)),
     parse=parse_cylinder, reference=_ref_cylinder,
     wrong=lambda radius, height: {"volume": 2 * math.pi * radius * height, "lateral_area": math.pi * radius * height,
                                   "total_area": 2 * math.pi * radius * height + math.pi * radius ** 2},
     random_inputs=lambda rng: {"radius": _rand_len(rng, 0.2, 40), "height": _rand_len(rng)},
     templates=["Find the volume of a cylinder with radius 3 and height 5.",
                "What is the total surface area of a cylinder with diameter 10 cm and height 20 cm?"],
     examples=[{"inputs": {"radius": 3, "height": 5}, "outputs": {"volume": 45 * math.pi, "lateral_area": 30 * math.pi, "total_area": 48 * math.pi}},
               {"inputs": {"radius": 5, "height": 20}, "outputs": {"total_area": 250 * math.pi}}],
     questions=[{"q": "Find the volume of a cylinder with radius 3 and height 5.", "outputs": {"volume": 45 * math.pi}},
                {"q": "What is the total surface area of a cylinder with diameter 10 cm and height 20 cm?", "outputs": {"total_area": 250 * math.pi}},
                {"q": "Find the volume of a cylinder with radius 3 cm and height 2 m.", "outcome": "OUT_OF_SCOPE"},
                {"q": "Find the volume of a cylinder with radius 3.", "outcome": "NEEDS_INPUT"}])


def cone_measures(radius, height):
    r, h = _pos(radius, "radius"), _pos(height, "height")
    s = math.hypot(r, h)
    return {"volume": math.pi * r * r * h / 3, "slant_height": s, "lateral_area": math.pi * r * s, "total_area": math.pi * r * (r + s)}


def _ref_cone(radius, height):
    import sympy as sp
    z = sp.symbols("z", real=True)
    r, h = _R(radius), _R(height)
    f = r * (1 - z / h)
    vol = sp.integrate(sp.pi * f ** 2, (z, 0, h))
    lat = sp.integrate(2 * sp.pi * f * sp.sqrt(1 + sp.diff(f, z) ** 2), (z, 0, h))
    return {"volume": float(vol), "lateral_area": float(lat), "total_area": float(lat + sp.pi * r ** 2),
            "slant_height": float(sp.Point(0, 0).distance(sp.Point(r, h)))}


def parse_cone(q0):
    q = _norm(q0)
    if not re.search(r"\b(cone|conical)\b", q) or re.search(r"\b(frustum|truncated|solid angle|half[- ]angle|apex angle)\b|\(", q):
        return None
    if re.search(r"\b(volume|area)\b\s*(?:is|=|of)\s*" + _N, q):
        return None
    return _scoped(q, _clean({"radius": _radius_of(q), "height": _after(q, r"height|tall|high|altitude")}))


_add("cone_measures", "beginner",
     name="Cone: volume, slant height and surface area", topic="solids / cones",
     purpose="Volume, slant height, lateral area and total surface area of a right circular cone.",
     inputs=[_I("radius", "radius of the base"), _I("height", "perpendicular height")],
     outputs=[_O("volume", "pi r^2 h / 3"), _O("slant_height", "sqrt(r^2 + h^2)"), _O("lateral_area", "pi r l"),
              _O("total_area", "pi r (r + l)")], primary="volume", conditions=["radius, height > 0"],
     method="V = pi r^2 h/3; l = sqrt(r^2 + h^2); lateral = pi r l", latex=r"V = \tfrac13 \pi r^2 h,\quad l = \sqrt{r^2+h^2},\quad A_L = \pi r l",
     run=cone_measures,
     verify=lambda i, o: _res((o["volume"] * 3, math.pi * i["radius"] ** 2 * i["height"]), (o["slant_height"] ** 2, i["radius"] ** 2 + i["height"] ** 2),
                              (o["lateral_area"], math.pi * i["radius"] * o["slant_height"]), (o["total_area"], o["lateral_area"] + math.pi * i["radius"] ** 2)),
     parse=parse_cone, reference=_ref_cone,
     wrong=lambda radius, height: {"volume": math.pi * radius ** 2 * height, "slant_height": radius + height,
                                   "lateral_area": math.pi * radius * height, "total_area": math.pi * radius * (radius + height)},
     random_inputs=lambda rng: {"radius": _rand_len(rng, 0.2, 40), "height": _rand_len(rng)},
     templates=["Find the volume of a cone with radius 3 and height 4.", "What is the slant height of a cone with base radius 5 cm and height 12 cm?"],
     examples=[{"inputs": {"radius": 3, "height": 4}, "outputs": {"volume": 12 * math.pi, "slant_height": 5.0, "lateral_area": 15 * math.pi, "total_area": 24 * math.pi}},
               {"inputs": {"radius": 5, "height": 12}, "outputs": {"slant_height": 13.0}}],
     questions=[{"q": "Find the volume of a cone with radius 3 and height 4.", "outputs": {"volume": 12 * math.pi}},
                {"q": "What is the slant height of a cone with base radius 5 cm and height 12 cm?", "outputs": {"slant_height": 13.0}},
                {"q": "Find the total surface area of a cone with radius 3 and height 4.", "outputs": {"total_area": 24 * math.pi}},
                {"q": "Find the volume of a cone with height 4.", "outcome": "NEEDS_INPUT"}])


def sphere_measures(radius):
    r = _pos(radius, "radius")
    return {"volume": 4 * math.pi * r ** 3 / 3, "surface_area": 4 * math.pi * r * r}


def _ref_sphere(radius):
    import sympy as sp
    z, th, ph = sp.symbols("z theta phi", real=True)
    r = _R(radius)
    vol = sp.integrate(sp.pi * (r ** 2 - z ** 2), (z, -r, r))
    area = sp.integrate(sp.integrate(r ** 2 * sp.sin(ph), (ph, 0, sp.pi)), (th, 0, 2 * sp.pi))
    return {"volume": float(vol), "surface_area": float(area)}


def parse_sphere(q0):
    q = _norm(q0)
    if not re.search(r"\b(sphere|ball|spherical)\b", q) or re.search(
            r"\b(cap|spheroid|great[- ]circle|latitude|longitude|points|through|hemisphere|segment|triangle|earth|ellipsoid)\b|\(", q):
        return None
    if re.search(r"\b(volume|area)\b\s*(?:is|=|of)\s*" + _N, q) or not re.search(r"\b(volume|area|surface)\b", q):
        return None
    return _scoped(q, _clean({"radius": _radius_of(q)}))


_add("sphere_measures", "beginner",
     name="Sphere: volume and surface area", topic="solids / spheres",
     purpose="Volume and surface area of a sphere from its radius (or diameter).",
     inputs=[_I("radius", "radius")], outputs=[_O("volume", "4/3 pi r^3"), _O("surface_area", "4 pi r^2")], primary="volume",
     conditions=["radius > 0"], method="V = 4/3 pi r^3; S = 4 pi r^2", latex=r"V = \tfrac43\pi r^3,\quad S = 4\pi r^2",
     run=sphere_measures,
     verify=lambda i, o: _res((o["volume"], 4 * math.pi * i["radius"] ** 3 / 3), (o["surface_area"], 3 * o["volume"] / i["radius"])),
     parse=parse_sphere, reference=_ref_sphere,
     wrong=lambda radius: {"volume": 4 * math.pi * radius ** 3, "surface_area": 2 * math.pi * radius ** 2},
     random_inputs=lambda rng: {"radius": _rand_len(rng, 0.1, 50)},
     templates=["Find the volume of a sphere with radius 3.", "What is the surface area of a ball with diameter 10 cm?"],
     examples=[{"inputs": {"radius": 3}, "outputs": {"volume": 36 * math.pi, "surface_area": 36 * math.pi}},
               {"inputs": {"radius": 5}, "outputs": {"surface_area": 100 * math.pi}}],
     questions=[{"q": "Find the volume of a sphere with radius 3.", "outputs": {"volume": 36 * math.pi}},
                {"q": "What is the surface area of a ball with diameter 10 cm?", "outputs": {"surface_area": 100 * math.pi}},
                {"q": "Find the volume of a sphere with radius 0.", "outcome": "INVALID_INPUT"},
                {"q": "Find the volume of a sphere.", "outcome": "NEEDS_INPUT"}])


# ==================================================================================================================
# coordinate geometry (beginner / intermediate)
# ==================================================================================================================
# ==================================================================================================================
# triangles from sides (beginner / intermediate)
# ==================================================================================================================
_RANDP = lambda rng: round(rng.uniform(-50, 50), 2) if rng.random() < 0.7 else float(rng.randint(-20, 20))

def _tri_check(a, b, c):
    a, b, c = _pos(a, "a"), _pos(b, "b"), _pos(c, "c")
    x, y, z = sorted((_fr(a), _fr(b), _fr(c)))
    if x + y <= z:
        raise ValueError("the sides violate the triangle inequality (each side must be shorter than the sum of the other two)")
    return a, b, c


def _three_sides(q):
    m = re.search(r"\bsides?\b(?:\s*(?:of|are|=|:|measuring|lengths?|with lengths?))*\s*(" + _N + r")\s*(?:[a-z]{1,6}\b)?\s*,?\s*(" + _N +
                  r")\s*(?:[a-z]{1,6}\b)?\s*,?\s*(?:and\s*)?(" + _N + r")", q)
    if m:
        return [float(m.group(k)) for k in (1, 2, 3)]
    m = re.search(r"\ba\s*=\s*(" + _N + r").*?\bb\s*=\s*(" + _N + r").*?\bc\s*=\s*(" + _N + r")", q)
    if m:
        return [float(m.group(k)) for k in (1, 2, 3)]
    return None


_CLASSIFY = r"\b(classify|type of triangle|kind of triangle|what type|what kind|acute|obtuse|scalene|isosceles|equilateral|is it a right|is the triangle right|right[- ]angled\?)"


def triangle_classify(a, b, c):
    a, b, c = _tri_check(a, b, c)
    x, y, z = sorted((_fr(a), _fr(b), _fr(c)))
    sides = "equilateral" if x == z else ("isosceles" if x == y or y == z else "scalene")
    d = x * x + y * y - z * z
    angles = "right" if d == 0 else ("acute" if d > 0 else "obtuse")
    return {"by_sides": sides, "by_angles": angles}


def _ref_classify(a, b, c):
    """Exact coordinates (sympy), the largest angle compared with pi/2 symbolically."""
    import sympy as sp
    A_, B_, C_ = (sp.nsimplify(str(float(v)), rational=True) for v in (a, b, c))
    pB, pC = sp.Point(0, 0), sp.Point(A_, 0)
    pA = [p for p in sp.Circle(pB, C_).intersection(sp.Circle(pC, B_)) if p.y > 0][0]
    angs = sp.Triangle(pA, pB, pC).angles.values()
    big = max(angs, key=lambda t: float(t))
    diff = sp.nsimplify(sp.cos(big))
    angles = "right" if sp.simplify(diff) == 0 else ("obtuse" if float(diff) < 0 else "acute")
    s = {A_, B_, C_}
    return {"by_sides": {1: "equilateral", 2: "isosceles", 3: "scalene"}[len(s)], "by_angles": angles}


def _verify_classify(i, o):
    a, b, c = sorted((_fr(i["a"]), _fr(i["b"]), _fr(i["c"])))
    largest_cos = (a * a + b * b - c * c) / (2 * a * b)     # law of cosines at the largest angle
    want = "right" if largest_cos == 0 else ("obtuse" if largest_cos < 0 else "acute")
    eq = len({a, b, c})
    return o["by_angles"] == want and o["by_sides"] == {1: "equilateral", 2: "isosceles", 3: "scalene"}[eq]


def _rand_classify(rng):
    k = rng.random()
    if k < 0.15:
        t = rng.choice([(3, 4, 5), (5, 12, 13), (8, 15, 17), (7, 24, 25), (20, 21, 29)])
        f = rng.randint(1, 5)
        s = [v * f for v in t]
    elif k < 0.3:
        e = rng.randint(2, 30)
        s = [e, e, rng.randint(1, 2 * e - 1)]
    elif k < 0.35:
        e = rng.randint(1, 30)
        s = [e, e, e]
    else:
        a, b = rng.randint(2, 40), rng.randint(2, 40)
        s = [a, b, rng.randint(abs(a - b) + 1, a + b - 1)]
    rng.shuffle(s)
    return {"a": s[0], "b": s[1], "c": s[2]}


def parse_classify(q0):
    q = _norm(q0)
    if not re.search(r"\btriangle\b", q) or not re.search(_CLASSIFY, q) or re.search(r"\(|vertices", q):
        return None
    s = _three_sides(q)
    return {"a": s[0], "b": s[1], "c": s[2]} if s else {}


_add("triangle_classify", "beginner",
     name="Classify a triangle by its sides and angles", topic="plane figures / triangles",
     purpose="Whether a triangle with sides a, b, c is equilateral, isosceles or scalene, and acute, right or obtuse.",
     inputs=[_I("a", "side a"), _I("b", "side b"), _I("c", "side c")],
     outputs=[_O("by_sides", "equilateral / isosceles / scalene", "text"), _O("by_angles", "acute / right / obtuse", "text")],
     primary="by_angles", conditions=["a, b, c > 0", "triangle inequality", "the sides are compared exactly as written"],
     method="exact comparison: with z the longest side, x^2 + y^2 - z^2 > 0 acute, = 0 right, < 0 obtuse (converse of Pythagoras)",
     latex=r"x^2 + y^2 \gtreqless z^2 \;\Leftrightarrow\; \text{acute / right / obtuse}",
     run=triangle_classify, verify=_verify_classify, parse=parse_classify, reference=_ref_classify,
     wrong=lambda a, b, c: (lambda r: dict(r, by_angles={"acute": "obtuse", "obtuse": "acute", "right": "right"}[r["by_angles"]]))(triangle_classify(a, b, c)),
     random_inputs=_rand_classify,
     templates=["Classify the triangle with sides 3, 4 and 5.", "What type of triangle has sides 5, 5 and 8?"],
     examples=[{"inputs": {"a": 3, "b": 4, "c": 5}, "outputs": {"by_sides": "scalene", "by_angles": "right"}},
               {"inputs": {"a": 5, "b": 5, "c": 8}, "outputs": {"by_sides": "isosceles", "by_angles": "obtuse"}},
               {"inputs": {"a": 0.3, "b": 0.4, "c": 0.5}, "outputs": {"by_angles": "right"}},
               {"inputs": {"a": 6, "b": 6, "c": 6}, "outputs": {"by_sides": "equilateral", "by_angles": "acute"}}],
     questions=[{"q": "Classify the triangle with sides 3, 4 and 5.", "outputs": {"by_sides": "scalene", "by_angles": "right"}},
                {"q": "What type of triangle has sides 5, 5 and 8?", "outputs": {"by_sides": "isosceles", "by_angles": "obtuse"}},
                {"q": "Is the triangle with sides 7, 8 and 9 acute, right or obtuse?", "outputs": {"by_angles": "acute"}},
                {"q": "Classify the triangle with sides 1, 1 and 3.", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# polygons from coordinates (intermediate)
# ==================================================================================================================
_POLY_WORDS = r"\b(polygon|quadrilateral|pentagon|hexagon|heptagon|octagon|nonagon|decagon|figure|shape|region|rectangle|square|parallelogram|trapezoid|trapezium|kite)\b"


def _point_list(q):
    p = _pts2(q)
    if p:
        return _flat(p)
    m = re.search(r"\b(?:vertices|points|corners)\b\s*:?\s*((?:" + _N + r"\s+" + _N + r"\s*[,;]\s*)+" + _N + r"\s+" + _N + r")", q)
    if m:
        return [float(x) for x in re.findall(_N, m.group(1))]
    return None


def polygon_area(points):
    pts = _pairs(points, "points", 3)
    _simple_polygon(pts)
    s = _shoelace(pts)
    return {"area": abs(s), "perimeter": _perimeter(pts), "orientation": "counterclockwise" if s > 0 else "clockwise"}


def _ref_polygon_area(points):
    from shapely.geometry import Polygon
    P = Polygon(_pairs(points, "points", 3))
    assert P.is_valid
    return {"area": P.area, "perimeter": P.length, "orientation": "counterclockwise" if P.exterior.is_ccw else "clockwise"}


def _verify_polygon_area(i, o):
    """Area by the trapezoid rule (sum of (x2 - x1)(y1 + y2)/2), a different decomposition from the shoelace."""
    pts = _pairs(i["points"], "points", 3)
    n = len(pts)
    trap = -math.fsum((pts[(k + 1) % n][0] - pts[k][0]) * (pts[k][1] + pts[(k + 1) % n][1]) for k in range(n)) / 2
    scale = math.fsum(abs(pts[k][0]) + abs(pts[k][1]) for k in range(n)) ** 2 / n
    return max(abs(o["area"] - abs(trap)) / max(1.0, abs(trap), scale * 1e-6),
               _rel(o["perimeter"], math.fsum(math.hypot(pts[(k + 1) % n][0] - pts[k][0], pts[(k + 1) % n][1] - pts[k][1]) for k in range(n))),
               0.0 if o["orientation"] == ("counterclockwise" if trap > 0 else "clockwise") else 1.0)


def parse_polygon_area(q0):
    q = _norm(q0)
    if not re.search(_POLY_WORDS, q) or re.search(r"\btriangle\b", q) or not re.search(r"\b(area|perimeter|orientation|clockwise|counterclockwise)\b", q):
        return None
    if re.search(r"\b(centroid|cent(?:er|re) of (?:mass|gravity)|inside|contains?|within|hull|convex|lattice|interior points|"
                 r"intersection|overlap|enclosing|bounding|pick)\b", q):
        return None
    pts = _point_list(q)
    if pts is None or len(pts) < 6:
        return None
    return {"points": pts}


_add("polygon_area", "intermediate",
     name="Polygon from vertices: area, perimeter, orientation", topic="polygons / coordinates",
     purpose="Area (shoelace formula), perimeter and orientation of a simple polygon given its vertices in order.",
     inputs=[_V("points", "vertices in order: x1 y1, x2 y2, ...", n_min=6)],
     outputs=[_O("area", "enclosed area"), _O("perimeter", "sum of the edge lengths"), _O("orientation", "counterclockwise or clockwise", "text")],
     primary="area", conditions=["at least 3 vertices, in order", "the polygon is simple (its edges do not cross)"],
     method="shoelace formula A = |sum(x_i y_{i+1} - x_{i+1} y_i)| / 2; the sign gives the orientation; simplicity checked exactly",
     latex=r"A = \tfrac12\left|\sum_{i} (x_i y_{i+1} - x_{i+1} y_i)\right|",
     run=polygon_area, verify=_verify_polygon_area, parse=parse_polygon_area, reference=_ref_polygon_area,
     wrong=lambda points: (lambda r: dict(r, area=2 * r["area"]))(polygon_area(points)),
     random_inputs=lambda rng: {"points": _flat(_star_polygon(rng))},
     templates=["Find the area of the polygon with vertices (0, 0), (4, 0), (4, 3), (0, 3).",
                "What is the area and perimeter of the quadrilateral with vertices (1, 1), (5, 2), (4, 6), (0, 4)?"],
     examples=[{"inputs": {"points": "0 0, 4 0, 4 3, 0 3"}, "outputs": {"area": 12.0, "perimeter": 14.0, "orientation": "counterclockwise"}},
               {"inputs": {"points": [0, 0, 0, 2, 2, 2, 2, 0]}, "outputs": {"area": 4.0, "orientation": "clockwise"}},
               {"inputs": {"points": "0 0, 4 0, 4 4, 2 1, 0 4"}, "outputs": {"area": 10.0}}],
     questions=[{"q": "Find the area of the polygon with vertices (0, 0), (4, 0), (4, 3), (0, 3).", "outputs": {"area": 12.0, "perimeter": 14.0}},
                {"q": "What is the area of the pentagon with vertices (0, 0), (4, 0), (4, 4), (2, 1), (0, 4)?", "outputs": {"area": 10.0}},
                {"q": "Find the area of the quadrilateral with vertices (0, 0), (2, 2), (2, 0), (0, 2).", "outcome": "INVALID_INPUT"}])


def polygon_centroid(points):
    pts = _pairs(points, "points", 3)
    _simple_polygon(pts)
    n, a, cx, cy = len(pts), 0.0, 0.0, 0.0
    x0, y0 = pts[0]                                      # shifted to the first vertex: no cancellation far from the origin
    q = [(x - x0, y - y0) for x, y in pts]
    for k in range(n):
        (xa, ya), (xb, yb) = q[k], q[(k + 1) % n]
        w = xa * yb - xb * ya
        a += w
        cx += (xa + xb) * w
        cy += (ya + yb) * w
    return {"cx": cx / (3 * a) + x0, "cy": cy / (3 * a) + y0}


def _verify_centroid(i, o):
    """Fan triangulation from the claimed centroid's neighbourhood: first moments about (cx, cy) vanish."""
    pts = _pairs(i["points"], "points", 3)
    cx, cy = o["cx"], o["cy"]
    n = len(pts)
    mx = my = area = 0.0
    scale = max(max(abs(p[0] - cx), abs(p[1] - cy)) for p in pts)
    for k in range(n):
        (xa, ya), (xb, yb) = (pts[k][0] - cx, pts[k][1] - cy), (pts[(k + 1) % n][0] - cx, pts[(k + 1) % n][1] - cy)
        t = (xa * yb - xb * ya) / 2                      # triangle (centroid, a, b): signed area, centroid (a + b)/3
        area += t
        mx += t * (xa + xb) / 3
        my += t * (ya + yb) / 3
    return max(abs(mx), abs(my)) / (abs(area) * max(scale, 1.0))


def parse_polygon_centroid(q0):
    q = _norm(q0)
    if not re.search(r"\b(centroid|cent(?:er|re) of (?:mass|gravity))\b", q) or not re.search(_POLY_WORDS, q) or re.search(r"\btriangle\b", q):
        return None
    pts = _point_list(q)
    return {"points": pts} if pts and len(pts) >= 6 else None


_add("polygon_centroid", "intermediate",
     name="Centroid of a polygon", topic="polygons / coordinates",
     purpose="The centroid (centre of mass of the uniform region) of a simple polygon given its vertices in order.",
     inputs=[_V("points", "vertices in order: x1 y1, x2 y2, ...", n_min=6)],
     outputs=[_O("cx", "x of the centroid"), _O("cy", "y of the centroid")], primary="cx",
     conditions=["at least 3 vertices, in order", "simple polygon"],
     method="area-weighted shoelace sums: Cx = sum((x_i + x_{i+1}) w_i) / (3 * sum w_i), w_i = x_i y_{i+1} - x_{i+1} y_i",
     latex=r"C_x = \frac{1}{6A}\sum_i (x_i + x_{i+1})(x_i y_{i+1} - x_{i+1} y_i)",
     run=polygon_centroid, verify=_verify_centroid, parse=parse_polygon_centroid,
     reference=lambda points: (lambda P: {"cx": P.centroid.x, "cy": P.centroid.y})(
         __import__("shapely.geometry", fromlist=["Polygon"]).Polygon(_pairs(points, "points", 3))),
     wrong=lambda points: (lambda p: {"cx": sum(x for x, _ in p) / len(p), "cy": sum(y for _, y in p) / len(p)})(_pairs(points, "points", 3)),
     random_inputs=lambda rng: {"points": _flat(_star_polygon(rng, 4, 12))},
     templates=["Find the centroid of the polygon with vertices (0, 0), (4, 0), (4, 2), (0, 2).",
                "Where is the centre of mass of the quadrilateral with vertices (0, 0), (6, 0), (6, 6), (0, 3)?"],
     examples=[{"inputs": {"points": "0 0, 4 0, 4 2, 0 2"}, "outputs": {"cx": 2.0, "cy": 1.0}},
               {"inputs": {"points": "0 0, 6 0, 6 6, 0 3"}, "outputs": {"cx": 10 / 3, "cy": 7 / 3}}],
     questions=[{"q": "Find the centroid of the polygon with vertices (0, 0), (4, 0), (4, 2), (0, 2).", "outputs": {"cx": 2.0, "cy": 1.0}},
                {"q": "Where is the centre of mass of the quadrilateral with vertices (0, 0), (6, 0), (6, 6), (0, 3)?", "outputs": {"cx": 10 / 3, "cy": 7 / 3}}])


def _tri_pts(points):
    pts = _pairs(points, "points", 3, 3)
    if _xcross(*pts) == 0:
        raise ValueError("the three points are collinear: they do not form a triangle")
    return pts


def triangle_coordinates(points):
    A, B, C = _tri_pts(points)
    s = _shoelace([A, B, C])
    a, b, c = math.dist(B, C), math.dist(A, C), math.dist(A, B)
    k = abs(s)
    ang = lambda opp, s1, s2: math.degrees(math.atan2(4 * k, s1 * s1 + s2 * s2 - opp * opp))
    return {"area": k, "perimeter": a + b + c, "gx": (A[0] + B[0] + C[0]) / 3, "gy": (A[1] + B[1] + C[1]) / 3,
            "angle_A": ang(a, b, c), "angle_B": ang(b, a, c), "angle_C": ang(c, a, b)}


def _ref_triangle_coords(points):
    import sympy as sp
    P = [sp.Point(_R(x), _R(y)) for x, y in _pairs(points, "points", 3, 3)]
    T = sp.Triangle(*P)
    ang = T.angles
    return {"area": float(abs(T.area)), "perimeter": float(T.perimeter), "gx": float(T.centroid.x), "gy": float(T.centroid.y),
            "angle_A": float(sp.deg(ang[P[0]])), "angle_B": float(sp.deg(ang[P[1]])), "angle_C": float(sp.deg(ang[P[2]]))}


def _verify_tri_coords(i, o):
    A, B, C = _pairs(i["points"], "points", 3, 3)
    a, b, c = math.dist(B, C), math.dist(A, C), math.dist(A, B)
    return _res((o["angle_A"] + o["angle_B"] + o["angle_C"], 180.0), (2 * o["area"], b * c * math.sin(math.radians(o["angle_A"]))),
                (a * math.sin(math.radians(o["angle_B"])), b * math.sin(math.radians(o["angle_A"]))),
                (o["perimeter"], a + b + c), (3 * o["gx"], A[0] + B[0] + C[0]), (3 * o["gy"], A[1] + B[1] + C[1]))


def _rand_tri(rng):
    while True:
        p = [(_RANDP(rng), _RANDP(rng)) for _ in range(3)]
        a, b, c = math.dist(p[1], p[2]), math.dist(p[0], p[2]), math.dist(p[0], p[1])
        if min(a, b, c) > 1 and abs(_shoelace(p)) > 0.05 * max(a, b, c) ** 2:
            return {"points": _flat(p)}


_TRI_CENTRES = r"\b(circum\w*|incent\w*|inradius|incircle|in-circle|inscribed|orthocent\w*|barycentric|inside|contains?|within|nine-point|euler)\b"


def parse_triangle_coords(q0):
    q = _norm(q0)
    if not re.search(r"\btriangle\b", q) or re.search(_TRI_CENTRES + r"|\bspher\w*|\bearth\b", q) or _pts3(q):
        return None
    if not re.search(r"\b(area|perimeter|centroid|angles?)\b", q):
        return None
    p = _pts2(q)
    if not p:
        return None
    return {"points": _flat(p)} if len(p) == 3 else None


_add("triangle_coordinates", "intermediate",
     name="Triangle from vertex coordinates: area, perimeter, centroid, angles", topic="coordinate geometry / triangles",
     purpose="Area, perimeter, centroid and the angles at A, B, C of the triangle with vertices A, B, C.",
     inputs=[_V("points", "the vertices A, B, C: xA yA, xB yB, xC yC", n_min=6, n_max=6)],
     outputs=[_O("area", "area (shoelace)"), _O("perimeter", "perimeter"), _O("gx", "x of the centroid"), _O("gy", "y of the centroid"),
              _O("angle_A", "angle at A, degrees"), _O("angle_B", "angle at B, degrees"), _O("angle_C", "angle at C, degrees")],
     primary="area", conditions=["exactly 3 vertices", "not collinear"],
     method="shoelace area; centroid = mean of the vertices; angles by the law of cosines as atan2(4K, b^2 + c^2 - a^2)",
     latex=r"K = \tfrac12 |(x_B-x_A)(y_C-y_A) - (x_C-x_A)(y_B-y_A)|,\quad G = \tfrac13(A+B+C)",
     run=triangle_coordinates, verify=_verify_tri_coords, parse=parse_triangle_coords, reference=_ref_triangle_coords,
     wrong=lambda points: (lambda r: dict(r, area=2 * r["area"]))(triangle_coordinates(points)),
     random_inputs=_rand_tri,
     templates=["Find the area of the triangle with vertices (0, 0), (4, 0) and (0, 3).",
                "Find the centroid and angles of the triangle with vertices (1, 1), (5, 1) and (3, 4)."],
     examples=[{"inputs": {"points": "0 0, 4 0, 0 3"}, "outputs": {"area": 6.0, "perimeter": 12.0, "gx": 4 / 3, "gy": 1.0, "angle_A": 90.0}},
               {"inputs": {"points": [0, 0, 2, 0, 1, math.sqrt(3)]}, "outputs": {"angle_B": 60.0}}],
     questions=[{"q": "Find the area of the triangle with vertices (0, 0), (4, 0) and (0, 3).", "outputs": {"area": 6.0, "angle_A": 90.0}},
                {"q": "What is the centroid of the triangle with vertices (1, 1), (5, 1) and (3, 4)?", "outputs": {"gx": 3.0, "gy": 2.0}},
                {"q": "Find the area of the triangle with vertices (0, 0), (1, 1) and (2, 2).", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# triangle centres from coordinates (intermediate / advanced)
# ==================================================================================================================
def _snap(v, scale):
    """Rounding residue of an exact zero becomes 0.0 (no -0.0, no 1e-17 noise)."""
    return 0.0 if abs(v) <= 1e-13 * max(1.0, scale) else v + 0.0


def _scale_of(pts):
    return max(max(abs(c) for c in p) for p in pts)


def triangle_circumcircle(points):
    A, B, C = _tri_pts(points)
    bx, by, cx, cy = B[0] - A[0], B[1] - A[1], C[0] - A[0], C[1] - A[1]
    d = 2 * (bx * cy - by * cx)
    ux = (cy * (bx * bx + by * by) - by * (cx * cx + cy * cy)) / d
    uy = (bx * (cx * cx + cy * cy) - cx * (bx * bx + by * by)) / d
    s = _scale_of([A, B, C])
    return {"cx": _snap(A[0] + ux, s), "cy": _snap(A[1] + uy, s), "radius": math.hypot(ux, uy)}


def _verify_circum(i, o):
    pts = _pairs(i["points"], "points", 3, 3)
    return max(abs(math.dist((o["cx"], o["cy"]), p) - o["radius"]) / o["radius"] for p in pts)


def _ref_circum(points):
    import sympy as sp
    T = sp.Triangle(*[sp.Point(_R(x), _R(y)) for x, y in _pairs(points, "points", 3, 3)])
    O = T.circumcenter
    return {"cx": _F(O.x), "cy": _F(O.y), "radius": _F(T.circumradius)}


def parse_circum(q0):
    q = _norm(q0)
    p = _pts2(q)
    tri = re.search(r"\btriangle\b", q) and re.search(r"\b(circumcent\w*|circumradius|circumcircle|circumscribed circle)\b", q)
    thr = re.search(r"\bcircle\b", q) and re.search(r"\b(through|passing through)\b", q) and not re.search(r"\b(line|tangent|intersect\w*|smallest|minimum|enclos\w*)\b", q)
    if not (tri or thr) or _pts3(q):
        return None
    return {"points": _flat(p)} if len(p) == 3 else (None if p else {})


_add("triangle_circumcircle", "intermediate",
     name="Circumcircle: the circle through three points", topic="coordinate geometry / triangle centres",
     purpose="Circumcentre and circumradius of a triangle given by its vertices (the circle through three points).",
     inputs=[_V("points", "the three points: x1 y1, x2 y2, x3 y3", n_min=6, n_max=6)],
     outputs=[_O("cx", "x of the circumcentre"), _O("cy", "y of the circumcentre"), _O("radius", "circumradius")], primary="radius",
     conditions=["exactly 3 points", "not collinear"],
     method="intersection of the perpendicular bisectors, solved in coordinates relative to the first point",
     latex=r"|O - A| = |O - B| = |O - C| = R", run=triangle_circumcircle, verify=_verify_circum, parse=parse_circum, reference=_ref_circum,
     wrong=lambda points: (lambda p: (lambda g: {"cx": g[0], "cy": g[1], "radius": math.dist(g, p[0])})(
         ((p[0][0] + p[1][0] + p[2][0]) / 3, (p[0][1] + p[1][1] + p[2][1]) / 3)))(_pairs(points, "points", 3, 3)),
     random_inputs=_rand_tri,
     templates=["Find the circumcenter and circumradius of the triangle with vertices (0, 0), (4, 0) and (0, 3).",
                "Find the circle through (1, 0), (0, 1) and (-1, 0)."],
     examples=[{"inputs": {"points": "0 0, 4 0, 0 3"}, "outputs": {"cx": 2.0, "cy": 1.5, "radius": 2.5}},
               {"inputs": {"points": "1 0, 0 1, -1 0"}, "outputs": {"cx": 0.0, "cy": 0.0, "radius": 1.0}}],
     questions=[{"q": "Find the circumcenter and circumradius of the triangle with vertices (0, 0), (4, 0) and (0, 3).",
                 "outputs": {"cx": 2.0, "cy": 1.5, "radius": 2.5}},
                {"q": "Find the circle through (1, 0), (0, 1) and (-1, 0).", "outputs": {"cx": 0.0, "cy": 0.0, "radius": 1.0}},
                {"q": "Find the circle passing through (0, 0), (1, 1) and (2, 2).", "outcome": "INVALID_INPUT"}])


def triangle_incircle(points):
    A, B, C = _tri_pts(points)
    a, b, c = math.dist(B, C), math.dist(A, C), math.dist(A, B)
    p = a + b + c
    k = abs(_shoelace([A, B, C]))
    s = _scale_of([A, B, C])
    return {"ix": _snap((a * A[0] + b * B[0] + c * C[0]) / p, s), "iy": _snap((a * A[1] + b * B[1] + c * C[1]) / p, s), "radius": 2 * k / p}


def _verify_incircle(i, o):
    A, B, C = _pairs(i["points"], "points", 3, 3)
    I = (o["ix"], o["iy"])
    sgn = 1 if _shoelace([A, B, C]) > 0 else -1
    worst = 0.0
    for P, Q in ((A, B), (B, C), (C, A)):
        dist = sgn * _cross(P, Q, I) / math.dist(P, Q)       # signed: positive inside
        worst = max(worst, abs(dist - o["radius"]) / o["radius"])
    return worst


def _ref_incircle(points):
    """Intersect two internal angle bisectors (numpy); the radius is the distance from that point to a side."""
    import numpy as np
    A, B, C = (np.array(p, float) for p in _pairs(points, "points", 3, 3))
    unit = lambda v: v / np.linalg.norm(v)
    dA, dB = unit(B - A) + unit(C - A), unit(A - B) + unit(C - B)
    s_ = np.linalg.solve(np.column_stack([dA, -dB]), B - A)
    I = A + s_[0] * dA
    e = C - B
    r = abs(e[0] * (I - B)[1] - e[1] * (I - B)[0]) / np.linalg.norm(e)
    return {"ix": float(I[0]), "iy": float(I[1]), "radius": float(r)}


def parse_incircle(q0):
    q = _norm(q0)
    if not re.search(r"\btriangle\b", q) or not re.search(r"\b(incent\w*|inradius|incircle|in-circle|inscribed circle)\b", q) or _pts3(q):
        return None
    p = _pts2(q)
    return {"points": _flat(p)} if len(p) == 3 else (None if p else {})


_add("triangle_incircle", "intermediate",
     name="Incircle of a triangle: incentre and inradius", topic="coordinate geometry / triangle centres",
     purpose="Incentre and inradius of the triangle with the given vertices.",
     inputs=[_V("points", "the vertices: xA yA, xB yB, xC yC", n_min=6, n_max=6)],
     outputs=[_O("ix", "x of the incentre"), _O("iy", "y of the incentre"), _O("radius", "inradius")], primary="radius",
     conditions=["exactly 3 points", "not collinear"],
     method="incentre = (a A + b B + c C)/(a + b + c) with a, b, c the opposite side lengths; r = 2K/(a + b + c)",
     latex=r"I = \frac{aA + bB + cC}{a+b+c},\quad r = \frac{2K}{a+b+c}",
     run=triangle_incircle, verify=_verify_incircle, parse=parse_incircle, reference=_ref_incircle,
     wrong=lambda points: (lambda r: dict(r, ix=r["ix"] + r["radius"], iy=r["iy"]))(triangle_incircle(points)),
     random_inputs=_rand_tri,
     templates=["Find the incenter and inradius of the triangle with vertices (0, 0), (4, 0) and (0, 3)."],
     examples=[{"inputs": {"points": "0 0, 4 0, 0 3"}, "outputs": {"ix": 1.0, "iy": 1.0, "radius": 1.0}},
               {"inputs": {"points": "0 0, 6 0, 0 8"}, "outputs": {"ix": 2.0, "iy": 2.0, "radius": 2.0}}],
     questions=[{"q": "Find the incenter and inradius of the triangle with vertices (0, 0), (4, 0) and (0, 3).",
                 "outputs": {"ix": 1.0, "iy": 1.0, "radius": 1.0}},
                {"q": "What is the radius of the inscribed circle of the triangle with vertices (0, 0), (6, 0) and (0, 8)?", "outputs": {"radius": 2.0}}])


def triangle_orthocenter(points):
    A, B, C = _tri_pts(points)
    bx, by, cx, cy = B[0] - A[0], B[1] - A[1], C[0] - A[0], C[1] - A[1]
    # h = H - A: h . (C - B) = 0 and h . c = b . c
    m11, m12, m21, m22 = cx - bx, cy - by, cx, cy
    r2 = bx * cx + by * cy
    det = m11 * m22 - m12 * m21
    hx, hy = (-m12 * r2) / det, (m11 * r2) / det
    s = _scale_of([A, B, C])
    return {"hx": _snap(A[0] + hx, s), "hy": _snap(A[1] + hy, s)}


def _verify_ortho(i, o):
    A, B, C = _pairs(i["points"], "points", 3, 3)
    H = (o["hx"], o["hy"])
    L = max(math.dist(A, B), math.dist(B, C), math.dist(A, C))
    worst = 0.0
    for P, Q, R_ in ((A, B, C), (B, C, A), (C, A, B)):
        d = (H[0] - P[0]) * (R_[0] - Q[0]) + (H[1] - P[1]) * (R_[1] - Q[1])
        worst = max(worst, abs(d) / (L * max(L, math.dist(H, P))))
    return worst


def _ref_ortho(points):
    import sympy as sp
    T = sp.Triangle(*[sp.Point(_R(x), _R(y)) for x, y in _pairs(points, "points", 3, 3)])
    return {"hx": _F(T.orthocenter.x), "hy": _F(T.orthocenter.y)}


def parse_ortho(q0):
    q = _norm(q0)
    if not re.search(r"\btriangle\b", q) or not re.search(r"\borthocent(?:er|re)\b|altitudes meet|altitudes intersect", q) or _pts3(q):
        return None
    p = _pts2(q)
    return {"points": _flat(p)} if len(p) == 3 else (None if p else {})


_add("triangle_orthocenter", "intermediate",
     name="Orthocentre of a triangle", topic="coordinate geometry / triangle centres",
     purpose="The orthocentre (common point of the three altitudes) of the triangle with the given vertices.",
     inputs=[_V("points", "the vertices: xA yA, xB yB, xC yC", n_min=6, n_max=6)],
     outputs=[_O("hx", "x of the orthocentre"), _O("hy", "y of the orthocentre")], primary="hx",
     conditions=["exactly 3 points", "not collinear"],
     method="solve (H - A).(C - B) = 0 and (H - B).(C - A) = 0 (two altitudes) by Cramer's rule",
     latex=r"(H-A)\cdot(C-B) = 0,\quad (H-B)\cdot(C-A) = 0",
     run=triangle_orthocenter, verify=_verify_ortho, parse=parse_ortho, reference=_ref_ortho,
     wrong=lambda points: (lambda c: {"hx": c["cx"], "hy": c["cy"]})(triangle_circumcircle(points)),
     random_inputs=_rand_tri,
     templates=["Find the orthocenter of the triangle with vertices (0, 0), (4, 0) and (1, 3)."],
     examples=[{"inputs": {"points": "0 0, 4 0, 1 3"}, "outputs": {"hx": 1.0, "hy": 1.0}},
               {"inputs": {"points": "0 0, 4 0, 0 3"}, "outputs": {"hx": 0.0, "hy": 0.0}}],
     questions=[{"q": "Find the orthocenter of the triangle with vertices (0, 0), (4, 0) and (1, 3).", "outputs": {"hx": 1.0, "hy": 1.0}},
                {"q": "Where do the altitudes meet in the triangle with vertices (0, 0), (4, 0) and (0, 3)?", "outputs": {"hx": 0.0, "hy": 0.0}}])


# ==================================================================================================================
# lines (intermediate)
# ==================================================================================================================
def _line_inputs(k):
    return [_I(f"a{k}", f"coefficient of x in line {k} (a{k} x + b{k} y + c{k} = 0)"), _I(f"b{k}", f"coefficient of y in line {k}"),
            _I(f"c{k}", f"constant of line {k}")]


def _rand_line(rng, k):
    a, b = rng.randint(-9, 9), rng.randint(-9, 9)
    while a == 0 and b == 0:
        a, b = rng.randint(-9, 9), rng.randint(-9, 9)
    return {f"a{k}": float(a), f"b{k}": float(b), f"c{k}": float(rng.randint(-30, 30))}


def _two_lines_parse(q):
    ls = _lines(q)
    if len(ls) != 2:
        return None if len(ls) > 2 else {}
    (a1, b1, c1), (a2, b2, c2) = ls
    return {"a1": a1, "b1": b1, "c1": c1, "a2": a2, "b2": b2, "c2": c2}


def line_intersection(a1, b1, c1, a2, b2, c2):
    a1, b1, c1 = _line_check(a1, b1, c1, "line 1")
    a2, b2, c2 = _line_check(a2, b2, c2, "line 2")
    if _fr(a1) * _fr(b2) - _fr(a2) * _fr(b1) == 0:
        raise ValueError("the lines are parallel (or the same line): there is no single intersection point")
    det = a1 * b2 - a2 * b1
    s = max(abs(c1), abs(c2), 1.0)
    return {"x": _snap((b1 * c2 - b2 * c1) / det, 0.0), "y": _snap((c1 * a2 - c2 * a1) / det, 0.0)}


def _lres(a, b, c, x, y):
    return abs(a * x + b * y + c) / max(abs(a * x) + abs(b * y) + abs(c), 1e-300)


def _ref_line_int(a1, b1, c1, a2, b2, c2):
    P = _ref_line(a1, b1, c1).intersection(_ref_line(a2, b2, c2))[0]
    return {"x": float(P.x), "y": float(P.y)}


def parse_line_int(q0):
    q = _norm(q0)
    if not re.search(r"\b(intersect\w*|meet|cross)\b", q) or not re.search(r"\blines?\b", q) or \
            re.search(r"\b(circle|segment|parabola|plane|ellipse|hyperbola|curve)\b|\^2", q):
        return None
    return _two_lines_parse(q)


_add("line_intersection", "intermediate",
     name="Intersection point of two lines", topic="coordinate geometry / lines",
     purpose="The point where two lines a1 x + b1 y + c1 = 0 and a2 x + b2 y + c2 = 0 meet.",
     inputs=_line_inputs(1) + _line_inputs(2), outputs=[_O("x", "x of the intersection"), _O("y", "y of the intersection")], primary="x",
     conditions=["each line has a or b nonzero", "the lines are not parallel (exact test on the written coefficients)"],
     method="Cramer's rule: x = (b1 c2 - b2 c1)/D, y = (c1 a2 - c2 a1)/D, D = a1 b2 - a2 b1",
     latex=r"x = \frac{b_1c_2 - b_2c_1}{a_1b_2 - a_2b_1},\quad y = \frac{c_1a_2 - c_2a_1}{a_1b_2 - a_2b_1}",
     run=line_intersection,
     verify=lambda i, o: max(_lres(i["a1"], i["b1"], i["c1"], o["x"], o["y"]), _lres(i["a2"], i["b2"], i["c2"], o["x"], o["y"])),
     parse=parse_line_int, reference=_ref_line_int,
     wrong=lambda a1, b1, c1, a2, b2, c2: {"x": (b2 * c1 - b1 * c2) / (a1 * b2 - a2 * b1), "y": (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1)},
     random_inputs=lambda rng: {**_rand_line(rng, 1), **_rand_line(rng, 2)},
     templates=["Find the intersection of the lines 2x + y = 5 and x - y = 1.", "Where do the lines y = 2x + 1 and y = -x + 7 meet?"],
     examples=[{"inputs": {"a1": 2, "b1": 1, "c1": -5, "a2": 1, "b2": -1, "c2": -1}, "outputs": {"x": 2.0, "y": 1.0}},
               {"inputs": {"a1": 2, "b1": -1, "c1": 1, "a2": 1, "b2": 1, "c2": -7}, "outputs": {"x": 2.0, "y": 5.0}}],
     questions=[{"q": "Find the intersection of the lines 2x + y = 5 and x - y = 1.", "outputs": {"x": 2.0, "y": 1.0}},
                {"q": "Where do the lines y = 2x + 1 and y = -x + 7 meet?", "outputs": {"x": 2.0, "y": 5.0}},
                {"q": "Find the intersection of the lines y = 2x + 1 and y = 2x + 3.", "outcome": "INVALID_INPUT"}])


def angle_between_lines(a1, b1, c1, a2, b2, c2):
    a1, b1, c1 = _line_check(a1, b1, c1, "line 1")
    a2, b2, c2 = _line_check(a2, b2, c2, "line 2")
    cr, dt = a1 * b2 - a2 * b1, a1 * a2 + b1 * b2           # directions (b, -a): cross and dot
    return {"angle": math.degrees(math.atan2(abs(cr), abs(dt)))}


def _verify_angle_lines(i, o):
    n1, n2 = math.hypot(i["a1"], i["b1"]), math.hypot(i["a2"], i["b2"])
    cr = abs(i["a1"] * i["b2"] - i["a2"] * i["b1"]) / (n1 * n2)
    dt = abs(i["a1"] * i["a2"] + i["b1"] * i["b2"]) / (n1 * n2)
    t = math.radians(o["angle"])
    return max(abs(math.sin(t) - cr), abs(math.cos(t) - dt), 0.0 if 0 <= o["angle"] <= 90 else 1.0)


def _ref_angle_lines(a1, b1, c1, a2, b2, c2):
    import sympy as sp
    t = _ref_line(a1, b1, c1).angle_between(_ref_line(a2, b2, c2))
    return {"angle": float(sp.deg(sp.Min(t, sp.pi - t)))}


def parse_angle_lines(q0):
    q = _norm(q0)
    if not re.search(r"\bangle between\b", q) or not re.search(r"\blines?\b", q) or re.search(r"\b(vectors?|planes?|curves?)\b|\^2", q):
        return None
    return _two_lines_parse(q)


_add("angle_between_lines", "intermediate",
     name="Angle between two lines", topic="coordinate geometry / lines",
     purpose="The acute (or right) angle between two lines given by equations.",
     inputs=_line_inputs(1) + _line_inputs(2), outputs=[_O("angle", "angle between the lines, degrees, 0 to 90")], primary="angle",
     conditions=["each line has a or b nonzero", "the acute angle is reported (parallel lines: 0)"],
     method="directions (b, -a); angle = atan2(|cross|, |dot|)", latex=r"\tan\theta = \left|\frac{a_1b_2 - a_2b_1}{a_1a_2 + b_1b_2}\right|",
     run=angle_between_lines, verify=_verify_angle_lines, parse=parse_angle_lines, reference=_ref_angle_lines,
     wrong=lambda a1, b1, c1, a2, b2, c2: {"angle": 180.0 - angle_between_lines(a1, b1, c1, a2, b2, c2)["angle"]},
     random_inputs=lambda rng: {**_rand_line(rng, 1), **_rand_line(rng, 2)},
     templates=["Find the angle between the lines y = x + 1 and y = -x + 3.", "What is the angle between the lines y = 2x + 1 and y = 3x - 2?"],
     examples=[{"inputs": {"a1": 1, "b1": -1, "c1": 1, "a2": 1, "b2": 1, "c2": -3}, "outputs": {"angle": 90.0}},
               {"inputs": {"a1": 2, "b1": -1, "c1": 1, "a2": 3, "b2": -1, "c2": -2}, "outputs": {"angle": 8.13010235415598}}],
     questions=[{"q": "Find the angle between the lines y = x + 1 and y = -x + 3.", "outputs": {"angle": 90.0}},
                {"q": "What is the angle between the lines y = 2x + 1 and y = 3x - 2?", "outputs": {"angle": 8.13010235415598}},
                {"q": "Find the angle between the lines x = 0 and y = 0.", "outputs": {"angle": 90.0}}])


def reflect_point_line(px, py, a, b, c):
    px, py = _fin(px, "px"), _fin(py, "py")
    a, b, c = _line_check(a, b, c)
    k = (a * px + b * py + c) / (a * a + b * b)
    s = max(abs(px), abs(py))
    return {"x": _snap(px - 2 * a * k, s), "y": _snap(py - 2 * b * k, s)}


def _verify_reflect(i, o):
    a, b, c = i["a"], i["b"], i["c"]
    mx, my = (i["px"] + o["x"]) / 2, (i["py"] + o["y"]) / 2
    dx, dy = o["x"] - i["px"], o["y"] - i["py"]
    n = math.hypot(a, b)
    sc = max(1.0, abs(i["px"]), abs(i["py"]), abs(o["x"]), abs(o["y"]))
    return max(abs(a * mx + b * my + c) / (n * sc), abs(dx * b - dy * a) / (n * sc))


def _ref_reflect(px, py, a, b, c):
    import sympy as sp
    P = sp.Point(_R(px), _R(py)).reflect(_ref_line(a, b, c))
    return {"x": float(P.x), "y": float(P.y)}


def parse_reflect(q0):
    q = _norm(q0)
    if not re.search(r"\b(reflect\w*|mirror image|image of)\b", q) or re.search(r"\b(rotat\w*|dilat\w*|scal\w*|translat\w*|plane|circle)\b|\^2", q):
        return None
    p, ls = _pts2(q), _lines(q)
    if len(p) > 1 or len(ls) > 1 or _pts3(q):
        return None
    d = {}
    if p:
        d.update(px=p[0][0], py=p[0][1])
    if ls:
        d.update(a=ls[0][0], b=ls[0][1], c=ls[0][2])
    return d


_add("reflect_point_line", "intermediate",
     name="Reflection of a point in a line", topic="transformations / reflections",
     purpose="The mirror image of a point in the line a x + b y + c = 0 (including the axes and y = x).",
     inputs=[_I("px", "x of the point"), _I("py", "y of the point"), _I("a", "coefficient of x of the line"),
             _I("b", "coefficient of y of the line"), _I("c", "constant of the line")],
     outputs=[_O("x", "x of the image"), _O("y", "y of the image")], primary="x", conditions=["a and b not both 0"],
     method="P' = P - 2 (a px + b py + c)/(a^2 + b^2) (a, b)", latex=r"P' = P - 2\,\frac{a p_x + b p_y + c}{a^2+b^2}\,(a, b)",
     run=reflect_point_line, verify=_verify_reflect, parse=parse_reflect, reference=_ref_reflect,
     wrong=lambda px, py, a, b, c: (lambda k: {"x": px - a * k, "y": py - b * k})((a * px + b * py + c) / (a * a + b * b)),
     random_inputs=lambda rng: {"px": _RANDP(rng), "py": _RANDP(rng), **{k[0]: v for k, v in _rand_line(rng, 1).items()}},
     templates=["Reflect the point (3, 4) in the line y = x.", "Find the reflection of (1, 2) across the line x + y - 5 = 0.",
                "Find the mirror image of (2, -3) in the x-axis."],
     examples=[{"inputs": {"px": 3, "py": 4, "a": 1, "b": -1, "c": 0}, "outputs": {"x": 4.0, "y": 3.0}},
               {"inputs": {"px": 1, "py": 2, "a": 1, "b": 1, "c": -5}, "outputs": {"x": 3.0, "y": 4.0}}],
     questions=[{"q": "Reflect the point (3, 4) in the line y = x.", "outputs": {"x": 4.0, "y": 3.0}},
                {"q": "Find the reflection of (1, 2) across the line x + y - 5 = 0.", "outputs": {"x": 3.0, "y": 4.0}},
                {"q": "Find the mirror image of (2, -3) in the x-axis.", "outputs": {"x": 2.0, "y": 3.0}},
                {"q": "Reflect the point (3, 4) in the line.", "outcome": "NEEDS_INPUT"}])


def _centre_of(q):
    m = re.search(r"\b(?:about|around|cent(?:er|re)(?: of (?:rotation|dilation|enlargement))?|from)\s*(?:the\s*)?(?:point\s*)?" + _P2, q)
    return (float(m.group(1)), float(m.group(2)), m.start()) if m else None


def rotate_point(px, py, angle, cx=0.0, cy=0.0):
    px, py, t = _fin(px, "px"), _fin(py, "py"), _fin(angle, "angle")
    cx, cy = _fin(cx, "cx"), _fin(cy, "cy")
    r = t % 360.0
    exact = {0.0: (1.0, 0.0), 90.0: (0.0, 1.0), 180.0: (-1.0, 0.0), 270.0: (0.0, -1.0)}
    co, si = exact.get(r, (math.cos(math.radians(t)), math.sin(math.radians(t))))
    dx, dy = px - cx, py - cy
    s = max(abs(px), abs(py), abs(cx), abs(cy))
    return {"x": _snap(cx + dx * co - dy * si, s), "y": _snap(cy + dx * si + dy * co, s)}


def _verify_rotate(i, o):
    cx, cy = i.get("cx", 0.0), i.get("cy", 0.0)
    z = complex(i["px"] - cx, i["py"] - cy)
    w = complex(o["x"] - cx, o["y"] - cy)
    t = math.radians(i["angle"])
    return abs(w - z * complex(math.cos(t), math.sin(t))) / max(abs(z), 1.0, abs(cx), abs(cy))


def _ref_rotate(px, py, angle, cx=0.0, cy=0.0):
    from shapely import affinity
    from shapely.geometry import Point
    P = affinity.rotate(Point(px, py), angle, origin=(cx, cy))
    return {"x": P.x, "y": P.y}


def parse_rotate(q0):
    q = _norm(q0)
    if not re.search(r"\brotat\w*\b", q) or re.search(r"\b(polygon|triangle|square|axis|vector|plane|3d)\b", q) or _pts3(q):
        return None
    c = _centre_of(q)
    pts = [(x, y) for x, y in _pts2(q)]
    if c:
        pts = [p for p in pts if p != (c[0], c[1])] or pts[:0]
    if len(pts) > 1:
        return None
    t = _angle_in(q, r"angle|by|through")
    if t is not None and re.search(r"(?<!counter)(?<!counter-)(?<!anti)(?<!anti-)\bclockwise\b", q):
        t = -t
    d = {"angle": t}
    if pts:
        d.update(px=pts[0][0], py=pts[0][1])
    if c:
        d.update(cx=c[0], cy=c[1])
    return _clean(d)


_add("rotate_point", "intermediate",
     name="Rotation of a point", topic="transformations / rotations",
     purpose="The image of a point rotated by an angle about a centre (the origin unless given); counterclockwise is positive.",
     inputs=[_I("px", "x of the point"), _I("py", "y of the point"), _I("angle", "rotation angle in degrees, counterclockwise positive"),
             _I("cx", "x of the centre (default 0)", required=False), _I("cy", "y of the centre (default 0)", required=False)],
     outputs=[_O("x", "x of the image"), _O("y", "y of the image")], primary="x",
     conditions=["angle in degrees, counterclockwise positive ('clockwise' in a question makes it negative)", "centre defaults to the origin"],
     method="x' = cx + (x - cx) cos t - (y - cy) sin t, y' = cy + (x - cx) sin t + (y - cy) cos t (exact for multiples of 90 degrees)",
     latex=r"\begin{pmatrix}x'\\y'\end{pmatrix} = \begin{pmatrix}\cos\theta & -\sin\theta\\ \sin\theta & \cos\theta\end{pmatrix}\begin{pmatrix}x-c_x\\y-c_y\end{pmatrix} + \begin{pmatrix}c_x\\c_y\end{pmatrix}",
     run=rotate_point, verify=_verify_rotate, parse=parse_rotate, reference=_ref_rotate,
     wrong=lambda px, py, angle, cx=0.0, cy=0.0: rotate_point(px, py, -angle, cx, cy),
     random_inputs=lambda rng: {"px": _RANDP(rng), "py": _RANDP(rng), "angle": rng.choice([round(rng.uniform(-350, 350), 1), float(rng.randint(1, 359))]),
                                "cx": _RANDP(rng), "cy": _RANDP(rng)},
     templates=["Rotate the point (3, 4) by 90 degrees about the origin.", "Rotate (5, 1) by 30 degrees clockwise about (1, 1)."],
     examples=[{"inputs": {"px": 3, "py": 4, "angle": 90}, "outputs": {"x": -4.0, "y": 3.0}},
               {"inputs": {"px": 5, "py": 1, "angle": -30, "cx": 1, "cy": 1}, "outputs": {"x": 1 + 2 * math.sqrt(3), "y": -1.0}}],
     questions=[{"q": "Rotate the point (3, 4) by 90 degrees about the origin.", "outputs": {"x": -4.0, "y": 3.0}},
                {"q": "Rotate (5, 1) by 30 degrees clockwise about (1, 1).", "outputs": {"x": 1 + 2 * math.sqrt(3), "y": -1.0}},
                {"q": "Rotate the point (2, 0) by 180 degrees.", "outputs": {"x": -2.0, "y": 0.0}},
                {"q": "Rotate the point (2, 0) about the origin.", "outcome": "NEEDS_INPUT"}])


def dilate_point(px, py, k, cx=0.0, cy=0.0):
    px, py, k = _fin(px, "px"), _fin(py, "py"), _fin(k, "k")
    cx, cy = _fin(cx, "cx"), _fin(cy, "cy")
    if k == 0:
        raise ValueError("the scale factor must not be 0")
    return {"x": cx + k * (px - cx) + 0.0, "y": cy + k * (py - cy) + 0.0}


def _ref_dilate(px, py, k, cx=0.0, cy=0.0):
    from shapely import affinity
    from shapely.geometry import Point
    P = affinity.scale(Point(px, py), k, k, origin=(cx, cy))
    return {"x": P.x, "y": P.y}


def _verify_dilate(i, o):
    cx, cy = i.get("cx", 0.0), i.get("cy", 0.0)
    sc = max(1.0, abs(i["px"] - cx), abs(i["py"] - cy)) * max(1.0, abs(i["k"])) + abs(cx) + abs(cy)
    return max(abs((o["x"] - cx) - i["k"] * (i["px"] - cx)), abs((o["y"] - cy) - i["k"] * (i["py"] - cy))) / sc


def parse_dilate(q0):
    q = _norm(q0)
    if not re.search(r"\b(dilat\w*|enlarg\w*|scal(?:e|ed|ing)|homothety)\b", q) or re.search(r"\b(polygon|triangle|area|volume|rotat\w*|map)\b", q) or _pts3(q):
        return None
    c = _centre_of(q)
    pts = _pts2(q)
    if c:
        pts = [p for p in pts if p != (c[0], c[1])]
    if len(pts) > 1:
        return None
    m = re.search(r"\b(?:scale factor|factor|ratio)\b\s*(?:of|=|is)?\s*(" + _N + r"(?:/\d+)?)", q)
    d = {"k": float(Fraction(m.group(1))) if m else None}
    if pts:
        d.update(px=pts[0][0], py=pts[0][1])
    if c:
        d.update(cx=c[0], cy=c[1])
    return _clean(d)


_add("dilate_point", "beginner",
     name="Dilation (enlargement) of a point", topic="transformations / dilations",
     purpose="The image of a point under a dilation with scale factor k about a centre (the origin unless given).",
     inputs=[_I("px", "x of the point"), _I("py", "y of the point"), _I("k", "scale factor (negative: through the centre)"),
             _I("cx", "x of the centre (default 0)", required=False), _I("cy", "y of the centre (default 0)", required=False)],
     outputs=[_O("x", "x of the image"), _O("y", "y of the image")], primary="x", conditions=["k != 0", "centre defaults to the origin"],
     method="P' = C + k (P - C)", latex=r"P' = C + k\,(P - C)",
     run=dilate_point, verify=_verify_dilate, parse=parse_dilate, reference=_ref_dilate,
     wrong=lambda px, py, k, cx=0.0, cy=0.0: {"x": k * px, "y": k * py},
     random_inputs=lambda rng: {"px": _RANDP(rng), "py": _RANDP(rng), "k": rng.choice([round(rng.uniform(-4, 4), 2), float(rng.randint(2, 5))]) or 2.0,
                                "cx": float(rng.randint(1, 20)) * rng.choice([1, -1]), "cy": _RANDP(rng)},
     templates=["Dilate the point (2, 3) by a scale factor of 3 about the origin.",
                "Find the image of (6, 4) under an enlargement with scale factor 1/2 and centre (2, 2)."],
     examples=[{"inputs": {"px": 2, "py": 3, "k": 3}, "outputs": {"x": 6.0, "y": 9.0}},
               {"inputs": {"px": 6, "py": 4, "k": 0.5, "cx": 2, "cy": 2}, "outputs": {"x": 4.0, "y": 3.0}}],
     questions=[{"q": "Dilate the point (2, 3) by a scale factor of 3 about the origin.", "outputs": {"x": 6.0, "y": 9.0}},
                {"q": "Find the image of (6, 4) under an enlargement with scale factor 1/2 and centre (2, 2).", "outputs": {"x": 4.0, "y": 3.0}},
                {"q": "Dilate the point (2, 3) by a scale factor of 0.", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# circles and lines (intermediate / advanced)
# ==================================================================================================================
def _equations_pos(q):
    out = []
    for m in re.finditer(_EQ_AT_WORD_START + r"(" + _EQ_CHARS + r"+)=(" + _EQ_CHARS + r"+)", q):
        lhs, rhs = m.group(1).strip(), m.group(2).strip()
        rhs = re.split(r"\s+(?=[a-z]{2})", rhs)[0].rstrip(". ")
        lhs = re.split(r"(?<=[a-z]{2})\s+", lhs)[-1]
        if not re.search(r"[xy]", lhs + rhs):
            continue
        e = _equation(lhs.strip() + "=" + rhs.strip())
        if e is not None:
            out.append((m.start(), e))
    return out


def _circles(q):
    """Circles written as 'centre (h, k) and radius r', '(x - h)^2 + (y - k)^2 = R' or 'x^2 + y^2 + Dx + Ey + F = 0',
    in the order written: [(h, k, r)]. A written equation with no real circle gives r = None."""
    found = []
    for m in re.finditer(r"cent(?:er|re)\s*(?:at\s*)?" + _P2 + r"\s*(?:,|and|with)?\s*(?:a\s*)?radius\s*(?:of|=|is)?\s*(" + _N + r")", q):
        found.append((m.start(), float(m.group(1)), float(m.group(2)), float(m.group(3))))
    for m in re.finditer(r"radius\s*(?:of|=|is)?\s*(" + _N + r")\s*(?:,|and|with)?\s*(?:a\s*)?cent(?:er|re)\s*(?:at\s*)?" + _P2, q):
        found.append((m.start(), float(m.group(2)), float(m.group(3)), float(m.group(1))))
    sq = r"(?:\(\s*{v}\s*(?:([-+])\s*(" + _N + r"))?\s*\)|{v})\s*\^\s*2"
    for m in re.finditer(sq.format(v="x") + r"\s*\+\s*" + sq.format(v="y") + r"\s*=\s*(" + _N + r")", q):
        if "(" not in m.group(0):
            continue                                          # plain x^2 + y^2 = R: read as an equation below
        h = (-1 if m.group(1) == "+" else 1) * float(m.group(2)) if m.group(2) else 0.0
        k = (-1 if m.group(3) == "+" else 1) * float(m.group(4)) if m.group(4) else 0.0
        R2 = float(m.group(5))
        found.append((m.start(), h, k, math.sqrt(R2) if R2 > 0 else None))
    for pos, e in _equations_pos(q):
        if e["xx"] != 0 and e["xx"] == e["yy"] and e["xy"] == 0:
            D, E, F = e["x"] / e["xx"], e["y"] / e["xx"], e["1"] / e["xx"]
            R2 = D * D / 4 + E * E / 4 - F
            found.append((pos, -D / 2 + 0.0, -E / 2 + 0.0, math.sqrt(R2) if R2 > 0 else None))
    return [(h, k, r) for _p, h, k, r in sorted(found)]


def _two_pts_out(pts):
    pts = _sort_pts(pts)
    out = {"count": len(pts)}
    for n in (1, 2):
        p = pts[n - 1] if len(pts) >= n else (None, None)
        out[f"x{n}"], out[f"y{n}"] = p
    return out


def circle_line_intersection(cx, cy, r, a, b, c):
    cx, cy, r = _fin(cx, "cx"), _fin(cy, "cy"), _pos(r, "r")
    a, b, c = _line_check(a, b, c)
    A, B, Cc, X, Y, Rr = (_fr(v) for v in (a, b, c, cx, cy, r))
    num = (A * X + B * Y + Cc) ** 2
    n2 = A * A + B * B
    gap = Rr * Rr * n2 - num                                  # sign of r^2 - distance^2, exactly
    if gap < 0:
        return _two_pts_out([])
    k = (a * cx + b * cy + c) / (a * a + b * b)
    fx, fy = cx - a * k, cy - b * k
    if gap == 0:
        return _two_pts_out([(fx, fy)])
    h = math.sqrt(float(gap / n2)) / math.sqrt(a * a + b * b)  # half-chord along the unit direction (-b, a)/|n|
    return _two_pts_out([(fx - b * h, fy + a * h), (fx + b * h, fy - a * h)])


def _verify_circle_line(i, o):
    cx, cy, r, a, b, c = (i[k] for k in ("cx", "cy", "r", "a", "b", "c"))
    d = abs(a * cx + b * cy + c) / math.hypot(a, b)
    pts = [(o[f"x{n}"], o[f"y{n}"]) for n in (1, 2) if o[f"x{n}"] is not None]
    if len(pts) != o["count"] or any(o[f"x{n}"] is None for n in range(1, o["count"] + 1)):
        return 1.0
    if o["count"] == 0:
        return 0.0 if d > r * (1 - 1e-12) else 1.0
    worst = max(max(abs(math.dist(p, (cx, cy)) - r) / r, abs(a * p[0] + b * p[1] + c) / (math.hypot(a, b) * max(1.0, abs(p[0]), abs(p[1]), r)))
                for p in pts)
    if o["count"] == 1:
        worst = max(worst, abs(d - r) / r)
    elif math.dist(*pts) == 0:
        return 1.0
    return worst


def _ref_circle_line(cx, cy, r, a, b, c):
    import sympy as sp
    pts = sp.Circle(sp.Point(_R(cx), _R(cy)), _R(r)).intersection(_ref_line(a, b, c))
    return _two_pts_out([(float(p.x), float(p.y)) for p in pts])


def _rand_circle_line(rng):
    cx, cy, r = _RANDP(rng) / 2, _RANDP(rng) / 2, _rand_len(rng, 1, 30)
    a, b = float(rng.randint(-9, 9)), float(rng.randint(1, 9)) * rng.choice([1, -1])
    if rng.random() < 0.8:
        px, py = cx + rng.uniform(-0.9, 0.9) * r * 0.7, cy + rng.uniform(-0.9, 0.9) * r * 0.7
        c = float(round(-(a * px + b * py)))
    else:
        c = float(rng.randint(-60, 60))
    return {"cx": cx, "cy": cy, "r": r, "a": a, "b": b, "c": c}


def parse_circle_line(q0):
    q = _norm(q0)
    if not re.search(r"\bcircle\b", q) or not re.search(r"\bline\b", q) or not re.search(r"\b(intersect\w*|meet|cross\w*|common points?)\b", q):
        return None
    if re.search(r"\b(tangent|segment)\b", q):
        return None
    cs, ls = _circles(q), _lines(q)
    if len(cs) > 1 or len(ls) > 1:
        return None
    d = {}
    if cs:
        if cs[0][2] is None:
            return {"_out_of_scope": "the equation has no real circle (its radius squared is not positive)"}
        d.update(cx=cs[0][0], cy=cs[0][1], r=cs[0][2])
    if ls:
        d.update(a=ls[0][0], b=ls[0][1], c=ls[0][2])
    return d


_PT_OUT = [_O("count", "number of intersection points (0, 1 or 2)"), _O("x1", "x of the first point (ordered by x, then y)"),
           _O("y1", "y of the first point"), _O("x2", "x of the second point"), _O("y2", "y of the second point")]

_add("circle_line_intersection", "intermediate",
     name="Intersection of a circle and a line", topic="coordinate geometry / circles",
     purpose="The points where the line a x + b y + c = 0 meets the circle with centre (cx, cy) and radius r (0, 1 or 2 points).",
     inputs=[_I("cx", "x of the centre"), _I("cy", "y of the centre"), _I("r", "radius"), _I("a", "coefficient of x of the line"),
             _I("b", "coefficient of y of the line"), _I("c", "constant of the line")],
     outputs=_PT_OUT, primary="count",
     conditions=["r > 0", "a and b not both 0", "tangency is decided exactly on the written values", "points ordered by x, then y; absent points are empty"],
     method="foot of the perpendicular from the centre, then the half-chord sqrt(r^2 - d^2) along the line",
     latex=r"d = \frac{|a c_x + b c_y + c|}{\sqrt{a^2+b^2}},\quad P = F \pm \sqrt{r^2-d^2}\,\frac{(-b, a)}{\sqrt{a^2+b^2}}",
     run=circle_line_intersection, verify=_verify_circle_line, parse=parse_circle_line, reference=_ref_circle_line,
     wrong=lambda cx, cy, r, a, b, c: circle_line_intersection(cx, cy, r, a, b, -c) if c else circle_line_intersection(cx, cy, r, a, b, r * 3 * math.hypot(a, b) + 1),
     random_inputs=_rand_circle_line,
     templates=["Where does the line y = x + 1 intersect the circle x^2 + y^2 = 25?",
                "Find the intersection of the line 3x + 4y - 25 = 0 and the circle with center (0, 0) and radius 5."],
     examples=[{"inputs": {"cx": 0, "cy": 0, "r": 5, "a": 1, "b": -1, "c": 1}, "outputs": {"count": 2, "x1": -4.0, "y1": -3.0, "x2": 3.0, "y2": 4.0}},
               {"inputs": {"cx": 0, "cy": 0, "r": 5, "a": 3, "b": 4, "c": -25}, "outputs": {"count": 1, "x1": 3.0, "y1": 4.0, "x2": None}},
               {"inputs": {"cx": 0, "cy": 0, "r": 1, "a": 0, "b": 1, "c": -2}, "outputs": {"count": 0}}],
     questions=[{"q": "Where does the line y = x + 1 intersect the circle x^2 + y^2 = 25?", "outputs": {"count": 2, "x1": -4.0, "y1": -3.0, "x2": 3.0, "y2": 4.0}},
                {"q": "Find the intersection of the line 3x + 4y - 25 = 0 and the circle with center (0, 0) and radius 5.", "outputs": {"count": 1, "x1": 3.0, "y1": 4.0}},
                {"q": "Does the line y = 2 meet the circle (x - 1)^2 + (y + 1)^2 = 4?", "outputs": {"count": 0}},
                {"q": "Where does the line y = x + 1 intersect the circle x^2 + y^2 = -4?", "outcome": "OUT_OF_SCOPE"}])


def circle_circle_intersection(x1, y1, r1, x2, y2, r2):
    x1, y1, r1, x2, y2, r2 = _fin(x1, "x1"), _fin(y1, "y1"), _pos(r1, "r1"), _fin(x2, "x2"), _fin(y2, "y2"), _pos(r2, "r2")
    X1, Y1, R1, X2, Y2, R2 = (_fr(v) for v in (x1, y1, r1, x2, y2, r2))
    d2 = (X2 - X1) ** 2 + (Y2 - Y1) ** 2
    if d2 == 0:
        if R1 == R2:
            raise ValueError("the circles coincide: they share infinitely many points")
        return _two_pts_out([])
    if d2 > (R1 + R2) ** 2 or d2 < (R1 - R2) ** 2:
        return _two_pts_out([])
    a_ = (d2 + R1 * R1 - R2 * R2) / 2                         # = a * d
    h2 = R1 * R1 - a_ * a_ / d2                               # exact
    d = math.sqrt(float(d2))
    ux, uy = (x2 - x1) / d, (y2 - y1) / d
    a = float(a_) / d
    fx, fy = x1 + a * ux, y1 + a * uy
    if h2 == 0:
        return _two_pts_out([(fx, fy)])
    h = math.sqrt(float(h2))
    return _two_pts_out([(fx - h * uy, fy + h * ux), (fx + h * uy, fy - h * ux)])


def _verify_cc(i, o):
    C1, C2 = (i["x1"], i["y1"]), (i["x2"], i["y2"])
    pts = [(o[f"x{n}"], o[f"y{n}"]) for n in (1, 2) if o[f"x{n}"] is not None]
    if len(pts) != o["count"]:
        return 1.0
    d = math.dist(C1, C2)
    if o["count"] == 0:
        return 0.0 if d > i["r1"] + i["r2"] or d < abs(i["r1"] - i["r2"]) or (d == 0 and i["r1"] != i["r2"]) else 1.0
    worst = max(max(abs(math.dist(p, C1) - i["r1"]) / i["r1"], abs(math.dist(p, C2) - i["r2"]) / i["r2"]) for p in pts)
    if o["count"] == 2 and math.dist(*pts) == 0:
        return 1.0
    return worst


def _ref_cc(x1, y1, r1, x2, y2, r2):
    import sympy as sp
    pts = sp.Circle(sp.Point(_R(x1), _R(y1)), _R(r1)).intersection(sp.Circle(sp.Point(_R(x2), _R(y2)), _R(r2)))
    return _two_pts_out([(float(p.x), float(p.y)) for p in pts])


def _rand_cc(rng):
    x1, y1, r1 = _RANDP(rng) / 2, _RANDP(rng) / 2, _rand_len(rng, 1, 20)
    r2 = _rand_len(rng, 1, 20)
    t = rng.uniform(0, 2 * math.pi)
    d = rng.uniform(abs(r1 - r2) * 1.05 + 0.1, (r1 + r2) * 0.95) if rng.random() < 0.8 else rng.uniform(0.1, 60)
    return {"x1": x1, "y1": y1, "r1": r1, "x2": round(x1 + d * math.cos(t), 2), "y2": round(y1 + d * math.sin(t), 2), "r2": r2}


def parse_cc(q0):
    q = _norm(q0)
    if not re.search(r"\bcircles?\b", q) or not re.search(r"\b(intersect\w*|meet|common points?)\b", q) or re.search(r"\b(line|tangent|segment)\b", q):
        return None
    cs = _circles(q)
    if len(cs) != 2:
        return None if len(cs) > 2 else {}
    if any(c[2] is None for c in cs):
        return {"_out_of_scope": "an equation has no real circle (its radius squared is not positive)"}
    (a, b, r), (c, d, s) = cs
    return {"x1": a, "y1": b, "r1": r, "x2": c, "y2": d, "r2": s}


_add("circle_circle_intersection", "advanced",
     name="Intersection of two circles", topic="coordinate geometry / circles",
     purpose="The points where two circles meet (0, 1 or 2 points).",
     inputs=[_I("x1", "x of the first centre"), _I("y1", "y of the first centre"), _I("r1", "first radius"),
             _I("x2", "x of the second centre"), _I("y2", "y of the second centre"), _I("r2", "second radius")],
     outputs=_PT_OUT, primary="count",
     conditions=["r1, r2 > 0", "the circles are not identical", "tangency decided exactly on the written values", "points ordered by x, then y"],
     method="radical line: a = (d^2 + r1^2 - r2^2)/(2d) along the centre line, h = sqrt(r1^2 - a^2) across it",
     latex=r"a = \frac{d^2 + r_1^2 - r_2^2}{2d},\quad h = \sqrt{r_1^2 - a^2}",
     run=circle_circle_intersection, verify=_verify_cc, parse=parse_cc, reference=_ref_cc,
     wrong=lambda x1, y1, r1, x2, y2, r2: circle_circle_intersection(x2, y2, r1, x1, y1, r2) if (x1, y1) != (x2, y2) else _two_pts_out([(x1, y1)]),
     random_inputs=_rand_cc,
     templates=["Find the intersection points of the circle with center (0, 0) and radius 5 and the circle with center (6, 0) and radius 5.",
                "Where do the circles x^2 + y^2 = 25 and (x - 8)^2 + y^2 = 9 meet?"],
     examples=[{"inputs": {"x1": 0, "y1": 0, "r1": 5, "x2": 6, "y2": 0, "r2": 5}, "outputs": {"count": 2, "x1": 3.0, "y1": -4.0, "x2": 3.0, "y2": 4.0}},
               {"inputs": {"x1": 0, "y1": 0, "r1": 5, "x2": 8, "y2": 0, "r2": 3}, "outputs": {"count": 1, "x1": 5.0, "y1": 0.0}}],
     questions=[{"q": "Find the intersection points of the circle with center (0, 0) and radius 5 and the circle with center (6, 0) and radius 5.",
                 "outputs": {"count": 2, "x1": 3.0, "y1": -4.0, "x2": 3.0, "y2": 4.0}},
                {"q": "Where do the circles x^2 + y^2 = 25 and (x - 8)^2 + y^2 = 9 meet?", "outputs": {"count": 1, "x1": 5.0, "y1": 0.0}},
                {"q": "Find the intersection of the circle with center (0, 0) and radius 2 and the circle with center (0, 0) and radius 2.", "outcome": "INVALID_INPUT"}])


def tangent_from_point(cx, cy, r, px, py):
    cx, cy, r, px, py = _fin(cx, "cx"), _fin(cy, "cy"), _pos(r, "r"), _fin(px, "px"), _fin(py, "py")
    d2x = (_fr(px) - _fr(cx)) ** 2 + (_fr(py) - _fr(cy)) ** 2
    g = d2x - _fr(r) ** 2
    if g < 0:
        raise ValueError("the point is inside the circle: no tangent passes through it")
    if g == 0:
        return {"length": 0.0, "x1": px, "y1": py, "x2": px, "y2": py}
    vx, vy = px - cx, py - cy
    d2 = float(d2x)
    L = math.sqrt(float(g))
    f, gg = r * r / d2, r * L / d2
    pts = _sort_pts([(cx + f * vx - gg * vy, cy + f * vy + gg * vx), (cx + f * vx + gg * vy, cy + f * vy - gg * vx)])
    return {"length": L, "x1": pts[0][0], "y1": pts[0][1], "x2": pts[1][0], "y2": pts[1][1]}


def _verify_tangent(i, o):
    C, P, r = (i["cx"], i["cy"]), (i["px"], i["py"]), i["r"]
    worst = abs(o["length"] ** 2 + r * r - math.dist(C, P) ** 2) / math.dist(C, P) ** 2
    for n in (1, 2):
        T = (o[f"x{n}"], o[f"y{n}"])
        dot = (T[0] - C[0]) * (T[0] - P[0]) + (T[1] - C[1]) * (T[1] - P[1])
        worst = max(worst, abs(math.dist(T, C) - r) / r, abs(dot) / (r * max(o["length"], r)), abs(math.dist(T, P) - o["length"]) / max(o["length"], r))
    return worst


def _ref_tangent(cx, cy, r, px, py):
    """Thales: the tangency points are where the circle meets the circle on CP as diameter (numpy)."""
    import numpy as np
    C, P = np.array([cx, cy], float), np.array([px, py], float)
    M, rho = (C + P) / 2, np.linalg.norm(P - C) / 2
    d = np.linalg.norm(M - C)
    a = (d * d + r * r - rho * rho) / (2 * d)
    h = np.sqrt(max(r * r - a * a, 0.0))
    e = (M - C) / d
    base = C + a * e
    perp = np.array([-e[1], e[0]])
    pts = _sort_pts([tuple(map(float, base + h * perp)), tuple(map(float, base - h * perp))])
    return {"length": float(np.sqrt(np.dot(P - C, P - C) - r * r)), "x1": pts[0][0], "y1": pts[0][1], "x2": pts[1][0], "y2": pts[1][1]}


def _rand_tangent(rng):
    cx, cy, r = _RANDP(rng) / 2, _RANDP(rng) / 2, _rand_len(rng, 1, 20)
    t, d = rng.uniform(0, 2 * math.pi), r * rng.uniform(1.1, 4)
    return {"cx": cx, "cy": cy, "r": r, "px": round(cx + d * math.cos(t), 2), "py": round(cy + d * math.sin(t), 2)}


def parse_tangent(q0):
    q = _norm(q0)
    if not re.search(r"\btangents?\b", q) or not re.search(r"\bcircle\b", q) or re.search(r"\b(curve|parabola|ellipse|slope at)\b", q):
        return None
    m = re.search(r"\bfrom\s*(?:the\s*)?(?:external\s*)?(?:point\s*)?" + _P2, q)
    cs = _circles(q)
    if len(cs) > 1:
        return None
    d = {}
    if cs:
        if cs[0][2] is None:
            return {"_out_of_scope": "the equation has no real circle"}
        d.update(cx=cs[0][0], cy=cs[0][1], r=cs[0][2])
    if m:
        d.update(px=float(m.group(1)), py=float(m.group(2)))
    return d


_add("tangent_from_point", "advanced",
     name="Tangents from a point to a circle", topic="coordinate geometry / circles",
     purpose="The tangent length from an external point to a circle and the two points of tangency.",
     inputs=[_I("cx", "x of the centre"), _I("cy", "y of the centre"), _I("r", "radius"), _I("px", "x of the external point"), _I("py", "y of the external point")],
     outputs=[_O("length", "tangent length sqrt(d^2 - r^2)"), _O("x1", "x of the first tangency point (ordered by x, then y)"),
              _O("y1", "y of the first tangency point"), _O("x2", "x of the second tangency point"), _O("y2", "y of the second tangency point")],
     primary="length", conditions=["r > 0", "the point is not inside the circle (on the circle: length 0, both points at the point)"],
     method="L = sqrt(d^2 - r^2); T = C + (r^2/d^2)(P - C) +- (r L/d^2) perp(P - C)",
     latex=r"L = \sqrt{|P-C|^2 - r^2},\quad T = C + \frac{r^2}{d^2}(P-C) \pm \frac{rL}{d^2}(P-C)^{\perp}",
     run=tangent_from_point, verify=_verify_tangent, parse=parse_tangent, reference=_ref_tangent,
     wrong=lambda cx, cy, r, px, py: dict(tangent_from_point(cx, cy, r, px, py), length=math.sqrt((px - cx) ** 2 + (py - cy) ** 2 + r * r)),
     random_inputs=_rand_tangent,
     templates=["Find the length of the tangent from the point (5, 0) to the circle with center (0, 0) and radius 3.",
                "Find the points of tangency of the tangents from (0, 10) to the circle x^2 + y^2 = 36."],
     examples=[{"inputs": {"cx": 0, "cy": 0, "r": 3, "px": 5, "py": 0}, "outputs": {"length": 4.0, "x1": 1.8, "y1": -2.4, "x2": 1.8, "y2": 2.4}},
               {"inputs": {"cx": 0, "cy": 0, "r": 6, "px": 0, "py": 10}, "outputs": {"length": 8.0, "x1": -4.8, "y1": 3.6, "x2": 4.8, "y2": 3.6}}],
     questions=[{"q": "Find the length of the tangent from the point (5, 0) to the circle with center (0, 0) and radius 3.", "outputs": {"length": 4.0}},
                {"q": "Find the points of tangency of the tangents from (0, 10) to the circle x^2 + y^2 = 36.", "outputs": {"x1": -4.8, "y1": 3.6, "x2": 4.8, "y2": 3.6}},
                {"q": "Find the length of the tangent from the point (1, 0) to the circle with center (0, 0) and radius 3.", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# conics (advanced)
# ==================================================================================================================
def _conic_type(A, B, C, D, E, F):
    A, B, C, D, E, F = (_fr(v) for v in (A, B, C, D, E, F))
    if A == 0 and B == 0 and C == 0:
        raise ValueError("A, B and C are all 0: the equation is not quadratic")
    delta = A * C - B * B / 4
    big = A * (C * F - E * E / 4) - B / 2 * (B / 2 * F - E * D / 4) + D / 2 * (B / 2 * E / 2 - C * D / 2)
    if big == 0:
        return "degenerate"
    if delta > 0:
        if (A + C) * big > 0:
            return "empty"
        return "circle" if A == C and B == 0 else "ellipse"
    return "parabola" if delta == 0 else "hyperbola"


def conic_classify(A, B, C, D, E, F):
    for n, v in zip("ABCDEF", (A, B, C, D, E, F)):
        _fin(v, n)
    return {"type": _conic_type(A, B, C, D, E, F)}


def _verify_conic(i, o):
    """A different route: translate to the centre (exact) and read the constant term there."""
    A, B, C, D, E, F = (_fr(i[k]) for k in "ABCDEF")
    disc = 4 * A * C - B * B
    if disc != 0:
        x0 = (B * E - 2 * C * D) / disc
        y0 = (B * D - 2 * A * E) / disc
        f0 = A * x0 * x0 + B * x0 * y0 + C * y0 * y0 + D * x0 + E * y0 + F
        if f0 == 0:
            want = "degenerate"
        elif disc > 0:
            want = "empty" if (f0 > 0) == (A > 0) else ("circle" if A == C and B == 0 else "ellipse")
        else:
            want = "hyperbola"
    else:
        # parabolic: degenerate exactly when the full 3x3 determinant vanishes (expanded along the first row)
        b, d, e = B / 2, D / 2, E / 2
        det = A * (C * F - e * e) - b * (b * F - e * d) + d * (b * e - C * d)
        want = "degenerate" if det == 0 else "parabola"
    return o["type"] == want


def _ref_conic(A, B, C, D, E, F):
    """sympy: exact eigenvalues of the quadratic part, exact 3x3 determinant, the polynomial's value at its centre."""
    import sympy as sp
    a, b, c, d, e, f = (_R(v) for v in (A, B, C, D, E, F))
    M3 = sp.Matrix([[a, b / 2, d / 2], [b / 2, c, e / 2], [d / 2, e / 2, f]])
    if M3.det() == 0:
        return {"type": "degenerate"}
    ev = list(sp.Matrix([[a, b / 2], [b / 2, c]]).eigenvals(multiple=True))
    signs = [sp.sign(sp.nsimplify(v)) for v in ev]
    if 0 in signs:
        return {"type": "parabola"}
    if signs[0] != signs[1]:
        return {"type": "hyperbola"}
    x, y = sp.symbols("x y")
    poly = a * x ** 2 + b * x * y + c * y ** 2 + d * x + e * y + f
    sol = sp.solve([sp.diff(poly, x), sp.diff(poly, y)], [x, y], dict=True)[0]
    val = poly.subs(sol)
    if sp.sign(val) == signs[0]:
        return {"type": "empty"}
    return {"type": "circle" if sp.simplify(ev[0] - ev[1]) == 0 else "ellipse"}


def _rand_conic(rng):
    while True:
        v = {k: float(rng.randint(-6, 6)) for k in "ABCDEF"}
        if rng.random() < 0.1:
            v["B"], v["C"] = 0.0, v["A"]
        if rng.random() < 0.1:
            v["C"] = v["B"] ** 2 / (4 * v["A"]) if v["A"] else v["C"]
        if v["A"] or v["B"] or v["C"]:
            return v


def parse_conic(q0):
    q = _norm(q0)
    if not re.search(r"\b(classify|identify|what (?:type|kind) of (?:conic|curve)|which conic|type of (?:the )?conic|kind of conic)\b", q):
        return None
    es = [e for e in _equations(q) if e["xx"] or e["xy"] or e["yy"]]
    if len(es) != 1:
        return None if es or not re.search(r"\b(conic|curve|equation)\b", q) else {}
    e = es[0]
    return {"A": e["xx"], "B": e["xy"], "C": e["yy"], "D": e["x"], "E": e["y"], "F": e["1"]}


_add("conic_classify", "advanced",
     name="Classify a conic section from its general equation", topic="conic sections / general second-degree equation",
     purpose="Whether A x^2 + B xy + C y^2 + D x + E y + F = 0 is a circle, ellipse, parabola, hyperbola, a degenerate conic or has no real points.",
     inputs=[_I(k, m) for k, m in (("A", "coefficient of x^2"), ("B", "coefficient of xy"), ("C", "coefficient of y^2"),
                                   ("D", "coefficient of x"), ("E", "coefficient of y"), ("F", "constant term"))],
     outputs=[_O("type", "circle / ellipse / parabola / hyperbola / degenerate / empty", "text")], primary="type",
     conditions=["A, B, C not all 0", "exact on the written coefficients",
                 "'degenerate' = a point, a pair of lines or a degenerate equation (zero 3x3 determinant); 'empty' = an ellipse with no real points"],
     method="discriminant B^2 - 4AC and the 3x3 determinant of the conic's matrix, exact in fractions",
     latex=r"\Delta = \det\begin{pmatrix}A & B/2 & D/2\\ B/2 & C & E/2\\ D/2 & E/2 & F\end{pmatrix},\quad B^2 - 4AC \lessgtr 0",
     run=conic_classify, verify=_verify_conic, parse=parse_conic, reference=_ref_conic,
     wrong=lambda A, B, C, D, E, F: {"type": {"ellipse": "hyperbola", "circle": "hyperbola", "empty": "hyperbola", "hyperbola": "ellipse"}.get(
         _conic_type(A, B, C, D, E, F), _conic_type(A, B, C, D, E, F))},
     random_inputs=_rand_conic,
     templates=["Classify the conic x^2 + 4y^2 - 4 = 0.", "What type of conic is x^2 - y^2 + 2x - 3 = 0?"],
     examples=[{"inputs": {"A": 1, "B": 0, "C": 4, "D": 0, "E": 0, "F": -4}, "outputs": {"type": "ellipse"}},
               {"inputs": {"A": 1, "B": 0, "C": -1, "D": 2, "E": 0, "F": -3}, "outputs": {"type": "hyperbola"}},
               {"inputs": {"A": 1, "B": 0, "C": 0, "D": 0, "E": -1, "F": 0}, "outputs": {"type": "parabola"}},
               {"inputs": {"A": 1, "B": 0, "C": 1, "D": 0, "E": 0, "F": 1}, "outputs": {"type": "empty"}},
               {"inputs": {"A": 1, "B": 0, "C": -1, "D": 0, "E": 0, "F": 0}, "outputs": {"type": "degenerate"}}],
     questions=[{"q": "Classify the conic x^2 + 4y^2 - 4 = 0.", "outputs": {"type": "ellipse"}},
                {"q": "What type of conic is x^2 - y^2 + 2x - 3 = 0?", "outputs": {"type": "hyperbola"}},
                {"q": "Identify the conic 2x^2 + 2y^2 - 4x + 8y - 1 = 0.", "outputs": {"type": "circle"}},
                {"q": "Classify the conic x^2 + 2xy + y^2 + x = 0.", "outputs": {"type": "parabola"}},
                {"q": "What kind of conic is x^2 + y^2 + 1 = 0?", "outputs": {"type": "empty"}}])


# ==================================================================================================================
# computational geometry (intermediate / advanced / expert)
# ==================================================================================================================
def _query_and_rest(q):
    """The point named after 'point' is the query; the other points are the shape's vertices."""
    m = re.search(r"\bpoint\s*" + _P2, q)
    pts = _pts2(q)
    if not m:
        return None, pts
    qp = (float(m.group(1)), float(m.group(2)))
    rest = [p for k, p in enumerate(re.finditer(_P2, q)) if p.start() != m.start() + m.group(0).index("(")]
    return qp, [(float(p.group(1)), float(p.group(2))) for p in rest]


def point_in_polygon(px, py, points):
    pts = _pairs(points, "points", 3)
    _simple_polygon(pts)
    P = (_fin(px, "px"), _fin(py, "py"))
    n = len(pts)
    for k in range(n):
        if _on_segment_exact(P, pts[k], pts[(k + 1) % n]):
            return {"location": "boundary"}
    X, Y = _fr(P[0]), _fr(P[1])
    inside = False
    for k in range(n):
        (ax, ay), (bx, by) = [(_fr(c[0]), _fr(c[1])) for c in (pts[k], pts[(k + 1) % n])]
        if (ay > Y) != (by > Y):
            xi = ax + (Y - ay) * (bx - ax) / (by - ay)
            if X < xi:
                inside = not inside
    return {"location": "inside" if inside else "outside"}


def _verify_pip(i, o):
    """Winding number by quadrant counting (exact), a different rule from the crossing count."""
    pts = _pairs(i["points"], "points", 3)
    X, Y = _fr(i["px"]), _fr(i["py"])
    n = len(pts)
    if any(_on_segment_exact((i["px"], i["py"]), pts[k], pts[(k + 1) % n]) for k in range(n)):
        return o["location"] == "boundary"
    wn = 0
    for k in range(n):
        (ax, ay), (bx, by) = [(_fr(c[0]) - X, _fr(c[1]) - Y) for c in (pts[k], pts[(k + 1) % n])]
        if ay <= 0 < by and ax * by - bx * ay > 0:
            wn += 1
        elif by <= 0 < ay and ax * by - bx * ay < 0:
            wn -= 1
    return o["location"] == ("inside" if wn else "outside")


def _ref_pip(px, py, points):
    from shapely.geometry import Point, Polygon
    P, pt = Polygon(_pairs(points, "points", 3)), Point(px, py)
    return {"location": "boundary" if P.touches(pt) else ("inside" if P.contains(pt) else "outside")}


def _rand_pip(rng):
    pts = _star_polygon(rng)
    xs, ys = [p[0] for p in pts], [p[1] for p in pts]
    if rng.random() < 0.05:
        return {"px": pts[1][0], "py": pts[1][1], "points": _flat(pts)}
    return {"px": round(rng.uniform(min(xs) - 2, max(xs) + 2), 2), "py": round(rng.uniform(min(ys) - 2, max(ys) + 2), 2), "points": _flat(pts)}


def parse_pip(q0):
    q = _norm(q0)
    if not re.search(r"\b(inside|within|contains?|lies in|interior of|outside|in the (?:polygon|triangle|quadrilateral))\b", q):
        return None
    if not re.search(_POLY_WORDS + r"|\btriangle\b", q) or re.search(r"\b(hull|lattice|circle|sphere|barycentric)\b", q):
        return None
    qp, rest = _query_and_rest(q)
    if qp is None or len(rest) < 3:
        return None
    return {"px": qp[0], "py": qp[1], "points": _flat(rest)}


_add("point_in_polygon", "intermediate",
     name="Point in polygon test", topic="computational geometry / point location",
     purpose="Whether a point lies inside, outside or on the boundary of a simple polygon.",
     inputs=[_I("px", "x of the point"), _I("py", "y of the point"), _V("points", "polygon vertices in order: x1 y1, x2 y2, ...", n_min=6)],
     outputs=[_O("location", "inside / outside / boundary", "text")], primary="location",
     conditions=["simple polygon with at least 3 vertices", "decided exactly on the written coordinates"],
     method="exact boundary test, then the even-odd ray-crossing rule with exact rational arithmetic",
     latex=r"\text{inside} \iff \#\{\text{edges crossed by the ray } y = p_y, x > p_x\} \text{ is odd}",
     run=point_in_polygon, verify=_verify_pip, parse=parse_pip, reference=_ref_pip,
     wrong=lambda px, py, points: {"location": {"inside": "outside", "outside": "inside", "boundary": "boundary"}[point_in_polygon(px, py, points)["location"]]},
     random_inputs=_rand_pip,
     templates=["Is the point (2, 3) inside the polygon with vertices (0, 0), (5, 0), (5, 5), (0, 5)?",
                "Does the triangle with vertices (0, 0), (4, 0), (0, 4) contain the point (3, 3)?"],
     examples=[{"inputs": {"px": 2, "py": 3, "points": "0 0, 5 0, 5 5, 0 5"}, "outputs": {"location": "inside"}},
               {"inputs": {"px": 3, "py": 3, "points": "0 0, 4 0, 0 4"}, "outputs": {"location": "outside"}},
               {"inputs": {"px": 2, "py": 2, "points": "0 0, 4 0, 0 4"}, "outputs": {"location": "boundary"}}],
     questions=[{"q": "Is the point (2, 3) inside the polygon with vertices (0, 0), (5, 0), (5, 5), (0, 5)?", "outputs": {"location": "inside"}},
                {"q": "Does the triangle with vertices (0, 0), (4, 0), (0, 4) contain the point (3, 3)?", "outputs": {"location": "outside"}},
                {"q": "Is the point (2, 2) inside the polygon with vertices (0, 0), (4, 0), (4, 4), (2, 1.5), (0, 4)?", "outputs": {"location": "outside"}},
                {"q": "Is the point (2, 2) inside the polygon with vertices (0, 0), (4, 4), (4, 0), (0, 4)?", "outcome": "INVALID_INPUT"}])


def _hull(pts):
    """Andrew's monotone chain with exact orientation tests; counterclockwise from the smallest (x, y); collinear
    boundary points are not vertices."""
    P = sorted(set(pts))
    if len(P) < 3:
        return P

    def half(seq):
        h = []
        for p in seq:
            while len(h) >= 2 and _xcross(h[-2], h[-1], p) <= 0:
                h.pop()
            h.append(p)
        return h
    lower, upper = half(P), half(reversed(P))
    return lower[:-1] + upper[:-1]


def convex_hull(points):
    pts = _pairs(points, "points", 3, 5000)
    h = _hull(pts)
    if len(h) < 3:
        raise ValueError("all points are collinear (or fewer than 3 distinct): the hull has no area")
    return {"hull": [[x, y] for x, y in h], "n_vertices": len(h), "area": abs(_shoelace(h)), "perimeter": _perimeter(h)}


def _verify_hull(i, o):
    pts = _pairs(i["points"], "points", 3, 5000)
    h = [tuple(p) for p in o["hull"]]
    S = set(pts)
    if len(h) != o["n_vertices"] or len(h) < 3 or any(p not in S for p in h) or h[0] != min(h):
        return False
    n = len(h)
    if any(_xcross(h[k], h[(k + 1) % n], h[(k + 2) % n]) <= 0 for k in range(n)):
        return False                                          # strictly convex, counterclockwise
    for p in S:
        if any(_xcross(h[k], h[(k + 1) % n], p) < 0 for k in range(n)):
            return False                                      # every point inside or on the hull
    return _res((o["area"], abs(_shoelace(h))), (o["perimeter"], _perimeter(h)))


def _ref_hull(points):
    from scipy.spatial import ConvexHull
    import numpy as np
    P = np.array(_pairs(points, "points", 3, 5000))
    H = ConvexHull(P)
    v = [tuple(map(float, P[k])) for k in H.vertices]
    v = _ccw_start(v)
    return {"hull": [list(p) for p in v], "n_vertices": len(v), "area": float(H.volume), "perimeter": float(H.area)}


def _rand_points(rng, lo=5, hi=60, nd=3, scale=50.0):
    n = rng.randint(lo, hi)
    return _flat([(round(rng.uniform(-scale, scale), nd), round(rng.uniform(-scale, scale), nd)) for _ in range(n)])


def _pts_or_list(q):
    p = _pts2(q)
    if p:
        return _flat(p)
    m = re.search(r"\bpoints?\b\s*:?\s*((?:" + _N + r"\s+" + _N + r"\s*[,;]\s*)+" + _N + r"\s+" + _N + r")", q)
    return [float(x) for x in re.findall(_N, m.group(1))] if m else None


def parse_hull(q0):
    q = _norm(q0)
    if not re.search(r"\bconvex hull\b", q) or _pts3(q):
        return None
    pts = _pts_or_list(q)
    return {"points": pts} if pts else {}


_add("convex_hull", "advanced",
     name="Convex hull of points in the plane", topic="computational geometry / convex hulls",
     purpose="The convex hull of a set of points: its vertices (counterclockwise), their number, its area and perimeter.",
     inputs=[_V("points", "the points: x1 y1, x2 y2, ...", n_min=6)],
     outputs=[_O("hull", "hull vertices [x, y], counterclockwise from the smallest x (then smallest y)", "text"),
              _O("n_vertices", "number of hull vertices"), _O("area", "area of the hull"), _O("perimeter", "perimeter of the hull")],
     primary="area", conditions=["at least 3 points, not all collinear, at most 5000",
                                 "points on a hull edge between two vertices are not vertices", "repeated points count once"],
     method="Andrew's monotone chain with exact (rational) orientation tests; shoelace area",
     latex=r"\operatorname{conv}(S) = \{\textstyle\sum \lambda_i p_i : \lambda_i \ge 0, \sum\lambda_i = 1\}",
     run=convex_hull, verify=_verify_hull, parse=parse_hull, reference=_ref_hull,
     wrong=lambda points: (lambda r: dict(r, area=2 * r["area"]))(convex_hull(points)),
     random_inputs=lambda rng: {"points": _rand_points(rng)},
     templates=["Find the convex hull of the points (0, 0), (2, 1), (4, 0), (3, 3), (1, 2), (2, 4).",
                "What is the area of the convex hull of (0, 0), (1, 1), (2, 0), (2, 2), (0, 2)?"],
     examples=[{"inputs": {"points": "0 0, 2 1, 4 0, 3 3, 1 2, 2 4"}, "outputs": {"hull": [[0.0, 0.0], [4.0, 0.0], [3.0, 3.0], [2.0, 4.0]], "n_vertices": 4, "area": 9.0}},
               {"inputs": {"points": "0 0, 1 1, 2 0, 2 2, 0 2, 1 0"}, "outputs": {"n_vertices": 4, "area": 4.0, "perimeter": 8.0}}],
     questions=[{"q": "Find the convex hull of the points (0, 0), (2, 1), (4, 0), (3, 3), (1, 2), (2, 4).", "outputs": {"n_vertices": 4, "area": 9.0}},
                {"q": "What is the area of the convex hull of (0, 0), (1, 1), (2, 0), (2, 2), (0, 2)?", "outputs": {"area": 4.0}},
                {"q": "Find the convex hull of the points (0, 0), (1, 1), (2, 2).", "outcome": "INVALID_INPUT"}])


def closest_pair(points):
    pts = _pairs(points, "points", 2, 2000)
    P = sorted(pts)
    best = math.inf
    for k in range(len(P)):
        x0, y0 = P[k]
        for j in range(k + 1, len(P)):
            dx = P[j][0] - x0
            if dx >= best:
                break
            d = math.hypot(dx, P[j][1] - y0)
            if d < best:
                best = d
    return {"distance": best}


def _verify_closest(i, o):
    pts = _pairs(i["points"], "points", 2, 2000)
    d2 = min((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 for a, b in itertools.combinations(pts, 2))
    return abs(o["distance"] - math.sqrt(d2)) / max(math.sqrt(d2), 1e-300) if d2 else float(o["distance"] != 0)


def parse_closest(q0):
    q = _norm(q0)
    if not re.search(r"\b(closest pair|nearest pair|closest two|smallest distance between any two|minimum distance between any two|"
                     r"shortest distance between any two)\b", q) or _pts3(q):
        return None
    pts = _pts_or_list(q)
    return {"points": pts} if pts else {}


_add("closest_pair", "advanced",
     name="Closest pair of points", topic="computational geometry / proximity",
     purpose="The smallest distance between any two of a set of points in the plane.",
     inputs=[_V("points", "the points: x1 y1, x2 y2, ...", n_min=4)], outputs=[_O("distance", "smallest pairwise distance")],
     primary="distance", conditions=["at least 2 points, at most 2000", "a repeated point gives distance 0"],
     method="sort by x and sweep, stopping each scan once the x-gap exceeds the best distance",
     latex=r"\delta = \min_{i<j} \|p_i - p_j\|",
     run=closest_pair, verify=_verify_closest, parse=parse_closest,
     reference=lambda points: {"distance": float(__import__("scipy.spatial.distance", fromlist=["pdist"]).pdist(_pairs(points, "points", 2)).min())},
     wrong=lambda points: {"distance": closest_pair(points)["distance"] ** 2},
     random_inputs=lambda rng: {"points": _rand_points(rng, 2, 200)},
     templates=["Find the closest pair distance among the points (0, 0), (5, 5), (1, 1), (9, 2), (6, 6.5)."],
     examples=[{"inputs": {"points": "0 0, 5 5, 1 1, 9 2, 6 6.5"}, "outputs": {"distance": math.sqrt(2)}},
               {"inputs": {"points": "0 0, 3 4, 10 0"}, "outputs": {"distance": 5.0}}],
     questions=[{"q": "Find the closest pair distance among the points (0, 0), (5, 5), (1, 1), (9, 2), (6, 6.5).", "outputs": {"distance": math.sqrt(2)}},
                {"q": "What is the smallest distance between any two of the points (0, 0), (3, 4), (10, 0)?", "outputs": {"distance": 5.0}}])


def farthest_pair(points):
    pts = _pairs(points, "points", 2, 5000)
    h = _hull(pts) if len(set(pts)) >= 3 else sorted(set(pts))
    best = max((math.dist(a, b) for a, b in itertools.combinations(h, 2)), default=0.0)
    return {"distance": best}


def _verify_farthest(i, o):
    pts = _pairs(i["points"], "points", 2, 5000)
    d = math.sqrt(max((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 for a, b in itertools.combinations(pts, 2)))
    return abs(o["distance"] - d) / max(d, 1e-300)


def parse_farthest(q0):
    q = _norm(q0)
    if not re.search(r"\b(farthest pair|furthest pair|farthest apart|furthest apart|largest distance between any two|"
                     r"greatest distance between any two|maximum distance between any two|diameter of the (?:point )?set)\b", q) or _pts3(q):
        return None
    pts = _pts_or_list(q)
    return {"points": pts} if pts else {}


_add("farthest_pair", "advanced",
     name="Farthest pair of points (diameter of a point set)", topic="computational geometry / proximity",
     purpose="The largest distance between any two of a set of points in the plane.",
     inputs=[_V("points", "the points: x1 y1, x2 y2, ...", n_min=4)], outputs=[_O("distance", "largest pairwise distance")],
     primary="distance", conditions=["at least 2 points, at most 5000"],
     method="the farthest pair are vertices of the convex hull: hull first (monotone chain), then all hull-vertex pairs",
     latex=r"D = \max_{i<j} \|p_i - p_j\|", run=farthest_pair, verify=_verify_farthest, parse=parse_farthest,
     reference=lambda points: {"distance": float(__import__("scipy.spatial.distance", fromlist=["pdist"]).pdist(_pairs(points, "points", 2)).max())},
     wrong=lambda points: (lambda p: {"distance": math.hypot(max(x for x, _ in p) - min(x for x, _ in p), max(y for _, y in p) - min(y for _, y in p))})(_pairs(points, "points", 2)),
     random_inputs=lambda rng: {"points": _rand_points(rng, 3, 200)},
     templates=["Find the farthest pair distance among the points (0, 0), (3, 4), (1, 1), (2, 2)."],
     examples=[{"inputs": {"points": "0 0, 3 4, 1 1, 2 2"}, "outputs": {"distance": 5.0}},
               {"inputs": {"points": "0 0, 6 8"}, "outputs": {"distance": 10.0}}],
     questions=[{"q": "Find the farthest pair distance among the points (0, 0), (3, 4), (1, 1), (2, 2).", "outputs": {"distance": 5.0}},
                {"q": "What is the largest distance between any two of the points (0, 0), (6, 8), (3, 3)?", "outputs": {"distance": 10.0}}])


def segment_intersection(points):
    A, B, C, D = _pairs(points, "points", 4, 4)
    if A == B or C == D:
        raise ValueError("a segment has two equal endpoints")
    none = {"status": "none", "x": None, "y": None}
    if not _segments_meet_exact(A, B, C, D):
        return none
    ax, ay, bx, by, cx, cy, dx, dy = (_fr(v) for v in (*A, *B, *C, *D))
    rx, ry, sx, sy = bx - ax, by - ay, dx - cx, dy - cy
    den = rx * sy - ry * sx
    if den != 0:
        t = ((cx - ax) * sy - (cy - ay) * sx) / den
        return {"status": "point", "x": float(ax + t * rx) + 0.0, "y": float(ay + t * ry) + 0.0}
    rr = rx * rx + ry * ry                                    # collinear: overlap of the parameter intervals on AB
    t0 = ((cx - ax) * rx + (cy - ay) * ry) / rr
    t1 = ((dx - ax) * rx + (dy - ay) * ry) / rr
    lo, hi = max(Fraction(0), min(t0, t1)), min(Fraction(1), max(t0, t1))
    if lo == hi:
        return {"status": "point", "x": float(ax + lo * rx) + 0.0, "y": float(ay + lo * ry) + 0.0}
    return {"status": "overlap", "x": None, "y": None}


def _verify_seg(i, o):
    """Parametric test with floats (a different route from the orientation tests); the point must lie on both."""
    A, B, C, D = _pairs(i["points"], "points", 4, 4)
    rx, ry, sx, sy = B[0] - A[0], B[1] - A[1], D[0] - C[0], D[1] - C[1]
    den = rx * sy - ry * sx
    sc = max(math.hypot(rx, ry), math.hypot(sx, sy))
    if o["status"] == "point":
        P = (o["x"], o["y"])
        on = lambda U, V: abs(_cross(U, V, P)) / (math.dist(U, V) * max(1.0, sc)) + max(0.0, math.dist(U, P) + math.dist(P, V) - math.dist(U, V)) / max(1.0, sc)
        return max(on(A, B), on(C, D))
    collinear = den == 0 and abs(_cross(A, B, C)) <= 1e-12 * sc * sc
    if o["status"] == "overlap":
        return collinear
    if den == 0:
        if not collinear:
            return True                                       # parallel and apart
        rr = rx * rx + ry * ry
        t0 = ((C[0] - A[0]) * rx + (C[1] - A[1]) * ry) / rr
        t1 = ((D[0] - A[0]) * rx + (D[1] - A[1]) * ry) / rr
        return max(t0, t1) < 0 or min(t0, t1) > 1
    t = ((C[0] - A[0]) * sy - (C[1] - A[1]) * sx) / den
    u = ((C[0] - A[0]) * ry - (C[1] - A[1]) * rx) / den
    eps = 1e-9
    return t < -eps or t > 1 + eps or u < -eps or u > 1 + eps


def _ref_seg(points):
    from shapely.geometry import LineString
    A, B, C, D = _pairs(points, "points", 4, 4)
    g = LineString([A, B]).intersection(LineString([C, D]))
    if g.is_empty:
        return {"status": "none", "x": None, "y": None}
    if g.geom_type == "Point":
        return {"status": "point", "x": g.x, "y": g.y}
    return {"status": "overlap", "x": None, "y": None}


def _wrong_seg(points):
    A, B, C, D = _pairs(points, "points", 4, 4)
    rx, ry, sx, sy = B[0] - A[0], B[1] - A[1], D[0] - C[0], D[1] - C[1]
    den = rx * sy - ry * sx
    if den == 0:
        return {"status": "overlap", "x": None, "y": None}
    t = ((C[0] - A[0]) * sy - (C[1] - A[1]) * sx) / den            # the LINES' intersection, bounds ignored
    return {"status": "point", "x": A[0] + t * rx, "y": A[1] + t * ry}


def _rand_seg(rng):
    while True:
        pts = [(_RANDP(rng) / 2, _RANDP(rng) / 2) for _ in range(4)]
        if pts[0] != pts[1] and pts[2] != pts[3]:
            return {"points": _flat(pts)}


def parse_seg(q0):
    q = _norm(q0)
    if not re.search(r"\bsegments?\b", q) or not re.search(r"\b(intersect\w*|cross|meet)\b", q) or _pts3(q):
        return None
    p = _pts2(q)
    return {"points": _flat(p)} if len(p) == 4 else (None if p else {})


_add("segment_intersection", "advanced",
     name="Intersection of two line segments", topic="computational geometry / intersections",
     purpose="Whether segments AB and CD meet, and where: at one point, along an overlap, or not at all.",
     inputs=[_V("points", "the endpoints A, B, C, D: xA yA, xB yB, xC yC, xD yD", n_min=8, n_max=8)],
     outputs=[_O("status", "point / overlap / none", "text"), _O("x", "x of the common point (point only)"), _O("y", "y of the common point (point only)")],
     primary="status", conditions=["each segment has distinct endpoints", "decided exactly on the written coordinates (touching counts as meeting)"],
     method="exact orientation tests, then the intersection parameter in rational arithmetic; collinear overlaps by interval",
     latex=r"A + t(B-A) = C + u(D-C),\quad 0 \le t, u \le 1",
     run=segment_intersection, verify=_verify_seg, parse=parse_seg, reference=_ref_seg, wrong=_wrong_seg, random_inputs=_rand_seg,
     templates=["Do the segments from (0, 0) to (4, 4) and from (0, 4) to (4, 0) intersect?"],
     examples=[{"inputs": {"points": "0 0, 4 4, 0 4, 4 0"}, "outputs": {"status": "point", "x": 2.0, "y": 2.0}},
               {"inputs": {"points": "0 0, 1 1, 2 0, 3 -1"}, "outputs": {"status": "none"}},
               {"inputs": {"points": "0 0, 2 0, 1 0, 3 0"}, "outputs": {"status": "overlap"}},
               {"inputs": {"points": "0 0, 2 0, 2 0, 2 5"}, "outputs": {"status": "point", "x": 2.0, "y": 0.0}}],
     questions=[{"q": "Do the segments from (0, 0) to (4, 4) and from (0, 4) to (4, 0) intersect?", "outputs": {"status": "point", "x": 2.0, "y": 2.0}},
                {"q": "Do the segments from (0, 0) to (1, 1) and from (2, 0) to (3, -1) intersect?", "outputs": {"status": "none"}},
                {"q": "Do the segments from (0, 0) to (0, 0) and from (2, 0) to (3, -1) intersect?", "outcome": "INVALID_INPUT"}])


def point_segment_distance(px, py, x1, y1, x2, y2):
    P, A, B = (_fin(px, "px"), _fin(py, "py")), (_fin(x1, "x1"), _fin(y1, "y1")), (_fin(x2, "x2"), _fin(y2, "y2"))
    if A == B:
        raise ValueError("the segment's endpoints coincide")
    rx, ry = B[0] - A[0], B[1] - A[1]
    t = ((P[0] - A[0]) * rx + (P[1] - A[1]) * ry) / (rx * rx + ry * ry)
    t = min(1.0, max(0.0, t))
    Q = (A[0] + t * rx, A[1] + t * ry) if 0 < t < 1 else (A if t == 0 else B)
    return {"distance": math.dist(P, Q), "qx": Q[0] + 0.0, "qy": Q[1] + 0.0}


def _verify_psd(i, o):
    P, A, B = (i["px"], i["py"]), (i["x1"], i["y1"]), (i["x2"], i["y2"])
    Q = (o["qx"], o["qy"])
    L = math.dist(A, B)
    sc = max(1.0, L, math.dist(P, A))
    on = abs(_cross(A, B, Q)) / (L * sc) + max(0.0, math.dist(A, Q) + math.dist(Q, B) - L) / sc
    worst = max(on, abs(o["distance"] - math.dist(P, Q)) / sc)
    d_end = min(math.dist(P, A), math.dist(P, B))
    if o["distance"] > d_end * (1 + 1e-10) + 1e-300:
        return 1.0
    if Q not in (A, B):                                        # interior foot: PQ perpendicular to AB
        worst = max(worst, abs((P[0] - Q[0]) * (B[0] - A[0]) + (P[1] - Q[1]) * (B[1] - A[1])) / (L * sc))
    else:                                                     # endpoint: the projection falls outside (or on its end)
        far = B if Q == A else A
        if (P[0] - Q[0]) * (far[0] - Q[0]) + (P[1] - Q[1]) * (far[1] - Q[1]) > 1e-10 * L * sc:
            return 1.0
    return worst


def _ref_psd(px, py, x1, y1, x2, y2):
    from shapely.geometry import LineString, Point
    from shapely.ops import nearest_points
    S, P = LineString([(x1, y1), (x2, y2)]), Point(px, py)
    q = nearest_points(S, P)[0]
    return {"distance": S.distance(P), "qx": q.x, "qy": q.y}


def _rand_psd(rng):
    A = (_RANDP(rng), _RANDP(rng))
    B = (A[0] + rng.uniform(-30, 30), A[1] + rng.uniform(-30, 30))
    t, off = rng.uniform(-2, 3), rng.uniform(-20, 20)
    L = math.dist(A, B)
    nx, ny = -(B[1] - A[1]) / L, (B[0] - A[0]) / L
    return {"px": round(A[0] + t * (B[0] - A[0]) + off * nx, 2), "py": round(A[1] + t * (B[1] - A[1]) + off * ny, 2),
            "x1": A[0], "y1": A[1], "x2": round(B[0], 2), "y2": round(B[1], 2)}


def parse_psd(q0):
    q = _norm(q0)
    if not re.search(r"\bdistance\b|\bclosest point\b|\bnearest point\b", q) or not re.search(r"\bsegment\b", q) or re.search(r"\bintersect\w*\b", q) or _pts3(q):
        return None
    qp, rest = _query_and_rest(q)
    if qp is None:
        p = _pts2(q)
        if len(p) != 3:
            return None if p else {}
        qp, rest = p[0], p[1:]
    if len(rest) != 2:
        return None
    return {"px": qp[0], "py": qp[1], "x1": rest[0][0], "y1": rest[0][1], "x2": rest[1][0], "y2": rest[1][1]}


_add("point_segment_distance", "intermediate",
     name="Distance from a point to a segment", topic="computational geometry / proximity",
     purpose="The shortest distance from a point to the segment AB and the closest point of the segment.",
     inputs=[_I("px", "x of the point"), _I("py", "y of the point"), _I("x1", "x of A"), _I("y1", "y of A"), _I("x2", "x of B"), _I("y2", "y of B")],
     outputs=[_O("distance", "shortest distance"), _O("qx", "x of the closest point"), _O("qy", "y of the closest point")], primary="distance",
     conditions=["A != B"], method="project P onto AB, clamp the parameter to [0, 1]",
     latex=r"t^* = \operatorname{clamp}\!\left(\frac{(P-A)\cdot(B-A)}{\|B-A\|^2}, 0, 1\right),\quad d = \|P - (A + t^*(B-A))\|",
     run=point_segment_distance, verify=_verify_psd, parse=parse_psd, reference=_ref_psd,
     wrong=lambda px, py, x1, y1, x2, y2: (lambda t: {"distance": abs((x2 - x1) * (py - y1) - (y2 - y1) * (px - x1)) / math.hypot(x2 - x1, y2 - y1),
                                                     "qx": x1 + t * (x2 - x1), "qy": y1 + t * (y2 - y1)})(
         ((px - x1) * (x2 - x1) + (py - y1) * (y2 - y1)) / ((x2 - x1) ** 2 + (y2 - y1) ** 2)),
     random_inputs=_rand_psd,
     templates=["Find the distance from the point (3, 4) to the segment from (0, 0) to (6, 0).",
                "What is the distance from the point (8, 2) to the segment joining (0, 0) and (4, 0)?"],
     examples=[{"inputs": {"px": 3, "py": 4, "x1": 0, "y1": 0, "x2": 6, "y2": 0}, "outputs": {"distance": 4.0, "qx": 3.0, "qy": 0.0}},
               {"inputs": {"px": 8, "py": 3, "x1": 0, "y1": 0, "x2": 4, "y2": 0}, "outputs": {"distance": 5.0, "qx": 4.0, "qy": 0.0}}],
     questions=[{"q": "Find the distance from the point (3, 4) to the segment from (0, 0) to (6, 0).", "outputs": {"distance": 4.0}},
                {"q": "What is the distance from the point (8, 3) to the segment joining (0, 0) and (4, 0)?", "outputs": {"distance": 5.0, "qx": 4.0}}])


def polyline_length(points):
    pts = _pairs(points, "points", 2)
    return {"length": _perimeter(pts, closed=False), "segments": len(pts) - 1}


def _ref_polyline(points):
    from shapely.geometry import LineString
    pts = _pairs(points, "points", 2)
    return {"length": LineString(pts).length, "segments": len(pts) - 1}


def parse_polyline(q0):
    q = _norm(q0)
    if not re.search(r"\b(polyline|path|broken line|polygonal chain|route|walk)\b", q) or not re.search(r"\b(length|long|distance travelled|total distance)\b", q) or _pts3(q):
        return None
    if re.search(r"\b(polygon|closed|perimeter)\b", q):
        return None
    pts = _pts_or_list(q)
    return {"points": pts} if pts else {}


_add("polyline_length", "beginner",
     name="Length of a polyline (path through points)", topic="coordinate geometry / paths",
     purpose="The total length of the open path that visits the points in the order given.",
     inputs=[_V("points", "the points in order: x1 y1, x2 y2, ...", n_min=4)],
     outputs=[_O("length", "sum of the segment lengths"), _O("segments", "number of segments")], primary="length",
     conditions=["at least 2 points", "the path is open (not closed back to the start)"], method="sum of the distances between consecutive points",
     latex=r"L = \sum_{i=1}^{n-1} \|p_{i+1} - p_i\|", run=polyline_length,
     verify=lambda i, o: (lambda p: _res((o["length"], sum(math.sqrt((b[0] - a[0]) ** 2 + (b[1] - a[1]) ** 2) for a, b in zip(p, p[1:]))), (o["segments"], len(p) - 1)))(_pairs(i["points"], "points", 2)),
     parse=parse_polyline, reference=_ref_polyline,
     wrong=lambda points: (lambda p: {"length": math.dist(p[0], p[-1]), "segments": len(p) - 1})(_pairs(points, "points", 2)),
     random_inputs=lambda rng: {"points": _rand_points(rng, 3, 30)},
     templates=["Find the length of the path through (0, 0), (3, 4) and (3, 10)."],
     examples=[{"inputs": {"points": "0 0, 3 4, 3 10"}, "outputs": {"length": 11.0, "segments": 2}},
               {"inputs": {"points": [0, 0, 1, 0, 1, 1, 0, 1]}, "outputs": {"length": 3.0}}],
     questions=[{"q": "Find the length of the path through (0, 0), (3, 4) and (3, 10).", "outputs": {"length": 11.0}},
                {"q": "A route goes from (1, 1) to (4, 5) to (10, 13). How long is it?", "outputs": {"length": 15.0}}])


def polygon_is_convex(points):
    pts = _pairs(points, "points", 3)
    _simple_polygon(pts)
    n = len(pts)
    signs = {(_xcross(pts[k], pts[(k + 1) % n], pts[(k + 2) % n]) > 0) - (_xcross(pts[k], pts[(k + 1) % n], pts[(k + 2) % n]) < 0) for k in range(n)}
    signs.discard(0)
    return {"convex": "yes" if len(signs) <= 1 else "no"}


def _verify_convex(i, o):
    """Convex exactly when the polygon's area equals the area of its convex hull (exact rationals)."""
    pts = _pairs(i["points"], "points", 3)
    exact = lambda P: abs(sum(_fr(P[k][0]) * _fr(P[(k + 1) % len(P)][1]) - _fr(P[(k + 1) % len(P)][0]) * _fr(P[k][1]) for k in range(len(P))))
    return o["convex"] == ("yes" if exact(pts) == exact(_hull(pts)) else "no")


def _rand_convex(rng):
    if rng.random() < 0.3:
        while True:
            h = _hull([(round(rng.uniform(-30, 30), 2), round(rng.uniform(-30, 30), 2)) for _ in range(rng.randint(4, 15))])
            if len(h) >= 3:
                break
        if rng.random() < 0.5:
            h.reverse()
        k = rng.randrange(len(h))
        return {"points": _flat(h[k:] + h[:k])}
    return {"points": _flat(_star_polygon(rng, 6, 12))}


def parse_convex(q0):
    q = _norm(q0)
    if not re.search(r"\bconvex\b", q) or re.search(r"\bhull\b", q) or not re.search(_POLY_WORDS + r"|\btriangle\b", q):
        return None
    pts = _point_list(q)
    return {"points": pts} if pts and len(pts) >= 6 else {}


_add("polygon_is_convex", "intermediate",
     name="Is a polygon convex?", topic="polygons / convexity",
     purpose="Whether a simple polygon given by its vertices is convex.",
     inputs=[_V("points", "vertices in order: x1 y1, x2 y2, ...", n_min=6)], outputs=[_O("convex", "yes / no", "text")], primary="convex",
     conditions=["simple polygon", "a vertex on a straight edge (180 degrees) does not make it non-convex", "exact on the written coordinates"],
     method="every turn (cross product of consecutive edges) has the same sign, computed exactly",
     latex=r"\operatorname{sign}\big((p_{i+1}-p_i)\times(p_{i+2}-p_{i+1})\big) \text{ constant}",
     run=polygon_is_convex, verify=_verify_convex, parse=parse_convex,
     reference=lambda points: (lambda P: {"convex": "yes" if abs(P.convex_hull.area - P.area) <= 1e-12 * P.area else "no"})(
         __import__("shapely.geometry", fromlist=["Polygon"]).Polygon(_pairs(points, "points", 3))),
     wrong=lambda points: {"convex": "yes"}, random_inputs=_rand_convex,
     templates=["Is the polygon with vertices (0, 0), (4, 0), (4, 4), (2, 1), (0, 4) convex?"],
     examples=[{"inputs": {"points": "0 0, 4 0, 4 4, 2 1, 0 4"}, "outputs": {"convex": "no"}},
               {"inputs": {"points": "0 0, 4 0, 4 4, 0 4"}, "outputs": {"convex": "yes"}},
               {"inputs": {"points": "0 0, 2 0, 4 0, 4 4, 0 4"}, "outputs": {"convex": "yes"}}],
     questions=[{"q": "Is the polygon with vertices (0, 0), (4, 0), (4, 4), (2, 1), (0, 4) convex?", "outputs": {"convex": "no"}},
                {"q": "Is the quadrilateral with vertices (0, 0), (4, 0), (4, 4), (0, 4) convex?", "outputs": {"convex": "yes"}}])


def pick_counts(points):
    pts = _pairs(points, "points", 3)
    if any(v != int(v) for p in pts for v in p):
        raise ValueError("Pick's theorem needs integer (lattice) coordinates")
    pts = [(int(x), int(y)) for x, y in pts]
    _simple_polygon(pts)
    n = len(pts)
    twice = abs(sum(pts[k][0] * pts[(k + 1) % n][1] - pts[(k + 1) % n][0] * pts[k][1] for k in range(n)))
    b = sum(math.gcd(abs(pts[(k + 1) % n][0] - pts[k][0]), abs(pts[(k + 1) % n][1] - pts[k][1])) for k in range(n))
    return {"area": twice / 2, "boundary_points": b, "interior_points": (twice - b + 2) // 2}


def _ref_pick(points):
    """Count the lattice points one by one (shapely's robust predicates)."""
    from shapely.geometry import Point, Polygon
    pts = [(int(x), int(y)) for x, y in _pairs(points, "points", 3)]
    P = Polygon(pts)
    xs, ys = [p[0] for p in pts], [p[1] for p in pts]
    inside = boundary = 0
    for x in range(min(xs), max(xs) + 1):
        for y in range(min(ys), max(ys) + 1):
            pt = Point(x, y)
            if P.touches(pt):
                boundary += 1
            elif P.contains(pt):
                inside += 1
    return {"area": P.area, "boundary_points": boundary, "interior_points": inside}


def _verify_pick(i, o):
    pts = [(int(x), int(y)) for x, y in _pairs(i["points"], "points", 3)]
    n = len(pts)
    area = abs(sum((pts[(k + 1) % n][0] - pts[k][0]) * (pts[k][1] + pts[(k + 1) % n][1]) for k in range(n))) / 2
    b = 0
    for k in range(n):                                        # boundary lattice points counted edge by edge, directly
        (x0, y0), (x1, y1) = pts[k], pts[(k + 1) % n]
        dx, dy = x1 - x0, y1 - y0
        steps = max(abs(dx), abs(dy))
        b += sum(1 for s in range(steps) if (s * dx) % steps == 0 and (s * dy) % steps == 0)
    return _res((o["area"], area), (o["boundary_points"], b), (o["interior_points"], area - b / 2 + 1))


def _rand_pick(rng):
    return {"points": _flat([(int(round(x)), int(round(y))) for x, y in _star_polygon(rng, 3, 9, scale=12)])}


def parse_pick(q0):
    q = _norm(q0)
    if not re.search(r"\b(lattice points?|pick'?s? theorem|interior points|boundary points|integer points)\b", q):
        return None
    pts = _point_list(q)
    return {"points": pts} if pts and len(pts) >= 6 else {}


_add("pick_lattice_points", "advanced",
     name="Pick's theorem: lattice points of a polygon", topic="polygons / lattice polygons",
     purpose="Area, boundary lattice points and interior lattice points of a simple polygon with integer vertices.",
     inputs=[_V("points", "integer vertices in order: x1 y1, x2 y2, ...", n_min=6)],
     outputs=[_O("area", "area"), _O("boundary_points", "lattice points on the boundary"), _O("interior_points", "lattice points strictly inside")],
     primary="area", conditions=["integer coordinates", "simple polygon"],
     method="B = sum of gcd(|dx|, |dy|) over the edges; shoelace area A; Pick: I = A - B/2 + 1",
     latex=r"A = I + \frac{B}{2} - 1", run=pick_counts, verify=_verify_pick, parse=parse_pick, reference=_ref_pick,
     wrong=lambda points: (lambda r: dict(r, interior_points=r["interior_points"] - 1, area=r["area"] - 1))(pick_counts(points)),
     random_inputs=_rand_pick,
     templates=["How many lattice points are inside the polygon with vertices (0, 0), (4, 0), (4, 3), (0, 3)?"],
     examples=[{"inputs": {"points": "0 0, 4 0, 4 3, 0 3"}, "outputs": {"area": 12.0, "boundary_points": 14, "interior_points": 6}},
               {"inputs": {"points": "0 0, 5 0, 0 5"}, "outputs": {"area": 12.5, "boundary_points": 15, "interior_points": 6}}],
     questions=[{"q": "How many lattice points are inside the polygon with vertices (0, 0), (4, 0), (4, 3), (0, 3)?", "outputs": {"interior_points": 6}},
                {"q": "Use Pick's theorem for the triangle with vertices (0, 0), (5, 0), (0, 5).", "outputs": {"boundary_points": 15, "interior_points": 6}},
                {"q": "Count the lattice points inside the polygon with vertices (0, 0), (2.5, 0), (0, 2).", "outcome": "INVALID_INPUT"}])


def _cevians(a, b, c):
    a, b, c = _tri_check(a, b, c)
    x, y, z = sorted((a, b, c), reverse=True)
    K = 0.25 * math.sqrt((x + (y + z)) * (z - (x - y)) * (z + (x - y)) * (x + (y - z)))
    med = lambda o, p, q: 0.5 * math.sqrt(2 * p * p + 2 * q * q - o * o)
    bis = lambda o, p, q: math.sqrt(p * q * (p + q + o) * (p + q - o)) / (p + q)
    return {"median_a": med(a, b, c), "median_b": med(b, a, c), "median_c": med(c, a, b),
            "altitude_a": 2 * K / a, "altitude_b": 2 * K / b, "altitude_c": 2 * K / c,
            "bisector_a": bis(a, b, c), "bisector_b": bis(b, a, c), "bisector_c": bis(c, a, b)}


def _verify_cevians(i, o):
    """Stewart's theorem for each median and bisector; a h_a = b h_b = c h_c = 2K with K from the law of cosines."""
    a, b, c = i["a"], i["b"], i["c"]
    out = []
    for o_, p, q, s in ((a, b, c, "a"), (b, a, c, "b"), (c, a, b, "c")):
        m = o[f"median_{s}"]
        out.append((p * p * o_ / 2 + q * q * o_ / 2, o_ * (m * m + o_ * o_ / 4)))
        mm, nn = o_ * q / (p + q), o_ * p / (p + q)          # the bisector divides the opposite side in the ratio of the others
        t = o[f"bisector_{s}"]
        out.append((p * p * mm + q * q * nn, o_ * (t * t + mm * nn)))
    cosC = (a * a + b * b - c * c) / (2 * a * b)
    K = 0.5 * a * b * math.sqrt(max(0.0, 1 - cosC * cosC))
    out += [(a * o["altitude_a"], 2 * K), (b * o["altitude_b"], 2 * K), (c * o["altitude_c"], 2 * K)]
    return max(_rel(u, v) for u, v in out) if all(math.isfinite(u) for u, _ in out) else 1.0


def _ref_cevians(a, b, c):
    """Construct the triangle in coordinates (numpy) and measure: midpoint distance, distance to the opposite line, and
    the internal bisector direction (sum of unit edge vectors) intersected with the opposite side."""
    import numpy as np
    x = (c * c - b * b + a * a) / (2 * a)
    A, B, C = np.array([x, np.sqrt(c * c - x * x)]), np.array([0.0, 0.0]), np.array([a, 0.0])
    out = {}
    for V, U, W, s_ in ((A, B, C, "a"), (B, C, A, "b"), (C, A, B, "c")):
        out[f"median_{s_}"] = float(np.linalg.norm(V - (U + W) / 2))
        e = W - U
        out[f"altitude_{s_}"] = float(abs(e[0] * (V - U)[1] - e[1] * (V - U)[0]) / np.linalg.norm(e))
        d = (U - V) / np.linalg.norm(U - V) + (W - V) / np.linalg.norm(W - V)
        st = np.linalg.solve(np.column_stack([d, -e]), U - V)
        out[f"bisector_{s_}"] = float(st[0] * np.linalg.norm(d))
    return out


def parse_cevians(q0):
    q = _norm(q0)
    if not re.search(r"\btriangle\b", q) or not re.search(r"\b(medians?|altitudes?|heights|angle bisectors?|bisectors?)\b", q) or re.search(r"\(|vertices|\bbase\b", q):
        return None
    s = _three_sides(q)
    return _scoped(q, {"a": s[0], "b": s[1], "c": s[2]} if s else {})


_add("triangle_cevians", "intermediate",
     name="Medians, altitudes and angle bisectors of a triangle", topic="plane figures / triangles",
     purpose="The lengths of the three medians, three altitudes and three internal angle bisectors of a triangle from its sides.",
     inputs=[_I("a", "side a (opposite A)"), _I("b", "side b (opposite B)"), _I("c", "side c (opposite C)")],
     outputs=[_O(f"{k}_{s}", f"{k} from vertex {s.upper()} (to side {s})") for k in ("median", "altitude", "bisector") for s in "abc"],
     primary="median_a", conditions=["a, b, c > 0", "triangle inequality"],
     method="m_a = sqrt(2b^2 + 2c^2 - a^2)/2; h_a = 2K/a (Heron); t_a = sqrt(bc((b + c)^2 - a^2))/(b + c)",
     latex=r"m_a = \tfrac12\sqrt{2b^2+2c^2-a^2},\quad h_a = \frac{2K}{a},\quad t_a = \frac{\sqrt{bc\,[(b+c)^2-a^2]}}{b+c}",
     run=_cevians, verify=_verify_cevians, parse=parse_cevians, reference=_ref_cevians,
     wrong=lambda a, b, c: (lambda r: dict(r, median_a=2 * r["median_a"], median_b=2 * r["median_b"], median_c=2 * r["median_c"]))(_cevians(a, b, c)),
     random_inputs=lambda rng: (lambda a, b: {"a": a, "b": b, "c": round(rng.uniform(abs(a - b) + 0.5, a + b - 0.5), 2)})(
         round(rng.uniform(2, 40), 2), round(rng.uniform(2, 40), 2)),
     templates=["Find the medians of a triangle with sides 3, 4 and 5.", "Find the altitudes of a triangle with sides 13, 14 and 15."],
     examples=[{"inputs": {"a": 3, "b": 4, "c": 5}, "outputs": {"median_c": 2.5, "altitude_c": 2.4, "altitude_a": 4.0, "altitude_b": 3.0}},
               {"inputs": {"a": 13, "b": 14, "c": 15}, "outputs": {"altitude_b": 12.0, "median_b": math.sqrt(148)}}],
     questions=[{"q": "Find the medians of a triangle with sides 3, 4 and 5.", "outputs": {"median_c": 2.5}},
                {"q": "Find the altitudes of a triangle with sides 13, 14 and 15.", "outputs": {"altitude_b": 12.0}},
                {"q": "Find the angle bisectors of a triangle with sides 2, 3 and 9.", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# more plane figures and solids (beginner / intermediate)
# ==================================================================================================================
def annulus_area(outer, inner):
    R, r = _pos(outer, "outer radius"), _pos(inner, "inner radius")
    if r >= R:
        raise ValueError("the inner radius must be smaller than the outer radius")
    return {"area": math.pi * (R - r) * (R + r)}


def _ref_annulus(outer, inner):
    import sympy as sp
    rho = sp.Symbol("rho", positive=True)
    return {"area": _F(sp.integrate(2 * sp.pi * rho, (rho, _R(inner), _R(outer))))}


def parse_annulus(q0):
    q = _norm(q0)
    if not re.search(r"\b(annulus|ring|washer)\b", q) or re.search(r"\(|torus|sphere", q):
        return None
    R = _after(q, r"outer radius|external radius|larger radius")
    r = _after(q, r"inner radius|internal radius|smaller radius")
    if R is None and r is None:
        t = _two_after(q, r"radii")
        if t:
            R, r = max(t), min(t)
    return _scoped(q, _clean({"outer": R, "inner": r}))


_add("annulus_area", "beginner",
     name="Area of an annulus (ring)", topic="plane figures / circles",
     purpose="The area between two concentric circles.", inputs=[_I("outer", "outer radius"), _I("inner", "inner radius")],
     outputs=[_O("area", "pi (R^2 - r^2)")], primary="area", conditions=["0 < inner < outer"],
     method="A = pi (R - r)(R + r)", latex=r"A = \pi (R^2 - r^2)", run=annulus_area,
     verify=lambda i, o: _rel(o["area"], math.pi * i["outer"] ** 2 - math.pi * i["inner"] ** 2), parse=parse_annulus, reference=_ref_annulus,
     wrong=lambda outer, inner: {"area": math.pi * (outer - inner) ** 2},
     random_inputs=lambda rng: (lambda r: {"inner": r, "outer": round(r + rng.uniform(0.5, 30), 2)})(_rand_len(rng, 0.2, 30)),
     templates=["Find the area of an annulus with outer radius 5 and inner radius 3."],
     examples=[{"inputs": {"outer": 5, "inner": 3}, "outputs": {"area": 16 * math.pi}}],
     questions=[{"q": "Find the area of an annulus with outer radius 5 and inner radius 3.", "outputs": {"area": 16 * math.pi}},
                {"q": "A washer has an outer radius of 2 cm and an inner radius of 1 cm. What is its area?", "outputs": {"area": 3 * math.pi}},
                {"q": "Find the area of a ring with outer radius 3 and inner radius 5.", "outcome": "INVALID_INPUT"}])


def circular_segment(radius, angle):
    r, a = _pos(radius, "radius"), _fin(angle, "angle")
    if not 0 < a < 360:
        raise ValueError("the central angle must be between 0 and 360 degrees (exclusive)")
    t = math.radians(a)
    if t < 0.1:
        tms = t ** 3 / 6 - t ** 5 / 120 + t ** 7 / 5040 - t ** 9 / 362880 + t ** 11 / 39916800
    else:
        tms = t - math.sin(t)
    return {"area": r * r * tms / 2, "chord": 2 * r * math.sin(t / 2), "height": 2 * r * math.sin(t / 4) ** 2, "arc_length": r * t}


def _ref_segment(radius, angle):
    """Area between the chord and the arc, integrated across the chord (scipy.integrate.quad)."""
    from scipy.integrate import quad
    import numpy as np

    def minor(theta):                                       # theta < pi: the region above the chord y = r cos(theta/2)
        h = radius * np.sin(theta / 2)
        y0 = radius * np.cos(theta / 2)
        return quad(lambda x: np.sqrt(radius * radius - x * x) - y0, -h, h, epsabs=1e-14, epsrel=1e-14, limit=200)[0]
    t = np.radians(angle)
    area = minor(t) if t < np.pi else np.pi * radius ** 2 - minor(2 * np.pi - t)
    return {"area": float(area), "chord": float(2 * radius * np.sin(t / 2)), "arc_length": float(radius * t)}


def parse_segment(q0):
    q = _norm(q0)
    if not re.search(r"\bsegment\b", q) or not re.search(r"\b(circle|circular)\b", q) or re.search(r"\bline segment|segments|\(|intersect", q):
        return None
    return _scoped(q, _clean({"radius": _radius_of(q), "angle": _angle_in(q, r"angle|central angle")}))


_add("circular_segment", "intermediate",
     name="Circular segment: area, chord, height, arc", topic="plane figures / circles",
     purpose="Area, chord, height (sagitta) and arc length of the circular segment cut off by a chord with a given central angle.",
     inputs=[_I("radius", "radius"), _I("angle", "central angle in degrees")],
     outputs=[_O("area", "r^2 (theta - sin theta)/2"), _O("chord", "2 r sin(theta/2)"), _O("height", "r (1 - cos(theta/2))"), _O("arc_length", "r theta")],
     primary="area", conditions=["radius > 0", "0 < angle < 360 degrees"],
     method="A = r^2 (theta - sin theta)/2 (series for small theta to avoid cancellation); h = 2 r sin^2(theta/4)",
     latex=r"A = \tfrac12 r^2(\theta - \sin\theta),\quad c = 2r\sin\tfrac{\theta}{2},\quad h = r\left(1-\cos\tfrac{\theta}{2}\right)",
     run=circular_segment,
     verify=lambda i, o: (lambda r, t: _res((o["area"] + r * r * math.sin(t) / 2, r * r * t / 2), (o["chord"] ** 2, 2 * r * r * (1 - math.cos(t))),
                                            (o["height"], r - r * math.cos(t / 2)), (o["arc_length"], r * t)))(i["radius"], math.radians(i["angle"])),
     parse=parse_segment, reference=_ref_segment,
     wrong=lambda radius, angle: dict(circular_segment(radius, angle), area=radius ** 2 * (math.radians(angle) - math.sin(math.radians(angle)))),
     random_inputs=lambda rng: {"radius": _rand_len(rng, 0.5, 40), "angle": rng.choice([round(rng.uniform(5, 170), 1), round(rng.uniform(190, 355), 1)])},
     templates=["Find the area of the circular segment with radius 10 and central angle 60 degrees."],
     examples=[{"inputs": {"radius": 10, "angle": 60}, "outputs": {"area": 50 * (math.pi / 3 - math.sqrt(3) / 2), "chord": 10.0}},
               {"inputs": {"radius": 2, "angle": 180}, "outputs": {"area": 2 * math.pi, "height": 2.0}}],
     questions=[{"q": "Find the area of the circular segment with radius 10 and central angle 60 degrees.", "outputs": {"area": 50 * (math.pi / 3 - math.sqrt(3) / 2)}},
                {"q": "A segment of a circle of radius 2 has a central angle of 180 degrees. Find its area.", "outputs": {"area": 2 * math.pi}},
                {"q": "Find the area of the circular segment with radius 10.", "outcome": "NEEDS_INPUT"}])


def frustum_measures(r1, r2, height):
    R, r, h = _pos(r1, "r1"), _pos(r2, "r2"), _pos(height, "height")
    s = math.hypot(h, R - r)
    return {"volume": math.pi * h * (R * R + R * r + r * r) / 3, "slant_height": s, "lateral_area": math.pi * (R + r) * s,
            "total_area": math.pi * ((R + r) * s + R * R + r * r)}


def _ref_frustum(r1, r2, height):
    import sympy as sp
    z = sp.Symbol("z", real=True)
    R, r, h = _R(r1), _R(r2), _R(height)
    f = R + (r - R) * z / h
    vol = sp.integrate(sp.pi * f ** 2, (z, 0, h))
    lat = sp.integrate(2 * sp.pi * f * sp.sqrt(1 + sp.diff(f, z) ** 2), (z, 0, h))
    return {"volume": _F(vol), "lateral_area": _F(lat), "total_area": _F(lat + sp.pi * (R ** 2 + r ** 2)),
            "slant_height": _F(sp.Point(0, 0).distance(sp.Point(R - r, h)))}


def parse_frustum(q0):
    q = _norm(q0)
    if not re.search(r"\b(frustum|truncated cone)\b", q) or re.search(r"\(|pyramid", q):
        return None
    t = _two_after(q, r"radii")
    d = {"height": _after(q, r"height|tall|high")}
    if t:
        d.update(r1=t[0], r2=t[1])
    else:
        d.update(r1=_after(q, r"bottom radius|base radius|larger radius|lower radius"), r2=_after(q, r"top radius|smaller radius|upper radius"))
    return _scoped(q, _clean(d))


_add("frustum_measures", "beginner",
     name="Frustum of a cone: volume and surface area", topic="solids / cones",
     purpose="Volume, slant height, lateral and total surface area of a frustum of a right circular cone.",
     inputs=[_I("r1", "radius of one base"), _I("r2", "radius of the other base"), _I("height", "perpendicular height")],
     outputs=[_O("volume", "pi h (R^2 + R r + r^2)/3"), _O("slant_height", "sqrt(h^2 + (R - r)^2)"), _O("lateral_area", "pi (R + r) s"),
              _O("total_area", "lateral + pi (R^2 + r^2)")], primary="volume", conditions=["r1, r2, height > 0"],
     method="V = pi h (R^2 + R r + r^2)/3; s = sqrt(h^2 + (R - r)^2); lateral = pi (R + r) s",
     latex=r"V = \tfrac13\pi h (R^2 + Rr + r^2),\quad A_L = \pi (R+r)\sqrt{h^2+(R-r)^2}",
     run=frustum_measures,
     verify=lambda i, o: (lambda R, r, h: _res((3 * o["volume"], math.pi * h * (R * R + R * r + r * r)), (o["slant_height"] ** 2, h * h + (R - r) ** 2),
                                               (o["lateral_area"], math.pi * (R + r) * o["slant_height"]), (o["total_area"], o["lateral_area"] + math.pi * (R * R + r * r))))(
         i["r1"], i["r2"], i["height"]),
     parse=parse_frustum, reference=_ref_frustum,
     wrong=lambda r1, r2, height: dict(frustum_measures(r1, r2, height), volume=math.pi * height * (r1 ** 2 + r2 ** 2) / 2),
     random_inputs=lambda rng: {"r1": _rand_len(rng, 0.5, 30), "r2": _rand_len(rng, 0.5, 30), "height": _rand_len(rng, 0.5, 40)},
     templates=["Find the volume of a frustum with radii 3 and 6 and height 4."],
     examples=[{"inputs": {"r1": 6, "r2": 3, "height": 4}, "outputs": {"volume": 84 * math.pi, "slant_height": 5.0, "lateral_area": 45 * math.pi, "total_area": 90 * math.pi}}],
     questions=[{"q": "Find the volume of a frustum with radii 3 and 6 and height 4.", "outputs": {"volume": 84 * math.pi, "slant_height": 5.0}},
                {"q": "A truncated cone has bottom radius 6, top radius 3 and height 4. Find its total surface area.", "outputs": {"total_area": 90 * math.pi}},
                {"q": "Find the volume of a frustum with radii 3 and 6.", "outcome": "NEEDS_INPUT"}])


def pyramid_measures(length, width, height):
    l, w, h = _pos(length, "length"), _pos(width, "width"), _pos(height, "height")
    lat = l * math.hypot(h, w / 2) + w * math.hypot(h, l / 2)
    return {"volume": l * w * h / 3, "lateral_area": lat, "total_area": lat + l * w}


def _ref_pyramid(length, width, height):
    from scipy.spatial import ConvexHull
    pts = [(0, 0, 0), (length, 0, 0), (length, width, 0), (0, width, 0), (length / 2, width / 2, height)]
    H = ConvexHull(pts)
    return {"volume": float(H.volume), "total_area": float(H.area), "lateral_area": float(H.area) - length * width}


def parse_pyramid(q0):
    q = _norm(q0)
    if not re.search(r"\bpyramid\b", q) or re.search(r"\(|frustum|truncated|triangular|hexagonal|pentagonal", q):
        return None
    d = {"height": _after(q, r"height|tall|high|altitude")}
    m = re.search(r"(" + _N + r")\s*(?:[a-z]{1,6}\s*)?(?:by|x)\s*(" + _N + r")", q)
    if re.search(r"\bsquare\b", q):
        s = _after(q, r"side|base side|edge|base edge|base")
        d.update(length=s, width=s)
    elif m:
        d.update(length=float(m.group(1)), width=float(m.group(2)))
    else:
        d.update(length=_after(q, r"length"), width=_after(q, r"width"))
    return _scoped(q, _clean(d))


_add("pyramid_measures", "beginner",
     name="Right rectangular (or square) pyramid: volume and surface area", topic="solids / pyramids",
     purpose="Volume, lateral area and total surface area of a right pyramid on a rectangular base (apex above the base centre).",
     inputs=[_I("length", "base length"), _I("width", "base width"), _I("height", "perpendicular height")],
     outputs=[_O("volume", "l w h / 3"), _O("lateral_area", "area of the four triangular faces"), _O("total_area", "lateral + base")],
     primary="volume", conditions=["length, width, height > 0", "the apex is directly above the centre of the base"],
     method="V = lwh/3; each pair of faces has slant height sqrt(h^2 + (other side/2)^2)",
     latex=r"V = \tfrac13 lwh,\quad A_L = l\sqrt{h^2+(w/2)^2} + w\sqrt{h^2+(l/2)^2}",
     run=pyramid_measures,
     verify=lambda i, o: (lambda l, w, h: _res((3 * o["volume"], l * w * h), (o["total_area"], o["lateral_area"] + l * w),
                                               (o["lateral_area"], 2 * (0.5 * l * math.sqrt(h * h + w * w / 4)) + 2 * (0.5 * w * math.sqrt(h * h + l * l / 4)))))(
         i["length"], i["width"], i["height"]),
     parse=parse_pyramid, reference=_ref_pyramid,
     wrong=lambda length, width, height: (lambda lat: {"volume": length * width * height / 2, "lateral_area": lat, "total_area": lat + length * width})(
         2 * length * height),
     random_inputs=lambda rng: {"length": _rand_len(rng, 0.5, 40), "width": _rand_len(rng, 0.5, 40), "height": _rand_len(rng, 0.5, 40)},
     templates=["Find the volume of a square pyramid with base side 6 and height 4.", "A pyramid has a 6 by 8 rectangular base and height 12. Find its volume."],
     examples=[{"inputs": {"length": 6, "width": 6, "height": 4}, "outputs": {"volume": 48.0, "lateral_area": 60.0, "total_area": 96.0}},
               {"inputs": {"length": 6, "width": 8, "height": 12}, "outputs": {"volume": 192.0}}],
     questions=[{"q": "Find the volume of a square pyramid with base side 6 and height 4.", "outputs": {"volume": 48.0, "total_area": 96.0}},
                {"q": "A pyramid has a 6 by 8 rectangular base and height 12. Find its volume.", "outputs": {"volume": 192.0}},
                {"q": "Find the volume of a square pyramid with base side 6.", "outcome": "NEEDS_INPUT"}])


_PRISM_NAMES = {"triangular": 3, "square": 4, "pentagonal": 5, "hexagonal": 6, "heptagonal": 7, "octagonal": 8, "decagonal": 10}


def _turtle(n, s):
    pts, x, y, h = [], 0.0, 0.0, 0.0
    for _ in range(int(n)):
        pts.append((x, y))
        x, y, h = x + s * math.cos(h), y + s * math.sin(h), h + 2 * math.pi / int(n)
    return pts


def regular_prism(n, side, height):
    if isinstance(n, float):
        if n != int(n):
            raise ValueError("n must be a whole number")
        n = int(n)
    if n < 3 or n > 200:
        raise ValueError("the base has between 3 and 200 sides here")
    s, h = _pos(side, "side"), _pos(height, "height")
    B = n * s * s / (4 * math.tan(math.pi / n))
    return {"volume": B * h, "lateral_area": n * s * h, "total_area": n * s * h + 2 * B, "base_area": B}


def _ref_prism(n, side, height):
    from scipy.spatial import ConvexHull
    base = _turtle(n, side)
    H = ConvexHull([(x, y, 0.0) for x, y in base] + [(x, y, height) for x, y in base])
    from shapely.geometry import Polygon
    B = Polygon(base).area
    return {"volume": float(H.volume), "total_area": float(H.area), "lateral_area": float(H.area) - 2 * B, "base_area": B}


def parse_prism(q0):
    q = _norm(q0)
    if not re.search(r"\bprism\b", q) or re.search(r"\(|rectangular|pyramid", q):
        return None
    n = None
    for name, k in _PRISM_NAMES.items():
        if re.search(r"\b" + name + r" prism\b", q):
            n = k
    if n is None:
        n = _n_sides(q)
    if n is None or (n == 4 and not re.search(r"\bsquare prism\b", q)) and not re.search(r"\bregular\b", q):
        return None if n is None and not re.search(r"\bregular\b", q) else {"height": _after(q, r"height|tall|high|length")}
    if n == 3 and not re.search(r"\b(regular|equilateral)\b", q):
        return None
    side = _after(q, r"side|base side|edge|base edge|side length")
    return _scoped(q, _clean({"n": n, "side": side, "height": _after(q, r"height|tall|high|length")}))


_add("regular_prism", "beginner",
     name="Right prism on a regular polygon base: volume and surface area", topic="solids / prisms",
     purpose="Volume, base area, lateral and total surface area of a right prism whose base is a regular n-gon.",
     inputs=[{"symbol": "n", "kind": "number", "type": "integer", "meaning": "number of sides of the base", "required": True, "min": 3, "max": 200},
             _I("side", "side length of the base"), _I("height", "height (length) of the prism")],
     outputs=[_O("volume", "base area * height"), _O("lateral_area", "n s h"), _O("total_area", "lateral + 2 * base"), _O("base_area", "n s^2 / (4 tan(pi/n))")],
     primary="volume", conditions=["3 <= n <= 200", "side, height > 0", "a triangular prism is read only when 'regular' or 'equilateral' is said"],
     method="base area n s^2/(4 tan(pi/n)); V = B h; lateral = perimeter * h",
     latex=r"B = \frac{n s^2}{4\tan(\pi/n)},\quad V = Bh,\quad A = nsh + 2B", run=regular_prism,
     verify=lambda i, o: _res((o["base_area"] * 4 * math.tan(math.pi / i["n"]), i["n"] * i["side"] ** 2), (o["volume"], o["base_area"] * i["height"]),
                              (o["lateral_area"], i["n"] * i["side"] * i["height"]), (o["total_area"], o["lateral_area"] + 2 * o["base_area"])),
     parse=parse_prism, reference=_ref_prism,
     wrong=lambda n, side, height: dict(regular_prism(n, side, height), total_area=regular_prism(n, side, height)["lateral_area"] + regular_prism(n, side, height)["base_area"]),
     random_inputs=lambda rng: {"n": rng.randint(3, 12), "side": _rand_len(rng, 0.5, 20), "height": _rand_len(rng, 0.5, 40)},
     templates=["Find the volume of a regular hexagonal prism with side 2 and height 10.",
                "Find the total surface area of an equilateral triangular prism with side 4 and length 10."],
     examples=[{"inputs": {"n": 6, "side": 2, "height": 10}, "outputs": {"volume": 60 * math.sqrt(3), "lateral_area": 120.0}},
               {"inputs": {"n": 3, "side": 4, "height": 10}, "outputs": {"total_area": 120 + 8 * math.sqrt(3)}}],
     questions=[{"q": "Find the volume of a regular hexagonal prism with side 2 and height 10.", "outputs": {"volume": 60 * math.sqrt(3)}},
                {"q": "Find the total surface area of an equilateral triangular prism with side 4 and length 10.", "outputs": {"total_area": 120 + 8 * math.sqrt(3)}}])


_PHI = (1 + math.sqrt(5)) / 2
_PLATONIC = {   # V, E, F, volume/a^3, surface/a^2, circumradius/a, inradius/a
    "tetrahedron": (4, 6, 4, 1 / (6 * math.sqrt(2)), math.sqrt(3), math.sqrt(6) / 4, math.sqrt(6) / 12),
    "cube": (8, 12, 6, 1.0, 6.0, math.sqrt(3) / 2, 0.5),
    "octahedron": (6, 12, 8, math.sqrt(2) / 3, 2 * math.sqrt(3), 1 / math.sqrt(2), 1 / math.sqrt(6)),
    "dodecahedron": (20, 30, 12, (15 + 7 * math.sqrt(5)) / 4, 3 * math.sqrt(25 + 10 * math.sqrt(5)), math.sqrt(3) * _PHI / 2,
                     0.5 * math.sqrt((25 + 11 * math.sqrt(5)) / 10)),
    "icosahedron": (12, 30, 20, 5 * (3 + math.sqrt(5)) / 12, 5 * math.sqrt(3), math.sqrt(10 + 2 * math.sqrt(5)) / 4, math.sqrt(3) * (3 + math.sqrt(5)) / 12),
}


def platonic_solid(solid, edge):
    if solid not in _PLATONIC:
        raise ValueError("solid must be one of " + ", ".join(_PLATONIC))
    a = _pos(edge, "edge")
    V, E, F, kv, ks, kR, kr = _PLATONIC[solid]
    return {"vertices": V, "edges": E, "faces": F, "euler_characteristic": V - E + F, "volume": kv * a ** 3,
            "surface_area": ks * a * a, "circumradius": kR * a, "inradius": kr * a}


def _ref_platonic(solid, edge):
    """Canonical vertex coordinates, scipy's convex hull: faces = distinct facet planes, edges = ridges between them."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from scipy.spatial.distance import pdist
    p, ip = _PHI, 1 / _PHI
    cyc = lambda t: [t, (t[1], t[2], t[0]), (t[2], t[0], t[1])]
    if solid == "tetrahedron":
        P = [(1, 1, 1), (1, -1, -1), (-1, 1, -1), (-1, -1, 1)]
    elif solid == "cube":
        P = [(x, y, z) for x in (-1, 1) for y in (-1, 1) for z in (-1, 1)]
    elif solid == "octahedron":
        P = [(s, 0, 0) for s in (-1, 1)] + [(0, s, 0) for s in (-1, 1)] + [(0, 0, s) for s in (-1, 1)]
    elif solid == "icosahedron":
        P = [v for s1 in (-1, 1) for s2 in (-1, 1) for v in cyc((0, s1, s2 * p))]
    else:
        P = [(x, y, z) for x in (-1, 1) for y in (-1, 1) for z in (-1, 1)] + \
            [v for s1 in (-1, 1) for s2 in (-1, 1) for v in cyc((0, s1 * ip, s2 * p))]
    P = np.array(P, float)
    P *= edge / pdist(P).min()
    H = ConvexHull(P)
    planes = np.round(H.equations / np.abs(H.equations).max(), 9)
    key = [tuple(r) for r in planes]
    faces = len(set(key))
    ridges = set()
    for i, nb in enumerate(H.neighbors):
        for k, j in enumerate(nb):
            if key[i] != key[j]:
                ridges.add(tuple(sorted(np.delete(H.simplices[i], k))))
    V, E = len(H.vertices), len(ridges)
    return {"vertices": V, "edges": E, "faces": faces, "euler_characteristic": V - E + faces, "volume": float(H.volume),
            "surface_area": float(H.area), "circumradius": float(np.linalg.norm(P, axis=1).max()), "inradius": float((-H.equations[:, 3]).min())}


def _verify_platonic(i, o):
    V, E, F, kv, ks, kR, kr = _PLATONIC[i["solid"]]
    a = i["edge"]
    if (o["vertices"], o["edges"], o["faces"], o["euler_characteristic"]) != (V, E, F, 2):
        return 1.0
    # every face touches the insphere: V = r S / 3; the circumsphere through every vertex: R / r is the solid's ratio
    return _res((3 * o["volume"], o["inradius"] * o["surface_area"]), (o["circumradius"], kR * a), (o["surface_area"], ks * a * a))


def parse_platonic(q0):
    q = _norm(q0)
    names = [s for s in _PLATONIC if re.search(r"\b" + s + r"\b", q)]
    if len(names) != 1 or re.search(r"\(|vertices at", q):
        return None
    if names[0] == "cube" and not re.search(r"\b(circumradius|inradius|insphere|circumsphere|inscribed sphere|circumscribed sphere|"
                                             r"faces|vertices|edges|euler)\b", q):
        return None
    return _scoped(q, _clean({"solid": names[0], "edge": _after(q, r"edge|edge length|side|side length|edges of length")}))


_add("platonic_solid", "advanced",
     name="Platonic solids: counts, Euler characteristic, volume, surface, radii", topic="polyhedra / regular polyhedra",
     purpose="Vertices, edges, faces, Euler characteristic, volume, surface area, circumradius and inradius of a regular polyhedron of edge a.",
     inputs=[{"symbol": "solid", "kind": "text", "type": "choice", "choices": list(_PLATONIC), "meaning": "which Platonic solid", "required": True},
             _I("edge", "edge length a")],
     outputs=[_O("vertices", "V"), _O("edges", "E"), _O("faces", "F"), _O("euler_characteristic", "V - E + F"), _O("volume", "volume"),
              _O("surface_area", "surface area"), _O("circumradius", "radius of the circumscribed sphere"), _O("inradius", "radius of the inscribed sphere")],
     primary="volume", conditions=["edge > 0"],
     method="closed forms in the edge length (golden ratio for the dodecahedron and icosahedron); Euler: V - E + F = 2",
     latex=r"V - E + F = 2,\quad V_{\text{ico}} = \tfrac{5}{12}(3+\sqrt5)\,a^3", run=platonic_solid, verify=_verify_platonic, parse=parse_platonic,
     reference=_ref_platonic, wrong=lambda solid, edge: dict(platonic_solid(solid, edge), volume=2 * platonic_solid(solid, edge)["volume"]),
     random_inputs=lambda rng: {"solid": rng.choice(list(_PLATONIC)), "edge": _rand_len(rng, 0.2, 20)},
     templates=["Find the volume and surface area of a regular icosahedron with edge 2.", "What is the circumradius of a cube with edge 2?",
                "How many faces, edges and vertices does a dodecahedron with edge 1 have?"],
     examples=[{"inputs": {"solid": "cube", "edge": 2}, "outputs": {"volume": 8.0, "circumradius": math.sqrt(3), "inradius": 1.0, "euler_characteristic": 2}},
               {"inputs": {"solid": "octahedron", "edge": 1}, "outputs": {"vertices": 6, "edges": 12, "faces": 8, "volume": math.sqrt(2) / 3}},
               {"inputs": {"solid": "tetrahedron", "edge": 6}, "outputs": {"volume": 18 * math.sqrt(2), "surface_area": 36 * math.sqrt(3)}}],
     questions=[{"q": "Find the volume and surface area of a regular icosahedron with edge 2.", "outputs": {"volume": 10 * (3 + math.sqrt(5)) / 3, "surface_area": 20 * math.sqrt(3)}},
                {"q": "What is the circumradius of a cube with edge 2?", "outputs": {"circumradius": math.sqrt(3)}},
                {"q": "How many faces, edges and vertices does a dodecahedron have?", "outcome": "NEEDS_INPUT"},
                {"q": "Find the volume of a regular tetrahedron with edge 6.", "outputs": {"volume": 18 * math.sqrt(2)}}])


def spherical_cap(radius, height):
    R, h = _pos(radius, "radius"), _pos(height, "height")
    if h > 2 * R:
        raise ValueError("the cap height cannot exceed the sphere's diameter")
    return {"volume": math.pi * h * h * (3 * R - h) / 3, "cap_area": 2 * math.pi * R * h, "base_radius": math.sqrt(h * (2 * R - h))}


def _ref_cap(radius, height):
    import sympy as sp
    z = sp.Symbol("z", real=True)
    R, h = _R(radius), _R(height)
    th, ph = sp.symbols("theta phi", real=True)
    vol = sp.integrate(sp.pi * (R ** 2 - z ** 2), (z, R - h, R))              # stacked discs
    alpha = sp.acos((R - h) / R)                                              # polar angle of the cap's rim
    area = sp.integrate(sp.integrate(R ** 2 * sp.sin(th), (th, 0, alpha)), (ph, 0, 2 * sp.pi))
    return {"volume": _F(vol), "cap_area": _F(area), "base_radius": _F(R * sp.sin(alpha))}


def parse_cap(q0):
    q = _norm(q0)
    if not re.search(r"\b(spherical cap|cap of a sphere|dome)\b", q) or re.search(r"\(", q):
        return None
    return _scoped(q, _clean({"radius": _after(q, r"sphere radius|radius of the sphere|radius"), "height": _after(q, r"height|cap height|tall|high")}))


_add("spherical_cap", "intermediate",
     name="Spherical cap: volume, curved area, base radius", topic="solids / spheres",
     purpose="Volume, curved surface area and base radius of a cap of height h cut from a sphere of radius R.",
     inputs=[_I("radius", "radius R of the sphere"), _I("height", "height h of the cap")],
     outputs=[_O("volume", "pi h^2 (3R - h)/3"), _O("cap_area", "2 pi R h"), _O("base_radius", "sqrt(h (2R - h))")], primary="volume",
     conditions=["R > 0", "0 < h <= 2R"], method="V = pi h^2 (3R - h)/3; curved area 2 pi R h (Archimedes)",
     latex=r"V = \tfrac13 \pi h^2(3R - h),\quad A = 2\pi R h,\quad a = \sqrt{h(2R-h)}", run=spherical_cap,
     verify=lambda i, o: (lambda R, h: _res((3 * o["volume"], math.pi * h * h * (3 * R - h)), (o["cap_area"], 2 * math.pi * R * h),
                                            (o["base_radius"] ** 2 + (R - h) ** 2, R * R)))(i["radius"], i["height"]),
     parse=parse_cap, reference=_ref_cap,
     wrong=lambda radius, height: dict(spherical_cap(radius, height), volume=math.pi * height ** 2 * (3 * radius - height)),
     random_inputs=lambda rng: (lambda R: {"radius": R, "height": round(rng.uniform(0.05, 1.95) * R, 3)})(_rand_len(rng, 0.5, 40)),
     templates=["Find the volume of a spherical cap of height 2 cut from a sphere of radius 5."],
     examples=[{"inputs": {"radius": 5, "height": 2}, "outputs": {"volume": 52 * math.pi / 3, "cap_area": 20 * math.pi, "base_radius": 4.0}},
               {"inputs": {"radius": 3, "height": 3}, "outputs": {"volume": 18 * math.pi, "cap_area": 18 * math.pi}}],
     questions=[{"q": "Find the volume of a spherical cap of height 2 cut from a sphere of radius 5.", "outputs": {"volume": 52 * math.pi / 3, "base_radius": 4.0}},
                {"q": "Find the volume of a spherical cap of height 12 on a sphere of radius 5.", "outcome": "INVALID_INPUT"}])


def torus_measures(major, minor):
    R, r = _pos(major, "major radius"), _pos(minor, "minor radius")
    if r >= R:
        raise ValueError("the tube radius must be smaller than the major radius (a ring torus)")
    return {"volume": 2 * math.pi ** 2 * R * r * r, "surface_area": 4 * math.pi ** 2 * R * r}


def _ref_torus(major, minor):
    import sympy as sp
    u, v, rho = sp.symbols("u v rho", real=True)
    R, r = _R(major), _R(minor)
    area = sp.integrate(sp.integrate(r * (R + r * sp.cos(v)), (v, 0, 2 * sp.pi)), (u, 0, 2 * sp.pi))
    vol = sp.integrate(sp.integrate(sp.integrate(rho * (R + rho * sp.cos(v)), (rho, 0, r)), (v, 0, 2 * sp.pi)), (u, 0, 2 * sp.pi))
    return {"volume": _F(vol), "surface_area": _F(area)}


def parse_torus(q0):
    q = _norm(q0)
    if not re.search(r"\b(torus|doughnut|donut)\b", q) or re.search(r"\b(curvature|gaussian|mean)\b|\(", q):
        return None
    return _scoped(q, _clean({"major": _after(q, r"major radius|central radius|big radius|distance from the centre of the tube to the centre|r1"),
                              "minor": _after(q, r"minor radius|tube radius|small radius|r2")}))


_add("torus_measures", "intermediate",
     name="Torus: volume and surface area", topic="solids / solids of revolution",
     purpose="Volume and surface area of a ring torus with major radius R (centre to tube centre) and tube radius r.",
     inputs=[_I("major", "major radius R"), _I("minor", "tube (minor) radius r")],
     outputs=[_O("volume", "2 pi^2 R r^2"), _O("surface_area", "4 pi^2 R r")], primary="volume", conditions=["0 < r < R"],
     method="Pappus's theorems: area/volume of the revolved circle times the path 2 pi R", latex=r"V = 2\pi^2 R r^2,\quad A = 4\pi^2 R r",
     run=torus_measures,
     verify=lambda i, o: _res((o["volume"], (math.pi * i["minor"] ** 2) * (2 * math.pi * i["major"])), (o["surface_area"], 2 * o["volume"] / i["minor"])),
     parse=parse_torus, reference=_ref_torus, wrong=lambda major, minor: {"volume": math.pi * major * minor ** 2, "surface_area": 4 * math.pi * major * minor},
     random_inputs=lambda rng: (lambda R: {"major": R, "minor": round(rng.uniform(0.05, 0.95) * R, 3)})(_rand_len(rng, 1, 40)),
     templates=["Find the volume of a torus with major radius 5 and minor radius 2."],
     examples=[{"inputs": {"major": 5, "minor": 2}, "outputs": {"volume": 40 * math.pi ** 2, "surface_area": 40 * math.pi ** 2}}],
     questions=[{"q": "Find the volume of a torus with major radius 5 and minor radius 2.", "outputs": {"volume": 40 * math.pi ** 2}},
                {"q": "A doughnut has major radius 3 cm and tube radius 1 cm. What is its surface area?", "outputs": {"surface_area": 12 * math.pi ** 2}},
                {"q": "Find the volume of a torus with major radius 2 and minor radius 5.", "outcome": "INVALID_INPUT"}])


def spheroid_measures(a, c):
    a, c = _pos(a, "a"), _pos(c, "c")
    if a == c:
        S = 4 * math.pi * a * a
    elif c < a:                                              # oblate
        e = math.sqrt(1 - (c / a) ** 2)
        S = 2 * math.pi * a * a * (1 + (1 - e * e) / e * math.atanh(e))
    else:                                                    # prolate
        e = math.sqrt(1 - (a / c) ** 2)
        S = 2 * math.pi * a * a * (1 + c / (a * e) * math.asin(e))
    return {"surface_area": S, "volume": 4 * math.pi * a * a * c / 3, "shape": "sphere" if a == c else ("oblate" if c < a else "prolate")}


def _ref_spheroid(a, c):
    from scipy.integrate import quad
    import numpy as np
    S = quad(lambda t: 2 * np.pi * a * np.cos(t) * np.sqrt(a * a * np.sin(t) ** 2 + c * c * np.cos(t) ** 2), -np.pi / 2, np.pi / 2,
             epsabs=0, epsrel=2e-14, limit=200)[0]
    V = quad(lambda z: np.pi * a * a * (1 - (z / c) ** 2), -c, c, epsabs=0, epsrel=2e-14)[0]
    return {"surface_area": S, "volume": V}


def _verify_spheroid(i, o):
    """Trapezoidal rule on the periodic meridian integral (exponentially accurate for a smooth periodic integrand)."""
    a, c = i["a"], i["c"]
    n = 4096                                                 # composite Simpson over the meridian, t in [-pi/2, pi/2]
    h = math.pi / n
    f = lambda t: math.cos(t) * math.sqrt((a * math.sin(t)) ** 2 + (c * math.cos(t)) ** 2)
    s = math.fsum((1 if k in (0, n) else (4 if k % 2 else 2)) * f(-math.pi / 2 + k * h) for k in range(n + 1))
    S = 2 * math.pi * a * s * h / 3
    return max(_rel(o["surface_area"], S), _rel(o["volume"], 4 * math.pi * a * a * c / 3))


def parse_spheroid(q0):
    q = _norm(q0)
    if not re.search(r"\b(spheroid|oblate|prolate|ellipsoid of revolution)\b", q) or re.search(r"\(", q):
        return None
    return _scoped(q, _clean({"a": _after(q, r"equatorial radius|equatorial semi-axis|semi-axis a|a"),
                              "c": _after(q, r"polar radius|polar semi-axis|semi-axis c|c")}))


_add("spheroid_measures", "advanced",
     name="Spheroid (ellipsoid of revolution): surface area and volume", topic="solids / ellipsoids",
     purpose="Surface area and volume of an oblate or prolate spheroid with equatorial radius a and polar radius c.",
     inputs=[_I("a", "equatorial radius (semi-axis in the equator)"), _I("c", "polar radius (semi-axis along the axis)")],
     outputs=[_O("surface_area", "surface area"), _O("volume", "4/3 pi a^2 c"), _O("shape", "oblate / prolate / sphere", "text")], primary="surface_area",
     conditions=["a, c > 0"],
     method="oblate: S = 2 pi a^2 (1 + (1 - e^2)/e atanh e); prolate: S = 2 pi a^2 (1 + c/(a e) asin e); e the eccentricity",
     latex=r"S_{\text{obl}} = 2\pi a^2\left(1 + \frac{1-e^2}{e}\operatorname{artanh} e\right),\quad S_{\text{pro}} = 2\pi a^2\left(1 + \frac{c}{ae}\arcsin e\right)",
     run=spheroid_measures, verify=_verify_spheroid, parse=parse_spheroid, reference=_ref_spheroid,
     wrong=lambda a, c: dict(spheroid_measures(a, c), surface_area=4 * math.pi * a * c),
     random_inputs=lambda rng: {"a": _rand_len(rng, 0.5, 30), "c": _rand_len(rng, 0.5, 30)},
     templates=["Find the surface area of an oblate spheroid with equatorial radius 6378 and polar radius 6357.",
                "Find the surface area of a prolate spheroid with a = 1 and c = 2."],
     examples=[{"inputs": {"a": 1, "c": 2}, "outputs": {"surface_area": 2 * math.pi * (1 + 2 / (math.sqrt(3) / 2) * math.asin(math.sqrt(3) / 2)), "volume": 8 * math.pi / 3}},
               {"inputs": {"a": 3, "c": 3}, "outputs": {"surface_area": 36 * math.pi, "shape": "sphere"}}],
     questions=[{"q": "Find the surface area of a prolate spheroid with a = 1 and c = 2.", "outputs": {"surface_area": 2 * math.pi * (1 + 4 / math.sqrt(3) * math.pi / 3)}},
                {"q": "Find the volume of an oblate spheroid with equatorial radius 2 and polar radius 1.", "outputs": {"volume": 16 * math.pi / 3}}])


def ellipse_perimeter(a, b):
    a, b = _pos(a, "a"), _pos(b, "b")
    A, B = max(a, b), min(a, b)
    if B / A < 1e-3:
        raise ValueError("the axis ratio must be at least 0.001")
    m = 1 - (B / A) ** 2
    an, gn = 1.0, math.sqrt(1 - m)
    s, p = m / 2, 0.5                                        # c_0^2 / 2 with c_0^2 = m
    for _ in range(40):
        cn = (an - gn) / 2
        an, gn = (an + gn) / 2, math.sqrt(an * gn)
        p *= 2
        s += p * cn * cn
        if cn < 1e-17:
            break
    E = math.pi / (2 * an) * (1 - s)
    return {"perimeter": 4 * A * E, "area": math.pi * a * b}


def _verify_ellipse_perimeter(i, o):
    a, b = i["a"], i["b"]
    n = 4096                                                 # periodic trapezoidal rule: exponentially accurate
    s = math.fsum(math.sqrt((a * math.sin(2 * math.pi * k / n)) ** 2 + (b * math.cos(2 * math.pi * k / n)) ** 2) for k in range(n))
    P = s * 2 * math.pi / n
    return _res((o["perimeter"], P), (o["area"], math.pi * a * b))


def parse_ellipse_perimeter(q0):
    q = _norm(q0)
    if not re.search(r"\bellipse\b|\bellipt", q) or not re.search(r"\b(perimeter|circumference)\b", q) or re.search(r"\(", q):
        return None
    a = _after(q, r"semi-major axis|semimajor axis|semi-axis a|a")
    b = _after(q, r"semi-minor axis|semiminor axis|semi-axis b|b")
    m = re.search(r"x\^2\s*/\s*(" + _N + r")\s*\+\s*y\^2\s*/\s*(" + _N + r")\s*=\s*1\b", q)
    if m:
        a, b = math.sqrt(float(m.group(1))), math.sqrt(float(m.group(2)))
    return _scoped(q, _clean({"a": a, "b": b}))


_add("ellipse_perimeter", "expert",
     name="Perimeter of an ellipse (exact, by the complete elliptic integral)", topic="conic sections / ellipses",
     purpose="The exact perimeter (circumference) and area of an ellipse with semi-axes a and b.",
     inputs=[_I("a", "one semi-axis"), _I("b", "the other semi-axis")],
     outputs=[_O("perimeter", "4 a E(e^2), a the larger semi-axis"), _O("area", "pi a b")], primary="perimeter",
     conditions=["a, b > 0", "axis ratio at least 0.001"],
     method="complete elliptic integral of the second kind by the arithmetic-geometric mean (Gauss), quadratically convergent",
     latex=r"P = 4a\,E(e),\quad E(m) = \frac{\pi}{2\,\mathrm{AGM}(1,\sqrt{1-m})}\Big(1 - \sum_{n\ge0} 2^{n-1} c_n^2\Big)",
     run=ellipse_perimeter, verify=_verify_ellipse_perimeter, parse=parse_ellipse_perimeter,
     reference=lambda a, b: {"perimeter": 4 * max(a, b) * float(__import__("scipy.special", fromlist=["ellipe"]).ellipe(1 - (min(a, b) / max(a, b)) ** 2)),
                             "area": math.pi * a * b},
     wrong=lambda a, b: {"perimeter": 2 * math.pi * math.sqrt((a * a + b * b) / 2) * 1.01 if a != b else 2 * math.pi * a * 1.01, "area": math.pi * a * b},
     random_inputs=lambda rng: {"a": _rand_len(rng, 0.5, 40), "b": _rand_len(rng, 0.5, 40)},
     templates=["Find the perimeter of an ellipse with semi-major axis 5 and semi-minor axis 3.", "What is the circumference of the ellipse x^2/25 + y^2/9 = 1?"],
     examples=[{"inputs": {"a": 2, "b": 2}, "outputs": {"perimeter": 4 * math.pi, "area": 4 * math.pi}},
               {"inputs": {"a": 5, "b": 3}, "outputs": {"perimeter": 25.526998863398131, "area": 15 * math.pi}}],
     questions=[{"q": "Find the perimeter of an ellipse with semi-major axis 5 and semi-minor axis 3.", "outputs": {"perimeter": 25.526998863398131}},
                {"q": "What is the circumference of the ellipse x^2/25 + y^2/9 = 1?", "outputs": {"perimeter": 25.526998863398131}}])


# ==================================================================================================================
# solid coordinate geometry (intermediate / advanced / expert)
# ==================================================================================================================
def _vsub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def _vcross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def _vdot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def _vnorm(a):
    return math.sqrt(_vdot(a, a))


def _xvec(p):
    return tuple(_fr(c) for c in p)


def triangle_area_3d(points):
    A, B, C = _triples(points, "points", 3)
    if _vcross(_vsub(_xvec(B), _xvec(A)), _vsub(_xvec(C), _xvec(A))) == (0, 0, 0):
        raise ValueError("the three points are collinear")
    a, b, c = math.dist(B, C), math.dist(A, C), math.dist(A, B)
    x, y, z = sorted((a, b, c), reverse=True)
    return {"area": 0.25 * math.sqrt((x + (y + z)) * (z - (x - y)) * (z + (x - y)) * (x + (y - z))), "perimeter": a + b + c}


def _rand3(rng):
    return (_RANDP(rng), _RANDP(rng), _RANDP(rng))


def _rand_tri3(rng):
    while True:
        P = [_rand3(rng) for _ in range(3)]
        n = _vnorm(_vcross(_vsub(P[1], P[0]), _vsub(P[2], P[0])))
        if n > 0.05 * max(math.dist(P[0], P[1]), math.dist(P[1], P[2]), math.dist(P[0], P[2])) ** 2:
            return {"points": _flat(P)}


def parse_tri3(q0):
    q = _norm(q0)
    p = _pts3(q)
    if not re.search(r"\btriangle\b", q) or not re.search(r"\b(area|perimeter)\b", q) or not p or re.search(r"solid angle", q):
        return None
    return {"points": _flat(p)} if len(p) == 3 else None


_add("triangle_area_3d", "intermediate",
     name="Triangle in space: area and perimeter from vertices", topic="solid coordinate geometry / triangles",
     purpose="Area and perimeter of the triangle with vertices A, B, C in three-dimensional space.",
     inputs=[_V("points", "the vertices: xA yA zA, xB yB zB, xC yC zC", n_min=9, n_max=9)],
     outputs=[_O("area", "area"), _O("perimeter", "perimeter")], primary="area", conditions=["exactly 3 points, not collinear"],
     method="side lengths, then Heron's formula in Kahan's stable form", latex=r"K = \tfrac14\sqrt{(a+b+c)(-a+b+c)(a-b+c)(a+b-c)}",
     run=triangle_area_3d,
     verify=lambda i, o: (lambda P: _res((2 * o["area"], _vnorm(_vcross(_vsub(P[1], P[0]), _vsub(P[2], P[0])))),
                                         (o["perimeter"], math.dist(P[0], P[1]) + math.dist(P[1], P[2]) + math.dist(P[0], P[2]))))(_triples(i["points"], "points", 3)),
     parse=parse_tri3,
     reference=lambda points: (lambda np, P: {"area": float(0.5 * np.linalg.norm(np.cross(P[1] - P[0], P[2] - P[0]))),
                                               "perimeter": float(sum(np.linalg.norm(P[k] - P[(k + 1) % 3]) for k in range(3)))})(
         __import__("numpy"), __import__("numpy").array(_triples(points, "points", 3), float)),
     wrong=lambda points: dict(triangle_area_3d(points), area=2 * triangle_area_3d(points)["area"]), random_inputs=_rand_tri3,
     templates=["Find the area of the triangle with vertices (1, 0, 0), (0, 2, 0) and (0, 0, 3)."],
     examples=[{"inputs": {"points": "1 0 0, 0 2 0, 0 0 3"}, "outputs": {"area": 3.5}},
               {"inputs": {"points": "0 0 0, 3 0 0, 0 4 0"}, "outputs": {"area": 6.0, "perimeter": 12.0}}],
     questions=[{"q": "Find the area of the triangle with vertices (1, 0, 0), (0, 2, 0) and (0, 0, 3).", "outputs": {"area": 3.5}},
                {"q": "Find the area of the triangle with vertices (0, 0, 0), (1, 1, 1) and (2, 2, 2).", "outcome": "INVALID_INPUT"}])


def tetrahedron_measures(points):
    P = _triples(points, "points", 4)
    X = [_xvec(p) for p in P]
    if _vdot(_vsub(X[1], X[0]), _vcross(_vsub(X[2], X[0]), _vsub(X[3], X[0]))) == 0:
        raise ValueError("the four points are coplanar: they do not form a tetrahedron")
    det = _vdot(_vsub(P[1], P[0]), _vcross(_vsub(P[2], P[0]), _vsub(P[3], P[0])))
    S = sum(_vnorm(_vcross(_vsub(P[j], P[i]), _vsub(P[k], P[i]))) / 2 for i, j, k in ((0, 1, 2), (0, 1, 3), (0, 2, 3), (1, 2, 3)))
    return {"volume": abs(det) / 6, "surface_area": S}


def _verify_tetra(i, o):
    """Cayley-Menger determinant from the six edge lengths (288 V^2), and Heron per face."""
    P = _triples(i["points"], "points", 4)
    d = [[math.dist(P[a], P[b]) ** 2 for b in range(4)] for a in range(4)]
    M = [[0, 1, 1, 1, 1]] + [[1] + d[a] for a in range(4)]

    def det(m):
        if len(m) == 1:
            return m[0][0]
        return sum((-1) ** j * m[0][j] * det([r[:j] + r[j + 1:] for r in m[1:]]) for j in range(len(m)) if m[0][j])
    V2 = det(M) / 288
    S = 0.0
    for a, b, c in ((0, 1, 2), (0, 1, 3), (0, 2, 3), (1, 2, 3)):
        x, y, z = sorted((math.dist(P[b], P[c]), math.dist(P[a], P[c]), math.dist(P[a], P[b])), reverse=True)
        S += 0.25 * math.sqrt(max(0.0, (x + (y + z)) * (z - (x - y)) * (z + (x - y)) * (x + (y - z))))
    scale = max(max(abs(v) for row in d for v in row), 1.0) ** 1.5
    return max(abs(o["volume"] ** 2 - V2) / max(o["volume"] ** 2, 1e-6 * scale ** 2), _rel(o["surface_area"], S))


def _rand_tetra(rng):
    while True:
        P = [_rand3(rng) for _ in range(4)]
        v = abs(_vdot(_vsub(P[1], P[0]), _vcross(_vsub(P[2], P[0]), _vsub(P[3], P[0])))) / 6
        L = max(math.dist(a, b) for a, b in itertools.combinations(P, 2))
        if v > 0.02 * L ** 3:
            return {"points": _flat(P)}


def parse_tetra(q0):
    q = _norm(q0)
    p = _pts3(q)
    if not re.search(r"\btetrahedron\b", q) or not p:
        return None
    return {"points": _flat(p)} if len(p) == 4 else None


_add("tetrahedron_measures", "intermediate",
     name="Tetrahedron from four vertices: volume and surface area", topic="solid coordinate geometry / polyhedra",
     purpose="Volume and total surface area of the tetrahedron with four given vertices.",
     inputs=[_V("points", "the four vertices: x y z for each", n_min=12, n_max=12)],
     outputs=[_O("volume", "|det(B - A, C - A, D - A)| / 6"), _O("surface_area", "sum of the four face areas")], primary="volume",
     conditions=["exactly 4 points", "not coplanar"], method="scalar triple product; face areas from cross products",
     latex=r"V = \tfrac16\left|(B-A)\cdot\big((C-A)\times(D-A)\big)\right|", run=tetrahedron_measures, verify=_verify_tetra, parse=parse_tetra,
     reference=lambda points: (lambda H: {"volume": float(H.volume), "surface_area": float(H.area)})(
         __import__("scipy.spatial", fromlist=["ConvexHull"]).ConvexHull(_triples(points, "points", 4))),
     wrong=lambda points: dict(tetrahedron_measures(points), volume=tetrahedron_measures(points)["volume"] * 2), random_inputs=_rand_tetra,
     templates=["Find the volume of the tetrahedron with vertices (0, 0, 0), (1, 0, 0), (0, 1, 0) and (0, 0, 1)."],
     examples=[{"inputs": {"points": "0 0 0, 1 0 0, 0 1 0, 0 0 1"}, "outputs": {"volume": 1 / 6, "surface_area": 1.5 + math.sqrt(3) / 2}},
               {"inputs": {"points": "0 0 0, 3 0 0, 0 4 0, 0 0 6"}, "outputs": {"volume": 12.0}}],
     questions=[{"q": "Find the volume of the tetrahedron with vertices (0, 0, 0), (1, 0, 0), (0, 1, 0) and (0, 0, 1).", "outputs": {"volume": 1 / 6}},
                {"q": "Find the volume of the tetrahedron with vertices (0, 0, 0), (1, 0, 0), (0, 1, 0) and (1, 1, 0).", "outcome": "INVALID_INPUT"}])


def _lin3(text):
    if text.count("=") != 1:
        return None
    out = {"x": 0.0, "y": 0.0, "z": 0.0, "1": 0.0}
    for side, sg in zip(text.split("="), (1, -1)):
        s = side.replace(" ", "").replace("*", "")
        if not s:
            return None
        if s[0] not in "+-":
            s = "+" + s
        terms = re.findall(r"[+-][^+-]+", s)
        if "".join(terms) != s:
            return None
        for t in terms:
            m = re.fullmatch(r"([+-])(\d+(?:\.\d+)?|\.\d+)?([xyz])?", t)
            if not m or (m.group(2) is None and m.group(3) is None):
                return None
            c = float(m.group(2)) if m.group(2) else 1.0
            out[m.group(3) or "1"] += sg * (-c if m.group(1) == "-" else c)
    return out


def _planes(q):
    out = []
    for m in re.finditer(r"(?<![A-Za-z])([-+0-9.xyz* ]+)=([-+0-9.xyz* ]+)", q):
        lhs, rhs = m.group(1).strip(), re.split(r"\s+(?=[a-z]{2})", m.group(2).strip())[0].rstrip(". ")
        e = _lin3(lhs + "=" + rhs)
        if e and (e["x"] or e["y"] or e["z"]) and re.search(r"z", lhs + rhs):
            out.append((e["x"], e["y"], e["z"], e["1"]))
    return out


def _plane_check(a, b, c, d, name="plane"):
    a, b, c, d = (_fin(v, name) for v in (a, b, c, d))
    if a == 0 and b == 0 and c == 0:
        raise ValueError(f"{name}: a, b, c must not all be 0")
    return a, b, c, d


def plane_through_points(points):
    P = _triples(points, "points", 3)
    X = [_xvec(p) for p in P]
    n = _vcross(_vsub(X[1], X[0]), _vsub(X[2], X[0]))
    if n == (0, 0, 0):
        raise ValueError("the three points are collinear: they do not determine a plane")
    if next(c for c in n if c != 0) < 0:
        n = tuple(-c for c in n)
    nn = math.sqrt(float(n[0] ** 2 + n[1] ** 2 + n[2] ** 2))
    d = -(n[0] * X[0][0] + n[1] * X[0][1] + n[2] * X[0][2])
    return {"a": float(n[0]) / nn + 0.0, "b": float(n[1]) / nn + 0.0, "c": float(n[2]) / nn + 0.0, "d": float(d) / nn + 0.0}


def _verify_plane3(i, o):
    P = _triples(i["points"], "points", 3)
    n = (o["a"], o["b"], o["c"])
    first = next((c for c in n if abs(c) > 1e-12), 0.0)
    sc = max(1.0, max(abs(c) for p in P for c in p))
    return max([abs(_vnorm(n) - 1)] + [abs(_vdot(n, p) + o["d"]) / sc for p in P] + [0.0 if first > 0 else 1.0])


def _ref_plane3(points):
    import sympy as sp
    P = [sp.Point3D(*[_R(c) for c in p]) for p in _triples(points, "points", 3)]
    n = sp.Matrix(sp.Plane(*P).normal_vector)
    if next(c for c in n if c != 0) < 0:
        n = -n
    n = n / sp.sqrt(n.dot(n))
    d = -n.dot(sp.Matrix(P[0].args))
    return {"a": _F(n[0]), "b": _F(n[1]), "c": _F(n[2]), "d": _F(d)}


def parse_plane3(q0):
    q = _norm(q0)
    p = _pts3(q)
    if not re.search(r"\bplane\b", q) or not re.search(r"\b(through|passing through|containing|determined by)\b", q) or not p:
        return None
    if re.search(r"\b(distance|intersect\w*|angle|line)\b", q):
        return None
    return {"points": _flat(p)} if len(p) == 3 else None


_add("plane_through_points", "intermediate",
     name="Plane through three points", topic="solid coordinate geometry / planes",
     purpose="The equation a x + b y + c z + d = 0 of the plane through three points, with (a, b, c) a unit normal.",
     inputs=[_V("points", "the three points: x y z for each", n_min=9, n_max=9)],
     outputs=[_O("a", "x-component of the unit normal"), _O("b", "y-component"), _O("c", "z-component"), _O("d", "constant term")], primary="d",
     conditions=["exactly 3 points, not collinear", "convention: (a, b, c) has length 1 and its first nonzero component is positive"],
     method="normal n = (B - A) x (C - A) (exact), normalised; d = -n.A",
     latex=r"\mathbf n = (B-A)\times(C-A),\quad \mathbf n\cdot\mathbf x - \mathbf n\cdot A = 0", run=plane_through_points, verify=_verify_plane3,
     parse=parse_plane3, reference=_ref_plane3,
     wrong=lambda points: (lambda r: dict(r, d=-r["d"] if r["d"] else 1.0))(plane_through_points(points)), random_inputs=_rand_tri3,
     templates=["Find the equation of the plane through (1, 0, 0), (0, 1, 0) and (0, 0, 1)."],
     examples=[{"inputs": {"points": "1 0 0, 0 1 0, 0 0 1"}, "outputs": {"a": 1 / math.sqrt(3), "b": 1 / math.sqrt(3), "c": 1 / math.sqrt(3), "d": -1 / math.sqrt(3)}},
               {"inputs": {"points": "0 0 2, 1 0 2, 0 1 2"}, "outputs": {"a": 0.0, "b": 0.0, "c": 1.0, "d": -2.0}}],
     questions=[{"q": "Find the equation of the plane through (1, 0, 0), (0, 1, 0) and (0, 0, 1).", "outputs": {"d": -1 / math.sqrt(3)}},
                {"q": "Find the plane passing through (0, 0, 2), (1, 0, 2) and (0, 1, 2).", "outputs": {"c": 1.0, "d": -2.0}},
                {"q": "Find the plane through (0, 0, 0), (1, 1, 1) and (2, 2, 2).", "outcome": "INVALID_INPUT"}])


def point_plane_distance(px, py, pz, a, b, c, d):
    px, py, pz = _fin(px, "px"), _fin(py, "py"), _fin(pz, "pz")
    a, b, c, d = _plane_check(a, b, c, d)
    n2 = a * a + b * b + c * c
    s = (a * px + b * py + c * pz + d)
    return {"distance": abs(s) / math.sqrt(n2), "fx": px - a * s / n2 + 0.0, "fy": py - b * s / n2 + 0.0, "fz": pz - c * s / n2 + 0.0}


def _verify_ppd(i, o):
    P, n = (i["px"], i["py"], i["pz"]), (i["a"], i["b"], i["c"])
    Fp = (o["fx"], o["fy"], o["fz"])
    sc = max(1.0, max(abs(v) for v in P), abs(i["d"]) / _vnorm(n))
    on = abs(_vdot(n, Fp) + i["d"]) / (_vnorm(n) * sc)
    par = _vnorm(_vcross(_vsub(P, Fp), n)) / (_vnorm(n) * sc)
    return max(on, par, abs(o["distance"] - math.dist(P, Fp)) / sc)


def _ref_ppd(px, py, pz, a, b, c, d):
    import sympy as sp
    A, B, C, D = (_R(v) for v in (a, b, c, d))
    on = sp.Point3D(-D / A, 0, 0) if A != 0 else (sp.Point3D(0, -D / B, 0) if B != 0 else sp.Point3D(0, 0, -D / C))
    pl = sp.Plane(on, normal_vector=(A, B, C))
    P = sp.Point3D(_R(px), _R(py), _R(pz))
    f = pl.projection(P)
    return {"distance": _F(pl.distance(P)), "fx": _F(f.x), "fy": _F(f.y), "fz": _F(f.z)}


def _rand_plane(rng, k=""):
    a, b, c = (float(rng.randint(-9, 9)) for _ in range(3))
    if a == b == c == 0:
        a = 1.0
    return {f"a{k}": a, f"b{k}": b, f"c{k}": c, f"d{k}": float(rng.randint(-30, 30))}


def parse_ppd(q0):
    q = _norm(q0)
    if not re.search(r"\bdistance\b|\bfoot of the perpendicular\b|\bprojection\b", q) or not re.search(r"\bplane\b", q) or re.search(r"\bline\b", q):
        return None
    p, pl = _pts3(q), _planes(q)
    if len(p) > 1 or len(pl) > 1:
        return None
    d = {}
    if p:
        d.update(px=p[0][0], py=p[0][1], pz=p[0][2])
    if pl:
        d.update(a=pl[0][0], b=pl[0][1], c=pl[0][2], d=pl[0][3])
    return d


_add("point_plane_distance", "intermediate",
     name="Distance from a point to a plane", topic="solid coordinate geometry / planes",
     purpose="The distance from a point to the plane a x + b y + c z + d = 0 and the foot of the perpendicular.",
     inputs=[_I("px", "x of the point"), _I("py", "y of the point"), _I("pz", "z of the point"), _I("a", "coefficient of x"), _I("b", "coefficient of y"),
             _I("c", "coefficient of z"), _I("d", "constant term")],
     outputs=[_O("distance", "|a px + b py + c pz + d| / sqrt(a^2 + b^2 + c^2)"), _O("fx", "x of the foot"), _O("fy", "y of the foot"), _O("fz", "z of the foot")],
     primary="distance", conditions=["a, b, c not all 0"], method="signed value of the plane's equation divided by the normal's length",
     latex=r"d = \frac{|a p_x + b p_y + c p_z + d|}{\sqrt{a^2+b^2+c^2}}", run=point_plane_distance, verify=_verify_ppd, parse=parse_ppd, reference=_ref_ppd,
     wrong=lambda px, py, pz, a, b, c, d: dict(point_plane_distance(px, py, pz, a, b, c, d), distance=abs(a * px + b * py + c * pz + d)),
     random_inputs=lambda rng: {"px": _RANDP(rng), "py": _RANDP(rng), "pz": _RANDP(rng), **{k[0]: v for k, v in _rand_plane(rng).items()}},
     templates=["Find the distance from the point (1, 2, 3) to the plane 2x - y + 2z = 3."],
     examples=[{"inputs": {"px": 1, "py": 2, "pz": 3, "a": 2, "b": -1, "c": 2, "d": -3}, "outputs": {"distance": 1.0, "fx": 1 - 2 / 3, "fy": 2 + 1 / 3, "fz": 3 - 2 / 3}}],
     questions=[{"q": "Find the distance from the point (1, 2, 3) to the plane 2x - y + 2z = 3.", "outputs": {"distance": 1.0}},
                {"q": "Find the distance from the point (0, 0, 5) to the plane z = 1.", "outputs": {"distance": 4.0, "fz": 1.0}}])


def line_plane_intersection(x0, y0, z0, dx, dy, dz, a, b, c, d):
    P = tuple(_fin(v, "point") for v in (x0, y0, z0))
    D = tuple(_fin(v, "direction") for v in (dx, dy, dz))
    a, b, c, d = _plane_check(a, b, c, d)
    if D == (0.0, 0.0, 0.0):
        raise ValueError("the direction vector must not be zero")
    den = _fr(a) * _fr(dx) + _fr(b) * _fr(dy) + _fr(c) * _fr(dz)
    if den == 0:
        raise ValueError("the line is parallel to the plane (or lies in it): no single intersection point")
    t = (_fr(a) * _fr(x0) + _fr(b) * _fr(y0) + _fr(c) * _fr(z0) + _fr(d)) / -den
    return {"x": float(_fr(x0) + t * _fr(dx)) + 0.0, "y": float(_fr(y0) + t * _fr(dy)) + 0.0, "z": float(_fr(z0) + t * _fr(dz)) + 0.0, "t": float(t) + 0.0}


def _verify_lpi(i, o):
    X = (o["x"], o["y"], o["z"])
    n = (i["a"], i["b"], i["c"])
    P, D = (i["x0"], i["y0"], i["z0"]), (i["dx"], i["dy"], i["dz"])
    sc = max(1.0, max(abs(v) for v in X), max(abs(v) for v in P))
    on_plane = abs(_vdot(n, X) + i["d"]) / (_vnorm(n) * sc)
    on_line = _vnorm(_vsub(X, tuple(P[k] + o["t"] * D[k] for k in range(3)))) / sc
    return max(on_plane, on_line)


def _ref_lpi(x0, y0, z0, dx, dy, dz, a, b, c, d):
    import sympy as sp
    A, B, C, D = (_R(v) for v in (a, b, c, d))
    on = sp.Point3D(-D / A, 0, 0) if A != 0 else (sp.Point3D(0, -D / B, 0) if B != 0 else sp.Point3D(0, 0, -D / C))
    P0 = sp.Point3D(_R(x0), _R(y0), _R(z0))
    X = sp.Plane(on, normal_vector=(A, B, C)).intersection(sp.Line3D(P0, direction_ratio=[_R(dx), _R(dy), _R(dz)]))[0]
    return {"x": _F(X.x), "y": _F(X.y), "z": _F(X.z)}


def parse_lpi(q0):
    q = _norm(q0)
    if not re.search(r"\bline\b", q) or not re.search(r"\bplane\b", q) or not re.search(r"\b(intersect\w*|meet|cross|hit)\b", q):
        return None
    m1 = re.search(r"\bthrough\s*(?:the\s*point\s*)?" + _P3, q)
    m2 = re.search(r"\bdirection(?:\s*vector)?\s*(?:of\s*)?" + _P3, q)
    pl = _planes(q)
    if len(pl) > 1:
        return None
    d = {}
    if m1:
        d.update(x0=float(m1.group(1)), y0=float(m1.group(2)), z0=float(m1.group(3)))
    if m2:
        d.update(dx=float(m2.group(1)), dy=float(m2.group(2)), dz=float(m2.group(3)))
    if pl:
        d.update(a=pl[0][0], b=pl[0][1], c=pl[0][2], d=pl[0][3])
    return d if d or _pts3(q) or pl else None


_add("line_plane_intersection", "advanced",
     name="Intersection of a line and a plane", topic="solid coordinate geometry / lines and planes",
     purpose="The point where the line P0 + t v meets the plane a x + b y + c z + d = 0.",
     inputs=[_I("x0", "x of a point on the line"), _I("y0", "y of the point"), _I("z0", "z of the point"), _I("dx", "x of the direction"),
             _I("dy", "y of the direction"), _I("dz", "z of the direction"), _I("a", "coefficient of x"), _I("b", "coefficient of y"),
             _I("c", "coefficient of z"), _I("d", "constant term")],
     outputs=[_O("x", "x of the intersection"), _O("y", "y"), _O("z", "z"), _O("t", "line parameter at the intersection")], primary="x",
     conditions=["direction not zero", "a, b, c not all 0", "line not parallel to the plane (exact test)"],
     method="t = -(n.P0 + d)/(n.v) in exact rational arithmetic", latex=r"t = -\frac{\mathbf n\cdot P_0 + d}{\mathbf n\cdot\mathbf v},\quad X = P_0 + t\,\mathbf v",
     run=line_plane_intersection, verify=_verify_lpi, parse=parse_lpi, reference=_ref_lpi,
     wrong=lambda x0, y0, z0, dx, dy, dz, a, b, c, d: (lambda r: {"x": x0 - r["t"] * dx, "y": y0 - r["t"] * dy, "z": z0 - r["t"] * dz, "t": -r["t"]})(
         line_plane_intersection(x0, y0, z0, dx, dy, dz, a, b, c, d)),
     random_inputs=lambda rng: {"x0": _RANDP(rng), "y0": _RANDP(rng), "z0": _RANDP(rng), "dx": float(rng.randint(-9, 9)), "dy": float(rng.randint(-9, 9)),
                                "dz": float(rng.randint(1, 9)), **{k[0]: v for k, v in _rand_plane(rng).items()}},
     templates=["Where does the line through (1, 0, 0) with direction (1, 2, 3) meet the plane x + y + z = 7?"],
     examples=[{"inputs": {"x0": 1, "y0": 0, "z0": 0, "dx": 1, "dy": 2, "dz": 3, "a": 1, "b": 1, "c": 1, "d": -7}, "outputs": {"x": 2.0, "y": 2.0, "z": 3.0, "t": 1.0}}],
     questions=[{"q": "Where does the line through (1, 0, 0) with direction (1, 2, 3) meet the plane x + y + z = 7?", "outputs": {"x": 2.0, "y": 2.0, "z": 3.0}},
                {"q": "Where does the line through (0, 0, 0) with direction (1, -1, 0) meet the plane x + y + z = 7?", "outcome": "INVALID_INPUT"}])


def skew_lines_distance(p1, d1, p2, d2):
    P1, D1, P2, D2 = (tuple(_fin(float(v), n) for v in vec) for vec, n in ((p1, "p1"), (d1, "d1"), (p2, "p2"), (d2, "d2")))
    if D1 == (0, 0, 0) or D2 == (0, 0, 0):
        raise ValueError("a direction vector is zero")
    w = _vsub(P2, P1)
    nx = _vcross(_xvec(D1), _xvec(D2))
    if nx == (0, 0, 0):                                      # parallel lines
        dist = _vnorm(_vcross(w, D1)) / _vnorm(D1)
        return {"distance": dist, "parallel": "yes"}
    n = _vcross(D1, D2)
    return {"distance": abs(_vdot(w, n)) / _vnorm(n), "parallel": "no"}


def _verify_skew(i, o):
    """Minimise |P1 + s d1 - P2 - t d2| by solving the 2x2 normal equations (or the parallel projection)."""
    P1, D1, P2, D2 = (tuple(i[k]) for k in ("p1", "d1", "p2", "d2"))
    w = _vsub(P1, P2)
    a, b, c = _vdot(D1, D1), _vdot(D1, D2), _vdot(D2, D2)
    d, e = _vdot(D1, w), _vdot(D2, w)
    den = a * c - b * b
    if o["parallel"] == "yes":
        s, t = 0.0, e / c
    else:
        s, t = (b * e - c * d) / den, (a * e - b * d) / den
    gap = _vnorm(tuple(w[k] + s * D1[k] - t * D2[k] for k in range(3)))
    return abs(o["distance"] - gap) / max(1.0, _vnorm(w))


def _ref_skew(p1, d1, p2, d2):
    import numpy as np
    A = np.column_stack([np.array(d1, float), -np.array(d2, float)])
    rhs = np.array(p2, float) - np.array(p1, float)
    st = np.linalg.lstsq(A, rhs, rcond=None)[0]
    return {"distance": float(np.linalg.norm(A @ st - rhs))}


def parse_skew(q0):
    q = _norm(q0)
    if not re.search(r"\bdistance between\b", q) or not re.search(r"\blines\b", q):
        return None
    p = _pts3(q)
    if not p:
        return None
    if len(p) != 4:
        return {}
    return {"p1": list(p[0]), "d1": list(p[1]), "p2": list(p[2]), "d2": list(p[3])}


_V3 = lambda s, m: {"symbol": s, "kind": "number", "type": "vector", "meaning": m, "required": True, "min_length": 3, "max_length": 3}

_add("skew_lines_distance", "advanced",
     name="Distance between two lines in space (skew or parallel)", topic="solid coordinate geometry / lines",
     purpose="The shortest distance between the lines P1 + s d1 and P2 + t d2 in three dimensions.",
     inputs=[_V3("p1", "a point on line 1"), _V3("d1", "direction of line 1"), _V3("p2", "a point on line 2"), _V3("d2", "direction of line 2")],
     outputs=[_O("distance", "shortest distance"), _O("parallel", "yes / no (exact test)", "text")], primary="distance", conditions=["nonzero directions"],
     method="|(P2 - P1).(d1 x d2)| / |d1 x d2|; parallel lines: |(P2 - P1) x d1| / |d1|",
     latex=r"d = \frac{|(P_2-P_1)\cdot(\mathbf d_1\times\mathbf d_2)|}{\|\mathbf d_1\times\mathbf d_2\|}", run=skew_lines_distance, verify=_verify_skew,
     parse=parse_skew, reference=_ref_skew,
     wrong=lambda p1, d1, p2, d2: (lambda r: dict(r, distance=abs(_vdot(_vsub(p2, p1), _vcross(d1, d2))) if r["parallel"] == "no" else 2 * r["distance"] + 1))(
         skew_lines_distance(p1, d1, p2, d2)),
     random_inputs=lambda rng: {"p1": list(_rand3(rng)), "d1": [float(rng.randint(-9, 9)) for _ in range(2)] + [float(rng.randint(1, 9))],
                                "p2": list(_rand3(rng)), "d2": [float(rng.randint(1, 9))] + [float(rng.randint(-9, 9)) for _ in range(2)]},
     templates=["Find the distance between the lines through (0, 0, 0) with direction (1, 0, 0) and through (0, 1, 1) with direction (0, 1, 0)."],
     examples=[{"inputs": {"p1": "0 0 0", "d1": "1 0 0", "p2": "0 1 1", "d2": "0 1 0"}, "outputs": {"distance": 1.0, "parallel": "no"}},
               {"inputs": {"p1": [0, 0, 0], "d1": [1, 0, 0], "p2": [0, 3, 4], "d2": [2, 0, 0]}, "outputs": {"distance": 5.0, "parallel": "yes"}}],
     questions=[{"q": "Find the distance between the lines through (0, 0, 0) with direction (1, 0, 0) and through (0, 1, 1) with direction (0, 1, 0).", "outputs": {"distance": 1.0}},
                {"q": "Find the distance between the lines through (0, 0, 0) with direction (0, 0, 0) and through (0, 1, 1) with direction (0, 1, 0).", "outcome": "INVALID_INPUT"}])


def angle_between_planes(a1, b1, c1, a2, b2, c2):
    n1 = tuple(_fin(v, "plane 1") for v in (a1, b1, c1))
    n2 = tuple(_fin(v, "plane 2") for v in (a2, b2, c2))
    if n1 == (0, 0, 0) or n2 == (0, 0, 0):
        raise ValueError("a plane's normal (a, b, c) must not be zero")
    return {"angle": math.degrees(math.atan2(_vnorm(_vcross(n1, n2)), abs(_vdot(n1, n2))))}


def _ref_planes_angle(a1, b1, c1, a2, b2, c2):
    import sympy as sp
    t = sp.Plane(sp.Point3D(0, 0, 0), normal_vector=(_R(a1), _R(b1), _R(c1))).angle_between(sp.Plane(sp.Point3D(0, 0, 0), normal_vector=(_R(a2), _R(b2), _R(c2))))
    return {"angle": _F(sp.deg(sp.Min(t, sp.pi - t)))}


def parse_planes_angle(q0):
    q = _norm(q0)
    if not re.search(r"\bangle between\b", q) or not re.search(r"\bplanes\b", q):
        return None
    pl = _planes(q)
    if len(pl) != 2:
        return None if len(pl) > 2 else {}
    return {"a1": pl[0][0], "b1": pl[0][1], "c1": pl[0][2], "a2": pl[1][0], "b2": pl[1][1], "c2": pl[1][2]}


_add("angle_between_planes", "intermediate",
     name="Angle between two planes", topic="solid coordinate geometry / planes",
     purpose="The acute (dihedral) angle between two planes given by their equations (only the normals matter).",
     inputs=[_I("a1", "x-coefficient of plane 1"), _I("b1", "y-coefficient of plane 1"), _I("c1", "z-coefficient of plane 1"),
             _I("a2", "x-coefficient of plane 2"), _I("b2", "y-coefficient of plane 2"), _I("c2", "z-coefficient of plane 2")],
     outputs=[_O("angle", "angle between the planes, degrees, 0 to 90")], primary="angle", conditions=["nonzero normals", "the constants do not affect the angle"],
     method="angle between the normals, folded to [0, 90]: atan2(|n1 x n2|, |n1.n2|)", latex=r"\cos\theta = \frac{|\mathbf n_1\cdot\mathbf n_2|}{\|\mathbf n_1\|\|\mathbf n_2\|}",
     run=angle_between_planes,
     verify=lambda i, o: (lambda n1, n2: max(abs(math.cos(math.radians(o["angle"])) - abs(_vdot(n1, n2)) / (_vnorm(n1) * _vnorm(n2))),
                                             abs(math.sin(math.radians(o["angle"])) - _vnorm(_vcross(n1, n2)) / (_vnorm(n1) * _vnorm(n2)))))(
         (i["a1"], i["b1"], i["c1"]), (i["a2"], i["b2"], i["c2"])),
     parse=parse_planes_angle, reference=_ref_planes_angle,
     wrong=lambda a1, b1, c1, a2, b2, c2: {"angle": 180 - angle_between_planes(a1, b1, c1, a2, b2, c2)["angle"]},
     random_inputs=lambda rng: (lambda p, r: {"a1": p["a"], "b1": p["b"], "c1": p["c"], "a2": r["a"], "b2": r["b"], "c2": r["c"]})(
         _rand_plane(rng), _rand_plane(rng)),
     templates=["Find the angle between the planes x + y + z = 1 and x - y = 0 + 0z."],
     examples=[{"inputs": {"a1": 0, "b1": 0, "c1": 1, "a2": 1, "b2": 0, "c2": 1}, "outputs": {"angle": 45.0}}],
     questions=[{"q": "Find the angle between the planes z = 0 and x + z = 2.", "outputs": {"angle": 45.0}},
                {"q": "Find the angle between the planes x + y + z = 1 and x - y = 0 + 0z.", "outputs": {"angle": 90.0}}])


def rotate_point_3d(point, axis, angle):
    P = tuple(_fin(float(v), "point") for v in point)
    k = tuple(_fin(float(v), "axis") for v in axis)
    t = _fin(angle, "angle")
    nk = _vnorm(k)
    if nk == 0:
        raise ValueError("the axis must not be the zero vector")
    k = tuple(c / nk for c in k)
    r = t % 360.0
    co, si = {0.0: (1.0, 0.0), 90.0: (0.0, 1.0), 180.0: (-1.0, 0.0), 270.0: (0.0, -1.0)}.get(r, (math.cos(math.radians(t)), math.sin(math.radians(t))))
    kxp, kp = _vcross(k, P), _vdot(k, P)
    out = tuple(P[j] * co + kxp[j] * si + k[j] * kp * (1 - co) for j in range(3))
    s = max(abs(c) for c in P)
    return {"x": _snap(out[0], s), "y": _snap(out[1], s), "z": _snap(out[2], s)}


def _verify_rot3(i, o):
    P = tuple(float(v) for v in i["point"])
    k = tuple(float(v) for v in i["axis"])
    X = (o["x"], o["y"], o["z"])
    nk = _vnorm(k)
    u = tuple(c / nk for c in k)
    sc = max(1.0, _vnorm(P))
    # same length, same component along the axis, and the perpendicular parts turned by the angle (right-hand rule)
    pp = tuple(P[j] - u[j] * _vdot(u, P) for j in range(3))
    xp = tuple(X[j] - u[j] * _vdot(u, X) for j in range(3))
    t = math.radians(i["angle"])
    want = tuple(pp[j] * math.cos(t) + _vcross(u, pp)[j] * math.sin(t) for j in range(3))
    return max(abs(_vdot(u, X) - _vdot(u, P)) / sc, _vnorm(_vsub(xp, want)) / sc)


def _ref_rot3(point, axis, angle):
    import numpy as np
    from scipy.spatial.transform import Rotation
    k = np.array(axis, float)
    v = Rotation.from_rotvec(np.radians(angle) * k / np.linalg.norm(k)).apply(np.array(point, float))
    return {"x": float(v[0]), "y": float(v[1]), "z": float(v[2])}


def parse_rot3(q0):
    q = _norm(q0)
    if not re.search(r"\brotat\w*\b", q) or not re.search(r"\baxis\b", q):
        return None
    p = _pts3(q)
    axis = None
    m = re.search(r"\b(?:about|around)\s*the\s*([xyz])[- ]axis\b", q)
    if m:
        axis = [1.0 if m.group(1) == c else 0.0 for c in "xyz"]
        pts = p
    else:
        ma = re.search(r"\baxis\s*(?:direction\s*)?" + _P3, q)
        if ma:
            axis = [float(ma.group(k)) for k in (1, 2, 3)]
        pts = [x for x in p if not (axis and list(x) == axis)] if axis else p
    if len(pts) > 1:
        return None
    t = _angle_in(q, r"angle|by|through")
    if t is not None and re.search(r"(?<!counter)(?<!counter-)(?<!anti)(?<!anti-)\bclockwise\b", q):
        t = -t
    d = {"angle": t, "axis": axis, "point": list(pts[0]) if pts else None}
    return _clean(d)


_add("rotate_point_3d", "advanced",
     name="Rotation of a point in space about an axis", topic="transformations / rotations in space",
     purpose="The image of a point rotated by an angle about an axis through the origin (right-hand rule).",
     inputs=[_V3("point", "the point x y z"), _V3("axis", "axis direction (through the origin)"), _I("angle", "angle in degrees (right-hand rule)")],
     outputs=[_O("x", "x of the image"), _O("y", "y of the image"), _O("z", "z of the image")], primary="x",
     conditions=["axis not zero; it passes through the origin", "positive angle: counterclockwise looking from the axis tip toward the origin"],
     method="Rodrigues' formula v' = v cos t + (k x v) sin t + k (k.v)(1 - cos t), exact for multiples of 90 degrees",
     latex=r"\mathbf v' = \mathbf v\cos\theta + (\mathbf k\times\mathbf v)\sin\theta + \mathbf k(\mathbf k\cdot\mathbf v)(1-\cos\theta)",
     run=rotate_point_3d, verify=_verify_rot3, parse=parse_rot3, reference=_ref_rot3,
     wrong=lambda point, axis, angle: rotate_point_3d(point, axis, -angle),
     random_inputs=lambda rng: {"point": list(_rand3(rng)), "axis": [float(rng.randint(-5, 5)), float(rng.randint(-5, 5)), float(rng.randint(1, 5))],
                                "angle": round(rng.uniform(5, 355), 1)},
     templates=["Rotate the point (1, 0, 0) by 90 degrees about the z-axis.", "Rotate (1, 2, 3) by 120 degrees about the axis (1, 1, 1)."],
     examples=[{"inputs": {"point": "1 0 0", "axis": "0 0 1", "angle": 90}, "outputs": {"x": 0.0, "y": 1.0, "z": 0.0}},
               {"inputs": {"point": [1, 2, 3], "axis": [1, 1, 1], "angle": 120}, "outputs": {"x": 3.0, "y": 1.0, "z": 2.0}}],
     questions=[{"q": "Rotate the point (1, 0, 0) by 90 degrees about the z-axis.", "outputs": {"x": 0.0, "y": 1.0, "z": 0.0}},
                {"q": "Rotate (1, 2, 3) by 120 degrees about the axis (1, 1, 1).", "outputs": {"x": 3.0, "y": 1.0, "z": 2.0}}])


def sphere_four_points(points):
    P = _triples(points, "points", 4)
    X = [_xvec(p) for p in P]
    rows = [[2 * (X[k][j] - X[0][j]) for j in range(3)] for k in (1, 2, 3)]
    rhs = [sum(X[k][j] ** 2 - X[0][j] ** 2 for j in range(3)) for k in (1, 2, 3)]
    det3 = lambda m: (m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                      + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]))
    D = det3(rows)
    if D == 0:
        raise ValueError("the four points are coplanar: no unique sphere passes through them")
    c = []
    for j in range(3):
        m = [r[:] for r in rows]
        for k in range(3):
            m[k][j] = rhs[k]
        c.append(det3(m) / D)
    r2 = sum((X[0][j] - c[j]) ** 2 for j in range(3))
    return {"cx": float(c[0]) + 0.0, "cy": float(c[1]) + 0.0, "cz": float(c[2]) + 0.0, "radius": math.sqrt(float(r2))}


def _ref_sphere4(points):
    """Nonlinear least squares on |X - P_i| - R (scipy.optimize), started from the centroid."""
    import numpy as np
    from scipy.optimize import least_squares
    P = np.array(_triples(points, "points", 4), float)
    c0 = P.mean(axis=0)
    r0 = np.linalg.norm(P - c0, axis=1).mean()
    sol = least_squares(lambda v: np.linalg.norm(P - v[:3], axis=1) - v[3], np.append(c0, r0), xtol=1e-15, ftol=1e-15, gtol=1e-15, method="lm")
    return {"cx": float(sol.x[0]), "cy": float(sol.x[1]), "cz": float(sol.x[2]), "radius": float(abs(sol.x[3]))}


def parse_sphere4(q0):
    q = _norm(q0)
    p = _pts3(q)
    if not re.search(r"\bsphere\b", q) or not re.search(r"\b(through|passing through|containing|circumscrib\w*|circumsphere)\b", q) or not p:
        return None
    return {"points": _flat(p)} if len(p) == 4 else None


_add("sphere_four_points", "expert",
     name="Sphere through four points (circumsphere of a tetrahedron)", topic="solid coordinate geometry / spheres",
     purpose="Centre and radius of the sphere through four non-coplanar points.",
     inputs=[_V("points", "the four points: x y z for each", n_min=12, n_max=12)],
     outputs=[_O("cx", "x of the centre"), _O("cy", "y of the centre"), _O("cz", "z of the centre"), _O("radius", "radius")], primary="radius",
     conditions=["exactly 4 points, not coplanar"], method="subtract the first sphere equation from the others: a 3x3 linear system, solved exactly (Cramer)",
     latex=r"|X - P_i|^2 = R^2 \;(i = 1..4) \Rightarrow 2(P_i - P_1)\cdot X = |P_i|^2 - |P_1|^2", run=sphere_four_points,
     verify=lambda i, o: (lambda P: max(abs(math.dist((o["cx"], o["cy"], o["cz"]), p) - o["radius"]) / o["radius"] for p in P))(_triples(i["points"], "points", 4)),
     parse=parse_sphere4, reference=_ref_sphere4,
     wrong=lambda points: (lambda P: (lambda g: {"cx": g[0], "cy": g[1], "cz": g[2], "radius": math.dist(g, P[0])})(
         tuple(sum(p[j] for p in P) / 4 for j in range(3))))(_triples(points, "points", 4)),
     random_inputs=_rand_tetra,
     templates=["Find the sphere through (1, 0, 0), (0, 1, 0), (0, 0, 1) and (0, 0, 0)."],
     examples=[{"inputs": {"points": "1 0 0, 0 1 0, 0 0 1, 0 0 0"}, "outputs": {"cx": 0.5, "cy": 0.5, "cz": 0.5, "radius": math.sqrt(3) / 2}},
               {"inputs": {"points": "2 0 0, -2 0 0, 0 2 0, 0 0 2"}, "outputs": {"cx": 0.0, "cy": 0.0, "cz": 0.0, "radius": 2.0}}],
     questions=[{"q": "Find the sphere through (1, 0, 0), (0, 1, 0), (0, 0, 1) and (0, 0, 0).", "outputs": {"radius": math.sqrt(3) / 2}},
                {"q": "Find the sphere passing through (0, 0, 0), (1, 0, 0), (0, 1, 0) and (1, 1, 0).", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# spherical geometry and solid angles (advanced / expert)
# ==================================================================================================================
def _latlon(lat, lon, name):
    lat, lon = _fin(lat, f"{name} latitude"), _fin(lon, f"{name} longitude")
    if not -90 <= lat <= 90:
        raise ValueError(f"{name}: latitude must be between -90 and 90 degrees")
    if not -360 <= lon <= 360:
        raise ValueError(f"{name}: longitude must be between -360 and 360 degrees")
    return math.radians(lat), math.radians(lon)


def _central(p1, p2):
    (f1, l1), (f2, l2) = p1, p2
    h = math.sin((f2 - f1) / 2) ** 2 + math.cos(f1) * math.cos(f2) * math.sin((l2 - l1) / 2) ** 2
    h = min(1.0, max(0.0, h))
    return 2 * math.atan2(math.sqrt(h), math.sqrt(1 - h))


def _unit(lat_r, lon_r):
    return (math.cos(lat_r) * math.cos(lon_r), math.cos(lat_r) * math.sin(lon_r), math.sin(lat_r))


def great_circle(lat1, lon1, lat2, lon2, radius=6371.0):
    p1, p2 = _latlon(lat1, lon1, "point 1"), _latlon(lat2, lon2, "point 2")
    R = _pos(radius, "radius")
    c = _central(p1, p2)
    return {"central_angle": math.degrees(c), "distance": R * c}


def _verify_gc(i, o):
    a = _unit(math.radians(i["lat1"]), math.radians(i["lon1"]))
    b = _unit(math.radians(i["lat2"]), math.radians(i["lon2"]))
    t = math.radians(o["central_angle"])
    return max(abs(math.cos(t) - _vdot(a, b)), abs(math.sin(t) - _vnorm(_vcross(a, b))), _rel(o["distance"], i.get("radius", 6371.0) * t))


def _ref_gc(lat1, lon1, lat2, lon2, radius=6371.0):
    import numpy as np
    u = lambda f, l: np.array([np.cos(np.radians(f)) * np.cos(np.radians(l)), np.cos(np.radians(f)) * np.sin(np.radians(l)), np.sin(np.radians(f))])
    a, b = u(lat1, lon1), u(lat2, lon2)
    c = float(np.arctan2(np.linalg.norm(np.cross(a, b)), np.dot(a, b)))
    return {"central_angle": float(np.degrees(c)), "distance": radius * c}


def _rand_ll(rng):
    return round(rng.uniform(-89, 89), 4), round(rng.uniform(-180, 180), 4)


def _rand_gc(rng):
    while True:
        (a, b), (c, d) = _rand_ll(rng), _rand_ll(rng)
        x = great_circle(a, b, c, d, 1.0)["central_angle"]
        if 0.5 < x < 179:
            return {"lat1": a, "lon1": b, "lat2": c, "lon2": d, "radius": rng.choice([6371.0, 1.0, _rand_len(rng, 1, 7000)])}


def parse_gc(q0):
    q = _norm(q0)
    if not re.search(r"\b(great[- ]circle|haversine|central angle|on (?:the )?earth|on a sphere|spherical distance)\b", q):
        return None
    if not _pts2(q) and not re.search(r"\b(great[- ]circle|haversine)\b", q):
        return None
    if re.search(r"\b(bearing|heading|azimuth|triangle)\b", q):
        return None
    p = _pts2(q)
    if len(p) > 2:
        return None
    d = {"radius": _after(q, r"radius")}
    if len(p) == 2:
        d.update(lat1=p[0][0], lon1=p[0][1], lat2=p[1][0], lon2=p[1][1])
    return _clean(d)


_add("great_circle_distance", "advanced",
     name="Great-circle distance between two points on a sphere", topic="spherical geometry / distances",
     purpose="The central angle and great-circle distance between two points given by latitude and longitude (on the Earth unless a radius is given).",
     inputs=[_I("lat1", "latitude of point 1, degrees"), _I("lon1", "longitude of point 1, degrees"), _I("lat2", "latitude of point 2"),
             _I("lon2", "longitude of point 2"), _I("radius", "sphere radius (default 6371, the mean Earth radius in km)", required=False)],
     outputs=[_O("central_angle", "central angle, degrees"), _O("distance", "great-circle distance (radius units)")], primary="distance",
     conditions=["points written (latitude, longitude) in signed decimal degrees", "radius defaults to 6371 km (mean Earth radius; the Earth is treated as a sphere)"],
     method="haversine formula in its atan2 form", latex=r"h = \sin^2\tfrac{\Delta\varphi}{2} + \cos\varphi_1\cos\varphi_2\sin^2\tfrac{\Delta\lambda}{2},\quad d = 2R\,\operatorname{atan2}(\sqrt h, \sqrt{1-h})",
     run=great_circle, verify=_verify_gc, parse=parse_gc, reference=_ref_gc,
     wrong=lambda lat1, lon1, lat2, lon2, radius=6371.0: (lambda c: {"central_angle": c, "distance": radius * math.radians(c)})(
         math.degrees(math.hypot(math.radians(lat2 - lat1), math.radians(lon2 - lon1)))),
     random_inputs=_rand_gc,
     templates=["Find the great-circle distance between (51.5074, -0.1278) and (40.7128, -74.006) on the Earth.",
                "What is the central angle between (0, 0) and (0, 90) on a sphere of radius 1?"],
     examples=[{"inputs": {"lat1": 0, "lon1": 0, "lat2": 0, "lon2": 90, "radius": 1}, "outputs": {"central_angle": 90.0, "distance": math.pi / 2}},
               {"inputs": {"lat1": 90, "lon1": 0, "lat2": 0, "lon2": 45}, "outputs": {"central_angle": 90.0, "distance": 6371 * math.pi / 2}}],
     questions=[{"q": "What is the central angle between (0, 0) and (0, 90) on a sphere of radius 1?", "outputs": {"central_angle": 90.0, "distance": math.pi / 2}},
                {"q": "Find the great-circle distance between (90, 0) and (0, 45) on the Earth.", "outputs": {"distance": 6371 * math.pi / 2}},
                {"q": "Find the great-circle distance between (95, 0) and (0, 45) on the Earth.", "outcome": "INVALID_INPUT"}])


def initial_bearing(lat1, lon1, lat2, lon2):
    (f1, l1), (f2, l2) = _latlon(lat1, lon1, "point 1"), _latlon(lat2, lon2, "point 2")
    if abs(lat1) == 90:
        raise ValueError("the start is at a pole: every direction is south (or north); the bearing is undefined")
    if _central((f1, l1), (f2, l2)) < 1e-12 or _central((f1, l1), (f2, l2)) > math.pi - 1e-9:
        raise ValueError("the points coincide or are antipodal: the initial bearing is not unique")
    dl = l2 - l1
    t = math.atan2(math.sin(dl) * math.cos(f2), math.cos(f1) * math.sin(f2) - math.sin(f1) * math.cos(f2) * math.cos(dl))
    return {"bearing": (math.degrees(t) + 360.0) % 360.0}


def _ref_bearing(lat1, lon1, lat2, lon2):
    """Project the destination onto the local east and north directions at the start (numpy)."""
    import numpy as np
    f1, l1, f2, l2 = (np.radians(v) for v in (lat1, lon1, lat2, lon2))
    p2 = np.array([np.cos(f2) * np.cos(l2), np.cos(f2) * np.sin(l2), np.sin(f2)])
    east = np.array([-np.sin(l1), np.cos(l1), 0.0])
    north = np.array([-np.sin(f1) * np.cos(l1), -np.sin(f1) * np.sin(l1), np.cos(f1)])
    return {"bearing": float((np.degrees(np.arctan2(p2 @ east, p2 @ north)) + 360.0) % 360.0)}


def _verify_bearing(i, o):
    """Walk a tiny step from the start along the bearing: the great-circle distance to the target must shrink at the
    full rate (the cosine of the angle off the true direction is 1)."""
    f1, l1, f2, l2 = (math.radians(i[k]) for k in ("lat1", "lon1", "lat2", "lon2"))
    b = math.radians(o["bearing"])
    a = _unit(f1, l1)
    east, north = (-math.sin(l1), math.cos(l1), 0.0), (-math.sin(f1) * math.cos(l1), -math.sin(f1) * math.sin(l1), math.cos(f1))
    tdir = tuple(math.sin(b) * east[j] + math.cos(b) * north[j] for j in range(3))
    t = _unit(f2, l2)
    g = tuple(t[j] - _vdot(a, t) * a[j] for j in range(3))   # true direction toward the target in the tangent plane
    gn = _vnorm(g)
    return abs(_vdot(tdir, g) / gn - 1) + _vnorm(_vcross(tdir, g)) / gn


def parse_bearing(q0):
    q = _norm(q0)
    if not re.search(r"\b(bearing|initial heading|initial course|forward azimuth|azimuth)\b", q):
        return None
    p = _pts2(q)
    if len(p) != 2:
        return None if p else {}
    return {"lat1": p[0][0], "lon1": p[0][1], "lat2": p[1][0], "lon2": p[1][1]}


_add("initial_bearing", "advanced",
     name="Initial bearing (forward azimuth) along a great circle", topic="spherical geometry / navigation",
     purpose="The initial compass bearing from point 1 to point 2 along the great circle, degrees clockwise from north.",
     inputs=[_I("lat1", "latitude of the start, degrees"), _I("lon1", "longitude of the start"), _I("lat2", "latitude of the destination"), _I("lon2", "longitude of the destination")],
     outputs=[_O("bearing", "initial bearing, degrees in [0, 360), clockwise from north")], primary="bearing",
     conditions=["(latitude, longitude) in signed decimal degrees", "the start is not a pole; the points are neither equal nor antipodal"],
     method="theta = atan2(sin dl cos f2, cos f1 sin f2 - sin f1 cos f2 cos dl)",
     latex=r"\theta = \operatorname{atan2}\big(\sin\Delta\lambda\cos\varphi_2,\ \cos\varphi_1\sin\varphi_2 - \sin\varphi_1\cos\varphi_2\cos\Delta\lambda\big)",
     run=initial_bearing, verify=_verify_bearing, parse=parse_bearing, reference=_ref_bearing,
     wrong=lambda lat1, lon1, lat2, lon2: {"bearing": (math.degrees(math.atan2(lon2 - lon1, lat2 - lat1)) + 360.0) % 360.0},
     random_inputs=lambda rng: (lambda r: {k: r[k] for k in ("lat1", "lon1", "lat2", "lon2")})(_rand_gc(rng)),
     templates=["Find the initial bearing from (0, 0) to (0, 90)."],
     examples=[{"inputs": {"lat1": 0, "lon1": 0, "lat2": 0, "lon2": 90}, "outputs": {"bearing": 90.0}},
               {"inputs": {"lat1": 0, "lon1": 0, "lat2": 45, "lon2": 0}, "outputs": {"bearing": 0.0}}],
     questions=[{"q": "Find the initial bearing from (0, 0) to (0, 90).", "outputs": {"bearing": 90.0}},
                {"q": "What is the bearing from (10, 20) to (10, 20)?", "outcome": "INVALID_INPUT"}])


def spherical_triangle(points, radius=1.0):
    P = _pairs(points, "points", 3, 3)
    v = [_latlon(a, b, f"vertex {k + 1}") for k, (a, b) in enumerate(P)]
    R = _pos(radius, "radius")
    U = [_unit(*x) for x in v]
    if abs(_vdot(U[0], _vcross(U[1], U[2]))) < 1e-12:
        raise ValueError("the vertices lie on one great circle (or two coincide): no spherical triangle")
    a, b, c = _central(v[1], v[2]), _central(v[0], v[2]), _central(v[0], v[1])
    s = (a + b + c) / 2
    t = math.tan(s / 2) * math.tan((s - a) / 2) * math.tan((s - b) / 2) * math.tan((s - c) / 2)
    E = 4 * math.atan(math.sqrt(max(0.0, t)))
    return {"excess": math.degrees(E), "area": E * R * R}


def _ref_sph_tri(points, radius=1.0):
    """Girard's theorem: the vertex angles from tangent vectors (numpy), E = A + B + C - pi."""
    import numpy as np
    P = _pairs(points, "points", 3, 3)
    U = [np.array(_unit(np.radians(a), np.radians(b))) for a, b in P]
    ang = 0.0
    for k in range(3):
        A, B, C = U[k], U[(k + 1) % 3], U[(k + 2) % 3]
        tb, tc = B - (A @ B) * A, C - (A @ C) * A
        ang += np.arctan2(np.linalg.norm(np.cross(tb, tc)), tb @ tc)
    E = ang - np.pi
    return {"excess": float(np.degrees(E)), "area": float(E * radius ** 2)}


def _verify_sph(i, o):
    """Van Oosterom-Strackee: the solid angle of the triangle seen from the centre equals the excess."""
    P = _pairs(i["points"], "points", 3, 3)
    a, b, c = [_unit(math.radians(x), math.radians(y)) for x, y in P]
    num = abs(_vdot(a, _vcross(b, c)))
    den = 1 + _vdot(a, b) + _vdot(b, c) + _vdot(a, c)
    E = 2 * math.atan2(num, den)
    return _res((math.radians(o["excess"]), E), (o["area"], E * i.get("radius", 1.0) ** 2))


def _rand_sph(rng):
    while True:
        lat0, lon0 = rng.uniform(-60, 60), rng.uniform(-180, 180)
        P = [(round(lat0 + rng.uniform(-25, 25), 3), round(lon0 + rng.uniform(-30, 30), 3)) for _ in range(3)]
        try:
            r = spherical_triangle(_flat(P))
        except ValueError:
            continue
        if r["excess"] > 0.5:
            return {"points": _flat(P), "radius": rng.choice([1.0, 6371.0, _rand_len(rng, 1, 100)])}


def parse_sph(q0):
    q = _norm(q0)
    if not re.search(r"\bspherical triangle\b|\btriangle on (?:the|a) (?:sphere|earth|globe)\b", q):
        return None
    p = _pts2(q)
    d = {"radius": _after(q, r"radius")}
    if re.search(r"\bearth\b", q) and d["radius"] is None:
        d["radius"] = 6371.0
    if len(p) == 3:
        d["points"] = _flat(p)
    elif p:
        return None
    return _clean(d)


_add("spherical_triangle_area", "expert",
     name="Area and spherical excess of a spherical triangle", topic="spherical geometry / spherical triangles",
     purpose="The spherical excess and area of the triangle with three vertices given by latitude and longitude on a sphere of radius R.",
     inputs=[_V("points", "the vertices (lat lon): lat1 lon1, lat2 lon2, lat3 lon3, degrees", n_min=6, n_max=6),
             _I("radius", "sphere radius (default 1; 'Earth' means 6371 km)", required=False)],
     outputs=[_O("excess", "spherical excess A + B + C - 180, degrees"), _O("area", "area E R^2 (E in radians)")], primary="area",
     conditions=["the smaller triangle (each side less than 180 degrees)", "vertices not on one great circle", "radius defaults to 1"],
     method="side arcs by the haversine formula, then L'Huilier's theorem", latex=r"\tan\frac E4 = \sqrt{\tan\frac s2\tan\frac{s-a}2\tan\frac{s-b}2\tan\frac{s-c}2},\quad A = E R^2",
     run=spherical_triangle, verify=_verify_sph, parse=parse_sph, reference=_ref_sph_tri,
     wrong=lambda points, radius=1.0: (lambda r: dict(r, area=r["area"] * 2, excess=r["excess"] * 2))(spherical_triangle(points, radius)),
     random_inputs=_rand_sph,
     templates=["Find the area of the spherical triangle with vertices (0, 0), (0, 90) and (90, 0) on a sphere of radius 1."],
     examples=[{"inputs": {"points": "0 0, 0 90, 90 0", "radius": 1}, "outputs": {"excess": 90.0, "area": math.pi / 2}},
               {"inputs": {"points": "0 0, 0 90, 90 0", "radius": 2}, "outputs": {"area": 2 * math.pi}}],
     questions=[{"q": "Find the area of the spherical triangle with vertices (0, 0), (0, 90) and (90, 0) on a sphere of radius 1.", "outputs": {"excess": 90.0, "area": math.pi / 2}},
                {"q": "Find the area of the spherical triangle with vertices (0, 0), (0, 30) and (0, 60) on a sphere of radius 1.", "outcome": "INVALID_INPUT"}])


def solid_angle_cone(half_angle):
    a = _fin(half_angle, "half_angle")
    if not 0 < a <= 180:
        raise ValueError("the half-angle must be between 0 and 180 degrees")
    om = 4 * math.pi * math.sin(math.radians(a) / 2) ** 2
    return {"solid_angle": om, "fraction_of_sphere": om / (4 * math.pi)}


def _ref_solid_cone(half_angle):
    import sympy as sp
    th, ph = sp.symbols("theta phi")
    a = sp.pi * _R(half_angle) / 180
    om = sp.integrate(sp.integrate(sp.sin(th), (th, 0, a)), (ph, 0, 2 * sp.pi))
    return {"solid_angle": _F(om), "fraction_of_sphere": _F(om / (4 * sp.pi))}


def parse_solid_cone(q0):
    q = _norm(q0)
    if not re.search(r"\bsolid angle\b", q) or not re.search(r"\bcone\b", q) or re.search(r"\(", q):
        return None
    full = re.search(r"\b(apex angle|opening angle|full angle|vertex angle|aperture)\b", q)
    t = _angle_in(q, r"half-angle|half angle|apex angle|opening angle|full angle|vertex angle|aperture|angle")
    return _clean({"half_angle": t / 2 if (t is not None and full) else t})


_add("solid_angle_cone", "intermediate",
     name="Solid angle of a cone", topic="solid geometry / solid angles",
     purpose="The solid angle (steradians) subtended at the apex by a right circular cone of given half-angle.",
     inputs=[_I("half_angle", "half-angle of the cone (axis to side), degrees")],
     outputs=[_O("solid_angle", "2 pi (1 - cos a) steradians"), _O("fraction_of_sphere", "solid angle / (4 pi)")], primary="solid_angle",
     conditions=["0 < half-angle <= 180 degrees", "an apex (full opening) angle in the question is halved"],
     method="Omega = 2 pi (1 - cos a) = 4 pi sin^2(a/2) (no cancellation for small a)", latex=r"\Omega = 2\pi(1-\cos\alpha) = 4\pi\sin^2\tfrac{\alpha}{2}",
     run=solid_angle_cone, verify=lambda i, o: _res((o["solid_angle"], 2 * math.pi * (1 - math.cos(math.radians(i["half_angle"])))), (o["fraction_of_sphere"] * 4 * math.pi, o["solid_angle"])),
     parse=parse_solid_cone, reference=_ref_solid_cone,
     wrong=lambda half_angle: (lambda om: {"solid_angle": om, "fraction_of_sphere": om / (4 * math.pi)})(math.pi * math.sin(math.radians(half_angle)) ** 2),
     random_inputs=lambda rng: {"half_angle": round(rng.uniform(5, 175), 2)},
     templates=["What is the solid angle of a cone with half-angle 60 degrees?"],
     examples=[{"inputs": {"half_angle": 60}, "outputs": {"solid_angle": math.pi, "fraction_of_sphere": 0.25}},
               {"inputs": {"half_angle": 90}, "outputs": {"solid_angle": 2 * math.pi}}],
     questions=[{"q": "What is the solid angle of a cone with half-angle 60 degrees?", "outputs": {"solid_angle": math.pi}},
                {"q": "Find the solid angle of a cone with apex angle 180 degrees.", "outputs": {"solid_angle": 2 * math.pi}},
                {"q": "What is the solid angle of a cone with half-angle 200 degrees?", "outcome": "INVALID_INPUT"}])


def solid_angle_triangle(points):
    a, b, c = _triples(points, "points", 3)
    if 0.0 in (_vnorm(a), _vnorm(b), _vnorm(c)):
        raise ValueError("a vertex is at the origin (the observer)")
    if _vdot(_xvec(a), _vcross(_xvec(b), _xvec(c))) == 0:
        raise ValueError("the triangle's plane passes through the origin: it subtends no solid angle")
    la, lb, lc = _vnorm(a), _vnorm(b), _vnorm(c)
    num = abs(_vdot(a, _vcross(b, c)))
    den = la * lb * lc + _vdot(a, b) * lc + _vdot(a, c) * lb + _vdot(b, c) * la
    return {"solid_angle": 2 * math.atan2(num, den)}


def _ref_solid_tri(points):
    """Girard: project the vertices to the unit sphere, sum the vertex angles (numpy)."""
    import numpy as np
    U = [np.array(p, float) / np.linalg.norm(p) for p in _triples(points, "points", 3)]
    ang = 0.0
    for k in range(3):
        A, B, C = U[k], U[(k + 1) % 3], U[(k + 2) % 3]
        tb, tc = B - (A @ B) * A, C - (A @ C) * A
        ang += np.arctan2(np.linalg.norm(np.cross(tb, tc)), tb @ tc)
    return {"solid_angle": float(ang - np.pi)}


def _verify_solid_tri(i, o):
    """L'Huilier on the unit-sphere triangle of the three directions."""
    P = _triples(i["points"], "points", 3)
    U = [tuple(c / _vnorm(p) for c in p) for p in P]
    arc = lambda u, v: math.atan2(_vnorm(_vcross(u, v)), _vdot(u, v))
    a, b, c = arc(U[1], U[2]), arc(U[0], U[2]), arc(U[0], U[1])
    s = (a + b + c) / 2
    E = 4 * math.atan(math.sqrt(max(0.0, math.tan(s / 2) * math.tan((s - a) / 2) * math.tan((s - b) / 2) * math.tan((s - c) / 2))))
    return _rel(o["solid_angle"], E)


def _rand_solid_tri(rng):
    while True:
        P = [(rng.uniform(-5, 5), rng.uniform(-5, 5), rng.uniform(2, 10)) for _ in range(3)]
        P = [tuple(round(c, 3) for c in p) for p in P]
        try:
            if solid_angle_triangle(_flat(P))["solid_angle"] > 0.01:
                return {"points": _flat(P)}
        except ValueError:
            pass


def parse_solid_tri(q0):
    q = _norm(q0)
    if not re.search(r"\bsolid angle\b", q) or not re.search(r"\btriangle\b", q) or not re.search(r"\borigin\b", q):
        return None
    p = _pts3(q)
    return {"points": _flat(p)} if len(p) == 3 else (None if p else {})


_add("solid_angle_triangle", "expert",
     name="Solid angle subtended by a triangle", topic="solid geometry / solid angles",
     purpose="The solid angle (steradians) that the triangle with vertices A, B, C subtends at the origin.",
     inputs=[_V("points", "the vertices as seen from the origin: x y z for each", n_min=9, n_max=9)],
     outputs=[_O("solid_angle", "solid angle, steradians")], primary="solid_angle",
     conditions=["no vertex at the origin", "the triangle's plane does not pass through the origin", "observer at the origin"],
     method="Van Oosterom-Strackee: tan(Omega/2) = |a.(b x c)| / (abc + (a.b)c + (a.c)b + (b.c)a)",
     latex=r"\tan\frac{\Omega}{2} = \frac{|\mathbf a\cdot(\mathbf b\times\mathbf c)|}{abc + (\mathbf a\cdot\mathbf b)c + (\mathbf a\cdot\mathbf c)b + (\mathbf b\cdot\mathbf c)a}",
     run=solid_angle_triangle, verify=_verify_solid_tri, parse=parse_solid_tri, reference=_ref_solid_tri,
     wrong=lambda points: {"solid_angle": solid_angle_triangle(points)["solid_angle"] / 2}, random_inputs=_rand_solid_tri,
     templates=["What solid angle does the triangle with vertices (1, 0, 0), (0, 1, 0), (0, 0, 1) subtend at the origin?"],
     examples=[{"inputs": {"points": "1 0 0, 0 1 0, 0 0 1"}, "outputs": {"solid_angle": math.pi / 2}},
               {"inputs": {"points": "2 0 0, 0 3 0, 0 0 5"}, "outputs": {"solid_angle": math.pi / 2}}],
     questions=[{"q": "What solid angle does the triangle with vertices (1, 0, 0), (0, 1, 0), (0, 0, 1) subtend at the origin?", "outputs": {"solid_angle": math.pi / 2}},
                {"q": "What solid angle does the triangle with vertices (1, 0, 0), (0, 1, 0), (1, 1, 0) subtend at the origin?", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# curves and surfaces (advanced / expert)
# ==================================================================================================================
def _poly_x(expr):
    """Coefficients {degree: c} of a polynomial in x written term by term ('3x^3 - 2x + 1'), or None."""
    s = expr.replace(" ", "").replace("*", "")
    if not s:
        return None
    if s[0] not in "+-":
        s = "+" + s
    terms = re.findall(r"[+-][^+-]+", s)
    if "".join(terms) != s:
        return None
    out = {}
    for t in terms:
        m = re.fullmatch(r"([+-])((?:\d+(?:\.\d+)?|\.\d+)(?:/\d+)?)?(x(?:\^(\d+))?)?", t)
        if not m or (m.group(2) is None and m.group(3) is None):
            return None
        c = float(Fraction(m.group(2))) if m.group(2) else 1.0
        d = (int(m.group(4)) if m.group(4) else 1) if m.group(3) else 0
        out[d] = out.get(d, 0.0) + (-c if m.group(1) == "-" else c)
    return out


def _graph_poly(q):
    m = re.search(r"\by\s*=\s*([-+0-9.x^*/ ]+?)(?=\s*(?:,|\bat\b|\bfrom\b|\bbetween\b|\bwhere\b|\bfor\b|\?|$|\.\s|\.$))", q)
    return _poly_x(m.group(1)) if m else None


def _horner(co, x):
    v = 0.0
    for c in co:
        v = v * x + c
    return v


def _deriv(co):
    n = len(co) - 1
    return [c * (n - k) for k, c in enumerate(co[:-1])] or [0.0]


def curvature_graph(coefficients, x0):
    co = [_fin(float(c), "coefficients") for c in coefficients]
    x0 = _fin(x0, "x0")
    if len(co) > 11:
        raise ValueError("polynomials up to degree 10")
    d1, d2 = _deriv(co), _deriv(_deriv(co))
    yp, ypp = _horner(d1, x0), _horner(d2, x0)
    k = abs(ypp) / (1 + yp * yp) ** 1.5
    return {"curvature": k, "radius_of_curvature": 1 / k if k else None, "slope": yp}


def _verify_curv(i, o):
    co = [float(c) for c in i["coefficients"]]
    x0 = i["x0"]
    h = 1e-3 * max(1.0, abs(x0))
    # the osculating circle: its centre lies on the normal at distance 1/k; check with an independent central-difference
    # estimate of y'' (fourth-order), loose, plus exact algebraic consistency of the reported values
    yp = _horner(_deriv(co), x0)
    f = lambda x: _horner(co, x)
    ypp_fd = (-f(x0 + 2 * h) + 16 * f(x0 + h) - 30 * f(x0) + 16 * f(x0 - h) - f(x0 - 2 * h)) / (12 * h * h)
    k_fd = abs(ypp_fd) / (1 + yp * yp) ** 1.5
    if abs(o["curvature"] - k_fd) > 1e-4 * max(1.0, k_fd):
        return 1.0
    ypp = _horner(_deriv(_deriv(co)), x0)
    r = _res((o["slope"], yp), (o["curvature"] * (1 + yp * yp) ** 1.5, abs(ypp)))
    if o["curvature"]:
        r = max(r, _rel(o["radius_of_curvature"] * o["curvature"], 1.0))
    return r


def _ref_curv(coefficients, x0):
    import sympy as sp
    t = sp.Symbol("t")
    co = [_R(c) for c in coefficients]
    p = sum(c * t ** (len(co) - 1 - k) for k, c in enumerate(co))
    X = sp.Matrix([t, p, 0])
    d1, d2 = X.diff(t), X.diff(t, 2)
    k = (d1.cross(d2).norm() / d1.norm() ** 3).subs(t, _R(x0))
    return {"curvature": _F(k), "slope": _F(sp.diff(p, t).subs(t, _R(x0)))}


def _rand_curv(rng):
    while True:
        co = [float(rng.randint(-5, 5)) for _ in range(rng.randint(3, 5))]
        x0 = round(rng.uniform(-3, 3), 2)
        if co[0] and abs(_horner(_deriv(_deriv(co)), x0)) > 0.1:
            return {"coefficients": co, "x0": x0}


def parse_curv(q0):
    q = _norm(q0)
    if not re.search(r"\b(curvature|radius of curvature|osculating circle)\b", q) or re.search(r"\b(helix|ellipse|torus|surface|z\s*=)", q):
        return None
    pol = _graph_poly(q)
    m = re.search(r"\bat\s*x\s*=\s*(" + _N + r")", q) or re.search(r"\bat\s*\(\s*(" + _N + r")\s*,", q)
    if pol is None:
        return None
    n = max(pol)
    d = {"coefficients": [pol.get(n - k, 0.0) for k in range(n + 1)]}
    if m:
        d["x0"] = float(m.group(1))
    return d


_add("curvature_of_graph", "advanced",
     name="Curvature of a polynomial graph at a point", topic="curves / curvature",
     purpose="The curvature and radius of curvature of y = p(x) at x = x0 for a polynomial p.",
     inputs=[_V("coefficients", "coefficients from the highest power down: a_n, ..., a_1, a_0", n_min=1), _I("x0", "the x-coordinate of the point")],
     outputs=[_O("curvature", "|y''| / (1 + y'^2)^(3/2)"), _O("radius_of_curvature", "1 / curvature (none where the curvature is 0)"), _O("slope", "y'(x0)")],
     primary="curvature", conditions=["polynomial of degree at most 10"], method="exact derivatives of the polynomial (Horner), then k = |y''|/(1 + y'^2)^(3/2)",
     latex=r"\kappa = \frac{|y''|}{(1+y'^2)^{3/2}},\quad R = \frac1\kappa", run=curvature_graph, verify=_verify_curv, parse=parse_curv, reference=_ref_curv,
     wrong=lambda coefficients, x0: (lambda r: dict(r, curvature=abs(_horner(_deriv(_deriv([float(c) for c in coefficients])), x0))))(curvature_graph(coefficients, x0)),
     random_inputs=_rand_curv,
     templates=["Find the curvature of y = x^2 at x = 0.", "What is the radius of curvature of y = x^3 - 2x at x = 1?"],
     examples=[{"inputs": {"coefficients": "1 0 0", "x0": 0}, "outputs": {"curvature": 2.0, "radius_of_curvature": 0.5}},
               {"inputs": {"coefficients": [1, 0, -2, 0], "x0": 1}, "outputs": {"curvature": 6 / 2 ** 1.5, "slope": 1.0}}],
     questions=[{"q": "Find the curvature of y = x^2 at x = 0.", "outputs": {"curvature": 2.0}},
                {"q": "What is the radius of curvature of y = x^3 - 2x at x = 1?", "outputs": {"radius_of_curvature": 2 ** 1.5 / 6}},
                {"q": "Find the curvature of y = x^2.", "outcome": "NEEDS_INPUT"}])


def helix_measures(radius, pitch):
    r, p = _pos(radius, "radius"), _fin(pitch, "pitch")
    c = p / (2 * math.pi)
    den = r * r + c * c
    return {"curvature": r / den, "torsion": c / den, "length_per_turn": math.hypot(2 * math.pi * r, p)}


def _ref_helix(radius, pitch):
    """Frenet-Serret from sympy's exact derivatives at t = 0; the length per turn by scipy quadrature of |X'(t)|."""
    import sympy as sp
    import numpy as np
    from scipy.integrate import quad
    t = sp.Symbol("t", real=True)
    r, c = _R(radius), _R(pitch) / (2 * sp.pi)
    X = sp.Matrix([r * sp.cos(t), r * sp.sin(t), c * t])
    d1, d2, d3 = (X.diff(t, k).subs(t, 0) for k in (1, 2, 3))
    cr = d1.cross(d2)
    k = sp.sqrt(cr.dot(cr)) / sp.sqrt(d1.dot(d1)) ** 3
    tau = cr.dot(d3) / cr.dot(cr)
    cf = pitch / (2 * np.pi)
    L = quad(lambda s_: np.sqrt((radius * np.sin(s_)) ** 2 + (radius * np.cos(s_)) ** 2 + cf * cf), 0, 2 * np.pi, epsabs=0, epsrel=2e-14)[0]
    return {"curvature": _F(k), "torsion": _F(tau), "length_per_turn": float(L)}


def parse_helix(q0):
    q = _norm(q0)
    if not re.search(r"\bhelix\b|\bhelical\b|\bspring\b|\bcoil\b", q) or re.search(r"\(", q):
        return None
    return _scoped(q, _clean({"radius": _after(q, r"radius"), "pitch": _after(q, r"pitch|rise per turn|rise of")}))


_add("helix_measures", "expert",
     name="Helix: curvature, torsion and length per turn", topic="curves / space curves",
     purpose="Curvature, torsion and arc length per turn of the circular helix of radius r and pitch p (rise per turn).",
     inputs=[_I("radius", "radius r of the helix"), _I("pitch", "pitch p: rise per full turn (negative: left-handed)")],
     outputs=[_O("curvature", "r / (r^2 + c^2), c = p / (2 pi)"), _O("torsion", "c / (r^2 + c^2)"), _O("length_per_turn", "sqrt((2 pi r)^2 + p^2)")],
     primary="curvature", conditions=["radius > 0", "parametrised (r cos t, r sin t, c t)"], method="Frenet-Serret closed forms for the circular helix",
     latex=r"\kappa = \frac{r}{r^2+c^2},\quad \tau = \frac{c}{r^2+c^2},\quad c = \frac{p}{2\pi}", run=helix_measures,
     verify=lambda i, o: (lambda r, c: _res((o["curvature"] * (r * r + c * c), r), (o["torsion"] * (r * r + c * c), c),
                                            (o["length_per_turn"] ** 2, 4 * math.pi ** 2 * (r * r + c * c))))(i["radius"], i["pitch"] / (2 * math.pi)),
     parse=parse_helix, reference=_ref_helix,
     wrong=lambda radius, pitch: (lambda c: {"curvature": 1 / radius, "torsion": c / (radius * radius + c * c), "length_per_turn": 2 * math.pi * radius})(pitch / (2 * math.pi)),
     random_inputs=lambda rng: {"radius": _rand_len(rng, 0.2, 20), "pitch": round(rng.uniform(-30, 30), 2) or 1.0},
     templates=["Find the curvature and torsion of a helix with radius 2 and pitch 6.283185307179586."],
     examples=[{"inputs": {"radius": 1, "pitch": 2 * math.pi}, "outputs": {"curvature": 0.5, "torsion": 0.5, "length_per_turn": 2 * math.pi * math.sqrt(2)}},
               {"inputs": {"radius": 3, "pitch": 8 * math.pi}, "outputs": {"curvature": 3 / 25, "torsion": 4 / 25}}],
     questions=[{"q": "Find the curvature and torsion of a helix with radius 1 and pitch 6.283185307179586.", "outputs": {"curvature": 0.5, "torsion": 0.5}},
                {"q": "A spring is a helix of radius 3 cm and pitch 25.132741228718345 cm. Find its curvature.", "outputs": {"curvature": 0.12}},
                {"q": "Find the curvature of a helix with radius 2.", "outcome": "NEEDS_INPUT"}])


def ellipse_curvature(a, b, t):
    a, b, t = _pos(a, "a"), _pos(b, "b"), _fin(t, "t")
    tr = math.radians(t)
    q = (a * math.sin(tr)) ** 2 + (b * math.cos(tr)) ** 2
    k = a * b / q ** 1.5
    return {"curvature": k, "radius_of_curvature": 1 / k, "x": a * math.cos(tr) + 0.0, "y": b * math.sin(tr) + 0.0}


def _ref_ellipse_curv(a, b, t):
    import sympy as sp
    s = sp.Symbol("s", real=True)
    A, B = _R(a), _R(b)
    x, y = A * sp.cos(s), B * sp.sin(s)
    k = sp.Abs(sp.diff(x, s) * sp.diff(y, s, 2) - sp.diff(y, s) * sp.diff(x, s, 2)) / (sp.diff(x, s) ** 2 + sp.diff(y, s) ** 2) ** sp.Rational(3, 2)
    val = k.subs(s, sp.pi * _R(t) / 180)
    return {"curvature": _F(val), "radius_of_curvature": _F(1 / val)}


def parse_ellipse_curv(q0):
    q = _norm(q0)
    if not re.search(r"\b(curvature|radius of curvature)\b", q) or not re.search(r"\bellipse\b", q):
        return None
    a = _after(q, r"semi-major axis|semi-axis a|a")
    b = _after(q, r"semi-minor axis|semi-axis b|b")
    m = re.search(r"x\^2\s*/\s*(" + _N + r")\s*\+\s*y\^2\s*/\s*(" + _N + r")\s*=\s*1\b", q)
    if m:
        a, b = math.sqrt(float(m.group(1))), math.sqrt(float(m.group(2)))
    t = _angle_in(q, r"parameter|t|eccentric anomaly")
    if re.search(r"\bend of the major axis\b", q):
        t = 0.0
    if re.search(r"\bend of the minor axis\b", q):
        t = 90.0
    return _clean({"a": a, "b": b, "t": t})


_add("ellipse_curvature", "advanced",
     name="Curvature of an ellipse at a point", topic="curves / curvature",
     purpose="The curvature and radius of curvature of the ellipse (a cos t, b sin t) at parameter t (eccentric anomaly).",
     inputs=[_I("a", "semi-axis along x"), _I("b", "semi-axis along y"), _I("t", "parameter t (eccentric anomaly), degrees")],
     outputs=[_O("curvature", "ab / (a^2 sin^2 t + b^2 cos^2 t)^(3/2)"), _O("radius_of_curvature", "1 / curvature"), _O("x", "x of the point"), _O("y", "y of the point")],
     primary="curvature", conditions=["a, b > 0", "the point is (a cos t, b sin t); 'end of the major/minor axis' means t = 0 / 90 degrees"],
     method="parametric curvature |x'y'' - y'x''| / (x'^2 + y'^2)^(3/2) in closed form", latex=r"\kappa(t) = \frac{ab}{(a^2\sin^2 t + b^2\cos^2 t)^{3/2}}",
     run=ellipse_curvature,
     verify=lambda i, o: (lambda a, b, t: _res((o["curvature"] * ((a * math.sin(t)) ** 2 + (b * math.cos(t)) ** 2) ** 1.5, a * b),
                                               (o["radius_of_curvature"] * o["curvature"], 1.0), ((o["x"] / a) ** 2 + (o["y"] / b) ** 2, 1.0)))(i["a"], i["b"], math.radians(i["t"])),
     parse=parse_ellipse_curv, reference=_ref_ellipse_curv,
     wrong=lambda a, b, t: (lambda r: dict(r, curvature=a * b / ((a * math.cos(math.radians(t))) ** 2 + (b * math.sin(math.radians(t))) ** 2) ** 1.5))(
         ellipse_curvature(a, b, t)) if a != b else dict(ellipse_curvature(a, b, t), curvature=1 / a ** 2),
     random_inputs=lambda rng: {"a": _rand_len(rng, 0.5, 30), "b": _rand_len(rng, 0.5, 30), "t": round(rng.choice([rng.uniform(5, 85), rng.uniform(95, 355)]), 2)},
     templates=["Find the curvature of the ellipse with semi-major axis 5 and semi-minor axis 3 at the end of the major axis."],
     examples=[{"inputs": {"a": 5, "b": 3, "t": 0}, "outputs": {"curvature": 5 / 9, "radius_of_curvature": 1.8}},
               {"inputs": {"a": 5, "b": 3, "t": 90}, "outputs": {"curvature": 3 / 25}}],
     questions=[{"q": "Find the curvature of the ellipse with semi-major axis 5 and semi-minor axis 3 at the end of the major axis.", "outputs": {"curvature": 5 / 9}},
                {"q": "Find the radius of curvature of the ellipse x^2/25 + y^2/9 = 1 at the end of the minor axis.", "outputs": {"radius_of_curvature": 25 / 3}}])


def surface_curvature(A, B, C, D, E, x0, y0, F=0.0):
    for n, v in (("A", A), ("B", B), ("C", C), ("D", D), ("E", E), ("x0", x0), ("y0", y0), ("F", F)):
        _fin(v, n)
    p, q = 2 * A * x0 + B * y0 + D, B * x0 + 2 * C * y0 + E
    r, s, t = 2 * A, B, 2 * C
    w = 1 + p * p + q * q
    return {"gaussian": (r * t - s * s) / (w * w) + 0.0, "mean": ((1 + q * q) * r - 2 * p * q * s + (1 + p * p) * t) / (2 * w ** 1.5) + 0.0}


def _verify_surf(i, o):
    """Principal curvatures of the shape operator built from the fundamental forms: K = k1 k2, H = (k1 + k2)/2."""
    A, B, C, D, E, x0, y0 = (i[k] for k in ("A", "B", "C", "D", "E", "x0", "y0"))
    p, q = 2 * A * x0 + B * y0 + D, B * x0 + 2 * C * y0 + E
    EE, FF, GG = 1 + p * p, p * q, 1 + q * q
    nn = math.sqrt(1 + p * p + q * q)
    L, M, N = 2 * A / nn, B / nn, 2 * C / nn
    det = EE * GG - FF * FF
    K = (L * N - M * M) / det
    H = (EE * N - 2 * FF * M + GG * L) / (2 * det)
    sc = max(1.0, abs(A), abs(B), abs(C))
    return max(abs(o["gaussian"] - K) / max(abs(K), sc * sc * 1e-6), abs(o["mean"] - H) / max(abs(H), sc * 1e-6))


def _ref_surf(A, B, C, D, E, x0, y0, F=0.0):
    import sympy as sp
    x, y = sp.symbols("x y", real=True)
    f = _R(A) * x ** 2 + _R(B) * x * y + _R(C) * y ** 2 + _R(D) * x + _R(E) * y + _R(F)
    X = sp.Matrix([x, y, f])
    Xx, Xy = X.diff(x), X.diff(y)
    n = Xx.cross(Xy)
    n = n / n.norm()
    EE, FF, GG = Xx.dot(Xx), Xx.dot(Xy), Xy.dot(Xy)
    L, M, N = X.diff(x, 2).dot(n), X.diff(x, y).dot(n), X.diff(y, 2).dot(n)
    pt = {x: _R(x0), y: _R(y0)}
    K = ((L * N - M ** 2) / (EE * GG - FF ** 2)).subs(pt)
    H = ((EE * N - 2 * FF * M + GG * L) / (2 * (EE * GG - FF ** 2))).subs(pt)
    return {"gaussian": _F(K), "mean": _F(H)}


def parse_surf(q0):
    q = _norm(q0)
    if not re.search(r"\b(gaussian curvature|mean curvature|curvature)\b", q) or not re.search(r"\bz\s*=", q):
        return None
    m = re.search(r"\bz\s*=\s*([-+0-9.xy^*/ ]+?)(?=\s*(?:,|\bat\b|\?|\.?$))", q)
    e = _terms(m.group(1)) if m else None
    if e is None:
        return None
    p = _pts2(q)
    d = {"A": e["xx"], "B": e["xy"], "C": e["yy"], "D": e["x"], "E": e["y"], "F": e["1"]}
    if len(p) == 1:
        d.update(x0=p[0][0], y0=p[0][1])
    return d


_add("surface_curvature", "expert",
     name="Gaussian and mean curvature of a quadratic surface z = f(x, y)", topic="surfaces / curvature",
     purpose="Gaussian curvature K and mean curvature H of z = A x^2 + B xy + C y^2 + D x + E y + F at the point above (x0, y0).",
     inputs=[_I(k, m) for k, m in (("A", "coefficient of x^2"), ("B", "coefficient of xy"), ("C", "coefficient of y^2"), ("D", "coefficient of x"),
                                   ("E", "coefficient of y"), ("x0", "x of the point"), ("y0", "y of the point"))] + [_I("F", "constant term (does not affect curvature)", required=False)],
     outputs=[_O("gaussian", "Gaussian curvature K"), _O("mean", "mean curvature H (upward normal: positive where the surface opens upward)")], primary="gaussian",
     conditions=["the unit normal points upward, (-f_x, -f_y, 1)/|...|"],
     method="Monge patch: K = (f_xx f_yy - f_xy^2)/(1 + f_x^2 + f_y^2)^2, H = ((1 + f_y^2) f_xx - 2 f_x f_y f_xy + (1 + f_x^2) f_yy) / (2 (1 + f_x^2 + f_y^2)^(3/2))",
     latex=r"K = \frac{f_{xx}f_{yy}-f_{xy}^2}{(1+f_x^2+f_y^2)^2},\quad H = \frac{(1+f_y^2)f_{xx} - 2f_xf_yf_{xy} + (1+f_x^2)f_{yy}}{2(1+f_x^2+f_y^2)^{3/2}}",
     run=surface_curvature, verify=_verify_surf, parse=parse_surf, reference=_ref_surf,
     wrong=lambda A, B, C, D, E, x0, y0, F=0.0: (lambda r: dict(r, gaussian=4 * A * C - B * B))(surface_curvature(A, B, C, D, E, x0, y0, F)),
     random_inputs=lambda rng: {**{k: float(rng.randint(-5, 5)) for k in "ABCDE"}, "x0": round(rng.uniform(-2, 2), 2), "y0": round(rng.uniform(-2, 2), 2),
                                "F": float(rng.randint(-5, 5))} | ({"A": 1.0, "C": 2.0} if rng.random() < 0.3 else {}),
     templates=["Find the Gaussian and mean curvature of z = x^2 + y^2 at (0, 0).", "Find the Gaussian curvature of z = x^2 - y^2 + 2x at (1, 1)."],
     examples=[{"inputs": {"A": 1, "B": 0, "C": 1, "D": 0, "E": 0, "x0": 0, "y0": 0}, "outputs": {"gaussian": 4.0, "mean": 2.0}},
               {"inputs": {"A": 1, "B": 0, "C": -1, "D": 0, "E": 0, "x0": 0, "y0": 0}, "outputs": {"gaussian": -4.0, "mean": 0.0}}],
     questions=[{"q": "Find the Gaussian and mean curvature of z = x^2 + y^2 at (0, 0).", "outputs": {"gaussian": 4.0, "mean": 2.0}},
                {"q": "Find the Gaussian curvature of z = x^2 - y^2 + 2x at (1, 1).", "outputs": {"gaussian": -4 / (1 + 16 + 4) ** 2}},
                {"q": "Find the Gaussian curvature of z = x^2 + y^2.", "outcome": "NEEDS_INPUT"}])


def torus_curvature(major, minor, v):
    R, r, v = _pos(major, "major radius"), _pos(minor, "minor radius"), _fin(v, "v")
    if r >= R:
        raise ValueError("the tube radius must be smaller than the major radius (a ring torus)")
    cv = math.cos(math.radians(v))
    k1, k2 = 1 / r, cv / (R + r * cv)
    return {"gaussian": k1 * k2 + 0.0, "mean": (k1 + k2) / 2, "k1": k1, "k2": k2 + 0.0}


def _ref_torus_curv(major, minor, v):
    import sympy as sp
    u, w = sp.symbols("u w", real=True)
    R, r = _R(major), _R(minor)
    X = sp.Matrix([(R + r * sp.cos(w)) * sp.cos(u), (R + r * sp.cos(w)) * sp.sin(u), r * sp.sin(w)])
    pt = {u: 0, w: sp.pi * _R(v) / 180}
    at = lambda e: e.subs(pt).evalf(40)                      # symbolic derivatives, evaluated at the point to 40 digits
    Xp, Xu, Xw = at(X), at(X.diff(u)), at(X.diff(w))
    Xuu, Xuw, Xww = at(X.diff(u, 2)), at(X.diff(u, w)), at(X.diff(w, 2))
    n = Xu.cross(Xw)
    centre = sp.Matrix([R, 0, 0])
    if n.dot(Xp - centre) < 0:
        n = -n                                               # outward
    n = n / sp.sqrt(n.dot(n))
    EE, FF, GG = Xu.dot(Xu), Xu.dot(Xw), Xw.dot(Xw)
    L, M, N = Xuu.dot(n), Xuw.dot(n), Xww.dot(n)
    S = sp.Matrix([[EE, FF], [FF, GG]]).inv() * sp.Matrix([[L, M], [M, N]])
    import numpy as np
    Sf = np.array(S.evalf(30).tolist(), dtype=float)
    ks = sorted((-np.linalg.eigvals(Sf).real).tolist(), reverse=True)            # convex-positive sign
    return {"gaussian": _F(S.det()), "mean": -_F(S.trace()) / 2, "k1": ks[0], "k2": ks[1]}


def parse_torus_curv(q0):
    q = _norm(q0)
    if not re.search(r"\b(curvature)\b", q) or not re.search(r"\btorus\b", q):
        return None
    return _scoped(q, _clean({"major": _after(q, r"major radius|central radius"), "minor": _after(q, r"minor radius|tube radius"),
                              "v": _angle_in(q, r"angle|v|tube angle")}))


_add("torus_curvature", "expert",
     name="Curvature of a torus at a point", topic="surfaces / curvature",
     purpose="Gaussian, mean and principal curvatures of a ring torus at tube angle v (v = 0 on the outer equator, 180 on the inner).",
     inputs=[_I("major", "major radius R"), _I("minor", "tube radius r"), _I("v", "tube angle v, degrees (0 = outer equator)")],
     outputs=[_O("gaussian", "K = cos v / (r (R + r cos v))"), _O("mean", "H = (R + 2 r cos v) / (2 r (R + r cos v))"),
              _O("k1", "principal curvature around the tube, 1/r"), _O("k2", "principal curvature along the ring, cos v / (R + r cos v)")],
     primary="gaussian", conditions=["0 < r < R", "signs: curvatures positive where the surface bends away from the outward normal (a sphere is positive)"],
     method="principal curvatures of the surface of revolution: 1/r around the tube and cos v/(R + r cos v) along the ring",
     latex=r"K = \frac{\cos v}{r(R + r\cos v)},\quad H = \frac{R + 2r\cos v}{2r(R + r\cos v)}", run=torus_curvature,
     verify=lambda i, o: (lambda R, r, c: _res((o["gaussian"] * r * (R + r * c), c), (o["k1"] * r, 1.0), (o["k2"] * (R + r * c), c),
                                               (2 * o["mean"], o["k1"] + o["k2"]), (o["gaussian"], o["k1"] * o["k2"])))(
         i["major"], i["minor"], math.cos(math.radians(i["v"]))),
     parse=parse_torus_curv, reference=_ref_torus_curv,
     wrong=lambda major, minor, v: (lambda c: {"gaussian": c / (minor * major), "mean": (1 / minor + c / major) / 2, "k1": 1 / minor, "k2": c / major})(math.cos(math.radians(v))),
     random_inputs=lambda rng: (lambda R: {"major": R, "minor": round(rng.uniform(0.1, 0.9) * R, 3), "v": round(rng.uniform(0, 360), 1)})(_rand_len(rng, 1, 30)),
     templates=["Find the Gaussian curvature of a torus with major radius 3 and minor radius 1 at tube angle 0 degrees."],
     examples=[{"inputs": {"major": 3, "minor": 1, "v": 0}, "outputs": {"gaussian": 0.25, "mean": 0.625, "k1": 1.0, "k2": 0.25}},
               {"inputs": {"major": 3, "minor": 1, "v": 180}, "outputs": {"gaussian": -0.5, "k2": -0.5}},
               {"inputs": {"major": 3, "minor": 1, "v": 90}, "outputs": {"gaussian": 0.0}}],
     questions=[{"q": "Find the Gaussian curvature of a torus with major radius 3 and minor radius 1 at tube angle 0 degrees.", "outputs": {"gaussian": 0.25}},
                {"q": "Find the mean curvature of a torus with major radius 3 and minor radius 1 at tube angle 180 degrees.", "outputs": {"mean": 0.25}}])


def parabola_arc_length(a, b, x0, x1, c=0.0):
    a, b, x0, x1, c = _fin(a, "a"), _fin(b, "b"), _fin(x0, "x0"), _fin(x1, "x1"), _fin(c, "c")
    if x0 == x1:
        raise ValueError("the interval is empty (x0 = x1)")
    if a == 0:
        return {"length": math.sqrt(1 + b * b) * abs(x1 - x0)}
    F = lambda u: (u * math.sqrt(1 + u * u) + math.asinh(u)) / 2
    u0, u1 = 2 * a * x0 + b, 2 * a * x1 + b
    return {"length": abs((F(u1) - F(u0)) / (2 * a))}


def _verify_parab(i, o):
    """Gauss-Legendre (10 points) on 64 panels: an independent quadrature of sqrt(1 + y'^2)."""
    a, b, x0, x1 = i["a"], i["b"], i["x0"], i["x1"]
    nodes = [(-0.9739065285171717, 0.0666713443086881), (-0.8650633666889845, 0.1494513491505806), (-0.6794095682990244, 0.2190863625159820),
             (-0.4333953941292472, 0.2692667193099963), (-0.1488743389816312, 0.2955242247147529)]
    nodes += [(-x, w) for x, w in nodes]
    lo, hi = min(x0, x1), max(x0, x1)
    n = 64
    h = (hi - lo) / n
    s = 0.0
    for k in range(n):
        m = lo + (k + 0.5) * h
        s += sum(w * math.sqrt(1 + (2 * a * (m + x * h / 2) + b) ** 2) for x, w in nodes) * h / 2
    return _rel(o["length"], s)


def _ref_parab(a, b, x0, x1, c=0.0):
    from scipy.integrate import quad
    import numpy as np
    return {"length": abs(quad(lambda x: np.sqrt(1 + (2 * a * x + b) ** 2), x0, x1, epsabs=0, epsrel=2e-14, limit=200)[0])}


def parse_parab(q0):
    q = _norm(q0)
    if not re.search(r"\barc length\b|\blength of the (?:curve|parabola|arc)\b", q):
        return None
    pol = _graph_poly(q)
    if pol is None or max(pol) > 2 or any(k < 0 for k in pol):
        return None
    m = re.search(r"\bfrom\s*x\s*=\s*(" + _N + r")\s*to\s*x\s*=\s*(" + _N + r")", q) or re.search(r"\bbetween\s*x\s*=\s*(" + _N + r")\s*and\s*x\s*=\s*(" + _N + r")", q)
    d = {"a": pol.get(2, 0.0), "b": pol.get(1, 0.0), "c": pol.get(0, 0.0)}
    if m:
        d.update(x0=float(m.group(1)), x1=float(m.group(2)))
    return d


_add("parabola_arc_length", "advanced",
     name="Arc length of a parabola", topic="curves / arc length",
     purpose="The length of the curve y = a x^2 + b x + c between x = x0 and x = x1.",
     inputs=[_I("a", "coefficient of x^2"), _I("b", "coefficient of x"), _I("x0", "start x"), _I("x1", "end x"),
             _I("c", "constant (does not affect the length)", required=False)],
     outputs=[_O("length", "arc length")], primary="length", conditions=["x0 != x1 (either order)"],
     method="closed form of the integral of sqrt(1 + u^2), u = 2ax + b: (u sqrt(1+u^2) + asinh u)/2, divided by 2a",
     latex=r"L = \frac{1}{2a}\Big[\tfrac12\big(u\sqrt{1+u^2} + \operatorname{arsinh} u\big)\Big]_{u_0}^{u_1},\quad u = 2ax + b",
     run=parabola_arc_length, verify=_verify_parab, parse=parse_parab, reference=_ref_parab,
     wrong=lambda a, b, x0, x1, c=0.0: {"length": math.hypot(x1 - x0, (a * x1 * x1 + b * x1) - (a * x0 * x0 + b * x0))},
     random_inputs=lambda rng: (lambda x0: {"a": rng.choice([round(rng.uniform(-3, 3), 2), float(rng.randint(1, 4))]) or 1.0, "b": float(rng.randint(-5, 5)),
                                            "x0": x0, "x1": round(x0 + rng.uniform(0.5, 6), 2), "c": float(rng.randint(-5, 5))})(round(rng.uniform(-5, 5), 2)),
     templates=["Find the arc length of y = x^2 from x = 0 to x = 1."],
     examples=[{"inputs": {"a": 1, "b": 0, "x0": 0, "x1": 1}, "outputs": {"length": math.sqrt(5) / 2 + math.asinh(2) / 4}},
               {"inputs": {"a": 0, "b": 1, "x0": 0, "x1": 3}, "outputs": {"length": 3 * math.sqrt(2)}}],
     questions=[{"q": "Find the arc length of y = x^2 from x = 0 to x = 1.", "outputs": {"length": math.sqrt(5) / 2 + math.asinh(2) / 4}},
                {"q": "Find the arc length of y = x^2 + 3 from x = 2 to x = 2.", "outcome": "INVALID_INPUT"}])


# ==================================================================================================================
# computational geometry (advanced / expert)
# ==================================================================================================================
def _circ3(a, b, c):
    bx, by, cx, cy = b[0] - a[0], b[1] - a[1], c[0] - a[0], c[1] - a[1]
    d = 2 * (bx * cy - by * cx)
    if d == 0:
        return None
    ux = (cy * (bx * bx + by * by) - by * (cx * cx + cy * cy)) / d
    uy = (bx * (cx * cx + cy * cy) - cx * (bx * bx + by * by)) / d
    return (a[0] + ux, a[1] + uy, math.hypot(ux, uy))


def _in_circ(c, p):
    return math.hypot(p[0] - c[0], p[1] - c[1]) <= c[2] * (1 + 1e-12) + 1e-12


def min_enclosing_circle(points):
    import random as _random
    pts = list(dict.fromkeys(_pairs(points, "points", 1, 2000)))
    _random.Random(20261022).shuffle(pts)                    # a fixed order: deterministic, expected linear time
    c = None
    for i, p in enumerate(pts):
        if c is not None and _in_circ(c, p):
            continue
        c = (p[0], p[1], 0.0)
        for j in range(i):
            q = pts[j]
            if _in_circ(c, q):
                continue
            c = ((p[0] + q[0]) / 2, (p[1] + q[1]) / 2, math.dist(p, q) / 2)
            for k in range(j):
                r = pts[k]
                if _in_circ(c, r):
                    continue
                cc = _circ3(p, q, r)
                if cc is None:                               # collinear: the farthest pair spans it
                    a, b = max(((p, q), (p, r), (q, r)), key=lambda e: math.dist(*e))
                    cc = ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2, math.dist(a, b) / 2)
                c = cc
    return {"cx": c[0] + 0.0, "cy": c[1] + 0.0, "radius": c[2]}


def _verify_mec(i, o):
    """Every point inside; the boundary points are not all in one open half-circle (otherwise a smaller circle exists)."""
    pts = _pairs(i["points"], "points", 1, 2000)
    C, r = (o["cx"], o["cy"]), o["radius"]
    if r == 0:
        return len(set(pts)) == 1
    if any(math.dist(C, p) > r * (1 + 1e-9) for p in pts):
        return False
    on = [math.atan2(p[1] - C[1], p[0] - C[0]) for p in pts if abs(math.dist(C, p) - r) <= 1e-9 * r]
    if len(on) < 2:
        return False
    on.sort()
    gaps = [on[k + 1] - on[k] for k in range(len(on) - 1)] + [2 * math.pi - on[-1] + on[0]]
    return max(gaps) <= math.pi * (1 + 1e-9)


def _ref_mec(points):
    """Brute force: every circle on two points as diameter or through three points; the smallest containing all."""
    import numpy as np
    P = np.array(_pairs(points, "points", 1), float)
    best = None
    n = len(P)
    cands = []
    for a in range(n):
        for b in range(a + 1, n):
            c = (P[a] + P[b]) / 2
            cands.append((c, np.linalg.norm(P[a] - c)))
            for d in range(b + 1, n):
                cc = _circ3(tuple(P[a]), tuple(P[b]), tuple(P[d]))
                if cc:
                    cands.append((np.array(cc[:2]), cc[2]))
    for c, r in cands:
        if np.all(np.linalg.norm(P - c, axis=1) <= r * (1 + 1e-10)) and (best is None or r < best[1]):
            best = (c, r)
    return {"cx": float(best[0][0]), "cy": float(best[0][1]), "radius": float(best[1])}


def parse_mec(q0):
    q = _norm(q0)
    if not re.search(r"\b(smallest enclosing circle|minimum enclosing circle|minimal enclosing circle|smallest circle (?:containing|enclosing|covering|that contains|that encloses))\b", q):
        return None
    pts = _pts_or_list(q)
    return {"points": pts} if pts else {}


_add("min_enclosing_circle", "expert",
     name="Smallest enclosing circle of points", topic="computational geometry / enclosing shapes",
     purpose="Centre and radius of the smallest circle that contains every given point.",
     inputs=[_V("points", "the points: x1 y1, x2 y2, ...", n_min=2)],
     outputs=[_O("cx", "x of the centre"), _O("cy", "y of the centre"), _O("radius", "radius")], primary="radius",
     conditions=["1 to 2000 points", "the smallest enclosing circle is unique"],
     method="Welzl's incremental algorithm (fixed shuffled order), circles through 2 or 3 support points",
     latex=r"\min_{c, r} r \;\text{ s.t. } \|p_i - c\| \le r\ \forall i", run=min_enclosing_circle, verify=_verify_mec, parse=parse_mec, reference=_ref_mec,
     wrong=lambda points: (lambda p: (lambda g: {"cx": g[0], "cy": g[1], "radius": max(math.dist(g, x) for x in p)})(
         (sum(x for x, _ in p) / len(p), sum(y for _, y in p) / len(p))))(_pairs(points, "points", 1)),
     random_inputs=lambda rng: {"points": _rand_points(rng, 3, 25)},
     templates=["Find the smallest enclosing circle of the points (0, 0), (4, 0), (2, 1), (1, 3)."],
     examples=[{"inputs": {"points": "0 0, 4 0, 2 1"}, "outputs": {"cx": 2.0, "cy": 0.0, "radius": 2.0}},
               {"inputs": {"points": "0 0, 2 0, 1 2"}, "outputs": {"cx": 1.0, "cy": 0.75, "radius": 1.25}}],
     questions=[{"q": "Find the smallest enclosing circle of the points (0, 0), (4, 0), (2, 1).", "outputs": {"cx": 2.0, "cy": 0.0, "radius": 2.0}},
                {"q": "What is the smallest circle containing the points (0, 0), (2, 0) and (1, 2)?", "outputs": {"radius": 1.25}}])


def min_area_rectangle(points):
    pts = _pairs(points, "points", 3, 5000)
    h = _hull(pts)
    if len(h) < 3:
        raise ValueError("the points are collinear: the enclosing rectangle has no area")
    best = None
    n = len(h)
    for k in range(n):
        ex, ey = h[(k + 1) % n][0] - h[k][0], h[(k + 1) % n][1] - h[k][1]
        L = math.hypot(ex, ey)
        ux, uy = ex / L, ey / L
        us = [x * ux + y * uy for x, y in h]
        vs = [-x * uy + y * ux for x, y in h]
        w, t = max(us) - min(us), max(vs) - min(vs)
        if best is None or w * t < best[0]:
            best = (w * t, min(w, t), max(w, t))
    return {"area": best[0], "width": best[1], "length": best[2]}


def _verify_mar(i, o):
    """Every hull-edge orientation again (from the other endpoint), the minimum compared; sides multiply to the area."""
    h = _hull(_pairs(i["points"], "points", 3, 5000))
    n = len(h)
    best = math.inf
    for k in range(n):
        ex, ey = h[k - 1][0] - h[k][0], h[k - 1][1] - h[k][1]
        L = math.hypot(ex, ey)
        us = [((x - h[k][0]) * ex + (y - h[k][1]) * ey) / L for x, y in h]
        vs = [((y - h[k][1]) * ex - (x - h[k][0]) * ey) / L for x, y in h]
        best = min(best, (max(us) - min(us)) * (max(vs) - min(vs)))
    return _res((o["area"], best), (o["area"], o["width"] * o["length"]))


def parse_mar(q0):
    q = _norm(q0)
    if not re.search(r"\b(minimum[- ]area (?:bounding )?rectangle|smallest (?:area )?(?:bounding |enclosing )?rectangle|minimum bounding rectangle|"
                     r"oriented bounding box|minimum rotated rectangle)\b", q):
        return None
    pts = _pts_or_list(q)
    return {"points": pts} if pts else {}


_add("min_area_rectangle", "advanced",
     name="Minimum-area enclosing rectangle", topic="computational geometry / enclosing shapes",
     purpose="The area (and sides) of the smallest-area rectangle, in any orientation, enclosing the points.",
     inputs=[_V("points", "the points: x1 y1, x2 y2, ...", n_min=6)],
     outputs=[_O("area", "minimum area"), _O("width", "shorter side of that rectangle"), _O("length", "longer side")], primary="area",
     conditions=["at least 3 points, not all collinear", "when two orientations tie in area, the sides of the first found are reported"],
     method="convex hull, then one side of the optimal rectangle lies along a hull edge: try every edge (rotating calipers)",
     latex=r"A^* = \min_{e \in \text{hull edges}} w_e\, h_e", run=min_area_rectangle, verify=_verify_mar, parse=parse_mar,
     reference=lambda points: {"area": float(__import__("shapely").minimum_rotated_rectangle(__import__("shapely.geometry", fromlist=["MultiPoint"]).MultiPoint(
         _pairs(points, "points", 3))).area)},
     wrong=lambda points: (lambda p: (lambda w, t: {"area": w * t, "width": min(w, t), "length": max(w, t)})(
         max(x for x, _ in p) - min(x for x, _ in p), max(y for _, y in p) - min(y for _, y in p)))(_pairs(points, "points", 3)),
     random_inputs=lambda rng: {"points": _rand_points(rng, 4, 40)},
     templates=["Find the minimum area bounding rectangle of the points (0, 0), (2, 2), (4, 0), (2, -2)."],
     examples=[{"inputs": {"points": "0 0, 2 2, 4 0, 2 -2"}, "outputs": {"area": 8.0, "width": math.sqrt(8), "length": math.sqrt(8)}},
               {"inputs": {"points": "0 0, 3 0, 3 1, 0 1, 1 0.5"}, "outputs": {"area": 3.0, "width": 1.0, "length": 3.0}}],
     questions=[{"q": "Find the minimum area bounding rectangle of the points (0, 0), (2, 2), (4, 0), (2, -2).", "outputs": {"area": 8.0}},
                {"q": "Find the minimum area bounding rectangle of the points (0, 0), (1, 1), (2, 2).", "outcome": "INVALID_INPUT"}])


def delaunay_counts(points):
    pts = _pairs(points, "points", 3, 25)
    if len(set(pts)) != len(pts):
        raise ValueError("a point is repeated")
    X = [(_fr(x), _fr(y)) for x, y in pts]
    for a, b, c in itertools.combinations(X, 3):
        if (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]) == 0:
            raise ValueError("three points are collinear: not in general position")
    for a, b, c, d in itertools.combinations(X, 4):
        rows = [(p[0] - d[0], p[1] - d[1], (p[0] - d[0]) ** 2 + (p[1] - d[1]) ** 2) for p in (a, b, c)]
        det = (rows[0][0] * (rows[1][1] * rows[2][2] - rows[1][2] * rows[2][1]) - rows[0][1] * (rows[1][0] * rows[2][2] - rows[1][2] * rows[2][0])
               + rows[0][2] * (rows[1][0] * rows[2][1] - rows[1][1] * rows[2][0]))
        if det == 0:
            raise ValueError("four points are concyclic: the Delaunay triangulation is not unique")
    n, h = len(pts), len(_hull(pts))
    return {"triangles": 2 * n - 2 - h, "edges": 3 * n - 3 - h, "hull_vertices": h, "voronoi_vertices": 2 * n - 2 - h, "unbounded_cells": h}


def _verify_delaunay(i, o):
    n = len(i["points"]) // 2
    h = o["hull_vertices"]
    # Euler's formula for the planar graph of the triangulation (outer face included): V - E + F = 2
    return (n - o["edges"] + (o["triangles"] + 1) == 2 and 3 * o["triangles"] + h == 2 * o["edges"]
            and o["voronoi_vertices"] == o["triangles"] and o["unbounded_cells"] == h
            and h == len(_hull(_pairs(i["points"], "points", 3, 25))))


def _ref_delaunay(points):
    import numpy as np
    from scipy.spatial import ConvexHull, Delaunay, Voronoi
    P = np.array(_pairs(points, "points", 3), float)
    D, Vo = Delaunay(P), Voronoi(P)
    edges = {tuple(sorted((int(s[a]), int(s[b])))) for s in D.simplices for a, b in ((0, 1), (1, 2), (0, 2))}
    unb = sum(1 for k in Vo.point_region if -1 in Vo.regions[k] or not Vo.regions[k])
    return {"triangles": len(D.simplices), "edges": len(edges), "hull_vertices": len(ConvexHull(P).vertices),
            "voronoi_vertices": len(Vo.vertices), "unbounded_cells": unb}


def parse_delaunay(q0):
    q = _norm(q0)
    if not re.search(r"\b(delaunay|voronoi)\b", q):
        return None
    pts = _pts_or_list(q)
    return {"points": pts} if pts else {}


_add("delaunay_voronoi_counts", "expert",
     name="Delaunay triangulation and Voronoi diagram: counts", topic="computational geometry / triangulations",
     purpose="Numbers of Delaunay triangles and edges, hull vertices, Voronoi vertices and unbounded Voronoi cells of points in general position.",
     inputs=[_V("points", "the points: x1 y1, x2 y2, ... (3 to 25)", n_min=6)],
     outputs=[_O("triangles", "Delaunay triangles 2n - 2 - h"), _O("edges", "Delaunay edges 3n - 3 - h"), _O("hull_vertices", "h, vertices of the convex hull"),
              _O("voronoi_vertices", "Voronoi vertices (one per triangle)"), _O("unbounded_cells", "unbounded Voronoi cells (one per hull vertex)")],
     primary="triangles", conditions=["3 to 25 distinct points", "general position: no three collinear, no four concyclic (checked exactly)"],
     method="exact general-position checks, the convex hull, then Euler's formula for a triangulation; Voronoi is the dual",
     latex=r"T = 2n - 2 - h,\quad E = 3n - 3 - h", run=delaunay_counts, verify=_verify_delaunay, parse=parse_delaunay, reference=_ref_delaunay,
     wrong=lambda points: (lambda r: dict(r, triangles=r["triangles"] + 2, voronoi_vertices=r["voronoi_vertices"] + 2))(delaunay_counts(points)),
     random_inputs=lambda rng: {"points": _rand_points(rng, 3, 12)},
     templates=["How many triangles are in the Delaunay triangulation of the points (0, 0), (4, 0), (0, 3), (5, 4), (2, 1)?"],
     examples=[{"inputs": {"points": "0 0, 4 0, 0 3, 5 4, 2 1"}, "outputs": {"triangles": 4, "edges": 8, "hull_vertices": 4, "voronoi_vertices": 4, "unbounded_cells": 4}},
               {"inputs": {"points": "0 0, 1 0, 0 1"}, "outputs": {"triangles": 1, "edges": 3}}],
     questions=[{"q": "How many triangles are in the Delaunay triangulation of the points (0, 0), (4, 0), (0, 3), (5, 4), (2, 1)?", "outputs": {"triangles": 4}},
                {"q": "Count the Voronoi vertices for the points (0, 0), (1, 0), (1, 1), (0, 1).", "outcome": "INVALID_INPUT"}])


def _two_groups(q):
    m = re.search(r"\)\s*,?\s*(?:and|;)\s+(?:the\s+)?[a-z][a-z ]*\(", q)
    if not m:
        return None
    return _pts2(q[:m.start() + 1]), _pts2(q[m.end() - 1:])


def _ccw_convex(pts, name):
    _simple_polygon(pts, name)
    if polygon_is_convex(_flat(pts))["convex"] != "yes":
        raise ValueError(f"{name}: the polygon is not convex")
    return pts if _shoelace(pts) > 0 else pts[::-1]


def _clip(subject, clipper):
    """Sutherland-Hodgman: the part of `subject` inside the convex counterclockwise polygon `clipper`."""
    out = subject
    n = len(clipper)
    for k in range(n):
        a, b = clipper[k], clipper[(k + 1) % n]
        inp, out = out, []
        if not inp:
            break
        for j in range(len(inp)):
            p, q = inp[j - 1], inp[j]
            ip, iq = _cross(a, b, p) >= 0, _cross(a, b, q) >= 0
            if iq:
                if not ip:
                    out.append(_seg_line(p, q, a, b))
                out.append(q)
            elif ip:
                out.append(_seg_line(p, q, a, b))
    return out


def _seg_line(p, q, a, b):
    d1, d2 = _cross(a, b, p), _cross(a, b, q)
    t = d1 / (d1 - d2)
    return (p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1]))


def convex_intersection(p, q):
    P = _ccw_convex(_pairs(p, "p", 3), "p")
    Q = _ccw_convex(_pairs(q, "q", 3), "q")
    R = _clip(Q, P)
    return {"area": abs(_shoelace(R)) if len(R) >= 3 else 0.0}


def _verify_cint(i, o):
    P = _ccw_convex(_pairs(i["p"], "p", 3), "p")
    Q = _ccw_convex(_pairs(i["q"], "q", 3), "q")
    R = _clip(P, Q)                                          # clip the other way round
    a = abs(_shoelace(R)) if len(R) >= 3 else 0.0
    sc = max(abs(_shoelace(P)), abs(_shoelace(Q)))
    return abs(o["area"] - a) / max(a, 1e-12 * sc, 1e-300) if a or o["area"] else 0.0


def _ref_cint(p, q):
    from shapely.geometry import Polygon
    return {"area": Polygon(_pairs(p, "p", 3)).intersection(Polygon(_pairs(q, "q", 3))).area}


def _rand_convex_poly(rng, cx, cy, s):
    while True:
        h = _hull([(round(cx + rng.uniform(-s, s), 2), round(cy + rng.uniform(-s, s), 2)) for _ in range(rng.randint(3, 9))])
        if len(h) >= 3 and abs(_shoelace(h)) > 0.05 * s * s:
            return _flat(h if rng.random() < 0.5 else h[::-1])


def parse_cint(q0):
    q = _norm(q0)
    if not re.search(r"\b(intersection|overlap|common area|intersect)\b", q) or not re.search(_POLY_WORDS + r"|\btriangles?\b|\bpolygons\b", q):
        return None
    if re.search(r"\b(line|segment|circle)\b", q):
        return None
    g = _two_groups(q)
    if not g or len(g[0]) < 3 or len(g[1]) < 3:
        return None
    return {"p": _flat(g[0]), "q": _flat(g[1])}


_add("convex_polygon_intersection", "advanced",
     name="Area of the intersection of two convex polygons", topic="computational geometry / polygon clipping",
     purpose="The area common to two convex polygons given by their vertices.",
     inputs=[_V("p", "first convex polygon's vertices in order", n_min=6), _V("q", "second convex polygon's vertices in order", n_min=6)],
     outputs=[_O("area", "area of the intersection (0 if they do not overlap)")], primary="area", conditions=["both polygons simple and convex"],
     method="Sutherland-Hodgman clipping of one polygon by the other's edges, then the shoelace area",
     latex=r"A(P \cap Q)", run=convex_intersection, verify=_verify_cint, parse=parse_cint, reference=_ref_cint,
     wrong=lambda p, q: {"area": min(abs(_shoelace(_pairs(p, "p", 3))), abs(_shoelace(_pairs(q, "q", 3))))},
     random_inputs=lambda rng: {"p": _rand_convex_poly(rng, 0, 0, 10), "q": _rand_convex_poly(rng, rng.uniform(-8, 8), rng.uniform(-8, 8), 10)},
     templates=["Find the area of intersection of the polygon with vertices (0, 0), (2, 0), (2, 2), (0, 2) and the polygon with vertices (1, 1), (3, 1), (3, 3), (1, 3)."],
     examples=[{"inputs": {"p": "0 0, 2 0, 2 2, 0 2", "q": "1 1, 3 1, 3 3, 1 3"}, "outputs": {"area": 1.0}},
               {"inputs": {"p": "0 0, 1 0, 0 1", "q": "5 5, 6 5, 5 6"}, "outputs": {"area": 0.0}}],
     questions=[{"q": "Find the area of intersection of the polygon with vertices (0, 0), (2, 0), (2, 2), (0, 2) and the polygon with vertices (1, 1), (3, 1), (3, 3), (1, 3).",
                 "outputs": {"area": 1.0}},
                {"q": "Find the overlap of the polygon with vertices (0, 0), (4, 0), (4, 4), (2, 1), (0, 4) and the polygon with vertices (1, 1), (3, 1), (3, 3).", "outcome": "INVALID_INPUT"}])


def hausdorff_distance(a, b):
    A, B = _pairs(a, "a", 1, 1000), _pairs(b, "b", 1, 1000)
    d = lambda S, T: max(min(math.dist(s, t) for t in T) for s in S)
    return {"distance": max(d(A, B), d(B, A)), "from_a": d(A, B), "from_b": d(B, A)}


def _ref_haus(a, b):
    import numpy as np
    from scipy.spatial.distance import directed_hausdorff
    A, B = np.array(_pairs(a, "a", 1), float), np.array(_pairs(b, "b", 1), float)
    x, y = directed_hausdorff(A, B)[0], directed_hausdorff(B, A)[0]
    return {"distance": float(max(x, y)), "from_a": float(x), "from_b": float(y)}


def _verify_haus(i, o):
    A, B = _pairs(i["a"], "a", 1, 1000), _pairs(i["b"], "b", 1, 1000)
    dA = math.sqrt(max(min((s[0] - t[0]) ** 2 + (s[1] - t[1]) ** 2 for t in B) for s in A))
    dB = math.sqrt(max(min((s[0] - t[0]) ** 2 + (s[1] - t[1]) ** 2 for t in A) for s in B))
    return _res((o["from_a"], dA), (o["from_b"], dB), (o["distance"], max(dA, dB)))


def parse_haus(q0):
    q = _norm(q0)
    if not re.search(r"\bhausdorff\b", q):
        return None
    g = _two_groups(q)
    if not g or not g[0] or not g[1]:
        return {}
    return {"a": _flat(g[0]), "b": _flat(g[1])}


_add("hausdorff_distance", "advanced",
     name="Hausdorff distance between two point sets", topic="computational geometry / shape comparison",
     purpose="The Hausdorff distance between two finite point sets and both directed distances.",
     inputs=[_V("a", "first set: x1 y1, x2 y2, ...", n_min=2), _V("b", "second set", n_min=2)],
     outputs=[_O("distance", "max of the two directed distances"), _O("from_a", "max over A of the distance to the nearest point of B"),
              _O("from_b", "max over B of the distance to the nearest point of A")], primary="distance", conditions=["1 to 1000 points per set"],
     method="directed distances by exhaustive nearest-point search", latex=r"d_H(A,B) = \max\{\sup_{a}\inf_{b}\|a-b\|,\ \sup_{b}\inf_{a}\|a-b\|\}",
     run=hausdorff_distance, verify=_verify_haus, parse=parse_haus, reference=_ref_haus,
     wrong=lambda a, b: (lambda r: dict(r, distance=min(r["from_a"], r["from_b"])))(hausdorff_distance(a, b)),
     random_inputs=lambda rng: {"a": _rand_points(rng, 2, 30), "b": _rand_points(rng, 2, 30)},
     templates=["Find the Hausdorff distance between the set (0, 0), (1, 0) and the set (0, 0), (3, 4)."],
     examples=[{"inputs": {"a": "0 0, 1 0", "b": "0 0, 3 4"}, "outputs": {"distance": math.sqrt(20), "from_a": 1.0, "from_b": math.sqrt(20)}}],
     questions=[{"q": "Find the Hausdorff distance between the set (0, 0), (1, 0) and the set (0, 0), (3, 4).", "outputs": {"distance": math.sqrt(20)}}])


def nearest_neighbor(px, py, points):
    P = (_fin(px, "px"), _fin(py, "py"))
    pts = _pairs(points, "points", 1, 100000)
    best = min(range(len(pts)), key=lambda k: ((pts[k][0] - P[0]) ** 2 + (pts[k][1] - P[1]) ** 2, k))
    return {"index": best + 1, "distance": math.dist(P, pts[best]), "nx": pts[best][0], "ny": pts[best][1]}


def _verify_nn(i, o):
    P = (i["px"], i["py"])
    pts = _pairs(i["points"], "points", 1, 100000)
    k = o["index"] - 1
    if not 0 <= k < len(pts) or (o["nx"], o["ny"]) != pts[k]:
        return 1.0
    d2 = [(p[0] - P[0]) ** 2 + (p[1] - P[1]) ** 2 for p in pts]
    if d2[k] != min(d2) or d2.index(min(d2)) != k:
        return 1.0
    return _rel(o["distance"], math.sqrt(d2[k]))


def _ref_nn(px, py, points):
    from scipy.spatial import cKDTree
    pts = _pairs(points, "points", 1)
    d, k = cKDTree(pts).query([px, py])
    return {"index": int(k) + 1, "distance": float(d)}


def parse_nn(q0):
    q = _norm(q0)
    if not re.search(r"\b(nearest|closest)\b", q) or re.search(r"\b(pair|segment|line|circle|plane|polygon|two)\b", q):
        return None
    qp, rest = _query_and_rest(q)
    if qp is None or len(rest) < 1:
        return None
    return {"px": qp[0], "py": qp[1], "points": _flat(rest)}


_add("nearest_neighbor", "beginner",
     name="Nearest neighbour of a query point", topic="computational geometry / proximity",
     purpose="Which of a list of points is nearest to a query point, and how far it is.",
     inputs=[_I("px", "x of the query point"), _I("py", "y of the query point"), _V("points", "the candidate points: x1 y1, x2 y2, ...", n_min=2)],
     outputs=[_O("index", "position of the nearest point in the list (1 = first)"), _O("distance", "its distance"), _O("nx", "its x"), _O("ny", "its y")],
     primary="distance", conditions=["ties go to the earliest point in the list"], method="exhaustive search on squared distances",
     latex=r"k^* = \arg\min_k \|p_k - q\|", run=nearest_neighbor, verify=_verify_nn, parse=parse_nn, reference=_ref_nn,
     wrong=lambda px, py, points: dict(nearest_neighbor(px, py, points), index=nearest_neighbor(px, py, points)["index"] - 1),
     random_inputs=lambda rng: {"px": _RANDP(rng), "py": _RANDP(rng), "points": _rand_points(rng, 2, 60)},
     templates=["Which of the points (0, 0), (5, 5), (2, 3) is nearest to the point (3, 3)?"],
     examples=[{"inputs": {"px": 3, "py": 3, "points": "0 0, 5 5, 2 3"}, "outputs": {"index": 3, "distance": 1.0}}],
     questions=[{"q": "Which of the points (0, 0), (5, 5), (2, 3) is nearest to the point (3, 3)?", "outputs": {"index": 3, "distance": 1.0}}])


def barycentric(px, py, points):
    A, B, C = _tri_pts(points)
    P = (_fin(px, "px"), _fin(py, "py"))
    T = _cross(A, B, C)
    return {"l1": _cross(P, B, C) / T + 0.0, "l2": _cross(A, P, C) / T + 0.0, "l3": _cross(A, B, P) / T + 0.0}


def _verify_bary(i, o):
    A, B, C = _pairs(i["points"], "points", 3, 3)
    sc = max(1.0, _scale_of([A, B, C]), abs(i["px"]), abs(i["py"]))
    x = o["l1"] * A[0] + o["l2"] * B[0] + o["l3"] * C[0]
    y = o["l1"] * A[1] + o["l2"] * B[1] + o["l3"] * C[1]
    return max(abs(o["l1"] + o["l2"] + o["l3"] - 1) / max(1.0, abs(o["l1"]) + abs(o["l2"]) + abs(o["l3"])),
               abs(x - i["px"]) / sc, abs(y - i["py"]) / sc)


def _ref_bary(px, py, points):
    import numpy as np
    (ax, ay), (bx, by), (cx, cy) = _pairs(points, "points", 3, 3)
    lam = np.linalg.solve(np.array([[ax, bx, cx], [ay, by, cy], [1.0, 1.0, 1.0]]), np.array([px, py, 1.0]))
    return {"l1": float(lam[0]), "l2": float(lam[1]), "l3": float(lam[2])}


def parse_bary(q0):
    q = _norm(q0)
    if not re.search(r"\bbarycentric\b", q):
        return None
    qp, rest = _query_and_rest(q)
    if qp is None:
        return {}
    return {"px": qp[0], "py": qp[1], "points": _flat(rest)} if len(rest) == 3 else None


_add("barycentric_coordinates", "advanced",
     name="Barycentric coordinates of a point in a triangle", topic="coordinate geometry / triangles",
     purpose="The barycentric coordinates (l1, l2, l3) of a point with respect to triangle ABC: P = l1 A + l2 B + l3 C, l1 + l2 + l3 = 1.",
     inputs=[_I("px", "x of the point"), _I("py", "y of the point"), _V("points", "the triangle's vertices A, B, C", n_min=6, n_max=6)],
     outputs=[_O("l1", "weight of A"), _O("l2", "weight of B"), _O("l3", "weight of C")], primary="l1",
     conditions=["A, B, C not collinear", "all weights positive exactly when P is inside"],
     method="ratios of signed areas: l1 = [PBC]/[ABC], l2 = [APC]/[ABC], l3 = [ABP]/[ABC]", latex=r"\lambda_1 = \frac{[PBC]}{[ABC]},\ \lambda_2 = \frac{[APC]}{[ABC]},\ \lambda_3 = \frac{[ABP]}{[ABC]}",
     run=barycentric, verify=_verify_bary, parse=parse_bary, reference=_ref_bary,
     wrong=lambda px, py, points: (lambda r: {"l1": r["l2"], "l2": r["l1"], "l3": r["l3"]})(barycentric(px, py, points)),
     random_inputs=lambda rng: {"px": _RANDP(rng), "py": _RANDP(rng), **_rand_tri(rng)},
     templates=["Find the barycentric coordinates of the point (1, 1) in the triangle with vertices (0, 0), (4, 0), (0, 4)."],
     examples=[{"inputs": {"px": 1, "py": 1, "points": "0 0, 4 0, 0 4"}, "outputs": {"l1": 0.5, "l2": 0.25, "l3": 0.25}}],
     questions=[{"q": "Find the barycentric coordinates of the point (1, 1) in the triangle with vertices (0, 0), (4, 0), (0, 4).", "outputs": {"l1": 0.5, "l2": 0.25, "l3": 0.25}}])


def polygon_angles(points):
    pts = _pairs(points, "points", 3)
    _simple_polygon(pts)
    n = len(pts)
    sgn = 1 if _shoelace(pts) > 0 else -1
    ang = []
    for k in range(n):
        a, b, c = pts[k - 1], pts[k], pts[(k + 1) % n]
        u, v = (b[0] - a[0], b[1] - a[1]), (c[0] - b[0], c[1] - b[1])
        turn = math.degrees(math.atan2(sgn * (u[0] * v[1] - u[1] * v[0]), u[0] * v[0] + u[1] * v[1]))
        ang.append(_r12(180.0 - turn))
    return {"angles": ang, "angle_sum": float((n - 2) * 180), "reflex_vertices": sum(1 for x in ang if x > 180)}


def _verify_pangles(i, o):
    pts = _pairs(i["points"], "points", 3)
    n = len(pts)
    if len(o["angles"]) != n or o["reflex_vertices"] != sum(1 for x in o["angles"] if x > 180):
        return 1.0
    worst = _rel(sum(o["angles"]), (n - 2) * 180.0) + _rel(o["angle_sum"], (n - 2) * 180.0)
    sgn = 1 if _shoelace(pts) > 0 else -1
    for k in range(n):                                        # the angle between the two edges at the vertex (0..180) by acos,
        a, b, c = pts[k - 1], pts[k], pts[(k + 1) % n]         # mirrored to 360 - x at reflex (clockwise-turning) vertices
        u, v = (a[0] - b[0], a[1] - b[1]), (c[0] - b[0], c[1] - b[1])
        x = math.degrees(math.acos(max(-1.0, min(1.0, (u[0] * v[0] + u[1] * v[1]) / (math.hypot(*u) * math.hypot(*v))))))
        if sgn * _cross(a, b, c) < 0:
            x = 360 - x
        worst = max(worst, abs(o["angles"][k] - x) / 360 if abs(o["angles"][k] - x) > 1e-7 else 0.0)
    return worst


def _ref_pangles(points):
    import sympy as sp
    P = [sp.Point(_R(x), _R(y)) for x, y in _pairs(points, "points", 3)]
    poly = sp.Polygon(*P)
    ang = poly.angles
    vals = [_F(sp.deg(ang[p])) for p in P]
    return {"angles": vals, "angle_sum": _F(sum(sp.deg(ang[p]) for p in P))}


def parse_pangles(q0):
    q = _norm(q0)
    if not re.search(r"\b(interior angles|angles of the|angles at the vertices|angle at each vertex)\b", q) or not re.search(_POLY_WORDS, q) or re.search(r"\btriangle\b", q):
        return None
    pts = _point_list(q)
    return {"points": pts} if pts and len(pts) >= 6 else None


_add("polygon_interior_angles", "intermediate",
     name="Interior angles of a polygon from its vertices", topic="polygons / coordinates",
     purpose="The interior angle at every vertex of a simple polygon (reflex angles included), their sum and the number of reflex vertices.",
     inputs=[_V("points", "vertices in order: x1 y1, x2 y2, ...", n_min=6)],
     outputs=[_O("angles", "interior angle at each vertex, degrees, in the order given", "text"), _O("angle_sum", "(n - 2) 180"),
              _O("reflex_vertices", "number of angles above 180 degrees")], primary="angle_sum", conditions=["simple polygon", "either orientation"],
     method="turning angle at each vertex by atan2 of the cross and dot products of consecutive edges, interior = 180 - turn (orientation-corrected)",
     latex=r"\theta_i = 180^\circ - \operatorname{atan2}(\mathbf u\times\mathbf v, \mathbf u\cdot\mathbf v),\quad \sum\theta_i = (n-2)180^\circ",
     run=polygon_angles, verify=_verify_pangles, parse=parse_pangles, reference=_ref_pangles,
     wrong=lambda points: (lambda r: {"angles": [180 - x for x in r["angles"]], "angle_sum": 360.0, "reflex_vertices": 0})(polygon_angles(points)),
     random_inputs=lambda rng: {"points": _flat(_star_polygon(rng, 4, 10))},
     templates=["Find the interior angles of the polygon with vertices (0, 0), (4, 0), (4, 4), (2, 1), (0, 4)."],
     examples=[{"inputs": {"points": "0 0, 4 0, 4 4, 0 4"}, "outputs": {"angles": [90.0, 90.0, 90.0, 90.0], "angle_sum": 360.0, "reflex_vertices": 0}},
               {"inputs": {"points": "0 0, 2 0, 2 2, 1 1, 0 2"}, "outputs": {"angles": [90.0, 90.0, 45.0, 270.0, 45.0], "reflex_vertices": 1}}],
     questions=[{"q": "Find the interior angles of the polygon with vertices (0, 0), (2, 0), (2, 2), (1, 1), (0, 2).", "outputs": {"angles": [90.0, 90.0, 45.0, 270.0, 45.0]}}])


# ==================================================================================================================
# the reading guard: every parser above is wrapped by the same rules (ask_sweep 2026-10-23: 12 wrong answers, among
# them a spheroid read as a sphere, a frustum as a cone, a trapezoid as a square ("square millimeters"), an annulus as
# a circle, a spherical polygon as a sphere and a point-to-segment distance as a path length)
#   1. an area unit is not a shape: "square millimeters", "sq ft", "cubic meters" are read as units (mm^2, ft^2, m^3)
#      before any parser looks for a square or a cube;
#   2. every stated number is bound: a reading that leaves a number of the question unused is refused. Numbers that
#      are part of a name or a notation (x1, P2, 3D, an exponent ^2, "1st") are not stated values; a number a parser
#      converts through a stated relation is bound through it (a diameter is twice the radius, radians are converted
#      to degrees, an equation the capability reads in full is bound as a whole);
#   3. wording that names a different capability's task is refused (`_FOREIGN`): a sphere parser does not read a
#      spheroid, an equatorial radius or a spherical polygon; a cone parser does not read a frustum or two end radii; a
#      rectangle parser does not read parallel sides; a circle parser does not read an inner and an outer radius.
# ==================================================================================================================
_G_LEN_UNITS = r"(?:millimet(?:er|re)s?|centimet(?:er|re)s?|decimet(?:er|re)s?|kilomet(?:er|re)s?|met(?:er|re)s?|" \
               r"mm|cm|dm|km|m|inch(?:es)?|in|f(?:ee|oo)t|ft|yards?|yd|miles?|mi|units?)\b"
_G_AREA_UNIT = re.compile(r"\b(?:square|sq\.?)\s+(" + _G_LEN_UNITS + r")")
_G_VOL_UNIT = re.compile(r"\b(?:cubic|cu\.?)\s+(" + _G_LEN_UNITS + r")")
_G_NUM = re.compile(r"(\d+)\s*/\s*(\d+)(?![\d.])|(?:\d+(?:\.\d+)?|\.\d+)(?:e[-+]?\d+)?")
# notation that is not a stated value: an exponent, an ordinal, a dimension count, a name with an index (x1, p2, r_1)
_G_NOTATION = re.compile(r"\^\s*\(?\s*-?\s*\d+(?:\.\d+)?|(?<![\w.])\d+\s*-?\s*(?:st|nd|rd|th|d|dimensional)\b|"
                         r"(?<=[a-z_])\d+")
# a whole equation in x, y (and z) that a capability reads term by term (every coefficient is then bound)
_G_EQUATION = re.compile(r"(?<![A-Za-z])[-+0-9.xyz^*/() ]+=[-+0-9.xyz^*/() ]+")
_G_EQ_KEYS = {"line_intersection", "angle_between_lines", "reflect_point_line", "circle_line_intersection",
              "circle_circle_intersection", "tangent_from_point", "conic_classify", "triangle_orthocenter",
              "dilate_point", "rotate_point", "point_plane_distance", "line_plane_intersection", "angle_between_planes",
              "curvature_of_graph", "surface_curvature", "parabola_arc_length", "ellipse_perimeter", "ellipse_curvature"}

# rule 3: wording that asks for another capability's task, per capability
_FOREIGN = {
    "rectangle_measures": r"\btrapez|\bparallel\b|\brhomb|\bparallelogram|\bkite\b|\bannulus|\bcircle|\bcircular",
    "triangle_area_base_height": r"\btrapez|\bparallel sides\b|\bparallelogram|\bsides\b",
    "circle_measures": r"\bannul|\brings?\b|\bwasher|\binner\b|\bouter\b|\bconcentric|\bbetween\b|\bradii\b|"
                       r"\bsector|\bsemi-?circ|\bquarter|\bhalf\b|\bellipse|\bspher|\bcylind|\bcone\b|\btorus",
    "sphere_measures": r"\bspheroid|\bellipso|\boblate|\bprolate|\bequatorial|\bpolar\b|\bsemi-?axe?s|\baxes\b|"
                       r"\bpolygon|\btriangle|\bvertices|\bvertex|\bcap\b|\bdome\b|\bhemispher|\bshell|\bhollow|\binner\b|"
                       r"\bouter\b|\bwedge|\blune|\bzone\b|\bexcess\b|\bsolid angle|\bgreat[- ]circle|\bradii\b|"
                       r"\bnear-?spherical|\bapproximately spherical|\bcylind|\bcone\b|\bcube\b|\btorus",
    "cone_measures": r"\bfrustum|\btruncat|\b(?:wide|narrow|wider|narrower|top|bottom|upper|lower|large|small|"
                     r"larger|smaller)\s+(?:end|base|opening|rim)?\s*radius|\bends?\b|\bradii\b|\btwo radi|\bcut off|"
                     r"\bopenings?\b|\bcylind|\bspher|\bhemispher",
    "cylinder_measures": r"\bhollow|\binner\b|\bouter\b|\bthickness|\bradii\b|\bshell\b|\bcone\b|\bhemispher|\bfrustum",
    "cuboid_measures": r"\bpyramid|\bhollow|\binner\b|\bouter\b|\bthickness",
    "trapezoid_measures": r"\bnon-?parallel\b.*\bheight\b.*\bnot\b",
    "polyline_length": r"\bshortest\b|\bnearest\b|\bclosest\b|\bany point\b|\bperpendicular|"
                       r"\bto the (?:line )?segment\b",
    "sector_measures": r"\bsegment\b|\bannul|\binner\b|\bouter\b",
    "circular_segment": r"\bsector\b",
    "regular_polygon_measures": r"\bprism\b|\bpyramid\b",
    "regular_polygon_angles": r"\bprism\b|\bpyramid\b",
    "spherical_cap": r"\bspheroid|\bellipso",
    "great_circle_distance": r"\bbearing\b|\bazimuth\b|\bheading\b",
    "initial_bearing": r"\bdistance\b",
}
_FOREIGN_RE = {k: re.compile(v) for k, v in _FOREIGN.items()}


def _g_clean(q: str) -> str:
    """Rule 1: area and volume units are units, not shapes ("square meters" -> "meters^2")."""
    q = _G_AREA_UNIT.sub(lambda m: m.group(1) + "^2", q)
    return _G_VOL_UNIT.sub(lambda m: m.group(1) + "^3", q)


def _g_stated(q: str, key: str) -> list:
    """The numbers the question states (absolute values), notation and names excluded."""
    if key in _G_EQ_KEYS:
        q = _G_EQUATION.sub(lambda m: " " * len(m.group(0)) if re.search(r"[xyz]", m.group(0)) else m.group(0), q)
    q = _G_NOTATION.sub(lambda m: " " * len(m.group(0)), q)
    return [_g_value(m) for m in _G_NUM.finditer(q)]


def _g_value(m) -> float:
    """A written number; a fraction p/q is one number (a scale factor 1/2 is 0.5)."""
    if m.group(1) is not None:
        return float(Fraction(int(m.group(1)), int(m.group(2)))) if int(m.group(2)) else float("nan")
    return float(m.group(0))


def _g_bound(parsed) -> list:
    nums = []

    def walk(x):
        if isinstance(x, bool) or x is None:
            return
        if isinstance(x, (int, float)):
            nums.append(abs(float(x)))
        elif isinstance(x, str):
            nums.extend(abs(_g_value(t)) for t in _G_NUM.finditer(x.lower()))
        elif isinstance(x, dict):
            for v in x.values():
                walk(v)
        elif isinstance(x, (list, tuple)):
            for v in x:
                walk(v)
    walk(parsed)
    return nums


def _g_same(a, b):
    return abs(a - b) <= 1e-9 * max(1.0, abs(a), abs(b))


def _g_is_bound(v: float, bound: list, q: str) -> bool:
    for b in bound:
        if _g_same(v, b):
            return True
        if re.search(r"\bdiameters?\b", q) and _g_same(v, 2 * b):          # a diameter read as its radius
            return True
        if re.search(r"\b(?:apex|vertex|opening|full|aperture) angle\b", q) and _g_same(v, 2 * b):   # a full angle
            return True                                                     # read as the half-angle
        if re.search(r"\brad(?:ians?)?\b", q) and _g_same(math.degrees(v), b):   # radians read as degrees
            return True
    return False


def _g_guarded(key, parse):
    foreign = _FOREIGN_RE.get(key)

    def guarded(q0):
        q = _g_clean(_norm(q0))
        if foreign is not None and foreign.search(q):          # rule 3: the question asks for another task
            return None
        try:
            p = parse(q)                                       # the parsers normalize again (idempotent)
        except (ValueError, TypeError, ZeroDivisionError, OverflowError, IndexError, KeyError):
            return None                                        # a question the parser cannot read is refused
        if not p or "_out_of_scope" in p:
            return p
        bound = _g_bound(p)                                    # rule 2: every stated number is bound
        if any(not _g_is_bound(v, bound, q) for v in _g_stated(q, key)):
            return None
        return p
    return guarded


for _k, _spec in PROCEDURES.items():
    _spec["parse"] = _g_guarded(_k, _spec["parse"])

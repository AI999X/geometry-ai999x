# geometry-ai999x — offline, verified geometry solver with step-by-step answers

[![tests](https://github.com/AI999X/geometry-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/geometry-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/geometry-ai999x)](https://pypi.org/project/geometry-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/geometry-ai999x)

**[Install](https://pypi.org/project/geometry-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/geometry-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/geometry-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

geometry-ai999x is an offline, deterministic Python solver that answers geometry questions with verified results from 76 certified capabilities, gives 375 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**geometry-ai999x** is a compact, deterministic geometry engine for Python. It reads a question in plain English (or a typed call), picks a certified geometry capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified geometry capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 375 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** plane and solid figures, coordinate geometry, conic sections, polygons and polyhedra, computational geometry, transformations, spherical geometry and curves and surfaces.

## Model overview

| Property | Value |
|---|---|
| Package | `geometry-ai999x` 1.0.0 (import `geometry_ai999x`) |
| Type | Deterministic neuro-symbolic geometry engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 76 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 375 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install geometry-ai999x
```

```python
import geometry_ai999x as geometry

geometry.ask("Find the circumcenter and circumradius of the triangle with vertices (0, 0), (4, 0) and (0, 3).").show()
# -> SOLVED — cx = 2, cy = 1.5, radius = 2.5  (triangle_circumcircle)
geometry.ask("Find the convex hull of the points (0, 0), (2, 1), (4, 0), (3, 3), (1, 2), (2, 4).").show()
# -> SOLVED — hull = [[0.0, 0.0], [4.0, 0.0], [3.0, 3.0], [2.0, 4.0]], n_vertices = 4, area = 9, perimeter = 13.0486  (convex_hull)
geometry.ask("Where does the line y = x + 1 intersect the circle x^2 + y^2 = 25?").show()
# -> SOLVED — count = 2, x1 = -4, y1 = -3, x2 = 3  (circle_line_intersection)
geometry.ask("Find the volume and surface area of a regular icosahedron with edge 2.").show()
# -> SOLVED — vertices = 12, edges = 30, faces = 20, euler_characteristic = 2  (platonic_solid)
geometry.solve("great_circle_distance", lat1=51.5074, lon1=-0.1278, lat2=40.7128, lon2=-74.006).show()
# -> SOLVED — central_angle = 50.0942, distance = 5570.22  (great_circle_distance)
geometry.ask("Who created you?").message
# -> geometry-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `geometry-ai999x "<question>"`, `geometry-ai999x --capabilities`. The outputs above were produced by running this exact code on geometry-ai999x 1.0.0.

## Find the right capability, and draw it

`geometry.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "geometry-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import geometry_ai999x as geometry

geometry.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (algebra-ai999x, 0.67); Solution of two linear equations in two unknowns (algebra-ai999x, 0.57)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

76 certified procedures. A few of them:

- **Procedures:** Angle between two lines; Angle between two planes; Arc length of a parabola; Area and spherical excess of a spherical triangle; Area of a triangle from base and height; Area of an annulus (ring); Area of the intersection of two convex polygons; Barycentric coordinates of a point in a triangle; Centroid of a polygon; Circle: area, circumference and diameter; Circular sector: arc length, area and chord; Circular segment: area, chord, height, arc; Circumcircle: the circle through three points; Classify a conic section from its general equation; Classify a triangle by its sides and angles; Closest pair of points; Cone: volume, slant height and surface area; Convex hull of points in the plane; Cuboid (rectangular box): volume, surface area, space diagonal; Curvature of a polynomial graph at a point; Curvature of a torus at a point; Curvature of an ellipse at a point; Cylinder: volume and surface area; Delaunay triangulation and Voronoi diagram: counts …

`geometry_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified geometry capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (plane and solid figures, coordinate geometry, conic sections, polygons and polyhedra, computational geometry, transformations, spherical geometry and curves and surfaces); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{geometry_ai999x_1_0_0,
  title  = {geometry-ai999x: an offline, verified geometry solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/geometry-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/geometry-ai999x/discussions or visit https://ai999x.com.

## Support the project

geometry-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/geometry-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/geometry-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/geometry-ai999x/tree/main/onnx).

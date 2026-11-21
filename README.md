# Geometry (geometry-ai999x)

[![tests](https://github.com/AI999X/geometry-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/geometry-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/geometry-ai999x)](https://pypi.org/project/geometry-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/geometry-ai999x)

Geometry covers plane and solid geometry, coordinate geometry, conic sections, polygons and polyhedra, computational geometry (convex hulls, Voronoi, triangulation) and curves and surfaces: 78 calculations in 14 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Geometry desktop app: Circumcircle: the circle through three points, with its worked example and live scene](https://www.ai999x.com/?ax_shot=geometry-ai999x/hero.webp)](https://www.ai999x.com/models/geometry-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-geometry-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-geometry-1.6.6-linux-x86_64.tar.gz) |
| **Command line** `geometry` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/geometry-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install geometry-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Geometry desktop app: every scene and chart is drawn from the computed answer.

[![Geometry desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=geometry-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=geometry-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=geometry-ai999x/clip.mp4)*

| | |
|---|---|
| [![Circumcircle: the circle through three points: the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-1.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-triangle_circumcircle)<br>Circumcircle: the circle through three points | [![Convex hull of points in the plane: the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-2.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-convex_hull)<br>Convex hull of points in the plane |
| [![Platonic solids: counts, Euler characteristic, volume, surface, radii: the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-3.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-platonic_solid)<br>Platonic solids: counts, Euler characteristic, volume, surface, radii | [![Great-circle distance between two points on a sphere: the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-4.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-great_circle_distance)<br>Great-circle distance between two points on a sphere |
| [![Classify a conic section from its general equation: the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-5.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-conic_classify)<br>Classify a conic section from its general equation | [![Area of an annulus (ring): the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-6.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-annulus_area)<br>Area of an annulus (ring) |
| [![Centroid of a polygon: the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-7.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-polygon_centroid)<br>Centroid of a polygon | [![Similar figures: scale factor, missing lengths, area and volume ratios: the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-8.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-similar_figures)<br>Similar figures: scale factor, missing lengths, area and volume ratios |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Circumcircle: the circle through three points: a chart in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/chart-1.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-triangle_circumcircle)<br>Circumcircle: the circle through three points | [![Convex hull of points in the plane: a chart in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/chart-2.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-convex_hull)<br>Convex hull of points in the plane | [![Platonic solids: counts, Euler characteristic, volume, surface, radii: a chart in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/chart-3.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-platonic_solid)<br>Platonic solids: counts, Euler characteristic, volume, surface, radii | [![Great-circle distance between two points on a sphere: a chart in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/chart-4.webp)](https://www.ai999x.com/models/geometry-ai999x/#cap-great_circle_distance)<br>Great-circle distance between two points on a sphere |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import geometry_ai999x as geometry

a = geometry.ask(
    "Find the area of intersection of the polygon with vertices (0, "
    "0), (2, 0), (2, 2), (0, 2) and the polygon with vertices (1, 1), "
    "(3, 1), (3, 3), (1, 3)."
)
print(a.outcome, a.values)
# SOLVED {'area': 1.0}

a = geometry.ask("Classify the conic x^2 + 4y^2 - 4 = 0.")
print(a.outcome, a.values)
# SOLVED {'type': 'ellipse'}

a = geometry.ask(
    "Find the angle between the lines y = x + 1 and y = -x + 3."
)
print(a.outcome, a.values)
# SOLVED {'angle': 90.0}

a = geometry.ask("Find the arc length of y = x^2 from x = 0 to x = 1.")
print(a.outcome, a.values)
# SOLVED {'length': 1.47894285754}

a = geometry.ask(
    "Find the area of a triangle with base 10 and height 6."
)
print(a.outcome, a.values)
# SOLVED {'area': 30.0}

a = geometry.ask(
    "Find the centroid of the polygon with vertices (0, 0), (4, 0), "
    "(4, 2), (0, 2)."
)
print(a.outcome, a.values)
# SOLVED {'cx': 2.0, 'cy': 1.0}

a = geometry.ask(
    "Find the angle between the planes z = 0 and x + z = 2."
)
print(a.outcome, a.values)
# SOLVED {'angle': 45.0}

a = geometry.ask(
    "What is the solid angle of a cone with half-angle 60 degrees?"
)
print(a.outcome, a.values)
# SOLVED {'solid_angle': 3.14159265359, 'fraction_of_sphere': 0.25}

a = geometry.ask(
    "Find the area of the spherical triangle with vertices (0, 0), (0, "
    "90) and (90, 0) on a sphere of radius 1."
)
print(a.outcome, a.values)
# SOLVED {'excess': 90.0, 'area': 1.57079632679}

a = geometry.ask(
    "Find the Gaussian curvature of a torus with major radius 3 and "
    "minor radius 1 at tube angle 0 degrees."
)
print(a.outcome, a.values)
# SOLVED {'gaussian': 0.25, 'mean': 0.625, 'k1': 1.0, 'k2': 0.25}

a = geometry.ask(
    "Dilate the point (2, 3) by a scale factor of 3 about the origin."
)
print(a.outcome, a.values)
# SOLVED {'x': 6.0, 'y': 9.0}

a = geometry.ask(
    "Find the closest pair distance among the points (0, 0), (5, 5), "
    "(1, 1), (9, 2), (6, 6.5)."
)
print(a.outcome, a.values)
# SOLVED {'distance': 1.41421356237}

a = geometry.ask(
    "Find the perimeter of an ellipse with semi-major axis 5 and "
    "semi-minor axis 3."
)
print(a.outcome, a.values)
# SOLVED {'perimeter': 25.5269988634, 'area': 47.1238898038}

a = geometry.ask(
    "Find the barycentric coordinates of the point (1, 1) in the "
    "triangle with vertices (0, 0), (4, 0), (0, 4)."
)
print(a.outcome, a.values)
# SOLVED {'l1': 0.5, 'l2': 0.25, 'l3': 0.25}

a = geometry.ask("Find the curvature of y = x^2 at x = 0.")
print(a.outcome, a.values)
# SOLVED {'curvature': 2.0, 'radius_of_curvature': 0.5, 'slope': 0.0}
```

## Command line: 15 examples

Each command below was run with `geometry` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ geometry solve convex_polygon_intersection "p=[0, 0, 2, 0, 2, 2, 0, 2]" "q=[1, 1, 3, 1, 3, 3, 1, 3]"
area = 1

$ geometry solve conic_classify A=1 B=0 C=4 D=0 E=0 F=-4
type = ellipse

$ geometry solve angle_between_lines a1=-1 b1=1 c1=-1 a2=1 b2=1 c2=-3
angle = 90

$ geometry solve parabola_arc_length a=1 b=0 c=0 x0=0 x1=1
length = 1.478942858

$ geometry solve triangle_area_base_height base=10 height=6
area = 30

$ geometry solve polygon_centroid "points=[0, 0, 4, 0, 4, 2, 0, 2]"
cx = 2
cy = 1

$ geometry solve platonic_solid solid=icosahedron edge=2
vertices = 12
edges = 30
faces = 20
euler_characteristic = 2
volume = 17.45355993
surface_area = 34.64101615

$ geometry solve similar_figures measure=lengths m1=4 m2=6
scale_factor = 1.5
area_ratio = 2.25
volume_ratio = 3.375
length2 = null
area2 = null
volume2 = null

$ geometry solve angle_between_planes a1=0 b1=0 c1=1 a2=1 b2=0 c2=1
angle = 45

$ geometry solve solid_angle_cone half_angle=60
solid_angle = 3.141592654
fraction_of_sphere = 0.25

$ geometry solve cone_measures radius=3 height=4
volume = 37.69911184
slant_height = 5
lateral_area = 47.1238898
total_area = 75.39822369

$ geometry solve spherical_triangle_area radius=1 "points=[0, 0, 0, 90, 90, 0]"
excess = 90
area = 1.570796327

$ geometry solve torus_curvature major=3 minor=1 v=0
gaussian = 0.25
mean = 0.625
k1 = 1
k2 = 0.25

$ geometry solve dilate_point k=3 px=2 py=3
x = 6
y = 9

$ geometry solve closest_pair "points=[0, 0, 5, 5, 1, 1, 9, 2, 6, 6.5]"
distance = 1.414213562
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Geometry on ai999x.com](https://www.ai999x.com/models/geometry-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.6 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/geometry-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/geometry-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `geometry` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/geometry`) and its data (`rust/models/geometry/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p geometry-ai999x
./target/release/geometry --help
cargo test -p geometry-ai999x
```

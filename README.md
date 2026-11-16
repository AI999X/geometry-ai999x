# Geometry (geometry-ai999x)

[![tests](https://github.com/AI999X/geometry-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/geometry-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/geometry-ai999x)](https://pypi.org/project/geometry-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/geometry-ai999x)

Geometry covers plane and solid geometry, coordinate geometry, conic sections, polygons and polyhedra, computational geometry (convex hulls, Voronoi, triangulation) and curves and surfaces: 78 calculations in 14 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Geometry desktop app: Area of the intersection of two convex polygons, with its worked example and live scene](https://www.ai999x.com/?ax_shot=geometry-ai999x/hero.webp)](https://www.ai999x.com/models/geometry-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `geometry` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/geometry-ai999x/#try) |
| **Python** 3.10+ | `pip install geometry-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

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

## Screenshots

[![The Geometry desktop app: Area of the intersection of two convex polygons, with its worked example and live scene](https://www.ai999x.com/?ax_shot=geometry-ai999x/hero-dark.webp)](https://www.ai999x.com/models/geometry-ai999x/)

[![Length of a polyline (path through points): the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/see-1.webp)](https://www.ai999x.com/models/geometry-ai999x/)

*Length of a polyline (path through points)*

[![Area of a triangle from base and height: the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/see-2.webp)](https://www.ai999x.com/models/geometry-ai999x/)

*Area of a triangle from base and height*

[![Right prism on a regular polygon base: volume and surface area: the live scene in the Geometry desktop app](https://www.ai999x.com/?ax_shot=geometry-ai999x/see-3.webp)](https://www.ai999x.com/models/geometry-ai999x/)

*Right prism on a regular polygon base: volume and surface area*

Every calculation, with its formula, a worked example and its figures: **[Geometry on ai999x.com](https://www.ai999x.com/models/geometry-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/geometry-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/geometry-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `geometry` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/geometry`) and its data (`rust/models/geometry/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p geometry-ai999x
./target/release/geometry --help
cargo test -p geometry-ai999x
```

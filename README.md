# geometry-ai999x: offline, verified geometry solver with step-by-step answers

[![tests](https://github.com/AI999X/geometry-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/geometry-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/geometry-ai999x)](https://pypi.org/project/geometry-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/geometry-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/geometry-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/geometry-ai999x/)** · **[Guides](https://huggingface.co/AI999X/geometry-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/geometry-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Mathematics & Statistics** · Tasks: Question Answering

geometry-ai999x is an offline, deterministic geometry solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers geometry questions with verified results from 78 certified capabilities, gives 391 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**geometry-ai999x** reads a question in plain English (or a typed call), picks a certified geometry capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** geometry-ai999x comes as the AI999X desktop app and as the command-line program `geometry` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Geometry desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-geometry-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-geometry-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`geometry`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/geometry-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install geometry-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Geometry** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Geometry among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-geometry-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-geometry-1.5.0-linux-x86_64.tar.gz
./ai999x-geometry-1.5.0-linux-x86_64/ai999x-geometry
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
geometry --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
geometry --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ geometry ask "Find the circumcenter and circumradius of the triangle with vertices (0, 0), (4, 0) and (0, 3)."
Read as: Circumcircle: the circle through three points
cx = 2
cy = 1.5
radius = 2.5
  by: Circumcircle: the circle through three points  (intersection of the perpendicular bisectors, solved in coordinates relative to the first point)
  assumes: exactly 3 points; not collinear
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ geometry info circle_line_intersection
Intersection of a circle and a line
  foot of the perpendicular from the centre, then the half-chord sqrt(r^2 - d^2) along the line
  in   cx       x of the centre [1]
  in   cy       y of the centre [1]
  in   r        radius [1]
  in   a        coefficient of x of the line [1]
  in   b        coefficient of y of the line [1]
  in   c        constant of the line [1]
  out  count    number of intersection points (0, 1 or 2) [1]
  out  x1       x of the first point (ordered by x, then y) [1]
  out  y1       y of the first point [1]
  out  x2       x of the second point [1]
  out  y2       y of the second point [1]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ geometry solve circle_line_intersection cx=0.0 cy=0.0 r=5.0 a=-1.0 b=1.0 c=-1.0
count = 2
x1 = -4
y1 = -3
x2 = 3
y2 = 4
  by: Intersection of a circle and a line  (foot of the perpendicular from the centre, then the half-chord sqrt(r^2 - d^2) along the line)
  cx = 0
  cy = 0
  r = 5
  a = -1
  b = 1
  c = -1
  assumes: r > 0; a and b not both 0; tangency is decided exactly on the written values; points ordered by x, then y; absent points are empty
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | geometry batch

`cases.jsonl`:

```json
{"key": "circle_line_intersection", "inputs": {"cx": 0.0, "cy": 0.0, "r": 5.0, "a": -1.0, "b": 1.0, "c": -1.0}}
{"key": "triangle_circumcircle", "inputs": {"points": "[0, 0, 4, 0, 0, 3]"}}
```

```console
$ geometry batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":2.0,"unit":"","solved_for":"count","si":[["cx",0.0],["cy",0.0],["r",5.0],["a",-1.0],["b",1.0],["c",-1.0]],"values":[["count",2,""],["x1",-4.0,""],["y1",-3.0,""],["x2",3.0,""],["y2",4.0,""]],"target":"count","phrase":"number of intersection points (0, 1 or 2)","assumptions":["r > 0","a and b not both 0","tangency is decided exactly on the written values","points ordered by x, then y; absent points are empty"],"name":"Intersection of a circle and a line","equation":"foot of the perpendicular from the centre, then the half-chord sqrt(r^2 - d^2) along the line"}
{"outcome":"SOLVED","message":"","value":2.5,"unit":"","solved_for":"radius","values":[["cx",2.0,""],["cy",1.5,""],["radius",2.5,""]],"target":"radius","phrase":"x of the circumcentre","assumptions":["exactly 3 points","not collinear"],"name":"Circumcircle: the circle through three points","equation":"intersection of the perpendicular bisectors, solved in coordinates relative to the first point"}
```

### In your browser

Open [Geometry on ai999x.com](https://www.ai999x.com/models/geometry-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `geometry_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install geometry-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Area of the intersection of two convex polygons

The area common to two convex polygons given by their vertices.

Command line:

```console
$ geometry solve convex_polygon_intersection "p=[0, 0, 2, 0, 2, 2, 0, 2]" "q=[1, 1, 3, 1, 3, 3, 1, 3]"
area = 1
  by: Area of the intersection of two convex polygons  (Sutherland-Hodgman clipping of one polygon by the other's edges, then the shoelace area)
  assumes: both polygons simple and convex
```

Python:

```python
import geometry_ai999x as m

a = m.solve("convex_polygon_intersection", p=[0, 0, 2, 0, 2, 2, 0, 2], q=[1, 1, 3, 1, 3, 3, 1, 3])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/geometry-ai999x/#cap-convex_polygon_intersection) and press **Compute in your browser**.

_What this means:_ This calculation determines the common region shared by two convex polygons, defined by their ordered vertices. Clipping one polygon against the edges of the other and applying the shoelace formula to the resulting shape gives the precise overlap area. The result, 1, represents the total size of this intersection in square units, indicating how much space the two shapes occupy simultaneously.

### Closest pair of points

The smallest distance between any two of a set of points in the plane.

Command line:

```console
$ geometry solve closest_pair "points=[0, 0, 5, 5, 1, 1, 9, 2, 6, 6.5]"
distance = 1.414213562
  by: Closest pair of points  (sort by x and sweep, stopping each scan once the x-gap exceeds the best distance)
  assumes: at least 2 points, at most 2000; a repeated point gives distance 0
```

Python:

```python
import geometry_ai999x as m

a = m.solve("closest_pair", points=[0, 0, 5, 5, 1, 1, 9, 2, 6, 6.5])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/geometry-ai999x/#cap-closest_pair) and press **Compute in your browser**.

_What this means:_ This calculation determines the smallest Euclidean distance between any two points in the provided set. By sorting the points and sweeping through them, the algorithm identifies the minimum separation, which is reported as 1.41421. This value represents the tightest proximity found among all possible pairs.

### Convex hull of points in the plane

The convex hull of a set of points: its vertices (counterclockwise), their number, its area and perimeter.

Command line:

```console
$ geometry solve convex_hull "points=[0, 0, 2, 1, 4, 0, 3, 3, 1, 2, 2, 4]"
hull = [[0.0,0.0],[4.0,0.0],[3.0,3.0],[2.0,4.0]]
n_vertices = 4
area = 9
perimeter = 13.04862718
  by: Convex hull of points in the plane  (Andrew's monotone chain with exact (rational) orientation tests; shoelace area)
  assumes: at least 3 points, not all collinear, at most 5000; points on a hull edge between two vertices are not vertices; repeated points count once
```

Python:

```python
import geometry_ai999x as m

a = m.solve("convex_hull", points=[0, 0, 2, 1, 4, 0, 3, 3, 1, 2, 2, 4])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/geometry-ai999x/#cap-convex_hull) and press **Compute in your browser**.

_What this means:_ This calculation determines the smallest convex polygon that contains all provided points. The result identifies 4 specific vertices arranged in counterclockwise order, starting from the point with the smallest x-coordinate. The enclosed region has an area of 9 and a boundary length of 13.0486. This process assumes the input set contains at least three non-collinear points to form a valid two-dimensional shape.

### Delaunay triangulation and Voronoi diagram: counts

Numbers of Delaunay triangles and edges, hull vertices, Voronoi vertices and unbounded Voronoi cells of points in general position.

Command line:

```console
$ geometry solve delaunay_voronoi_counts "points=[0, 0, 4, 0, 0, 3, 5, 4, 2, 1]"
triangles = 4
edges = 8
hull_vertices = 4
voronoi_vertices = 4
unbounded_cells = 4
  by: Delaunay triangulation and Voronoi diagram: counts  (exact general-position checks, the convex hull, then Euler's formula for a triangulation; Voronoi is the dual)
  assumes: 3 to 25 distinct points; general position: no three collinear, no four concyclic (checked exactly)
```

Python:

```python
import geometry_ai999x as m

a = m.solve("delaunay_voronoi_counts", points=[0, 0, 4, 0, 0, 3, 5, 4, 2, 1])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/geometry-ai999x/#cap-delaunay_voronoi_counts) and press **Compute in your browser**.

_What this means:_ Given a set of distinct points in general position, this calculation determines the structural counts for both the Delaunay triangulation and its dual, the Voronoi diagram. The number of triangles is derived from the total point count minus the convex hull vertices, resulting in 4 triangles. Correspondingly, there are 8 edges connecting these points. The Voronoi diagram mirrors this structure with 4 vertices, matching the triangle count, and 4 unbounded regions, which correspond to the 4 vertices on the outer boundary of the point cloud.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/geometry-ai999x/blob/main/docs/geometry_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/geometry-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified geometry capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 391 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `geometry` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** plane and solid figures, coordinate geometry, conic sections, polygons and polyhedra, computational geometry, transformations, spherical geometry and curves and surfaces.

## Model overview

| Property | Value |
|---|---|
| Package | `geometry-ai999x` 1.6.0 (import `geometry_ai999x`) |
| Type | Deterministic neuro-symbolic geometry engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 78 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 391 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `geometry-ai999x "<question>"`; for the native program use `geometry` (above). The outputs above were produced by running this exact code on geometry-ai999x 1.6.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `geometry-ai999x`; the answer is the one it returned, with the steps it took.

**Circumcircle: the circle through three points** · _Coordinate geometry_

> Find the circumcenter and circumradius of the triangle with vertices (0, 0), (4, 0) and (0, 3).

→ **cx = 2, cy = 1.5, radius = 2.5**

<details><summary>Steps</summary>

- Circumcircle: the circle through three points: points = [0, 0, 4, 0, 0, 3] → cx = 2, cy = 1.5, radius = 2.5

</details>

**Convex hull of points in the plane** · _Computational geometry_

> Find the convex hull of the points (0, 0), (2, 1), (4, 0), (3, 3), (1, 2), (2, 4).

→ **hull = [[0, 0], [4, 0], [3, 3], [2, 4]], n_vertices = 4, area = 9, perimeter = 13.04862718**

<details><summary>Steps</summary>

- Convex hull of points in the plane: points = [0, 0, 2, 1, 4, 0, 3, 3, 1, 2, 2, 4] → hull = [[0, 0], [4, 0], [3, 3], [2, 4]], n_vertices = 4, area = 9, perimeter = 13.04862718

</details>

**Intersection of a circle and a line** · _Coordinate geometry_

> Where does the line y = x + 1 intersect the circle x^2 + y^2 = 25?

→ **count = 2, x1 = -4, y1 = -3, x2 = 3, y2 = 4**

<details><summary>Steps</summary>

- Intersection of a circle and a line: cx = 0, cy = 0, r = 5, a = -1, b = 1, c = -1 → count = 2, x1 = -4, y1 = -3, x2 = 3, y2 = 4

</details>

**Platonic solids: counts, Euler characteristic, volume, surface, radii** · _Polyhedra_

> Find the volume and surface area of a regular icosahedron with edge 2.

→ **vertices = 12, edges = 30, faces = 20, euler_characteristic = 2, volume = 17.45355993, surface_area = 34.64101615, circumradius = 1.902113033, inradius = 1.511522628**

<details><summary>Steps</summary>

- Platonic solids: counts, Euler characteristic, volume, surface, radii: solid = icosahedron, edge = 2 → vertices = 12, edges = 30, faces = 20, euler_characteristic = 2, volume = 17.45355993, surface_area = 34.64101615, circumradius = 1.902113033, inradius = 1.511522628

</details>

**Great-circle distance between two points on a sphere** · _Spherical geometry_

`m.solve("great_circle_distance", lat1=51.5074, lon1=-0.1278, lat2=40.7128, lon2=-74.006)`

→ **central_angle = 50.09421156, distance = 5570.22218**

<details><summary>Steps</summary>

- Great-circle distance between two points on a sphere: lat1 = 51.5074, lon1 = -0.1278, lat2 = 40.7128, lon2 = -74.006 → central_angle = 50.09421156, distance = 5570.22218

</details>

**Classify a conic section from its general equation** · _Conic sections_

> Classify the conic x^2 + 4y^2 - 4 = 0.

→ **type = ellipse**

<details><summary>Steps</summary>

- Classify a conic section from its general equation: A = 1, B = 0, C = 4, D = 0, E = 0, F = -4 → type = ellipse

</details>

## Figures

Every point was computed by `geometry-ai999x`.

**Convex hull and smallest enclosing circle of ten points**

![Convex hull and smallest enclosing circle of ten points, computed by geometry-ai999x](https://huggingface.co/AI999X/geometry-ai999x/resolve/main/figures/convex-hull-shape.svg)

_Ten points (0 0, 5 5, 1 1, 9 2, 6 6.5, 3 7, 7 3, 2 4, 8 6, 4 1.5), their convex hull as returned by the package (5 vertices, area 38.25, perimeter 26.0614), and the smallest circle enclosing them; marked: its centre x = 4.07895, y = 2.89474, radius 5.00173._

**A triangle's circles and centres**

![A triangle's circles and centres, computed by geometry-ai999x](https://huggingface.co/AI999X/geometry-ai999x/resolve/main/figures/triangle-circumcircle-shape.svg)

_The triangle with vertices 0 0, 4 0, 1 3: circumcircle (centre O = (2, 1), radius 2.23607), incircle (centre I = (1.45982, 1.05218), radius 1.05218), orthocentre H = (1, 1) and centroid G = (1.66667, 1), each computed by the package; O, G and H lie on the Euler line._

**A polygon's angles, area and centroid**

![A polygon's angles, area and centroid, computed by geometry-ai999x](https://huggingface.co/AI999X/geometry-ai999x/resolve/main/figures/polygon-centroid-shape.svg)

_The L-shaped polygon 0 0, 6 0, 6 2, 2 2, 2 5, 0 5, with the interior angle at each vertex (sum 720°, 1 reflex), area 18 and perimeter 22; marked: the centroid x = 2.33333, y = 1.83333, all computed by the package. The centroid of a non-convex shape need not be its middle._

**Tangent lines from an outside point to a circle**

![Tangent lines from an outside point to a circle, computed by geometry-ai999x](https://huggingface.co/AI999X/geometry-ai999x/resolve/main/figures/tangent-from-point-shape.svg)

_Tangents from P = (5, 0) to the circle of centre C = (0, 0) and radius 3: the package gives the touching points (1.8, -2.4) and (1.8, 2.4) and the tangent length 4. The circle on the diameter CP meets the circle at the same two points (its intersection, also computed by the package: (1.8, -2.4) and (1.8, 2.4))._

**Rotation, reflection and dilation of a triangle**

![Rotation, reflection and dilation of a triangle, computed by geometry-ai999x](https://huggingface.co/AI999X/geometry-ai999x/resolve/main/figures/rotate-point-shape.svg)

_The triangle (1, 1), (4, 1), (2, 3) and three images, each vertex mapped by the package: rotated 90° about the origin ((-1, 1), (-1, 4), (-3, 2)), reflected in the line x + y = 0 ((-1, -1), (-1, -4), (-3, -2)) and dilated about the origin by −0.5 ((-0.5, -0.5), (-2, -0.5), (-1, -1.5))._

**Angles of regular polygons, 3 to 20 sides**

![Angles of regular polygons, 3 to 20 sides, computed by geometry-ai999x](https://huggingface.co/AI999X/geometry-ai999x/resolve/main/figures/regular-polygon-angles-stems.svg)

_Interior and exterior angles of the regular polygons with 3 to 20 sides, each computed by the package: the interior angle climbs towards 180° (triangle 60°, 20-gon 162°) while the exterior angle 360°/n falls; marked: n = 6, interior angle = 120°._

## Capabilities

78 certified procedures. A few of them:

- **Procedures:** Angle between two lines; Angle between two planes; Arc length of a parabola; Area and spherical excess of a spherical triangle; Area of a triangle from base and height; Area of an annulus (ring); Area of the intersection of two convex polygons; Barycentric coordinates of a point in a triangle; Centroid of a polygon; Circle: area, circumference and diameter; Circular sector: arc length, area and chord; Circular segment: area, chord, height, arc; Circumcircle: the circle through three points; Classify a conic section from its general equation; Classify a triangle by its sides and angles; Closest pair of points; Cone: volume, slant height and surface area; Convex hull of points in the plane; Cuboid (rectangular box): volume, surface area, space diagonal; Curvature of a polynomial graph at a point; Curvature of a torus at a point; Curvature of an ellipse at a point; Cylinder: volume and surface area; Delaunay triangulation and Voronoi diagram: counts …

`geometry_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified geometry capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (plane and solid figures, coordinate geometry, conic sections, polygons and polyhedra, computational geometry, transformations, spherical geometry and curves and surfaces); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{geometry_ai999x_1_6_0,
  title  = {geometry-ai999x: an offline, verified geometry solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/geometry-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/geometry-ai999x/discussions or visit https://ai999x.com.

## Support the project

geometry-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/geometry-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.0`; the [GitHub release](https://github.com/AI999X/geometry-ai999x/releases/tag/v1.6.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `geometry` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/geometry`) and its data (`rust/models/geometry/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p geometry-ai999x
./target/release/geometry --help
cargo test -p geometry-ai999x
```

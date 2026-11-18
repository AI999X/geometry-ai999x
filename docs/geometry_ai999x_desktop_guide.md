# Geometry: the desktop app, user guide

**Geometry**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Geometry covers plane and solid geometry, coordinate geometry, conic sections, polygons and polyhedra, computational geometry (convex hulls, Voronoi, triangulation) and curves and surfaces. Its desktop app holds all 78 calculations in 14 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-geometry-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-geometry-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-geometry-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-geometry.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-geometry-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-geometry-1.6.5-linux-x86_64.tar.gz
./ai999x-geometry-1.6.5-linux-x86_64/ai999x-geometry
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Geometry app, opened on Circumcircle: the circle through three points with its worked example.](https://www.ai999x.com/?ax_shot=geometry-ai999x/hero.webp)

*The Geometry app, opened on Circumcircle: the circle through three points with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Coordinate geometry › Circumcircle: the circle through three points (or type part of its name in the search box). It opens with a worked example filled in: “Find the circumcenter and circumradius of the triangle with vertices (0, 0), (4, 0) and (0, 3).” The answer: cx = 2, cy = 1.5, radius = 2.5.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Circumcircle: the circle through three points: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-1.webp)

*Circumcircle: the circle through three points: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Find the circumcenter and circumradius of the triangle with vertices (0, 0), (4, 0) and (0, 3).” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Circumcircle: the circle through three points
cx = 2
cy = 1.5
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Convex hull of points in the plane](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-2.webp)

*Convex hull of points in the plane*

![Platonic solids: counts, Euler characteristic, volume, surface, radii](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-3.webp)

*Platonic solids: counts, Euler characteristic, volume, surface, radii*

![Great-circle distance between two points on a sphere](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-4.webp)

*Great-circle distance between two points on a sphere*

![Classify a conic section from its general equation](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-5.webp)

*Classify a conic section from its general equation*

![Area of an annulus (ring)](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-6.webp)

*Area of an annulus (ring)*

![Centroid of a polygon](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-7.webp)

*Centroid of a polygon*

![Similar figures: scale factor, missing lengths, area and volume ratios](https://www.ai999x.com/?ax_shot=geometry-ai999x/scene-8.webp)

*Similar figures: scale factor, missing lengths, area and volume ratios*

![Circumcircle: the circle through three points: a chart, enlarged.](https://www.ai999x.com/?ax_shot=geometry-ai999x/chart-1.webp)

*Circumcircle: the circle through three points: a chart, enlarged.*

![Convex hull of points in the plane: a chart, enlarged.](https://www.ai999x.com/?ax_shot=geometry-ai999x/chart-2.webp)

*Convex hull of points in the plane: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Geometry app in the dark theme.](https://www.ai999x.com/?ax_shot=geometry-ai999x/hero-dark.webp)

*The Geometry app in the dark theme.*

## 7. Every calculation

78 calculations in 14 topics; each has its own entry on the model's page (https://www.ai999x.com/models/geometry-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Computational geometry | Area of the intersection of two convex polygons, Closest pair of points, Convex hull of points in the plane, Delaunay triangulation and Voronoi diagram: counts, Distance from a point to a segment, Farthest pair of points (diameter of a point set), Hausdorff distance between two point sets, Intersection of two line segments, Minimum-area enclosing rectangle, Nearest neighbour of a query point, Point in polygon test, Smallest enclosing circle of points |
| Conic sections | Classify a conic section from its general equation, Perimeter of an ellipse (exact, by the complete elliptic integral) |
| Coordinate geometry | Angle between two lines, Barycentric coordinates of a point in a triangle, Circumcircle: the circle through three points, Incircle of a triangle: incentre and inradius, Intersection of a circle and a line, Intersection of two circles, Intersection point of two lines, Length of a polyline (path through points), Orthocentre of a triangle, Tangents from a point to a circle, Triangle from vertex coordinates: area, perimeter, centroid, angles |
| Curves | Arc length of a parabola, Curvature of a polynomial graph at a point, Curvature of an ellipse at a point, Helix: curvature, torsion and length per turn |
| Plane figures | Area of a triangle from base and height, Area of an annulus (ring), Circle: area, circumference and diameter, Circular sector: arc length, area and chord, Circular segment: area, chord, height, arc, Classify a triangle by its sides and angles, Medians, altitudes and angle bisectors of a triangle, Rectangle: area, perimeter and diagonal, Rhombus from its diagonals: area, side, perimeter, Trapezoid: area and midsegment |
| Polygons | Centroid of a polygon, Interior angles of a polygon from its vertices, Is a polygon convex?, Pick's theorem: lattice points of a polygon, Polygon angles and diagonals, Polygon from vertices: area, perimeter, orientation, Regular polygon: area, perimeter, apothem, circumradius |
| Polyhedra | Platonic solids: counts, Euler characteristic, volume, surface, radii |
| Similarity | Similar figures: scale factor, missing lengths, area and volume ratios |
| Solid coordinate geometry | Angle between two planes, Distance between two lines in space (skew or parallel), Distance from a point to a plane, Intersection of a line and a plane, Line in space: vector, parametric and symmetric equations, Plane through three points, Sphere through four points (circumsphere of a tetrahedron), Tetrahedron from four vertices: volume and surface area, Triangle in space: area and perimeter from vertices |
| Solid geometry | Solid angle of a cone, Solid angle subtended by a triangle |
| Solids | Cone: volume, slant height and surface area, Cuboid (rectangular box): volume, surface area, space diagonal, Cylinder: volume and surface area, Frustum of a cone: volume and surface area, Right prism on a regular polygon base: volume and surface area, Right rectangular (or square) pyramid: volume and surface area, Sphere: volume and surface area, Spherical cap: volume, curved area, base radius, Spheroid (ellipsoid of revolution): surface area and volume, Torus: volume and surface area |
| Spherical geometry | Area and spherical excess of a spherical triangle, Great-circle distance between two points on a sphere, Initial bearing (forward azimuth) along a great circle |
| Surfaces | Curvature of a torus at a point, Gaussian and mean curvature of a quadratic surface z = f(x, y) |
| Transformations | Dilation (enlargement) of a point, Reflection of a point in a line, Rotation of a point, Rotation of a point in space about an axis |

## 8. Other ways to use it

The same engine runs on the command line (`geometry`, its own guide), in Python (`pip install geometry-ai999x`, `import geometry_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/geometry-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/geometry-ai999x/

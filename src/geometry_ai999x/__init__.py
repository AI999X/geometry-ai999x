"""geometry-ai999x — offline geometry answers with their formula, steps and verification.

    import geometry_ai999x as m
    m.ask("Find the circumcenter and circumradius of the triangle with vertices (0, 0), (4, 0) and (0, 3).").show()
    #   cx = 2, cy = 1.5, radius = 2.5
    m.solve("great_circle_distance", lat1=51.5074, lon1=-0.1278, lat2=40.7128, lon2=-74.006).show()
    #   central_angle = 50.0942 degrees, distance = 5570.22 km
    m.capabilities()                   # every certified capability

76 certified capabilities: plane figures (rectangles, triangles, circles, sectors and segments, trapezoids, rhombi,
regular polygons, annuli), solids (boxes, cylinders, cones, frusta, pyramids, prisms, spheres, caps, tori, spheroids,
the five Platonic solids), coordinate geometry (triangle centres, lines, reflections, rotations, dilations, circle and
line intersections, tangents, conic classification, barycentric coordinates), polygons from vertices (area, centroid,
angles, convexity, Pick's theorem), computational geometry (convex hull, closest and farthest pairs, segment
intersection, point in polygon, smallest enclosing circle, minimum-area rectangle, Delaunay/Voronoi counts, polygon
clipping, Hausdorff distance), solid coordinate geometry (planes, lines, tetrahedra, spheres through four points,
rotations about an axis), spherical geometry (great-circle distance, bearing, spherical triangles, solid angles) and
curves and surfaces (curvature of graphs and ellipses, helices, arc length, Gaussian and mean curvature). A question
that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.2"
_P = Product("geometry", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "__version__"]

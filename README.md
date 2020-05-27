# cute-c2
Rust wrapper for the cute-headers 2D collision library. See the original [cute_c2 library](https://github.com/RandyGaul/cute_headers/blob/master/cute_c2.h) in the cute headers repository by Randy Gaul. This rust wrapper supports collision detection between Circles, AABBs, Capsules and up to 8-sided convex Polygons.


There is an example program in cute-c2-examples.

API example:
```rust
use cute_c2::{self as c2, prelude::*};

fn main() {
    let circle = c2::Circle::new(c2::Vec2::new(0.0, 0.0), 15.0);
    let aabb = c2::AABB {
        min: c2::Vec2::new(10.0, 5.0),
        max: c2::Vec2::new(20.0, 30.0),
    };

    circle.collides_with(&aabb);
    // returns true

    let capsule = c2::Capsule::new(c2::Vec2::new(5.0, 5.0), c2::Vec2::new(15.0, 10.0), 1.0);

    let poly = c2::Poly::from_slice(&[
        c2::Vec2::new(-1.0, -3.0),
        c2::Vec2::new(1.0, -3.0),
        c2::Vec2::new(1.0, 0.0),
        c2::Vec2::new(0.0, 1.0),
        c2::Vec2::new(-1.0, 0.0),
    ]);

    capsule.collides_with(&poly);
    // returns false

    let transformation =
        c2::Transformation::new(c2::Vec2::new(5.0, 4.0), std::f32::consts::PI / 2.0);

    circle.collides_with(&(poly, transformation));
    // returns true

    let manifold = circle.manifold(&poly);
    /* returns a struct containing:
    Manifold {
        count: i32,
        depths: [f32; 2],
        contact_points: [Vec2; 2],
        n: Vec2, // normal
    }
    */

    let gjk_response = poly.gjk(&circle).run();
    /* returns a struct containing:
    GjkResponse {
        distance: f32,
        closest_points: (Vec2, Vec2),
    }
    */
}
```
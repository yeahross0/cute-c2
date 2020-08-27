# cute-c2
A 2D collision library for Rust. Check out the original [c2.h library](https://github.com/RandyGaul/cute_headers/blob/master/cute_c2.h) in the cute headers repository by Randy Gaul. This rust wrapper supports collision detection between Circles, AABBs, Capsules and up to 8-sided convex Polygons. There is an example program in cute-c2-examples.

API example:
```rust
use c2::{prelude::*, AABB, Circle, Capsule, Poly, Transformation, Rotation};
use std::f32::consts::PI;

fn main() {
    let circle = Circle::new([0.0, 0.0], 15.0);
    let aabb = AABB::new([10.0, 5.0], [20.0, 30.0]);

    let collided = circle.collides_with(&aabb);
    assert!(collided);

    let capsule = Capsule::new([5.0, 5.0], [15.0, 10.0], 1.0);

    let poly = Poly::from_slice(&[
        [-1.0, -3.0],
        [1.0, -3.0],
        [1.0, 0.0],
        [0.0, 1.0],
        [-1.0, 0.0],
    ]);

    let collided = capsule.collides_with(&poly);
    assert!(!collided);

    let transformation =
        Transformation::new([5.0, 4.0], Rotation::radians(PI / 2.0));

    let collided = circle.collides_with(&(poly, transformation));
    assert!(collided);
    let manifold = circle.manifold(&poly);
    /*
        The manifold is used for resolving collisions and has the following methods:
        manifold.count() -> i32
        manifold.depths() -> [f32; 2]
        manifold.contact_points() -> [Vec2; 2]
        manifold.normal() -> Vec2
    */

    let gjk_response = poly.gjk(&circle).run();
    /*
        The result of the GJK algorithm:
        gjk_response.distance() -> f32
        gjk_response.closest_points() -> (Vec2, Vec2)
    */
}
```
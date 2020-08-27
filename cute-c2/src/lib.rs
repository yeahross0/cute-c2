//! # Getting started
//!
//! ```rust
//! use c2::{prelude::*, AABB, Circle, Capsule, Poly, Transformation, Rotation};
//! use std::f32::consts::PI;
//!
//! fn main() {
//!     let circle = Circle::new([0.0, 0.0], 15.0);
//!     let aabb = AABB::new([10.0, 5.0], [20.0, 30.0]);
//!
//!     let collided = circle.collides_with(&aabb);
//!     assert!(collided);
//!
//!     let capsule = Capsule::new([5.0, 5.0], [15.0, 10.0], 1.0);
//!
//!     let poly = Poly::from_slice(&[
//!         [-1.0, -3.0],
//!         [1.0, -3.0],
//!         [1.0, 0.0],
//!         [0.0, 1.0],
//!         [-1.0, 0.0],
//!     ]);
//!
//!     let collided = capsule.collides_with(&poly);
//!     assert!(!collided);
//!
//!     let transformation =
//!         Transformation::new([5.0, 4.0], Rotation::radians(PI / 2.0));
//!
//!     let collided = circle.collides_with(&(poly, transformation));
//!     assert!(collided);
//!     let manifold = circle.manifold(&poly);
//!     /* returns a struct with the following methods:
//!     manifold.count() -> i32
//!     manifold.depths() -> [f32; 2]
//!     manifold.contact_points() -> [Vec2; 2]
//!     manifold.normal() -> Vec2
//!     */
//!
//!     let gjk_response = poly.gjk(&circle).run();
//!     /* returns a struct with the following methods:
//!     gjk_response.distance() -> f32
//!     gjk_response.closest_points() -> (Vec2, Vec2)
//!     */
//! }
//! ```

use c2_sys as ffi;
use std::os::raw::c_void;

const MAX_POLYGON_VERTS: usize = ffi::C2_MAX_POLYGON_VERTS as usize;

/// A 2d vector
#[derive(Debug, Copy, Clone)]
pub struct Vec2(ffi::c2v);

impl Vec2 {
    /// Creates a new vector
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2(ffi::c2v { x, y })
    }

    /// The x coordinate
    pub fn x(self) -> f32 {
        self.0.x
    }

    /// Set the x coordinate
    pub fn set_x(&mut self, x: f32) {
        self.0.x = x;
    }

    /// The y coordinate
    pub fn y(self) -> f32 {
        self.0.y
    }

    /// Set the y coordinate
    pub fn set_y(&mut self, y: f32) {
        self.0.y = y;
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(v: [f32; 2]) -> Vec2 {
        Vec2::new(v[0], v[1])
    }
}

/// Rotation, an angle
#[derive(Debug, Copy, Clone)]
pub struct Rotation(ffi::c2r);

impl Rotation {
    /// No rotation
    pub fn zero() -> Self {
        Rotation::radians(0.0)
    }

    // Rotation in radians
    pub fn radians(radians: f32) -> Rotation {
        Rotation(ffi::c2r {
            c: radians.cos(),
            s: radians.sin(),
        })
    }

    /// Rotation in degrees
    pub fn degrees(degrees: f32) -> Rotation {
        Rotation::radians(degrees.to_radians())
    }

    /// cos(angle)
    pub fn cos(self) -> f32 {
        self.0.c
    }

    /// sin(angle)
    pub fn sin(self) -> f32 {
        self.0.s
    }
}

/// A circle with a point and a radius
#[derive(Debug, Copy, Clone)]
pub struct Circle(ffi::c2Circle);

impl Circle {
    /// Creates a circle from a position and a radius
    pub fn new<V: Into<Vec2>>(position: V, radius: f32) -> Circle {
        Circle(ffi::c2Circle {
            p: position.into().0,
            r: radius,
        })
    }
}

/// Rectangle with a min vector and a max vector
#[derive(Debug, Copy, Clone)]
pub struct AABB(ffi::c2AABB);

impl AABB {
    /// Creates a new AABB rectangle
    pub fn new<V: Into<Vec2>>(min: V, max: V) -> AABB {
        AABB(ffi::c2AABB {
            min: min.into().0,
            max: max.into().0,
        })
    }

    /// The minimum position
    pub fn min(self) -> Vec2 {
        Vec2(self.0.min)
    }

    /// The maximum position
    pub fn max(self) -> Vec2 {
        Vec2(self.0.max)
    }
}

/// A capsule with a line segment and a radius
#[derive(Debug, Copy, Clone)]
pub struct Capsule(ffi::c2Capsule);

impl Capsule {
    /// Creates a capsule from a start position, an end position and a radius
    pub fn new<V: Into<Vec2>>(start: V, end: V, radius: f32) -> Capsule {
        Capsule(ffi::c2Capsule {
            a: start.into().0,
            b: end.into().0,
            r: radius,
        })
    }
}

/// A polygon with up to 8 sides
#[derive(Debug, Copy, Clone)]
pub struct Poly(ffi::c2Poly);

impl Poly {
    /// Creates a polygon from a slice of vectors
    pub fn from_slice<V: Copy + Into<Vec2>>(verts: &[V]) -> Poly {
        let mut poly = ffi::c2Poly {
            count: verts.len() as i32,
            verts: [ffi::c2v { x: 0.0, y: 0.0 }; MAX_POLYGON_VERTS],
            norms: [ffi::c2v { x: 0.0, y: 0.0 }; MAX_POLYGON_VERTS],
        };
        for i in 0..verts.len().min(MAX_POLYGON_VERTS) {
            poly.verts[i] = verts[i].into().0;
        }
        unsafe {
            ffi::c2MakePoly(&mut poly);
        }
        Poly(poly)
    }

    /// Creates a polygon from an array
    pub fn from_array<V: Copy + Into<Vec2>>(count: usize, verts: [V; MAX_POLYGON_VERTS]) -> Poly {
        Poly::from_slice(&verts[..count])
    }

    /// The number of sides
    pub fn count(self) -> usize {
        self.0.count as usize
    }

    /// Gets the point of the polygon at the index
    pub fn get_vert(self, index: usize) -> Vec2 {
        Vec2(self.0.verts[index])
    }
}

/// For transforming the position and rotation of polygons
#[derive(Debug, Copy, Clone)]
pub struct Transformation(ffi::c2x);

impl Transformation {
    /// Creates a new transformation with a position and an angle
    pub fn new<V: Into<Vec2>>(position: V, rotation: Rotation) -> Transformation {
        Transformation(ffi::c2x {
            p: position.into().0,
            r: rotation.0,
        })
    }

    /// Gets the position of the tranformation
    pub fn position(self) -> Vec2 {
        Vec2(self.0.p)
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.0.p = position.0;
    }

    /// Gets the rotation of the transformation
    pub fn rotation(self) -> Rotation {
        Rotation(self.0.r)
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.0.r = rotation.0;
    }
}

/// A Ray struct for casting a ray
pub struct Ray(ffi::c2Ray);

impl Ray {
    /// Creates a Ray with a position and a ray (a vector containing the direction and magnitude)
    pub fn new<V: Into<Vec2>>(position: V, ray: V) -> Ray {
        let ray = ray.into().0;
        let distance = (ray.x * ray.x + ray.y * ray.y).sqrt();
        let direction = Vec2::new(ray.x / distance, ray.y / distance);
        Ray(ffi::c2Ray {
            p: position.into().0,
            d: direction.0,
            t: distance,
        })
    }
}

/// The result of the ray casting operations
#[derive(Debug, Copy, Clone)]
pub struct RayCast(ffi::c2Raycast);

impl RayCast {
    pub fn time_of_impact(self) -> f32 {
        self.0.t
    }

    pub fn position_of_impact(self, ray: Ray) -> Vec2 {
        Vec2::new(
            ray.0.p.x + ray.0.d.x * self.0.t,
            ray.0.p.y + ray.0.d.y * self.0.t,
        )
    }

    /// The normal of the surface at impact (unit length)
    pub fn normal(self) -> Vec2 {
        Vec2(self.0.n)
    }
}

pub type GjkCache = ffi::c2GJKCache;

/// Contains the data necessary for collision resolution
#[derive(Debug, Copy, Clone)]
pub struct Manifold(ffi::c2Manifold);

impl Manifold {
    pub fn count(self) -> i32 {
        self.0.count
    }

    pub fn depths(self) -> [f32; 2] {
        self.0.depths
    }

    pub fn contact_points(self) -> [Vec2; 2] {
        [
            Vec2(self.0.contact_points[0]),
            Vec2(self.0.contact_points[1]),
        ]
    }

    /// Points from the first shape to the second
    pub fn normal(self) -> Vec2 {
        Vec2(self.0.n)
    }
}

/// The type of the shape
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Type {
    None = ffi::C2_TYPE_C2_TYPE_NONE,
    Circle = ffi::C2_TYPE_C2_TYPE_CIRCLE,
    AABB = ffi::C2_TYPE_C2_TYPE_AABB,
    Capsule = ffi::C2_TYPE_C2_TYPE_CAPSULE,
    Poly = ffi::C2_TYPE_C2_TYPE_POLY,
}

/// The result of the GJK function
#[derive(Debug, Copy, Clone)]
pub struct GjkResponse {
    distance: f32,
    closest_points: (Vec2, Vec2),
}

impl GjkResponse {
    pub fn distance(self) -> f32 {
        self.distance
    }

    pub fn closest_points(self) -> (Vec2, Vec2) {
        self.closest_points
    }
}

/// A builder for running the GJK algorithm
pub struct GjkRunner<'a, ShapeA, ShapeB> {
    a: &'a ShapeA,
    b: &'a ShapeB,
    use_radius: bool,
    cache: Option<&'a mut GjkCache>,
    iterations: Option<&'a mut i32>,
}

/// A builder for finding the time of impact for two shapes
pub struct ToiRunner<'a, ShapeA, ShapeB> {
    a: &'a ShapeA,
    b: &'a ShapeB,
    a_velocity: Vec2,
    b_velocity: Vec2,
    use_radius: bool,
    iterations: Option<&'a mut i32>,
}

pub mod prelude {
    use super::*;

    impl Ray {
        pub fn cast<T: Shape + AdvancedShape>(self, shape: T) -> Option<RayCast> {
            unsafe {
                let mut raycast = RayCast(ffi::c2Raycast {
                    t: 0.0,
                    n: Vec2::new(0.0, 0.0).0,
                });
                let hit = ffi::c2CastRay(
                    self.0,
                    shape.shape(),
                    shape.transformation(),
                    T::shape_type() as u32,
                    &mut raycast.0,
                );
                if hit != 0 {
                    Some(raycast)
                } else {
                    None
                }
            }
        }
    }

    pub trait Shape {
        fn shape_type() -> Type;

        fn shape(&self) -> *const c_void {
            self as *const _ as *const c_void
        }

        fn transformation(&self) -> *const ffi::c2x {
            std::ptr::null()
        }
    }

    pub trait BasicShape: Shape {
        /// Returns true if the two shapes are colliding, false otherwise
        fn collides_with<T: Shape>(&self, other: &T) -> bool {
            unsafe {
                ffi::c2Collided(
                    self.shape(),
                    self.transformation(),
                    Self::shape_type() as u32,
                    other.shape(),
                    other.transformation(),
                    T::shape_type() as u32,
                ) != 0
            }
        }

        /// Returns the Manifold struct for collision resolution
        fn manifold<T: Shape>(&self, other: &T) -> Manifold {
            let mut manifold = Manifold(ffi::c2Manifold {
                count: 0,
                depths: [0.0, 0.0],
                contact_points: [ffi::c2v { x: 0.0, y: 0.0 }, ffi::c2v { x: 0.0, y: 0.0 }],
                n: ffi::c2v { x: 0.0, y: 0.0 },
            });
            unsafe {
                ffi::c2Collide(
                    self.shape(),
                    self.transformation(),
                    Self::shape_type() as u32,
                    other.shape(),
                    other.transformation(),
                    T::shape_type() as u32,
                    &mut manifold.0,
                );
            }
            manifold
        }
    }

    impl<'a, ShapeA, ShapeB> GjkRunner<'a, ShapeA, ShapeB>
    where
        ShapeA: Shape,
        ShapeB: Shape,
    {
        pub fn new(a: &'a ShapeA, b: &'a ShapeB) -> GjkRunner<'a, ShapeA, ShapeB> {
            GjkRunner {
                a,
                b,
                use_radius: true,
                cache: None,
                iterations: None,
            }
        }

        pub fn use_radius(mut self, use_radius: bool) -> GjkRunner<'a, ShapeA, ShapeB> {
            self.use_radius = use_radius;
            self
        }

        pub fn set_cache(mut self, cache: &'a mut GjkCache) -> GjkRunner<'a, ShapeA, ShapeB> {
            self.cache = Some(cache);
            self
        }

        pub fn set_iterations(mut self, iterations: &'a mut i32) -> GjkRunner<'a, ShapeA, ShapeB> {
            self.iterations = Some(iterations);
            self
        }

        /// Finds the closest pair between two shapes
        pub fn run(self) -> GjkResponse {
            let mut response = GjkResponse {
                distance: 0.0,
                closest_points: (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            };
            let cache_ptr = match self.cache {
                Some(cache) => cache as *mut GjkCache,
                None => std::ptr::null_mut(),
            };
            let iterations_ptr = match self.iterations {
                Some(iterations) => iterations as *mut i32,
                None => std::ptr::null_mut(),
            };
            unsafe {
                let distance = ffi::c2GJK(
                    self.a.shape(),
                    ShapeA::shape_type() as u32,
                    self.a.transformation(),
                    self.b.shape(),
                    ShapeB::shape_type() as u32,
                    self.b.transformation(),
                    &mut (response.closest_points.0).0,
                    &mut (response.closest_points.1).0,
                    self.use_radius as i32,
                    iterations_ptr,
                    cache_ptr,
                );
                response.distance = distance;
            }
            response
        }
    }

    impl<'a, ShapeA, ShapeB> ToiRunner<'a, ShapeA, ShapeB>
    where
        ShapeA: Shape,
        ShapeB: Shape,
    {
        pub fn new(a: &'a ShapeA, b: &'a ShapeB) -> ToiRunner<'a, ShapeA, ShapeB> {
            ToiRunner {
                a,
                b,
                a_velocity: Vec2::new(0.0, 0.0),
                b_velocity: Vec2::new(0.0, 0.0),
                use_radius: true,
                iterations: None,
            }
        }

        pub fn set_velocities(
            mut self,
            a_velocity: Vec2,
            b_velocity: Vec2,
        ) -> ToiRunner<'a, ShapeA, ShapeB> {
            self.a_velocity = a_velocity;
            self.b_velocity = b_velocity;
            self
        }

        pub fn use_radius(mut self, use_radius: bool) -> ToiRunner<'a, ShapeA, ShapeB> {
            self.use_radius = use_radius;
            self
        }

        pub fn set_iterations(mut self, iterations: &'a mut i32) -> ToiRunner<'a, ShapeA, ShapeB> {
            self.iterations = Some(iterations);
            self
        }

        pub fn run(self) -> f32 {
            let iterations_ptr = match self.iterations {
                Some(iterations) => iterations as *mut i32,
                None => std::ptr::null_mut(),
            };
            let result = unsafe {
                ffi::c2TOI(
                    self.a.shape(),
                    ShapeA::shape_type() as u32,
                    self.a.transformation(),
                    self.a_velocity.0,
                    self.b.shape(),
                    ShapeB::shape_type() as u32,
                    self.b.transformation(),
                    self.b_velocity.0,
                    self.use_radius as i32,
                    iterations_ptr,
                )
            };
            result
        }
    }

    pub trait AdvancedShape: Shape {
        fn gjk<'a, T: Shape>(&'a self, other: &'a T) -> GjkRunner<'a, Self, T>
        where
            Self: Sized,
            T: Sized,
        {
            GjkRunner::new(self, other)
        }

        fn time_of_impact<'a, T: Shape>(&'a self, other: &'a T) -> ToiRunner<'a, Self, T>
        where
            Self: Sized,
            T: Sized,
        {
            ToiRunner::new(self, other)
        }
    }

    impl Shape for Circle {
        fn shape_type() -> Type {
            Type::Circle
        }
    }

    impl BasicShape for Circle {}
    impl AdvancedShape for Circle {}

    impl Shape for AABB {
        fn shape_type() -> Type {
            Type::AABB
        }
    }

    impl BasicShape for AABB {}
    impl AdvancedShape for AABB {}

    impl Shape for Capsule {
        fn shape_type() -> Type {
            Type::Capsule
        }
    }

    impl BasicShape for Capsule {}
    impl AdvancedShape for Capsule {}

    impl Shape for Poly {
        fn shape_type() -> Type {
            Type::Poly
        }
    }

    impl BasicShape for Poly {}
    impl AdvancedShape for Poly {}

    impl Shape for (Circle, Transformation) {
        fn shape_type() -> Type {
            Type::Circle
        }

        fn shape(&self) -> *const c_void {
            &self.0 as *const _ as *const c_void
        }

        fn transformation(&self) -> *const ffi::c2x {
            &self.1 as *const _ as *const ffi::c2x
        }
    }

    impl AdvancedShape for (Circle, Transformation) {}

    impl Shape for (AABB, Transformation) {
        fn shape_type() -> Type {
            Type::AABB
        }

        fn shape(&self) -> *const c_void {
            &self.0 as *const _ as *const c_void
        }

        fn transformation(&self) -> *const ffi::c2x {
            &self.1 as *const _ as *const ffi::c2x
        }
    }

    impl AdvancedShape for (AABB, Transformation) {}

    impl Shape for (Capsule, Transformation) {
        fn shape_type() -> Type {
            Type::Capsule
        }

        fn shape(&self) -> *const c_void {
            &self.0 as *const _ as *const c_void
        }

        fn transformation(&self) -> *const ffi::c2x {
            &self.1 as *const _ as *const ffi::c2x
        }
    }

    impl AdvancedShape for (Capsule, Transformation) {}

    impl Shape for (Poly, Transformation) {
        fn shape_type() -> Type {
            Type::Poly
        }

        fn shape(&self) -> *const c_void {
            &self.0 as *const _ as *const c_void
        }

        fn transformation(&self) -> *const ffi::c2x {
            &self.1 as *const _ as *const ffi::c2x
        }
    }

    impl BasicShape for (Poly, Transformation) {}
    impl AdvancedShape for (Poly, Transformation) {}
}

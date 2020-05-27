//! # Getting started
//!
//! ```rust,no_run
//! use cute_c2::{self as c2, prelude::*};
//!
//! fn main() {
//!     let circle = c2::Circle::new(c2::Vec2::new(0.0, 0.0), 15.0);
//!     let aabb = c2::AABB {
//!         min: c2::Vec2::new(10.0, 5.0),
//!         max: c2::Vec2::new(20.0, 30.0),
//!     };
//!
//!     circle.collides_with(&aabb);
//!     // returns true
//!
//!     let capsule = c2::Capsule::new(c2::Vec2::new(5.0, 5.0), c2::Vec2::new(15.0, 10.0), 1.0);
//!
//!     let poly = c2::Poly::from_slice(&[
//!         c2::Vec2::new(-1.0, -3.0),
//!         c2::Vec2::new(1.0, -3.0),
//!         c2::Vec2::new(1.0, 0.0),
//!         c2::Vec2::new(0.0, 1.0),
//!         c2::Vec2::new(-1.0, 0.0),
//!     ]);
//!
//!     capsule.collides_with(&poly);
//!     // returns false
//!
//!     let transformation =
//!         c2::Transformation::new(c2::Vec2::new(5.0, 4.0), std::f32::consts::PI / 2.0);
//!
//!     circle.collides_with(&(poly, transformation));
//!     // returns true
//! }
//! ```

use cute_c2_sys as ffi;
use std::os::raw::c_void;

const MAX_POLYGON_VERTS: usize = ffi::C2_MAX_POLYGON_VERTS as usize;

pub type Vec2 = ffi::c2v;
pub type Rotation = ffi::c2r;
pub type Circle = ffi::c2Circle;
pub type AABB = ffi::c2AABB;
pub type Capsule = ffi::c2Capsule;
pub type Poly = ffi::c2Poly;
pub type Transformation = ffi::c2x;
pub type Ray = ffi::c2Ray;
pub type RayCast = ffi::c2Raycast;
pub type GjkCache = ffi::c2GJKCache;
pub type Manifold = ffi::c2Manifold;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Type {
    None = ffi::C2_TYPE_C2_TYPE_NONE,
    Circle = ffi::C2_TYPE_C2_TYPE_CIRCLE,
    AABB = ffi::C2_TYPE_C2_TYPE_AABB,
    Capsule = ffi::C2_TYPE_C2_TYPE_CAPSULE,
    Poly = ffi::C2_TYPE_C2_TYPE_POLY,
}

#[derive(Debug, Copy, Clone)]
pub struct GjkResponse {
    pub distance: f32,
    pub closest_points: (Vec2, Vec2),
}

pub struct GjkRunner<'a, ShapeA, ShapeB> {
    a: &'a ShapeA,
    b: &'a ShapeB,
    use_radius: bool,
    cache: Option<&'a mut GjkCache>,
    iterations: Option<&'a mut i32>,
}

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

    pub trait C2V {
        fn new(x: f32, y: f32) -> Self;
    }
    impl C2V for Vec2 {
        fn new(x: f32, y: f32) -> Vec2 {
            Vec2 { x, y }
        }
    }

    pub trait C2R {
        fn new(radians: f32) -> Self;
    }
    impl C2R for Rotation {
        fn new(radians: f32) -> Self {
            Rotation {
                c: radians.cos(),
                s: radians.sin(),
            }
        }
    }

    pub trait C2Circle {
        fn new(position: Vec2, radius: f32) -> Self;
    }
    impl C2Circle for Circle {
        fn new(position: Vec2, radius: f32) -> Circle {
            Circle {
                p: position,
                r: radius,
            }
        }
    }

    pub trait C2AABB {
        fn new(min: Vec2, max: Vec2) -> Self;
    }
    impl C2AABB for AABB {
        fn new(min: Vec2, max: Vec2) -> AABB {
            AABB { min, max }
        }
    }

    pub trait C2Capsule {
        fn new(a: Vec2, b: Vec2, radius: f32) -> Self;
    }
    impl C2Capsule for Capsule {
        fn new(a: Vec2, b: Vec2, radius: f32) -> Capsule {
            Capsule { a, b, r: radius }
        }
    }

    pub trait C2Poly {
        fn from_slice(verts: &[Vec2]) -> Self;
        fn from_array(count: usize, verts: [Vec2; 8]) -> Self;
    }
    impl C2Poly for Poly {
        fn from_slice(verts: &[Vec2]) -> Poly {
            let mut poly = Poly {
                count: verts.len() as i32,
                verts: [Vec2::new(0.0, 0.0); MAX_POLYGON_VERTS],
                norms: [Vec2::new(0.0, 0.0); MAX_POLYGON_VERTS],
            };
            for i in 0..verts.len().min(MAX_POLYGON_VERTS) {
                poly.verts[i] = verts[i];
            }
            unsafe {
                ffi::c2MakePoly(&mut poly);
            }
            poly
        }
        fn from_array(count: usize, verts: [Vec2; 8]) -> Poly {
            let mut poly = Poly {
                count: count.max(8) as i32,
                verts,
                norms: [Vec2::new(0.0, 0.0); MAX_POLYGON_VERTS],
            };
            unsafe {
                ffi::c2MakePoly(&mut poly);
            }
            poly
        }
    }

    pub trait C2X {
        fn new(position: Vec2, rotation: f32) -> Self;
    }
    impl C2X for Transformation {
        fn new(position: Vec2, rotation: f32) -> Transformation {
            Transformation {
                p: position,
                r: Rotation::new(rotation),
            }
        }
    }

    pub trait C2Ray {
        fn new(position: Vec2, ray: Vec2) -> Self;
    }

    impl C2Ray for Ray {
        fn new(position: Vec2, ray: Vec2) -> Ray {
            let distance = (ray.x * ray.x + ray.y * ray.y).sqrt();
            let direction = Vec2::new(ray.x / distance, ray.y / distance);
            Ray {
                p: position,
                d: direction,
                t: distance,
            }
        }
    }

    pub trait C2RayCast {
        fn time_of_impact(self) -> f32;
        fn position_of_impact(self, ray: Ray) -> Vec2;
        fn normal(self) -> Vec2;
    }

    impl C2RayCast for RayCast {
        fn time_of_impact(self) -> f32 {
            self.t
        }

        fn position_of_impact(self, ray: Ray) -> Vec2 {
            Vec2 {
                x: ray.p.x + ray.d.x * self.t,
                y: ray.p.y + ray.d.y * self.t,
            }
        }

        fn normal(self) -> Vec2 {
            self.n
        }
    }

    pub trait C2Manifold {
        fn normal(&self) -> Vec2;
    }

    impl C2Manifold for Manifold {
        fn normal(&self) -> Vec2 {
            self.n
        }
    }

    pub trait Shape {
        fn shape_type() -> Type;

        fn shape(&self) -> *const c_void {
            self as *const _ as *const c_void
        }

        fn transformation(&self) -> *const Transformation {
            std::ptr::null()
        }
    }

    pub trait BasicShape: Shape {
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

        fn manifold<T: Shape>(&self, other: &T) -> Manifold {
            let mut manifold = Manifold {
                count: 0,
                depths: [0.0, 0.0],
                contact_points: [ffi::c2v { x: 0.0, y: 0.0 }, ffi::c2v { x: 0.0, y: 0.0 }],
                n: ffi::c2v { x: 0.0, y: 0.0 },
            };
            unsafe {
                ffi::c2Collide(
                    self.shape(),
                    self.transformation(),
                    Self::shape_type() as u32,
                    other.shape(),
                    other.transformation(),
                    T::shape_type() as u32,
                    &mut manifold,
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
                    &mut response.closest_points.0,
                    &mut response.closest_points.1,
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
                    self.a_velocity,
                    self.b.shape(),
                    ShapeB::shape_type() as u32,
                    self.b.transformation(),
                    self.b_velocity,
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

        fn transformation(&self) -> *const Transformation {
            &self.1 as *const _ as *const Transformation
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

        fn transformation(&self) -> *const Transformation {
            &self.1 as *const _ as *const Transformation
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

        fn transformation(&self) -> *const Transformation {
            &self.1 as *const _ as *const Transformation
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

        fn transformation(&self) -> *const Transformation {
            &self.1 as *const _ as *const Transformation
        }
    }

    impl BasicShape for (Poly, Transformation) {}
    impl AdvancedShape for (Poly, Transformation) {}
}

/*
	------------------------------------------------------------------------------
		Licensing information can be found at the end of the file.
	------------------------------------------------------------------------------

	cute_c2.h - v1.08

	To create implementation (the function definitions)
		#define CUTE_C2_IMPLEMENTATION
	in *one* C/CPP file (translation unit) that includes this file


	SUMMARY

		cute_c2 is a single-file header that implements 2D collision detection routines
		that test for overlap, and optionally can find the collision manifold. The
		manifold contains all necessary information to prevent shapes from inter-
		penetrating, which is useful for character controllers, general physics
		simulation, and user-interface programming.

		This header implements a group of "immediate mode" functions that should be
		very easily adapted into pre-existing projects.


	THE IMPORTANT PARTS

		Most of the math types in this header are for internal use. Users care about
		the shape types and the collision functions.

		SHAPE TYPES:
		* c2Circle
		* c2Capsule
		* c2AABB
		* c2Ray
		* c2Poly

		COLLISION FUNCTIONS (*** is a shape name from the above list):
		* c2***to***         - boolean YES/NO hittest
		* c2***to***Manifold - construct manifold to describe how shapes hit
		* c2GJK              - runs GJK algorithm to find closest point pair between two shapes
		* c2TOI              - computes the time of impact between two shapes, useful for sweeping shapes, or doing shape casts
		* c2MakePoly         - Runs convex hull algorithm and computes normals on input point-set
		* c2Collided         - generic version of c2***to*** funcs
		* c2Collide          - generic version of c2***to***Manifold funcs
		* c2CastRay          - generic version of c2Rayto*** funcs

		The rest of the header is more or less for internal use. Here is an example of
		making some shapes and testing for collision:

			c2Circle c;
			c.p = position;
			c.r = radius;

			c2Capsule cap;
			cap.a = first_endpoint;
			cap.b = second_endpoint;
			cap.r = radius;

			int hit = c2CircletoCapsule(c, cap);
			if (hit)
			{
				handle collision here...
			}
	
		For more code examples and tests please see:
		https://github.com/RandyGaul/cute_header/tree/master/examples_cute_gl_and_c2

		Here is a past discussion thread on this header:
		https://www.reddit.com/r/gamedev/comments/5tqyey/tinyc2_2d_collision_detection_library_in_c/

		Here is a very nice repo containing various tests and examples using SFML for rendering:
		https://github.com/sro5h/tinyc2-tests


	FEATURES

		* Circles, capsules, AABBs, rays and convex polygons are supported
		* Fast boolean only result functions (hit yes/no)
		* Slghtly slower manifold generation for collision normals + depths +points
		* GJK implementation (finds closest points for disjoint pairs of shapes)
		* Shape casts/sweeps with c2TOI function (time of impact)
		* Robust 2D convex hull generator
		* Lots of correctly implemented and tested 2D math routines
		* Implemented in portable C, and is readily portable to other languages
		* Generic c2Collide, c2Collided and c2CastRay function (can pass in any shape type)
		* Extensive examples at: https://github.com/RandyGaul/cute_headers/tree/master/examples_cute_gl_and_c2


	Revision History
	
		1.0  (02/13/2017) initial release
		1.01 (02/13/2017) const crusade, minor optimizations, capsule degen
		1.02 (03/21/2017) compile fixes for c on more compilers
		1.03 (09/15/2017) various bugfixes and quality of life changes to manifolds
		1.04 (03/25/2018) fixed manifold bug in c2CircletoAABBManifold
		1.05 (11/01/2018) added c2TOI (time of impact) for shape cast/sweep test
		1.06 (08/23/2019) C2_*** types to C2_TYPE_***, and CUTE_C2_API
		1.07 (10/19/2019) Optimizations to c2TOI - breaking change to c2GJK API
		1.08 (12/22/2019) Remove contact point + normal from c2TOI, removed feather
		                  radius from c2GJK, fixed various bugs in capsule to poly
		                  manifold, did a pass on all docs


	Contributors

		Plastburk         1.01 - const pointers pull request
		mmozeiko          1.02 - 3 compile bugfixes
		felipefs          1.02 - 3 compile bugfixes
		seemk             1.02 - fix branching bug in c2Collide
		sro5h             1.02 - bug reports for multiple manifold funcs
		sro5h             1.03 - work involving quality of life fixes for manifolds
		Wizzard033        1.06 - C2_*** types to C2_TYPE_***, and CUTE_C2_API
		Tyler Glaeil      1.08 - Lots of bug reports and disussion on capsules + TOIs


	DETAILS/ADVICE

		BROAD PHASE

			This header does not implement a broad-phase, and instead concerns itself with
			the narrow-phase. This means this header just checks to see if two individual
			shapes are touching, and can give information about how they are touching.

			Very common 2D broad-phases are tree and grid approaches. Quad trees are good
			for static geometry that does not move much if at all. Dynamic AABB trees are
			good for general purpose use, and can handle moving objects very well. Grids
			are great and are similar to quad trees.

			If implementing a grid it can be wise to have each collideable grid cell hold
			an integer. This integer refers to a 2D shape that can be passed into the
			various functions in this header. The shape can be transformed from "model"
			space to "world" space using c2x -- a transform struct. In this way a grid
			can be implemented that holds any kind of convex shape (that this header
			supports) while conserving memory with shape instancing.

		NUMERIC ROBUSTNESS

			Many of the functions in cute c2 use `c2GJK`, an implementation of the GJK
			algorithm. Internally GJK computes signed area values, and these values are
			very numerically sensitive to large shapes. This means the GJK function will
			break down if input shapes are too large or too far away from the origin.

			In general it is best to compute collision detection on small shapes very
			close to the origin. One trick is to keep your collision information numerically
			very tiny, and simply scale it up when rendering to the appropriate size.

			For reference, if your shapes are all AABBs and contain a width and height
			of somewhere between 1.0f and 10.0f, everything will be fine. However, once
			your shapes start approaching a width/height of 100.0f to 1000.0f GJK can
			start breaking down.

			This is a complicated topic, so feel free to ask the author for advice here.

			Here is an example demonstrating this problem with two large AABBs:
			https://github.com/RandyGaul/cute_headers/issues/160

		Please email at my address with any questions or comments at:
		author's last name followed by 1748 at gmail
*/

#if !defined(CUTE_C2_H)

// this can be adjusted as necessary, but is highly recommended to be kept at 8.
// higher numbers will incur quite a bit of memory overhead, and convex shapes
// over 8 verts start to just look like spheres, which can be implicitly rep-
// resented as a point + radius. usually tools that generate polygons should be
// constructed so they do not output polygons with too many verts.
// Note: polygons in cute_c2 are all *convex*.
#define C2_MAX_POLYGON_VERTS 8

// 2d vector
typedef struct c2v
{
	float x;
	float y;
} c2v;

// 2d rotation composed of cos/sin pair
typedef struct c2r
{
	float c;
	float s;
} c2r;

// 2d rotation matrix
typedef struct c2m
{
	c2v x;
	c2v y;
} c2m;

// 2d transformation "x"
// These are used especially for c2Poly when a c2Poly is passed to a function.
// Since polygons are prime for "instancing" a c2x transform can be used to
// transform a polygon from local space to world space. In functions that take
// a c2x pointer (like c2PolytoPoly), these pointers can be NULL, which represents
// an identity transformation and assumes the verts inside of c2Poly are already
// in world space.
typedef struct c2x
{
	c2v p;
	c2r r;
} c2x;

// 2d halfspace (aka plane, aka line)
typedef struct c2h
{
	c2v n;   // normal, normalized
	float d; // distance to origin from plane, or ax + by = d
} c2h;

typedef struct c2Circle
{
	c2v p;
	float r;
} c2Circle;

typedef struct c2AABB
{
	c2v min;
	c2v max;
} c2AABB;

// a capsule is defined as a line segment (from a to b) and radius r
typedef struct c2Capsule
{
	c2v a;
	c2v b;
	float r;
} c2Capsule;

typedef struct c2Poly
{
	int count;
	c2v verts[C2_MAX_POLYGON_VERTS];
	c2v norms[C2_MAX_POLYGON_VERTS];
} c2Poly;

// IMPORTANT:
// Many algorithms in this file are sensitive to the magnitude of the
// ray direction (c2Ray::d). It is highly recommended to normalize the
// ray direction and use t to specify a distance. Please see this link
// for an in-depth explanation: https://github.com/RandyGaul/cute_headers/issues/30
typedef struct c2Ray
{
	c2v p;   // position
	c2v d;   // direction (normalized)
	float t; // distance along d from position p to find endpoint of ray
} c2Ray;

typedef struct c2Raycast
{
	float t; // time of impact
	c2v n;   // normal of surface at impact (unit length)
} c2Raycast;

// position of impact p = ray.p + ray.d * raycast.t
#define c2Impact(ray, t) c2Add(ray.p, c2Mulvs(ray.d, t))

// contains all information necessary to resolve a collision, or in other words
// this is the information needed to separate shapes that are colliding. Doing
// the resolution step is *not* included in cute_c2.
typedef struct c2Manifold
{
	int count;
	float depths[2];
	c2v contact_points[2];

	// always points from shape A to shape B (first and second shapes passed into
	// any of the c2***to***Manifold functions)
	c2v n;
} c2Manifold;

// This define allows exporting/importing of the header to a dynamic library.
// Here's an example.
// #define CUTE_C2_API extern "C" __declspec(dllexport)
#if !defined(CUTE_C2_API)
#	define CUTE_C2_API
#endif

// boolean collision detection
// these versions are faster than the manifold versions, but only give a YES/NO result
CUTE_C2_API int c2CircletoCircle(c2Circle A, c2Circle B);
CUTE_C2_API int c2CircletoAABB(c2Circle A, c2AABB B);
CUTE_C2_API int c2CircletoCapsule(c2Circle A, c2Capsule B);
CUTE_C2_API int c2AABBtoAABB(c2AABB A, c2AABB B);
CUTE_C2_API int c2AABBtoCapsule(c2AABB A, c2Capsule B);
CUTE_C2_API int c2CapsuletoCapsule(c2Capsule A, c2Capsule B);
CUTE_C2_API int c2CircletoPoly(c2Circle A, const c2Poly* B, const c2x* bx);
CUTE_C2_API int c2AABBtoPoly(c2AABB A, const c2Poly* B, const c2x* bx);
CUTE_C2_API int c2CapsuletoPoly(c2Capsule A, const c2Poly* B, const c2x* bx);
CUTE_C2_API int c2PolytoPoly(const c2Poly* A, const c2x* ax, const c2Poly* B, const c2x* bx);

// ray operations
// output is placed into the c2Raycast struct, which represents the hit location
// of the ray. the out param contains no meaningful information if these funcs
// return 0
CUTE_C2_API int c2RaytoCircle(c2Ray A, c2Circle B, c2Raycast* out);
CUTE_C2_API int c2RaytoAABB(c2Ray A, c2AABB B, c2Raycast* out);
CUTE_C2_API int c2RaytoCapsule(c2Ray A, c2Capsule B, c2Raycast* out);
CUTE_C2_API int c2RaytoPoly(c2Ray A, const c2Poly* B, const c2x* bx_ptr, c2Raycast* out);

// manifold generation
// these functions are (generally) slower than the boolean versions, but will compute one
// or two points that represent the plane of contact. This information is
// is usually needed to resolve and prevent shapes from colliding. If no coll
// ision occured the count member of the manifold struct is set to 0.
CUTE_C2_API void c2CircletoCircleManifold(c2Circle A, c2Circle B, c2Manifold* m);
CUTE_C2_API void c2CircletoAABBManifold(c2Circle A, c2AABB B, c2Manifold* m);
CUTE_C2_API void c2CircletoCapsuleManifold(c2Circle A, c2Capsule B, c2Manifold* m);
CUTE_C2_API void c2AABBtoAABBManifold(c2AABB A, c2AABB B, c2Manifold* m);
CUTE_C2_API void c2AABBtoCapsuleManifold(c2AABB A, c2Capsule B, c2Manifold* m);
CUTE_C2_API void c2CapsuletoCapsuleManifold(c2Capsule A, c2Capsule B, c2Manifold* m);
CUTE_C2_API void c2CircletoPolyManifold(c2Circle A, const c2Poly* B, const c2x* bx, c2Manifold* m);
CUTE_C2_API void c2AABBtoPolyManifold(c2AABB A, const c2Poly* B, const c2x* bx, c2Manifold* m);
CUTE_C2_API void c2CapsuletoPolyManifold(c2Capsule A, const c2Poly* B, const c2x* bx, c2Manifold* m);
CUTE_C2_API void c2PolytoPolyManifold(const c2Poly* A, const c2x* ax, const c2Poly* B, const c2x* bx, c2Manifold* m);

typedef enum
{
	C2_TYPE_NONE,
	C2_TYPE_CIRCLE,
	C2_TYPE_AABB,
	C2_TYPE_CAPSULE,
	C2_TYPE_POLY
} C2_TYPE;

// This struct is only for advanced usage of the c2GJK function. See comments inside of the
// c2GJK function for more details.
typedef struct c2GJKCache
{
	float metric;
	int count;
	int iA[3];
	int iB[3];
	float div;
} c2GJKCache;

// This is an advanced function, intended to be used by people who know what they're doing.
//
// Runs the GJK algorithm to find closest points, returns distance between closest points.
// outA and outB can be NULL, in this case only distance is returned. ax_ptr and bx_ptr
// can be NULL, and represent local to world transformations for shapes A and B respectively.
// use_radius will apply radii for capsules and circles (if set to false, spheres are
// treated as points and capsules are treated as line segments i.e. rays). The cache parameter
// should be NULL, as it is only for advanced usage (unless you know what you're doing, then
// go ahead and use it). iterations is an optional parameter.
//
// IMPORTANT NOTE:
// The GJK function is sensitive to large shapes, since it internally will compute signed area
// values. `c2GJK` is called throughout cute c2 in many ways, so try to make sure all of your
// collision shapes are not gigantic. For example, try to keep the volume of all your shapes
// less than 100.0f. If you need large shapes, you should use tiny collision geometry for all
// cute c2 function, and simply render the geometry larger on-screen by scaling it up.
CUTE_C2_API float c2GJK(const void* A, C2_TYPE typeA, const c2x* ax_ptr, const void* B, C2_TYPE typeB, const c2x* bx_ptr, c2v* outA, c2v* outB, int use_radius, int* iterations, c2GJKCache* cache);

// This is an advanced function, intended to be used by people who know what they're doing.
//
// Computes the time of impact from shape A and shape B. The velocity of each shape is provided
// by vA and vB respectively. The shapes are *not* allowed to rotate over time. The velocity is
// assumed to represent the change in motion from time 0 to time 1, and so the return value will
// be a number from 0 to 1. To move each shape to the colliding configuration, multiply vA and vB
// each by the return value. ax_ptr and bx_ptr are optional parameters to transforms for each shape,
// and are typically used for polygon shapes to transform from model to world space. Set these to
// NULL to represent identity transforms. iterations is an optional parameter. use_radius
// will apply radii for capsules and circles (if set to false, spheres are treated as points and
// capsules are treated as line segments i.e. rays).
//
// IMPORTANT NOTE:
// The c2TOI function can be used to implement a "swept character controller", but it can be
// difficult to do so. Say we compute a time of impact with `c2TOI` and move the shapes to the
// time of impact, and adjust the velocity by zeroing out the velocity along the surface normal.
// If we then call `c2TOI` again, it will fail since the shapes will be considered to start in
// a colliding configuration. There are many styles of tricks to get around this problem, and
// all of them involve giving the next call to `c2TOI` some breathing room. It is recommended
// to use some variation of the following algorithm:
//
// 1. Call c2TOI.
// 2. Move the shapes to the TOI.
// 3. Slightly inflate the size of one, or both, of the shapes so they will be intersecting.
//    The purpose is to make the shapes numerically intersecting, but not visually intersecting.
// 4. Compute the collision manifold between the inflated shapes (for example, use c2PolytoPolyManifold).
// 5. Gently push the shapes apart. This will give the next call to c2TOI some breathing room.
CUTE_C2_API float c2TOI(const void* A, C2_TYPE typeA, const c2x* ax_ptr, c2v vA, const void* B, C2_TYPE typeB, const c2x* bx_ptr, c2v vB, int use_radius, int* iterations);

// Computes 2D convex hull. Will not do anything if less than two verts supplied. If
// more than C2_MAX_POLYGON_VERTS are supplied extras are ignored.
CUTE_C2_API int c2Hull(c2v* verts, int count);
CUTE_C2_API void c2Norms(c2v* verts, c2v* norms, int count);

// runs c2Hull and c2Norms, assumes p->verts and p->count are both set to valid values
CUTE_C2_API void c2MakePoly(c2Poly* p);

// Generic collision detection routines, useful for games that want to use some poly-
// morphism to write more generic-styled code. Internally calls various above functions.
// For AABBs/Circles/Capsules ax and bx are ignored. For polys ax and bx can define
// model to world transformations, or be NULL for identity transforms.
CUTE_C2_API int c2Collided(const void* A, const c2x* ax, C2_TYPE typeA, const void* B, const c2x* bx, C2_TYPE typeB);
CUTE_C2_API void c2Collide(const void* A, const c2x* ax, C2_TYPE typeA, const void* B, const c2x* bx, C2_TYPE typeB, c2Manifold* m);
CUTE_C2_API int c2CastRay(c2Ray A, const void* B, const c2x* bx, C2_TYPE typeB, c2Raycast* out);

#ifdef _MSC_VER
	#define C2_INLINE __forceinline
#else
	#define C2_INLINE inline __attribute__((always_inline))
#endif

// adjust these primitives as seen fit
#include <string.h> // memcpy
#include <math.h>
#define c2Sin(radians) sinf(radians)
#define c2Cos(radians) cosf(radians)
#define c2Sqrt(a) sqrtf(a)
#define c2Min(a, b) ((a) < (b) ? (a) : (b))
#define c2Max(a, b) ((a) > (b) ? (a) : (b))
#define c2Abs(a) ((a) < 0 ? -(a) : (a))
#define c2Clamp(a, lo, hi) c2Max(lo, c2Min(a, hi))
C2_INLINE void c2SinCos(float radians, float* s, float* c) { *c = c2Cos(radians); *s = c2Sin(radians); }
#define c2Sign(a) (a < 0 ? -1.0f : 1.0f)

// The rest of the functions in the header-only portion are all for internal use
// and use the author's personal naming conventions. It is recommended to use one's
// own math library instead of the one embedded here in cute_c2, but for those
// curious or interested in trying it out here's the details:

// The Mul functions are used to perform multiplication. x stands for transform,
// v stands for vector, s stands for scalar, r stands for rotation, h stands for
// halfspace and T stands for transpose.For example c2MulxvT stands for "multiply
// a transform with a vector, and transpose the transform".

// vector ops
C2_INLINE c2v c2V(float x, float y) { c2v a; a.x = x; a.y = y; return a; }
C2_INLINE c2v c2Add(c2v a, c2v b) { a.x += b.x; a.y += b.y; return a; }
C2_INLINE c2v c2Sub(c2v a, c2v b) { a.x -= b.x; a.y -= b.y; return a; }
C2_INLINE float c2Dot(c2v a, c2v b) { return a.x * b.x + a.y * b.y; }
C2_INLINE c2v c2Mulvs(c2v a, float b) { a.x *= b; a.y *= b; return a; }
C2_INLINE c2v c2Mulvv(c2v a, c2v b) { a.x *= b.x; a.y *= b.y; return a; }
C2_INLINE c2v c2Div(c2v a, float b) { return c2Mulvs(a, 1.0f / b); }
C2_INLINE c2v c2Skew(c2v a) { c2v b; b.x = -a.y; b.y = a.x; return b; }
C2_INLINE c2v c2CCW90(c2v a) { c2v b; b.x = a.y; b.y = -a.x; return b; }
C2_INLINE float c2Det2(c2v a, c2v b) { return a.x * b.y - a.y * b.x; }
C2_INLINE c2v c2Minv(c2v a, c2v b) { return c2V(c2Min(a.x, b.x), c2Min(a.y, b.y)); }
C2_INLINE c2v c2Maxv(c2v a, c2v b) { return c2V(c2Max(a.x, b.x), c2Max(a.y, b.y)); }
C2_INLINE c2v c2Clampv(c2v a, c2v lo, c2v hi) { return c2Maxv(lo, c2Minv(a, hi)); }
C2_INLINE c2v c2Absv(c2v a) { return c2V(c2Abs(a.x), c2Abs(a.y)); }
C2_INLINE float c2Hmin(c2v a) { return c2Min(a.x, a.y); }
C2_INLINE float c2Hmax(c2v a) { return c2Max(a.x, a.y); }
C2_INLINE float c2Len(c2v a) { return c2Sqrt(c2Dot(a, a)); }
C2_INLINE c2v c2Norm(c2v a) { return c2Div(a, c2Len(a)); }
C2_INLINE c2v c2SafeNorm(c2v a) { float sq = c2Dot(a, a); return sq ? c2Div(a, c2Len(a)) : c2V(0, 0); }
C2_INLINE c2v c2Neg(c2v a) { return c2V(-a.x, -a.y); }
C2_INLINE c2v c2Lerp(c2v a, c2v b, float t) { return c2Add(a, c2Mulvs(c2Sub(b, a), t)); }
C2_INLINE int c2Parallel(c2v a, c2v b, float kTol)
{
	float k = c2Len(a) / c2Len(b);
	b = c2Mulvs(b, k);
	if (c2Abs(a.x - b.x) < kTol && c2Abs(a.y - b.y) < kTol) return 1;
	return 0;
}

// rotation ops
C2_INLINE c2r c2Rot(float radians) { c2r r; c2SinCos(radians, &r.s, &r.c); return r; }
C2_INLINE c2r c2RotIdentity() { c2r r; r.c = 1.0f; r.s = 0; return r; }
C2_INLINE c2v c2RotX(c2r r) { return c2V(r.c, r.s); }
C2_INLINE c2v c2RotY(c2r r) { return c2V(-r.s, r.c); }
C2_INLINE c2v c2Mulrv(c2r a, c2v b)  { return c2V(a.c * b.x - a.s * b.y,  a.s * b.x + a.c * b.y); }
C2_INLINE c2v c2MulrvT(c2r a, c2v b) { return c2V(a.c * b.x + a.s * b.y, -a.s * b.x + a.c * b.y); }
C2_INLINE c2r c2Mulrr(c2r a, c2r b)  { c2r c; c.c = a.c * b.c - a.s * b.s; c.s = a.s * b.c + a.c * b.s; return c; }
C2_INLINE c2r c2MulrrT(c2r a, c2r b) { c2r c; c.c = a.c * b.c + a.s * b.s; c.s = a.c * b.s - a.s * b.c; return c; }

C2_INLINE c2v c2Mulmv(c2m a, c2v b) { c2v c; c.x = a.x.x * b.x + a.y.x * b.y; c.y = a.x.y * b.x + a.y.y * b.y; return c; }
C2_INLINE c2v c2MulmvT(c2m a, c2v b) { c2v c; c.x = a.x.x * b.x + a.x.y * b.y; c.y = a.y.x * b.x + a.y.y * b.y; return c; }
C2_INLINE c2m c2Mulmm(c2m a, c2m b)  { c2m c; c.x = c2Mulmv(a, b.x);  c.y = c2Mulmv(a, b.y); return c; }
C2_INLINE c2m c2MulmmT(c2m a, c2m b) { c2m c; c.x = c2MulmvT(a, b.x); c.y = c2MulmvT(a, b.y); return c; }

// transform ops
C2_INLINE c2x c2xIdentity() { c2x x; x.p = c2V(0, 0); x.r = c2RotIdentity(); return x; }
C2_INLINE c2v c2Mulxv(c2x a, c2v b) { return c2Add(c2Mulrv(a.r, b), a.p); }
C2_INLINE c2v c2MulxvT(c2x a, c2v b) { return c2MulrvT(a.r, c2Sub(b, a.p)); }
C2_INLINE c2x c2Mulxx(c2x a, c2x b) { c2x c; c.r = c2Mulrr(a.r, b.r); c.p = c2Add(c2Mulrv(a.r, b.p), a.p); return c; }
C2_INLINE c2x c2MulxxT(c2x a, c2x b) { c2x c; c.r = c2MulrrT(a.r, b.r); c.p = c2MulrvT(a.r, c2Sub(b.p, a.p)); return c; }
C2_INLINE c2x c2Transform(c2v p, float radians) { c2x x; x.r = c2Rot(radians); x.p = p; return x; }

// halfspace ops
C2_INLINE c2v c2Origin(c2h h) { return c2Mulvs(h.n, h.d); }
C2_INLINE float c2Dist(c2h h, c2v p) { return c2Dot(h.n, p) - h.d; }
C2_INLINE c2v c2Project(c2h h, c2v p) { return c2Sub(p, c2Mulvs(h.n, c2Dist(h, p))); }
C2_INLINE c2h c2Mulxh(c2x a, c2h b) { c2h c; c.n = c2Mulrv(a.r, b.n); c.d = c2Dot(c2Mulxv(a, c2Origin(b)), c.n); return c; }
C2_INLINE c2h c2MulxhT(c2x a, c2h b) { c2h c; c.n = c2MulrvT(a.r, b.n); c.d = c2Dot(c2MulxvT(a, c2Origin(b)), c.n); return c; }
C2_INLINE c2v c2Intersect(c2v a, c2v b, float da, float db) { return c2Add(a, c2Mulvs(c2Sub(b, a), (da / (da - db)))); }

C2_INLINE void c2BBVerts(c2v* out, c2AABB* bb)
{
	out[0] = bb->min;
	out[1] = c2V(bb->max.x, bb->min.y);
	out[2] = bb->max;
	out[3] = c2V(bb->min.x, bb->max.y);
}

#define CUTE_C2_H
#endif



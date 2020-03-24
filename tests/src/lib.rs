#[cfg(test)]
mod tests {
    use cute_c2 as c2;
    use cute_c2::detection::*;

    fn circle() -> c2::Circle {
        c2::Circle::new(c2::Vec2::new(-1.0, 0.0), 2.0)
    }
    fn aabb() -> c2::AABB {
        c2::AABB::new(c2::Vec2::new(-1.0, -2.0), c2::Vec2::new(3.0, 2.0))
    }
    fn capsule() -> c2::Capsule {
        c2::Capsule::new(c2::Vec2::new(-2.0, 0.0), c2::Vec2::new(0.0, 0.0), 1.0)
    }
    fn poly() -> c2::Poly {
        c2::Poly::from_slice(&[
            c2::Vec2::new(-1.0, -3.0),
            c2::Vec2::new(1.0, -3.0),
            c2::Vec2::new(1.0, 0.0),
            c2::Vec2::new(0.0, 1.0),
            c2::Vec2::new(-1.0, 0.0),
        ])
    }

    #[test]
    fn circle_to_circle() {
        let other = c2::Circle::new(c2::Vec2::new(1.0, 0.0), 2.0);

        assert!(circle().collides_with(&other));
    }

    #[test]
    fn not_circle_to_circle() {
        let other = c2::Circle::new(c2::Vec2::new(6.0, 1.0), 2.0);

        assert!(!circle().collides_with(&other));
    }

    #[test]
    fn circle_to_aabb() {
        assert!(circle().collides_with(&aabb()));
    }

    #[test]
    fn circle_to_capsule() {
        assert!(circle().collides_with(&capsule()));
    }

    #[test]
    fn circle_to_poly() {
        assert!(circle().collides_with(&poly()));
    }

    #[test]
    fn aabb_to_aabb() {
        let other = c2::AABB::new(c2::Vec2::new(-5.0, -2.0), c2::Vec2::new(1.0, 2.0));

        assert!(aabb().collides_with(&other));
    }

    #[test]
    fn not_aabb_to_aabb() {
        let other = c2::AABB::new(c2::Vec2::new(8.0, 8.0), c2::Vec2::new(12.0, 15.0));

        assert!(!aabb().collides_with(&other));
    }

    #[test]
    fn aabb_to_capsule() {
        assert!(aabb().collides_with(&capsule()));
    }

    #[test]
    fn aabb_to_poly() {
        assert!(aabb().collides_with(&poly()));
    }

    #[test]
    fn capsule_to_capsule() {
        let other = c2::Capsule::new(c2::Vec2::new(-4.0, 0.0), c2::Vec2::new(2.0, 0.0), 1.0);

        assert!(capsule().collides_with(&other));
    }

    #[test]
    fn not_capsule_to_capsule() {
        let other = c2::Capsule::new(c2::Vec2::new(-4.0, 10.0), c2::Vec2::new(2.0, 20.0), 1.0);

        assert!(!capsule().collides_with(&other));
    }

    #[test]
    fn capsule_to_poly() {
        assert!(capsule().collides_with(&poly()));
    }

    #[test]
    fn poly_to_poly() {
        let other = c2::Poly::from_slice(&[
            c2::Vec2::new(-10.0, -10.0),
            c2::Vec2::new(5.0, -9.0),
            c2::Vec2::new(10.0, 5.0),
            c2::Vec2::new(-4.0, 9.0),
        ]);
        assert!(poly().collides_with(&other));
    }

    #[test]
    fn not_poly_to_poly() {
        let other = c2::Poly::from_slice(&[
            c2::Vec2::new(-10.0, -10.0),
            c2::Vec2::new(-17.0, -10.0),
            c2::Vec2::new(-10.0, -15.0),
        ]);
        assert!(!poly().collides_with(&other));
    }

    #[test]
    fn gjk() {
        let other = c2::Poly::from_slice(&[
            c2::Vec2::new(-10.0, -10.0),
            c2::Vec2::new(-17.0, -10.0),
            c2::Vec2::new(-10.0, -15.0),
        ]);

        let mut iterations = 0;
        let r = poly().gjk(&other).set_iterations(&mut iterations).run();

        assert!(r.distance != 0.0);
        assert!(iterations > 0);
    }
}

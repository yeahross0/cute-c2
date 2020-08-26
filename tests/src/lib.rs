#[cfg(test)]
mod tests {
    use c2::prelude::*;

    fn circle() -> c2::Circle {
        c2::Circle::new([-1.0, 0.0], 2.0)
    }
    fn aabb() -> c2::AABB {
        c2::AABB::new([-1.0, -2.0], [3.0, 2.0])
    }
    fn capsule() -> c2::Capsule {
        c2::Capsule::new([-2.0, 0.0], [0.0, 0.0], 1.0)
    }
    fn poly() -> c2::Poly {
        c2::Poly::from_slice(&[
            [-1.0, -3.0],
            [1.0, -3.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [-1.0, 0.0],
        ])
    }

    #[test]
    fn circle_to_circle() {
        let other = c2::Circle::new([1.0, 0.0], 2.0);

        assert!(circle().collides_with(&other));
    }

    #[test]
    fn not_circle_to_circle() {
        let other = c2::Circle::new([6.0, 1.0], 2.0);

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
        let other = c2::AABB::new([-5.0, -2.0], [1.0, 2.0]);

        assert!(aabb().collides_with(&other));
    }

    #[test]
    fn not_aabb_to_aabb() {
        let other = c2::AABB::new([8.0, 8.0], [12.0, 15.0]);

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
        let other = c2::Capsule::new([-4.0, 0.0], [2.0, 0.0], 1.0);

        assert!(capsule().collides_with(&other));
    }

    #[test]
    fn not_capsule_to_capsule() {
        let other = c2::Capsule::new([-4.0, 10.0], [2.0, 20.0], 1.0);

        assert!(!capsule().collides_with(&other));
    }

    #[test]
    fn capsule_to_poly() {
        assert!(capsule().collides_with(&poly()));
    }

    #[test]
    fn poly_to_poly() {
        let other = c2::Poly::from_slice(&[[-10.0, -10.0], [5.0, -9.0], [10.0, 5.0], [-4.0, 9.0]]);
        assert!(poly().collides_with(&other));
    }

    #[test]
    fn not_poly_to_poly() {
        let other = c2::Poly::from_slice(&[[-10.0, -10.0], [-17.0, -10.0], [-10.0, -15.0]]);
        assert!(!poly().collides_with(&other));
    }

    #[test]
    fn gjk() {
        let other = c2::Poly::from_slice(&[[-10.0, -10.0], [-17.0, -10.0], [-10.0, -15.0]]);

        let mut iterations = 0;
        let r = poly().gjk(&other).set_iterations(&mut iterations).run();

        assert!(r.distance != 0.0);
        assert!(iterations > 0);
    }
}

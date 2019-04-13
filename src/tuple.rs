use float_cmp::*;

#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.x.approx_eq_ulps(&other.x, 100000000)
            && self.y.approx_eq_ulps(&other.y, 100000000)
            && self.z.approx_eq_ulps(&other.z, 100000000)
            && self.w.approx_eq_ulps(&other.w, 100000000)
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        !self.is_point()
    }

    pub fn add(&self, other: &Tuple) -> Tuple {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }

    pub fn sub(&self, other: &Tuple) -> Tuple {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }

    pub fn negate(&self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }

    pub fn mult(&self, factor: f64) -> Tuple {
        Tuple::new(
            self.x * factor,
            self.y * factor,
            self.z * factor,
            self.w * factor,
        )
    }

    pub fn div(&self, divisor: f64) -> Tuple {
        Tuple::new(
            self.x / divisor,
            self.y / divisor,
            self.z / divisor,
            self.w / divisor,
        )
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let m = self.magnitude();
        self.div(m)
    }

    pub fn dot_product(&self, other: &Tuple) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    pub fn cross_product(&self, other: &Tuple) -> Tuple {
        Tuple::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_tuple_with_w_1_0_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1.add(&a2), Tuple::new_point(1.0, 1.0, 6.0))
    }

    #[test]
    fn subtracting_two_points() {
        let a1 = Tuple::new_point(3.0, 2.0, 1.0);
        let a2 = Tuple::new_point(5.0, 6.0, 7.0);
        assert_eq!(a1.sub(&a2), Tuple::new_vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_vector_from_a_point() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);

        assert_eq!(p.sub(&v), Tuple::new_point(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);

        assert_eq!(v1.sub(&v2), Tuple::new_vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        let v = Tuple::new_vector(1.0, -2.0, 3.0);

        assert_eq!(zero.sub(&v), Tuple::new_vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_vector() {
        let v = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(v.negate(), Tuple::new(-1.0, 2.0, -3.0, 4.0))
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a.mult(3.5), Tuple::new(3.5, -7.0, 10.5, -14.0))
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a.mult(0.5), Tuple::new(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a.div(2.0), Tuple::new(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_0_0() {
        let v = Tuple::new_vector(1.0, 0.0, 0.0);

        assert_eq!(v.magnitude(), 1.0)
    }

    #[test]
    fn normalizing_a_vector() {
        let v = Tuple::new_vector(4.0, 0.0, 0.0);

        assert_eq!(v.normalize(), Tuple::new_vector(1.0, 0.0, 0.0))
    }

    #[test]
    fn normalizing_another_vector() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);

        assert_eq!(
            v.normalize(),
            Tuple::new_vector(0.267261241, 0.534522483, 0.801783725)
        )
    }

    #[test]
    fn magnitude_of_a_normalized_vector_is_1() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);

        assert_eq!(v.normalize().magnitude(), 1.0)
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(a.dot_product(&b), 20.0)
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross_product(&b), Tuple::new_vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross_product(&a), Tuple::new_vector(1.0, -2.0, 1.0));
    }
}

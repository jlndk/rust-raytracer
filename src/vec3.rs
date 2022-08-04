use glam::Vec3;
use rand::{thread_rng, Rng};

pub trait Vec3Extension {
    fn rand() -> Vec3;
    fn rand_range(min: f32, max: f32) -> Vec3;
    fn random_in_unit_sphere() -> Vec3;
    fn random_unit_vector() -> Vec3;
    fn is_near_zero(&self) -> bool;
    fn reflect_in(self, n: Vec3) -> Vec3;
    fn refract_off(self, n: Vec3, etai_over_etat: f32) -> Vec3;
}

impl Vec3Extension for Vec3 {
    fn rand_range(min: f32, max: f32) -> Vec3 {
        let mut rng = thread_rng();

        return Vec3::new(rng.gen_range(min..=max), rng.gen_range(min..=max), rng.gen_range(min..=max));
    }

    fn rand() -> Vec3 {
        return Vec3::rand_range(0.0, 1.0);
    }

    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::rand_range(-1.0, 1.0);

            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    fn random_unit_vector() -> Vec3 {
        return Vec3::random_in_unit_sphere().normalize();
    }

    // Return true if the vector is close to zero in all dimensions.
    fn is_near_zero(&self) -> bool {
        let s = 0.00000001;
        return (self.x < s) && (self.y < s) && (self.z < s);
    }

    fn reflect_in(self, n: Vec3) -> Vec3 {
        return self - 2.0 * self.dot(n) * n;
    }

    fn refract_off(self, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.0);

        let r_out_perp =  etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = (-(1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;

        return r_out_perp + r_out_parallel;
    }
}

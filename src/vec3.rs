use glam::Vec3;
use rand::{thread_rng, Rng};

pub trait RandomVec3 {
    fn rand() -> Vec3;
    fn rand_range(min: f32, max: f32) -> Vec3;
    fn random_in_unit_sphere() -> Vec3;
    fn random_unit_vector() -> Vec3;
}

impl RandomVec3 for Vec3 {
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
}

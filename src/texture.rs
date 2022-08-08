use glam::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, point: &Vec3) -> Vec3;
}

pub struct SolidColor {
    color_value: Vec3,
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _point: &Vec3) -> Vec3 {
        return self.color_value;
    }
}

impl SolidColor {
    pub const fn new(color_value: Vec3) -> Self {
        return Self { color_value };
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> Self {
        return Self { odd, even };
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, point: &Vec3) -> Vec3 {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, point);
        } else {
            return self.even.value(u, v, point);
        }
    }
}

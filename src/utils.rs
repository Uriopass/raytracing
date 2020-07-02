use ultraviolet::Vec3;

pub fn random_in_sphere() -> Vec3 {
    loop {
        let p = Vec3::from(rand::random::<[f32; 3]>());
        if p.mag_sq() <= 1.0 {
            return p;
        }
    }
}

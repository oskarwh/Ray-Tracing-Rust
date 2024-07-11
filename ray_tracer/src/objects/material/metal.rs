use crate::{vectors::{vec3::{Color, Vec3, reflect, dot, random_in_unit_sphere}, ray::Ray}, objects::hit_record::HitRecord};

use super::material::Material;

#[derive(Copy, Clone)]
pub struct Metal
{
    pub albedo: Color,
    pub fuzz: f32
}

impl Metal
{
    pub fn new(a: Color, f: f32) -> Metal
    {
        let fuzz = if f < 1.0 {
            f
        }else {
            1.0
        };

        Metal
        {
            albedo: a,
            fuzz: fuzz
        }
    }
}

impl Material for Metal
{
    fn scatter(&self, 
        r_in: &Ray, 
        rec: &HitRecord, 
        attenuation: &mut Color, 
        scattered: &mut Ray) -> bool 
    {
        let reflected: Vec3 = reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + random_in_unit_sphere().const_mul(self.fuzz), r_in.time());
        *attenuation = self.albedo;
        // Return
        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
use crate::{vectors::{vec3::{Color, random_unit_vector}, ray::Ray}, objects::hit_record::HitRecord};

use super::material::Material;

#[derive(Copy, Clone)]

pub struct Lambertian
{
    pub albedo: Color
}

impl Default for Lambertian
{
    fn default() -> Self {
        Lambertian { 
            albedo: Color::default()
         }
    }
}

impl Lambertian
{
    pub fn new(a: Color) -> Lambertian
    {
        Lambertian
        {
            albedo: a
        }
    }
}

impl Material for Lambertian
{
    fn scatter(&self, 
        r_in: &Ray, 
        rec: &HitRecord, 
        attenuation: &mut Color, 
        scattered: &mut Ray) -> bool 
    {
        let mut scatter_driection = rec.normal + random_unit_vector();   

        // Catch degenerate scatter direction
        if scatter_driection.near_zero()
        {
            scatter_driection = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_driection, r_in.time());
        *attenuation = self.albedo;
        return true
    }
}
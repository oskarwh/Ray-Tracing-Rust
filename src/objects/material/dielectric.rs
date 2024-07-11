use crate::{vectors::{ray::Ray, vec3::{Color, refract, dot, reflect, Vec3}}, objects::hit_record::HitRecord, utility::rtweekend::random_number};

use super::material::Material;

#[derive(Copy, Clone)]

pub struct Dielectric
{
    pub ir: f32
}

impl Default for Dielectric
{
    fn default() -> Self {
        Dielectric { 
            ir: 1.0 // Index of Refraction
         }
    }
}

impl Dielectric
{
    pub fn new(index_of_refraction: f32) -> Dielectric
    {
        Dielectric
        {
            ir: index_of_refraction
        }
    }
}

impl Material for Dielectric
{
    fn scatter(&self, 
        r_in: &Ray, 
        rec: &HitRecord, 
        attenuation: &mut Color, 
        scattered: &mut Ray) -> bool 
    {
        *attenuation = Color::new(1.0,1.0,1.0);
        let refraction_ratio = if rec.front_face {
            1.0/self.ir
        } else {
            self.ir
        };
        
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = dot(&unit_direction.negate_vec(), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        // Check if ray can refract from object
        let cannot_refract = refraction_ratio * sin_theta > 1.0 ;
        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_number() {
            // Must Reflect
            direction = reflect(&unit_direction, &rec.normal);
        }else {
            // Can Refract
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.p, direction, r_in.time());
        return true
    }
}

/**
 * Check angle of impact for ray, to figure out if Dielectric material should reflect or not
 */
fn reflectance(cosine: f32, ref_idx: f32) -> f32
{
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    r0 + (1.0-r0)*((1.0-cosine).powf(5.0))
}
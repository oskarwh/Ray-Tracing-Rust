use crate::{vectors::{ray::Ray, vec3::Color}, objects::hit_record::HitRecord};

/**
 * Implementation for material
 */
pub trait Material
{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
} 
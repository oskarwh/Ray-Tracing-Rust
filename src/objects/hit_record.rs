use std::{rc::Rc, sync::Arc};

use crate::vectors::{vec3::{Point3, Vec3, dot}, ray::Ray};

use super::material::{material::Material, lambertian::Lambertian};

/**
 * A struct holding data about a hit on a Hittable object
 */
#[derive(Clone)]
pub struct HitRecord
{
    pub p: Point3, // Point of impact
    pub normal: Vec3, // Normal
    pub mat_ptr: Arc<dyn Material>,// Material which ray hit
    pub t: f32, // Root
    pub front_face: bool
}

/**
 * Deafult HitRecord
 */
impl Default for HitRecord
{
    fn default() -> HitRecord
    {
        HitRecord { 
            p: Point3::new(0.0,0.0,0.0), 
            normal: Vec3::new(0.0,0.0,0.0), 
            mat_ptr: Arc::new(Lambertian::default()),
            t: 0.0, 
            front_face: true 
        }
    }
}

/**
 * Public trait for a HitRecord
 */
impl HitRecord
{
    // Checks which direction the normal should be pointed, based on the incoming ray.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) 
    {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            // Need to clone as we do not own outward_normal
            outward_normal.clone()
        } else {
            // Do not need to copy as negate_vec() gives us new vec
            outward_normal.negate_vec()
        }
    }

    /**
     * Returns a new clone of the material object
     */
    pub fn get_material(&self) -> Arc<dyn Material>
    {
        self.mat_ptr.clone()
    }

    /**
     * Sets a new material for the object
     */
    pub fn set_material(&mut self, material: Arc<dyn Material>) 
    {
        self.mat_ptr = material;
    }
}
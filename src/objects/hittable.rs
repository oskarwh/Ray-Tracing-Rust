use std::rc::Rc;

use crate::vectors::ray::Ray;
use crate::vectors::vec3::Vec3;
use crate::utility::aabb::AABB;

use super::{hit_record::HitRecord, material::material::Material};

/*
* Struct that contains all information about a object that does not specify the shape, etc.
*/
pub struct HittableObject
{
    pub material: Rc<dyn Material>,
    pub travel_vec: Option<Vec3>,
    pub bbox: Rc<AABB>,
}

/**
 * Public trait for a hittable object
 */
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool; 

    /*
     * Getters for hittable object fields 
     */
    fn get_material(&self) -> Rc<dyn Material>;
    fn get_bounding_box(&self) -> Rc<AABB>;
    fn get_travel_vec(&self) -> Option<Vec3>;
}

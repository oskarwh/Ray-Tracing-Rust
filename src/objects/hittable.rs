use std::cmp::Ordering;
use std::sync::Arc;

use crate::{utility::interval::Interval, vectors::ray::Ray};
use crate::vectors::vec3::Vec3;
use crate::utility::aabb::AABB;

use super::{hit_record::HitRecord, material::material::Material};


/*
* Struct that contains all information about a object that does not specify the shape, etc.
*/
pub struct HittableObjectData
{
    pub material: Arc<dyn Material>,
    pub travel_vec: Option<Vec3>,
    pub bbox: Arc<AABB>,
}

/**
 * Public trait for a hittable object
 */
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, hit_rec: &mut HitRecord) -> bool; 
    fn get_bounding_box(&self) -> AABB;
}

/*
* Public trait describing how to fetch obligatory data fields
*/
pub trait Object {
    /*
     * Getters for hittable object fields 
     */
    fn get_material(&self) -> Arc<dyn Material>;
    fn get_travel_vec(&self) -> Option<Vec3>;
}

pub trait HittableObject: Hittable + Object {}

/**
 * Compare function to return which boundary is the smallest
 */
pub fn compare_aabb(object1: &'_ Arc<dyn Hittable>, object2: &'_ Arc<dyn Hittable>) -> Ordering{
    let b1_volume = object1.get_bounding_box().volume();
    let b2_volume = object2.get_bounding_box().volume();
    
    let mut order: Ordering;
    if b1_volume == b2_volume {
        order = Ordering::Equal;
    }else if b1_volume < b2_volume {
        order = Ordering::Less;
    }else {
        order = Ordering::Greater
    }

    return order;
}
use std::vec::Vec;
use std::rc::Rc;
use std::sync::{Arc, RwLock};


use crate::utility::aabb::AABB;
use crate::utility::interval::Interval;
use crate::vectors::ray::Ray;
use crate::vectors::vec3::Vec3;

use super::hit_record::HitRecord;
use super::hittable::{Hittable, HittableObject};

/**
 * Struct for a list of hittable objects
 */
pub struct HittableList
{
    list: Vec<Arc<dyn Hittable>>,
    bbox: Arc<RwLock<AABB>>
}

impl HittableList
{
    pub fn new() -> HittableList
    {
        // Boundary volume is "empty" when list is empy
        let c_vec = Vec3::new(0.0,0.0,0.0);
        let bbox = Arc::new(RwLock::new(AABB::new(c_vec, c_vec)));

        HittableList
        {
            list: Vec::<Arc<dyn Hittable>>::new(),
            bbox: bbox
        }
    }

    pub fn clear(&mut self)
    {
        self.list.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>)
    {
        self.list.push(object.clone());
        // Add new objects boundary to list boundary
        let bbox_read = *(self.bbox.read().unwrap());
        let mut bbox_write= self.bbox.write().unwrap();

        *bbox_write = AABB::from_aabb(&bbox_read, &(object.get_bounding_box()));
    }

    pub fn get_list(&mut self) -> &mut Vec<Arc<dyn Hittable>> {
        &mut (self.list)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, hit_rec: &mut HitRecord) -> bool
    {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.list.clone()
        {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec)
            {
                hit_anything = true;
                // If ray hit object it can not hit another object thta has a higher root as that would mean we would hit a object that is unreachable.
                closest_so_far = temp_rec.t;
                // Ownership transfers, so we needed to implement copy trait for HitRecord -- OLD
                // Owenrship still transfers but we now do a clone
                // Copy occurs here
                *hit_rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }
    
    fn get_bounding_box(&self) -> AABB {
        *(self.bbox.read().unwrap())
    }
}
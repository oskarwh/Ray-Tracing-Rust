use std::sync::{Arc, RwLock};

use crate::{utility::{aabb::AABB, interval::Interval}, vectors::ray::Ray};

use super::{hit_record::HitRecord, hittable::Hittable};

pub struct BvhNode {
    pub bbox: Arc<RwLock<AABB>>,
    pub left: Arc<RwLock<BvhNode>>,
    pub right: Arc<RwLock<BvhNode>>
} 

impl BvhNode {
    
    pub fn get_bounding_box(&self) -> AABB {
        *(self.bbox.read().unwrap())
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_interval: Interval, hit_rec: &mut HitRecord) -> bool {
        let bbox_read = *(self.bbox.read().unwrap());

        if !bbox_read.hit(&r, ray_interval) {return false}

        let hit_left = (*(self.left.read().unwrap())).hit(r, ray_interval, hit_rec);

        let right_max = if hit_left {hit_rec.t} else {ray_interval.max};
        let right_interval = Interval::new(ray_interval.min, right_max);
        let hit_right = (*(self.right.read().unwrap())).hit(r, right_interval, hit_rec);

        return hit_left || hit_right
    }
}
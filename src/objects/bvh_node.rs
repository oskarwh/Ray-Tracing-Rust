use std::sync::{Arc, RwLock};

use crate::{utility::{aabb::AABB, interval::Interval}, vectors::ray::Ray};

use super::{hit_record::HitRecord, hittable::{compare_aabb, Hittable}, hittable_list::HittableList};

pub struct BvhNode {
    pub bbox: Arc<RwLock<AABB>>,
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>
} 

impl BvhNode {
    /**
     * Add List of Hittable objects(World Scene) to a BVH Tree
     */
    pub fn add(hittable_list: &mut HittableList) -> Arc<BvhNode>{
        let list_ref = &mut hittable_list.get_list();
        let list_len = list_ref.len();

        Arc::new(Self::bvh_node(list_ref, 0, list_len))
    }

    /**
     * Split Hittable list into BVH Nodes 
     */
    fn bvh_node(list: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        // Size of list 
        let object_span = end - start;

        // Set up left and right sub tree
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        match object_span {
            1 => {
                let object = list[0].clone();
                left = object.clone();
                right = object;
            }
            2 => {
                left = list[0].clone();
                right = list[1].clone();
            }
            _ => {
                // Sort list and call on each half recursively
                list.sort_by(compare_aabb);
                let middle = start + object_span/2;
                left = Arc::new(Self::bvh_node(list, start, middle));
                right = Arc::new(Self::bvh_node(list, middle, end));
            }
        }
        // Set up node boundary
        let bbox = Arc::new(RwLock::new(AABB::from_aabb(
            &(left.get_bounding_box()), 
            &(right.get_bounding_box()))));
        
        // Return node
        BvhNode {
            bbox: bbox,
            left: left,
            right: right
        }
    }
}

impl Hittable for BvhNode {
    /**
     * Check if Node is hit
     */
    fn hit(&self, r: &Ray, ray_interval: Interval, hit_rec: &mut HitRecord) -> bool {
        let bbox_read = *(self.bbox.read().unwrap());

        if !bbox_read.hit(&r, ray_interval) {return false}

        let hit_left = (*(self.left)).hit(r, ray_interval, hit_rec);

        let right_max = if hit_left {hit_rec.t} else {ray_interval.max};
        let right_interval = Interval::new(ray_interval.min, right_max);
        let hit_right = (*(self.right)).hit(r, right_interval, hit_rec);

        return hit_left || hit_right
    }

    fn get_bounding_box(&self) -> AABB {
        *(self.bbox.read().unwrap())
    }
}
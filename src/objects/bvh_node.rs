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
                left = list[start].clone();
                right = list[start].clone();
            }
            2 => {
                left = list[start].clone();
                right = list[start + 1].clone();
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
        let bbox_read: AABB = *(self.bbox.read().unwrap());

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


#[cfg(test)]
mod test{
    use crate::{objects::{material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal}, sphere::Sphere}, vectors::vec3::{Color, Point3}};

    use super::*;

    fn set_up_scene() -> HittableList {
        let mut spheres: HittableList = HittableList::new();

        // Background 
        let ground_material = Arc::new(Lambertian::new(Color::new(0.5,0.5,0.5)));
        spheres.add(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material, None)));

        
        // Add spheres with different sizes unordered
        let material1 = Arc::new(Dielectric::new(1.5));
        spheres.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 3.0, material1, None)));

        let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        spheres.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2, None)));

        let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        spheres.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 2.0, material3, None)));

        let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        spheres.add(Arc::new(Sphere::new(Point3::new(2.0, 1.0, 0.0), 1.5, material3, None)));


        return spheres;
    }

    #[test]
    fn bvh_tree_structure() {
        let mut scene = set_up_scene();

        // Fetch bvh root 
        let bvh_node = BvhNode::add(&mut scene);  

        while true {

            let bbox = AABB::from_aabb(
                &(bvh_node.left.get_bounding_box()), 
                &(bvh_node.right.get_bounding_box()));

            assert_eq!(bbox.volume(), bvh_node.get_bounding_box().volume());

            let less = if bvh_node.left.get_bounding_box().volume() != bvh_node.right.get_bounding_box().volume() {
                true
            } else {
                false
            };
            assert_eq!(less, true);

            break;
        }        
    }

    #[test]
    fn aabb_sort() {
        let mut scene = set_up_scene();

        // Sort scene
        scene.get_list().sort_by(compare_aabb);

        // Fetch sorted list
        {
            let list = scene.get_list();

            let mut sorted = true;
            for index in 0..list.len() {
                if index == 0 {continue;}

                if list[index-1].get_bounding_box().volume() >= list[index].get_bounding_box().volume() {
                    sorted = false;
                    break;
                }
            }

            assert_eq!(sorted, true);
        }

        // Add object to make list unordered
        {
            let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
            scene.add(Arc::new(Sphere::new(Point3::new(2.0, 1.0, 0.0), 0.5, material3, None)));

            let list = scene.get_list();

            let mut sorted = true;
            for index in 0..list.len() {
                if index == 0 {continue;}

                if list[index-1].get_bounding_box().volume() >= list[index].get_bounding_box().volume() {
                    sorted = false;
                    break;
                }
            }

            assert_eq!(sorted, false);
        }
    }
}
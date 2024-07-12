use std::{rc::Rc, sync::Arc};
use std::option::Option;

use crate::{utility::{aabb::AABB, interval::Interval}, vectors::{ray::Ray, vec3::{dot, Point3, Vec3}}};

use super::{hit_record::HitRecord, hittable::{Hittable, HittableObject, HittableObjectData, Object}, material::material::Material,};

pub struct Sphere
{
    center: Point3,
    radius: f32,
    hittable_object: HittableObjectData
}

impl Sphere
{
    pub fn new(cen: Point3, r: f32, material: Arc<dyn Material>, destination: Option<Point3>) -> Sphere
    {
        // Set up boundary box for object
        let radius_vector = Vec3::new(r,r,r);
        let bbox: Arc<AABB>;

        let mut travel_vec = None;
        match destination {
            Some(dest) => {
                travel_vec = Some(dest - cen);
                
                // Create aabb box from sphere at t=0 and t=1
                let b1 = AABB::new(cen - radius_vector, cen + radius_vector);
                let b2 = AABB::new(dest - radius_vector, dest + radius_vector);
                bbox = Arc::new(AABB::from_aabb(&b1, &b2));
            }
            None => {
                // Create aabb box from center as sphere is static
                bbox = Arc::new(AABB::new(cen-radius_vector, cen+radius_vector));
            }
        }

        // Create hittable object struct for sphere
        let object = HittableObjectData {
            material: material,
            travel_vec: travel_vec,
            bbox: bbox
        };

        Sphere {
            radius: r,
            center: cen,
            hittable_object: object
        }
    }

    /*
     * Function to find center fo sphere at a specific time in a frame(Maybe this should be a trait)
     */
    pub fn center_position(&self, time: f32) -> Point3
    {
        let mut center = self.center;
        match self.get_travel_vec() {
            Some(travel_vec) => {center = center + travel_vec.const_mul(time);}
            None => {}
        }

        return center 
    }
}

/*
 * Sphere implements hittable trait, to check if rays it it
 */
impl Hittable for Sphere 
{
    fn hit(&self, r: &Ray, ray_t: Interval, hit_rec: &mut HitRecord) -> bool
    {
        // Find center at current time 
        let center = self.center_position(r.time());

        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - (self.radius*self.radius);
        let discriminant = half_b*half_b - a*c;
        
        // Check if ray hit the object
        if discriminant < 0.0
        {
            return false
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root)
        {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root)
            {
                return false
            }
        }

        // Set the hit record for the object
        hit_rec.t = root;
        hit_rec.p = r.at(hit_rec.t);
        
        // Set correct direction on normal
        let outward_normal = &(hit_rec.p - center).const_div(self.radius); 
        hit_rec.set_face_normal(r, outward_normal);
        
        // Set which material ray hit
        hit_rec.set_material(self.get_material());

        return true
    }

    fn get_bounding_box(&self) -> AABB {
        *(self.hittable_object.bbox)
    }
}

/*
* Implement Object to to use default access for data
*/
impl Object for Sphere {
    fn get_material(&self) -> Arc<dyn Material> {
        return self.hittable_object.material.clone()
    }
    
    fn get_travel_vec(&self) -> Option<Vec3> {
        return self.hittable_object.travel_vec
    }
}

/*
* Needs to implement empty trait to ensure that both Object and Hittable is implemented
*/
impl HittableObject for Sphere {}
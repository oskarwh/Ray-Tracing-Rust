use std::rc::Rc;

use crate::vectors::{vec3::{Point3, dot}, ray::Ray};

use super::{hittable::Hittable, hit_record::HitRecord, material::{material::Material, self}};

pub struct Sphere
{
    center: Point3,
    radius: f32,
    material: Rc<dyn Material>
}

impl Sphere
{
    pub fn new(cen: Point3, r: f32, material: Rc<dyn Material>) -> Sphere
    {
        Sphere {
            radius: r,
            center: cen,
            material: material
        }
    }
}

/*
 * Sphere implements hittable trait, to check if rays it it
 */
 impl Hittable for Sphere 
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool
    {
        let oc = r.origin() - self.center;
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
        if root < t_min || t_max < root
        {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root
            {
                return false
            }
        }

        // Set the hit record for the object
        hit_rec.t = root;
        hit_rec.p = r.at(hit_rec.t);
        
        // Set correct direction on normal
        let outward_normal = &(hit_rec.p - self.center).const_div(self.radius); 
        hit_rec.set_face_normal(r, outward_normal);
        
        // Set which material ray hit
        let material_clone = Rc::clone(&self.material);
        hit_rec.setMaterial(material_clone);

        return true
    }
}

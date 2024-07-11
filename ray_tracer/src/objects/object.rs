//use super::{hittable::Hittable, material::material::Material};

use crate::{utility::aabb::AABB};
use crate::vectors::{vec3::{Vec3}};


pub struct Object
{
    //material: Rc<dyn Material>,
    travel_vec: Option<Vec3>,
    bbox: AABB,
}

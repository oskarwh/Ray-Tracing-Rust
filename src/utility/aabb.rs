use crate::vectors::{vec3::Point3, ray::Ray};
use crate::utility::interval::Interval;

const N_AXIS: i32 = 3;

#[derive(Clone, Copy)]
pub struct AABB
{
    x: Interval,
    y: Interval,
    z: Interval
}

impl AABB 
{
    pub fn new(point1: Point3, point2: Point3) -> AABB
    {
        let x = 
            if point1.x() <= point2.x() {
                Interval::new(point1.x(), point2.x())
            } else {
                Interval::new(point2.x(), point1.x())
            };

        let y = 
            if point1.y() <= point2.y() {
                Interval::new(point1.y(), point2.y())
            } else {
                Interval::new(point2.y(), point1.y())
            };
        
        let z = 
            if point1.z() <= point2.z() {
                Interval::new(point1.z(), point2.z())
            } else {
                Interval::new(point2.z(), point1.z())
            };

        AABB {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn from_aabb(b1: &AABB, b2: &AABB) -> AABB {
        AABB {
            x: Interval::from_intervals(b1.x, b2.x),
            y: Interval::from_intervals(b1.y, b2.y),
            z: Interval::from_intervals(b1.z, b2.z),
        }
    }

    pub fn hit(&self, ray: &Ray, mut ray_interval: Interval) -> bool
    {
        let origin =  ray.origin().as_array();
        let direction = ray.direction().as_array();
        let axes = self.as_array();

        for axis in 0..N_AXIS {
            let axis: usize = axis as usize;
            let axis_interval = axes[axis];
            let adinv = 1.0 / direction[axis];

            // Calcualte ray intersection of min and max values in interval
            let t0 = (axis_interval.min - origin[axis]) * adinv; 
            let t1 = (axis_interval.max - origin[axis]) * adinv;
            
            if t0 < t1 {
                if t0 > ray_interval.min {ray_interval.min = t0;}
                if t1 < ray_interval.max {ray_interval.max = t1;}
            }else {
                if t1 > ray_interval.min {ray_interval.min = t1;}
                if t0 < ray_interval.max {ray_interval.max = t0;}
            }
            
            if ray_interval.max <= ray_interval.min {return false}
        }
   
        return true
    }

    /**
     * Returns internal intervals as an array
     */
    fn as_array(&self) -> [Interval; 3]  {
        [self.x, self.y, self.z]
    }

    /**
     * Computes the total volume of the boundary
     */
    pub fn volume(&self) -> f32 {
        let x_hat = self.x.max - self.x.min;
        let y_hat = self.y.max - self.y.min;
        let z_hat = self.z.max - self.z.min;
        return x_hat * y_hat * z_hat
    }
   
}
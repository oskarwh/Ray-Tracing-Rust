use crate::rtweekend{};

use super::{vec3::Point3, ray::Ray}

const N_AXIS = 3;

struct AABB
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
            x,
            y,
            z
        }
    }

    pub fn hit(&self, ray: Ray) -> bool
    {
        let origin =  ray.origin;
        let direction = ray.dir;
        let axes = self.as_array();

        for axis in 0..N_AXIS {
            let interval = axes[axis];
            let adinv = 1.0 / direction[axis]

            // Calcualte ray intersection of min and max values in interval
            let t0 = (interval.min - origin[axis]) * adinv; 
            let t1 = (interval.max - origin[axis]) * adinv;
            
            if t0 < t1 {
                
            }
            
        }
    }


    fn as_array(&self) -> [Interval; 3] {
        [self.x, self.y, self.z]
    }

    /*
     * Getters
     */
    pub fn x(&self) -> Interval {self.x}
    pub fn y(&self) -> Interval {self.y}
    pub fn z(&self) -> Interval {self.z}
}
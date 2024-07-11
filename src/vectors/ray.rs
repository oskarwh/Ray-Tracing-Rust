use super::vec3::Point3;
use super::vec3::Vec3;

pub struct Ray 
{
    origin: Point3,
    direction: Vec3,
    time: f32,
}

impl Default for Ray
{
    fn default() -> Ray {
        Ray {
            origin: Point3::default(),
            direction: Vec3::default(),
            time: 0.0
        }
    }
}

impl Ray
{
    /**
     * Returns a new ray with given start point and direction
     */
    pub fn new(origin: Point3, direction:Vec3, time:f32) -> Ray
    {
        Ray {
            origin: origin,
            direction: direction,
            time: time
        }
    }

    /*
    * Returns current position of ray
     */
    pub fn at(&self, t: f32) -> Point3
    {
        self.origin + self.direction.const_mul(t)
    }

    /*
     * Return origin point of ray
     */
    pub fn origin(&self) -> Point3
    {
        self.origin
    }

    /*
     * Return origin vecotor of ray
     */
    pub fn direction(&self) -> Vec3
    {
        self.direction
    }

    /*
     * Return start time for ray(based of camera)
     */
    pub fn time(&self) -> f32
    {
        self.time
    }
}
use crate::{vectors::{vec3::{Point3, Vec3, cross, random_in_unit_disk}, ray::Ray}, utility::rtweekend::degrees_to_radians};
use crate::random_number_custom;

pub struct Camera
{
    aspect_ratio: f32,
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,

    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,

    u: Vec3,
    v: Vec3, 
    w: Vec3,
    lens_radius: f32,

    frames_second: i32,
}

impl Default for Camera
{
    /**
     * Creates a camera with the default settings
     */
    fn default() -> Camera
    {
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0; 
        let origin = Point3::new(0.0,0.0,0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal.const_div(2.0) - vertical.const_div(2.0) - Vec3::new(0.0, 0.0, focal_length);

        Camera{
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_length: focal_length,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            lens_radius: 0.0,
            frames_second: 1, // One frame per second
        }
    }
}

impl Camera {

    /**
     * Creates a new camera with given aspect_ratio and vertical fov
     */
    pub fn new(lookfrom: Point3,
               lookat: Point3,
               vup: Vec3,
               vfov: f32, // vertical field-of-view in degrees
               aspect_ratio: f32,
               aperture: f32,
               focus_dist: f32,
               frames_second: i32
            ) -> Camera
    {
        // Calculate vfov
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan() as f32;
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = (cross(&vup, &w)).unit_vector();
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u.const_mul(viewport_width*focus_dist);
        let vertical = v.const_mul(viewport_height*focus_dist);
        let lower_left_corner = origin - horizontal.const_div(2.0) - vertical.const_div(2.0) - (w.const_mul(focus_dist));

        let lens_radius = aperture / 2.0;

        // Set to 1.0 as it not used here.
        let focal_legnth = 1.0;

        Camera{
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_length: 1.0,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner, 
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
            frames_second: frames_second
        }
    }

    /**
     * Creates ray based on given input (u,v)
     */
    pub fn get_ray(&self, s: f32, t: f32) -> Ray
    {
        let rd: Vec3 = random_in_unit_disk().const_mul(self.lens_radius);
        let offset = self.u.const_mul(rd.x()) + self.v.const_mul(rd.y());

        // Ray will be initiated at a random time in the current frame
        let max_time = 1.0 / self.frames_second as f32;
        let ray_time = random_number_custom(0.0, max_time);

        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + self.horizontal.const_mul(s) + self.vertical.const_mul(t) - self.origin - offset,
            ray_time
        )
    }
}
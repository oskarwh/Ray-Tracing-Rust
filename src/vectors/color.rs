use crate::{Color, objects::{hit_record::HitRecord, hittable_list::HittableList}};

use std::{io::{Write, StdoutLock}, f32::INFINITY};

use super::ray::Ray;

// Constants
const SPHERE_INTERSECT: f32 = 0.001;

/**
 * A utility function to write a single pixel's color out to the standard output stream
 */
pub fn write_color(handle: &mut StdoutLock, color: &Color, samples_per_pixel: i32) -> std::io::Result<()>
{  
    // Fetch rgb data
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale: f32 = 1.0 / samples_per_pixel as f32;
    r = (r*scale).sqrt();
    g = (g*scale).sqrt();
    b = (b*scale).sqrt();

    // Write the translated [0,255] value of each color component.
    r = 256.0 * clamp(r, 0.0, 0.999);
    g = 256.0 * clamp(g, 0.0, 0.999);
    b = 256.0 * clamp(b, 0.0, 0.999);

    let output = format!("{} {} {}\n",r, g, b);
    handle.write_all(output.as_bytes())?;

    return Ok(())
}

/**
 *  A function that check if a ray will hit any object, if no object is hit will return no light(color(0,0,0))
 */
pub fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color
{
    let mut rec = HitRecord::default();

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0
    {
        return Color::new(0.0,0.0,0.0);
    }

    // Check if ray hit anything
    if world.hit(r, SPHERE_INTERSECT, INFINITY, &mut rec)
    {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth-1)
        }
        return Color::new(0.0,0.0,0.0)
        // Calculate target by creating random ray's around unit sphere from 
        // impact point.
        //let target = rec.p + rec.normal + random_unit_vector();
        //return ray_color(&Ray::new(rec.p, target - rec.p), world, depth-1).const_mul(0.5);
        //return rec.normal + Color::new(1.0,1.0,1.0).const_mul(0.5);
    }

    // Not hit, will be background
    let unit_direction = r.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    Color::new(1.0,1.0,1.0).const_mul(1.0-t) + Color::new(0.5,0.7,1.0).const_mul(t)
}

/**
 * Clamps given value to the given range
 */
pub fn clamp(x: f32, min: f32, max: f32) -> f32
{
    if x < min {
        return min;
    } else if x > max {
        return max;
    }

    return x;
}

/*
 * Simple function, that hard codes a sphere in the image
 */
/*pub fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32
{
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - (radius*radius);
    let discriminant = half_b*half_b - a*c;

    // Check if the ray hit the ball
    if discriminant < 0.0
    {
        return -1.0
    } else 
    {
        return (-half_b - discriminant.sqrt()) / a;
    }
}*/
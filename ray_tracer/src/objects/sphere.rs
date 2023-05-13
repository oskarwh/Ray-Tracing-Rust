pub struct Sphere
{
    center: Point3,
    radius: f32
}

impl Hittable for Sphere 
{
    fn new(cen: Point3, r: f32) -> Sphere
    {
        Sphere {
            radius: r,
            center: cen
        }
    }

    fn hit(r: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool
    {
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - (radius*radius);
        let discriminant = half_b*half_b - a*c;
        
        // Check if ray hit the object
        if discriminant < 0 
        {
            return false
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root
        {
            root = (-half_b + sqrtd) / a;
            if (root < t_min || t_max < root)
            {
                return false
            }
        }

        // Set the hit record for the object
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - center) / radius;
    
        return true
    }
}

   
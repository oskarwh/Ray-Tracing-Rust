#[derive(Copy, Clone)]
pub struct Interval 
{
    pub min: f32,
    pub max: f32
}

impl Interval
{
    /*
     * Create a Interval between two points
     */
    pub fn new(min: f32, max: f32) -> Interval
    {
        Interval
        {
            min,
            max
        }
    }

    /*
     * Create a new Interval from two old Intervals
     */
    pub fn from_intervals(i1: Interval, i2: Interval) -> Interval
    {
        let min = if i1.min <= i2.min {i1.min} else {i2.min};
        let max = if i1.max >= i2.max {i1.max} else {i2.max};
        Interval
        {
            min: min,
            max: max,
        }
    }

    /*
     * Controll that value resides inside interval
     */
    pub fn surrounds(&self, v: f32) -> bool {
        return v >= self.min && v <= self.max
    }

    /*
     * Put value inside interval, if not already
     */
    pub fn clamp(&self, value: f32) -> f32
    {
        if value < self.min {return self.min}
        if value > self.max {return self.max}

        return value
    }

    /*
     * Expand interval 
     */
    pub fn expand(&self, delta: f32) -> Interval
    {   
        let padding = delta/2.0;
        return Interval::new(self.min - padding, self.max + padding)
    }
}
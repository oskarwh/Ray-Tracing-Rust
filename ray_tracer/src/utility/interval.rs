#[derive(Copy, Clone)]
pub struct Interval 
{
    pub min: f32,
    pub max: f32
}

impl Interval
{
    pub fn new(min: f32, max: f32) -> Interval
    {
        Interval
        {
            min,
            max
        }
    }

    /*
     * Controll that value to use resides inside interval
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
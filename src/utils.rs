pub fn arctan2(y: f32, x: f32) -> f32 {
    if x > 0.0 {
        return (y / x).atan();
    }
    if x < 0.0 && y >= 0.0 {
        return (y / x).atan() + PI;
    }
    if x < 0.0 && y < 0.0 {
        return (y / x).atan() - PI;
    }
    if x == 0.0 && y > 0.0 {
        return PI * 0.5;
    }
    if x == 0.0 && y < 0.0 {
        return PI * 0.5 * -1.0;
    }
    0.0
}

pub const PI: f32 = 3.141592;

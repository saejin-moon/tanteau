use std::f32::consts::PI;

pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// min of two values
pub fn min<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

// magnitude
pub fn mag(a: f32, b: f32) -> f32 {
    (a * a + b * b).sqrt()
}

// distance between two points
pub fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    mag(x2 - x1, y2 - y1)
}

// distance squared between two points
// faster cos no sqrt call
pub fn distsq(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let a = x2 - x1;
    let b = y2 - y1;
    a * a + b * b
}

// manhattan distance between two points
pub fn manhattan(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

// chebyshev distance between two points
pub fn chebyshev(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    max((x2 - x1).abs(), (y2 - y1).abs())
}

// normalize a value
pub fn norm(val: f32, low: f32, high: f32) -> f32 {
    (val - low) / (high - low)
}

// map a value from one range to another
pub fn map(val: f32, low1: f32, high1: f32, low2: f32, high2: f32) -> f32 {
    low2 + (high2 - low2) * norm(val, low1, high1)
}

// lerp two values
pub fn lerp(val: f32, target: f32, amt: f32) -> f32 {
    ((target - val) * amt) + val
}

pub fn constrain(val: f32, low: f32, high: f32) -> f32 {
    max(min(val, high), low)
}

// radians to degrees
pub fn rad(val: f32) -> f32 {
    val * (180. / PI)
}

// degrees to radians
pub fn deg(val: f32) -> f32 {
    val * (PI / 180.)
}
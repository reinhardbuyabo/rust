pub struct Rectangle {
    length: f32,
    width: f32,
}

impl Rectangle {
    // 'Constructor'
    pub fn new(l: f32, w: f32) -> Self {
        return Rectangle{
            length: l,
            width: w,
        };
    }

    pub fn area(&self) -> f32 {
        &self.length * &self.width
    }

    pub fn perimeter(&self) -> f32 {
        2.0 * (&self.length + &self.width)
    }
}
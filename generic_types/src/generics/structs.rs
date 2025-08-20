pub struct Point<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}
impl<X1, Y1> Point<X1, Y1> {
    pub fn x(&self) -> &X1 {
       &self.x
    }
    pub fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

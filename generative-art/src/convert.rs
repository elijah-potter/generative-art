use glam::Vec2;
use imageproc::point::Point;


pub trait ToPoint<T> {
    fn to_point(self) -> Point<T>;
}

impl ToPoint<i32> for Vec2 {
    fn to_point(self) -> Point<i32> {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

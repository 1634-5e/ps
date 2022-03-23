use iced::{Point, Size};

pub fn get_size(left_top: Point, right_bottom: Point) -> Size {
    Size::new(right_bottom.x - left_top.x, right_bottom.y - left_top.y)
}

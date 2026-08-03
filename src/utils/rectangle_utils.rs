use raylib::math::{Rectangle, Vector2};

pub fn center_of_rect(rect: Rectangle) -> Vector2 {
    return Vector2 {
        x: rect.x + rect.width / 2.0,
        y: rect.y + rect.height / 2.0,
    };
}

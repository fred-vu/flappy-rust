use macroquad::prelude::*;

pub fn circle_rect_collision(circle: Vec2, radius: f32, rect: Rect) -> bool {
    let closest_x = clamp(circle.x, rect.x, rect.x + rect.w);
    let closest_y = clamp(circle.y, rect.y, rect.y + rect.h);
    let dx = circle.x - closest_x;
    let dy = circle.y - closest_y;
    (dx * dx + dy * dy) <= radius * radius
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_rect_collision_detects_overlap() {
        let rect = Rect::new(10.0, 10.0, 20.0, 20.0);
        let circle = vec2(18.0, 18.0);
        assert!(circle_rect_collision(circle, 6.0, rect));
    }

    #[test]
    fn circle_rect_collision_detects_separation() {
        let rect = Rect::new(10.0, 10.0, 20.0, 20.0);
        let circle = vec2(0.0, 0.0);
        assert!(!circle_rect_collision(circle, 3.0, rect));
    }

    #[test]
    fn clamp_limits_values() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-2.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(12.0, 0.0, 10.0), 10.0);
    }
}

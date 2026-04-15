#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectBounds {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl RectBounds {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

pub fn circle_rect_collision(circle_x: f32, circle_y: f32, radius: f32, rect: RectBounds) -> bool {
    let closest_x = clamp(circle_x, rect.x, rect.x + rect.w);
    let closest_y = clamp(circle_y, rect.y, rect.y + rect.h);
    let dx = circle_x - closest_x;
    let dy = circle_y - closest_y;
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
        let rect = RectBounds::new(10.0, 10.0, 20.0, 20.0);
        assert!(circle_rect_collision(18.0, 18.0, 6.0, rect));
    }

    #[test]
    fn circle_rect_collision_detects_separation() {
        let rect = RectBounds::new(10.0, 10.0, 20.0, 20.0);
        assert!(!circle_rect_collision(0.0, 0.0, 3.0, rect));
    }

    #[test]
    fn clamp_limits_values() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-2.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(12.0, 0.0, 10.0), 10.0);
    }
}

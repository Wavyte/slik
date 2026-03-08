use crate::bezier::CubicBezier;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Easing {
    Linear,
    #[default]
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    Snappy,
    Custom(f64, f64, f64, f64),
}

impl Easing {
    pub fn to_bezier(self) -> CubicBezier {
        let (x1, y1, x2, y2) = match self {
            Easing::Linear => (0.0, 0.0, 1.0, 1.0),
            Easing::Ease => (0.25, 0.1, 0.25, 1.0),
            Easing::EaseIn => (0.42, 0.0, 1.0, 1.0),
            Easing::EaseOut => (0.0, 0.0, 0.58, 1.0),
            Easing::EaseInOut => (0.42, 0.0, 0.58, 1.0),
            Easing::Snappy => (0.16, 1.0, 0.3, 1.0),
            Easing::Custom(x1, y1, x2, y2) => (x1, y1, x2, y2),
        };
        CubicBezier::new(x1, y1, x2, y2)
    }

    #[inline]
    pub fn solve(self, t: f64) -> f64 {
        self.to_bezier().solve(t)
    }
}

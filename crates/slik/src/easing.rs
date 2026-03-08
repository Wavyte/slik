//! Easing presets and cubic-bezier helpers.

use crate::bezier::CubicBezier;

/// Preset easing curves supported by tween and keyframe transitions.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Easing {
    /// Linear progress.
    Linear,
    /// CSS-like `ease`.
    #[default]
    Ease,
    /// CSS-like `ease-in`.
    EaseIn,
    /// CSS-like `ease-out`.
    EaseOut,
    /// CSS-like `ease-in-out`.
    EaseInOut,
    /// A faster, punchier ease-out style curve.
    Snappy,
    /// A custom cubic-bezier curve.
    Custom(f64, f64, f64, f64),
}

impl Easing {
    /// Converts the easing definition into a cubic-bezier solver.
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
    /// Solves the easing curve at a normalized progress value in `[0.0, 1.0]`.
    pub fn solve(self, t: f64) -> f64 {
        self.to_bezier().solve(t)
    }
}

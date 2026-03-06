use crate::bezier::CubicBezier;

/// Named easing curves, each resolving to a `CubicBezier`.
///
/// Standard CSS curves plus `Snappy` (a tasteful overshoot).
/// `Custom` accepts raw `(x1, y1, x2, y2)` control points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    /// Constant speed. `(0, 0, 1, 1)`
    Linear,
    /// Gentle start and end. `(0.25, 0.1, 0.25, 1.0)`
    Ease,
    /// Slow start, fast end. `(0.42, 0, 1.0, 1.0)`
    EaseIn,
    /// Fast start, slow end. `(0, 0, 0.58, 1.0)`
    EaseOut,
    /// Symmetric slow-fast-slow. `(0.42, 0, 0.58, 1.0)`
    EaseInOut,
    /// Quick settle with subtle overshoot. `(0.16, 1.0, 0.3, 1.0)`
    Snappy,
    /// Raw control points `(x1, y1, x2, y2)`.
    Custom(f64, f64, f64, f64),
}

impl Easing {
    /// Resolve to the underlying `CubicBezier` solver.
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

    /// Evaluate progress for a normalized time `t ∈ [0, 1]`.
    #[inline]
    pub fn solve(self, t: f64) -> f64 {
        self.to_bezier().solve(t)
    }
}

impl Default for Easing {
    fn default() -> Self {
        Easing::Ease
    }
}

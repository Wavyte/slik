//! Cubic-bezier curve solving utilities.

/// A cubic-bezier timing curve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezier {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

const NEWTON_ITERATIONS: usize = 8;
const NEWTON_MIN_SLOPE: f64 = 1e-6;
const SUBDIVISION_PRECISION: f64 = 1e-7;
const SUBDIVISION_MAX_ITERS: usize = 12;

impl CubicBezier {
    /// Creates a new cubic-bezier curve from control points.
    pub const fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Returns `true` when the curve is equivalent to linear interpolation.
    pub fn is_linear(&self) -> bool {
        (self.x1 - self.y1).abs() < 1e-6 && (self.x2 - self.y2).abs() < 1e-6
    }

    /// Solves the curve's y-value for a normalized x-value in `[0.0, 1.0]`.
    pub fn solve(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        if x >= 1.0 {
            return 1.0;
        }
        if self.is_linear() {
            return x;
        }

        let t = self.solve_curve_x(x);
        Self::bezier_component(t, self.y1, self.y2)
    }

    fn solve_curve_x(&self, x: f64) -> f64 {
        let mut t = x;
        for _ in 0..NEWTON_ITERATIONS {
            let slope = Self::bezier_slope(t, self.x1, self.x2);
            if slope.abs() < NEWTON_MIN_SLOPE {
                break;
            }
            let current_x = Self::bezier_component(t, self.x1, self.x2) - x;
            t -= current_x / slope;
        }

        let residual = Self::bezier_component(t, self.x1, self.x2) - x;
        if residual.abs() < SUBDIVISION_PRECISION {
            return t;
        }

        let mut lo = 0.0;
        let mut hi = 1.0;
        t = x;

        for _ in 0..SUBDIVISION_MAX_ITERS {
            let current_x = Self::bezier_component(t, self.x1, self.x2);
            let err = current_x - x;
            if err.abs() < SUBDIVISION_PRECISION {
                break;
            }
            if err > 0.0 {
                hi = t;
            } else {
                lo = t;
            }
            t = (lo + hi) * 0.5;
        }

        t
    }

    #[inline]
    fn bezier_component(t: f64, c1: f64, c2: f64) -> f64 {
        let a = 1.0 + 3.0 * (c1 - c2);
        let b = 3.0 * (c2 - 2.0 * c1);
        let c = 3.0 * c1;
        ((a * t + b) * t + c) * t
    }

    #[inline]
    fn bezier_slope(t: f64, c1: f64, c2: f64) -> f64 {
        let a = 1.0 + 3.0 * (c1 - c2);
        let b = 3.0 * (c2 - 2.0 * c1);
        let c = 3.0 * c1;
        (3.0 * a * t + 2.0 * b) * t + c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_returns_identity() {
        let b = CubicBezier::new(0.0, 0.0, 1.0, 1.0);
        for i in 0..=10 {
            let x = i as f64 / 10.0;
            assert!((b.solve(x) - x).abs() < 1e-6, "linear at {x}");
        }
    }

    #[test]
    fn endpoints_are_exact() {
        let b = CubicBezier::new(0.42, 0.0, 0.58, 1.0);
        assert_eq!(b.solve(0.0), 0.0);
        assert_eq!(b.solve(1.0), 1.0);
    }

    #[test]
    fn ease_in_is_slow_start() {
        let ease_in = CubicBezier::new(0.42, 0.0, 1.0, 1.0);
        assert!(ease_in.solve(0.5) < 0.5);
    }

    #[test]
    fn ease_out_is_fast_start() {
        let ease_out = CubicBezier::new(0.0, 0.0, 0.58, 1.0);
        assert!(ease_out.solve(0.5) > 0.5);
    }

    #[test]
    fn snappy_is_fast_but_does_not_overshoot() {
        let snappy = CubicBezier::new(0.16, 1.0, 0.3, 1.0);
        let mid = snappy.solve(0.3);
        assert!(mid > 0.3, "snappy should advance quickly, got {mid}");
        assert!(mid <= 1.0, "snappy should not overshoot, got {mid}");
    }
}

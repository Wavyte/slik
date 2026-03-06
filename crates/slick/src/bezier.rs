/// Cubic Bézier easing curve solver.
///
/// Solves `B(t) = (1-t)³·P0 + 3(1-t)²t·P1 + 3(1-t)t²·P2 + t³·P3`
/// where P0=(0,0) and P3=(1,1), so only P1=(x1,y1) and P2=(x2,y2) are specified.
///
/// Given a normalized time `x ∈ [0,1]`, finds the corresponding progress `y`.
/// Uses Newton-Raphson with bisection fallback for robust convergence.
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
    pub const fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Returns `true` if the curve is effectively linear (both control points on the diagonal).
    pub fn is_linear(&self) -> bool {
        (self.x1 - self.y1).abs() < 1e-6 && (self.x2 - self.y2).abs() < 1e-6
    }

    /// Evaluate the Y (progress) coordinate for a given X (time) coordinate.
    /// Both input and output are in `[0, 1]`.
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

        // Find the parametric t for the given x
        let t = self.solve_curve_x(x);
        // Evaluate y at that t
        Self::bezier_component(t, self.y1, self.y2)
    }

    /// Solve for parametric `t` given `x` using Newton-Raphson + bisection fallback.
    fn solve_curve_x(&self, x: f64) -> f64 {
        // Newton-Raphson: fast convergence when derivative is non-tiny
        let mut t = x; // initial guess
        for _ in 0..NEWTON_ITERATIONS {
            let slope = Self::bezier_slope(t, self.x1, self.x2);
            if slope.abs() < NEWTON_MIN_SLOPE {
                break;
            }
            let current_x = Self::bezier_component(t, self.x1, self.x2) - x;
            t -= current_x / slope;
        }

        // Verify convergence; if not, fall back to bisection
        let residual = Self::bezier_component(t, self.x1, self.x2) - x;
        if residual.abs() < SUBDIVISION_PRECISION {
            return t;
        }

        // Bisection fallback — guaranteed convergence
        let mut lo = 0.0_f64;
        let mut hi = 1.0_f64;
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

    /// Evaluate one component (x or y) of the cubic bezier at parametric value `t`.
    /// `c1` and `c2` are the corresponding control point coordinates.
    ///
    /// B(t) = 3(1-t)²t·c1 + 3(1-t)t²·c2 + t³
    ///      = t·(3c1 + t·(-6c1 + 3c2 + t·(3c1 - 3c2 + 1)))
    #[inline]
    fn bezier_component(t: f64, c1: f64, c2: f64) -> f64 {
        // Horner's form for efficiency
        let a = 1.0 + 3.0 * (c1 - c2);
        let b = 3.0 * (c2 - 2.0 * c1);
        let c = 3.0 * c1;
        ((a * t + b) * t + c) * t
    }

    /// First derivative of bezier_component with respect to `t`.
    /// B'(t) = 3(1-t)²·c1 + 6(1-t)t·(c2-c1) + 3t²·(1-c2)
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
        // At t=0.5, progress should be less than 0.5 (slow start)
        assert!(ease_in.solve(0.5) < 0.5);
    }

    #[test]
    fn ease_out_is_fast_start() {
        let ease_out = CubicBezier::new(0.0, 0.0, 0.58, 1.0);
        // At t=0.5, progress should be more than 0.5 (fast start)
        assert!(ease_out.solve(0.5) > 0.5);
    }

    #[test]
    fn overshoot_curve_exceeds_one() {
        // "snappy" preset overshoots
        let snappy = CubicBezier::new(0.16, 1.0, 0.3, 1.0);
        // Some mid-range value should exceed 1.0
        let mid = snappy.solve(0.3);
        assert!(mid > 1.0, "snappy should overshoot, got {mid}");
    }
}

use crate::bezier::CubicBezier;
use crate::easing::Easing;

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// A `Driver` interpolates a scalar value from its current position toward a target
/// over successive ticks. Implementations include physics-based springs and
/// duration-based bezier tweens.
///
/// All drivers operate on `f64` scalars. Higher-level types (colors, vectors)
/// decompose into per-channel drivers.
pub trait Driver: Send + Sync {
    /// Advance the simulation by `dt` seconds.
    fn tick(&mut self, dt: f64);

    /// Current interpolated value.
    fn value(&self) -> f64;

    /// `true` when the driver has converged / completed.
    fn is_done(&self) -> bool;

    /// Redirect the driver mid-flight. `current_value` is the live output value;
    /// `new_target` is the new destination.
    ///
    /// - **Spring**: keeps position & velocity, swaps target.
    /// - **Tween/Keyframe**: restarts from `current_value → new_target`.
    fn set_target(&mut self, current_value: f64, new_target: f64);
}

// ---------------------------------------------------------------------------
// SpringDriver
// ---------------------------------------------------------------------------

/// Physics-based spring using semi-implicit Euler integration.
///
/// Converges when both displacement and velocity drop below `ε`.
/// Has no fixed duration — the spring determines its own settling time.
#[derive(Debug, Clone)]
pub struct SpringDriver {
    /// Current position
    x: f64,
    /// Current velocity
    v: f64,
    /// Target position
    target: f64,
    /// Stiffness coefficient (`k`)
    stiffness: f64,
    /// Damping coefficient (`d`)
    damping: f64,
    /// Mass (`m`)
    mass: f64,
    /// Convergence threshold
    epsilon: f64,
    done: bool,
}

impl SpringDriver {
    pub fn new(stiffness: f64, damping: f64, mass: f64) -> Self {
        Self {
            x: 0.0,
            v: 0.0,
            target: 0.0,
            stiffness,
            damping,
            mass,
            epsilon: 0.001,
            done: true,
        }
    }

    /// Underdamped (bouncy) preset: `k=120, d=14, m=1`
    pub fn bouncy() -> Self {
        Self::new(120.0, 14.0, 1.0)
    }

    /// Critically damped (snappy, no overshoot): `k=170, d=26, m=1`
    pub fn snappy() -> Self {
        Self::new(170.0, 26.0, 1.0)
    }

    /// Overdamped (gentle) preset: `k=170, d=60, m=1`
    pub fn gentle() -> Self {
        Self::new(170.0, 60.0, 1.0)
    }
}

impl Driver for SpringDriver {
    fn tick(&mut self, dt: f64) {
        if self.done {
            return;
        }

        // Semi-implicit Euler: update velocity first, then position
        let displacement = self.x - self.target;
        let force = -self.stiffness * displacement - self.damping * self.v;
        let accel = force / self.mass;

        self.v += accel * dt;
        self.x += self.v * dt;

        // Convergence check
        if displacement.abs() < self.epsilon && self.v.abs() < self.epsilon {
            self.x = self.target;
            self.v = 0.0;
            self.done = true;
        }
    }

    fn value(&self) -> f64 {
        self.x
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn set_target(&mut self, current_value: f64, new_target: f64) {
        if self.done {
            // Fresh or converged: start from the current animated position
            self.x = current_value;
            self.v = 0.0;
        }
        // Mid-flight: keep x and v for velocity preservation — only swap destination
        self.target = new_target;
        self.done = false;
    }
}

// ---------------------------------------------------------------------------
// TweenDriver
// ---------------------------------------------------------------------------

/// Duration-based tween with a single cubic bézier easing curve.
///
/// Linearly maps `elapsed / duration → [0,1]`, applies the bezier, then lerps
/// from `from` to `to`.
#[derive(Debug, Clone)]
pub struct TweenDriver {
    from: f64,
    to: f64,
    elapsed: f64,
    duration: f64,
    bezier: CubicBezier,
    done: bool,
}

impl TweenDriver {
    /// Create a tween with the given `duration` (in seconds) and `easing` curve.
    pub fn new(duration: f64, easing: Easing) -> Self {
        Self {
            from: 0.0,
            to: 0.0,
            elapsed: 0.0,
            duration,
            bezier: easing.to_bezier(),
            done: true,
        }
    }
}

impl Driver for TweenDriver {
    fn tick(&mut self, dt: f64) {
        if self.done {
            return;
        }

        self.elapsed += dt;

        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.done = true;
        }
    }

    fn value(&self) -> f64 {
        if self.done && self.elapsed >= self.duration {
            return self.to;
        }

        let t = if self.duration > 0.0 {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        } else {
            1.0
        };

        let progress = self.bezier.solve(t);
        self.from + (self.to - self.from) * progress
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn set_target(&mut self, current_value: f64, new_target: f64) {
        // No velocity to preserve; restart from where we are now.
        self.from = current_value;
        self.to = new_target;
        self.elapsed = 0.0;
        self.done = false;
    }
}

// ---------------------------------------------------------------------------
// KeyframeDriver
// ---------------------------------------------------------------------------

/// A keyframe offset with a target value and the easing curve used to reach it.
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// Offset in `[0, 1]` — position within the total duration.
    pub offset: f64,
    /// Value at this keyframe.
    pub value: f64,
    /// Easing to apply *to reach* this keyframe from the previous one.
    pub easing: Easing,
}

/// Multi-segment keyframe animation. Each segment between consecutive keyframes
/// is an independent bezier tween.
#[derive(Debug, Clone)]
pub struct KeyframeDriver {
    /// Sorted keyframes. First must be offset=0, last must be offset=1.
    keyframes: Vec<Keyframe>,
    elapsed: f64,
    duration: f64,
    /// Cached bezier solvers, one per segment (len = keyframes.len() - 1)
    beziers: Vec<CubicBezier>,
    done: bool,
    /// When retargeted mid-flight, the original "from" value is remapped
    /// so the first keyframe's value becomes `from` and the last becomes `to`.
    value_from: f64,
    value_to: f64,
}

impl KeyframeDriver {
    /// Build from a list of keyframes and total duration (seconds).
    ///
    /// Keyframes must include offsets `0.0` and `1.0`. They will be sorted by offset.
    /// Easing on the `offset=0.0` keyframe is ignored (there's no preceding segment).
    pub fn new(mut keyframes: Vec<Keyframe>, duration: f64) -> Self {
        keyframes.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());

        debug_assert!(
            keyframes.len() >= 2,
            "KeyframeDriver requires at least 2 keyframes"
        );

        let beziers: Vec<CubicBezier> = keyframes
            .windows(2)
            .map(|pair| pair[1].easing.to_bezier())
            .collect();

        let value_from = keyframes.first().map(|k| k.value).unwrap_or(0.0);
        let value_to = keyframes.last().map(|k| k.value).unwrap_or(0.0);

        Self {
            keyframes,
            elapsed: 0.0,
            duration,
            beziers,
            done: true,
            value_from,
            value_to,
        }
    }

    /// Find the active segment index for the given normalized time.
    fn segment_for(&self, global_t: f64) -> usize {
        for (i, pair) in self.keyframes.windows(2).enumerate() {
            if global_t <= pair[1].offset {
                return i;
            }
        }
        self.keyframes.len().saturating_sub(2)
    }
}

impl Driver for KeyframeDriver {
    fn tick(&mut self, dt: f64) {
        if self.done {
            return;
        }

        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.done = true;
        }
    }

    fn value(&self) -> f64 {
        if self.keyframes.len() < 2 {
            return self.value_to;
        }

        if self.done && self.elapsed >= self.duration {
            return self.value_to;
        }

        // Normalized global time
        let global_t = if self.duration > 0.0 {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        } else {
            1.0
        };

        let seg = self.segment_for(global_t);
        let kf_start = &self.keyframes[seg];
        let kf_end = &self.keyframes[seg + 1];

        // Local t within this segment
        let seg_span = kf_end.offset - kf_start.offset;
        let local_t = if seg_span > 0.0 {
            ((global_t - kf_start.offset) / seg_span).clamp(0.0, 1.0)
        } else {
            1.0
        };

        // Apply segment's bezier easing
        let eased = self.beziers[seg].solve(local_t);

        // Map keyframe values through the retarget remapping.
        // If value_from == first keyframe and value_to == last keyframe, this is identity.
        // Otherwise, we linearly remap: keyframe value domain → [value_from, value_to].
        let kf_domain_from = self.keyframes.first().unwrap().value;
        let kf_domain_to = self.keyframes.last().unwrap().value;
        let kf_span = kf_domain_to - kf_domain_from;

        let raw_value = kf_start.value + (kf_end.value - kf_start.value) * eased;

        if kf_span.abs() < f64::EPSILON {
            // All keyframes have the same value; just return the target
            return self.value_to;
        }

        // Remap from keyframe-space to actual value-space
        let normalized = (raw_value - kf_domain_from) / kf_span;
        self.value_from + (self.value_to - self.value_from) * normalized
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn set_target(&mut self, current_value: f64, new_target: f64) {
        self.value_from = current_value;
        self.value_to = new_target;
        self.elapsed = 0.0;
        self.done = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spring_converges() {
        let mut s = SpringDriver::snappy();
        s.set_target(0.0, 100.0);
        s.x = 0.0;

        for _ in 0..600 {
            s.tick(1.0 / 60.0); // 10 seconds at 60fps
        }

        assert!(s.is_done(), "spring should converge after 10s");
        assert!((s.value() - 100.0).abs() < 0.01);
    }

    #[test]
    fn tween_completes_at_duration() {
        let mut tw = TweenDriver::new(0.3, Easing::Linear);
        tw.set_target(0.0, 10.0);

        // Tick past the duration
        tw.tick(0.35);

        assert!(tw.is_done());
        assert!((tw.value() - 10.0).abs() < 1e-6);
    }

    #[test]
    fn tween_midpoint_with_linear() {
        let mut tw = TweenDriver::new(1.0, Easing::Linear);
        tw.set_target(0.0, 100.0);
        tw.tick(0.5);

        assert!((tw.value() - 50.0).abs() < 0.5);
    }

    #[test]
    fn keyframes_traverse_segments() {
        let kfs = vec![
            Keyframe { offset: 0.0, value: 0.0, easing: Easing::Linear },
            Keyframe { offset: 0.5, value: 100.0, easing: Easing::Linear },
            Keyframe { offset: 1.0, value: 50.0, easing: Easing::Linear },
        ];
        let mut kd = KeyframeDriver::new(kfs, 1.0);
        kd.set_target(0.0, 50.0); // maps: kf 0→0, kf 100→(scaled), kf 50→50

        // At t=0.5s (offset=0.5), keyframe value = 100, which in normalized space = 1.0
        // remapped: 0 + (50-0)*1.0 = 50
        kd.tick(0.5);
        let val = kd.value();
        assert!(
            (val - 50.0).abs() < 1.0,
            "at midpoint expected ~50, got {val}"
        );
    }
}

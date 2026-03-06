use crate::bezier::CubicBezier;
use crate::easing::Easing;
use std::error::Error;
use std::fmt;

pub trait Driver: Send + Sync {
    fn tick(&mut self, dt: f64);
    fn value(&self) -> f64;
    fn is_done(&self) -> bool;
    fn set_target(&mut self, current_value: f64, new_target: f64);
}

#[derive(Debug, Clone)]
pub struct SpringDriver {
    x: f64,
    v: f64,
    target: f64,
    stiffness: f64,
    damping: f64,
    mass: f64,
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

    pub fn bouncy() -> Self {
        Self::new(120.0, 14.0, 1.0)
    }

    pub fn snappy() -> Self {
        Self::new(170.0, 26.0, 1.0)
    }

    pub fn gentle() -> Self {
        Self::new(170.0, 60.0, 1.0)
    }
}

impl Driver for SpringDriver {
    fn tick(&mut self, dt: f64) {
        if self.done {
            return;
        }

        let displacement = self.x - self.target;
        let force = -self.stiffness * displacement - self.damping * self.v;
        let accel = force / self.mass;

        self.v += accel * dt;
        self.x += self.v * dt;

        let remaining = self.x - self.target;
        if remaining.abs() < self.epsilon && self.v.abs() < self.epsilon {
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
        self.x = current_value;
        if self.done {
            self.v = 0.0;
        }
        self.target = new_target;
        self.done = false;
    }
}

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
            self.elapsed = self.duration.max(0.0);
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
        self.from = current_value;
        self.to = new_target;
        self.elapsed = 0.0;
        self.done = self.duration <= 0.0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyframeValue {
    Current,
    Target,
    Absolute(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Keyframe {
    pub offset: f64,
    pub value: KeyframeValue,
    pub easing: Easing,
}

impl Keyframe {
    pub fn current(offset: f64) -> Self {
        Self {
            offset,
            value: KeyframeValue::Current,
            easing: Easing::Linear,
        }
    }

    pub fn target(offset: f64) -> Self {
        Self {
            offset,
            value: KeyframeValue::Target,
            easing: Easing::Linear,
        }
    }

    pub fn absolute(offset: f64, value: f64) -> Self {
        Self {
            offset,
            value: KeyframeValue::Absolute(value),
            easing: Easing::Linear,
        }
    }

    pub fn ease(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyframeError {
    Empty,
    NeedsAtLeastTwo,
    DurationMustBeFinite,
    OffsetNotFinite,
    OffsetOutOfRange,
    OffsetsMustStartAtZero,
    OffsetsMustEndAtOne,
    OffsetsMustIncrease,
    AbsoluteValueNotFinite,
    FinalKeyframeMustTarget,
}

impl fmt::Display for KeyframeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "keyframes cannot be empty",
            Self::NeedsAtLeastTwo => "keyframes require at least two entries",
            Self::DurationMustBeFinite => "keyframe duration must be finite and non-negative",
            Self::OffsetNotFinite => "keyframe offsets must be finite",
            Self::OffsetOutOfRange => "keyframe offsets must be between 0.0 and 1.0",
            Self::OffsetsMustStartAtZero => "the first keyframe offset must be 0.0",
            Self::OffsetsMustEndAtOne => "the last keyframe offset must be 1.0",
            Self::OffsetsMustIncrease => "keyframe offsets must be strictly increasing",
            Self::AbsoluteValueNotFinite => "absolute keyframe values must be finite",
            Self::FinalKeyframeMustTarget => "the final keyframe must use KeyframeValue::Target",
        };
        f.write_str(message)
    }
}

impl Error for KeyframeError {}

#[derive(Debug, Clone)]
pub struct KeyframeTransition {
    keyframes: Vec<Keyframe>,
    duration: f64,
}

impl KeyframeTransition {
    pub fn new(mut keyframes: Vec<Keyframe>, duration: f64) -> Result<Self, KeyframeError> {
        if keyframes.is_empty() {
            return Err(KeyframeError::Empty);
        }
        if keyframes.len() < 2 {
            return Err(KeyframeError::NeedsAtLeastTwo);
        }
        if !duration.is_finite() || duration < 0.0 {
            return Err(KeyframeError::DurationMustBeFinite);
        }

        keyframes.sort_by(|a, b| a.offset.total_cmp(&b.offset));

        if keyframes[0].offset != 0.0 {
            return Err(KeyframeError::OffsetsMustStartAtZero);
        }
        if keyframes.last().map(|k| k.offset) != Some(1.0) {
            return Err(KeyframeError::OffsetsMustEndAtOne);
        }
        if !matches!(
            keyframes.last().map(|k| k.value),
            Some(KeyframeValue::Target)
        ) {
            return Err(KeyframeError::FinalKeyframeMustTarget);
        }

        let mut prev = None;
        for keyframe in &keyframes {
            if !keyframe.offset.is_finite() {
                return Err(KeyframeError::OffsetNotFinite);
            }
            if !(0.0..=1.0).contains(&keyframe.offset) {
                return Err(KeyframeError::OffsetOutOfRange);
            }
            if let Some(prev_offset) = prev {
                if keyframe.offset <= prev_offset {
                    return Err(KeyframeError::OffsetsMustIncrease);
                }
            }
            if let KeyframeValue::Absolute(value) = keyframe.value {
                if !value.is_finite() {
                    return Err(KeyframeError::AbsoluteValueNotFinite);
                }
            }
            prev = Some(keyframe.offset);
        }

        Ok(Self {
            keyframes,
            duration,
        })
    }

    pub fn duration(&self) -> f64 {
        self.duration
    }

    pub fn keyframes(&self) -> &[Keyframe] {
        &self.keyframes
    }
}

#[derive(Debug, Clone)]
pub struct KeyframeDriver {
    transition: KeyframeTransition,
    elapsed: f64,
    beziers: Vec<CubicBezier>,
    resolved_values: Vec<f64>,
    done: bool,
}

impl KeyframeDriver {
    pub fn new(transition: KeyframeTransition) -> Self {
        let beziers = transition
            .keyframes
            .windows(2)
            .map(|pair| pair[1].easing.to_bezier())
            .collect();

        Self {
            transition,
            elapsed: 0.0,
            beziers,
            resolved_values: Vec::new(),
            done: true,
        }
    }

    fn segment_for(&self, global_t: f64) -> usize {
        for (i, pair) in self.transition.keyframes.windows(2).enumerate() {
            if global_t <= pair[1].offset {
                return i;
            }
        }
        self.transition.keyframes.len().saturating_sub(2)
    }

    fn resolve_value(kind: KeyframeValue, current_value: f64, new_target: f64) -> f64 {
        match kind {
            KeyframeValue::Current => current_value,
            KeyframeValue::Target => new_target,
            KeyframeValue::Absolute(value) => value,
        }
    }
}

impl Driver for KeyframeDriver {
    fn tick(&mut self, dt: f64) {
        if self.done {
            return;
        }

        self.elapsed += dt;
        if self.elapsed >= self.transition.duration {
            self.elapsed = self.transition.duration.max(0.0);
            self.done = true;
        }
    }

    fn value(&self) -> f64 {
        if self.resolved_values.len() < 2 {
            return self.resolved_values.last().copied().unwrap_or(0.0);
        }

        if self.done && self.elapsed >= self.transition.duration {
            return *self.resolved_values.last().unwrap();
        }

        let global_t = if self.transition.duration > 0.0 {
            (self.elapsed / self.transition.duration).clamp(0.0, 1.0)
        } else {
            1.0
        };

        let seg = self.segment_for(global_t);
        let kf_start = &self.transition.keyframes[seg];
        let kf_end = &self.transition.keyframes[seg + 1];
        let seg_span = kf_end.offset - kf_start.offset;
        let local_t = if seg_span > 0.0 {
            ((global_t - kf_start.offset) / seg_span).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let eased = self.beziers[seg].solve(local_t);

        let from = self.resolved_values[seg];
        let to = self.resolved_values[seg + 1];
        from + (to - from) * eased
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn set_target(&mut self, current_value: f64, new_target: f64) {
        self.resolved_values = self
            .transition
            .keyframes
            .iter()
            .map(|keyframe| Self::resolve_value(keyframe.value, current_value, new_target))
            .collect();
        self.elapsed = 0.0;
        self.done = self.transition.duration <= 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spring_converges() {
        let mut spring = SpringDriver::snappy();
        spring.set_target(0.0, 100.0);
        for _ in 0..600 {
            spring.tick(1.0 / 60.0);
        }
        assert!(spring.is_done());
        assert!((spring.value() - 100.0).abs() < 0.01);
    }

    #[test]
    fn tween_completes_at_duration() {
        let mut tween = TweenDriver::new(0.3, Easing::Linear);
        tween.set_target(0.0, 10.0);
        tween.tick(0.35);
        assert!(tween.is_done());
        assert!((tween.value() - 10.0).abs() < 1e-6);
    }

    #[test]
    fn tween_midpoint_with_linear() {
        let mut tween = TweenDriver::new(1.0, Easing::Linear);
        tween.set_target(0.0, 100.0);
        tween.tick(0.5);
        assert!((tween.value() - 50.0).abs() < 0.5);
    }

    #[test]
    fn keyframes_can_overshoot_and_return_to_target() {
        let transition = KeyframeTransition::new(
            vec![
                Keyframe::current(0.0),
                Keyframe::absolute(0.5, 100.0),
                Keyframe::target(1.0),
            ],
            1.0,
        )
        .unwrap();

        let mut driver = KeyframeDriver::new(transition);
        driver.set_target(0.0, 20.0);
        driver.tick(0.5);
        assert!((driver.value() - 100.0).abs() < 1e-6);
        driver.tick(0.5);
        assert!((driver.value() - 20.0).abs() < 1e-6);
    }

    #[test]
    fn keyframes_require_target_at_end() {
        let err = KeyframeTransition::new(
            vec![Keyframe::current(0.0), Keyframe::absolute(1.0, 1.0)],
            0.2,
        )
        .unwrap_err();
        assert_eq!(err, KeyframeError::FinalKeyframeMustTarget);
    }
}

//! Transition configuration types.

use crate::driver::{Keyframe, KeyframeError, KeyframeTransition};
use crate::easing::Easing;
use crate::style::MotionProp;
use std::error::Error;
use std::fmt;

/// Supported transition families.
#[derive(Debug, Clone)]
pub enum Transition {
    /// A physics-based spring.
    Spring {
        /// Spring stiffness coefficient.
        stiffness: f64,
        /// Spring damping coefficient.
        damping: f64,
        /// Spring mass. Must be greater than zero.
        mass: f64,
    },
    /// A time-based tween with easing.
    Tween {
        /// Duration in seconds.
        duration: f64,
        /// Easing curve.
        easing: Easing,
    },
    /// A keyframe sequence.
    Keyframes(KeyframeTransition),
}

/// Validation errors for spring and tween transition constructors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    /// Spring stiffness was not finite.
    StiffnessNotFinite,
    /// Spring stiffness was negative.
    StiffnessNegative,
    /// Spring damping was not finite.
    DampingNotFinite,
    /// Spring damping was negative.
    DampingNegative,
    /// Spring mass was not finite.
    MassNotFinite,
    /// Spring mass was zero or negative.
    MassNonPositive,
    /// Tween duration was not finite.
    DurationNotFinite,
    /// Tween duration was negative.
    DurationNegative,
}

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::StiffnessNotFinite => "spring stiffness must be finite",
            Self::StiffnessNegative => "spring stiffness must be non-negative",
            Self::DampingNotFinite => "spring damping must be finite",
            Self::DampingNegative => "spring damping must be non-negative",
            Self::MassNotFinite => "spring mass must be finite",
            Self::MassNonPositive => "spring mass must be greater than zero",
            Self::DurationNotFinite => "tween duration must be finite",
            Self::DurationNegative => "tween duration must be non-negative",
        };
        f.write_str(message)
    }
}

impl Error for TransitionError {}

impl Transition {
    /// Returns the default spring transition used across the crate.
    pub fn spring() -> Self {
        Self::Spring {
            stiffness: 170.0,
            damping: 26.0,
            mass: 1.0,
        }
    }

    /// Returns a looser, more playful spring preset.
    pub fn spring_bouncy() -> Self {
        Self::Spring {
            stiffness: 120.0,
            damping: 14.0,
            mass: 1.0,
        }
    }

    /// Returns a slower, more damped spring preset.
    pub fn spring_gentle() -> Self {
        Self::Spring {
            stiffness: 170.0,
            damping: 60.0,
            mass: 1.0,
        }
    }

    /// Creates a validated custom spring transition.
    pub fn spring_custom(stiffness: f64, damping: f64, mass: f64) -> Result<Self, TransitionError> {
        validate_spring(stiffness, damping, mass)?;
        Ok(Self::Spring {
            stiffness,
            damping,
            mass,
        })
    }

    /// Creates a validated tween transition.
    pub fn tween(duration_secs: f64, easing: Easing) -> Result<Self, TransitionError> {
        validate_tween(duration_secs)?;
        Ok(Self::Tween {
            duration: duration_secs,
            easing,
        })
    }

    /// Creates a keyframe transition.
    pub fn keyframes(keyframes: Vec<Keyframe>, duration_secs: f64) -> Result<Self, KeyframeError> {
        Ok(Self::Keyframes(KeyframeTransition::new(
            keyframes,
            duration_secs,
        )?))
    }
}

impl Transition {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn is_runtime_valid(&self) -> bool {
        match self {
            Self::Spring {
                stiffness,
                damping,
                mass,
            } => validate_spring(*stiffness, *damping, *mass).is_ok(),
            Self::Tween { duration, .. } => validate_tween(*duration).is_ok(),
            Self::Keyframes(_) => true,
        }
    }
}

fn validate_spring(stiffness: f64, damping: f64, mass: f64) -> Result<(), TransitionError> {
    if !stiffness.is_finite() {
        return Err(TransitionError::StiffnessNotFinite);
    }
    if stiffness < 0.0 {
        return Err(TransitionError::StiffnessNegative);
    }
    if !damping.is_finite() {
        return Err(TransitionError::DampingNotFinite);
    }
    if damping < 0.0 {
        return Err(TransitionError::DampingNegative);
    }
    if !mass.is_finite() {
        return Err(TransitionError::MassNotFinite);
    }
    if mass <= 0.0 {
        return Err(TransitionError::MassNonPositive);
    }

    Ok(())
}

fn validate_tween(duration_secs: f64) -> Result<(), TransitionError> {
    if !duration_secs.is_finite() {
        return Err(TransitionError::DurationNotFinite);
    }
    if duration_secs < 0.0 {
        return Err(TransitionError::DurationNegative);
    }

    Ok(())
}

impl Default for Transition {
    fn default() -> Self {
        Self::spring()
    }
}

/// Dense transition storage with a default transition plus optional per-property
/// overrides.
#[derive(Debug, Clone)]
pub struct TransitionMap {
    default: Transition,
    per_prop: [Option<Transition>; MotionProp::COUNT],
}

impl TransitionMap {
    /// Creates a transition map with `default` used for every property unless
    /// overridden.
    pub fn new(default: Transition) -> Self {
        Self {
            default,
            per_prop: std::array::from_fn(|_| None),
        }
    }

    /// Overrides the transition for a specific property.
    pub fn with(mut self, prop: MotionProp, transition: Transition) -> Self {
        self.per_prop[prop.index()] = Some(transition);
        self
    }

    /// Resolves the transition to use for `prop`.
    pub fn for_prop(&self, prop: MotionProp) -> Transition {
        self.per_prop[prop.index()]
            .clone()
            .unwrap_or_else(|| self.default.clone())
    }
}

impl Default for TransitionMap {
    fn default() -> Self {
        Self::new(Transition::default())
    }
}

impl From<Transition> for TransitionMap {
    fn from(transition: Transition) -> Self {
        Self::new(transition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_map_uses_default_and_override() {
        let map = TransitionMap::new(Transition::spring()).with(
            MotionProp::Opacity,
            Transition::tween(0.2, Easing::EaseOut).unwrap(),
        );

        match map.for_prop(MotionProp::Opacity) {
            Transition::Tween { duration, .. } => assert_eq!(duration, 0.2),
            _ => panic!("expected tween"),
        }

        match map.for_prop(MotionProp::X) {
            Transition::Spring { .. } => {}
            _ => panic!("expected spring"),
        }
    }

    #[test]
    fn spring_custom_rejects_invalid_inputs() {
        assert_eq!(
            Transition::spring_custom(f64::NAN, 26.0, 1.0).unwrap_err(),
            TransitionError::StiffnessNotFinite
        );
        assert_eq!(
            Transition::spring_custom(170.0, -1.0, 1.0).unwrap_err(),
            TransitionError::DampingNegative
        );
        assert_eq!(
            Transition::spring_custom(170.0, 26.0, 0.0).unwrap_err(),
            TransitionError::MassNonPositive
        );
    }

    #[test]
    fn tween_rejects_invalid_inputs() {
        assert_eq!(
            Transition::tween(f64::NAN, Easing::Linear).unwrap_err(),
            TransitionError::DurationNotFinite
        );
        assert_eq!(
            Transition::tween(-0.1, Easing::Linear).unwrap_err(),
            TransitionError::DurationNegative
        );
    }
}

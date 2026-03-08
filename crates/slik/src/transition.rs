use crate::driver::{Keyframe, KeyframeError, KeyframeTransition};
use crate::easing::Easing;
use crate::style::MotionProp;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Transition {
    Spring {
        stiffness: f64,
        damping: f64,
        mass: f64,
    },
    Tween {
        duration: f64,
        easing: Easing,
    },
    Keyframes(KeyframeTransition),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    StiffnessNotFinite,
    StiffnessNegative,
    DampingNotFinite,
    DampingNegative,
    MassNotFinite,
    MassNonPositive,
    DurationNotFinite,
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
    pub fn spring() -> Self {
        Self::Spring {
            stiffness: 170.0,
            damping: 26.0,
            mass: 1.0,
        }
    }

    pub fn spring_bouncy() -> Self {
        Self::Spring {
            stiffness: 120.0,
            damping: 14.0,
            mass: 1.0,
        }
    }

    pub fn spring_gentle() -> Self {
        Self::Spring {
            stiffness: 170.0,
            damping: 60.0,
            mass: 1.0,
        }
    }

    pub fn spring_custom(stiffness: f64, damping: f64, mass: f64) -> Result<Self, TransitionError> {
        validate_spring(stiffness, damping, mass)?;
        Ok(Self::Spring {
            stiffness,
            damping,
            mass,
        })
    }

    pub fn tween(duration_secs: f64, easing: Easing) -> Result<Self, TransitionError> {
        validate_tween(duration_secs)?;
        Ok(Self::Tween {
            duration: duration_secs,
            easing,
        })
    }

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

#[derive(Debug, Clone)]
pub struct TransitionMap {
    default: Transition,
    per_prop: [Option<Transition>; MotionProp::COUNT],
}

impl TransitionMap {
    pub fn new(default: Transition) -> Self {
        Self {
            default,
            per_prop: std::array::from_fn(|_| None),
        }
    }

    pub fn with(mut self, prop: MotionProp, transition: Transition) -> Self {
        self.per_prop[prop.index()] = Some(transition);
        self
    }

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

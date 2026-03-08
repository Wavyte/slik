use crate::driver::{Keyframe, KeyframeError, KeyframeTransition};
use crate::easing::Easing;
use crate::style::MotionProp;

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

    pub fn spring_custom(stiffness: f64, damping: f64, mass: f64) -> Self {
        Self::Spring {
            stiffness,
            damping,
            mass,
        }
    }

    pub fn tween(duration_secs: f64, easing: Easing) -> Self {
        Self::Tween {
            duration: duration_secs,
            easing,
        }
    }

    pub fn keyframes(keyframes: Vec<Keyframe>, duration_secs: f64) -> Result<Self, KeyframeError> {
        Ok(Self::Keyframes(KeyframeTransition::new(
            keyframes,
            duration_secs,
        )?))
    }
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
        let map = TransitionMap::new(Transition::spring())
            .with(MotionProp::Opacity, Transition::tween(0.2, Easing::EaseOut));

        match map.for_prop(MotionProp::Opacity) {
            Transition::Tween { duration, .. } => assert_eq!(duration, 0.2),
            _ => panic!("expected tween"),
        }

        match map.for_prop(MotionProp::X) {
            Transition::Spring { .. } => {}
            _ => panic!("expected spring"),
        }
    }
}

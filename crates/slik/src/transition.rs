#[cfg(not(target_arch = "wasm32"))]
use crate::driver::KeyframeTransition;
#[cfg(target_arch = "wasm32")]
use crate::driver::{Driver, KeyframeTransition, SpringDriver, TweenDriver};
use crate::driver::{Keyframe, KeyframeError};
use crate::easing::Easing;
use crate::props::MotionProp;

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

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn create_driver(&self) -> Box<dyn Driver> {
        match self {
            Self::Spring {
                stiffness,
                damping,
                mass,
            } => Box::new(SpringDriver::new(*stiffness, *damping, *mass)),
            Self::Tween { duration, easing } => Box::new(TweenDriver::new(*duration, *easing)),
            Self::Keyframes(transition) => {
                Box::new(crate::driver::KeyframeDriver::new(transition.clone()))
            }
        }
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::spring()
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransitionConfig {
    pub default: Transition,
    pub opacity: Option<Transition>,
    pub x: Option<Transition>,
    pub y: Option<Transition>,
    pub scale: Option<Transition>,
    pub scale_x: Option<Transition>,
    pub scale_y: Option<Transition>,
    pub rotate: Option<Transition>,
}

impl TransitionConfig {
    pub fn new(default: Transition) -> Self {
        Self {
            default,
            ..Default::default()
        }
    }

    pub fn with(mut self, prop: MotionProp, transition: Transition) -> Self {
        match prop {
            MotionProp::Opacity => self.opacity = Some(transition),
            MotionProp::X => self.x = Some(transition),
            MotionProp::Y => self.y = Some(transition),
            MotionProp::Scale => self.scale = Some(transition),
            MotionProp::ScaleX => self.scale_x = Some(transition),
            MotionProp::ScaleY => self.scale_y = Some(transition),
            MotionProp::Rotate => self.rotate = Some(transition),
        }
        self
    }

    pub fn for_prop(&self, prop: MotionProp) -> Transition {
        let specific = match prop {
            MotionProp::Opacity => &self.opacity,
            MotionProp::X => &self.x,
            MotionProp::Y => &self.y,
            MotionProp::Scale => &self.scale,
            MotionProp::ScaleX => &self.scale_x,
            MotionProp::ScaleY => &self.scale_y,
            MotionProp::Rotate => &self.rotate,
        };

        specific.clone().unwrap_or_else(|| self.default.clone())
    }
}

impl From<Transition> for TransitionConfig {
    fn from(transition: Transition) -> Self {
        Self::new(transition)
    }
}

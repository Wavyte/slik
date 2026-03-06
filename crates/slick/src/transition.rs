use crate::driver::{Driver, Keyframe, KeyframeDriver, SpringDriver, TweenDriver};
use crate::easing::Easing;

// ---------------------------------------------------------------------------
// Transition — user-facing driver factory
// ---------------------------------------------------------------------------

/// Describes *how* a value should animate between states.
///
/// Each variant configures a different [`Driver`] strategy:
/// - **Spring**: physics-based, no fixed duration, velocity preservation on interrupt.
/// - **Tween**: duration-based with a single cubic bézier easing.
/// - **Keyframes**: multi-segment duration-based with per-segment bezier easing.
#[derive(Debug, Clone)]
pub enum Transition {
    Spring {
        stiffness: f64,
        damping: f64,
        mass: f64,
    },
    Tween {
        /// Duration in seconds.
        duration: f64,
        easing: Easing,
    },
    Keyframes {
        keyframes: Vec<Keyframe>,
        /// Total duration in seconds.
        duration: f64,
    },
}

impl Transition {
    // -- Convenience constructors ------------------------------------------

    /// Default spring: critically damped, snappy feel.
    pub fn spring() -> Self {
        Transition::Spring {
            stiffness: 170.0,
            damping: 26.0,
            mass: 1.0,
        }
    }

    /// Bouncy spring preset.
    pub fn spring_bouncy() -> Self {
        Transition::Spring {
            stiffness: 120.0,
            damping: 14.0,
            mass: 1.0,
        }
    }

    /// Gentle (overdamped) spring preset.
    pub fn spring_gentle() -> Self {
        Transition::Spring {
            stiffness: 170.0,
            damping: 60.0,
            mass: 1.0,
        }
    }

    /// Custom spring.
    pub fn spring_custom(stiffness: f64, damping: f64, mass: f64) -> Self {
        Transition::Spring {
            stiffness,
            damping,
            mass,
        }
    }

    /// Duration-based tween with the given easing.
    pub fn tween(duration_secs: f64, easing: Easing) -> Self {
        Transition::Tween {
            duration: duration_secs,
            easing,
        }
    }

    /// Multi-segment keyframe animation.
    pub fn keyframes(keyframes: Vec<Keyframe>, duration_secs: f64) -> Self {
        Transition::Keyframes {
            keyframes,
            duration: duration_secs,
        }
    }

    // -- Factory -----------------------------------------------------------

    /// Instantiate a boxed [`Driver`] configured by this transition.
    pub(crate) fn create_driver(&self) -> Box<dyn Driver> {
        match self {
            Transition::Spring {
                stiffness,
                damping,
                mass,
            } => Box::new(SpringDriver::new(*stiffness, *damping, *mass)),
            Transition::Tween { duration, easing } => {
                Box::new(TweenDriver::new(*duration, *easing))
            }
            Transition::Keyframes {
                keyframes,
                duration,
            } => Box::new(KeyframeDriver::new(keyframes.clone(), *duration)),
        }
    }
}

// Default to snappy spring
impl Default for Transition {
    fn default() -> Self {
        Transition::spring()
    }
}

// ---------------------------------------------------------------------------
// TransitionConfig — per-property overrides
// ---------------------------------------------------------------------------

/// Allows specifying a different [`Transition`] per animated property,
/// with a fallback default for unlisted properties.
///
/// ```ignore
/// TransitionConfig::new(Transition::spring())
///     .with("opacity", Transition::tween(0.3, Easing::EaseOut))
///     .with("x", Transition::spring_bouncy())
/// ```
#[derive(Debug, Clone, Default)]
pub struct TransitionConfig {
    /// Fallback used for any property without an explicit override.
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

    /// Set an override for a named property. Returns `self` for chaining.
    pub fn with(mut self, prop: &str, transition: Transition) -> Self {
        match prop {
            "opacity" => self.opacity = Some(transition),
            "x" => self.x = Some(transition),
            "y" => self.y = Some(transition),
            "scale" => self.scale = Some(transition),
            "scale_x" | "scaleX" => self.scale_x = Some(transition),
            "scale_y" | "scaleY" => self.scale_y = Some(transition),
            "rotate" => self.rotate = Some(transition),
            _ => {} // unknown properties silently ignored in v0.1
        }
        self
    }

    /// Resolve the transition for a given property name.
    pub fn for_prop(&self, prop: &str) -> Transition {
        let specific = match prop {
            "opacity" => &self.opacity,
            "x" => &self.x,
            "y" => &self.y,
            "scale" => &self.scale,
            "scale_x" | "scaleX" => &self.scale_x,
            "scale_y" | "scaleY" => &self.scale_y,
            "rotate" => &self.rotate,
            _ => &None,
        };
        specific.clone().unwrap_or_else(|| self.default.clone())
    }
}

impl From<Transition> for TransitionConfig {
    fn from(t: Transition) -> Self {
        Self::new(t)
    }
}

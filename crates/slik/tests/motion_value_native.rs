use leptos::prelude::*;
use slik::prelude::*;

#[test]
fn motion_value_snaps_immediately_on_non_wasm() {
    Owner::new().with(|| {
        let value = MotionValue::new(0.0, Transition::spring());
        value.set_target(24.0);

        assert_eq!(value.get_untracked(), 24.0);
        assert_eq!(value.target(), 24.0);
    });
}

#[test]
fn motion_value_jump_and_stop_are_stable() {
    Owner::new().with(|| {
        let value = MotionValue::new(5.0, Transition::spring());
        value.jump(11.0);
        assert_eq!(value.get_untracked(), 11.0);
        assert_eq!(value.target(), 11.0);

        value.set_target(18.0);
        value.stop();
        assert_eq!(value.get_untracked(), 18.0);
        assert_eq!(value.target(), 18.0);
    });
}

#[test]
fn invalid_transition_variants_degrade_safely() {
    Owner::new().with(|| {
        let value = MotionValue::new(
            0.0,
            Transition::Tween {
                duration: f64::NAN,
                easing: Easing::Linear,
            },
        );

        value.set_target(10.0);

        assert_eq!(value.get_untracked(), 10.0);
        assert_eq!(value.target(), 10.0);
        assert!(!value.is_animating_untracked());
    });
}

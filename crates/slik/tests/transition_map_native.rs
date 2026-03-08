use slik::prelude::*;

#[test]
fn transition_map_resolves_override_and_default() {
    let map = TransitionMap::new(Transition::spring())
        .with(MotionProp::Opacity, Transition::tween(0.2, Easing::EaseOut));

    match map.for_prop(MotionProp::Opacity) {
        Transition::Tween { duration, .. } => assert_eq!(duration, 0.2),
        other => panic!("expected opacity tween, got {other:?}"),
    }

    match map.for_prop(MotionProp::Rotate) {
        Transition::Spring { .. } => {}
        other => panic!("expected default spring, got {other:?}"),
    }
}

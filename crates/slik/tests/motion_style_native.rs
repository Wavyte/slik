use slik::prelude::*;

#[test]
fn motion_style_builder_and_contains_work() {
    let style = MotionStyle::new().opacity(0.4).x(12.0).rotate(90.0);

    assert_eq!(style.get(MotionProp::Opacity), Some(0.4));
    assert_eq!(style.get(MotionProp::X), Some(12.0));
    assert_eq!(style.get(MotionProp::Rotate), Some(90.0));
    assert!(style.contains(MotionProp::Opacity));
    assert!(!style.contains(MotionProp::Scale));
}

#[test]
fn motion_style_value_or_default_falls_back_to_prop_default() {
    let style = MotionStyle::new().x(20.0);

    assert_eq!(style.value_or_default(MotionProp::X), 20.0);
    assert_eq!(style.value_or_default(MotionProp::Opacity), 1.0);
    assert_eq!(style.value_or_default(MotionProp::Scale), 1.0);
}

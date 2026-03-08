//! Motion style types.

/// Supported motion properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotionProp {
    /// Maps to CSS `opacity`.
    Opacity,
    /// Maps to `translateX(...)`.
    X,
    /// Maps to `translateY(...)`.
    Y,
    /// Maps to uniform scale.
    Scale,
    /// Multiplies the x-axis scale.
    ScaleX,
    /// Multiplies the y-axis scale.
    ScaleY,
    /// Maps to `rotate(...)`.
    Rotate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    None,
    Px,
    Deg,
}

#[derive(Debug, Clone, Copy)]
struct PropMeta {
    default: f64,
    unit: Unit,
    css_name: &'static str,
}

const PROP_META: [PropMeta; MotionProp::COUNT] = [
    PropMeta {
        default: 1.0,
        unit: Unit::None,
        css_name: "opacity",
    },
    PropMeta {
        default: 0.0,
        unit: Unit::Px,
        css_name: "translateX",
    },
    PropMeta {
        default: 0.0,
        unit: Unit::Px,
        css_name: "translateY",
    },
    PropMeta {
        default: 1.0,
        unit: Unit::None,
        css_name: "scale",
    },
    PropMeta {
        default: 1.0,
        unit: Unit::None,
        css_name: "scaleX",
    },
    PropMeta {
        default: 1.0,
        unit: Unit::None,
        css_name: "scaleY",
    },
    PropMeta {
        default: 0.0,
        unit: Unit::Deg,
        css_name: "rotate",
    },
];

impl MotionProp {
    /// Number of supported motion properties.
    pub const COUNT: usize = 7;
    /// All supported properties in dense-storage order.
    pub const ALL: [Self; Self::COUNT] = [
        Self::Opacity,
        Self::X,
        Self::Y,
        Self::Scale,
        Self::ScaleX,
        Self::ScaleY,
        Self::Rotate,
    ];

    /// Returns this property's dense-storage index.
    pub const fn index(self) -> usize {
        match self {
            Self::Opacity => 0,
            Self::X => 1,
            Self::Y => 2,
            Self::Scale => 3,
            Self::ScaleX => 4,
            Self::ScaleY => 5,
            Self::Rotate => 6,
        }
    }

    /// Returns the property's default value when it is not explicitly present.
    pub const fn default_value(self) -> f64 {
        PROP_META[self.index()].default
    }
    const fn unit(self) -> Unit {
        PROP_META[self.index()].unit
    }
}

/// Sparse, builder-style motion target definition.
///
/// A `MotionStyle` only stores properties that are explicitly present. Missing
/// properties are treated as absent, not as implicit resets.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotionStyle {
    values: [Option<f64>; MotionProp::COUNT],
}

impl MotionStyle {
    /// Creates an empty motion style.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a specific motion property.
    pub fn set(mut self, prop: MotionProp, value: f64) -> Self {
        self.values[prop.index()] = Some(value);
        self
    }

    /// Returns the explicitly stored value for `prop`, if present.
    pub fn get(&self, prop: MotionProp) -> Option<f64> {
        self.values[prop.index()]
    }

    /// Returns `true` when `prop` is explicitly present in the style.
    pub fn contains(&self, prop: MotionProp) -> bool {
        self.get(prop).is_some()
    }

    /// Returns the explicit value for `prop`, or the property's default.
    pub fn value_or_default(&self, prop: MotionProp) -> f64 {
        self.get(prop).unwrap_or_else(|| prop.default_value())
    }

    /// Iterates over explicitly present properties in dense-storage order.
    pub fn iter_present(&self) -> impl Iterator<Item = (MotionProp, f64)> + '_ {
        MotionProp::ALL
            .into_iter()
            .filter_map(|prop| self.get(prop).map(|value| (prop, value)))
    }

    /// Sets [`MotionProp::Opacity`].
    pub fn opacity(self, v: f64) -> Self {
        self.set(MotionProp::Opacity, v)
    }

    /// Sets [`MotionProp::X`].
    pub fn x(self, v: f64) -> Self {
        self.set(MotionProp::X, v)
    }

    /// Sets [`MotionProp::Y`].
    pub fn y(self, v: f64) -> Self {
        self.set(MotionProp::Y, v)
    }

    /// Sets [`MotionProp::Scale`].
    pub fn scale(self, v: f64) -> Self {
        self.set(MotionProp::Scale, v)
    }

    /// Sets [`MotionProp::ScaleX`].
    pub fn scale_x(self, v: f64) -> Self {
        self.set(MotionProp::ScaleX, v)
    }

    /// Sets [`MotionProp::ScaleY`].
    pub fn scale_y(self, v: f64) -> Self {
        self.set(MotionProp::ScaleY, v)
    }

    /// Sets [`MotionProp::Rotate`].
    pub fn rotate(self, v: f64) -> Self {
        self.set(MotionProp::Rotate, v)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ComposedDomStyle {
    pub opacity: Option<String>,
    pub transform: Option<String>,
    pub will_change: Option<String>,
}

#[inline]
pub(crate) const fn prop_bit(prop: MotionProp) -> u16 {
    1_u16 << prop.index()
}

#[inline]
pub(crate) const fn owns_prop(mask: u16, prop: MotionProp) -> bool {
    (mask & prop_bit(prop)) != 0
}

pub(crate) fn mask_for_style(style: &MotionStyle) -> u16 {
    let mut mask = 0;
    for prop in MotionProp::ALL {
        if style.contains(prop) {
            mask |= prop_bit(prop);
        }
    }
    mask
}

pub(crate) fn compose_dom_style(
    owned_mask: u16,
    active_mask: u16,
    values: &[f64; MotionProp::COUNT],
) -> ComposedDomStyle {
    let opacity = if owns_prop(owned_mask, MotionProp::Opacity) {
        Some(format_value(
            MotionProp::Opacity,
            values[MotionProp::Opacity.index()],
        ))
    } else {
        None
    };

    let mut transforms = Vec::with_capacity(4);

    if owns_prop(owned_mask, MotionProp::X) {
        transforms.push(format!(
            "{}({})",
            PROP_META[MotionProp::X.index()].css_name,
            format_value(MotionProp::X, values[MotionProp::X.index()])
        ));
    }
    if owns_prop(owned_mask, MotionProp::Y) {
        transforms.push(format!(
            "{}({})",
            PROP_META[MotionProp::Y.index()].css_name,
            format_value(MotionProp::Y, values[MotionProp::Y.index()])
        ));
    }
    if owns_prop(owned_mask, MotionProp::Scale)
        || owns_prop(owned_mask, MotionProp::ScaleX)
        || owns_prop(owned_mask, MotionProp::ScaleY)
    {
        let scale = values[MotionProp::Scale.index()];
        let scale_x = if owns_prop(owned_mask, MotionProp::ScaleX) {
            values[MotionProp::ScaleX.index()]
        } else {
            1.0
        };
        let scale_y = if owns_prop(owned_mask, MotionProp::ScaleY) {
            values[MotionProp::ScaleY.index()]
        } else {
            1.0
        };
        transforms.push(format!("scale({}, {})", scale * scale_x, scale * scale_y));
    }
    if owns_prop(owned_mask, MotionProp::Rotate) {
        transforms.push(format!(
            "{}({})",
            PROP_META[MotionProp::Rotate.index()].css_name,
            format_value(MotionProp::Rotate, values[MotionProp::Rotate.index()])
        ));
    }

    let transform = if transforms.is_empty() {
        None
    } else {
        Some(transforms.join(" "))
    };

    let mut will_change = Vec::with_capacity(2);
    if owns_prop(active_mask, MotionProp::Opacity) {
        will_change.push("opacity");
    }
    if transform_group_active(active_mask) {
        will_change.push("transform");
    }

    ComposedDomStyle {
        opacity,
        transform,
        will_change: if will_change.is_empty() {
            None
        } else {
            Some(will_change.join(", "))
        },
    }
}

fn transform_group_active(mask: u16) -> bool {
    owns_prop(mask, MotionProp::X)
        || owns_prop(mask, MotionProp::Y)
        || owns_prop(mask, MotionProp::Scale)
        || owns_prop(mask, MotionProp::ScaleX)
        || owns_prop(mask, MotionProp::ScaleY)
        || owns_prop(mask, MotionProp::Rotate)
}

fn format_value(prop: MotionProp, value: f64) -> String {
    match prop.unit() {
        Unit::None => value.to_string(),
        Unit::Px => format!("{value}px"),
        Unit::Deg => format!("{value}deg"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_builders_write_dense_storage() {
        let style = MotionStyle::new().opacity(0.5).x(12.0).rotate(45.0);
        assert_eq!(style.get(MotionProp::Opacity), Some(0.5));
        assert_eq!(style.get(MotionProp::X), Some(12.0));
        assert_eq!(style.get(MotionProp::Rotate), Some(45.0));
        assert!(!style.contains(MotionProp::Scale));
    }

    #[test]
    fn mask_tracks_owned_props() {
        let style = MotionStyle::new().opacity(0.5).scale(1.2);
        let mask = mask_for_style(&style);
        assert!(owns_prop(mask, MotionProp::Opacity));
        assert!(owns_prop(mask, MotionProp::Scale));
        assert!(!owns_prop(mask, MotionProp::Rotate));
    }

    #[test]
    fn compose_dom_style_uses_fixed_transform_order() {
        let mut values = [0.0; MotionProp::COUNT];
        values[MotionProp::Opacity.index()] = 0.8;
        values[MotionProp::X.index()] = 10.0;
        values[MotionProp::Y.index()] = -3.0;
        values[MotionProp::Scale.index()] = 1.2;
        values[MotionProp::ScaleX.index()] = 0.5;
        values[MotionProp::Rotate.index()] = 90.0;
        let mask = prop_bit(MotionProp::Opacity)
            | prop_bit(MotionProp::X)
            | prop_bit(MotionProp::Y)
            | prop_bit(MotionProp::Scale)
            | prop_bit(MotionProp::ScaleX)
            | prop_bit(MotionProp::Rotate);

        let style = compose_dom_style(mask, mask, &values);

        assert_eq!(style.opacity.as_deref(), Some("0.8"));
        assert_eq!(
            style.transform.as_deref(),
            Some("translateX(10px) translateY(-3px) scale(0.6, 1.2) rotate(90deg)")
        );
        assert_eq!(style.will_change.as_deref(), Some("opacity, transform"));
    }

    #[test]
    fn compose_dom_style_only_marks_active_groups_for_will_change() {
        let mut values = [0.0; MotionProp::COUNT];
        values[MotionProp::Opacity.index()] = 0.8;
        values[MotionProp::X.index()] = 10.0;
        let owned_mask = prop_bit(MotionProp::Opacity) | prop_bit(MotionProp::X);
        let active_mask = prop_bit(MotionProp::X);

        let style = compose_dom_style(owned_mask, active_mask, &values);

        assert_eq!(style.opacity.as_deref(), Some("0.8"));
        assert_eq!(style.transform.as_deref(), Some("translateX(10px)"));
        assert_eq!(style.will_change.as_deref(), Some("transform"));
    }
}

/// Bag of animatable CSS properties for the `<Motion>` component.
///
/// Only populated fields participate in animation.
/// Fields left as `None` are not touched — the element keeps its natural style.
///
/// ```ignore
/// AnimProps::new().opacity(0.0).x(-20.0).scale(0.9)
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimProps {
    pub opacity: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub scale: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub rotate: Option<f64>,
}

impl AnimProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn opacity(mut self, v: f64) -> Self {
        self.opacity = Some(v);
        self
    }
    pub fn x(mut self, v: f64) -> Self {
        self.x = Some(v);
        self
    }
    pub fn y(mut self, v: f64) -> Self {
        self.y = Some(v);
        self
    }
    pub fn scale(mut self, v: f64) -> Self {
        self.scale = Some(v);
        self
    }
    pub fn scale_x(mut self, v: f64) -> Self {
        self.scale_x = Some(v);
        self
    }
    pub fn scale_y(mut self, v: f64) -> Self {
        self.scale_y = Some(v);
        self
    }
    pub fn rotate(mut self, v: f64) -> Self {
        self.rotate = Some(v);
        self
    }

    /// Resolve the initial value for a property, falling back to sensible CSS defaults.
    pub fn get_or_default(&self, prop: &str) -> f64 {
        match prop {
            "opacity" => self.opacity.unwrap_or(1.0),
            "x" => self.x.unwrap_or(0.0),
            "y" => self.y.unwrap_or(0.0),
            "scale" => self.scale.unwrap_or(1.0),
            "scale_x" => self.scale_x.unwrap_or(1.0),
            "scale_y" => self.scale_y.unwrap_or(1.0),
            "rotate" => self.rotate.unwrap_or(0.0),
            _ => 0.0,
        }
    }

    /// Merge two prop bags: `other` takes precedence where defined.
    pub fn merge(&self, other: &AnimProps) -> AnimProps {
        AnimProps {
            opacity: other.opacity.or(self.opacity),
            x: other.x.or(self.x),
            y: other.y.or(self.y),
            scale: other.scale.or(self.scale),
            scale_x: other.scale_x.or(self.scale_x),
            scale_y: other.scale_y.or(self.scale_y),
            rotate: other.rotate.or(self.rotate),
        }
    }

    /// Returns an iterator of `(prop_name, value)` for all defined properties.
    pub fn defined_props(&self) -> Vec<(&'static str, f64)> {
        let mut out = Vec::new();
        if let Some(v) = self.opacity { out.push(("opacity", v)); }
        if let Some(v) = self.x { out.push(("x", v)); }
        if let Some(v) = self.y { out.push(("y", v)); }
        if let Some(v) = self.scale { out.push(("scale", v)); }
        if let Some(v) = self.scale_x { out.push(("scale_x", v)); }
        if let Some(v) = self.scale_y { out.push(("scale_y", v)); }
        if let Some(v) = self.rotate { out.push(("rotate", v)); }
        out
    }
}

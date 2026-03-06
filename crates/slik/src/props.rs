#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotionProp {
    Opacity,
    X,
    Y,
    Scale,
    ScaleX,
    ScaleY,
    Rotate,
}

impl MotionProp {
    pub const ALL: [Self; 7] = [
        Self::Opacity,
        Self::X,
        Self::Y,
        Self::Scale,
        Self::ScaleX,
        Self::ScaleY,
        Self::Rotate,
    ];

    pub const fn default_value(self) -> f64 {
        match self {
            Self::Opacity => 1.0,
            Self::X | Self::Y | Self::Rotate => 0.0,
            Self::Scale | Self::ScaleX | Self::ScaleY => 1.0,
        }
    }
}

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

    pub fn value_for(&self, prop: MotionProp) -> Option<f64> {
        match prop {
            MotionProp::Opacity => self.opacity,
            MotionProp::X => self.x,
            MotionProp::Y => self.y,
            MotionProp::Scale => self.scale,
            MotionProp::ScaleX => self.scale_x,
            MotionProp::ScaleY => self.scale_y,
            MotionProp::Rotate => self.rotate,
        }
    }

    pub fn get_or_default(&self, prop: MotionProp) -> f64 {
        self.value_for(prop).unwrap_or_else(|| prop.default_value())
    }
}

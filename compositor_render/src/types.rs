use std::{collections::HashMap, fmt::Display, sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct Frame {
    pub data: YuvData,
    pub resolution: Resolution,
    pub pts: Duration,
}

#[derive(Debug, Clone)]
pub struct YuvData {
    pub y_plane: bytes::Bytes,
    pub u_plane: bytes::Bytes,
    pub v_plane: bytes::Bytes,
}

#[derive(Debug)]
pub struct FrameSet<Id>
where
    Id: From<Arc<str>>,
{
    pub frames: HashMap<Id, Frame>,
    pub pts: Duration,
}

impl<Id> FrameSet<Id>
where
    Id: From<Arc<str>>,
{
    pub fn new(pts: Duration) -> Self {
        FrameSet {
            frames: HashMap::new(),
            pts,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Framerate {
    pub num: u32,
    pub den: u32,
}

impl Framerate {
    pub fn get_interval_duration(self) -> Duration {
        Duration::from_nanos(1_000_000_000u64 * self.den as u64 / self.num as u64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RendererId(pub Arc<str>);

impl Display for RendererId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InputId(pub Arc<str>);

impl Display for InputId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Arc<str>> for InputId {
    fn from(value: Arc<str>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OutputId(pub Arc<str>);

impl Display for OutputId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Arc<str>> for OutputId {
    fn from(value: Arc<str>) -> Self {
        Self(value)
    }
}

pub const MAX_NODE_RESOLUTION: Resolution = Resolution {
    width: 7682,
    height: 4320,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

impl Resolution {
    pub fn ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

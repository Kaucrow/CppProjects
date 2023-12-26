use std::ops::Index;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dither {
    Normal,
    Light,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Status {
    Index(IndexType),
    Ready,
    Done,
}

pub type IndexType = u8;

pub struct Image {
    pub normal: Vec<String>,
    pub light: Vec<String>,
    pub shift: Vec<String>,
    pub light_shift: Vec<String>,
    pub wave_offset: Vec<i8>,
    pub mask: Vec<(Dither, Status)>,
}

impl Image {
    pub fn new<const U: usize>(mask: [Dither; U]) -> Self {
        Self {
            normal: Vec::new(),
            light: Vec::new(),
            shift: Vec::new(),
            light_shift: Vec::new(),
            wave_offset: Vec::new(),
            mask: mask.into_iter().map(|dither| (dither, Status::Ready)).collect(),
        }
    }
}
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

pub struct Ferris {
    pub normal: Vec<String>,
    pub light: Vec<String>,
    //pub to_draw: BTreeMap<Index, Dither>,
    pub mask: Vec<(Dither, Status)>
}

impl Ferris {
    pub fn new<const U: usize>(mask: [Dither; U]) -> Self {
        Self {
            normal: Vec::new(),
            light: Vec::new(),
            mask: mask.into_iter().map(|dither| (dither, Status::Ready)).collect(),
        }
    }
}
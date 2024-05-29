use crate::pts_loader::define::Define;

#[derive(Clone)]
pub enum Block<'a> {
    Begin { index: usize, event: &'a Define },
    End { index: usize, event: &'a Define },
}

impl<'a> Block<'a> {
    pub fn index(&self) -> usize {
        match self {
            Block::Begin { index, .. } => *index,
            Block::End { index, .. } => *index,
        }
    }
    pub fn event(&self) -> &'a Define {
        match self {
            Block::Begin { event, .. } => event,
            Block::End { event, .. } => event,
        }
    }

    pub fn is_begin(&self) -> bool {
        match self {
            Block::Begin { .. } => true,
            _ => false,
        }
    }

    pub fn is_end(&self) -> bool {
        match self {
            Block::Begin { .. } => false,
            _ => true,
        }
    }
}

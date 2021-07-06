
use std::ops::Range;

/// The affinity of a cursor on a line break
pub enum Affinity {
    Downstream,
    Upstream,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Selection {
    pub anchor: usize,
    pub active: usize,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            anchor: 0,
            active: 0,
        }
    }

    pub fn carret(carret_position: usize) -> Self {
        Self {
            anchor: carret_position,
            active: carret_position,
        }
    }

    pub fn min(&self) -> usize {
        self.anchor.min(self.active)
    }

    pub fn max(&self) -> usize {
        self.anchor.max(self.active)
    }

    pub fn len(&self) -> usize {
        self.max() - self.min()
    }

    pub fn range(&self) -> Range<usize> {
        self.min()..self.max()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CursorIndex {
    line: usize,
    character: usize,
}

impl CursorIndex {
    pub fn next(self, lines: Vec<std::ops::Range<usize>>) -> Option<Self> {
        
        //let index = self;
        
        lines.iter().nth(self.line).and_then(|rng| {
            if self.character >= rng.clone().count() {
                Some(CursorIndex {
                    line: self.line + 1,
                    character: 0,
                })
            } else {
                Some(CursorIndex {
                    line: self.line,
                    character: self.character + 1,
                })
            }
        })
    }
}

pub enum Cursor {
    Index(CursorIndex),
    Selection {
        start: CursorIndex,
        end: CursorIndex,
    }
}

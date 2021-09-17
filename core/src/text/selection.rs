
// Adapted from xi-editor

// Copyright 2017 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// The affinity of a cursor on a line break
pub enum Affinity {
    Downstream,
    Upstream,
}

pub struct SelectionRegion {
    pub start: usize,
    pub end: usize,
    pub affinity: Affinity,
}

pub struct Selection {
    regions: Vec<SelectionRegion>,
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

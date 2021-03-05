
struct TextPos {
    row: u32,
    col: u32,
}

impl TextPos {
    pub fn new(row: u32, col: u32) -> Self {
        Self {
            row,
            col, 
        }
    }
}


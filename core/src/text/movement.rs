pub enum TextDirection {
    LeftToRight,
    Downstream,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MovementOperation {

    NoMove,

    /// Move to the start of a document
    Start,

    /// Move to the start of the current line
    StartOfLine,

    /// Move to the start of the current block
    StartOfBlock,

    /// Move bytewise backwards by one grapheme cluster. 
    /// For left-to-right text this is equivalent to moving left by one grapheme cluster.
    Left,

    /// Move bytewise forwards by one grapheme cluster. 
    /// For left-to-right text this is equivalent to moving right by one grapheme cluster.
    Right,

    /// Move bytewise backwards by one word.
    Up,

    /// Move to the right by one word.
    Down,

    /// Move to start of visible line.
    UpstreamLine,

    /// Move to end of visible line.
    DownstreamLine,

    /// Move up one visible line.
    UpLine,

    /// Move down one visible line.
    DownLine,

    /// Move up one viewport height.
    UpPage,

    /// Move down one viewport height.
    DownPage,

    /// Move to the start of the document.
    StartOfDocument,

    /// Move to the end of the document
    EndOfDocument,


}


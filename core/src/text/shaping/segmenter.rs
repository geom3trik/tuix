

// Split text into runs where each run has a different script or orientation

/// The script of the text run
enum Script {

}

/// The orientation of the text run
enum Orientation {
    Horizontal,
    Vertical,
}

/// Represents a run of text with a particular script and orientation
struct Run {
    /// The range of characters of the original text
    range: Range<usize>,
    /// The script of this run
    script: Script,
    /// The orientation of this run 
    orientation: Orientation, 
}

/// Splits the input text into runs
fn split_runs() -> Vec<Run> {
    
}
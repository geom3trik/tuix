pub enum Token<'a> {
    Ident(&'a str),
    Delim(char),
    Hash(&'a str),
    WhiteSpace(&'a str),
    Number(f32),
    Percentage(f32),
    Dimension(f32, &'a str),
    Colon,
    Comma,
    OpenCurlyBracket,
    CloseCurlyBracket,
}
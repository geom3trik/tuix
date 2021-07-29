use tuix::*;

const PSEUDOCLASS_ERROR: &str = r#"
    
    button {
        background-color: red;
    }

    button:hver {
        background-color: blue;
    }
"#;

#[test]
fn pseudoclass_error() {
    let mut state = State::new();
    state.add_theme(PSEUDOCLASS_ERROR);
}



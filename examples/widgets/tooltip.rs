use tuix::*;


const STYLE: &str = r#"

    tooltip {

    }

"#;


fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window|{
        
        // Basic Tooltip
        let first = Button::with_label("Hover Me").build(state, window, |builder| 
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .set_tooltip(|state, tooltip| {
                    // Basic tooltip
                    Tooltip::new("Basic Tooltip").build(state, tooltip, |builder| builder);
                })
        );

        // Complex Tooltip
        let second = Button::with_label("Hover Me").build(state, window, |builder| 
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .set_tooltip(|state, tooltip| {
                    // More complex tooltip made from other widgets
                    Label::new("Title").build(state, tooltip, |builder| builder);
                    Label::new("Description Text").build(state, tooltip, |builder| builder);
                })
        );
    });

    app.run();
}
use tuix::*;
use tuix::widgets::*;

static THEME: &'static str = include_str!("../themes/tabs_theme.css");

fn main() {
    let window_description = WindowDescription::new().with_title("tabs");
    let app = Application::new(window_description, |state, window| {
        state.add_theme(THEME);

        window
            .set_layout_type(state, LayoutType::Row);

        // Create a tab manager
        let (tab_bar1, tab_viewport1) =
            TabView::new().build(state, window.entity(), |builder| builder);

        // Add a tab to the tab bar
        let first_tab = MovableTab::new("first").build(state, tab_bar1, |builder| {
            builder.set_text("First").class("tab")
        });

        first_tab.set_checked(state, true);

        // Add a tab container to the tab manager viewport
        let first_container = TabContainer::new("first")
            .build(state, tab_viewport1, |builder| builder.class("first"));

        // Add a button to this container
        Button::with_label("First Button")
            .build(state, first_container, |builder| builder.class("test"));

        let _second_tab = MovableTab::new("second").build(state, tab_bar1, |builder| {
            builder.set_text("Second").class("tab")
        });

        let second_container = TabContainer::new("second")
            .build(state, tab_viewport1, |builder| builder.class("second"));
        second_container.set_display(state, Display::None);

        Button::with_label("Second Button")
            .build(state, second_container, |builder| builder.class("test"));

        let _third_tab = MovableTab::new("third").build(state, tab_bar1, |builder| {
            builder.set_text("Third").class("tab").set_width(Auto)
        });

        Element::new().build(state, _third_tab, |builder| builder.set_width(Pixels(80.0)).set_hoverable(false));

        let third_container = TabContainer::new("third")
            .build(state, tab_viewport1, |builder| builder.class("third"));
            third_container.set_display(state, Display::None);

        Button::with_label("Third Button")
            .build(state, third_container, |builder| builder.class("test"));
    });

    app.run()
}

extern crate tuix;

use tuix::*;

static THEME: &'static str = include_str!("themes/tabs_theme.css");

fn main() {
    let window_description = WindowDescription::new().with_title("tabs");
    let app = Application::new(window_description, |state, window| {
        state.add_theme(THEME);

        window
            .set_layout_type(state, LayoutType::Row);

        Element::new().build(state, window.entity(), |builder| {
            builder
                .set_width(Pixels(200.0))
                .set_child_space(Pixels(10.0))
        });

        // Create a tab manager
        let (tab_bar1, tab_viewport1) =
            TabView::new().build(state, window.entity(), |builder| builder);

        // Add a tab to the tab bar
        let first_tab = Tab::new("first").build(state, tab_bar1, |builder| {
            builder.set_text("First").class("tab")
        });

        first_tab.set_checked(state, true);

        // Add a tab container to the tab manager viewport
        let first_container = TabContainer::new("first")
            .build(state, tab_viewport1, |builder| builder.class("first"));

        // Add a button to this container
        Button::with_label("First Button")
            .build(state, first_container, |builder| builder.class("test"));

        let _second_tab = Tab::new("second").build(state, tab_bar1, |builder| {
            builder.set_text("Second").class("tab")
        });

        let second_container = TabContainer::new("second")
            .build(state, tab_viewport1, |builder| builder.class("second"));
        second_container.set_display(state, Display::None);

        Button::with_label("Second Button")
            .build(state, second_container, |builder| builder.class("test"));

        // Tab::new("third").build(state, tab_bar1, |builder| {
        //     builder.set_text("Third").class("tab")
        // });

        // Tab::new("fourth").build(state, tab_bar1, |builder| {
        //     builder.set_text("Fourth").class("tab")
        // });

        // Tab::new("fifth").build(state, tab_bar1, |builder| {
        //     builder.set_text("Fifth").class("tab")
        // });

        // // I hear you like tabs, so I put some tabs in your tabs
        // let more_tabs = Element::new().build(state, second_container, |builder| {
        //     builder
        //         .set_height(Stretch(1.0))
        //         .set_background_color(Color::rgb(30, 30, 30))
        //         .set_layout_type(LayoutType::Row)
        //         .set_child_top(Pixels(2.0))
        // });

        // // Create a tab manager
        // let (tab_bar2, tab_viewport2) =
        //     TabManager::new().build(state, more_tabs, |builder| builder.class("vertical"));

        // let first_tab = Tab::new("first").build(state, tab_bar2, |builder| {
        //     builder.set_text("First").class("tab")
        // });

        // first_tab.set_checked(state, true);

        // let first_container = TabContainer::new("first")
        //     .build(state, tab_viewport2, |builder| builder.class("first"));

        // // Add a button to this container
        // Button::with_label("First Button")
        //     .build(state, first_container, |builder| builder.class("test"));

        // let _second_tab = Tab::new("second").build(state, tab_bar2, |builder| {
        //     builder.set_text("Second").class("tab")
        // });

        // let second_container = TabContainer::new("second")
        //     .build(state, tab_viewport2, |builder| builder.class("second"));
        // second_container.set_display(state, Display::None);

        // Button::with_label("Second Button")
        //     .build(state, second_container, |builder| builder.class("test"));

        // // DROPDOWN
        // let (_, _, dropdown_container) =
        //     Dropdown::new("Select Tab").build(state, controls, |builder| builder);
        // let list = List::new().build(state, dropdown_container, |builder| builder);
        // Button::new()
        //     .on_press(Event::new(TabEvent::SwitchTab("first".to_string())).target(tab_bar1))
        //     .build(state, list, |builder| {
        //         builder.set_text("First").class("item")
        //     });
        // Button::new()
        //     .on_press(Event::new(TabEvent::SwitchTab("second".to_string())).target(tab_bar1))
        //     .build(state, list, |builder| {
        //         builder.set_text("Second").class("item")
        //     });
    });

    app.run()
}

// Example which shows all of the supported widgets

extern crate tuix;

use tuix::*;

use tuix::style::themes::DEFAULT_THEME;

// static THEME: &'static str = include_str!("themes/light_theme.css");

#[derive(Debug, Clone, PartialEq)]
enum TestEvent {
    SomethingChanged(f32),
}

fn main() {
    Application::new(|win_desc, state, window| {
        state.insert_theme(DEFAULT_THEME);



        // Menu bar
        let menu_bar = Element::new().build(state, window, |builder| {
            builder.class("menu_bar").set_height(Length::Pixels(40.0))
        });

        // Horizontal Container
        let hbox = HBox::new().build(state, window, |builder| builder.set_flex_grow(1.0));

        // Resizable Vertical Container
        let rvbox = ResizableVBox::new().build(state, hbox, |builder| {
            builder
                .set_width(Length::Pixels(300.0))
                .set_height(Length::Percentage(1.0))
                .set_background_color(Color::rgb(60, 60, 60))
        });

        // //
        let panel = Panel::new("Buttons").build(state, rvbox, |builder| builder);

        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Button").build(state, row, |builder| builder);
        Button::with_label("Press Me").build(state, row, |builder| builder);

        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Checkbox").build(state, row, |builder| builder);
        Checkbox::new(false).build(state, row, |builder| builder);

        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Switch").build(state, row, |builder| builder);
        Switch::new(false).build(state, row, |builder| builder);

        let panel = Panel::new("Input").build(state, rvbox, |builder| builder);

        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Textbox").build(state, row, |builder| builder);
        Textbox::new("Some Text").build(state, row, |builder| builder);

        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Spinner").build(state, row, |builder| builder);
        Spinner::new(100.0, 1.0).build(state, row, |builder| builder);

        let panel = Panel::new("Lists").build(state, rvbox, |builder| builder);

        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Dropdown").build(state, row, |builder| builder);
        let dropdown = Dropdown::new("Dropdown").build(state, row, |builder| builder).2;
        Item::new("Item 1","Item 1").build(state, dropdown, |builder| builder);
        Item::new("Item 2","Item 2").build(state, dropdown, |builder| builder);
        Item::new("Item 3","Item 3").build(state, dropdown, |builder| builder);

        let panel = Panel::new("Sliders").build(state, rvbox, |builder| builder);
        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Value").build(state, row, |builder| builder);
        let textbox = Textbox::new("0.0").build(state, row, |builder| builder);
        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Slider").build(state, row, |builder| builder);
        let slider = Slider2::new(move |value| Event::new(TextboxEvent::SetValue(value.to_string())).target(textbox)).build(state, row, |builder| builder);


        let panel = Panel::new("Radio List").build(state, rvbox, |builder| builder);
        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Radio List").build(state, row, |builder| builder);
        RadioList::new("group1").build(state, row, |builder| builder);
        RadioBox::new("group1").build(state, row, |builder| builder);

        // Tabs
        // let (tab_bar, tab_container) = TabContainer::new().build(state, window, |builder| builder);

        // Button::with_label("First")
        // .on_press(Event::new(TabEvent::SwitchTab(0)))
        // .build(state, tab_bar, |builder| builder.set_checked(true));
        // let first = Button::new().build(state, tab_container, |builder| builder.class("item1"));
        // Button::with_label("First Button").build(state, first, |builder| builder.class("test"));

        win_desc
    })
    .run();
}

// Example which shows all of the supported widgets

extern crate tuix;

use tuix::*;

use tuix::style::themes::DEFAULT_THEME;

static THEME: &'static str = include_str!("themes/widgets_theme.css");

fn main() {
    Application::new(|win_desc, state, window| {
        state.add_theme(DEFAULT_THEME);
        state.add_theme(THEME);

        // // Menu bar
        // let menu_bar = Element::new().build(state, window, |builder| {
        //     builder.class("menu_bar").set_height(Length::Pixels(40.0))
        // });

        // Horizontal Container
        let hbox = HBox::new().build(state, window, |builder| builder.set_flex_grow(1.0));

        // Resizable Vertical Container
        let rvbox = ResizableVBox::new().build(state, hbox, |builder| {
            builder
                .set_width(Length::Pixels(300.0))
                .set_height(Length::Percentage(1.0))
                .set_background_color(Color::rgb(60, 60, 60))
        });

        // BUTTONS PANEL
        let panel = Panel::new("Buttons").build(state, rvbox, |builder| builder);

        // BUTTON
        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Button").build(state, row, |builder| builder);
        Button::with_label("Press Me").build(state, row, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_flex_grow(1.0)
        });

        // CHECKBOX
        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Checkbox").build(state, row, |builder| builder);
        Checkbox::new(false).build(state, row, |builder| builder);

        // SWITCH
        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Switch").build(state, row, |builder| builder);
        let switch = Switch::new(false).build(state, row, |builder| builder);

        // // INPUT PANEL
        // let panel = Panel::new("Input").build(state, rvbox, |builder| builder);

        // // TEXTBOX
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Textbox").build(state, row, |builder| builder);
        // Textbox::new("Some Text").build(state, row, |builder| builder);

        // // SPINNER
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Spinner").build(state, row, |builder| builder);
        // Spinner::new(100)
        //     .with_min(95)
        //     .with_max(105)
        //     .on_min(Event::new(CheckboxEvent::Uncheck).target(switch))
        //     .on_max(Event::new(CheckboxEvent::Check).target(switch))
        //     .build(state, row, |builder| builder);

        // // VECTOR EDIT
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Vector Edit").build(state, row, |builder| builder);
        // VectorEdit::<f32>::new().build(state, row, |builder| builder.set_flex_grow(1.0));

        // // LISTS PANEL
        // let panel = Panel::new("Lists").build(state, rvbox, |builder| builder);

        // // DROPDOWN
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Dropdown").build(state, row, |builder| builder);
        // let (_, _, dropdown) = Dropdown::new("Dropdown").build(state, row, |builder| builder);
        // Item::new("Item 1", "Item 1").build(state, dropdown, |builder| builder);
        // Item::new("Item 2", "Item 2").build(state, dropdown, |builder| builder);
        // Item::new("Item 3", "Item 3").build(state, dropdown, |builder| builder);

        // // LISTBOX
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Listbox").build(state, row, |builder| builder);

        // // SLIDERS PANEL
        // let panel = Panel::new("Sliders").build(state, rvbox, |builder| builder);

        // // PROGRESS BAR
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Progress Bar").build(state, row, |builder| builder);
        // ProgressBar::new()
        //     .with_value(0.5)
        //     .build(state, row, |builder| builder.set_flex_grow(1.0));

        // // SLIDER 1
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Slider").build(state, row, |builder| builder);
        // Slider::new().build(state, row, |builder| builder.set_flex_grow(1.0));

        // // SLIDER 2
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Slider").build(state, row, |builder| builder);
        // Slider::new().build(state, row, |builder| {
        //     builder.set_flex_grow(1.0).class("custom1")
        // });

        // // SLIDER 3
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Slider").build(state, row, |builder| builder);
        // Slider::new().build(state, row, |builder| {
        //     builder.set_flex_grow(1.0).class("custom2")
        // });

        // let panel = Panel::new("Radio List").build(state, rvbox, |builder| builder);
        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Radio List (V)").build(state, row, |builder| builder);
        // let radio_list = RadioList::new().build(state, row, |builder| {
        //     builder
        //         .set_height(Length::Pixels(80.0))
        //         .set_justify_content(JustifyContent::SpaceEvenly)
        // });

        // let item = HBox::new().build(state, radio_list, |builder| builder);
        // let first = Radio::new().build(state, item, |builder| builder);
        // Label::new("Option 1").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |builder| builder);
        // Radio::new().build(state, item, |builder| builder);
        // Label::new("Option 2").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |builder| builder);
        // Radio::new().build(state, item, |builder| builder);
        // Label::new("Option 3").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // first.set_checked(state, true);

        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Radio List (H)").build(state, row, |builder| builder);
        // let radio_list = RadioList::new().build(state, row, |builder| {
        //     builder
        //         .set_flex_direction(FlexDirection::Row)
        //         .set_flex_grow(1.0)
        //         .set_justify_content(JustifyContent::SpaceEvenly)
        // });
        // let item = HBox::new().build(state, radio_list, |builder| builder);
        // let first = Radio::new().build(state, item, |builder| builder);
        // Label::new("1").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(30.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |builder| builder);
        // Radio::new().build(state, item, |builder| builder);
        // Label::new("2").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(30.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });
        // let item = HBox::new().build(state, radio_list, |builder| builder);
        // Radio::new().build(state, item, |builder| builder);
        // Label::new("3").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(30.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // first.set_checked(state, true);

        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Radio Buttons").build(state, row, |builder| builder);
        // let radio_list = RadioList::new().build(state, row, |builder| {
        //     builder
        //         .set_flex_direction(FlexDirection::Row)
        //         .set_flex_grow(1.0)
        // });

        // RadioButton::new().build(state, radio_list, |builder| {
        //     builder.set_text("A").class("first")
        // });
        // RadioButton::new().build(state, radio_list, |builder| {
        //     builder.set_text("B").class("button")
        // });
        // RadioButton::new().build(state, radio_list, |builder| {
        //     builder.set_text("C").class("last")
        // });

        // let row = HBox::new().build(state, panel, |builder| builder);
        // Label::new("Radio List (V)").build(state, row, |builder| builder);
        // let radio_list = RadioList::new().build(state, row, |builder| {
        //     builder
        //         .set_height(Length::Pixels(80.0))
        //         .set_justify_content(JustifyContent::SpaceEvenly)
        // });

        // let item = HBox::new().build(state, radio_list, |builder| builder);
        // Checkbox::new(true)
        //     .on_checked(Event::new(WindowEvent::WindowClose).target(Entity::root()))
        //     .build(state, item, |builder| builder);
        // Label::new("Option 1").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |builder| builder);
        // Checkbox::new(false).build(state, item, |builder| builder);
        // Label::new("Option 2").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |builder| builder);
        // Checkbox::new(false).build(state, item, |builder| builder);
        // Label::new("Option 3").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        win_desc
    })
    .run();
}

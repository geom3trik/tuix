// Example which shows all of the supported widgets

extern crate tuix;

use tuix::*;

use tuix::style::themes::DEFAULT_THEME;

static THEME: &'static str = include_str!("themes/widgets_theme.css");

fn main() {
    Application::new(|mut ctx, window| {
        ctx.state().add_theme(DEFAULT_THEME);
        ctx.state().add_theme(THEME);

        // // Menu bar
        // let menu_bar = Element::new().build(state, window, |context| {
        //     context.class("menu_bar").set_height(Length::Pixels(40.0))
        // });

        // Horizontal Container
        let mut hbox = HBox::new().build(&mut ctx).set_flex_grow(1.0);

        // Resizable Vertical Container
        let mut rvbox = ResizableVBox::new().build(&mut hbox)
                .set_width(Length::Pixels(300.0))
                .set_height(Length::Percentage(1.0))
                .set_background_color(Color::rgb(60, 60, 60));

        // // BUTTONS PANEL
        // let panel = Panel::new("Buttons").build(state, rvbox, |context| context);

        // // BUTTON
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Button").build(state, row, |context| context);
        // Button::with_label("Press Me").build(state, row, |context| {
        //     context
        //         .set_width(Length::Pixels(100.0))
        //         .set_height(Length::Pixels(30.0))
        //         .set_flex_grow(1.0)
        // });

        // // CHECKBUTTON
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Check Button").build(state, row, |context| context);
        // CheckButton::new(false).build(state, row, |context| context);

        // // CHECKBOX
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Checkbox").build(state, row, |context| context);
        // Checkbox::new(false).build(state, row, |context| context);

        // // SWITCH
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Switch").build(state, row, |context| context);
        // let switch = Switch::new(false).build(state, row, |context| context);

        // // INPUT PANEL
        // let panel = Panel::new("Input").build(state, rvbox, |context| context);

        // // TEXTBOX
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Textbox").build(state, row, |context| context);
        // Textbox::new("Some Text").build(state, row, |context| context);

        // // SPINNER
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Spinner").build(state, row, |context| context);
        // Spinbox::new(100)
        //     .with_min(95)
        //     .with_max(105)
        //     .on_min(Event::new(CheckboxEvent::Uncheck).target(switch))
        //     .on_max(Event::new(CheckboxEvent::Check).target(switch))
        //     .build(state, row, |context| context);

        // // VECTOR EDIT
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Vector Edit").build(state, row, |context| context);
        // VectorEdit::<f32>::new().build(state, row, |context| context.set_flex_grow(1.0));

        // LISTS PANEL
        let mut panel = Panel::new("Check Button Lists").build(&mut rvbox);

        // LIST
        let mut row = HBox::new().build(&mut panel);
        Label::new("List").build(&mut row);
        let mut list = List::new().build(&mut row).set_flex_grow(1.0);
        CheckButton::new(true).build(&mut list)
                .set_text("Option 1")
                .set_flex_basis(Length::Pixels(30.0))
                .set_padding_left(Length::Pixels(5.0));
        CheckButton::new(false).build(&mut list)
                .set_text("Option 2")
                .set_flex_basis(Length::Pixels(30.0))
                .set_padding_left(Length::Pixels(5.0));
        CheckButton::new(false).build(&mut list)
                .set_text("Option 3")
                .set_flex_basis(Length::Pixels(30.0))
                .set_padding_left(Length::Pixels(5.0));

        // MULTILIST
        let mut row = HBox::new().build(&mut panel);
        Label::new("Multilist").build(&mut row);
        let mut list = List::new()
            .set_multi()
            .build(&mut row).set_flex_grow(1.0);
        CheckButton::new(true).build(&mut list)
                .set_text("Option 1")
                .set_flex_basis(Length::Pixels(30.0))
                .set_padding_left(Length::Pixels(5.0));
        CheckButton::new(false).build(&mut list)
                .set_text("Option 2")
                .set_flex_basis(Length::Pixels(30.0))
                .set_padding_left(Length::Pixels(5.0));
        CheckButton::new(false).build(&mut list)
                .set_text("Option 3")
                .set_flex_basis(Length::Pixels(30.0))
                .set_padding_left(Length::Pixels(5.0));

        // LISTS PANEL
        let mut panel = Panel::new("Dropdown Lists").build(&mut rvbox);

        // DROPDOWN LIST
        let mut row = HBox::new().build(&mut panel);
        Label::new("List").build(&mut row);
        let mut popup = Dropdown::new("Dropdown").build(&mut row)
            .set_height(Length::Pixels(30.0)).set_flex_grow(1.0);
        let mut list = List::new().build(&mut popup).set_flex_grow(1.0);
        CheckButton::new(true).build(&mut list)
                .set_text("Option 1")
                .set_flex_basis(Length::Pixels(30.0))
                .set_padding_left(Length::Pixels(5.0));
        CheckButton::new(false).build(&mut list)
                .set_text("Option 2")
                .set_flex_basis(Length::Pixels(30.0))
                .set_padding_left(Length::Pixels(5.0));
        CheckButton::new(false).build(&mut list)
                .set_text("Option 3")
                .set_flex_basis(Length::Pixels(30.0))
                .set_padding_left(Length::Pixels(5.0));

        // DROPDOWN MULTILIST
        let mut row = HBox::new().build(&mut panel);
        Label::new("Multilist").build(&mut row);
        let mut popup = Dropdown::new("Dropdown")
            .set_multi()
            .build(&mut row)
                .set_height(Length::Pixels(30.0)).set_flex_grow(1.0);
        let mut list = List::new()
            .set_multi()
            .build(&mut popup).set_flex_grow(1.0);
        CheckButton::new(true).build(&mut list)
            .set_text("Option 1")
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));

        CheckButton::new(false).build(&mut list)
            .set_text("Option 2")
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));

        CheckButton::new(false).build(&mut list)
            .set_text("Option 3")
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));

        // DROPDOWN CHECKITEM LIST
        let mut row = HBox::new().build(&mut panel);
        Label::new("List").build(&mut row);
        let mut popup = Dropdown::new("Dropdown").build(&mut row)
            .set_height(Length::Pixels(30.0)).set_flex_grow(1.0);
        let mut list = List::new().build(&mut popup).set_flex_grow(1.0);
        CheckItem::new("Option 1", true).build(&mut list)
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));
        CheckItem::new("Option2", false).build(&mut list)
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));
        CheckItem::new("Option3", false).build(&mut list)
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));

        // DROPDOWN CHECK MULTILIST
        let mut row = HBox::new().build(&mut panel);
        Label::new("Multilist").build(&mut row);
        let mut popup = Dropdown::new("Dropdown")
            .set_multi()
            .build(&mut row)
            .set_height(Length::Pixels(30.0)).set_flex_grow(1.0);
        let mut list = List::new()
            .set_multi()
            .build(&mut popup).set_flex_grow(1.0);
        CheckItem::new("Option 1", true).build(&mut list)
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));
        CheckItem::new("Option2", false).build(&mut list)
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));
        CheckItem::new("Option3", false).build(&mut list)
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));

        // LISTS PANEL
        let mut panel = Panel::new("Check List").build(&mut rvbox);

        // LISTBOX
        let mut row = HBox::new().build(&mut panel);
        Label::new("List").build(&mut row);
        let mut list = List::new().build(&mut row)
            .set_flex_grow(1.0);
        CheckItem::new("Option1", true).build(&mut list)
            //.set_text("Option 1")
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));
        CheckItem::new("Option2", false).build(&mut list)
            //.set_text("Option 2")
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));
        CheckItem::new("Option3", false).build(&mut list)
            //.set_text("Option 3")
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));

        // LISTBOX
        let mut row = HBox::new().build(&mut panel);
        Label::new("Multilist").build(&mut row);
        let mut list = List::new()
            .set_multi()
            .build(&mut row).set_flex_grow(1.0);
        CheckItem::new("Option1", true).build(&mut list)
            //.set_text("Option 1")
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));
        CheckItem::new("Option2", false).build(&mut list)
            //.set_text("Option 2")
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));
        CheckItem::new("Option3", false).build(&mut list)
            //.set_text("Option 3")
            .set_flex_basis(Length::Pixels(30.0))
            .set_padding_left(Length::Pixels(5.0));

        //

        // // SLIDERS PANEL
        // let panel = Panel::new("Sliders").build(state, rvbox, |context| context);

        // // PROGRESS BAR
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Progress Bar").build(state, row, |context| context);
        // ProgressBar::new()
        //     .with_value(0.5)
        //     .build(state, row, |context| context.set_flex_grow(1.0));

        // // VALUE SLIDER
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Value Slider").build(state, row, |context| context);
        // ValueSlider::new("value").build(state, row, |context| context.set_flex_grow(1.0));

        // // SLIDER 1
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Slider").build(state, row, |context| context);
        // Slider::new().build(state, row, |context| context.set_flex_grow(1.0));

        // // SLIDER 2
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Slider").build(state, row, |context| context);
        // Slider::new().build(state, row, |context| {
        //     context.set_flex_grow(1.0).class("custom1")
        // });

        // // SLIDER 3
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Slider").build(state, row, |context| context);
        // Slider::new().build(state, row, |context| {
        //     context.set_flex_grow(1.0).class("custom2")
        // });

        // let panel = Panel::new("Radio List").build(state, rvbox, |context| context);
        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Radio List (V)").build(state, row, |context| context);
        // let radio_list = List::new().build(state, row, |context| {
        //     context
        //         .set_height(Length::Pixels(80.0))
        //         .set_justify_content(JustifyContent::SpaceEvenly)
        // });

        // let item = HBox::new().build(state, radio_list, |context| context);
        // let first = Radio::new().build(state, item, |context| context);
        // Label::new("Option 1").build(state, item, |context| {
        //     context
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |context| context);
        // Radio::new().build(state, item, |context| context);
        // Label::new("Option 2").build(state, item, |context| {
        //     context
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |context| context);
        // Radio::new().build(state, item, |context| context);
        // Label::new("Option 3").build(state, item, |context| {
        //     context
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // first.set_checked(state, true);

        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Radio List (H)").build(state, row, |context| context);
        // let radio_list = List::new().build(state, row, |context| {
        //     context
        //         .set_flex_direction(FlexDirection::Row)
        //         .set_flex_grow(1.0)
        //         .set_justify_content(JustifyContent::SpaceEvenly)
        // });
        // let item = HBox::new().build(state, radio_list, |context| context);
        // let first = Radio::new().build(state, item, |context| context);
        // Label::new("1").build(state, item, |context| {
        //     context
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(30.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |context| context);
        // Radio::new().build(state, item, |context| context);
        // Label::new("2").build(state, item, |context| {
        //     context
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(30.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });
        // let item = HBox::new().build(state, radio_list, |context| context);
        // Radio::new().build(state, item, |context| context);
        // Label::new("3").build(state, item, |context| {
        //     context
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(30.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // first.set_checked(state, true);

        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Radio Buttons").build(state, row, |context| context);
        // let radio_list = List::new().build(state, row, |context| {
        //     context
        //         .set_flex_direction(FlexDirection::Row)
        //         .set_flex_grow(1.0)
        // });

        // let first = CheckButton::new(true).build(state, radio_list, |context| {
        //     context.set_text("A").class("first")
        // });

        // CheckButton::new(false).build(state, radio_list, |context| {
        //     context.set_text("B").class("button")
        // });

        // CheckButton::new(false).build(state, radio_list, |context| {
        //     context.set_text("C").class("last")
        // });

        // first.set_checked(state, true);

        // let row = HBox::new().build(state, panel, |context| context);
        // Label::new("Radio List (V)").build(state, row, |context| context);
        // let radio_list = List::new().build(state, row, |context| {
        //     context
        //         .set_height(Length::Pixels(80.0))
        //         .set_justify_content(JustifyContent::SpaceEvenly)
        // });

        // let item = HBox::new().build(state, radio_list, |context| context);
        // Checkbox::new(true)
        //     .on_checked(Event::new(WindowEvent::WindowClose).target(Entity::root()))
        //     .build(state, item, |context| context);
        // Label::new("Option 1").build(state, item, |context| {
        //     context
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |context| context);
        // Checkbox::new(false).build(state, item, |context| context);
        // Label::new("Option 2").build(state, item, |context| {
        //     context
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });

        // let item = HBox::new().build(state, radio_list, |context| context);
        // Checkbox::new(false).build(state, item, |context| context);
        // Label::new("Option 3").build(state, item, |context| {
        //     context
        //         .set_font_color(Color::black())
        //         .set_width(Length::Pixels(50.0))
        //         .set_margin_left(Length::Pixels(5.0))
        // });
    }).run();
}

// Example which shows all of the supported widgets

extern crate tuix;

use tuix::*;

use tuix::style::themes::DEFAULT_THEME;

static THEME: &'static str = include_str!("themes/widgets_theme.css");

fn main() {
    Application::new(|state, window| {
        state.add_theme(DEFAULT_THEME);
        state.add_theme(THEME);

        window.set_min_inner_size(300, 300);

        // Resizable Vertical Container
        let rcolumn = ResizableColumn::new().build(state, window.entity(), |builder| {
            builder
                .set_width(Stretch(1.0))
                .set_max_width(Pixels(500.0))
                .set_min_width(Pixels(300.0))
                .set_height(Stretch(1.0))
                //.set_background_color(Color::rgb(60, 60, 60))
        });

        let scroll = ScrollContainer::new().build(state, rcolumn, |builder| builder);

        // BUTTONS PANEL
        let panel = Panel::new("Buttons").build(state, scroll, |builder| 
            builder
                .set_width(Stretch(1.0))
        );

        panel.set_child_space(state, Pixels(10.0)).set_child_between(state, Pixels(10.0));

        // // BUTTON
        let row = Row::new().build(state, panel, |builder| builder);
        Label::new("Button").build(state, row, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_width(Pixels(120.0))
        );
        Button::with_label("Press Me").build(state, row, |builder| {
            builder
                // .set_height(Pixels(30.0))
        });

        // // CHECKBUTTON
        // let row = Row::new().build(state, panel, |builder| builder.set_width(Stretch(1.0)));
        // Label::new("Check Button").build(state, row, |builder| builder);
        // CheckButton::new(false).build(state, row, |builder| builder);

        // CHECKBOX
        let row = Row::new().build(state, panel, |builder| builder);
        Label::new("Checkbox").build(state, row, |builder| builder);
        Checkbox::new(false).build(state, row, |builder| builder);

        // SWITCH
        let row = Row::new().build(state, panel, |builder| builder);
        Label::new("Switch").build(state, row, |builder| builder);
        let switch = Switch::new(false).build(state, row, |builder| builder);

        // INPUT PANEL
        let panel = Panel::new("Input").build(state, scroll, |builder| builder);

        panel.set_child_space(state, Pixels(10.0)).set_child_between(state, Pixels(10.0));


        // TEXTBOX
        let row = Row::new().build(state, panel, |builder| builder);
        Label::new("Textbox").build(state, row, |builder| builder);
        Textbox::new("Some Text").build(state, row, |builder| builder);

        // SPINNER
        let row = Row::new().build(state, panel, |builder| builder);
        Label::new("Spinner").build(state, row, |builder| builder);
        Spinbox::new(100)
            .with_min(95)
            .with_max(105)
            .on_min(Event::new(CheckboxEvent::Uncheck).target(switch))
            .on_max(Event::new(CheckboxEvent::Check).target(switch))
            .build(state, row, |builder| builder);

        // // VECTOR EDIT
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Vector Edit").build(state, row, |builder| builder);
        // VectorEdit::<f32>::new().build(state, row, |builder| builder.set_flex_grow(1.0));


        //

        // LISTS PANEL
        let panel = Panel::new("Check Button Lists").build(state, scroll, |builder| builder);

        panel.set_child_space(state, Pixels(10.0)).set_child_between(state, Pixels(10.0));
        // LIST
        let row = Row::new().build(state, panel, |builder| builder);
        Label::new("List").build(state, row, |builder| builder);
        let list = List::new().build(state, row, |builder| 
            builder
                .set_width(Stretch(1.0))
                .set_height(Auto)
        );
        CheckButton::new(true).build(state, list, |builder| {
            builder
                .set_text("Option 1")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });
        CheckButton::new(false).build(state, list, |builder| {
            builder
                .set_text("Option 2")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });
        CheckButton::new(false).build(state, list, |builder| {
            builder
                .set_text("Option 3")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });

        // MULTILIST
        let row = Row::new().build(state, panel, |builder| builder);
        Label::new("Multilist").build(state, row, |builder| builder);
        let list = List::new()
            .set_multi()
            .build(state, row, |builder| 
                builder
                    .set_width(Stretch(1.0))
                    .set_height(Auto)
            );
        CheckButton::new(true).build(state, list, |builder| {
            builder
                .set_text("Option 1")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });
        CheckButton::new(false).build(state, list, |builder| {
            builder
                .set_text("Option 2")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });
        CheckButton::new(false).build(state, list, |builder| {
            builder
                .set_text("Option 3")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });

        // LISTS PANEL
        let panel = Panel::new("Dropdown Lists").build(state, scroll, |builder| builder);

        panel.set_child_space(state, Pixels(10.0)).set_child_between(state, Pixels(10.0));

        // DROPDOWN LIST
        let row = Row::new().build(state, panel, |builder| builder);
        Label::new("List").build(state, row, |builder| builder);
        let (_, _, popup) = Dropdown::new("Dropdown").build(state, row, |builder| {
            builder.set_height(Pixels(30.0)).set_width(Stretch(1.0))
        });
        let list = List::new()
            .build(state, popup, |builder| builder);
        CheckButton::new(true)
            .build(state, list, |builder| {
                builder
                    .set_text("Option 1")
                    .set_height(Pixels(30.0))
                    .set_child_left(Pixels(5.0))
            });
        CheckButton::new(false)
            .build(state, list, |builder| {
                builder
                    .set_text("Option 2")
                    .set_height(Pixels(30.0))
                    .set_child_left(Pixels(5.0))
            });
        CheckButton::new(false).build(state, list, |builder| {
            builder
                .set_text("Option 3")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });

        // DROPDOWN MULTILIST
        let row = Row::new().build(state, panel, |builder| builder);
        Label::new("Multilist").build(state, row, |builder| builder);
        let (_, _, popup) = Dropdown::new("Dropdown")
            .set_multi()
            .build(state, row, |builder| {
                builder.set_height(Pixels(30.0))
            });
        let list = List::new()
            .set_multi()
            .build(state, popup, |builder| builder);
        CheckButton::new(true).build(state, list, |builder| {
            builder
                .set_text("Option 1")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });
        CheckButton::new(false).build(state, list, |builder| {
            builder
                .set_text("Option 2")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });
        CheckButton::new(false).build(state, list, |builder| {
            builder
                .set_text("Option 3")
                .set_height(Pixels(30.0))
                .set_child_left(Pixels(5.0))
        });

        // // DROPDOWN CHECKITEM LIST
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("List").build(state, row, |builder| builder);
        // let (_, _, popup) = Dropdown::new("Dropdown").build(state, row, |builder| {
        //     builder.set_height(Pixels(30.0)).set_flex_grow(1.0)
        // });
        // let list = List::new().build(state, popup, |builder| builder.set_flex_grow(1.0));
        // CheckItem::new("Option 1", true).build(state, list, |builder| {
        //     builder
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });
        // CheckItem::new("Option2", false).build(state, list, |builder| {
        //     builder
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });
        // CheckItem::new("Option3", false).build(state, list, |builder| {
        //     builder
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });

        // // DROPDOWN CHECK MULTILIST
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Multilist").build(state, row, |builder| builder);
        // let (_, _, popup) = Dropdown::new("Dropdown")
        //     .set_multi()
        //     .build(state, row, |builder| {
        //         builder.set_height(Pixels(30.0)).set_flex_grow(1.0)
        //     });
        // let list = List::new()
        //     .set_multi()
        //     .build(state, popup, |builder| builder.set_flex_grow(1.0));
        // CheckItem::new("Option 1", true).build(state, list, |builder| {
        //     builder
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });
        // CheckItem::new("Option2", false).build(state, list, |builder| {
        //     builder
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });
        // CheckItem::new("Option3", false).build(state, list, |builder| {
        //     builder
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });

        // // LISTS PANEL
        // let panel = Panel::new("Check List").build(state, rcolumn, |builder| builder);

        // // LISTBOX
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("List").build(state, row, |builder| builder);
        // let list = List::new().build(state, row, |builder| builder.set_flex_grow(1.0));
        // CheckItem::new("Option1", true).build(state, list, |builder| {
        //     builder
        //         //.set_text("Option 1")
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });
        // CheckItem::new("Option2", false).build(state, list, |builder| {
        //     builder
        //         //.set_text("Option 2")
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });
        // CheckItem::new("Option3", false).build(state, list, |builder| {
        //     builder
        //         //.set_text("Option 3")
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });

        // // LISTBOX
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Multilist").build(state, row, |builder| builder);
        // let list = List::new()
        //     .set_multi()
        //     .build(state, row, |builder| builder.set_flex_grow(1.0));
        // CheckItem::new("Option1", true).build(state, list, |builder| {
        //     builder
        //         //.set_text("Option 1")
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });
        // CheckItem::new("Option2", false).build(state, list, |builder| {
        //     builder
        //         //.set_text("Option 2")
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });
        // CheckItem::new("Option3", false).build(state, list, |builder| {
        //     builder
        //         //.set_text("Option 3")
        //         .set_flex_basis(Pixels(30.0))
        //         .set_child_left(Pixels(5.0))
        // });

        //

        // SLIDERS PANEL
        let panel = Panel::new("Sliders").build(state, scroll, |builder| builder);
        panel.set_child_space(state, Pixels(10.0));
        // // PROGRESS BAR
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Progress Bar").build(state, row, |builder| builder);
        // ProgressBar::new()
        //     .with_value(0.5)
        //     .build(state, row, |builder| builder.set_flex_grow(1.0));

        // // VALUE SLIDER
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Value Slider").build(state, row, |builder| builder);
        // ValueSlider::new("value").build(state, row, |builder| builder.set_flex_grow(1.0));

        // SLIDER 1
        let row = Row::new().build(state, panel, |builder| builder.set_width(Stretch(1.0)));
        Label::new("Slider").build(state, row, |builder| builder);
        Slider::new()
            .with_initial_value(10.0)
            .with_range(0.0..20.0)
            .on_change(|val| Event::new(WindowEvent::Debug(format!("slider on_changed - {}", val))))
            //.on_changing(|val| Event::new(WindowEvent::Debug(format!("slider on_changing - {}", val))))
            .on_press(Event::new(WindowEvent::Debug("slider on_press".to_owned())))
            .on_release(Event::new(WindowEvent::Debug("slider on_release".to_owned())))
            .on_min(|val| Event::new(WindowEvent::Debug(format!("slider on_min - {}", val))))
            .on_max(|val| Event::new(WindowEvent::Debug(format!("slider on_max - {}", val))))
            .on_over(Event::new(WindowEvent::Debug("slider on_over".to_owned())))
            .on_out(Event::new(WindowEvent::Debug("slider on_out".to_owned())))

            .build(state, row, |builder| 
                builder
                    .set_top(Stretch(1.0))
                    .set_bottom(Stretch(1.0))
            );

        // // SLIDER 2
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Slider").build(state, row, |builder| builder);
        // Slider::new().build(state, row, |builder| {
        //     builder.set_flex_grow(1.0).class("custom1")
        // });

        // // SLIDER 3
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Slider").build(state, row, |builder| builder);
        // Slider::new().build(state, row, |builder| {
        //     builder.set_flex_grow(1.0).class("custom2")
        // });

        // let panel = Panel::new("Radio List").build(state, rcolumn, |builder| builder);
        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Radio List (V)").build(state, row, |builder| builder);
        // let radio_list = List::new().build(state, row, |builder| {
        //     builder
        //         .set_height(Pixels(80.0))
        //         .set_justify_content(JustifyContent::SpaceEvenly)
        // });

        // let item = Row::new().build(state, radio_list, |builder| builder);
        // let first = Radio::new().build(state, item, |builder| builder);
        // Label::new("Option 1").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Pixels(50.0))
        //         .set_margin_left(Pixels(5.0))
        // });

        // let item = Row::new().build(state, radio_list, |builder| builder);
        // Radio::new().build(state, item, |builder| builder);
        // Label::new("Option 2").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Pixels(50.0))
        //         .set_margin_left(Pixels(5.0))
        // });

        // let item = Row::new().build(state, radio_list, |builder| builder);
        // Radio::new().build(state, item, |builder| builder);
        // Label::new("Option 3").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Pixels(50.0))
        //         .set_margin_left(Pixels(5.0))
        // });

        // first.set_checked(state, true);

        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Radio List (H)").build(state, row, |builder| builder);
        // let radio_list = List::new().build(state, row, |builder| {
        //     builder
        //         .set_flex_direction(FlexDirection::Row)
        //         .set_flex_grow(1.0)
        //         .set_justify_content(JustifyContent::SpaceEvenly)
        // });
        // let item = Row::new().build(state, radio_list, |builder| builder);
        // let first = Radio::new().build(state, item, |builder| builder);
        // Label::new("1").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Pixels(30.0))
        //         .set_margin_left(Pixels(5.0))
        // });

        // let item = Row::new().build(state, radio_list, |builder| builder);
        // Radio::new().build(state, item, |builder| builder);
        // Label::new("2").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Pixels(30.0))
        //         .set_margin_left(Pixels(5.0))
        // });
        // let item = Row::new().build(state, radio_list, |builder| builder);
        // Radio::new().build(state, item, |builder| builder);
        // Label::new("3").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Pixels(30.0))
        //         .set_margin_left(Pixels(5.0))
        // });

        // first.set_checked(state, true);

        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Radio Buttons").build(state, row, |builder| builder);
        // let radio_list = List::new().build(state, row, |builder| {
        //     builder
        //         .set_flex_direction(FlexDirection::Row)
        //         .set_flex_grow(1.0)
        // });

        // let first = CheckButton::new(true).build(state, radio_list, |builder| {
        //     builder.set_text("A").class("first")
        // });

        // CheckButton::new(false).build(state, radio_list, |builder| {
        //     builder.set_text("B").class("button")
        // });

        // CheckButton::new(false).build(state, radio_list, |builder| {
        //     builder.set_text("C").class("last")
        // });

        // first.set_checked(state, true);

        // let row = Row::new().build(state, panel, |builder| builder);
        // Label::new("Radio List (V)").build(state, row, |builder| builder);
        // let radio_list = List::new().build(state, row, |builder| {
        //     builder
        //         .set_height(Pixels(80.0))
        //         .set_justify_content(JustifyContent::SpaceEvenly)
        // });

        // let item = Row::new().build(state, radio_list, |builder| builder);
        // Checkbox::new(true)
        //     .on_checked(Event::new(WindowEvent::WindowClose).target(Entity::root()))
        //     .build(state, item, |builder| builder);
        // Label::new("Option 1").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Pixels(50.0))
        //         .set_margin_left(Pixels(5.0))
        // });

        // let item = Row::new().build(state, radio_list, |builder| builder);
        // Checkbox::new(false).build(state, item, |builder| builder);
        // Label::new("Option 2").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Pixels(50.0))
        //         .set_margin_left(Pixels(5.0))
        // });

        // let item = Row::new().build(state, radio_list, |builder| builder);
        // Checkbox::new(false).build(state, item, |builder| builder);
        // Label::new("Option 3").build(state, item, |builder| {
        //     builder
        //         .set_font_color(Color::black())
        //         .set_width(Pixels(50.0))
        //         .set_margin_left(Pixels(5.0))
        // });
    })
    .run();
}

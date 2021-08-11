

use tuix::*;

const STYLE: &str = r#"
    list {
        border-width: 1px;
        border-color: #555555;
    }

    list>check_button {
        height: 30px;
        child-space: 1s;
        child-left:
        background-color: white;
        color: black;
    }

    list>check_button:checked {
        background-color: #AAAAFF;
    }

    list>check_button:focus {
        border-width: 1px;
        border-color: black;
    }

    stack {
        border-width: 1px;
        border-color: #555555;
        left: -1px;
    }

    button {
        child-space: 1s;
        border-radius: 3px;
        color: black;
        background-color: #d2d2d2;
    }

    button:hover {
        background-color: #e2e2e2;
    }

    button:active {
        background-color: #c2c2c2;
    }

    slider {
        height: 20px;
    }

    slider>.track {
        background-color: #dfdfdf;
        border-radius: 2px;
    }

    slider>.track>.active {
        background-color: #f74c00;
        border-radius: 2px;
    }

    slider>.thumb {
        background-color: white;
        width: 20px;
        height: 20px;
        border-radius: 9.5px;
        border-color: #757575;
        border-width: 1px;
    }

    checkbox {
        width: 20px;
        height: 20px;
        background-color: white;
        border-width: 1px;
        border-color: #757575;
        color: black;
    }

    spinbox {
        background-color: white;
    }

    spinbox .increment {
        color: #ff5e1a;
        text-justify: center;
    }

    spinbox .decrement {
        color: #ff5e1a;
        text-justify: center;
    }

    spinbox>textbox {
        color: black;
        border-width: 1px;
        border-color: #757575;
        right: -1px;
        child-space: 1s;
    }

    spinbox>.arrow_container {
        border-width: 1px;
        border-color: #757575;
    }

    .page {
        child-left: 10px;
        child-top: 10px;
    }

"#;

pub enum Widgets {
    Button,
    Checkbox,

}

pub enum WidgetGalleryEvent {
    Switch(Widgets),
}

pub struct WidgetGallery {

}

impl WidgetGallery {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Widget for WidgetGallery {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        // Horizontal Split
        let row = Row::new().build(state, entity, |builder| builder);
        
        let widget_list = List::new().build(state, row, |builder| builder.set_width(Pixels(300.0)));
        let widget_view = Stack::new().build(state, row, |builder| builder);
        
        CheckButton::with_label("Button")
        .on_checked(move |_, state, button| {
            button.emit_to(state, widget_view, StackEvent::SetIndex(0));
        })
        .build(state, widget_list, |builder| builder.set_height(Pixels(30.0)));
        
        CheckButton::with_label("Checkbox")
        .on_checked(move |_, state, button| {
            button.emit_to(state, widget_view, StackEvent::SetIndex(1));
        })
        .build(state, widget_list, |builder| builder.set_height(Pixels(30.0)));

        CheckButton::with_label("Spinbox")
        .on_checked(move |_, state, button| {
            button.emit_to(state, widget_view, StackEvent::SetIndex(2));
        })
        .build(state, widget_list, |builder| builder.set_height(Pixels(30.0)));

        CheckButton::with_label("Slider")
        .on_checked(move |_, state, button| {
            button.emit_to(state, widget_view, StackEvent::SetIndex(3));
        })
        .build(state, widget_list, |builder| builder.set_height(Pixels(30.0)));
        

        let button_page = Element::new().build(state, widget_view, |builder| 
            builder
                .set_text("A simple push button.")
                .set_color(Color::black())
                .class("page")
            );

        Button::with_label("Change Color")
            // .on_press(|button, state, entity| {
            //     let r: u8 = rand::thread_rng().gen();
            //     let g: u8 = rand::thread_rng().gen();
            //     let b: u8 = rand::thread_rng().gen();
            //     state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(r, g, b, 255))).target(entity));
            // })
            .build(state, button_page, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });
        
        
        let checkbox_page = Element::new().build(state, widget_view, |builder| 
            builder
                .set_text("A simple checkbox.")
                .set_color(Color::black())
                .class("page")
            );

        Checkbox::new(true)
            // .on_checked(|checkbox, state, entity| {
            //     state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(255, 100, 100, 255))).target(entity));
            // })
            // .on_unchecked(|checkbox, state, entity|{
            //     state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(255, 255, 255, 255))).target(entity));
            // })
            .build(state, checkbox_page, |builder| {
                builder
                    .set_space(Stretch(1.0))
            });

        
        let spinbox_page = Element::new().build(state, widget_view, |builder| 
            builder
                .set_text("A spinbox widget.")
                .set_color(Color::black())
                .class("page")
            );

        Spinbox::new(255u8)
            // .on_change(|spinbox, state, entity| {
            //     state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(spinbox.value, spinbox.value, spinbox.value, 255))).target(entity));
            // })
            .build(state, spinbox_page, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });


        let slider_page = Element::new().build(state, widget_view, |builder| 
            builder
                .set_text("A slider widget.")
                .set_color(Color::black())
                .class("page")
            );
        Slider::new()
            // .on_changing(|slider, state, entity| {
            //     let val = (slider.value * 255.0) as u8;
            //     state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(val, val, val, 255))).target(entity));
            // })
            .build(state, slider_page, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });
        
        entity
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new().with_title("Widget Gallery"),
    |state, window| {

            window.set_background_color(state, Color::white()).set_focusable(state, false);

            state.add_theme(STYLE);
            
            WidgetGallery::new().build(state, window, |builder| builder);

        },
    );

    app.run();
}
extern crate tuix;

use image::buffer::Pixels;
use tuix::widgets::Button;
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::*;

use tuix::style::themes::DEFAULT_THEME;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomEvent {
    TestEvent,
}

fn custom_handler(state: &mut State, handle: &Handle, meta: &Meta, event: &mut CustomEvent) -> bool {
    
    match event {
        CustomEvent::TestEvent => {
            println!("Test Event!");
        }
    }

    false
}

fn another_handler(state: &mut State, handle: &Handle, meta: &Meta, event: &mut CustomEvent) -> bool {

    match event {
        CustomEvent::TestEvent => {
            println!("Another Test Event!");
        }
    }

    false
}

fn main() {
    let app = Application::new(|win_desc, state, window| {
        state.insert_theme(DEFAULT_THEME);

        let test_button = Button::with_label("Hello Test").build(state, &window);
        test_button.set_width(Length::Pixels(400.0));
        test_button.set_height(Length::Pixels(400.0));

        //let mut handlers: Vec<fn(&mut State, &Handle, &mut Event) -> bool> = Vec::new();
        //handlers.push(custom_handler);
        //handlers.push(another_handler);
        //state.handlers.insert(test_button.entity, handlers);

        state.insert_event_handler(test_button.entity, |state, handle, meta, event : &mut CustomEvent|{
            match event {
                CustomEvent::TestEvent => {
                    println!("Test!");
                }
            }

            false
        });
        state.insert_event_handler(test_button.entity, another_handler);

        // let child_button = Button::with_label("Child").build(state, &test_button);
        // child_button.set_width(Length::Pixels(300.0));
        // test_button.set_height(Length::Pixels(300.0));

        let my_button = Button::with_label("Test")
            .on_press(Event::new(CustomEvent::TestEvent).target(test_button.entity))
            .build(state, &window)
            .set_width(Length::Pixels(100.0))
            .set_height(Length::Pixels(30.0));

        // let my_button = Button::with_label("Hello GUI!").build(state, window, |builder| {
        //     builder.set_text("Button")
        // });

        // my_button.mutate(state, |ctx|
        //     ctx
        //         .set_width(Length::Pixels(100.0))
        //         .set_height(Length::Pixels(100.0))
        // );

        // if let Some(button) = my_button.testy::<Button>(state) {
        //     println!("{:?}", button.text)
        // }

        //my_button.testy2::<Button,_>(state, |button| println!("{:?}", button.text));

        win_desc.with_title("Hello GUI")
    });

    app.run();
}

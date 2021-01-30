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

pub fn custom_event_handler(state: &mut State, handle: &Handle, event_data: &EventData, event: &mut CustomEvent) -> bool {
    match event {
        CustomEvent::TestEvent => {
            println!("Test!");
        }
    }

    false
}

fn main() {
    let app = Application::new(|win_desc, state, window| {
        state.insert_theme(DEFAULT_THEME);


        // let mut test_button = Button::with_label("Hello GUI")
        //     .build(state, &window)
        //     .add_event_handler2(custom_event_handler);

        let radio_list = RadioList::new().build(state, &window);

        let my_button = Button::with_label("A")
            .build(state, &radio_list)
            .add_event_handler2(radio_button_event_handler);

        let my_button = Button::with_label("B")
            .build(state, &radio_list)
            .add_event_handler2(radio_button_event_handler);

        let my_button = Button::with_label("C")
            .build(state, &radio_list)
            .add_event_handler2(radio_button_event_handler);



        //.on_press(Event::new(CustomEvent::TestEvent).target(test_button.entity))



        // let mut test_button = state
        //     .add_widget(&window)
        //     .set_width(Length::Pixels(200.0))
        //     .set_height(Length::Pixels(200.0))
        //     .set_text("Hello GUI")
        //     .set_element("button")
        //     .add_draw_hander(DefaultDrawHandler::default())
        //     .add_event_handler2(|state, handle, meta, event: &mut CustomEvent| {
        //         match event {
        //             CustomEvent::TestEvent => {
        //                 println!("Test!");
        //             }
        //         }

        //         false
        //     });

        // let my_button = state
        //     .add_widget(&window)
        //     .set_text("Test")
        //     .set_width(Length::Pixels(100.0))
        //     .set_height(Length::Pixels(30.0))
        //     .set_element("button")
        //     .add_draw_hander(DefaultDrawHandler::default())
        //     .add_component(ButtonState::default()
        //         .on_press(Event::new(CustomEvent::TestEvent).target(test_button.entity))
        //     )
        //     .add_event_handler(button_handler);




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

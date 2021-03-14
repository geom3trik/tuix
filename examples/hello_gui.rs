use tuix::*;

fn main() {
    let app = Application::new(|win_desc, state, window| {
        Button::with_label("Button")
        .on_test(|button, state, entity| {
            state.insert_event(Event::new(WindowEvent::WindowClose));
            entity.set_text(state,"Pressed");
        })
        .build(state, window, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::from("#ff5e1a"))
                .set_text_justify(Justify::Center)
        });

        

        win_desc.with_title("Hello GUI")
    });

    app.run();
}

// struct Widget {
//     pub state: f32,
//     pub on_change: Option<Box<dyn Fn(&mut Self)>>,
    
// }

// impl Widget {
//     pub fn new<F>(state: f32, callback: F) -> Self
//     where F: 'static + Fn(&mut Self)
//     {
//         Self {
//             state,
//             on_change: Some(Box::new(callback)),
//         }
//     }
    
//     pub fn trigger(&mut self) {
//         if let Some(on_change) = self.on_change.take() {
//             on_change(self);

//             self.on_change = Some(on_change);
//         }
       
//     }
// }

// fn main() {
//     let mut widget = Widget::new(3.0, |widget| widget.state = 5.0);
//     widget.trigger();
// }

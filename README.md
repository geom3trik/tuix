# tuix
Cross-platform GUI toolkit written in Rust

## How to use
Since it's probably best to learn by example, here is the "hello world" of GUI applications:

```
extern crate rust_gui;

use rust_gui::Application;
use rust_gui::widgets::Button;
use rust_gui::events::BuildHandler;
use rust_gui::style::{Color, Length};

fn main() {
    let mut app = Application::new(|window| window.with_title("Hello GUI"));

    let state = app.get_state();
    let window = state.root;

    Button::new().build(state, window, |builder| {
        builder
            .set_width(Length::Pixels(100.0))
            .set_height(Length::Pixels(50.0))
            .set_background_color(Color::rgb(50,50,100))
    });

    app.run();
}
```

The first line inside the main function creates our application. A window closure allows us to set properties of the window such as its title. Unfortunately (name) is currently a single window GUI, but multiwindow is coming soon! (maybe)

After creating the application we have a couple of helper lines to make the code more readable. The first gets the State from the application and the second gets the Entity id of the window from the state.

Now comes the fun part, actually adding some widgets to our window. In this example we add a button to the window and set some inline style properties. Firstly we create the button instance with the usual Button::new() function. Then we call the build() function on this instance which consumes it into the application. Once the build function is called we no longer have direct access to the widget instance, so local data that belongs to the instance is set either with arguments to the new() function or with setter functions between the new() and build() functions.

The build function takes 3 arguments: the state, the entity id of the parent widget (in this case the window), and a builder closure that allows us to set the style properties of the widget. For this example we set the width, height, and background color.

The last line inside of main starts our application main loop.

You can run this example with: ```cargo run --example hello_gui```
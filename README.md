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

The first line inside the main function creates our application. A window closure allows us to set properties of the window such as its title. Unfortunately tuix is currently a single window GUI, but multiwindow is coming soon! (maybe)

After creating the application we have a couple of helper lines to make the code more readable. The first gets the State from the application and the second gets the Entity id of the window from the state.

Now comes the fun part, actually adding some widgets to our window. In this example we add a button to the window and set some inline style properties. Firstly we create the button instance with the usual Button::new() function. Then we call the build() function on this instance which consumes it into the application. Once the build function is called we no longer have direct access to the widget instance, so local data that belongs to the instance is set either with arguments to the new() function or with setter functions between the new() and build() functions.

The build function takes 3 arguments: the state, the entity id of the parent widget (in this case the window), and a builder closure that allows us to set the style properties of the widget. For this example we set the width, height, and background color.

The last line inside of main starts the application main loop.

You can run this example with: ```cargo run --example hello_gui```

## Creating your own widgets

Creating your own widgets in tuix is quite easy. There are just 4 steps.

### Step 1 - Defining the widget struct

The first step in creating a widget is to define a struct for it. You can have data inside the struct but this data will be local data.

```
pub struct MyAwesomeWidget {
  pub some_local_data: f32,
}
```

### Step 2 - Implement the widget struct

Techinally this step is optional, but if you've got data inside your widget then it's just good practice.

```
impl MyAwesomeWidget {
  pub fn new() -> Self {
    MyAwesomeWidget {
      some_local_data: 42.0
    }
  }
}
```

### Step 3 - Implement the BuildHandler trait

So this trait has one function, on_build(), which is called when a widget is built for the first time. The purpose of this function is to allow for inline properties to be set and for composition of widgets. For example, our brand new widget could contain a Button widget that gets created when an instance of MyAwesomeWidget is built.

The on_build function has 3 arguments, a mutable reference to self, a mutable reference to State, and an Entity id. The Enitity id is created by the application when the widget is built and allows us to get and set the properties of the widget within the State. The mutable reference allows us to access, and modify if we want, the local data in the widget instance.

You might have noticed already that the return type of the on_build function is an associated type. This is because the entity that is returned is the one you have access to when you call the build() function on a widget. However, if a widget is composed of multiple things we might need access to more than one entity. An example of this might be a tab container which has an entity for containing the tabs and an entity for containing the content, and in this case we might change Ret to (Entity, Entity) so we can return both. In this case though, we will just return the single entity which corresponds to the instance of our new widget.

```
impl BuildHandler for MyAwesomeWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        let some_button = Button::new().build(state, entity, |builder| builder);
        
        entity
    }
}
```

### Step 4 - Implement the EventHandler trait

```
impl EventHandler for MyAwesomeWidget {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        false
    }
}

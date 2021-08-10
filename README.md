[Join the Discord channel](https://discord.gg/aNkTPsRm2w)

# tuix
Tuix is a cross-platform GUI toolkit written in Rust.

The driving principle behind tuix is to be a self-contained, small-as-possible, but still fast, toolkit for creating graphical user interfaces in Rust. 

![calculator](https://github.com/geom3trik/tuix/blob/develop/docs/calculator.png?raw=true)

## Features

 - Cross-platform
 - GPU based drawing using OpenGl (powered by [femtovg](https://github.com/femtovg/femtovg))
 - Flexible layout system
 - CSS-like styling
 - Animatable style properties
 - Built-in widgets
 - Extendable and reusable behaviour
 
## Including tuix
 
Add tuix to your project by adding `tuix = {git = "https://github.com/geom3trik/tuix", branch = "main"}` to your projects Cargo.toml under dependencies. 

## Getting Started

### Running Examples

You can run any of the examples with:
```
cargo run --example example_name
```
To run any example with the `baseview` backend:
```
cargo run --example example_name --no-default-features --features "baseview"
```

### Hello GUI
Since it's probably best to learn by example, here is the "hello world" of GUI applications in tuix:

```Rust
use tuix::*;

fn main() {

    let app = Application::new(WindowDescription::new().with_title("Hello GUI"), |state, window| {
        
        Button::with_label("Button")
            .build(state, window.entity(), |builder| {
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::from("#ff5e1a"))
                    .set_child_space(Stretch(1.0)
            });
    });

    app.run();
}
```

You can run this example with: ```cargo run --example hello_gui```

# Overview
The general idea of tuix is that widgets contain the application data which is modified in response to events. Let's build a simple counter application to see how this works:

First we need a widget to hold our data. In tuix a widget is just a struct:

```Rust
pub struct Counter {
    value: i32,
    label: Entity,
}
```
Inside our counter widget we have a field for the value of the counter and a field which will store the `Entity` of the label widget that will display the value. An `Entity` is an id which is used to modify `State`, a sort of database for the gui state of the app. 

While `State` stores the data for the GUI, like style properties and events, the data inside the widgets, like our `value` is application data defined by the user. Once the the widget is built into the app this data can only be accessed by the widget at three different stages:

1. During the build stage which happens only once when a widget is built
2. During the event handling stage which happens whenever the widget receives an event
3. During the drawing phase which happens when the app needs to be redrawn

Next, we provide an implementation for the `Counter` so an instance can be created and the initial value can be set:

```Rust
impl Counter {
    pub fn new() -> Self {
        Self {
            value: 0,
            label: Entity::default(),
        }
    }
    
    fn with_initial_value(mut self, val: i32) -> Self {
        self.value = val;
        self
    }
}
```

Next we define the messages that the counter should respond to. In tuix, any struct or enum that implements `Debug`, `Clone`, and `PartialEq` can be a message:

```Rust
pub enum CounterMessage {
    Increment,
    Decrement,
}
```

Now, to make our counter struct an actual widget it needs to implement the `Widget` trait. Within the `Widget` trait is a method called `on_build` which will describe how our widget is constructed with all of its sub-widgets:

```Rust
impl Widget for Counter {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        Button::with_label("increment")
            .on_press(Event::new(CounterMessage::Increment))
            .build(state, entity, |builder| builder.class("increment"))
        
        Button::with_label("decrement")
            .on_press(Event::new(CounterMessage::Decrement))
            .build(state, entity, |builder| builder.class("decrement"));
            
        self.label = Label::new("0")
            .build(state, entity, |builder| builder);
            
        entity.set_element(state, "counter")
    }
}
```
There's a lot going on here so let's break it down. The `on_build()` method will be called when an instance of our counter widget is constructed using `build()`. 

Inside this method we create two buttons and a label. The first button is our increment button so we give it an event to emit when pressed with the `Increment` message we defined before. We've also given the button the class name "increment" so we can more easily style the button. Notice that for the parent we've used `entity`, which is the id given to our counter widget instance when built.  

We repeat this process for the decrement button, but this time with an event message of `Decrement` and a class name of "decrement". Then, we construct the label and set the text to the intial value stored in the `Counter` instance, which may have been initialised with the `with_initial_value` method. Lastly we set the element name of our counter widget to "counter", so we can use this name to style the entire widget.

With the building part done we can now move on to event handling. The `Widget` trait has a another method called `on_event()`, which we can specify so that our counter can react to the events sent by our buttons:

```Rust
fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
    if let Some(counter_event) = event.message.downcast::<CounterMessage>() {
        match counter_event {
            CounterMessage::Increment => {
                self.value += 1;
                self.label.set_text(state, &self.value.to_string());
                event.consume();
            }

            CounterMessage::Decrement => {
                self.value -= 1;
                self.label.set_text(state, &self.value.to_string());
                event.consume();
            }
        }
    }
}
```
To react to events we override the `on_event()` method. This function is similar to `on_build` but with the addition of an `event` argument. Because our counter widget could receive multiple types of events, we need to try downcasting the event message into the `CounterMessage` that we are looking for. After this, we match on the variants, update the internal `value`, and set the text of the `label` accordingly.

It's worth spending a moment to discuss what's happening here. How does the counter receive the messages? This is roughly the order of events happening in tuix when the user presses the increment button:

1. Increment button pushes the 'on_press' event (in this case an event with a `CounterMessage::Increment` message) into an event queue in `State`.
2. Events from state are moved to an `EventManager` which then propagates those events through the tree of widgets. By default event are sent down the tree to the target and then back up to the root. We didn't specify a target for the `on_press` event so it defaults to the button itself.
3. Because our counter widget is the parent of the button it receives the event during the down phase of propagation and also on the up phase. We intercept this event and react to it, incrementing the value and changing the label text. To prevent the counter incrementing again during the up phase we consume the event with `event.consume()`.
4. Because we set the text on the label, which is a style property, this automatically triggers a redraw of the UI. In general, relayout and redraw will only trigger when a property that would affect the layout or visuals of the app changes (there is also a way to manually trigger them).

Before we finish off this counter example by placing our new counter widget into an app, we need to set some style properties. We could use inline styles within our rust code to do this, but for better reusability we'll use CSS, which can be defined in its own file. You can find some example styling for this counter in the 'examples/themes' directory of tuix, under the name 'counter_theme.css'.

Now that we've created our widget we need to put it into an app:

```Rust
static THEME: &'static str = include_str!("themes/counter_theme.css");

fn main() {
    // Create the app
    let app = Application::new(|state, window| {
        state.add_theme(THEME);
        
        window.set_title("Counter").set_inner_size(400, 100);

        Counter::new()
            .with_initial_value(50)
            .build(state, window.entity(), |builder| builder);
  
    });

    app.run();
}
```
Now we create an app which gives us a closure with three arguments. The first is a `WindowDescription` we can use to set the properties of the window like the title and size, which we do last because the closure expects the window description to be returned. The second is the `State` which is created by the app and passed to all widgets during building, events, and drawing. The final argument of the closure is an `Entity` id to the window which acts as the top level widget in the tree.  

The first line of the closure adds the stylsheet to the app. Next, we create an instance of our counter, set the initial value, and then build the widget into the app the same way we did with the sub-widgets of the counter. Then, as mentioned before, we set the window title and intial size and return the window description.

And we're done! 

![counter](https://github.com/geom3trik/tuix/blob/develop/docs/counter.png?raw=true)

You can find the full code for this example in the examples folder under the name 'counter.rs'. Run it with:
```Bash
cargo run --example counter
```

# Tuix Guide (In Development)

You can find a guide to getting started with Tuix here: [Guide](https://geom3trik.github.io/tuix-book/)



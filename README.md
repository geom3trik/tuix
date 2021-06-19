<p align="center"><img src="https://github.com/geom3trik/tuix/blob/main/docs/tuix-logo.png" width="320" height="180" alt="logo"></p>

---
A cross-platform GUI toolkit written in the Rust programming langauge.

The driving principle behind tuix is to be a self-contained, small-as-possible, but still fast, toolkit for creating graphical user interfaces in Rust. 

<p align="center"><a href="https://github.com/geom3trik/tuix/blob/main/examples/calculator.rs"><img src="https://github.com/geom3trik/tuix/blob/develop/docs/calculator.png" alt="calculator"></a></p>

[Join the Discord channel!](https://discord.gg/aNkTPsRm2w)

## Features

 - Cross-platform
 - GPU based drawing using opengl (powered by [femtovg](https://github.com/femtovg/femtovg))
 - Flexbox-based layout system
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
use tuix::widgets::Button;
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::style::themes::DEFAULT_THEME;

fn main() {
    let mut app = Application::new(|win_desc, state, window| {

        state.insert_theme(DEFAULT_THEME);

        Button::new().build(state, window, |builder| {
            builder.set_text("Button")
        });

        win_desc.with_title("Hello GUI")
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
Inside our counter widget we have a field for the value of the counter and a field which will store the `Entity` of the label widget that will display the value. An `Entity` is an id which is used to modify `State`, a sort of database for the state of the app. 

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

Now, to make our counter struct an actual widget it needs to implement two traits: `BuildHandler` and `EventHandler`. The first is the `BuildHandler` which has a single function called `on_build` which will describe how our widget is constructed with all of its sub-widgets:

```Rust
impl BuildHandler for Counter {
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

With the building part done we can now implement the `EventHandler` trait so that our counter can react to the events sent by our buttons:

```Rust
impl EventHandler for Counter {
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
}
```
To react to events we override the `on_event()` method. This function is similar to `on_build` but with the addition of an `event` argument. Because our counter widget could receive multiple types of events, we need to try downcasting the event message into the `CounterMessage` that we are looking for. After this, we match on the variants, update the internal `value`, and set the text of the `label` accordingly.

It's worth spending a moment to discuss what's happening here. How does the counter receive the messages? This is roughly the order of events happening in tuix when the user presses the increment button:

1. Increment button pushes the 'on_press' event (in this case an event with a `CounterMessage::Increment` message) into an event queue in `State`.
2. Events from state are moved to an `EventManager` which then propagates those events through the hierarchy of widgets. By default event are sent down the hierarchy to the target and then back up to the root. We didn't specify a target for the `on_press` event so it defaults to the button itself.
3. Because our counter widget is the parent of the button it receives the event during the down phase of propagation and also on the up phase. We intercept this event and react to it, incrementing the value and changing the label text. To prevent the counter incrementing again during the up phase we consume the event with `event.consume()`.
4. Because we set the text on the label, which is a style property, this automatically triggers a redraw of the UI. In general, relayout and redraw will only trigger when a property that would affect the layout or visuals of the app changes (there is also a way to manually trigger them).

Before we finish off this counter example by placing our new counter widget into an app, we need to set some style properties. We could use inline styles within our rust code to do this, but for better reusability we'll use CSS, which can be defined in its own file. You can find some example styling for this counter in the 'examples/themes' directory of tuix, under the name 'counter_theme.css'.

Now that we've created our widget we need to put it into an app:

```Rust
static THEME: &'static str = include_str!("themes/counter_theme.css");

fn main() {
    // Create the app
    let app = Application::new(|win_desc, state, window| {
        state.add_theme(THEME);

        Counter::new()
            .with_initial_value(50)
            .build(state, window, |builder| builder);
            
        win_desc.with_title("Counter").with_inner_size(400, 100)
    });

    app.run();
}
```
Now we create an app which gives us a closure with three arguments. The first is a `WindowDescription` we can use to set the properties of the window like the title and size, which we do last because the closure expects the window description to be returned. The second is the `State` which is created by the app and passed to all widgets during building, events, and drawing. The final argument of the closure is an `Entity` id to the window which acts as the top level widget in the hierarchy.  

The first line of the closure adds the stylsheet to the app. Next, we create an instance of our counter, set the initial value, and then build the widget into the app the same way we did with the sub-widgets of the counter. Then, as mentioned before, we set the window title and intial size and return the window description.

And we're done! 

<p align="center"><a href="https://github.com/geom3trik/tuix/blob/main/examples/counter.rs"><img src="https://github.com/geom3trik/tuix/blob/develop/docs/counter.png" alt="counter"></a></p>

You can find the full code for this example in the examples folder under the name 'counter.rs'. Run it with:
```Bash
cargo run --example counter
```

# How tuix works

Tuix can be thought of as 5 seperate processes which happen in order:

- Building
- Events
- Styling
- Layout
- Drawing

## Building
Building is the process of creating the widgets in the application. This can be done before the application loop begins, or in response to an event. The `hello_gui` example shown above demonstrates how to create and then build a button widget. The `build()` function takes three parameters: a mutable reference to `State`, the `Entity` id of the parent widget, and a closure which provides a builder which can be used to set inline style properties on the button. 

More information about building widgets can be found on the [Building Widgets](https://github.com/geom3trik/tuix/wiki/Building-Widgets) wiki page.

## Event Handling
Tuix uses an event queue to pass custom messages between widgets.

[Events](https://github.com/geom3trik/tuix/wiki/Events)


## Styling
Tuix uses a modified subset of CSS properties to perform styling of widgets. The `hello_gui` example uses the `DEFAULT_THEME` provided within the tuix crate. The `custom_styling` example shows how to style the button with a custom theme provided by a stylesheet in a css file, as well as inline styling using setter functions on the builder.

```Rust
extern crate tuix;

use tuix::widgets::Button;
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::style::themes::DEFAULT_THEME;

use tuix::style::Length;

// This example uses a custom theme defined in the 'custom_theme.css' stylesheet
static CUSTOM_THEME: &'static str = include_str!("themes/custom_theme.css");

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.insert_theme(DEFAULT_THEME);

        // Properties defined in CUSTOM_THEME override the same properties defined in DEFAULT_THEME
        state.insert_theme(CUSTOM_THEME);

        Button::new().build(state, window, |builder| {
            builder
                // These are inline properties which cannot be overriden by a theme
                .set_left(Length::Pixels(100.0))    
                .set_top(Length::Pixels(50.0))
                .set_text("Button")
        });

        win_desc.with_title("Hello GUI")
    });

    app.run();
}
```

More information about styling can be found on the [Styling Widgets](https://github.com/geom3trik/tuix/wiki/Styling-Widgets) wiki page.

## Layout
Tuix uses a flexbox model to perform layout. The layout process positions the widgets based on the style properties you give them. Users should also be familiar with the [css box model](https://www.w3schools.com/css/css_boxmodel.asp).

The example `flexible_layout` shows how to create three flexible elements with their own inline properties as well as showing how to center a button widget within another element. By default, the flex direction of elements, including the window, is set to column.

```Rust
extern crate tuix;

use tuix::widgets::{Element, Button};
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::style::themes::DEFAULT_THEME;

use tuix::style::{Length, Color, JustifyContent, AlignItems};

// This example uses a custom theme defined in the 'custom_theme.css' stylesheet
static CUSTOM_THEME: &'static str = include_str!("themes/custom_theme.css");

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.insert_theme(DEFAULT_THEME);

        // Properties defined in CUSTOM_THEME override the same properties defined in DEFAULT_THEME
        state.insert_theme(CUSTOM_THEME);

        // An element is the simplest widget. It has no built in styling and doesn't handle any events.
        let first = Element::new().build(state, window, |builder| 
            builder
                // Allow the element to grow in size to fill the parent (in height)
                .set_flex_grow(1.0)
                // The flexbox way of centering the child elements
                .set_justify_content(JustifyContent::Center)
                .set_align_items(AlignItems::Center)
                
                .set_background_color(Color::rgb(100,50,50))
        );

        Element::new().build(state, window, |builder| 
            builder
                // A flex-grow of 2 rsults in a twice as large element in this case
                .set_flex_grow(2.0)
                .set_background_color(Color::rgb(50,100,50))
        );

        Element::new().build(state, window, |builder| 
            builder
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(50,50,100))
        );

        // The button is now a child of the first element instead of the window
        Button::new().build(state, first, |builder| {
            builder.set_text("Button")
        });

        win_desc.with_title("Flexible Layout")
    });

    app.run();
}
```

More information about how widgets are psoitioned can be found on the [Layout Widgets](https://github.com/geom3trik/tuix/wiki/Layout-Widgets) wiki page.

## Drawing
After styling, the widgets are drawn to the window. The visual look of the widgets is determined by the style propeties set on them, but it's also possible to override this with a custom drawing function for your own custom widgets.

[Rendering Widgets](https://github.com/geom3trik/tuix/wiki/Rendering-Widgets)

## Building a simple synth

<p align="center"><a href="https://github.com/geom3trik/tuix_audio_synth"><img src="https://github.com/geom3trik/tuix_audio_synth/blob/main/screenshot.png" alt="calculator"></a></p>

You can find a more complex example application and a tutorial for building a simple synth with a tuix gui here: https://github.com/geom3trik/tuix_audio_synth




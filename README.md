[Join the Discord channel](https://discord.gg/aNkTPsRm2w)

# tuix
Tuix is a cross-platform GUI toolkit written in Rust.

The driving principle behind tuix is to be a self-contained, small-as-possible, but still fast, toolkit for creating graphical user interfaces in Rust. 

## Features

 - Cross-platform
 - 2D GPU based drawing using opengl (powered by femtovg)
 - Flexbox-based layout system
 - CSS-like styling
 - Animatable style properties
 - Built-in composable widgets

## Getting Started

### Running Examples

You can run any of the examples with:
```
cargo run --example example_name
```

### Hello GUI
Since it's probably best to learn by example, here is the "hello world" of GUI applications:

```Rust
extern crate tuix;

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

# How tuix works

Tuix can be thought of as 5 seperate processes which happen in order:

- Building
- Layout
- Styling
- Events
- Drawing

## Building
Building is the process of creating the widgets in the application. This can be done before the application loop begins, or in response to an event.

[Building Widgets](https://github.com/geom3trik/tuix/wiki/Building-Widgets)

## Layout
Tuix uses a flexbox model to perform layout. The layout process positions the widgets based on the style properties you give them.

[Layout Widgets](https://github.com/geom3trik/tuix/wiki/Layout-Widgets)

## Styling
After layout, the widgets are styled using the CSS properties you give them.

[Styling Widgets](https://github.com/geom3trik/tuix/wiki/Styling-Widgets)

## Event Handling
Tuix uses an event queue to pass custom messages between widgets.

[Events](https://github.com/geom3trik/tuix/wiki/Events)

## Drawing
After styling, the widgets are drawn to the window. The visual look of the widgets is determined by the style propeties set on them, but it's also possible to override this with a custom drawing function for your own custom widgets.

[Rendering Widgets](https://github.com/geom3trik/tuix/wiki/Rendering-Widgets)

## Building a simple synth

![alt text](https://github.com/geom3trik/tuix_audio_synth/blob/main/screenshot.png?raw=true)

You can find a more complex example application and a tutorial for building a simple synth with a tuix gui here: https://github.com/geom3trik/tuix_audio_synth




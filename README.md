[Join the Discord channel](https://discord.gg/aNkTPsRm2w)

# tuix
Tuix is a cross-platform GUI toolkit written in Rust.

The driving principle behind tuix is to be a self-contained, small-as-possible, but still fast, toolkit for creating graphical user interfaces in Rust. 

![calculator](https://github.com/geom3trik/tuix/blob/develop/docs/calculator.png?raw=true)

## Features

### Build Cross-Platform Applications

 Image goes here

### Flexible Layout

 Image goes here

### Fully Customisable Styling

 Image goes here

### Animatable Style Properties

 Image goes here

### A Reactive Data Model

 Image goes here

### Numerous Built-In Widgets

 Image goes here

### Custom Widgets

 Image goes here

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
                    .set_child_space(Stretch(1.0))
            });
    });

    app.run();
}
```

You can run this example with: ```cargo run --example hello_gui```

# Tuix Book (In Development)
[The Book](https://geom3trik.github.io/tuix-book/)



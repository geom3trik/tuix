<p align="center"><img src="https://github.com/geom3trik/tuix/blob/main/docs/tuix-logo.png" width="320" height="180" alt="logo"></p>

---

![GitHub](https://img.shields.io/github/license/geom3trik/tuix)
[![Build](https://github.com/geom3trik/tuix/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/geom3trik/tuix/actions/workflows/build.yml)
[![Discord](https://img.shields.io/discord/791142189005537332.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/aNkTPsRm2w)

Tuix is a cross-platform GUI toolkit written in Rust.

The driving principle behind tuix is to be a self-contained, small-as-possible, but still fast, toolkit for creating graphical user interfaces in Rust. 

<p align="center"><a href="https://github.com/geom3trik/tuix/blob/main/examples/calculator.rs"><img src="https://github.com/geom3trik/tuix/blob/main/docs/calculator.png" alt="calculator"></a></p>

## Features

 - Build Cross-Platform Applications
 - Flexible Layout
 - Fully Customisable Styling
 - Animatable Style Properties
 - A Reactive Data Model
 - Numerous Built-In Widgets
 - Custom Widgets

<p align="center"><a href="https://github.com/geom3trik/tuix/blob/main/examples/editor.rs"><img src="https://github.com/geom3trik/tuix/blob/main/docs/editor.png" alt="editor"></a></p>

## Including tuix
 
Add tuix to your project by adding `tuix = {git = "https://github.com/geom3trik/tuix", branch = "main"}` to your projects Cargo.toml under dependencies. 

### Debug Performance
Note: in order to get acceptable performance when running your app in Debug mode, it is highly recommended to add this to the Cargo.toml of your root crate:

```toml
[profile.dev.package.tuix_core]
opt-level = 2
[profile.dev.package.femtovg]
opt-level = 2
```

## Getting Started

### Running Examples

You can run any of the examples with:
```
cargo run --example example_name --release
```
To run any example with the `baseview` backend:
```
cargo run --example example_name --no-default-features --features "baseview" --release
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

You can run this example with: ```cargo run --example hello_gui --release```

# Tuix Book (In Development)
[The Book](https://geom3trik.github.io/tuix-book/)



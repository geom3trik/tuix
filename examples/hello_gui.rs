extern crate tuix;

use image::{buffer::Pixels};
use tuix::{Element, widgets::Button};
use tuix::Application;

use tuix::events::BuildHandler;

use tuix::PropSet;

use tuix::Length;

use tuix::style::themes::DEFAULT_THEME;

fn main() {
    let app = Application::new(|win_desc, state, window| {
        state.insert_theme(DEFAULT_THEME);

        state.resource_manager.add_image("lena", "resources/images/lena.png");

        Element::new().build(state, window, |builder| builder.set_background_image("image"));

        win_desc.with_title("Hello GUI")
    });

    app.run();
}

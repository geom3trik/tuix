

extern crate tuix;

use tuix::*;
use tuix::widgets::*;

static THEME: &'static str = include_str!("themes/menus_theme.css");

fn main() {

    let window_description = WindowDescription::new().with_title("Menus");

    // Create the app
    let app = Application::new(window_description, |state, window| {
        state.add_theme(THEME);

        let menu_bar = MenuBar::new().build(state, window.entity(), |builder| {
            builder.set_layout_type(LayoutType::Row)
        });
        
        let menu1 = Menu::new("File").build(state, menu_bar, |builder| {
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
                .class("menu")
        });

        Button::with_label("Item 1").build(state, menu1, |builder| builder.class("item"));
        Button::with_label("Item 2").build(state, menu1, |builder| builder.class("item"));
        Button::with_label("Item 3")
            .on_press(|widget, state, button|{
                button.emit(state, WindowEvent::WindowClose);
            })
            .build(state, menu1, |builder| builder.class("item"));
        Button::with_label("Item 4").build(state, menu1, |builder| builder.class("item"));
        /*
        let menu1 = Menu::new().build(state, menu_bar, |builder| {
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
                .class("menu")
        });

        Button::with_label("Item 1").build(state, menu1, |builder| builder.class("item"));
        Button::with_label("Item 2").build(state, menu1, |builder| builder.class("item"));
        Button::with_label("Item 3")
            .on_press(|widget, state, button|{
                button.emit(state, WindowEvent::WindowClose);
            })
            .build(state, menu1, |builder| builder.class("item"));
        Button::with_label("Item 4").build(state, menu1, |builder| builder.class("item"));    
        */


        // // Button::new().build2(state, menu1, |builder| builder.class("spacer2"));

        // Button::with_label("SubItem 1").build(state, menu2, |builder| builder.class("item"));
        // Button::with_label("SubItem 2").build(state, menu2, |builder| builder.class("item"));
        // Button::with_label("SubItem 3")
        //     .on_press(Event::new(WindowEvent::WindowClose))
        //     .build(state, menu2, |builder| builder.class("item"));

        // Button::new().build(state, menu1, |builder| builder.class("spacer2"));

        // let menu3 = Menu::new("SubSubMenu", MenuPosition::Right).build(state, menu2, |builder| {
        //     builder.class("item").class("submenu")
        // });

        // Button::with_label("SubSubItem 1").build(state, menu3, |builder| builder.class("item"));
        // Button::with_label("SubSubItem 2").build(state, menu3, |builder| builder.class("item"));
        // Button::with_label("SubSubItem 3")
        //     .on_press(Event::new(WindowEvent::WindowClose))
        //     .build(state, menu3, |builder| builder.class("item"));

        // let button = Button::with_label("Right Click Me").build(state, window, |builder| {
        //     builder
        //         .set_left(Units::Pixels(100.0))
        //         .set_top(Units::Pixels(100.0))
        //         .set_width(Units::Pixels(150.0))
        //         .set_height(Units::Pixels(30.0))
        //         .set_background_color(Color::green())
        // });

        // let (_container, menu) = ContextMenu::new().build(state, button, |builder| {
        //     builder
        //         .set_width(Units::Percentage(1.0))
        //         .set_height(Units::Percentage(1.0))
        // });

        // menu.set_width(state, Units::Pixels(100.0));

        // Button::with_label("Option 1").build(state, menu, |builder| {
        //     builder
        //         .set_height(Units::Pixels(30.0))
        //         .set_background_color(Color::rgb(50, 50, 50))
        // });

        // Button::with_label("Option 2").build(state, menu, |builder| {
        //     builder
        //         .set_height(Units::Pixels(30.0))
        //         .set_background_color(Color::rgb(50, 50, 50))
        // });

        // Button::with_label("Option 3").build(state, menu, |builder| {
        //     builder
        //         .set_height(Units::Pixels(30.0))
        //         .set_background_color(Color::rgb(50, 50, 50))
        // });
    });

    app.run();
}
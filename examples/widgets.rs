// Example which shows all of the supported widgets

extern crate tuix;

use tuix::*;

static THEME: &'static str = include_str!("themes/light_theme.css");



fn main() {
    Application::new(|win_desc, state, window|{

        state.style.parse_theme(THEME);

        // Menu bar
        let menu_bar = Element::new().build(state, window, |builder| 
            builder
                .class("menu_bar")
                .set_height(Length::Pixels(40.0))
                .set_width(Length::Percentage(1.0))
        );

        // Horizontal Container
        let hbox = HBox::new().build(state, window, |builder|
            builder.set_flex_grow(1.0)
        );

        // Resizable Vertical Container
        let rvbox = ResizableVBox::new().build(state, hbox, |builder| 
            builder
                .set_width(Length::Pixels(300.0))
                .set_height(Length::Percentage(1.0))
                .set_background_color(Color::rgb(100,50,50))
        );

        // 
        let panel = Panel::new("Buttons").build(state, rvbox, |builder| builder);

        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Button").build(state, row, |builder| builder);
        Button::with_label("Press Me").build(state, row, |builder| builder);
    
        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Checkbox").build(state, row, |builder| builder);
        Checkbox::new(false).build(state, row, |builder| builder);


        // Tabs
        // let (tab_bar, tab_container) = TabContainer::new().build(state, window, |builder| builder);

        // Button::with_label("First")
        // .on_press(Event::new(TabEvent::SwitchTab(0)))
        // .build(state, tab_bar, |builder| builder.set_checked(true));
        // let first = Button::new().build(state, tab_container, |builder| builder.class("item1"));
        // Button::with_label("First Button").build(state, first, |builder| builder.class("test"));


        win_desc
    }).run();    

}
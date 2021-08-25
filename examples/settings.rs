

use tuix::*;

const STYLE: &str = r#"

    dropdown {
        border-radius: 3px;
        color: #ababab;
        border-width: 1px;
        border-color: #ababab;
    }

    dropdown>.header>.label {
        color: black;
        child-left: 10px;
        child-right: 1s;
    }

    dropdown>.header>.icon {
        color: #686868;
    }

    textbox {
        border-radius: 3px;
        border-width: 1px;
        border-color: #ababab;
        child-top: 1s;
        child-bottom: 1s;
        child-left: 10px;
        child-right: 1s;
        color: black;
        font-size: 14px;
    }

    label {
        font-size: 14px;
    }
"#;

#[derive(Default)]
pub struct Settings {

}

impl Widget for Settings {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let row = Element::new().build(state, entity, |builder| 
            builder
                .set_layout_type(LayoutType::Row)
        );

        let column = Element::new().build(state, row,|builder| 
            builder
                .set_width(Pixels(300.0))
        );

        Element::new().build(state, column, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_text("Settings")
                .set_child_space(Stretch(1.0))
                .set_color(Color::black())
                .set_background_color(Color::rgb(246,246,246))
                .set_font_size(18.0)
        );

        Element::new().build(state, column, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_text("Audio")
                .set_child_left(Pixels(40.0))
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_color(Color::black())
                .set_background_color(Color::rgb(193, 167, 231))
                .set_color(Color::black())
                .set_border_width(Pixels(1.0))
                .set_border_color(Color::rgb(171, 171, 171))
                .set_left(Pixels(-1.0))
                .set_right(Pixels(-1.0))
        );

        Element::new().build(state, column, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_text("Midi")
                .set_child_left(Pixels(40.0))
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_color(Color::black())
                .set_background_color(Color::rgb(255,255,255))
                .set_border_width(Pixels(1.0))
                .set_border_color(Color::rgb(171, 171, 171))
                .set_left(Pixels(-1.0))
                .set_right(Pixels(-1.0))
                .set_top(Pixels(-1.0))
        );

        Element::new().build(state, row, |builder|
            builder
                .set_width(Pixels(1.0))
                .set_background_color(Color::rgb(171, 171, 171))
        );

        let panels = Element::new().build(state, row,|builder| 
            builder
                .set_child_space(Pixels(30.0))
        );

        Element::new().build(state, panels, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_text("Audio")
                .set_child_space(Stretch(1.0))
                .set_color(Color::black())
                .set_background_color(Color::rgb(246,246,246))
                .set_space(Pixels(0.0))
                .set_font_size(18.0)
        );

        Element::new().build(state, panels, |builder|
            builder
                .set_height(Pixels(1.0))
                .set_background_color(Color::rgb(171, 171, 171))
                .set_space(Pixels(0.0))
        );

        Label::new("Audio Device").build(state, panels, |builder|
            builder
                .set_color(Color::black())
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_height(Pixels(30.0))
                .set_top(Pixels(30.0))
                .set_bottom(Pixels(10.0))
                .set_font_size(18.0)
        );

        let audio_device = Element::new().build(state, panels, |builder|
            builder
                .set_background_color(Color::rgb(246,246,246))
                .set_outer_shadow_h_offset(Pixels(1.0))
                .set_outer_shadow_v_offset(Pixels(1.0))
                .set_outer_shadow_blur(Pixels(4.0))
                .set_outer_shadow_color(Color::rgba(0,0,0,64))
                .set_child_space(Pixels(20.0))
                //.set_border_radius(Pixels(3.0))
        );

        Label::new("System Device").build(state, audio_device, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_color(Color::black())
                .set_font_size(18.0)
        );

        Element::new().build(state, audio_device, |builder|
            builder
                .set_height(Pixels(1.0))
                .set_background_color(Color::rgb(171, 171, 171))
                .set_bottom(Pixels(10.0))
        );

        let row = Row::new().build(state, audio_device, |builder| 
            builder
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Stretch(1.0))
                .set_height(Pixels(30.0))
                .set_top(Pixels(20.0))
        );

        Label::new("Driver Model").build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_color(Color::black())
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Pixels(200.0))
        );

        Dropdown::new("Jack")
            .build(state, row, |builder| {
                builder
                    .set_width(Pixels(200.0))
                    .set_height(Pixels(30.0))
            });

        let row = Row::new().build(state, audio_device, |builder| 
            builder
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Stretch(1.0))
                .set_height(Pixels(30.0))
                .set_top(Pixels(20.0))
        );

        Label::new("Sample Rate").build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_color(Color::black())
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Pixels(200.0))
        );

        Dropdown::new("Auto")
            .build(state, row, |builder| {
                builder
                    .set_width(Pixels(200.0))
                    .set_height(Pixels(30.0))
            });


        // 

        let row = Row::new().build(state, audio_device, |builder| 
            builder
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Stretch(1.0))
                .set_height(Pixels(30.0))
                .set_top(Pixels(30.0))
        );

        Label::new("Outputs").build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_color(Color::black())
                .set_font_size(18.0)
        );

        Button::with_label("+ Add Device")
            .build(state, row, |builder| {
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(20.0))
                    .set_background_color(Color::rgb(128, 0, 238))
                    .set_right(Auto)
                    .set_left(Stretch(1.0))
                    .set_child_space(Stretch(1.0))
                    .set_border_radius(Pixels(3.0))
                    .set_color(Color::rgb(246,246,246))
                    .set_font_size(14.0)
                    .set_outer_shadow_v_offset(Pixels(1.0))
                    .set_outer_shadow_blur(Pixels(4.0))
                    .set_outer_shadow_color(Color::rgba(0,0,0,64))
            });

        Element::new().build(state, audio_device, |builder|
            builder
                .set_height(Pixels(1.0))
                .set_background_color(Color::rgb(171, 171, 171))
                .set_bottom(Pixels(10.0))
        );

        let row = Row::new().build(state, audio_device, |builder| 
            builder
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Stretch(1.0))
                .set_height(Pixels(30.0))
                .set_top(Pixels(20.0))
        );

        Label::new("Name").build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_color(Color::black())
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Pixels(200.0))
        );

        Textbox::new("Stereo Speaker Out")
            .build(state, row, |builder| {
                builder
                    .set_width(Stretch(1.0))
                    .set_height(Pixels(30.0))
            });

        let row = Row::new().build(state, audio_device, |builder| 
            builder
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Stretch(1.0))
                .set_height(Pixels(30.0))
                .set_top(Pixels(20.0))
        );

        let minus = Button::new()
        .build(state, row, |builder| {
            builder
                .set_width(Pixels(20.0))
                .set_height(Pixels(20.0))
                .set_border_color(Color::rgb(100, 100, 100))
                .set_border_width(Pixels(2.0))
                .set_top(Stretch(1.0))
                .set_bottom(Stretch(1.0))
                .set_right(Pixels(10.0))
                .set_child_space(Stretch(1.0))
                .set_border_radius(Pixels(10.0))
                .set_color(Color::black())
        });

        Element::new().build(state, minus, |builder|
            builder

                .set_height(Pixels(2.0))
                .set_width(Pixels(10.0))
                .set_background_color(Color::rgb(100, 100, 100))
        );

        Label::new("Port 1").build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_color(Color::black())
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Pixels(170.0))
        );

        Dropdown::new("system:playback_1")
            .build(state, row, |builder| {
                builder
                    .set_width(Pixels(200.0))
                    .set_height(Pixels(30.0))
            });

        let row = Row::new().build(state, audio_device, |builder| 
            builder
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Stretch(1.0))
                .set_height(Pixels(30.0))
                .set_top(Pixels(20.0))
        );

        let minus = Button::new()
        .build(state, row, |builder| {
            builder
                .set_width(Pixels(20.0))
                .set_height(Pixels(20.0))
                .set_border_color(Color::rgb(100, 100, 100))
                .set_border_width(Pixels(2.0))
                .set_top(Stretch(1.0))
                .set_bottom(Stretch(1.0))
                .set_right(Pixels(10.0))
                .set_child_space(Stretch(1.0))
                .set_border_radius(Pixels(10.0))
                .set_color(Color::black())
        });

        Element::new().build(state, minus, |builder|
            builder

                .set_height(Pixels(2.0))
                .set_width(Pixels(10.0))
                .set_background_color(Color::rgb(100, 100, 100))
        );

        Label::new("Port 2").build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_color(Color::black())
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_width(Pixels(170.0))
        );

        Dropdown::new("system:playback_2")
            .build(state, row, |builder| {
                builder
                    .set_width(Pixels(200.0))
                    .set_height(Pixels(30.0))
            });
        
        Button::with_label("+ Add Port")
            .build(state, audio_device, |builder| {
                builder
                    .set_width(Pixels(80.0))
                    .set_height(Pixels(20.0))
                    .set_background_color(Color::rgb(128, 0, 238))
                    .set_top(Pixels(20.0))
                    .set_bottom(Stretch(1.0))
                    .set_right(Auto)
                    .set_left(Stretch(1.0))
                    .set_child_space(Stretch(1.0))
                    .set_border_radius(Pixels(3.0))
                    .set_color(Color::rgb(246,246,246))
                    .set_font_size(14.0)
                    .set_outer_shadow_v_offset(Pixels(1.0))
                    .set_outer_shadow_blur(Pixels(4.0))
                    .set_outer_shadow_color(Color::rgba(0,0,0,64))
            });

        entity
    }
}

fn main() {
    let window_description = WindowDescription::new().with_title("Preferences");
    let app = Application::new(window_description, |state, window|{

        state.add_theme(STYLE);

        Settings::default().build(state, window, |builder| builder);

        window.set_background_color(state, Color::rgb(229,229,229));
    });

    app.run();
}


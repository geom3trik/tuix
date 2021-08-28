use tuix::*;

const STYLE: &str = r#"

    window {
        background-color: #404040;
        
    }

    panel>.header {
        height: 30px;
    }

    panel .container2 {
        child-space: 10px;
        row-between: 10px;
    }

    panel panel .container2 {
        child-right: 0px;
        child-left: 25px;
    }

    panel panel>.header>.label {
        width: 0px;
    }

    length_box {
        height: 30px;
    }

    textbox {
        background-color: #2e2e2e;
        color: white;
    }

    dropdown {
        background-color: #494949;
    }
    
    dropdown .label {
        child-space: 1s;
        color: whte;
    }

    dropdown .icon {
        color: #909090;
    }

    popup {
        background-color: #404040;
        outer-shadow: 0px 2px 5px #40000000;
    }


    list {
        border-width: 1px;
        border-color: #404040;
    }

    list>check_button {
        height: 30px;
        child-space: 1s;
        background-color: #404040;
        color: white;
    }

    list>check_button:hover {
        background-color: #4f4f4f;
    }

    list>check_button:active {
        background-color: #404040;
    }

    list>check_button:checked {
        background-color: #2a97f0;
    }

    list>check_button:focus {
        border-width: 1px;
        border-color: black;
    }

    slider>.track {
        background-color: #2e2e2e;
    }

    slider>.track>.active {
        background-color: #494949;
    }

    slider>.thumb {
        width: 0px;
    }

"#;

fn main() {
    let window_description = WindowDescription::new().with_title("Style Editor");
    let app = Application::new(window_description, |state, window|{
        state.add_theme(STYLE);

        let app_data = AppData::default().build(state, window);
        
        App::default().build(state, app_data, |builder| builder);

    });

    app.run();
}

#[derive(PartialEq)]
pub enum AppEvent {
    SetBorderWidth(Units),
    SetBorderRadius(Units),
    SetBorderRadiusTopLeft(Units),
    SetBorderRadiusTopRight(Units),
    SetBorderRadiusBottomLeft(Units),
    SetBorderRadiusBottomRight(Units),
}

#[derive(Default, Clone, Lens)]
pub struct StyleData {
    pub background_color: Color,
    pub border_width: Units,
    pub border_radius: Units,
    pub border_radius_top_left: Units,
    pub border_radius_top_right: Units,
    pub border_radius_bottom_left: Units,
    pub border_radius_bottom_right: Units,
}

#[derive(Default, Lens)]
pub struct AppData {
    pub style_data: StyleData,
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetBorderWidth(val) => {
                    self.style_data.border_width = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderRadius(val) => {
                    self.style_data.border_radius = *val;
                    self.style_data.border_radius_top_left = *val;
                    self.style_data.border_radius_top_right = *val;
                    self.style_data.border_radius_bottom_left = *val;
                    self.style_data.border_radius_bottom_right = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderRadiusTopLeft(val) => {
                    self.style_data.border_radius_top_left = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderRadiusTopRight(val) => {
                    self.style_data.border_radius_top_right = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderRadiusBottomLeft(val) => {
                    self.style_data.border_radius_bottom_left = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderRadiusBottomRight(val) => {
                    self.style_data.border_radius_bottom_right = *val;
                    entity.emit(state, BindEvent::Update);
                }

                _=> {}
            }
        }
    }
}

#[derive(Default)]
pub struct App {

}

impl Widget for App {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        let row = Row::new().build(state, entity, |builder| builder);
        Canvas::default()
            .bind(AppData::style_data, |style_data| style_data.clone())
            .build(state, row, |builder| 
                builder
                    //.set_background_color(Color::red())
            );
        StyleControls::default().build(state, row, |builder| 
            builder
                .set_background_color(Color::rgb(56,56,56))
        );

        PopupWindow::new("Window Title").build(state, entity, |builder| 
            builder
                .set_width(Pixels(400.0))
                .set_height(Pixels(300.0))
                .set_space(Stretch(1.0))
        );
        
        entity
    }
}

#[derive(Default)]
pub struct Canvas {
    element: Entity,
}

impl Widget for Canvas {
    type Ret = Entity;
    type Data = StyleData;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.element = Element::new().build(state, entity, |builder|
            builder
                .set_width(Pixels(300.0))
                .set_height(Pixels(200.0))
                .set_background_color(Color::rgb(100, 100, 200))
                .set_border_color(Color::black())
        );

        entity.set_child_space(state, Stretch(1.0))
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        self.element
            //.set_background_color(state, data.background_color)
            .set_border_width(state, data.border_width)
            .set_border_radius_top_left(state, data.border_radius_top_left)
            .set_border_radius_top_right(state, data.border_radius_top_right)
            .set_border_radius_bottom_left(state, data.border_radius_bottom_left)
            .set_border_radius_bottom_right(state, data.border_radius_bottom_right);
    }
}

#[derive(Default)]
pub struct StyleControls {

}

impl Widget for StyleControls {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let border_panel = Panel::new("Border").build(state, entity, |builder| 
            builder
                //.set_background_color(Color::blue())
        );

        LengthBox::new("Border Width")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderWidth(data.value()));                
            })
            .build(state, border_panel, |builder| builder);



        let (border_radius_panel, border_radius_panel_header) = Panel::new("BR").build(state, border_panel, |builder| builder);
        
        LengthBox::new("Border Radius")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderRadius(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::border_radius), |val| *val)
            .build(state, border_radius_panel_header, |builder| builder);

        LengthBox::new("Top Left")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderRadiusTopLeft(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::border_radius_top_left), |val| *val)
            .build(state, border_radius_panel, |builder| builder);

        LengthBox::new("Top Right")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderRadiusTopRight(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::border_radius_top_right), |val| *val)
            .build(state, border_radius_panel, |builder| builder);

        LengthBox::new("Bottom Left")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderRadiusBottomLeft(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::border_radius_bottom_left), |val| *val)
            .build(state, border_radius_panel, |builder| builder);
            
        LengthBox::new("Bottom Right")
                .on_changed(|data, state, lengthbox|{
                    lengthbox.emit(state, AppEvent::SetBorderRadiusBottomRight(data.value()));      
                })
                .bind(AppData::style_data.then(StyleData::border_radius_bottom_right), |val| *val)
                .build(state, border_radius_panel, |builder| builder);

        entity.set_width(state, Pixels(300.0))
    }
}

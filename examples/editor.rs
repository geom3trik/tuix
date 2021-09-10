use tuix::*;

const STYLE: &str = r#"

    window {
        background-color: #383838;
        
    }

    window>.header {
        background-color: #404040;
    }

    panel>.header {
        height: 30px;
    }

    panel .container2 {
        child-space: 10px;
        row-between: 10px;
    }

    panel.group {
        right: 10px;
    }

    panel.group .container2 {
        child-right: 0px;
        child-left: 25px;
    }

    panel.group>.header>.label {
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

// So we can use the same color picker for background and border
#[derive(PartialEq, Clone, Copy)]
pub enum ColorPickFor {
    Background,
    Border,
}

impl Default for ColorPickFor {
    fn default() -> Self {
        ColorPickFor::Background
    }
}

#[derive(PartialEq, Clone)]
pub enum AppEvent {

    OpenColorPicker(ColorPickFor),

    // Size
    SetSize(Units),
    SetWidth(Units),
    SetHeight(Units),
    // Space
    SetSpace(Units),
    SetLeft(Units),
    SetRight(Units),
    SetTop(Units),
    SetBottom(Units),
    // Border
    SetBorderWidth(Units),
    SetBorderRadius(Units),
    SetBorderRadiusTopLeft(Units),
    SetBorderRadiusTopRight(Units),
    SetBorderRadiusBottomLeft(Units),
    SetBorderRadiusBottomRight(Units),
    // Background
    SetBackgroundColor(Color),
}

#[derive(Default, Clone, Lens)]
pub struct StyleData {

    current_color: ColorPickFor,

    // Size
    pub size: Units,
    pub width: Units,
    pub height: Units,
    // Space
    pub space: Units,
    pub left: Units,
    pub top: Units,
    pub right: Units,
    pub bottom: Units,

    // Background
    pub background_color: Color,
    // Border
    pub border_width: Units,
    pub border_radius: Units,
    pub border_radius_top_left: Units,
    pub border_radius_top_right: Units,
    pub border_radius_bottom_left: Units,
    pub border_radius_bottom_right: Units,
}

impl StyleData {
    pub fn new() -> Self {
        Self {
            width: Units::Pixels(100.0),
            height: Units::Pixels(100.0),
            ..Default::default()
        }
    }
}

#[derive(Default, Lens)]
pub struct AppData {
    pub style_data: StyleData,
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {

                // Size
                AppEvent::SetSize(val) => {
                    self.style_data.size = *val;
                    self.style_data.width = *val;
                    self.style_data.height = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetWidth(val) => {
                    self.style_data.width = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetHeight(val) => {
                    self.style_data.height = *val;
                    entity.emit(state, BindEvent::Update);
                }

                // Space

                AppEvent::SetSpace(val) => {
                    self.style_data.space = *val;
                    self.style_data.left = *val;
                    self.style_data.right = *val;
                    self.style_data.top = *val;
                    self.style_data.bottom = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetLeft(val) => {
                    self.style_data.left = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetRight(val) => {
                    self.style_data.right = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetTop(val) => {
                    self.style_data.top = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBottom(val) => {
                    self.style_data.bottom = *val;
                    entity.emit(state, BindEvent::Update);
                }

                // Border
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

                // Background
                AppEvent::SetBackgroundColor(color) => {
                    self.style_data.background_color = *color;
                    entity.emit(state, BindEvent::Update);
                }

                _=> {}
            }
        }
    }
}

#[derive(Default)]
pub struct App {
    color_picker: Entity,
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
                .set_child_space(Pixels(10.0))
                .set_row_between(Pixels(10.0))
        );

        self.color_picker = PopupWindow::new("Window Title").build(state, entity, |builder| 
            builder
                .set_width(Pixels(400.0))
                .set_height(Pixels(300.0))
                .set_space(Stretch(1.0))
                //.set_child_space(Stretch(1.0))
        ).entity();

        ColorPicker::new()
        .on_changing(|data, state, color_picker|{
            color_picker.emit(state, AppEvent::SetBackgroundColor(data.color()));
        })
        .build(state, self.color_picker, |builder| 
            builder
                .set_border_width(Pixels(1.0))
                .set_border_color(Color::black())
        );
        
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::OpenColorPicker(_) => {
                    entity.emit_to(state, self.color_picker, PopupEvent::Open);
                }

                _=> {}
            }
        }
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

        

        let canvas = Element::new().build(state, entity, |builder|
            builder
                .set_width(Pixels(400.0))
                .set_height(Pixels(300.0))
                .set_background_color(Color::rgb(255, 255, 255))
        );

        self.element = Element::new().build(state, canvas, |builder|
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(100.0))
                .set_background_color(Color::rgb(20, 80, 200))
        );

        entity.set_child_space(state, Stretch(1.0))
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        self.element
            .set_background_color(state, data.background_color)
            // Size
            .set_width(state, data.width)
            .set_height(state, data.height)
            // Space
            .set_left(state, data.left)
            .set_right(state, data.right)
            .set_top(state, data.top)
            .set_bottom(state, data.bottom)
            //Border
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


        let (background_panel) = Panel::new("Background").build(state, entity, |builder| 
            builder
        );

        let row = Row::new().build(state, background_panel, |builder| 
            builder
                .set_height(Pixels(30.0))
        );

        Label::new("Color:").build(state, row, |builder| builder);
        Button::new()
            .on_press(|data, state, button|{
                button.emit(state, AppEvent::OpenColorPicker(ColorPickFor::Background));
            })
            .build(state, row, |builder| builder.set_background_color(Color::black()));

        let (size_panel, size_panel_header) = Panel::new("").build(state, entity, |builder| 
            builder
                .class("group")
        );

        LengthBox::new("Size")
            .with_pixels_max(200.0)
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetSize(data.value()));                
            })
            .bind(AppData::style_data.then(StyleData::size), |val| *val)
            .build(state, size_panel_header, |builder| builder);

        LengthBox::new("Width")
            .with_pixels_max(200.0)
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetWidth(data.value()));                
            })
            .bind(AppData::style_data.then(StyleData::width), |val| *val)
            .build(state, size_panel, |builder| builder);

        LengthBox::new("Height")
            .with_pixels_max(200.0)
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetHeight(data.value()));                
            })
            .bind(AppData::style_data.then(StyleData::height), |val| *val)
            .build(state, size_panel, |builder| builder);

        
        let (space_panel, space_panel_header) = Panel::new("")
            .build(state, entity, |builder| 
                builder
                    .class("group")
            );
    
        LengthBox::new("Space")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetSpace(data.value()));
            })
            .bind(AppData::style_data.then(StyleData::space), |val| *val)
            .build(state, space_panel_header, |builder| builder);

        LengthBox::new("Left")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetLeft(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::left), |val| *val)
            .build(state, space_panel, |builder| builder);

        LengthBox::new("Top")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetTop(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::top), |val| *val)
            .build(state, space_panel, |builder| builder);

        LengthBox::new("Right")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetRight(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::right), |val| *val)
            .build(state, space_panel, |builder| builder);
            
        LengthBox::new("Bottom")
                .on_changed(|data, state, lengthbox|{
                    lengthbox.emit(state, AppEvent::SetBottom(data.value()));      
                })
                .bind(AppData::style_data.then(StyleData::bottom), |val| *val)
                .build(state, space_panel, |builder| builder);
    
        
        let border_panel = Panel::new("Border").build(state, entity, |builder| 
            builder
                //.set_background_color(Color::blue())
        );

        LengthBox::new("Border Width")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderWidth(data.value()));                
            })
            .build(state, border_panel, |builder| builder);


        let (border_radius_panel, border_radius_panel_header) = Panel::new("")
            .build(state, border_panel, |builder| 
                builder
                    .class("group")
            );
        
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
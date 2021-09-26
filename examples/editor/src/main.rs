use tuix::*;

mod overlay;
use overlay::*;

mod canvas_options;
use canvas_options::*;

const STYLE: &str = r#"

    window {
        background-color: #383838;
        
    }

    window>.header {
        background-color: #404040;
    }

    .spacer {
        background-color: #202020;
        height: 1px;
        left: 0px;
        right: 0px;
    }

    .divider {
        background-color: #202020;
        width: 1px;
        top: 0px;
        bottom: 0px;
    }

    panel {
        bottom: 2px;
    }

    panel>.header>.label {
        color: #8a8a8a;
    }

    panel>.header>.icon {
        color: #8a8a8a;
    }

    panel>.header {
        height: 30px;
    }

    panel .container2 {
        child-space: 10px;
        child-right: 0px;
        row-between: 10px;
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
        child-left: 5px;
        color: whte;
    }

    dropdown .icon {
        color: #909090;
    }

    dropdown.corner popup {
        width: 1s;
    }

    popup {
        width: 80px;
        background-color: #404040;
        outer-shadow: 0px 2px 5px #40000000;
    }


    list {
        border-width: 1px;
        border-color: #404040;
        width: 1s;
    }

    list>check_button {
        height: 30px;
        child-space: 1s;
        child-left: 5px;
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

    scroll_container>.scrollbar {
        background-color: #464646;
        width: 10px;
        transition: width 0.1 0.0;
    }

    scroll_container:enabled>.scrollbar {
        width: 10px;
        transition: width 0.1 0.0;
    }

    scroll_container:disabled>.scrollbar {
        width: 0px;
        transition: width 0.1 0.0;
    }


    tab_manager {
        width: 1s;
    }
    
    tab_bar {
        background-color: #202020;
        height: 30px;
        layout-type: row;
    }
    
    tab_bar>.tab {
        background-color: #2e2e2e;
        right: 1px;
        bottom: 1px;
        width: 80px;
        child-space: 1s;
    }
    
    tab_bar>.tab:checked {
        bottom: 0px;
        background-color: #383838;
        /* background-color: red; */
    }
    

"#;

fn main() {
    let window_description = WindowDescription::new().with_title("Style Editor").with_inner_size(1600, 800);
    let app = Application::new(window_description, |state, window|{
        state.add_theme(STYLE);

        let app_data = AppData::default().build(state, window);
        
        App::default().build(state, app_data, |builder| builder);

    });

    app.run();
}

// So we can use the same color picker for background and border
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ColorPickFor {
    Background,
    Border,
    Shadow,
    Text,
}

impl Default for ColorPickFor {
    fn default() -> Self {
        ColorPickFor::Background
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum AppEvent {

    SelectWidget(Entity),

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
    // Child Space
    SetChildSpace(Units),
    SetChildLeft(Units),
    SetChildRight(Units),
    SetChildTop(Units),
    SetChildBottom(Units),
    // Border
    SetBorderWidth(Units),
    SetBorderRadius(Units),
    SetBorderRadiusTopLeft(Units),
    SetBorderRadiusTopRight(Units),
    SetBorderRadiusBottomLeft(Units),
    SetBorderRadiusBottomRight(Units),
    SetBorderShape(BorderCornerShape),
    SetBorderTopLeftShape(BorderCornerShape),
    SetBorderTopRightShape(BorderCornerShape),
    SetBorderBottomLeftShape(BorderCornerShape),
    SetBorderBottomRightShape(BorderCornerShape),
    // Color for background, border, text, shadow, etc...
    SetColor(Color),
    // Text
    SetText(String),
    // Outer Shadow
    SetOuterShadowHOffset(Units),
    SetOuterShadowVOffset(Units),
    SetOuterShadowBlur(Units),
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
    // Child Space
    pub child_space: Units,
    pub child_left: Units,
    pub child_top: Units,
    pub child_right: Units,
    pub child_bottom: Units,


    // Background
    pub background_color: Color,
    // Border
    pub border_width: Units,

    pub border_color: Color,

    pub border_shape: BorderCornerShape,
    pub border_top_left_shape: BorderCornerShape,
    pub border_top_right_shape: BorderCornerShape,
    pub border_bottom_left_shape: BorderCornerShape,
    pub border_bottom_right_shape: BorderCornerShape,

    pub border_radius: Units,
    pub border_radius_top_left: Units,
    pub border_radius_top_right: Units,
    pub border_radius_bottom_left: Units,
    pub border_radius_bottom_right: Units,

    pub text: String,
    pub font_color: Color,

    // Outser Shadow
    pub outer_shadow_color: Color,
    pub outer_shadow_h_offset: Units,
    pub outer_shadow_v_offset: Units,
    pub outer_shadow_blur: Units,
}

impl StyleData {
    pub fn new() -> Self {
        Self {
            size: Units::Pixels(100.0),
            width: Units::Pixels(100.0),
            height: Units::Pixels(100.0),
            space: Units::Stretch(1.0),
            left: Units::Stretch(1.0),
            right: Units::Stretch(1.0),
            top: Units::Stretch(1.0),
            bottom: Units::Stretch(1.0),
            ..Default::default()
        }
    }
}

#[derive(Clone)]
pub struct CanvasData {
    width: u32,
    height: u32,
    background_color: Color,
}

impl CanvasData {
    pub fn new() -> Self {
        Self {
            width: 400,
            height: 400,
            background_color: Color::white(),
        }
    }
}

#[derive(Lens)]
pub struct AppData {
    pub style_data: StyleData,
    pub canvas_data: CanvasData,
    pub selected: Entity,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            style_data: StyleData::new(),
            canvas_data: CanvasData::new(),
            selected: Entity::null(),
        }
    }
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {

                AppEvent::SelectWidget(selected) => {
                    self.selected = *selected;
                    entity.emit(state, BindEvent::Update);

                }

                AppEvent::OpenColorPicker(picker_for) => {
                    self.style_data.current_color = *picker_for;
                    entity.emit(state, BindEvent::Update);
                }

                // Size
                AppEvent::SetSize(val) => {
                    //println!("Received {:?} from: {}", val, event.origin);
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

                // Child Space

                AppEvent::SetChildSpace(val) => {
                    self.style_data.child_space = *val;
                    self.style_data.child_left = *val;
                    self.style_data.child_right = *val;
                    self.style_data.child_top = *val;
                    self.style_data.child_bottom = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetChildLeft(val) => {
                    self.style_data.child_left = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetChildRight(val) => {
                    self.style_data.child_right = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetChildTop(val) => {
                    self.style_data.child_top = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetChildBottom(val) => {
                    self.style_data.child_bottom = *val;
                    entity.emit(state, BindEvent::Update);
                }

                // Border
                AppEvent::SetBorderWidth(val) => {
                    self.style_data.border_width = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderShape(shape) => {
                    self.style_data.border_shape = *shape;
                    self.style_data.border_bottom_left_shape = *shape;
                    self.style_data.border_bottom_right_shape = *shape;
                    self.style_data.border_top_left_shape = *shape;
                    self.style_data.border_top_right_shape = *shape;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderTopLeftShape(shape) => {
                    self.style_data.border_top_left_shape = *shape;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderTopRightShape(shape) => {
                    self.style_data.border_top_right_shape = *shape;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderBottomLeftShape(shape) => {
                    self.style_data.border_bottom_left_shape = *shape;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetBorderBottomRightShape(shape) => {
                    self.style_data.border_bottom_right_shape = *shape;
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

                // Color
                AppEvent::SetColor(color) => {
                    match self.style_data.current_color {
                        ColorPickFor::Background => {
                            self.style_data.background_color = *color;
                        }

                        ColorPickFor::Border => {
                            self.style_data.border_color = *color;
                        }

                        ColorPickFor::Shadow => {
                            self.style_data.outer_shadow_color = *color;
                        }

                        ColorPickFor::Text => {
                            self.style_data.font_color = *color;
                        }
                    }
                    
                    entity.emit(state, BindEvent::Update);
                }

                // Text
                AppEvent::SetText(text) => {
                    self.style_data.text = text.clone();
                    entity.emit(state, BindEvent::Update);
                }

                // Outer Shadow
                AppEvent::SetOuterShadowHOffset(val) => {
                    self.style_data.outer_shadow_h_offset = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetOuterShadowVOffset(val) => {
                    self.style_data.outer_shadow_v_offset = *val;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::SetOuterShadowBlur(val) => {
                    self.style_data.outer_shadow_blur = *val;
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
        
        // Placeholder for treeview
        Element::new().build(state, entity, |builder|
            builder
                .set_width(Pixels(300.0))
                .set_background_color(Color::rgb(56, 56, 56))
        );

        Element::new().build(state, entity, |builder|
            builder
                .class("divider")
        );

        let col = Column::new().build(state, entity, |builder| builder);

        // Placeholder for top bar
        let top_bar = Element::new().build(state, col, |builder| 
            builder
                .set_height(Pixels(50.0))
                .set_background_color(Color::rgb(56, 56, 56))
        );

        CanvasOptionsDropdown::new().build(state, top_bar, |builder| 
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
        );

        let canvas = Canvas::default()
            .bind(AppData::style_data, |style_data| style_data.clone())
            .build(state, col, |builder| 
                builder
                    //.set_background_color(Color::red())
            );

        Element::new().build(state, entity, |builder|
            builder
                .class("divider")
        );

        StyleControls::default().build(state, entity, |builder| 
            builder
                .set_background_color(Color::rgb(56,56,56))
                
        );

        self.color_picker = PopupWindow::new("Color Picker").build(state, entity, |builder| 
            builder
                .set_width(Pixels(400.0))
                .set_height(Pixels(300.0))
                .set_space(Stretch(1.0))
                .set_display(Display::None)
                //.set_child_space(Stretch(1.0))
        ).entity();

        ColorPicker::new()
            .on_changing(|data, state, color_picker|{
                color_picker.emit(state, AppEvent::SetColor(data.color()));
            })
            .build(state, self.color_picker, |builder| 
                builder
            );

        // Overlay::new()
        //     .bind(AppData::selected, |selected| *selected)
        //     .build(state, canvas, |builder| builder);
        
        entity.set_layout_type(state, LayoutType::Row)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::OpenColorPicker(_) => {
                    //println!("Open Color Picker: {}", self.color_picker);
                    self.color_picker.emit(state, PopupEvent::Open);
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

        entity.emit(state, AppEvent::SelectWidget(self.element));

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
            // Child Space
            .set_child_left(state, data.child_left)
            .set_child_top(state, data.child_top)
            .set_child_right(state, data.child_right)
            .set_child_bottom(state, data.child_bottom)
            // Border
            .set_border_color(state, data.border_color)
            // Border Shape
            .set_border_top_left_shape(state, data.border_top_left_shape)
            .set_border_top_right_shape(state, data.border_top_right_shape)
            .set_border_bottom_left_shape(state, data.border_bottom_left_shape)
            .set_border_bottom_right_shape(state, data.border_bottom_right_shape)
            // Border Width
            .set_border_width(state, data.border_width)
            // Border Radius
            .set_border_radius_top_left(state, data.border_radius_top_left)
            .set_border_radius_top_right(state, data.border_radius_top_right)
            .set_border_radius_bottom_left(state, data.border_radius_bottom_left)
            .set_border_radius_bottom_right(state, data.border_radius_bottom_right)
            // Text
            .set_text(state, &data.text)
            .set_color(state, data.font_color)
            // Outer Shadow
            .set_outer_shadow_color(state, data.outer_shadow_color)
            .set_outer_shadow_h_offset(state, data.outer_shadow_h_offset)
            .set_outer_shadow_v_offset(state, data.outer_shadow_v_offset)
            .set_outer_shadow_blur(state, data.outer_shadow_blur);


    }
}

#[derive(Default)]
pub struct StyleControls {

}

impl Widget for StyleControls {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {


        // Create a tab manager
        let (tab_bar, tab_viewport) =
        TabView::new().build(state, entity, |builder| builder);

        // Add a tab to the tab bar
        let first_tab = Tab::new("layout").build(state, tab_bar, |builder| {
            builder.set_text("Layout").class("tab")
        });

        first_tab.set_checked(state, true);

        Tab::new("style").build(state, tab_bar, |builder| {
            builder.set_text("Style").class("tab")
        });

        Tab::new("canvas").build(state, tab_bar, |builder| {
            builder.set_text("Canvas").class("tab")
        });

        // Add a tab container to the tab viewport
        let layout_page = TabContainer::new("layout")
            .build(state, tab_viewport, |builder| builder.class("layout"));

        let style_page = TabContainer::new("style")
            .build(state, tab_viewport, |builder| builder.class("style"));
        // TODO - replace tab view with stack
        style_page.set_display(state, Display::None);

        let canvas_page = TabContainer::new("canvas")
            .build(state, tab_viewport, |builder| builder.class("canvas"));
        canvas_page.set_display(state, Display::None);

        let scroll = ScrollContainer::new()
            .build(state, layout_page, |builder| 
                builder 
            );

        scroll
            .set_child_right(state, Pixels(10.0))
            .set_child_space(state, Pixels(10.0))
            .set_row_between(state, Pixels(10.0));

        Label::new("Size:").build(state, scroll, |builder| 
            builder
                .set_height(Pixels(30.0))
        );
        let (size_panel, size_panel_header) = Panel::new("")
        .build(state, scroll, |builder| 
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

        Label::new("Space:").build(state, scroll, |builder| 
            builder
                .set_height(Pixels(30.0))
        );
        let (space_panel, space_panel_header) = Panel::new("")
            .build(state, scroll, |builder| 
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

        Label::new("Space:").build(state, scroll, |builder| 
            builder
                .set_height(Pixels(30.0))
        );
        let (child_space_panel, child_space_panel_header) = Panel::new("")
            .build(state, scroll, |builder| 
                builder
                    .class("group")
            );
    
        LengthBox::new("Child Space")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetChildSpace(data.value()));
            })
            .bind(AppData::style_data.then(StyleData::child_space), |val| *val)
            .build(state, child_space_panel_header, |builder| builder);

        LengthBox::new("Child Left")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetChildLeft(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::child_left), |val| *val)
            .build(state, child_space_panel, |builder| builder);

        LengthBox::new("Child Top")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetChildTop(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::child_top), |val| *val)
            .build(state, child_space_panel, |builder| builder);

        LengthBox::new("Child Right")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetChildRight(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::child_right), |val| *val)
            .build(state, child_space_panel, |builder| builder);
            
        LengthBox::new("Child Bottom")
                .on_changed(|data, state, lengthbox|{
                    lengthbox.emit(state, AppEvent::SetChildBottom(data.value()));      
                })
                .bind(AppData::style_data.then(StyleData::child_bottom), |val| *val)
                .build(state, child_space_panel, |builder| builder);
    
        // STYLE PAGE
        let scroll = ScrollContainer::new()
            .build(state, style_page, |builder| 
                builder 
            );
        
        scroll
            .set_child_right(state, Pixels(10.0))
            .set_child_space(state, Pixels(10.0))
            .set_row_between(state, Pixels(10.0));

        let background_panel = Panel::new("BACKGROUND").build(state, scroll, |builder| 
            builder
        );

        let row = Row::new().build(state, background_panel, |builder| 
            builder
                .set_height(Pixels(30.0))
        );

        Label::new("Color:").build(state, row, |builder| builder);
        ColorButton::new()
            .on_press(|data, state, button|{
                button.emit(state, AppEvent::OpenColorPicker(ColorPickFor::Background));
            })
            .bind(AppData::style_data.then(StyleData::background_color), |col| *col)
            .build(state, row, |builder| builder.set_background_color(Color::black()));
        
        Element::new().build(state, scroll, |builder| builder.class("spacer"));

        let border_panel = Panel::new("BORDER").build(state, scroll, |builder| 
            builder
                //.class("group")
                //.set_background_color(Color::blue())
        );

        let row = Row::new().build(state, border_panel, |builder| 
            builder
                .set_height(Pixels(30.0))
        );

        Label::new("Color:").build(state, row, |builder| builder);
        ColorButton::new()
            .on_press(|data, state, button|{
                button.emit(state, AppEvent::OpenColorPicker(ColorPickFor::Border));
            })
            .bind(AppData::style_data.then(StyleData::border_color), |col| *col)
            .build(state, row, |builder| builder.set_background_color(Color::black()));

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

        let row = Row::new().build(state, border_radius_panel_header, |builder| 
            builder
                .set_col_between(Pixels(10.0))
                .set_height(Auto)
        );

        LengthBox::new("Border Radius")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderRadius(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::border_radius), |val| *val)
            .build(state, row, |builder| builder);

        border_shape_dropdown(state, row, Corner::All, AppData::style_data.then(StyleData::border_shape));

        let row = Row::new().build(state, border_radius_panel, |builder| 
            builder
                .set_col_between(Pixels(10.0))
                .set_height(Auto)
        );

        LengthBox::new("Top Left")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderRadiusTopLeft(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::border_radius_top_left), |val| *val)
            .build(state, row, |builder| builder);
        
        border_shape_dropdown(state, row, Corner::TopLeft, AppData::style_data.then(StyleData::border_top_left_shape));

        let row = Row::new().build(state, border_radius_panel, |builder| 
            builder
                .set_col_between(Pixels(10.0))
                .set_height(Auto)
        );

        LengthBox::new("Top Right")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderRadiusTopRight(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::border_radius_top_right), |val| *val)
            .build(state, row, |builder| builder);
        
        border_shape_dropdown(state, row, Corner::TopRight, AppData::style_data.then(StyleData::border_top_right_shape));

        let row = Row::new().build(state, border_radius_panel, |builder| 
            builder
                .set_col_between(Pixels(10.0))
                .set_height(Auto)
        );

        LengthBox::new("Bottom Left")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetBorderRadiusBottomLeft(data.value()));      
            })
            .bind(AppData::style_data.then(StyleData::border_radius_bottom_left), |val| *val)
            .build(state, row, |builder| builder);
    
        border_shape_dropdown(state, row, Corner::BottomLeft, AppData::style_data.then(StyleData::border_bottom_left_shape));

        let row = Row::new().build(state, border_radius_panel, |builder| 
            builder
                .set_col_between(Pixels(10.0))
                .set_height(Auto)
        );

        LengthBox::new("Bottom Right")
                .on_changed(|data, state, lengthbox|{
                    lengthbox.emit(state, AppEvent::SetBorderRadiusBottomRight(data.value()));      
                })
                .bind(AppData::style_data.then(StyleData::border_radius_bottom_right), |val| *val)
                .build(state, row, |builder| builder);

        border_shape_dropdown(state, row, Corner::BottomRight, AppData::style_data.then(StyleData::border_bottom_right_shape));

        Element::new().build(state, scroll, |builder| builder.class("spacer"));
    
        let shadow_panel = Panel::new("SHADOW").build(state, scroll, |builder| 
            builder
        );

        let row = Row::new().build(state, shadow_panel, |builder| 
            builder
                .set_height(Pixels(30.0))
        );

        Label::new("Color:").build(state, row, |builder| builder);
        ColorButton::new()
            .on_press(|data, state, button|{
                button.emit(state, AppEvent::OpenColorPicker(ColorPickFor::Shadow));
            })
            .bind(AppData::style_data.then(StyleData::outer_shadow_color), |col| *col)
            .build(state, row, |builder| builder.set_background_color(Color::black()));
        

        LengthBox::new("H Offset")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetOuterShadowHOffset(data.value()));                
            })
            .build(state, shadow_panel, |builder| builder);
        
        LengthBox::new("V Offset")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetOuterShadowVOffset(data.value()));                
            })
            .build(state, shadow_panel, |builder| builder);
        
        LengthBox::new("Blur Radius")
            .on_changed(|data, state, lengthbox|{
                lengthbox.emit(state, AppEvent::SetOuterShadowBlur(data.value()));                
            })
            .build(state, shadow_panel, |builder| builder);
        
        Element::new().build(state, scroll, |builder| builder.class("spacer"));

        let text_panel = Panel::new("TEXT").build(state, scroll, |builder| 
            builder
                //.class("group")
                //.set_background_color(Color::blue())
        );

        let row = Row::new().build(state, text_panel, |builder| 
            builder
                .set_height(Pixels(30.0))
        );

        Label::new("Color:").build(state, row, |builder| builder);
        ColorButton::new()
            .on_press(|data, state, button|{
                button.emit(state, AppEvent::OpenColorPicker(ColorPickFor::Text));
            })
            .bind(AppData::style_data.then(StyleData::font_color), |col| *col)
            .build(state, row, |builder| builder.set_background_color(Color::black()));
        

        Textbox::new("")
        .on_submit(|data, state, textbox|{
            textbox.emit(state, AppEvent::SetText(data.text.clone()));
        })
        .build(state, text_panel, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_child_left(Pixels(5.0))
                .set_child_right(Stretch(1.0))
        );

        Element::new().build(state, scroll, |builder| builder.class("spacer"));


        entity.set_width(state, Pixels(500.0))
    }
}

enum Corner {
    All,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn border_shape_dropdown<L: Lens<Target = BorderCornerShape>>(state: &mut State, parent: Entity, corner: Corner, lens: L)
{

    let (round, bevel) = match corner {

        Corner::All => {
            (AppEvent::SetBorderShape(BorderCornerShape::Round), AppEvent::SetBorderShape(BorderCornerShape::Bevel))
        }

        Corner::TopLeft => {
            (AppEvent::SetBorderTopLeftShape(BorderCornerShape::Round), AppEvent::SetBorderTopLeftShape(BorderCornerShape::Bevel))
        }

        Corner::TopRight => {
            (AppEvent::SetBorderTopRightShape(BorderCornerShape::Round), AppEvent::SetBorderTopRightShape(BorderCornerShape::Bevel))
        }

        Corner::BottomLeft => {
            (AppEvent::SetBorderBottomLeftShape(BorderCornerShape::Round), AppEvent::SetBorderBottomLeftShape(BorderCornerShape::Bevel))
        }

        Corner::BottomRight => {
            (AppEvent::SetBorderBottomRightShape(BorderCornerShape::Round), AppEvent::SetBorderBottomRightShape(BorderCornerShape::Bevel))
        }
    };

    let dropdown = Dropdown::new("TEST")
        .bind(lens, |shape| *shape)
        .on_update(|data, state, dropdown|{
            match data.value {
                BorderCornerShape::Round => {
                    dropdown.emit(state, DropdownEvent::SetText("Round".to_string()));
                }

                BorderCornerShape::Bevel => {
                    dropdown.emit(state, DropdownEvent::SetText("Bevel".to_string()));
                }
            }
        })
        .build(state, parent, |builder| {
            builder
                .set_width(Pixels(80.0))
                .set_height(Pixels(30.0))
                .class("corner")
        });

    // Spacer
    CheckButton::with_label("Round")
        //.set_checked(true)
        .on_checked(move |_, state, button|{
            button.emit(state, round.clone());
            button.emit(state, PopupEvent::Close);
            button.emit(state, DropdownEvent::SetText("Round".to_string()));
        })
        .build(state, dropdown, |builder| 
            builder
        );

    CheckButton::with_label("Bevel")
        .on_checked(move |_, state, button|{
            button.emit(state, bevel.clone());
            button.emit(state, PopupEvent::Close);
            button.emit(state, DropdownEvent::SetText("Bevel".to_string()));
        })
        .build(state, dropdown, |builder| 
            builder
        );
}


pub struct ColorButton {
    button: Button,
}

impl ColorButton {
    pub fn new() -> Self {
        Self {
            button: Button::new(),
        }
    }

    pub fn on_press<F>(mut self, callback: F) -> Self
    where F: 'static + Fn(&mut Button, &mut State, Entity)
    {
        self.button = self.button.on_press(callback);

        self
    }
}

impl Widget for ColorButton {
    type Ret = Entity;
    type Data = Color;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.button.on_build(state, entity)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.button.on_event(state, entity, event)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        if *data != entity.get_background_color(state) {
            entity.set_background_color(state, *data);
        }
    }
}
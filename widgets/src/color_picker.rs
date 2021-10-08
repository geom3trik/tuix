
use femtovg::{ImageFlags, ImageId, PixelFormat, RenderTarget};
use tuix_core::Store;

use crate::{Column, Label, Row, Textbox, TextboxEvent, common::*};

const ICON_LEFT_DIR: &str = "\u{25c2}";
const ICON_RIGHT_DIR: &str = "\u{25b8}";

#[derive(Default, Lens)]
pub struct ColorPickerData {
    hue: f32,
    sat: f32,
    val: f32,
    alpha: f32,
}

impl Model for ColorPickerData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(color_picker_event) = event.message.downcast() {
            match color_picker_event {
                ColorPickerEvent::SetHue(hue) => {
                    self.hue = *hue;
                    entity.emit(state, BindEvent::Update);
                }

                ColorPickerEvent::SetSat(sat) => {
                    self.sat = *sat;
                    entity.emit(state, BindEvent::Update);
                }

                ColorPickerEvent::SetVal(val) => {
                    self.val = *val;
                    entity.emit(state, BindEvent::Update);
                }

                ColorPickerEvent::SetAlpha(alpha) => {
                    self.alpha = *alpha;
                    entity.emit(state, BindEvent::Update);
                }

                _=> {}
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ColorPickerEvent {
    HueChanged(f32),
    AlphaChanged(f32),
    SatValChanged(f32, f32),
    SetColor(Color),
    SetHue(f32),
    SetSat(f32),
    SetVal(f32),
    SetAlpha(f32),
}

pub struct ColorPicker {
    //thumb: Entity,

    data: Store<ColorPickerData>,

    on_changing: Option<Box<dyn Fn(&mut ColorPicker, &mut State, Entity)>>,
}

impl ColorPicker {
    pub fn new() -> Self {
        Self {
            //thumb: Entity::null(),

            data: Store::new(ColorPickerData::default()),

            on_changing: None,
        }
    }

    pub fn on_changing<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut ColorPicker, &mut State, Entity),
    {
        self.on_changing = Some(Box::new(callback));
        self
    }

    pub fn color(&self) -> Color {
        if let Some(data) = self.data.get() {
            let (h, s, l) = hsv_to_hsl(data.hue as f64, data.sat as f64, data.val as f64);
            Color::hsla(h as f32, s as f32, l as f32, data.alpha)
        } else {
            Color::rgba(0, 0, 0, 0)
        }
    }


}

impl Widget for ColorPicker {
    type Ret = Entity;
    type Data = Color;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.data.on_build(state, entity);
        
        ColorGradient::new()
            .on_changing(|data, state, grad|{
                grad.emit(state, ColorPickerEvent::SetSat(data.saturation));
                grad.emit(state, ColorPickerEvent::SetVal(data.value));
            })
            .bind_ref(ColorPickerData::root)
            .build(state, entity, |builder|
                builder
                    .set_width(Pixels(250.0))
                    .set_height(Pixels(250.0))
            );

        HueSlider::new()
            .on_changing(move |data, state, slider|{
                //grad.emit(state, ColorPickerEvent::SetSat(data.saturation));
                slider.emit(state, ColorPickerEvent::SetHue(data.value));
            })
            .bind_ref(ColorPickerData::hue)
            .build(state, entity, |builder|
                builder
                    .set_width(Pixels(45.0))
                    .set_height(Pixels(260.0))
                    .set_top(Pixels(-5.0))
            );

        AlphaSlider::new()
            .on_changing(move |data, state, slider|{
                slider.emit(state, ColorPickerEvent::SetAlpha(data.value));
            })
            .bind_ref(ColorPickerData::root)
            .build(state, entity, |builder|
                builder
                    .set_width(Pixels(45.0))
                    .set_height(Pixels(260.0))
                    .set_top(Pixels(-5.0))
            );

        let column = Column::new().build(state, entity, |builder|
            builder
        );

        let row = Row::new().build(state, column, |builder| builder);
        Label::new("H:").build(state, row, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_width(Pixels(30.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
            );
        Textbox::new("")
        .on_submit(|data, state, textbox|{
            if let Ok(value) = data.text.parse::<u8>() {
                textbox.emit(state, ColorPickerEvent::SetHue(value as f32 / 255.0));
            }
        })
        .bind(ColorPickerData::hue, |hue| ((hue * 255.0) as u8).to_string())
        .build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_width(Pixels(70.0))
                .set_border_color(Color::black())
                .set_border_width(Pixels(1.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(5.0))
        );

        let row = Row::new().build(state, column, |builder| builder);
        Label::new("S:").build(state, row, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_width(Pixels(30.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
            );
        Textbox::new("")
        .on_submit(|data, state, textbox|{
            if let Ok(value) = data.text.parse::<u8>() {
                textbox.emit(state, ColorPickerEvent::SetSat(value as f32 / 255.0));
            }
        })
        .bind(ColorPickerData::sat, |sat| ((sat * 255.0) as u8).to_string())
        .build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_width(Pixels(70.0))
                .set_border_color(Color::black())
                .set_border_width(Pixels(1.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(5.0))
        );

        let row = Row::new().build(state, column, |builder| builder);
        Label::new("V:").build(state, row, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_width(Pixels(30.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
            );
        Textbox::new("")
        .on_submit(|data, state, textbox|{
            if let Ok(value) = data.text.parse::<u8>() {
                textbox.emit(state, ColorPickerEvent::SetVal(value as f32 / 255.0));
            }
        })
        .bind(ColorPickerData::val, |val| ((val * 255.0) as u8).to_string())
        .build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_width(Pixels(70.0))
                .set_border_color(Color::black())
                .set_border_width(Pixels(1.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(5.0))
        );

        let row = Row::new().build(state, column, |builder| builder);
        Label::new("A:").build(state, row, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_width(Pixels(30.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
            );
        Textbox::new("")
        .on_submit(|data, state, textbox|{
            if let Ok(value) = data.text.parse::<u8>() {
                textbox.emit(state, ColorPickerEvent::SetAlpha(value as f32 / 255.0));
            } else {
                println!("Buffer: {}", data.buffer);
                textbox.emit(state, TextboxEvent::SetValue(data.buffer.clone()));
            }
        })
        .bind(ColorPickerData::alpha, |alpha| ((alpha * 255.0) as u8).to_string())
        .build(state, row, |builder|
            builder
                .set_height(Pixels(30.0))
                .set_width(Pixels(70.0))
                .set_border_color(Color::black())
                .set_border_width(Pixels(1.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(5.0))
        );

        entity
            .set_layout_type(state, LayoutType::Row)
            //.set_background_color(state, Color::rgb(56, 56, 56))
            //.set_child_space(state, Stretch(1.0))
            .set_col_between(state, Pixels(5.0))
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.data.on_event(state, entity, event);

        if let Some(color_picker_event) = event.message.downcast() {
            match color_picker_event {
                ColorPickerEvent::SetSat(_) | 
                ColorPickerEvent::SetVal(_) | 
                ColorPickerEvent::SetHue(_) | 
                ColorPickerEvent::SetAlpha(_) => {
                    if let Some(callback) = self.on_changing.take() {
                        (callback)(self, state, entity);

                        self.on_changing = Some(callback);
                    }
                }

                _=> {}
            }
        }
    }
    // fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        
    // }
}

pub struct HueSlider {

    pub value: f32,

    prev: f32,

    left_arrow: Entity,
    right_arrow: Entity,
    track: Entity,

    on_changing: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl HueSlider {
    pub fn new() -> Self {
        Self {

            value: 0.0,
            prev: 0.0,

            left_arrow: Entity::null(),
            right_arrow: Entity::null(),
            track: Entity::null(),

            on_changing: None,
        }
    }

    pub fn on_changing<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_changing = Some(Box::new(callback));
        self
    }

    fn update_value(&mut self, state: &mut State, entity: Entity, mut dx: f32) {
        let height = state.data.get_height(entity);
        let thumb_size = state.data.get_height(self.left_arrow);

        dx = height - dx;

        if dx <= thumb_size / 2.0 {
            dx = thumb_size / 2.0;
        }
        if dx >= height - thumb_size / 2.0 {
            dx = height - thumb_size / 2.0;
        }

        let nx = (dx - thumb_size / 2.0) / (height - thumb_size);

        self.left_arrow.set_bottom(state, Units::Percentage(100.0 * (dx - thumb_size / 2.0) / height));
        self.right_arrow.set_bottom(state, Units::Percentage(100.0 * (dx - thumb_size / 2.0) / height));

        self.value = nx.clamp(0.0, 1.0);
    }

    // fn update_visuals(&mut self, state: &mut State, entity: Entity) {
    //     let normalised_value = self.value;

    //     let height = state.data.get_height(entity);
    //     let thumb_size = state.data.get_height(self.left_arrow);

    //     let dx = normalised_value * (height - thumb_size) + thumb_size / 2.0;

    //     self.update_value(state, entity, dx);
    // }
}

impl Widget for HueSlider {
    type Ret = Entity;
    type Data = f32;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_layout_type(state, LayoutType::Row);

        self.left_arrow = Element::new().build(state, entity, |builder| 
            builder
                .set_width(Pixels(10.0))
                .set_height(Pixels(10.0))
                .set_color(Color::rgb(180, 180, 180))
                .set_font("icons")
                .set_text(ICON_RIGHT_DIR)
                .set_child_space(Stretch(1.0))
                .set_child_right(Pixels(0.0))
                .set_bottom(Percentage(0.0))
                .set_top(Stretch(1.0))
                .set_font_size(20.0)
        );

        self.track = Element::new().build(state, entity, |builder|
            builder
                .set_background_gradient(LinearGradient::new(GradientDirection::TopToBottom)
                    .add_stop(GradientStop::new(Units::Percentage(0.0), Color::rgb(255, 0, 0)))
                    .add_stop(GradientStop::new(Units::Percentage(16.7), Color::rgb(255, 0, 255)))
                    .add_stop(GradientStop::new(Units::Percentage(33.3), Color::rgb(0, 0, 255)))
                    .add_stop(GradientStop::new(Units::Percentage(50.0), Color::rgb(0, 255, 255)))
                    .add_stop(GradientStop::new(Units::Percentage(66.7), Color::rgb(0, 255, 0)))
                    .add_stop(GradientStop::new(Units::Percentage(83.3), Color::rgb(255, 255, 0)))
                    .add_stop(GradientStop::new(Units::Percentage(100.0), Color::rgb(255, 0, 0)))
                )
                .set_top(Pixels(5.0))
                .set_bottom(Pixels(5.0))
        );

        self.right_arrow = Element::new().build(state, entity, |builder| 
            builder
                .set_width(Pixels(10.0))
                .set_height(Pixels(10.0))
                .set_color(Color::rgb(180, 180, 180))
                .set_font("icons")
                .set_text(ICON_LEFT_DIR)
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
                .set_bottom(Percentage(0.0))
                .set_top(Stretch(1.0))
                .set_font_size(20.0)
        );

        entity
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        if *data != self.value {
            let height = state.data.get_height(entity);
            let thumb_size = state.data.get_height(self.left_arrow);
    
            
            let dx = *data * (height - thumb_size) + (thumb_size / 2.0);
    
            self.left_arrow.set_bottom(state, Units::Percentage(100.0 * (dx - thumb_size / 2.0) / height));
            self.right_arrow.set_bottom(state, Units::Percentage(100.0 * (dx - thumb_size / 2.0) / height));
            self.value = *data;
        }    

    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                
                //TODO
                // WindowEvent::GeometryChanged(_) if event.target == entity => {
                //     self.update_visuals(state, entity);
                // }

                WindowEvent::MouseDown(button) if event.target == self.track || event.target == self.left_arrow || event.target == self.right_arrow => {
                    if *button == MouseButton::Left {
                        state.capture(entity);

                        self.prev = self.value;

                        entity.set_active(state, true);

                        let dx = state.mouse.left.pos_down.1 - state.data.get_posy(entity);

                        self.update_value(state, entity, dx);

                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, state, entity);
                            self.on_changing = Some(callback);
                        }
                    }
                }

                WindowEvent::MouseUp(button) if event.target == entity => {
                    if *button == MouseButton::Left {
                        state.release(entity);

                        entity.set_active(state, false);

                        if self.prev != self.value {
                            //self.send_value_event(state, entity, &self.on_change);
                            // if let Some(callback) = self.on_change.take() {
                            //     (callback)(self, state, entity);
                            //     self.on_change = Some(callback);
                            // }

                        }
                    }
                }

                WindowEvent::MouseMove(_, y) if event.target == entity => {
                    if entity.is_active(state) {
                        let dx = *y - state.data.get_posy(entity);

                        self.update_value(state, entity, dx);
                        
                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, state, entity);
                            self.on_changing = Some(callback);
                        }
                    }
                }

                // TODO - Add keyboard control
                _ => {}
            }
        }

    }
}

pub struct ColorGradient {
    thumb: Entity,
    image: Option<ImageId>,
    hue: f32,
    saturation: f32,
    value: f32,

    on_changing: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}   

impl ColorGradient {
    pub fn new() -> Self {
        Self {
            thumb: Entity::null(),
            image: None,
            hue: 0.0,
            saturation: 0.0,
            value: 0.0,

            on_changing: None,
        }
    }

    pub fn color(&self) -> Color {
        let (h, s, l) = hsv_to_hsl(self.hue as f64, self.saturation as f64, self.value as f64);
        Color::hsl(h as f32, s as f32, l as f32)
    }

    pub fn on_changing<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_changing = Some(Box::new(callback));
        self
    }
}

impl Widget for ColorGradient {
    type Ret = Entity;
    type Data = ColorPickerData;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.thumb = Element::new().build(state, entity, |builder|
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_width(Pixels(10.0))
                .set_height(Pixels(10.0))
                .set_border_radius(Percentage(50.0))
                .set_border_color(Color::white())
                .set_border_width(Pixels(2.0))
                .set_clip_widget(entity)
        );

        entity
    }
    
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {


        let visibility = state.data.get_visibility(entity);

        if visibility == Visibility::Invisible {
            return;
        }
        
        if self.image.is_none() {
            let image_id = canvas
            .create_image_empty(
                63,
                63,
                PixelFormat::Rgb8,
                ImageFlags::empty(),
            )
            .unwrap();

            self.image = Some(image_id);
        }

        if let Some(image_id) = self.image {

            canvas.save();
            canvas.reset();
            canvas.reset_scissor();
            canvas.reset_transform();
            if let Ok(size) = canvas.image_size(image_id) {
                canvas.set_render_target(RenderTarget::Image(image_id));
                
    
                canvas.clear_rect(0, 0, size.0 as u32, size.1 as u32, femtovg::Color::rgb(0, 0, 0));
                for x in 0..64 {
                    for y in 0..64 {
    
                        let x_ratio = x as f64 / 63 as f64;
                        let y_ratio = y as f64 / 63 as f64;
    
                        let (_, s, v) = hsv_to_hsl(0.0, x_ratio, y_ratio);
    
                        canvas.clear_rect(
                            x as u32,
                            y as u32,
                            1,
                            1,
                            femtovg::Color::hsl(self.hue, s as f32, v as f32),
                        );
                    }
                }
            }
            canvas.restore();
            canvas.set_render_target(RenderTarget::Screen);

            //println!("Draw Picker: {} {:?}", entity, image_id);
            canvas.save();
            canvas.reset();
            canvas.reset_scissor();
            canvas.reset_transform();
            

            let bounds = state.data.get_bounds(entity);

            //println!("Bounds: {:?}", bounds);

            let mut path = femtovg::Path::new();
            path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
            canvas.fill_path(
                &mut path,
                femtovg::Paint::image(image_id, bounds.x, bounds.y, bounds.w, bounds.h, 0f32, 1f32),
            );
            canvas.restore();
        }
    }
    
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(color_picker_event) = event.message.downcast() {
            match color_picker_event {
                ColorPickerEvent::HueChanged(val) => {
                    self.hue = *val;

                    if let Some(callback) = self.on_changing.take() {
                        (callback)(self, state, entity);

                        self.on_changing = Some(callback);
                    }
                }
                _=> {}
            }
        }
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == entity {
                        state.capture(entity);
                        entity.set_active(state, true);
                        let mut dx = (state.mouse.left.pos_down.0 - state.data.get_posx(entity)) / state.data.get_width(entity);
                        let mut dy = (state.mouse.left.pos_down.1 - state.data.get_posy(entity)) / state.data.get_height(entity);
                        
                        dx = dx.clamp(0.0, 1.0);
                        dy = dy.clamp(0.0, 1.0);

                        self.saturation = dx;
                        self.value = 1.0 - dy;
                    

                        let width = state.data.get_width(entity);
                        let height = state.data.get_height(entity);

                        self.thumb.set_left(state, Pixels(dx * width - 5.0)).set_top(state, Pixels(dy * height - 5.0));
                    
                        if dx < 0.2 && dy < 0.2 {
                            // TODO - change to outer shadow
                            self.thumb.set_border_color(state, Color::black());
                        } else {
                            self.thumb.set_border_color(state, Color::white());
                        }

                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, state, entity);
    
                            self.on_changing = Some(callback);
                        }
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    entity.set_active(state, false);
                    if event.target == entity {
                        state.release(entity);
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    if entity.is_active(state) {
                        let mut dx = (*x- state.data.get_posx(entity)) / state.data.get_width(entity);
                        let mut dy = (*y - state.data.get_posy(entity)) / state.data.get_height(entity);

                        dx = dx.clamp(0.0, 1.0);
                        dy = dy.clamp(0.0, 1.0);

                        self.saturation = dx;
                        self.value = 1.0 - dy;

                        let width = state.data.get_width(entity);
                        let height = state.data.get_height(entity);

                        self.thumb.set_left(state, Pixels(dx * width - 5.0)).set_top(state, Pixels(dy * height - 5.0));
                    
                        if dx < 0.2 && dy < 0.2 {
                            // TODO - change to outer shadow
                            self.thumb.set_border_color(state, Color::black());
                        } else {
                            self.thumb.set_border_color(state, Color::white());
                        }

                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, state, entity);
    
                            self.on_changing = Some(callback);
                        }
                    }
                }

                _=> {}
            }
        }
    }
    
    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        self.hue = data.hue;

        // self.saturation = dx;
        // self.value = 1.0 - dy;

        let dx = data.sat;
        let dy = 1.0 - data.val;

        let width = state.data.get_width(entity);
        let height = state.data.get_height(entity);

        self.thumb.set_left(state, Pixels(dx * width - 5.0)).set_top(state, Pixels(dy * height - 5.0));
    
    }

}

pub struct AlphaSlider {
    pub value: f32,

    prev: f32,

    hue: f32,
    sat: f32,
    val: f32,
    
    left_arrow: Entity,
    right_arrow: Entity,
    track: Entity,

    image: Option<ImageId>,

    on_changing: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,

}

impl AlphaSlider {
    pub fn new() -> Self {
        Self {
            value: 0.0,

            prev: 0.0,

            hue: 0.0,
            sat: 0.0,
            val: 0.0,

            left_arrow: Entity::null(),
            right_arrow: Entity::null(),
            track: Entity::null(),

            image: None,

            on_changing: None,
        }
    }

    pub fn on_changing<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_changing = Some(Box::new(callback));
        self
    }

    fn update_value(&mut self, state: &mut State, entity: Entity, mut dx: f32) {
        let height = state.data.get_height(entity);
        let thumb_size = state.data.get_height(self.left_arrow);

        dx = height - dx;

        if dx <= thumb_size / 2.0 {
            dx = thumb_size / 2.0;
        }
        if dx >= height - thumb_size / 2.0 {
            dx = height - thumb_size / 2.0;
        }

        let nx = (dx - thumb_size / 2.0) / (height - thumb_size);

        self.left_arrow.set_bottom(state, Units::Percentage(100.0 * (dx - thumb_size / 2.0) / height));
        self.right_arrow.set_bottom(state, Units::Percentage(100.0 * (dx - thumb_size / 2.0) / height));

        self.value = nx.clamp(0.0, 1.0);
    }
}

impl Widget for AlphaSlider {
    type Ret = Entity;
    type Data = ColorPickerData;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_layout_type(state, LayoutType::Row);

        self.left_arrow = Element::new().build(state, entity, |builder| 
            builder
                .set_width(Pixels(10.0))
                .set_height(Pixels(10.0))
                .set_color(Color::rgb(180, 180, 180))
                .set_font("icons")
                .set_text(ICON_RIGHT_DIR)
                .set_child_space(Stretch(1.0))
                .set_child_right(Pixels(0.0))
                .set_bottom(Percentage(0.0))
                .set_top(Stretch(1.0))
                .set_font_size(20.0)
        );

        // self.track = Element::new().build(state, entity, |builder|
        //     builder
        //         .set_background_gradient(LinearGradient::new(GradientDirection::TopToBottom)
        //             .add_stop(GradientStop::new(Units::Percentage(0.0), Color::rgb(255, 0, 0)))
        //             .add_stop(GradientStop::new(Units::Percentage(16.7), Color::rgb(255, 0, 255)))
        //             .add_stop(GradientStop::new(Units::Percentage(33.3), Color::rgb(0, 0, 255)))
        //             .add_stop(GradientStop::new(Units::Percentage(50.0), Color::rgb(0, 255, 255)))
        //             .add_stop(GradientStop::new(Units::Percentage(66.7), Color::rgb(0, 255, 0)))
        //             .add_stop(GradientStop::new(Units::Percentage(83.3), Color::rgb(255, 255, 0)))
        //             .add_stop(GradientStop::new(Units::Percentage(100.0), Color::rgb(255, 0, 0)))
        //         )
        //         .set_top(Pixels(5.0))
        //         .set_bottom(Pixels(5.0))
        // );

        self.track = Element::new().build(state, entity, |builder|
            builder
                .set_top(Pixels(5.0))
                .set_bottom(Pixels(5.0)) 
                .set_background_gradient(LinearGradient::new(GradientDirection::TopToBottom)
                    .add_stop(GradientStop::new(Units::Percentage(0.0), Color::red()))
                    .add_stop(GradientStop::new(Units::Percentage(20.0), Color::rgba(255, 0, 0, 180)))
                    .add_stop(GradientStop::new(Units::Percentage(100.0), Color::rgba(0, 0, 0, 0))))  
        );

        self.right_arrow = Element::new().build(state, entity, |builder| 
            builder
                .set_width(Pixels(10.0))
                .set_height(Pixels(10.0))
                .set_color(Color::rgb(180, 180, 180))
                .set_font("icons")
                .set_text(ICON_LEFT_DIR)
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
                .set_bottom(Percentage(0.0))
                .set_top(Stretch(1.0))
                .set_font_size(20.0)
        );

        entity
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        let (h, s, l) = hsv_to_hsl(data.hue as f64, data.sat as f64, data.val as f64);
        
        self.track.set_background_gradient(state, LinearGradient::new(GradientDirection::TopToBottom)
        .add_stop(GradientStop::new(Units::Percentage(0.0), Color::hsl(h as f32, s as f32, l as f32)))
        .add_stop(GradientStop::new(Units::Percentage(20.0), Color::hsla(h as f32, s as f32, l as f32, 0.75)))
        .add_stop(GradientStop::new(Units::Percentage(100.0), Color::rgba(0, 0, 0, 0))));
    
        if data.alpha != self.value {

            let height = state.data.get_height(entity);
            let thumb_size = state.data.get_height(self.left_arrow);
    
            
            let dx = data.alpha * (height - thumb_size) + (thumb_size / 2.0);
    
            self.left_arrow.set_bottom(state, Units::Percentage(100.0 * (dx - thumb_size / 2.0) / height));
            self.right_arrow.set_bottom(state, Units::Percentage(100.0 * (dx - thumb_size / 2.0) / height));
            self.value = data.alpha;
        }   
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                
                //TODO
                // WindowEvent::GeometryChanged(_) if event.target == entity => {
                //     self.update_visuals(state, entity);
                // }

                WindowEvent::MouseDown(button) if event.target == self.track || event.target == self.left_arrow || event.target == self.right_arrow => {
                    if *button == MouseButton::Left {
                        state.capture(entity);

                        self.prev = self.value;

                        entity.set_active(state, true);

                        let dx = state.mouse.left.pos_down.1 - state.data.get_posy(entity);

                        self.update_value(state, entity, dx);

                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, state, entity);
                            self.on_changing = Some(callback);
                        }
                    }
                }

                WindowEvent::MouseUp(button) if event.target == entity => {
                    if *button == MouseButton::Left {
                        state.release(entity);

                        entity.set_active(state, false);

                        if self.prev != self.value {
                            //self.send_value_event(state, entity, &self.on_change);
                            // if let Some(callback) = self.on_change.take() {
                            //     (callback)(self, state, entity);
                            //     self.on_change = Some(callback);
                            // }

                        }
                    }
                }

                WindowEvent::MouseMove(_, y) if event.target == entity => {
                    if entity.is_active(state) {
                        let dx = *y - state.data.get_posy(entity);

                        self.update_value(state, entity, dx);
                        
                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, state, entity);
                            self.on_changing = Some(callback);
                        }
                    }
                }

                // TODO - Add keyboard control
                _ => {}
            }
        }
    }
    
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        
        let grid_size: usize = 16;
        
        if self.image.is_none() {
            let image_id = canvas
            .create_image_empty(
                3 * grid_size,
                16 * grid_size,
                PixelFormat::Rgb8,
                ImageFlags::empty(),
            )
            .unwrap();

            self.image = Some(image_id);
        }

        if let Some(image_id) = self.image {

            canvas.save();
            canvas.reset();
            canvas.reset_scissor();
            canvas.reset_transform();
            if let Ok(size) = canvas.image_size(image_id) {
                canvas.set_render_target(RenderTarget::Image(image_id));
                
    
                canvas.clear_rect(0, 0, size.0 as u32, size.1 as u32, femtovg::Color::rgb(0, 0, 0));
                
                for x in 0..(size.0 / grid_size) {
                    for y in 0..(size.1 / grid_size) {
    
                        canvas.clear_rect(
                            (x * grid_size) as u32,
                            (y * grid_size) as u32,
                            (grid_size) as u32,
                            (grid_size) as u32,
                            
                            match (x % 2, y % 2) {
                                (0, 0) => femtovg::Color::rgb(125, 125, 125),
                                (1, 0) => femtovg::Color::rgb(155, 155, 155),
                                (0, 1) => femtovg::Color::rgb(155, 155, 155),
                                (1, 1) => femtovg::Color::rgb(125, 125, 125),
                                _ => femtovg::Color::rgb(255, 0, 255),
                            },
                        );
                    }
                }
            }
            canvas.restore();
            canvas.set_render_target(RenderTarget::Screen);

            //println!("Draw Picker: {} {:?}", entity, image_id);
            canvas.save();
            canvas.reset();
            canvas.reset_scissor();
            canvas.reset_transform();
            

            let bounds = state.data.get_bounds(self.track);

            //println!("Bounds: {:?}", bounds);

            let mut path = femtovg::Path::new();
            path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
            canvas.fill_path(
                &mut path,
                femtovg::Paint::image(image_id, bounds.x, bounds.y, bounds.w, bounds.h, 0f32, 1f32),
            );

            canvas.restore();
        }
    }
    
}

// fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
//     let mut t = t;
//     if t < 0. {
//         t += 1.
//     }
//     if t > 1. {
//         t -= 1.
//     };
//     if t < 1. / 6. {
//         return p + (q - p) * 6. * t;
//     }
//     if t < 1. / 2. {
//         return q;
//     }
//     if t < 2. / 3. {
//         return p + (q - p) * (2. / 3. - t) * 6.;
//     }
//     return p;
// }

// fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f32, f32, f32) {
//     let r;
//     let g;
//     let b;

//     if s == 0.0 {
//         r = l;
//         g = l;
//         b = l; // achromatic
//     } else {
//         let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };

//         let p = 2. * l - q;
//         r = hue_to_rgb(p, q, h + 1. / 3.);
//         g = hue_to_rgb(p, q, h);
//         b = hue_to_rgb(p, q, h - 1. / 3.);
//     }

//     return (
//         r as f32,
//         g as f32,
//         b as f32,
//         //(r * 255.).round() as u8,
//         //(g * 255.).round() as u8,
//         //(b * 255.).round() as u8,
//     );
// }

fn hsv_to_hsl(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    //   *hh = h;
    let mut ll = (2.0 - s) * v;
    let mut ss = s * v;
    ss /= if ll <= 1.0 {ll} else {2.0 - ll};
    ll /= 2.0;

    (h, ss, ll)
}
//    10
//    11 void hsl_to_hsv(double hh, double ss, double ll,
//    12 double* h, double* s, double *v)
//    13 {
//    14     *h = hh;
//    15     ll *= 2;
//    16     ss *= (ll <= 1) ? ll : 2 - ll;
//    17     *v = (ll + ss) / 2;
//    18     *s = (2 * ss) / (ll + ss);
//    19 }
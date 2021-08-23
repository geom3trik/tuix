use crate::common::*;

use femtovg::{renderer::OpenGl, Align, Baseline, Canvas, Color, Paint, Path, Solidity};

use crate::Key;

#[derive(Debug, Clone, PartialEq)]
pub enum TextboxEvent {
    SetValue(String),
    ValueChanged(String),
    ResetValue,
    Clear,
}

//impl Message for TextboxEvent {}

pub struct Textbox {
    entity: Entity,
    pub text: String,

    buffer: String,

    units: String,
    multiplier: f32,

    select_pos: u32,
    cursor_pos: u32,
    edit: bool,
    hitx: f32,
    dragx: f32,

    // Events
    on_change: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    on_submit: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl Textbox {
    pub fn new(text: &str) -> Self {
        // id.set_text(state, "Test".to_string())
        //     .set_background(state, nanovg::Color::from_rgb(100, 50, 50));

        Textbox {
            entity: Entity::null(),

            text: text.to_string(),

            buffer: String::new(),

            units: String::new(),

            multiplier: 1.0,

            select_pos: 0,
            cursor_pos: 0,
            edit: false,
            hitx: -1.0,
            dragx: -1.0,

            on_change: None,
            on_submit: None,
        }
    }

    pub fn with_units(mut self, uints: &str) -> Self {
        self.units = uints.to_string();

        self
    }

    pub fn on_change<F>(mut self, on_change: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_change = Some(Box::new(on_change));

        self
    }

    pub fn on_submit<F>(mut self, on_submit: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_submit = Some(Box::new(on_submit));

        self
    }
    // pub fn set_enabled(&self, state: &mut WidgetState, val: bool) {
    //     if val {
    //         self.id
    //             .set_background(state, nanovg::Color::from_rgb(100, 50, 50));
    //     } else {
    //         self.id
    //             .set_background(state, nanovg::Color::from_rgb(50, 50, 100));
    //     }
    // }
}

impl Widget for Textbox {
    type Ret = Entity;
    type Data<'a> = &'a String;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_text(state, &(self.text.to_owned() + &self.units));

        self.entity = entity;

        entity.set_clip_widget(state, entity);

        entity.set_element(state, "textbox")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(textbox_event) = event.message.downcast::<TextboxEvent>() {
            match textbox_event {
                TextboxEvent::SetValue(val) => {
                    if event.target == entity {
                        entity.set_text(state, &(val.to_owned() + &self.units));

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );

                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                    }
                }

                TextboxEvent::Clear => {
                    self.text.clear();
                    self.buffer.clear();
                    entity.set_text(state, "");
                    state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                }

                // TextboxEvent::ResetValue => {
                //     if let Some(text_data) = state.style.text.get_mut(entity) {
                //         text_data.text = self.buffer.clone();
                //     }
                // }
                _ => {}
            }
        }

        let text_data = state.style.text.get(entity).cloned().unwrap_or_default();

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseMove(x, _) => {
                    if self.hitx != -1.0 {
                        self.dragx = *x;

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );

                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                    }
                }

                WindowEvent::MouseDown(button) => {
                    if entity == state.hovered {
                        if self.edit == false && !entity.is_disabled(state) {
                            self.cursor_pos = text_data.len() as u32;
                            self.select_pos = 0;
                            self.buffer = text_data.clone();
                            //state.captured = entity;
                            state.capture(entity);
                            //self.edit = true;
                            entity.set_active(state, true);
                            state.set_focus(entity);
                            
                        }
                        if self.edit == true {
                            self.hitx = state.mouse.cursorx;
                            self.dragx = state.mouse.cursorx;
                        }
                        self.edit = true;

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );

                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                    } else {
                        self.edit = false;
                        entity.set_active(state, false);

                        state.insert_event(
                            Event::new(TextboxEvent::ValueChanged(text_data.clone()))
                                .target(entity),
                        );

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );

                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

                        if state.captured == entity {
                            state.insert_event(
                                Event::new(WindowEvent::MouseDown(*button)).target(state.hovered),
                            );
                        }

                        //state.captured = Entity::null();
                        state.release(entity);
                    }
                }

                WindowEvent::MouseUp(_) => {
                    self.hitx = -1.0;
                }

                WindowEvent::KeyDown(_, key) => {
                    //println!("Code: {:?} Key: {:?}", code, key);

                    
                    if *key == Some(Key::ArrowLeft) {
                        if self.edit {
                            self.hitx = -1.0;
                            if self.cursor_pos > 0 {
                                self.cursor_pos -= 1;
                            }
                            if !state.modifiers.shift {
                                self.select_pos = self.cursor_pos;
                            }

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
                            );
                        }
                    }

                    if *key == Some(Key::ArrowRight) {
                        if self.edit {
                            self.hitx = -1.0;
                            if self.cursor_pos < text_data.len() as u32 {
                                self.cursor_pos += 1;
                            }
                            if !state.modifiers.shift {
                                self.select_pos = self.cursor_pos;
                            }

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
                            );
                        }
                    }
                    if *key == Some(Key::Backspace) {




                        if self.edit {
                            let start = std::cmp::min(self.select_pos, self.cursor_pos) as usize;
                            let end = std::cmp::max(self.select_pos, self.cursor_pos) as usize;
                            //let start = text_data.select_pos as usize;
                            //let end = text_data.cursor_pos as usize;
                            if start == end && self.cursor_pos > 0 {
                                if let Some(txt) = state.style.text.get_mut(entity) {
                                    //txt.remove((self.cursor_pos - 1) as usize);
                                    txt.pop();
                                }

                                self.cursor_pos -= 1;
                                self.select_pos -= 1;
                            } else {
                                if let Some(txt) = state.style.text.get_mut(entity) {
                                    txt.replace_range(start..end, "");
                                }
                                self.cursor_pos = start as u32;
                                self.select_pos = start as u32;
                            }

                            self.text = state.style.text.get(entity).unwrap().to_owned();

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            if let Some(callback) = self.on_change.take() {
                                (callback)(self, state, entity);
                                self.on_change = Some(callback);
                            }

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
                            );
                        }
                    }
                    if *key == Some(Key::Enter) {
                        if self.edit {
                            //text_data.buffer = text_data.text.clone();
                            state.insert_event(
                                Event::new(TextboxEvent::ValueChanged(text_data.clone()))
                                    .target(entity),
                            );

                            if let Some(callback) = self.on_submit.take() {
                                (callback)(self, state, entity);
                                self.on_submit = Some(callback);
                            }

                            self.edit = false;
                            entity.set_active(state, false);
                            state.release(entity);


                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
                            );
                        }
                    }
                    if *key == Some(Key::Escape) {
                        if self.edit {
                            self.text = self.buffer.clone();
                            self.edit = false;
                            entity.set_active(state, false);

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
                            );
                        }
                    }
                }

                WindowEvent::CharInput(input) => {
                    if *input as u8 != 8 && *input as u8 != 13 {
                        // Ignore input when ctrl is being held
                        if state.modifiers.ctrl {
                            return;
                        }
                        if self.edit {
                            //println!("Input: {}", input);
                            let start = std::cmp::min(self.select_pos, self.cursor_pos) as usize;
                            let end = std::cmp::max(self.select_pos, self.cursor_pos) as usize;
                            //let start = text_data.select_pos as usize;
                            //let end = text_data.cursor_pos as usize;
                            if start == end {
                                if let Some(txt) = state.style.text.get_mut(entity) {
                                    txt.insert(start, *input);
                                }

                                //text_data.text.remove((text_data.cursor_pos - 1) as usize);
                                self.cursor_pos += 1;
                                self.select_pos += 1;
                            } else {
                                if let Some(txt) = state.style.text.get_mut(entity) {
                                    txt.replace_range(start..end, &input.to_string());
                                }
                                self.cursor_pos = (start + 1) as u32;
                                self.select_pos = (start + 1) as u32;
                            }

                            self.text = state.style.text.get(entity).unwrap().to_owned();

                            if let Some(callback) = self.on_change.take() {
                                (callback)(self, state, entity);
                                self.on_change = Some(callback);
                            }

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
                            );
                        }
                    }
                }


                WindowEvent::FocusOut => {
                    self.edit = false;
                    entity.set_active(state, false);
                    state.release(entity);


                    state.insert_event(
                        Event::new(WindowEvent::Redraw).target(Entity::root()),
                    );
                }

                _ => {}
            }
        }
    }


    fn on_update<'a>(&mut self, state: &mut State, entity: Entity, data: &Self::Data<'a>) {
        self.text = data.to_string();
        entity.set_text(state, &self.text);
    }

    fn on_draw(
        &mut self,
        state: &mut State,
        entity: Entity,
        canvas: &mut Canvas<OpenGl>,
        //images: &HashMap<String, nanovg::Image>,
    ) {
        // Skip window
        if entity == Entity::root() {
            return;
        }

        // Skip invisible widgets
        if state.data.get_visibility(entity) == Visibility::Invisible {
            //println!("Invisible: {}", entity);
            return;
        }

        // Skip widgets that have 0 opacity
        if state.data.get_opacity(entity) == 0.0 {
            //println!("Zero Opacity: {}", entity);
            return;
        }

        let posx = state.data.get_posx(entity);
        let posy = state.data.get_posy(entity);
        let width = state.data.get_width(entity);
        let height = state.data.get_height(entity);

        let padding_left = match state.style.child_left.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_right = match state.style.child_right.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_top = match state.style.child_top.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_bottom = match state.style.child_bottom.get(entity).unwrap_or(&Units::Auto) {
            Units::Pixels(val) => val,
            _ => &0.0,
        };

        let background_color = state
            .style
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let font_color = state
            .style
            .font_color
            .get(entity)
            .cloned()
            .unwrap_or(crate::Color::rgb(255, 255, 255));

        let border_color = state
            .style
            .border_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let parent = state
            .tree
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let border_radius_top_left = match state
            .style
            .border_radius_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state
            .style
            .border_radius_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state
            .style
            .border_radius_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state
            .style
            .border_radius_bottom_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let opacity = state.data.get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        // Skip widgets with no width or no height
        if width == 0.0 || height == 0.0 {
            return;
        }

        // Apply transformations
        let transform = state.data.get_transform(entity);

        canvas.save();
        canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);
        canvas.translate(posx, posy);

        //let pt = canvas.transform().inversed().transform_point(posx + width / 2.0, posy + height / 2.0);
        //canvas.translate(posx + width / 2.0, posy + width / 2.0);
        // canvas.translate(pt.0, pt.1);
        // canvas.scale(1.0, scaley.0);
        // canvas.translate(-pt.0, -pt.1);

        // Apply Scissor
        let clip_region = state.data.get_clip_region(entity);
        // canvas.scissor(
        //     clip_region.x - posx,
        //     clip_region.y - posy,
        //     clip_region.w,
        //     clip_region.h,
        // );

        let outer_shadow_h_offset = match state
            .style
            .outer_shadow_h_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let outer_shadow_v_offset = match state
            .style
            .outer_shadow_v_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let outer_shadow_blur = match state
            .style
            .outer_shadow_blur
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let outer_shadow_color = state
            .style
            .outer_shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let mut outer_shadow_color: femtovg::Color = outer_shadow_color.into();
        outer_shadow_color.set_alphaf(outer_shadow_color.a * opacity);

        let inner_shadow_h_offset = match state
            .style
            .inner_shadow_h_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let inner_shadow_v_offset = match state
            .style
            .inner_shadow_v_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let inner_shadow_blur = match state
            .style
            .inner_shadow_blur
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let inner_shadow_color = state
            .style
            .inner_shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let mut inner_shadow_color: femtovg::Color = inner_shadow_color.into();
        inner_shadow_color.set_alphaf(inner_shadow_color.a * opacity);

        // Draw outer shadow
        let mut path = Path::new();
        path.rect(
            0.0 - outer_shadow_blur + outer_shadow_h_offset,
            0.0 - outer_shadow_blur + outer_shadow_v_offset,
            width + 2.0 * outer_shadow_blur,
            height + 2.0 * outer_shadow_blur,
        );
        path.rounded_rect_varying(
            0.0,
            0.0,
            width,
            height,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        path.solidity(Solidity::Hole);

        let paint = Paint::box_gradient(
            0.0 + outer_shadow_h_offset,
            0.0 + outer_shadow_v_offset,
            width,
            height,
            border_radius_top_left,
            outer_shadow_blur,
            outer_shadow_color,
            femtovg::Color::rgba(0, 0, 0, 0),
        );

        canvas.fill_path(&mut path, paint);

        let mut path = Path::new();

        if border_radius_bottom_left == (width - 2.0 * border_width) / 2.0
            && border_radius_bottom_right == (width - 2.0 * border_width) / 2.0
            && border_radius_top_left == (width - 2.0 * border_width) / 2.0
            && border_radius_top_right == (width - 2.0 * border_width) / 2.0
        {
            path.circle(
                0.0 + (border_width / 2.0) + (width - border_width) / 2.0,
                0.0 + (border_width / 2.0) + (height - border_width) / 2.0,
                width / 2.0,
            );
        } else {
            // Draw rounded rect
            path.rounded_rect_varying(
                0.0 + (border_width / 2.0),
                0.0 + (border_width / 2.0),
                width - border_width,
                height - border_width,
                border_radius_top_left,
                border_radius_top_right,
                border_radius_bottom_right,
                border_radius_bottom_left,
            );
        }

        // Fill with background color
        let mut paint = Paint::color(background_color);

        // Gradient overrides background color
        if let Some(background_gradient) = state.style.background_gradient.get_mut(entity) {
            let (start_x, start_y, end_x, end_y) = match background_gradient.direction {
                Direction::LeftToRight => (0.0, 0.0, width, 0.0),
                Direction::TopToBottom => (0.0, 0.0, 0.0, height),
                _ => (0.0, 0.0, width, 0.0),
            };

            paint = Paint::linear_gradient_stops(
                start_x,
                start_y,
                end_x,
                end_y,
                background_gradient
                    .get_stops(parent_width)
                    .iter()
                    .map(|stop| {
                        let col: femtovg::Color = stop.1.into();
                        (stop.0, col)
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
        }

        // Fill the quad
        canvas.fill_path(&mut path, paint);

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        canvas.stroke_path(&mut path, paint);

        // Draw inner shadow
        let mut path = Path::new();
        path.rounded_rect_varying(
            0.0,
            0.0,
            width,
            height,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_left,
            border_radius_bottom_right,
        );

        let paint = Paint::box_gradient(
            0.0 + inner_shadow_h_offset,
            0.0 + inner_shadow_v_offset,
            width,
            height,
            border_radius_top_left,
            inner_shadow_blur,
            femtovg::Color::rgba(0, 0, 0, 0),
            inner_shadow_color,
        );
        canvas.fill_path(&mut path, paint);

        let mut font_color: femtovg::Color = font_color.into();
        font_color.set_alphaf(font_color.a * opacity);

        canvas.translate(-posx, -posy);
        canvas.restore();

        canvas.save();
        canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);
        //canvas.scissor(clip_region.x, clip_region.y, clip_region.w, clip_region.h);

        if let Some(text) = state.style.text.get_mut(entity) {
            let font = state.style.font.get(entity).cloned().unwrap_or_default();

            let font_id = match font.as_ref() {
                "sans" => state.fonts.regular.unwrap(),
                "icons" => state.fonts.icons.unwrap(),
                _ => state.fonts.regular.unwrap(),
            };

            let mut x = posx;
            let mut y = posy;
            let mut sx = posx;
            let mut sy = posy;

            let text_string = text.to_owned();

            let font_size = state.style.font_size.get(entity).cloned().unwrap_or(16.0);

            let mut paint = Paint::color(font_color);
            paint.set_font_size(font_size);
            paint.set_font(&[font_id]);

            let font_metrics = canvas
                .measure_font(paint)
                .expect("Failed to read font metrics");

            // TODO - Move this to a text layout system and include constraints
            let child_left = state
                .style
                .child_left
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let child_right = state
                .style
                .child_right
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let child_top = state
                .style
                .child_top
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let child_bottom = state
                .style
                .child_bottom
                .get(entity)
                .cloned()
                .unwrap_or_default();

            let align = match child_left {
                Units::Pixels(val) => match child_right {
                    Units::Stretch(_) => {
                        x += val + border_width;
                        Align::Left
                    }

                    _ => Align::Left,
                },

                Units::Stretch(_) => match child_right {
                    Units::Pixels(val) => {
                        x += width - val - border_width;
                        Align::Right
                    }

                    Units::Stretch(_) => {
                        x += 0.5 * width;
                        Align::Center
                    }

                    _ => Align::Right,
                },

                _ => Align::Left,
            };

            let baseline = match child_top {
                Units::Pixels(val) => match child_bottom {
                    Units::Stretch(_) => {
                        y += val + border_width;
                        Baseline::Top
                    }

                    _ => Baseline::Top,
                },

                Units::Stretch(_) => match child_bottom {
                    Units::Pixels(val) => {
                        y += height - val - border_width;
                        sy = y - font_metrics.height();
                        Baseline::Bottom
                    }

                    Units::Stretch(_) => {
                        y += 0.5 * height;
                        sy = y - font_metrics.height() * 0.5;
                        Baseline::Middle
                    }

                    _ => Baseline::Top,
                },

                _ => Baseline::Top,
            };

            paint.set_text_align(align);
            paint.set_text_baseline(baseline);

            if let Ok(res) = canvas.fill_text(x, y, &text_string, paint) {
                let text_width = res.width();
                //let mut glyph_positions = res.glyphs.iter().peekable();

                let mut caretx = x;

                let mut selectx = caretx;

                if self.edit {
                    let startx = if let Some(first_glyph) = res.glyphs.first() {
                        first_glyph.x
                    } else {
                        0.0 + padding_right
                    };
                    //let startx = x - text_width / 2.0;
                    let endx = startx + text_width;

                    if self.hitx != -1.0 {
                        //let endx = res.glyphs.last().unwrap().x + res.glyphs.last().unwrap().w;

                        selectx = if self.hitx < startx + text_width / 2.0 {
                            self.select_pos = 0;
                            startx
                        } else {
                            self.select_pos = text.len() as u32;
                            endx
                        };

                        caretx = if self.dragx < startx + text_width / 2.0 {
                            self.cursor_pos = 0;
                            startx
                        } else {
                            self.cursor_pos = text.len() as u32;
                            endx
                        };

                        let mut n = 0;
                        let mut px = x + padding_left;

                        for glyph in res.glyphs.iter() {
                            let left_edge = glyph.x;
                            let right_edge = left_edge + glyph.width;
                            let gx = left_edge * 0.3 + right_edge * 0.7;

                            // if n == 0 && self.hitx <= glyph.x {
                            //     selectx = left_edge;
                            //     self.select_pos = 0;
                            // }

                            // if n == res.glyphs.len() as u32 && self.hitx >= glyph.x + glyph.width {
                            //     selectx = right_edge;
                            //     self.select_pos = n;
                            // }

                            // if n == 0 && self.dragx <= glyph.x {
                            //     caretx = left_edge;
                            //     self.cursor_pos = 0;
                            // }

                            // if n == res.glyphs.len() as u32 && self.hitx >= glyph.x + glyph.width {
                            //     caretx = right_edge;
                            //     self.cursor_pos = n;
                            // }

                            if self.hitx >= px && self.hitx < gx {
                                selectx = left_edge;

                                self.select_pos = n;
                            }

                            if self.dragx >= px && self.dragx < gx {
                                caretx = left_edge;

                                self.cursor_pos = n;
                            }

                            px = gx;
                            n += 1;
                        }
                    } else {
                        let mut n = 0;
                        //println!("cursor: {}", self.cursor_pos);
                        //let mut start_x = 0.0;

                        for glyph in res.glyphs.iter() {
                            if n == self.cursor_pos {
                                caretx = glyph.x;
                            }

                            if n == self.select_pos {
                                selectx = glyph.x;
                            }

                            n += 1;
                        }

                        if self.cursor_pos as usize == text.len() && text.len() != 0 {
                            caretx = endx;
                        }

                        if self.select_pos as usize == text.len() && text.len() != 0 {
                            selectx = endx;
                        }
                    }

                    //Draw selection
                    let select_width = (caretx - selectx).abs();
                    if selectx > caretx {
                        let mut path = Path::new();
                        path.rect(caretx, sy, select_width, font_metrics.height());
                        canvas.fill_path(&mut path, Paint::color(Color::rgba(0, 0, 0, 64)));
                    } else if caretx > selectx {
                        let mut path = Path::new();
                        path.rect(selectx, sy, select_width, font_metrics.height());
                        canvas.fill_path(&mut path, Paint::color(Color::rgba(0, 0, 0, 64)));
                    }

                    // Draw Caret
                    let mut path = Path::new();
                    path.rect(caretx.floor(), sy, 1.0, font_metrics.height());
                    canvas.fill_path(&mut path, Paint::color(Color::rgba(247, 76, 0, 255)));

                    // let mut path = Path::new();
                    // path.rect(endx, y - 0.25 * height, 1.0, height * 0.5);
                    // canvas.fill_path(&mut path, Paint::color(Color::rgba(255, 0, 0, 255)));
                }
            }
        }
        canvas.restore();
    }
}

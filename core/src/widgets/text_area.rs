use crate::Fsm;
use crate::Movement;
use crate::Selection;
use crate::TextHandler;
use crate::TextMessage;
use crate::TextState;
use crate::TextData;
use crate::style::*;
use crate::widgets::*;
use femtovg::{Align, Baseline, Paint, Path};


pub struct TextArea {
    //text_span: TextSpan,
    text_state: TextState,
    text_data: TextData,
    //selection: Selection,
    //insert_position: usize,

    break_width: f32,
}

impl TextArea {
    pub fn new(text: &str) -> Self {
        Self {
            text_state: TextState::default(),
            text_data: TextData::new(text),

            break_width: 0.0,
        }
    }
}

impl Widget for TextArea {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {


        state.set_focus(entity);

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::KeyDown(_, key) => match key {
                    
                    Some(Key::Character(c)) if !state.modifiers.ctrl && !state.modifiers.alt => {
                        self.text_state = self.text_state.handle_message(TextMessage::Insert(c.to_owned()), &mut self.text_data);
                    
                    }
                    
                    
                    Some(Key::ArrowLeft) => {
                        self.text_state = self.text_state.handle_message(TextMessage::Move(Movement::Upstream), &mut self.text_data);
                        entity.emit_to(state, Entity::root(), WindowEvent::Redraw);
                    }

                    Some(Key::ArrowRight) => {
                        self.text_state = self.text_state.handle_message(TextMessage::Move(Movement::Downstream), &mut self.text_data);
                        entity.emit_to(state, Entity::root(), WindowEvent::Redraw);
                    }

                    _ => {}
                },

                WindowEvent::CharInput(c) => {
                    self.text_state.handle_message(TextMessage::Insert(c.to_string()), &mut self.text_data);
                    entity.emit_to(state, Entity::root(), WindowEvent::Redraw);
                }

                _ => {}
            }
        }
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        let bounds = state.data.get_bounds(entity);

        let parent = state
            .hierarchy
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let background_color = state
            .style
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let border_radius_top_left = match state
            .style
            .border_radius_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Pixels(val) => val,
            Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state
            .style
            .border_radius_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Pixels(val) => val,
            Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state
            .style
            .border_radius_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Pixels(val) => val,
            Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state
            .style
            .border_radius_bottom_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Pixels(val) => val,
            Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let mut background_color: femtovg::Color = background_color.into();

        let mut path = Path::new();

        path.rounded_rect_varying(
            bounds.x,
            bounds.y,
            bounds.w,
            bounds.h,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        let mut paint = Paint::color(background_color);
        canvas.fill_path(&mut path, paint);

        // Draw the text
        let font_id = state.fonts.icons.unwrap();

        let text_string = self.text_data.slice(0..self.text_data.len());

        let mut x = bounds.x;
        let mut y = bounds.y;

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

        // let align = match child_left {
        //     Units::Pixels(val) => match child_right {
        //         Units::Stretch(_) => {
        //             x += val;
        //             Align::Left
        //         }

        //         _ => Align::Left,
        //     },

        //     Units::Stretch(_) => match child_right {
        //         Units::Pixels(val) => {
        //             x += bounds.w - val;
        //             Align::Right
        //         }

        //         Units::Stretch(_) => {
        //             x += 0.5 * bounds.w;
        //             Align::Center
        //         }

        //         _ => Align::Right,
        //     },

        //     _ => Align::Left,
        // };

        // let baseline = match child_top {
        //     Units::Pixels(val) => match child_bottom {
        //         Units::Stretch(_) => {
        //             y += val;
        //             Baseline::Top
        //         }

        //         _ => Baseline::Top,
        //     },

        //     Units::Stretch(_) => match child_bottom {
        //         Units::Pixels(val) => {
        //             y += bounds.h - val;
        //             Baseline::Bottom
        //         }

        //         Units::Stretch(_) => Baseline::Middle,

        //         _ => Baseline::Bottom,
        //     },

        //     _ => Baseline::Top,
        // };

        let font_size = state.style.font_size.get(entity).cloned().unwrap_or(16.0);

        let mut paint = Paint::color(femtovg::Color::black());
        paint.set_font_size(font_size);
        paint.set_font(&[font_id]);
        paint.set_text_align(Align::Left);
        paint.set_text_baseline(Baseline::Top);

        // let text_lines = canvas.break_text_vec(bounds.w, text_string, paint).unwrap();
        // let mut total_height = 0.0;
        // for line_range in text_lines.into_iter() {
        //     let text_metreics = canvas.measure_text(x, y, &text_string[line_range], paint).unwrap();

        //     total_height += text_metreics.height();

        // }
        let font_metrics = canvas.measure_font(paint).unwrap();
        

        // The x position of the selection anchor point
        let (mut anchorx, mut anchory) = (0.0, 0.0);
        // The x position of the selection anchor point
        let (mut activex, mut activey) = (0.0, 0.0);

        //let dy = y;
        let mut i = 0;

        let text_lines = canvas.break_text_vec(bounds.w, &text_string, paint).unwrap();
        
        
        for (line_num, line_range) in text_lines.into_iter().enumerate() {
            if let Ok(text_metrics) = canvas.fill_text(x, y, &text_string[line_range.clone()], paint) {
                
                for (index, glyph) in text_metrics.glyphs.iter().enumerate() {
                    
                    println!("i: {} {}", line_num, i);
                    if i == self.text_data.selection().anchor {
                        anchorx = glyph.x;
                        anchory = y;
                    }
                    //println!("{} {} {} {}", (prev_line_length*line_num) + index, prev_line_length, line_num, index);
                    if i == self.text_data.selection().active {
                        activex = glyph.x;
                        activey = y;
                        //println!("{}", (line_range.len()*line_num) + index);
                        //println!("{} {}",self.text_data.selection().active, text_string.chars().nth(self.text_data.selection().active).unwrap());
                    }

                    i += 1;

                }

                

                y += text_metrics.height();
            }
        }

        // //Draw selection
        // let select_width = (anchorx - selectx).abs();
        // if anchorx > activex {
        //     let mut path = Path::new();
        //     path.rect(caretx, sy, select_width, font_metrics.height());
        //     canvas.fill_path(&mut path, Paint::color(Color::rgba(0, 0, 0, 64)));
        // } else if caretx > selectx {
        //     let mut path = Path::new();
        //     path.rect(selectx, sy, select_width, font_metrics.height());
        //     canvas.fill_path(&mut path, Paint::color(Color::rgba(0, 0, 0, 64)));
        // }

        // Draw Caret
        let mut path = Path::new();
        path.rect(activex.floor(), activey, 1.0, font_metrics.height());
        canvas.fill_path(&mut path, Paint::color(femtovg::Color::rgba(247, 76, 0, 255)));
    }
}

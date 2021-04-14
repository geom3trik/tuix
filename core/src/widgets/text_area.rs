


use crate::widgets::*;
use crate::style::*;
use femtovg::{Path,Paint, Align, Baseline};

#[derive(Default)]
pub struct TextSpan {
    text: String,
}

impl TextSpan {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }
}

#[derive(Default)]
pub struct Text {
    text: String,
    position: usize,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            position: 0,
        }
    }
}

impl Text {

    /// 
    fn length(&mut self) -> usize {
        self.text.chars().count()
    }

    /// Set the text cursor position
    fn set_cursor(&mut self, position: usize) {
        self.position = position;
    }

    /// Get the text cursor position
    fn get_cursor(&mut self) -> usize {
        self.position
    }

    /// Move text cursor to next codepoint
    fn next(&mut self) -> Option<usize> {
        let current_position = self.get_cursor();

        if current_position == self.text.len() {
            None
        } else {
            let byte = self.text.as_bytes()[current_position];
            self.position = current_position + len_utf8_from_first_byte(byte);
            Some(current_position)
        }
    }

    /// Move text cursor to previous codepoint
    fn prev(&mut self) -> Option<usize> {
        let current_position = self.position;

        if current_position == 0 {
            None
        } else {
            let mut length = 1;
            while !self.text.is_char_boundary(current_position - length) {
                length += 1;
            }
            self.position = current_position - length;
            Some(self.position)
        }
    }
}

// https://github.com/linebender/druid/blob/master/druid/src/text/editable_text.rs
pub fn len_utf8_from_first_byte(byte: u8) -> usize {
    match byte {
        byte if byte < 0x80 => 1,
        byte if byte < 0xe0 => 2,
        byte if byte < 0xf0 => 3,
        _ => 4,
    }
}

#[derive(Default)]
pub struct Selection {

}



pub struct TextArea {
    text_span: TextSpan,
    text: Text,
    selection: Selection,
    insert_position: usize,

    break_width: f32,
}

impl TextArea {
    pub fn new(text: &str) -> Self {
        Self {
            text_span: TextSpan::new(text),
            text: Text::new(text),
            selection: Default::default(),
            insert_position: 0,

            break_width: 0.0,
        }
    }
}

impl Widget for TextArea {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::KeyDown(code, key) => {
                    match key {
                        Some(Key::ArrowLeft) => {

                        }

                        Some(Key::ArrowRight) => {
                            let c = self.text.text.chars().nth(self.text.position);
                            println!("{:?}", c);
                            self.text.next();
                        }

                        _=> {}
                    }
                }

                WindowEvent::CharInput(c) => {
                    self.text_span.text.insert(0, *c);
                }

                _=> {}
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

        let text_string = self.text_span.text.as_str();

        let mut x = bounds.x;
        let mut y = bounds.y;

        let text_align = state
            .style
            .text_align
            .get(entity)
            .cloned()
            .unwrap_or_default();
        let text_justify = state
            .style
            .text_justify
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let align = match text_justify {
            Justify::Start => {
                Align::Left
            }
            Justify::Center => {
                x += 0.5 * bounds.w;
                Align::Center
            }
            Justify::End => {
                x += bounds.w;
                Align::Right
            }
        };

        let baseline = match text_align {
            crate::Align::Start => {
                Baseline::Top
            }
            crate::Align::Center => {
                y += 0.5 * bounds.h;
                Baseline::Middle
            }
            crate::Align::End => {
                y += bounds.h;
                Baseline::Bottom
            }
        };

        let font_size = state.style.font_size.get(entity).cloned().unwrap_or(16.0);

        let mut paint = Paint::color(femtovg::Color::black());
        paint.set_font_size(font_size);
        paint.set_font(&[font_id]);
        paint.set_text_align(align);
        paint.set_text_baseline(baseline);



        // let text_lines = canvas.break_text_vec(bounds.w, text_string, paint).unwrap();
        // let mut total_height = 0.0;
        // for line_range in text_lines.into_iter() {
        //     let text_metreics = canvas.measure_text(x, y, &text_string[line_range], paint).unwrap();

        //     total_height += text_metreics.height();

        // }
        let font_metrics = canvas.measure_font(paint).unwrap();

        let text_lines = canvas.break_text_vec(bounds.w, text_string, paint).unwrap();
        for (line_num, line_range) in text_lines.into_iter().enumerate() {
            if let Ok(text_metrics) = canvas.fill_text(x, y, &text_string[line_range], paint) {
                // Do nothing
                y += font_metrics.height();
            }
        }




    }
}
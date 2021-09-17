

use crate::{State,Entity, PropGet};
use crate::style::*;

#[allow(dead_code)]
use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, FillRule, FontId, ImageFlags, ImageId, LineCap,
    LineJoin, Paint, Path, Renderer, Solidity,
};

// Draws a shadow based on the shadow style properties of the entity
pub fn draw_shadow(state: &mut State, entity: Entity, canvas: Canvas<OpenGl>) {

}

// Draws the widget with specified background and border
pub fn draw_widget(state: &mut State, entity: Entity, canvas: Canvas<OpenGl>) {

}

// Draws the text based on style properties
pub fn draw_text(state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
    let parent = entity.get_parent(state).unwrap();

    let parent_width = state.data.get_width(parent);
    let parent_height = state.data.get_height(parent);

    let padding_left = entity.get_padding_left(state).get_value(parent_width);
    let padding_right = entity.get_padding_right(state).get_value(parent_width);
    let padding_top = entity.get_padding_top(state).get_value(parent_height);
    let padding_bottom = entity.get_padding_bottom(state).get_value(parent_height);

    let border_width = entity.get_border_width(state).get_value(parent_width);


    let width = state.data.get_width(entity);
    let height = state.data.get_width(entity);

    if let Some(text) = state.style.text.get(entity) {
        // Get the desired font id
        let font_id = match text.font.as_ref() {
            "sans" => state.fonts.regular.unwrap(),
            "icons" => state.fonts.icons.unwrap(),
            "emoji" => state.fonts.emoji.unwrap(),

            _ => state.fonts.regular.unwrap(),
        };

        let mut x = 0.0;
        let mut y = 0.0;

        let text_string = text.text.to_owned();

        // Maybe this should use justify_content and align_items?
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
                x += padding_left;
                Align::Left
            }
            Justify::Center => {
                x += 0.5 * width;
                Align::Center
            }
            Justify::End => {
                x += width - padding_right;
                Align::Right
            }
        };

        let baseline = match text_align {
            crate::Align::Start => {
                y += padding_top;
                Baseline::Top
            }
            crate::Align::Center => {
                y += 0.5 * height;
                Baseline::Middle
            }
            crate::Align::End => {
                y += height - padding_bottom;
                Baseline::Bottom
            }
        };

        let font_color = state
            .style
            .font_color
            .get(entity)
            .cloned()
            .unwrap_or(crate::Color::rgb(255, 255, 255));
        
        let opacity = state.data.get_opacity(entity);

        let mut font_color: femtovg::Color = font_color.into();
        font_color.set_alphaf(font_color.a * opacity);

        let font_size = state.style.font_size.get(entity).cloned().unwrap_or(16.0);

        let mut paint = Paint::color(font_color);
        paint.set_font_size(font_size);
        paint.set_font(&[font_id]);
        paint.set_text_align(align);
        paint.set_text_baseline(baseline);
        paint.set_anti_alias(false);

        canvas.fill_text(x, y, &text_string, paint);
    }
}

// Applies a clipping scissor to the widget
pub fn apply_scissor(state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
    let mut clip_region = state.data.get_clip_region(entity);
    canvas.scissor(clip_region.x, clip_region.y, clip_region.w, clip_region.h);
}

// Converts from widget coordinates to screen coordinates
// pub fn widget_to_screen(widget_coordinates: (f32, f32)) -> (f32,f32) {
    
// }
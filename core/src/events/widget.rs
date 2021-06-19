use crate::{builder::Builder, EventHandler, WindowEvent};
use crate::{AsEntity, Entity, Hierarchy, State};
use femtovg::{
    renderer::OpenGl, Align, Baseline, FillRule, FontId, ImageFlags, ImageId, LineCap, LineJoin,
    Paint, Path, Renderer, Solidity,
};

use crate::style::{Direction, Justify, Units, Visibility};
use crate::{Event, EventManager, Message};

use fnv::FnvHashMap;

pub type Canvas = femtovg::Canvas<OpenGl>;

use std::any::Any;
pub trait Widget: std::marker::Sized + 'static {
    type Ret;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret;

    /// Adds the widget into state and returns the associated type Ret - an entity id or a tuple of entity ids
    fn build<F>(mut self, state: &mut State, parent: Entity, mut builder: F) -> Self::Ret
    where
        F: FnMut(Builder<Self>) -> Builder<Self>,
        Self: std::marker::Sized + 'static,
    {
        // Create a new entity
        let entity = state.add(parent);

        state.insert_event(Event::new(WindowEvent::ChildAdded(entity)).direct(parent));

        // Call the on_build function of the widget
        let ret = self.on_build(state, entity);

        // Call the builder closure
        builder(Builder::new(state, entity)).build(self);

        // Return the entity or entities returned by the on_build method
        ret
    }

    // Called when events are flushed
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {}

    // Called when a redraw occurs
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        // Skip window
        if entity == Entity::root() {
            return;
        }

        // Skip invisible widgets
        if state.data.get_visibility(entity) == Visibility::Invisible {
            return;
        }

        // Skip widgets that have 0 opacity
        if state.data.get_opacity(entity) == 0.0 {
            return;
        }

        let bounds = state.data.get_bounds(entity);

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
            .hierarchy
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
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let mut clip_region = state.data.get_clip_region(entity);
        canvas.scissor(
            clip_region.x,
            clip_region.y,
            clip_region.w,
            clip_region.h,
        );

        // Apply transformations
        //let rotate = state.style.rotate.get(entity).unwrap_or(&0.0);
        let mut transform = state.data.get_transform(entity);


        canvas.save();
        canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);

        //canvas.translate(bounds.x, bounds.y);

        //let pt = canvas.transform().inversed().transform_point(posx + width / 2.0, posy + height / 2.0);
        //canvas.translate(posx + width / 2.0, posy + width / 2.0);
        // canvas.translate(pt.0, pt.1);
        // canvas.scale(1.0, scaley.0);
        // canvas.translate(-pt.0, -pt.1);

        // Apply Scissor
        // let mut clip_region = state.data.get_clip_region(entity);

        // let scale = state.data.get_scale(entity);

        // transform.inverse();
        // let (mut clip_x, mut clip_y) = transform.transform_point(clip_region.x, clip_region.y);
        // let (mut clip_w, mut clip_h) = transform.transform_point(clip_region.w - clip_region.x, clip_region.h - clip_region.y);

        // clip_x = clip_region.x;
        // clip_y = clip_region.y;
        // clip_w += clip_x;
        // clip_h += clip_y;

        // canvas.scissor(
        //     clip_x,
        //     clip_y,
        //     clip_w,
        //     clip_h,
        // );

        // canvas.scissor(
        //     clip_region.x,
        //     clip_region.y,
        //     clip_region.w,
        //     clip_region.h,
        // );

        //println!("Entity: {} x: {} y: {} w: {} h: {}", entity,  clip_x, clip_y, clip_w, clip_h);

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
        path.rounded_rect_varying(
            0.0 - outer_shadow_blur + outer_shadow_h_offset,
            0.0 - outer_shadow_blur + outer_shadow_v_offset,
            bounds.w + 2.0 * outer_shadow_blur,
            bounds.h + 2.0 * outer_shadow_blur,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        path.rounded_rect_varying(
            0.0,
            0.0,
            bounds.w,
            bounds.h,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        path.solidity(Solidity::Hole);

        let mut paint = Paint::box_gradient(
            0.0 + outer_shadow_h_offset,
            0.0 + outer_shadow_v_offset,
            bounds.w,
            bounds.h,
            border_radius_top_left
                .max(border_radius_top_right)
                .max(border_radius_bottom_left)
                .max(border_radius_bottom_right),
            outer_shadow_blur,
            outer_shadow_color,
            femtovg::Color::rgba(0, 0, 0, 0),
        );

        canvas.fill_path(&mut path, paint);

        let mut path = Path::new();

        if border_radius_bottom_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_bottom_right == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_right == (bounds.w - 2.0 * border_width) / 2.0
        {
            path.circle(
                bounds.x + (border_width / 2.0) + (bounds.w - border_width) / 2.0,
                bounds.y + (border_width / 2.0) + (bounds.h - border_width) / 2.0,
                bounds.w / 2.0,
            );
        } else {
            // Draw rounded rect
            path.rounded_rect_varying(
                bounds.x + (border_width / 2.0),
                bounds.y + (border_width / 2.0),
                bounds.w - border_width,
                bounds.h - border_width,
                border_radius_top_left,
                border_radius_top_right,
                border_radius_bottom_right,
                border_radius_bottom_left,
            );
        }

        // Fill with background color
        let mut paint = Paint::color(background_color);

        if let Some(background_image) = state.style.background_image.get(entity) {
            if let Some(image_id) = state.resource_manager.image_ids.get(background_image) {
                match image_id {
                    crate::ImageOrId::Id(id) => {
                        paint = Paint::image(*id, 0.0, 0.0, 100.0, 100.0, 0.0, 1.0);
                    }

                    _ => {}
                }
            }
        }

        // Gradient overrides background color
        if let Some(background_gradient) = state.style.background_gradient.get_mut(entity) {
            let (start_x, start_y, end_x, end_y) = match background_gradient.direction {
                Direction::LeftToRight => (0.0, 0.0, bounds.w, 0.0),
                Direction::TopToBottom => (0.0, 0.0, 0.0, bounds.h),
                _ => (0.0, 0.0, bounds.w, 0.0),
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
            0.0 + border_width,
            0.0 + border_width,
            bounds.w - border_width * 2.0,
            bounds.h - border_width * 2.0,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );

        let mut paint = Paint::box_gradient(
            0.0 + inner_shadow_h_offset + border_width,
            0.0 + inner_shadow_v_offset + border_width,
            bounds.w - border_width * 2.0,
            bounds.h - border_width * 2.0,
            border_radius_top_left
                .max(border_radius_top_right)
                .max(border_radius_bottom_left)
                .max(border_radius_bottom_right),
            inner_shadow_blur,
            femtovg::Color::rgba(0, 0, 0, 0),
            inner_shadow_color,
        );
        canvas.fill_path(&mut path, paint);

        
        // Draw text
        if let Some(text) = state.style.text.get_mut(entity) {
            let font = state.style.font.get(entity).cloned().unwrap_or_default();

            let font_id = match font.as_ref() {
                "sans" => state.fonts.regular.unwrap(),
                "icons" => state.fonts.icons.unwrap(),
                "emoji" => state.fonts.emoji.unwrap(),

                _ => state.fonts.regular.unwrap(),
            };

            // let mut x = posx + (border_width / 2.0);
            // let mut y = posy + (border_width / 2.0);

            let mut x = bounds.x;
            let mut y = bounds.y;

            let text_string = text.to_owned();

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
                    Units::Stretch(_) | Units::Auto => {
                        x += val + border_width;
                        Align::Left
                    }

                    _ => Align::Left,
                },

                Units::Stretch(_) => match child_right {
                    Units::Pixels(val) => {
                        x += bounds.w - val - border_width;
                        Align::Right
                    }

                    Units::Stretch(_) => {
                        x += 0.5 * bounds.w;
                        Align::Center
                    }

                    _ => Align::Right,
                },

                _ => Align::Left,
            };

            let baseline = match child_top {
                Units::Pixels(val) => match child_bottom {
                    Units::Stretch(_) | Units::Auto => {
                        y += val + border_width;
                        Baseline::Top
                    }

                    _ => Baseline::Top,
                },

                Units::Stretch(_) => match child_bottom {
                    Units::Pixels(val) => {
                        y += bounds.h - val - border_width;
                        Baseline::Bottom
                    }

                    Units::Stretch(_) => {
                        y += 0.5 * bounds.h;
                        Baseline::Middle
                    }

                    _ => Baseline::Bottom,
                },

                _ => Baseline::Top,
            };

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
        
        //canvas.translate(-bounds.x, -bounds.y);
        canvas.restore();

        //canvas.reset_scissor();
    }
}

// Implement EventHandler for any type implementing Widget
impl<T: Widget> EventHandler for T
where
    T: Widget + 'static,
{
    fn on_event_(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        <T as Widget>::on_event(self, state, entity, event);
    }

    fn on_draw_(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        <T as Widget>::on_draw(self, state, entity, canvas);
    }
}

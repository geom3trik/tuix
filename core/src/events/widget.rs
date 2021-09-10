use crate::{builder::Builder, EventHandler, WindowEvent};
use crate::{AsEntity, BorderCornerShape, Entity, Lens, LensWrap, Node, PropType, State, Tree, Wrapper};
use femtovg::{BlendFactor, CompositeOperation};
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
    type Data: Node;

    fn widget_name(&self) -> String {
        String::new()
    }

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret;

    /// Adds the widget into state and returns the associated type Ret - an entity id or a tuple of entity ids
    fn build<F>(mut self, state: &mut State, parent: impl AsEntity, mut builder: F) -> Self::Ret
    where
        F: FnMut(Builder<Self>) -> Builder<Self>,
        Self: std::marker::Sized + 'static,
    {
        // Create a new entity
        let entity = state.add(parent.entity());

        state.insert_event(Event::new(WindowEvent::ChildAdded(entity)).direct(parent.entity()));

        // Call the on_build function of the widget
        let ret = self.on_build(state, entity);

        // Call the builder closure
        builder(Builder::new(state, entity)).build(self);

        // Return the entity or entities returned by the on_build method
        ret
    }

    fn bind<L: Lens, F>(self, lens: L, converter: F) -> Wrapper<L, Self> 
    where F: 'static + Fn(&<L as Lens>::Target) -> <Self as Widget>::Data
    {
        Wrapper::new(self, lens, converter)
    }

    fn bind2<L: Lens>(self, lens: L) -> LensWrap<L, Self> {
        LensWrap::new(self, lens)
    }

    // Called when data bound to this widget is changed
    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {}

    // Called when events are flushed
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {}

    fn on_style(&mut self, state: &mut State, entity: Entity, property: (String, PropType)) {}

    // Called when a redraw occurs
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {


        let bounds = state.data.get_bounds(entity);

        //Skip widgets with no width or no height
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }
    

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

        let border_corner_shape = state
            .style
            .border_corner_shape
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
            Units::Pixels(val) => val,
            Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
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
            Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
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
            Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
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
            Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
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
            Units::Percentage(val) => bounds.w.min(bounds.h) * (val / 100.0),
            _ => 0.0,
        };




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
            bounds.x - outer_shadow_blur + outer_shadow_h_offset,
            bounds.y - outer_shadow_blur + outer_shadow_v_offset,
            bounds.w + 2.0 * outer_shadow_blur,
            bounds.h + 2.0 * outer_shadow_blur,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
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
        path.solidity(Solidity::Hole);

        let mut paint = Paint::box_gradient(
            bounds.x + outer_shadow_h_offset,
            bounds.y + outer_shadow_v_offset,
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

        match border_corner_shape {
            BorderCornerShape::Round => {
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
            }

            BorderCornerShape::Bevel => {
                path.move_to(bounds.x + border_radius_top_left, bounds.y);
                path.line_to(bounds.x + bounds.w - border_radius_top_right, bounds.y);
                path.line_to(bounds.x + bounds.w, bounds.y + border_radius_top_right);
                path.line_to(bounds.x + bounds.w, bounds.y + bounds.h - border_radius_bottom_right);
                path.line_to(bounds.x + bounds.w - border_radius_bottom_right, bounds.y + bounds.h);
                path.line_to(bounds.x + border_radius_bottom_left, bounds.y + bounds.h);
                path.line_to(bounds.x, bounds.y + bounds.h - border_radius_bottom_left);
                path.line_to(bounds.x, bounds.y + border_radius_top_left);
                path.close()
            }
        }

        // Fill with background color
        let mut paint = Paint::color(background_color);

        // if let Some(background_image) = state.style.background_image.get(entity) {
        //     if let Some(image_id) = state.resource_manager.image_ids.get(background_image) {
        //         match image_id {
        //             crate::ImageOrId::Id(id) => {
        //                 paint = Paint::image(*id, 0.0, 0.0, 100.0, 100.0, 0.0, 1.0);
        //             }

        //             _ => {}
        //         }
        //     }
        // }

        // Gradient overrides background color
        if let Some(background_gradient) = state.style.background_gradient.get_mut(entity) {
            let (start_x, start_y, end_x, end_y, parent_length) = match background_gradient.direction {
                Direction::LeftToRight => (0.0, 0.0, bounds.w, 0.0, parent_width),
                Direction::TopToBottom => (0.0, 0.0, 0.0, bounds.h, parent_height),
                _ => (0.0, 0.0, bounds.w, 0.0, parent_width),
            };

            paint = Paint::linear_gradient_stops(
                bounds.x,
                bounds.y,
                bounds.x + end_x,
                bounds.y + end_y,
                background_gradient
                    .get_stops(parent_length)
                    .iter()
                    .map(|stop| {
                        let col: femtovg::Color = stop.1.into();
                        (stop.0, col)
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
        }

        //canvas.global_composite_blend_func(BlendFactor::DstColor, BlendFactor::OneMinusSrcAlpha);

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
    }
}

// Implement EventHandler for any type implementing Widget
impl<T: Widget> EventHandler for T
where
    T: std::marker::Sized + Widget + 'static,
{

    fn widget_name(&self) -> String {
        <T as Widget>::widget_name(self)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &dyn Node) {
        if let Some(data) = node.downcast_ref() {
             <T as Widget>::on_update(self, state, entity, data);
        }
    }

    fn on_event_(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        <T as Widget>::on_event(self, state, entity, event);
    }

    fn on_style(&mut self, state: &mut State, entity: Entity, property: (String, PropType)) {
        <T as Widget>::on_style(self, state, entity, property);
    }

    fn on_draw_(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        <T as Widget>::on_draw(self, state, entity, canvas);
    }
}



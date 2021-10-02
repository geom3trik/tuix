use crate::{Builder, EventHandler, WindowEvent};
use crate::{AsEntity, BorderCornerShape, Entity, FontOrId, IntoChildIterator, Lens, LensWrap, Node, PropType, State, Tree, Wrapper};
use femtovg::{BlendFactor, CompositeOperation, PixelFormat, RenderTarget};
use femtovg::{
    renderer::OpenGl, Align, Baseline, FillRule, FontId, ImageFlags, ImageId, LineCap, LineJoin,
    Paint, Path, Renderer, Solidity,
};

use crate::{Direction, Units, Visibility};
use crate::{Event, EventManager, Message};

use fnv::FnvHashMap;

pub type Canvas = femtovg::Canvas<OpenGl>;

use std::any::Any;


// Length proportional to radius of a cubic bezier handle for 90deg arcs.
const KAPPA90: f32 = 0.5522847493;

/// Trait implemented by all widgets
pub trait Widget: std::marker::Sized + 'static {
    /// The `Ret` associated type determines whether a single entity or a tuple of entities will be returned when the widget is built.
    /// This can be useful for widgets which are made up of sub-widgets which need to be accessible, see [TabView] for an example.
    type Ret: AsEntity;

    /// The `Data` associated type is used by the binding system to determine the type of data the widget receives during an update.
    type Data: Node;

    fn widget_name(&self) -> String {
        String::new()
    }

    /// Called when the widget is built into state 
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
    /// Bind a piece of data to the widget using a lens and a conversion closure
    fn bind<L: Lens, F>(self, lens: L, converter: F) -> Wrapper<L, Self> 
    where F: 'static + Fn(&<L as Lens>::Target) -> <Self as Widget>::Data
    {
        Wrapper::new(self, lens, converter)
    }

    /// Bind a piece of data to the widget without conversion, allowing the data to be passed as a reference
    fn bind_ref<L: Lens>(self, lens: L) -> LensWrap<L, Self> {
        LensWrap::new(self, lens)
    }

    /// Called when data bound to this widget is changed
    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {

    }

    /// Called when the widget receives an event
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {}


    fn on_style(&mut self, state: &mut State, entity: Entity, property: (String, PropType)) {}

    /// Called when the widget is redrawn. Allows for custom drawing of the widget
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
            .unwrap_or(crate::Color::rgb(0, 0, 0));

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

        let border_shape_top_left = state
            .style
            .border_shape_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let border_shape_top_right = state
            .style
            .border_shape_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let border_shape_bottom_left = state
            .style
            .border_shape_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let border_shape_bottom_right = state
            .style
            .border_shape_bottom_right
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
            Units::Percentage(val) => bounds.w * (val / 100.0),
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
            Units::Percentage(val) => bounds.w * (val / 100.0),
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
            Units::Percentage(val) => bounds.w * (val / 100.0),
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
            Units::Percentage(val) => bounds.w * (val / 100.0),
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
            Units::Percentage(val) => bounds.w * (val / 100.0),
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
            Units::Percentage(val) => bounds.w * (val / 100.0),
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
        
        // // Draw outer shadow
        // let mut path = Path::new();
        // path.rounded_rect_varying(
        //     bounds.x - outer_shadow_blur + outer_shadow_h_offset,
        //     bounds.y - outer_shadow_blur + outer_shadow_v_offset,
        //     bounds.w + 2.0 * outer_shadow_blur,
        //     bounds.h + 2.0 * outer_shadow_blur,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );
        // path.rounded_rect_varying(
        //     bounds.x,
        //     bounds.y,
        //     bounds.w,
        //     bounds.h,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );
        // path.solidity(Solidity::Hole);

        // let mut paint = Paint::box_gradient(
        //     bounds.x + outer_shadow_h_offset,
        //     bounds.y + outer_shadow_v_offset,
        //     bounds.w,
        //     bounds.h,
        //     border_radius_top_left
        //         .max(border_radius_top_right)
        //         .max(border_radius_bottom_left)
        //         .max(border_radius_bottom_right),
        //     outer_shadow_blur,
        //     outer_shadow_color,
        //     femtovg::Color::rgba(0, 0, 0, 0),
        // );

        // canvas.fill_path(&mut path, paint);

        
        //let start = std::time::Instant::now();
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

            let x = bounds.x + border_width / 2.0;
            let y = bounds.y + border_width / 2.0;
            let w = bounds.w - border_width;
            let h = bounds.h - border_width;;
            let halfw = w.abs() * 0.5;
            let halfh = h.abs() * 0.5;

            let rx_bl = border_radius_bottom_left.min(halfw) * w.signum();
            let ry_bl = border_radius_bottom_left.min(halfh) * h.signum();

            let rx_br = border_radius_bottom_right.min(halfw) * w.signum();
            let ry_br = border_radius_bottom_right.min(halfh) * h.signum();

            let rx_tr = border_radius_top_right.min(halfw) * w.signum();
            let ry_tr = border_radius_top_right.min(halfh) * h.signum();

            let rx_tl = border_radius_top_left.min(halfw) * w.signum();
            let ry_tl = border_radius_top_left.min(halfh) * h.signum();

            path.move_to(x, y + ry_tl);
            path.line_to(x, y + h - ry_bl);
            if border_radius_bottom_left != 0.0 {
                if border_shape_bottom_left == BorderCornerShape::Round {
                    path.bezier_to(x, y + h - ry_bl * (1.0 - KAPPA90), x + rx_bl * (1.0 - KAPPA90), y + h, x + rx_bl, y + h);
                } else {
                    path.line_to(x + rx_bl, y + h);
                } 
            }

            path.line_to(x + w - rx_br, y + h);
            
            if border_radius_bottom_right != 0.0 {
                if border_shape_bottom_right == BorderCornerShape::Round {
                    path.bezier_to(x + w - rx_br * (1.0 - KAPPA90), y + h, x + w, y + h - ry_br * (1.0 - KAPPA90), x + w, y + h - ry_br);
                } else {
                    path.line_to(x + w, y + h - ry_br);
                }                
            }

            path.line_to(x + w, y + ry_tr);
            
            if border_radius_top_right != 0.0 {
                if border_shape_top_right == BorderCornerShape::Round {
                    path.bezier_to(x + w, y + ry_tr * (1.0 - KAPPA90), x + w - rx_tr * (1.0 - KAPPA90), y, x + w - rx_tr, y);
                } else {
                    path.line_to(x + w - rx_tr, y);
                }                
            }

            path.line_to(x + rx_tl, y);

            if border_radius_top_left != 0.0 {
                if border_shape_top_left == BorderCornerShape::Round {
                    path.bezier_to(x + rx_tl * (1.0 - KAPPA90), y, x, y + ry_tl * (1.0 - KAPPA90), x, y + ry_tl);
                } else {
                    path.line_to(x, y + ry_tl);
                }                
            }
            
            path.close();

        }

        // Draw outer shadow
        if state.style.outer_shadow_color.get(entity).is_some() {


            let sigma = outer_shadow_blur / 2.0;
            let d = (sigma * 5.0).ceil();

            let shadow_image = state.data.shadow_image.get(&entity).cloned().unwrap_or(
                (
                    canvas.create_image_empty((bounds.w + d) as usize, 
                    (bounds.h + d) as usize, 
                    PixelFormat::Rgba8, 
                    ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                    ).expect("Failed to create image"),

                    canvas.create_image_empty((bounds.w + d) as usize, 
                    (bounds.h + d) as usize, 
                    PixelFormat::Rgba8, 
                    ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                    ).expect("Failed to create image"),
                )
            );

            canvas.save();

            let size = canvas.image_size(shadow_image.0).expect("Failed to get image");


            let (source, target) = if size.0 != (bounds.w + d) as usize || size.1 != (bounds.h + d) as usize {
                canvas.delete_image(shadow_image.0);
                canvas.delete_image(shadow_image.1);

                (
                    canvas.create_image_empty((bounds.w + d) as usize, 
                    (bounds.h + d) as usize, 
                    PixelFormat::Rgba8, 
                    ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                    ).expect("Failed to create image"),

                    canvas.create_image_empty((bounds.w + d) as usize, 
                    (bounds.h + d) as usize, 
                    PixelFormat::Rgba8, 
                    ImageFlags::FLIP_Y | ImageFlags::PREMULTIPLIED,
                    ).expect("Failed to create image"),
                )
            } else {
                (shadow_image.0, shadow_image.1)
            };
        

            state.data.shadow_image.insert(entity, (source, target));

            
            canvas.set_render_target(RenderTarget::Image(source));
            canvas.clear_rect(0, 0, size.0 as u32, size.1 as u32, femtovg::Color::rgba(0,0, 0, 0));
            canvas.translate(-bounds.x + d/2.0, -bounds.y + d/2.0);
            let mut outer_shadow = path.clone();
            let mut paint = Paint::color(outer_shadow_color);
            canvas.fill_path(&mut outer_shadow, paint);


            canvas.restore();

            let target_image = if outer_shadow_blur > 0.0 {
                canvas.filter_image(
                    target,
                    femtovg::ImageFilter::GaussianBlur { sigma },
                    source,
                );
                target
            } else {
                source
            };

            canvas.set_render_target(RenderTarget::Screen);

            canvas.save();
            canvas.translate(outer_shadow_h_offset, outer_shadow_v_offset);
            let mut path = Path::new();
            path.rect(bounds.x - d/2.0, bounds.y - d/2.0, bounds.w + d, bounds.h + d);
            
            canvas.fill_path(&mut path, Paint::image(
                target_image, 
                bounds.x - d/2.0, 
                bounds.y - d/2.0, 
                bounds.w + d, 
                bounds.h + d, 
                0f32, 
                1f32)
            );
            //canvas.fill_path(&mut path, Paint::color(femtovg::Color::rgb(0,0,0)));
            canvas.restore();
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
        if let Some(background_gradient) = state.style.background_gradient.get(entity) {
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

        //println!("{:.2?} seconds for whatever you did.", start.elapsed());

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        canvas.stroke_path(&mut path, paint);

        // // Draw inner shadow
        // let mut path = Path::new();
        // path.rounded_rect_varying(
        //     0.0 + border_width,
        //     0.0 + border_width,
        //     bounds.w - border_width * 2.0,
        //     bounds.h - border_width * 2.0,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );

        // let mut paint = Paint::box_gradient(
        //     0.0 + inner_shadow_h_offset + border_width,
        //     0.0 + inner_shadow_v_offset + border_width,
        //     bounds.w - border_width * 2.0,
        //     bounds.h - border_width * 2.0,
        //     border_radius_top_left
        //         .max(border_radius_top_right)
        //         .max(border_radius_bottom_left)
        //         .max(border_radius_bottom_right),
        //     inner_shadow_blur,
        //     femtovg::Color::rgba(0, 0, 0, 0),
        //     inner_shadow_color,
        // );
        // canvas.fill_path(&mut path, paint);

        
        // Draw text
        if let Some(text) = state.style.text.get(entity) {
            let font = state.style.font.get(entity).cloned().unwrap_or_default();

            // TODO - This should probably be cached in state to save look-up time
            let default_font = state.resource_manager.fonts.get(&state.style.default_font).and_then(|font|{
                match font {
                    FontOrId::Id(id) => Some(id),
                    _=> None,
                }
            }).expect("Failed to find default font");

            let font_id = state.resource_manager.fonts.get(&font).and_then(|font|{
                match font {
                    FontOrId::Id(id) => Some(id),
                    _=> None,
                }
            }).unwrap_or(default_font);

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
            paint.set_font(&[font_id.clone()]);
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

    fn on_update_(&mut self, state: &mut State, entity: Entity, node: &dyn Node) {
        if let Some(data) = node.downcast_ref() {
             <T as Widget>::on_update(self, state, entity, data);
        } else {
            for (_index, child) in entity.child_iter(&state.tree.clone()).enumerate() {
                if let Some(mut event_handler) = state.event_handlers.remove(&child) {

                    event_handler.on_update_(state, child, node);
    
                    state.event_handlers.insert(child, event_handler);
                }
            }
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
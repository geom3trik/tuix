use morphorm::GeometryChanged;
use crate::{Message, State, Widget};
use crate::{entity::Entity, Builder, EventHandler, Propagation};
use crate::{state::style::*, AsEntity, Pos};

use crate::{Event, WindowEvent};

use crate::state::tree::*;

use morphorm::{Cache, LayoutType, PositionType, Units};

use std::rc::Rc;

// // Flag that the posx of the widget may need to update
// fn flag_geo_change(state: &mut State, entity: Entity) {
//     if let Some(parent) = entity.parent(&state.tree) {
//         if let Some(geo_changed) = state.data.geometry_changed.get_mut(parent.index_unchecked()) {
//             geo_changed.set(GeometryChanged::CHANGE_POSX, true);
//         }         
//     }

//     // if let Some(geo_changed) = state.data.geometry_changed.get_mut(entity.index_unchecked()) {
//     //     geo_changed.set(GeometryChanged::CHANGE_POSX, true);
//     // }

// }

fn needs_redraw(state: &mut State, entity: Entity) {
    if let Some(geo_changed) = state.data.geometry_changed.get_mut(entity.index_unchecked()) {
        geo_changed.set(GeometryChanged::POSX_CHANGED, true);
    }
}

pub trait PropSet: AsEntity + Sized {

    /// Helper method for sending an event to self with default propagation
    fn emit(&self, state: &mut State, message: impl Message) -> Entity
    where
        Self: 'static,
    {
        state.insert_event(Event::new(message).target(self.entity()).origin(self.entity()).propagate(Propagation::Up));

        self.entity()
    }

    /// Helper method for sending an event to target with default propagation
    fn emit_to(&self, state: &mut State, target: Entity, message: impl Message) -> Entity {
        state.insert_event(Event::new(message).target(target).origin(self.entity()).propagate(Propagation::Direct));

        self.entity()
    }

    fn add_listener<F,W>(&self, state: &mut State, listener: F) -> Entity
    where 
        W: Widget, 
        F: 'static + Fn(&mut W, &mut State, Entity, &mut Event)
    {  
        state.listeners.insert(self.entity(), Box::new(move |event_handler, state, entity, event|{
            if let Some(widget) = event_handler.downcast::<W>() {
                (listener)(widget, state, entity, event);
            }
        }));

        self.entity()
    }

    fn restyle(&self, state: &mut State) {
        state.insert_event(Event::new(WindowEvent::Restyle).target(self.entity()).origin(self.entity()).unique());
    }

    fn relayout(&self, state: &mut State) {
        state.insert_event(Event::new(WindowEvent::Relayout).target(self.entity()).origin(self.entity()).unique());
    }

    fn redraw(&self, state: &mut State) {
        state.insert_event(Event::new(WindowEvent::Redraw).target(self.entity()).origin(self.entity()).unique());
    }

    fn set_name(self, state: &mut State, name: &str) -> Entity {
        state.style.name.insert(self.entity(), name.to_string());

        self.entity()
    }

    /// Add a class name to an entity
    fn class(self, state: &mut State, class_name: &str) -> Entity {
        if let Some(class_list) = state.style.classes.get_mut(self.entity()) {
            class_list.insert(class_name.to_string());
        } else {
            let mut class_list = HashSet::new();
            class_list.insert(class_name.to_string());
            state.style.classes.insert(self.entity(), class_list);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        ////flag_geo_change(state, self.entity());

        self.entity()
    }

    // TODO move to PropGet
    fn get_parent(self, state: &mut State) -> Option<Entity> {
        self.entity().parent(&state.tree)
    }

    // Pseudoclass

    fn set_disabled(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClasses::DISABLED, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        ////flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_checked(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClasses::CHECKED, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        ////flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_over(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClasses::OVER, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        ////flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_active(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClasses::ACTIVE, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_hover(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClasses::HOVER, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_focus(self, state: &mut State, value: bool) -> Entity {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self.entity()) {
            pseudo_classes.set(PseudoClasses::FOCUS, value);
        }

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    // Style
    fn set_element(self, state: &mut State, value: &str) -> Entity {

        state.style.elements.insert(self.entity(), value.to_string());

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_id(self, state: &mut State, value: &str) -> Entity {
        todo!();

        self.entity()
    }

    // Visibility
    fn set_visibility(self, state: &mut State, value: Visibility) -> Entity {
        state.style.visibility.insert(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_hoverable(self, state: &mut State, value: bool) -> Entity {
        state.data.set_hoverable(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_focusable(self, state: &mut State, value: bool) -> Entity {
        state.data.set_focusable(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    // Overflow
    fn set_overflow(self, state: &mut State, value: Overflow) -> Entity {
        state.style.overflow.insert(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    // Display
    fn set_display(self, state: &mut State, value: Display) -> Entity {
        state.style.display.insert(self.entity(), value);

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    //Opacity
    fn set_opacity(self, state: &mut State, value: f32) -> Entity {
        state.style.opacity.insert(self.entity(), Opacity(value));

        Entity::root().restyle(state);
        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    // Rotate
    fn set_rotate(self, state: &mut State, value: f32) -> Entity {
        state.style.rotate.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_translate(self, state: &mut State, value: (f32, f32)) -> Entity {
        state.style.translate.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_scale(self, state: &mut State, value: f32) -> Entity {
        state.style.scale.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    // Position
    fn set_position_type(self, state: &mut State, value: PositionType) -> Entity {
        state.style.positioning_type.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_space(self, state: &mut State, value: Units) -> Entity {
        state.style.left.insert(self.entity(), value);
        state.style.right.insert(self.entity(), value);
        state.style.top.insert(self.entity(), value);
        state.style.bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());
        self.entity()
    } 

    fn set_left(self, state: &mut State, value: Units) -> Entity {
        //println!("Set Left {} {} {:?}", self.entity(), state.style.elements.get(self.entity()).cloned().unwrap_or_default(), value);
        state.style.left.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_right(self, state: &mut State, value: Units) -> Entity {
        state.style.right.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_top(self, state: &mut State, value: Units) -> Entity {
        state.style.top.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_bottom(self, state: &mut State, value: Units) -> Entity {
        state.style.bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    // Position Constraints
    fn set_min_left(self, state: &mut State, value: Units) -> Entity {
        state.style.min_left.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_max_left(self, state: &mut State, value: Units) -> Entity {
        state.style.max_left.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_min_right(self, state: &mut State, value: Units) -> Entity {
        state.style.min_right.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_max_right(self, state: &mut State, value: Units) -> Entity {
        state.style.max_right.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_min_top(self, state: &mut State, value: Units) -> Entity {
        state.style.min_top.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_max_top(self, state: &mut State, value: Units) -> Entity {
        state.style.max_top.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_min_bottom(self, state: &mut State, value: Units) -> Entity {
        state.style.min_bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_max_bottom(self, state: &mut State, value: Units) -> Entity {
        state.style.max_bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    // Size
    fn set_width(self, state: &mut State, value: Units) -> Entity {
        
        state.style.width.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    fn set_height(self, state: &mut State, value: Units) -> Entity {
        //println!("Set Height: {} {:?}", self.entity(), value);
        state.style.height.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        self.entity()
    }

    // Size Constraints
    fn set_min_width(self, state: &mut State, value: Units) -> Entity {
        state.style.min_width.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_max_width(self, state: &mut State, value: Units) -> Entity {
        state.style.max_width.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_min_height(self, state: &mut State, value: Units) -> Entity {
        state.style.min_height.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_max_height(self, state: &mut State, value: Units) -> Entity {
        state.style.max_height.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    // Text
    fn set_text(self, state: &mut State, text: &str) -> Entity {
        state.style.text.insert(self.entity(), text.to_owned());

        Entity::root().redraw(state);

        self.entity()
    }

    // Text Font
    fn set_font(self, state: &mut State, font: &str) -> Entity {
        state.style.font.insert(self.entity(), font.to_owned());

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_font_size(self, state: &mut State, value: f32) -> Entity {
        state.style.font_size.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_color(self, state: &mut State, value: Color) -> Entity {
        state.style.font_color.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    // Tooltip
    fn set_tooltip(self, state: &mut State, text: &str) -> Entity {
        state.style.tooltip.insert(self.entity(), text.to_owned());

        Entity::root().redraw(state);

        self.entity()
    }

    // Background
    fn set_background_color(self, state: &mut State, value: Color) -> Entity {
        state.style.background_color.insert(self.entity(), value);

        Entity::root().redraw(state);

        needs_redraw(state, self.entity());

        self.entity()
    }

    fn set_background_image(self, state: &mut State, value: Rc<()>) -> Entity {
        state.style.background_image.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    // Border
    fn set_border_width(self, state: &mut State, value: Units) -> Entity {
        state.style.border_width.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_color(self, state: &mut State, value: Color) -> Entity {
        state.style.border_color.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_corner_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_top_left.insert(self.entity(), value);
        state.style.border_shape_top_right.insert(self.entity(), value);
        state.style.border_shape_bottom_left.insert(self.entity(), value);
        state.style.border_shape_bottom_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_top_left_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_top_left.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_top_right_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_top_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_bottom_left_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_bottom_left.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_bottom_right_shape(self, state: &mut State, value: BorderCornerShape) -> Entity {
        state.style.border_shape_bottom_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }


    // Border Radius
    fn set_border_radius(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_top_left.insert(self.entity(), value);
        state.style.border_radius_top_right.insert(self.entity(), value);
        state.style.border_radius_bottom_left.insert(self.entity(), value);
        state.style.border_radius_bottom_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }


    fn set_border_radius_top_left(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_top_left.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_radius_top_right(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_top_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_radius_bottom_left(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_bottom_left.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_border_radius_bottom_right(self, state: &mut State, value: Units) -> Entity {
        state.style.border_radius_bottom_right.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    // Outer Shadow
    fn set_outer_shadow_h_offset(mut self, state: &mut State, value: Units) -> Self {
        state
            .style
            .outer_shadow_h_offset
            .insert(self.entity(), value);

        self
    }

    fn set_outer_shadow_v_offset(mut self, state: &mut State, value: Units) -> Self {
        state
            .style
            .outer_shadow_v_offset
            .insert(self.entity(), value);

        self
    }

    fn set_outer_shadow_color(mut self, state: &mut State, value: Color) -> Self {
        state.style.outer_shadow_color.insert(self.entity(), value);

        self
    }

    fn set_outer_shadow_blur(mut self, state: &mut State, value: Units) -> Self {
        state.style.outer_shadow_blur.insert(self.entity(), value);

        self
    }

    // Clipping
    fn set_clip_widget(self, state: &mut State, value: Entity) -> Entity {
        state.style.clip_widget.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_z_order(self, state: &mut State, value: i32) -> Entity {
        state.style.z_order.insert(self.entity(), value);

        Entity::root().redraw(state);

        self.entity()
    }

    fn set_next_focus(self, state: &mut State, value: Entity) -> Entity {
        if let Some(entity) = state.style.focus_order.get_mut(self.entity()) {
            entity.next = value;
        } else {
            state.style.focus_order.insert(
                self.entity(),
                FocusOrder {
                    next: value,
                    ..Default::default()
                },
            );
        }

        self.entity()
    }

    fn set_prev_focus(self, state: &mut State, value: Entity) -> Entity {
        if let Some(focus_order) = state.style.focus_order.get_mut(self.entity()) {
            focus_order.prev = value;
        } else {
            state.style.focus_order.insert(
                self.entity(),
                FocusOrder {
                    prev: value,
                    ..Default::default()
                },
            );
        }

        self.entity()
    }

    fn set_focus_order(self, state: &mut State, prev: Entity, next: Entity) -> Entity {
        if let Some(focus_order) = state.style.focus_order.get_mut(self.entity()) {
            focus_order.prev = prev;
            focus_order.next = next;
        } else {
            state.style.focus_order.insert(
                self.entity(),
                FocusOrder {
                    prev,
                    next,
                },
            );
        }

        self.entity()
    }


    //fn mutate<F: FnMut(Builder<Self>) -> Builder<Self>>(self, state: &mut State, builder: F) -> Self;

    fn set_layout_type(&self, state: &mut State, value: LayoutType) -> Entity {
        state.style.layout_type.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_child_space(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_left.insert(self.entity(), value);
        state.style.child_right.insert(self.entity(), value);
        state.style.child_top.insert(self.entity(), value);
        state.style.child_bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_child_left(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_left.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_row_between(&self, state: &mut State, value: Units) -> Entity {
        state.style.row_between.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_col_between(&self, state: &mut State, value: Units) -> Entity {
        state.style.col_between.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_child_right(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_right.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_child_top(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_top.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_child_bottom(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_bottom.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_grid_rows(&self, state: &mut State, value: Vec<Units>) -> Entity {
        state.style.grid_rows.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);
        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_grid_cols(&self, state: &mut State, value: Vec<Units>) -> Entity {
        state.style.grid_cols.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_row_index(&self, state: &mut State, value: usize) -> Entity {
        state.style.row_index.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_col_index(&self, state: &mut State, value: usize) -> Entity {
        state.style.col_index.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_row_span(&self, state: &mut State, value: usize) -> Entity {
        state.style.row_span.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);

        //flag_geo_change(state, self.entity());

        self.entity()
    }

    fn set_col_span(mut self, state: &mut State, value: usize) -> Self {
        state.style.col_span.insert(self.entity(), value);

        Entity::root().relayout(state);
        Entity::root().redraw(state);
        
        //flag_geo_change(state, self.entity());

        self
    }

}

impl<T: AsEntity> PropSet for T {
    // fn mutate<F>(self, state: &mut State, mut builder: F) -> Self
    // where
    //     F: FnMut(Builder<Self>) -> Builder<Self>,
    // {
    //     builder(Builder::new(state, self));

    //     self
    // }
}
pub trait PropGet: AsEntity {


    fn name(&self, state: &mut State) -> String {
        state.style.name.get(self.entity()).cloned().unwrap_or_default()
    }

    fn element(&self, state: &mut State) -> String {
        state.style.elements.get(self.entity()).cloned().unwrap_or_default()
    }

    fn is_disabled(self, state: &mut State) -> bool;
    fn is_checked(self, state: &mut State) -> bool;
    fn is_over(self, state: &mut State) -> bool;
    fn is_active(self, state: &mut State) -> bool;
    fn is_focused(self, state: &mut State) -> bool;
    fn is_selected(self, state: &mut State) -> bool;
    fn is_hovered(self, state: &mut State) -> bool;

    //
    fn get_overflow(&self, state: &mut State) -> Overflow;

    // Display
    fn get_display(&self, state: &mut State) -> Display;

    fn get_layout_type(&self, state: &mut State) -> LayoutType {
        state
            .style
            .layout_type
            .get(self.entity())
            .cloned()
            .unwrap_or_default()
    }

    // Background Color
    fn get_background_color(&self, state: &mut State) -> Color {
        state.style.background_color.get(self.entity()).cloned().unwrap_or_default()
    }

    // Position
    fn get_left(&self, state: &mut State) -> Units;
    fn get_right(&self, state: &mut State) -> Units;
    fn get_top(&self, state: &mut State) -> Units;
    fn get_bottom(&self, state: &mut State) -> Units;

    // Size
    fn get_width(&self, state: &mut State) -> Units;
    fn get_height(&self, state: &mut State) -> Units;

    // Size Constraints
    fn get_min_width(&self, state: &mut State) -> Units;
    fn get_max_width(&self, state: &mut State) -> Units;
    fn get_min_height(&self, state: &mut State) -> Units;
    fn get_max_height(&self, state: &mut State) -> Units;

    // Border
    fn get_border_width(&self, state: &mut State) -> Units;

    // Tooltip
    fn get_tooltip(&self, state: &mut State) -> String;

    // Text
    fn get_text(&self, state: &mut State) -> String;
    fn get_font(&self, state: &mut State) -> String;
}

impl PropGet for Entity {
    fn is_disabled(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClasses::DISABLED)
        } else {
            false
        }
    }
    fn is_hovered(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClasses::HOVER)
        } else {
            false
        }
    }
    fn is_selected(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClasses::SELECTED)
        } else {
            false
        }
    }
    fn is_checked(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClasses::CHECKED)
        } else {
            false
        }
    }
    fn is_over(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClasses::OVER)
        } else {
            false
        }
    }
    fn is_active(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClasses::ACTIVE)
        } else {
            false
        }
    }
    fn is_focused(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.contains(PseudoClasses::FOCUS)
        } else {
            false
        }
    }

    fn get_overflow(&self, state: &mut State) -> Overflow {
        state.style.overflow.get(*self).cloned().unwrap_or_default()
    }

    // Display
    fn get_display(&self, state: &mut State) -> Display {
        state.style.display.get(*self).cloned().unwrap_or_default()
    }

    // Position
    fn get_left(&self, state: &mut State) -> Units {
        state.style.left.get(*self).cloned().unwrap_or_default()
    }
    fn get_right(&self, state: &mut State) -> Units {
        state.style.right.get(*self).cloned().unwrap_or_default()
    }
    fn get_top(&self, state: &mut State) -> Units {
        state.style.top.get(*self).cloned().unwrap_or_default()
    }
    fn get_bottom(&self, state: &mut State) -> Units {
        state.style.bottom.get(*self).cloned().unwrap_or_default()
    }

    // Size
    fn get_width(&self, state: &mut State) -> Units {
        state.style.width.get(*self).cloned().unwrap_or_default()
    }

    fn get_height(&self, state: &mut State) -> Units {
        state.style.height.get(*self).cloned().unwrap_or_default()
    }

    // Size Constraints
    fn get_min_width(&self, state: &mut State) -> Units {
        state
            .style
            .min_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_max_width(&self, state: &mut State) -> Units {
        state
            .style
            .max_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_min_height(&self, state: &mut State) -> Units {
        state
            .style
            .min_height
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_max_height(&self, state: &mut State) -> Units {
        state
            .style
            .max_height
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Border
    fn get_border_width(&self, state: &mut State) -> Units {
        state
            .style
            .border_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Tooltip
    fn get_tooltip(&self, state: &mut State) -> String {
        state.style.tooltip.get(*self).cloned().unwrap_or_default()
    }

    // Text
    fn get_text(&self, state: &mut State) -> String {
        state.style.text.get(*self).cloned().unwrap_or_default()
    }

    fn get_font(&self, state: &mut State) -> String {
        state.style.font.get(*self).cloned().unwrap_or_default()
    }
}

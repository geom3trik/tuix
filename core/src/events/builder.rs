use crate::state::*;
use crate::{Entity, EventHandler, State, Widget};

use std::{cell::RefCell, marker::PhantomData};
use std::rc::Rc;

/// Contains an entity id and a mutable reference to state and can be used to set properties of a widget at build time
pub struct Builder<'a,T> {
    pub entity: Entity,
    pub state: &'a mut State,
    phantom: std::marker::PhantomData<T>,
}

impl<'a,T> Builder<'a,T> {

    /// Creates a new Builder
    pub(crate) fn new(state: &'a mut State, entity: Entity) -> Self {
        Builder::<T> { entity, state, phantom: PhantomData}
    }

    /// Builds the widget into State
    pub(crate) fn build(mut self, event_handler: T) -> Entity
    where
        T: EventHandler + 'static + Sized,
    {
        self.state
            .event_handlers
            .insert(self.entity, Box::new(event_handler));

        self.entity
    }

    /// Returns a mutable reference to State.
    pub fn state(&mut self) -> &mut State {
        self.state
    }

    /// Returns the entity id contained within the builder.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Adds a class name to the widget.
    ///
    /// # Example
    /// Adds the class name `foo` to the widget.
    /// ```
    /// Element::new().build(state, parent, |builder| builder.class("foo"));
    pub fn class(mut self, class_name: &str) -> Self {
        //self.state.style.insert_class(self.entity, class);
        self.entity.class(self.state, class_name);

        self
    }

    // TODO
    pub fn set_name(mut self, name: &str) -> Self {
        self.state.style.name.insert(self.entity(), name.to_string());
    
        self
    }

    /// Sets the element name of the widget.
    ///
    pub fn set_element(mut self, element: &str) -> Self {
        //self.state.style.insert_element(self.entity, element);

        self.entity.set_element(self.state, element);

        self
    }

    /// Sets the id of the entity
    pub fn set_id(mut self, id: &str) -> Self {
        //self.state.style.insert_id(self.entity, id);

        todo!();
        self
    }

    pub fn set_disabled(mut self, value: bool) -> Self {
        self.entity().set_disabled(self.state, value);

        self
    }


    /// Sets whether the entity can be hovered
    pub fn set_hoverable(mut self, value: bool) -> Self {
        self.state.data.set_hoverable(self.entity, value);

        self
    }

    /// Sets whether the entity can be focused
    pub fn set_focusable(mut self, value: bool) -> Self {
        self.state.data.set_focusable(self.entity, value);

        self
    }

    /// Sets the opacity of the entity
    pub fn set_opacity(mut self, value: f32) -> Self {
        self.state.style.opacity.insert(self.entity, Opacity(value));

        self
    }

    /// Sets the checked state of the entity
    pub fn set_checked(mut self, value: bool) -> Self {
        self.entity().set_checked(self.state, value);

        self
    }

    /// Sets the z-order of the entity
    pub fn set_z_order(mut self, value: i32) -> Self {
        self.state.style.z_order.insert(self.entity, value);

        self
    }

    /// Sets the clip widget of the entity. The clip bounds of the entity are set to the bounds of the clip widget
    pub fn set_clip_widget(mut self, value: Entity) -> Self {
        self.state.style.clip_widget.insert(self.entity, value);

        self
    }

    // Sets the text displayed within the entity
    pub fn set_text(mut self, value: &str) -> Self {
        self.state.style.text.insert(self.entity, value.to_owned());

        self
    }

    pub fn set_font(mut self, value: &str) -> Self {
        self.state.style.font.insert(self.entity, value.to_owned());

        self
    }

    // Sets the tooltip associated with the entity
    pub fn set_tooltip(mut self, value: &str) -> Self {
        self.state
            .style
            .tooltip
            .insert(self.entity, value.to_string());

        self
    }

    // pub fn set_tooltip<F>(mut self, value: &str) -> Self 
    // where F: FnOnce(&mut State, Entity)
    // {
    //     self.state
    //         .style
    //         .tooltip
    //         .insert(self.entity, value.to_string());

    //     self
    // }

    // Display
    /// Sets the display type of the entity
    pub fn set_display(mut self, value: Display) -> Self {
        self.state.style.display.insert(self.entity, value);

        self
    }

    pub fn set_visibility(mut self, value: Visibility) -> Self {
        self.state.style.visibility.insert(self.entity, value);

        self
    }

    pub fn set_overflow(mut self, value: Overflow) -> Self {
        self.state.style.overflow.insert(self.entity, value);

        self
    }

    // Background
    pub fn set_background_color(mut self, value: Color) -> Self {
        self.state.style.background_color.insert(self.entity, value);

        self
    }

    pub fn set_background_image(mut self, value: Rc<()>) -> Self {
        self.state
            .style
            .background_image
            .insert(self.entity, value.clone());

        self
    }

    pub fn set_background_gradient(mut self, value: LinearGradient) -> Self {
        self.state
            .style
            .background_gradient
            .insert(self.entity, value);

        self
    }

    // Outer Shadow
    pub fn set_outer_shadow_h_offset(mut self, value: Units) -> Self {
        self.state
            .style
            .outer_shadow_h_offset
            .insert(self.entity, value);

        self
    }

    pub fn set_outer_shadow_v_offset(mut self, value: Units) -> Self {
        self.state
            .style
            .outer_shadow_v_offset
            .insert(self.entity, value);

        self
    }

    pub fn set_outer_shadow_color(mut self, value: Color) -> Self {
        self.state.style.outer_shadow_color.insert(self.entity, value);

        self
    }

    pub fn set_outer_shadow_blur(mut self, value: Units) -> Self {
        self.state.style.outer_shadow_blur.insert(self.entity, value);

        self
    }

    // Inner Shadow
    pub fn set_inner_shadow_h_offset(mut self, value: Units) -> Self {
        self.state
            .style
            .inner_shadow_h_offset
            .insert(self.entity, value);

        self
    }

    pub fn set_inner_shadow_v_offset(mut self, value: Units) -> Self {
        self.state
            .style
            .inner_shadow_v_offset
            .insert(self.entity, value);

        self
    }

    pub fn set_inner_shadow_color(mut self, value: Color) -> Self {
        self.state.style.inner_shadow_color.insert(self.entity, value);

        self
    }

    pub fn set_inner_shadow_blur(mut self, value: Units) -> Self {
        self.state.style.inner_shadow_blur.insert(self.entity, value);

        self
    }

    // Positioning

    pub fn set_space(mut self, value: Units) -> Self {
        self.state.style.left.insert(self.entity, value);
        self.state.style.right.insert(self.entity, value);
        self.state.style.top.insert(self.entity, value);
        self.state.style.bottom.insert(self.entity, value);

        self
    }

    pub fn set_left(mut self, value: Units) -> Self {
        self.entity.set_left(self.state, value);

        self
    }

    pub fn set_right(mut self, value: Units) -> Self {
        self.entity.set_right(self.state, value);

        self
    }

    pub fn set_top(mut self, value: Units) -> Self {
        self.entity.set_top(self.state, value);
        
        self
    }

    pub fn set_bottom(mut self, value: Units) -> Self {
        self.entity.set_bottom(self.state, value);
        
        self
    }

    // Size

    pub fn set_width(mut self, value: Units) -> Self {
        self.entity.set_width(self.state, value);

        self
    }

    pub fn set_height(mut self, value: Units) -> Self {
        self.entity.set_height(self.state, value);

        self
    }

    // Size Constraints

    pub fn set_min_width(mut self, value: Units) -> Self {
        self.state.style.min_width.insert(self.entity, value);

        self
    }

    pub fn set_max_width(mut self, value: Units) -> Self {
        self.state.style.max_width.insert(self.entity, value);

        self
    }

    pub fn set_min_height(mut self, value: Units) -> Self {
        self.state.style.min_height.insert(self.entity, value);

        self
    }

    pub fn set_max_height(mut self, value: Units) -> Self {
        self.state.style.max_height.insert(self.entity, value);

        self
    }

    pub fn set_min_left(mut self, value: Units) -> Self {
        self.state.style.min_left.insert(self.entity, value);
        self
    }

    pub fn set_min_right(mut self, value: Units) -> Self {
        self.state.style.min_right.insert(self.entity, value);
        self
    }

    pub fn set_min_top(mut self, value: Units) -> Self {
        self.state.style.min_top.insert(self.entity, value);
        self
    }

    pub fn set_min_bottom(mut self, value: Units) -> Self {
        self.state.style.min_bottom.insert(self.entity, value);
        self
    }

    pub fn set_max_left(mut self, value: Units) -> Self {
        self.state.style.max_left.insert(self.entity, value);
        self
    }

    pub fn set_max_right(mut self, value: Units) -> Self {
        self.state.style.max_right.insert(self.entity, value);
        self
    }

    pub fn set_max_top(mut self, value: Units) -> Self {
        self.state.style.max_top.insert(self.entity, value);
        self
    }

    pub fn set_max_bottom(mut self, value: Units) -> Self {
        self.state.style.max_bottom.insert(self.entity, value);
        self
    }

    // Border

    pub fn set_border_color(mut self, value: Color) -> Self {
        self.state.style.border_color.insert(self.entity, value);

        self
    }

    pub fn set_border_width(mut self, value: Units) -> Self {
        self.state.style.border_width.insert(self.entity, value);

        self
    }

    pub fn set_border_corner_shape(mut self, value: BorderCornerShape) -> Self {
        self.state.style.border_shape_top_left.insert(self.entity, value);
        self.state.style.border_shape_top_right.insert(self.entity, value);
        self.state.style.border_shape_bottom_left.insert(self.entity, value);
        self.state.style.border_shape_bottom_right.insert(self.entity, value);

        self
    }

    pub fn set_border_top_left_shape(mut self, value: BorderCornerShape) -> Self {
        self.state.style.border_shape_top_left.insert(self.entity, value);

        self
    }

    pub fn set_border_top_right_shape(mut self, value: BorderCornerShape) -> Self {
        self.state.style.border_shape_top_right.insert(self.entity, value);

        self
    }

    pub fn set_border_bottom_left_shape(mut self, value: BorderCornerShape) -> Self {
        self.state.style.border_shape_bottom_left.insert(self.entity, value);

        self
    }

    pub fn set_border_bottom_right_shape(mut self, value: BorderCornerShape) -> Self {
        self.state.style.border_shape_bottom_right.insert(self.entity, value);

        self
    }

    pub fn set_border_radius(mut self, value: Units) -> Self {
        self.state
            .style
            .border_radius_top_left
            .insert(self.entity, value);
        self.state
            .style
            .border_radius_top_right
            .insert(self.entity, value);
        self.state
            .style
            .border_radius_bottom_left
            .insert(self.entity, value);
        self.state
            .style
            .border_radius_bottom_right
            .insert(self.entity, value);

        self
    }

    pub fn set_border_radius_top_left(mut self, value: Units) -> Self {
        self.state
            .style
            .border_radius_top_left
            .insert(self.entity, value);

        self
    }

    pub fn set_border_radius_top_right(mut self, value: Units) -> Self {
        self.state
            .style
            .border_radius_top_right
            .insert(self.entity, value);

        self
    }

    pub fn set_border_radius_bottom_left(mut self, value: Units) -> Self {
        self.state
            .style
            .border_radius_bottom_left
            .insert(self.entity, value);

        self
    }

    pub fn set_border_radius_bottom_right(mut self, value: Units) -> Self {
        self.state
            .style
            .border_radius_bottom_right
            .insert(self.entity, value);

        self
    }

    pub fn set_color(mut self, value: Color) -> Self {
        self.state.style.font_color.insert(self.entity, value);

        self
    }

    pub fn set_font_size(mut self, value: f32) -> Self {
        self.state.style.font_size.insert(self.entity, value);

        self
    }

    pub fn set_next_focus(mut self, value: Entity) -> Self {
        if let Some(entity) = self.state.style.focus_order.get_mut(self.entity) {
            entity.next = value;
        } else {
            self.state.style.focus_order.insert(
                self.entity,
                FocusOrder {
                    next: value,
                    ..Default::default()
                },
            );
        }

        self
    }

    pub fn set_prev_focus(mut self, value: Entity) -> Self {
        if let Some(entity) = self.state.style.focus_order.get_mut(self.entity) {
            entity.prev = value;
        } else {
            self.state.style.focus_order.insert(
                self.entity,
                FocusOrder {
                    prev: value,
                    ..Default::default()
                },
            );
        }

        self
    }

    pub fn set_focus(mut self, next: Entity, prev: Entity) -> Self {
        if let Some(entity) = self.state.style.focus_order.get_mut(self.entity) {
            entity.next = next;
            entity.prev = prev;
        } else {
            self.state
                .style
                .focus_order
                .insert(self.entity, FocusOrder { next, prev });
        }

        self
    }

    pub fn set_rotate(mut self, rotate: f32) -> Self {
        self.state.style.rotate.insert(self.entity, rotate);

        self
    }

    pub fn set_scale(mut self, scale: f32) -> Self {
        self.state.style.scale.insert(self.entity, scale);

        self
    }

    pub fn set_child_left(mut self, value: Units) -> Self {
        self.state.style.child_left.insert(self.entity, value);
        self
    }

    pub fn set_child_right(mut self, value: Units) -> Self {
        self.state.style.child_right.insert(self.entity, value);
        self
    }

    pub fn set_child_top(mut self, value: Units) -> Self {
        self.state.style.child_top.insert(self.entity, value);
        self
    }

    pub fn set_child_bottom(mut self, value: Units) -> Self {
        self.state.style.child_bottom.insert(self.entity, value);
        self
    }

    pub fn set_row_between(mut self, value: Units) -> Self {
        self.state.style.row_between.insert(self.entity, value);
        self
    }


    pub fn set_col_between(mut self, value: Units) -> Self {
        self.state.style.col_between.insert(self.entity, value);
        self
    }


    pub fn set_child_space(mut self, value: Units) -> Self {
        self.state.style.child_left.insert(self.entity, value);
        self.state.style.child_right.insert(self.entity, value);
        self.state.style.child_top.insert(self.entity, value);
        self.state.style.child_bottom.insert(self.entity, value);
        self
    }

    pub fn set_position_type(mut self, value: PositionType) -> Self {
        self.state.style.positioning_type.insert(self.entity, value);
        self
    }

    pub fn set_layout_type(mut self, value: LayoutType) -> Self {
        self.state.style.layout_type.insert(self.entity, value);
        self
    }

    pub fn set_row_index(mut self, value: usize) -> Self {
        self.state.style.row_index.insert(self.entity, value);

        self
    }

    pub fn set_col_index(mut self, value: usize) -> Self {
        self.state.style.col_index.insert(self.entity, value);

        self
    }

    pub fn set_row_span(mut self, value: usize) -> Self {
        self.state.style.row_span.insert(self.entity, value);

        self
    }

    pub fn set_col_span(mut self, value: usize) -> Self {
        self.state.style.col_span.insert(self.entity, value);

        self
    }

    pub fn set_grid_rows(mut self, value: Vec<Units>) -> Self {
        self.state.style.grid_rows.insert(self.entity(), value);

        self
    }

    pub fn set_grid_cols(mut self, value: Vec<Units>) -> Self {
        self.state.style.grid_cols.insert(self.entity(), value);

        self
    }
}
 
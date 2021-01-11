use crate::{Entity, State};

use crate::hierarchy::*;
use crate::style::*;

use crate::flexbox::AlignItems;

pub fn apply_z_ordering(state: &mut State, hierarchy: &Hierarchy) {
    for entity in hierarchy.into_iter() {
        if entity == Entity::new(0, 0) {
            continue;
        }

        let parent = hierarchy.get_parent(entity).unwrap();

        if let Some(z_order) = state.style.z_order.get(entity) {
            state.transform.set_z_order(entity, *z_order);
        } else {
            let parent_z_order = state.transform.get_z_order(parent);
            state.transform.set_z_order(entity, parent_z_order);
        }
    }
}


/* Depreciated
pub fn layout_fun(state: &mut State, hierarchy: &Hierarchy) {
    // Reset
    for entity in hierarchy.entities.iter() {
        state.transform.set_child_sum(*entity, 0.0);
        state.transform.set_child_pos(*entity, 0.0);
        state.transform.set_child_grow_sum(*entity, 0.0);
        state.transform.set_child_shrink_sum(*entity, 0.0);
    }

    //////////////////////
    // Walk up the tree //
    //////////////////////
    for entity in hierarchy.entities.iter().rev() {
        
        // Stop before the window
        if *entity == Entity::new(0, 0) {
            break;
        }

        // Skip non-displayed widgets
        let display = state
            .style
            .display
            .get(*entity)
            .cloned()
            .unwrap_or_default();
        if display == Display::None {
            continue;
        }

        let parent = hierarchy.get_parent(*entity).unwrap();

        let parent_width = state.transform.get_width(parent);
        let parent_height = state.transform.get_height(parent);

        let child_min_width = match state
            .style
            .min_width
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_max_width = match state
            .style
            .max_width
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => std::f32::INFINITY,
        };

        let child_min_height = match state
            .style
            .min_height
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_max_height = match state
            .style
            .max_height
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => std::f32::INFINITY,
        };

        let child_margin_left = match state
            .style
            .margin_left
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_margin_right = match state
            .style
            .margin_right
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_margin_top = match state
            .style
            .margin_top
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_margin_bottom = match state
            .style
            .margin_bottom
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_padding_left = match state
            .style
            .padding_left
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_padding_right = match state
            .style
            .padding_right
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_padding_top = match state
            .style
            .padding_top
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_padding_bottom = match state
            .style
            .padding_bottom
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_border_width = match state
            .style
            .border_width
            .get(*entity)
            .cloned()
            .unwrap_or_default() 
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_border_width = match state
            .style
            .border_width
            .get(parent)
            .cloned()
            .unwrap_or_default() 
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_left = match state
            .style
            .padding_left
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_right = match state
            .style
            .padding_right
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_top = match state
            .style
            .padding_top
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_padding_bottom = match state
            .style
            .padding_bottom
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_flex_direction = state
            .style
            .flex_direction
            .get(parent)
            .cloned()
            .unwrap_or_default();
        let child_flex_direction = state
            .style
            .flex_direction
            .get(*entity)
            .cloned()
            .unwrap_or_default();

        // Get the desired width from the style
        let width = state.style.width.get(*entity).cloned().unwrap_or_default();

        // Get the desired height from the style
        let height = state.style.height.get(*entity).cloned().unwrap_or_default();

        let mut new_width;
        let mut new_height;

        match child_flex_direction {
            FlexDirection::Row => {
                // Set width to the sum of the widths of the children
                new_width = state.transform.get_child_sum(*entity);
                // Set height to the maximum height of the children
                new_height = state.transform.get_child_max(*entity);
            }

            FlexDirection::Column => {
                // Set width to the maximum width of the children
                new_width = state.transform.get_child_max(*entity);
                // Set height to the maximum height of the children
                new_height = state.transform.get_child_sum(*entity);
            }
        }

        match parent_flex_direction {
            FlexDirection::Row => {
                // Start with desired width if specified in pixels
                match width {
                    Length::Pixels(val) => {
                        new_width = val;
                    }

                    Length::Percentage(val) => {
                        new_width = (parent_width
                            - parent_padding_left
                            - parent_padding_right
                            - 2.0 * parent_border_width)
                            * val;
                    }

                    _ => {}
                };

                // Flex basis overrides desired width
                if let Some(flex_basis) = state.style.flex_basis.get(*entity) {
                    new_width = *flex_basis;
                }

                // Set height to desired height if specified in pixels
                match height {
                    Length::Pixels(val) => {
                        new_height = val;
                    }

                    _ => {}
                }

                // Apply size constraints
                if new_width < child_min_width {
                    new_width = child_min_width;
                }

                if new_width > child_max_width {
                    new_width = child_max_width;
                }

                if new_height < child_min_height {
                    new_height = child_min_height;
                }

                if new_height > child_max_height {
                    new_height = child_max_height;
                }

                // Apply margins, padding, and border
                new_width += child_margin_left
                    + child_margin_right
                    + child_padding_left
                    + child_padding_right
                    + 2.0 * child_border_width;
                new_height += child_margin_top
                    + child_margin_bottom
                    + child_padding_top
                    + child_padding_bottom
                    + 2.0 * child_border_width;

                let position = state
                    .style
                    .position
                    .get(*entity)
                    .cloned()
                    .unwrap_or_default();

                match position {
                    Position::Relative => {
                        state.transform.set_child_sum(
                            parent,
                            state.transform.get_child_sum(parent) + new_width,
                        );
                        //state.transform.set_child_max(parent, new_height);
                        state.transform.set_child_max(
                            parent,
                            new_height.max(state.transform.get_child_max(parent)),
                        );
                    }

                    _ => {}
                }
            }

            FlexDirection::Column => {
                match height {
                    Length::Pixels(val) => {
                        new_height = val;
                    }

                    Length::Percentage(val) => {
                        new_height = (parent_height
                            - parent_padding_top
                            - parent_padding_bottom
                            - 2.0 * parent_border_width)
                            * val;
                    }
                    _ => {}
                };

                if let Some(flex_basis) = state.style.flex_basis.get(*entity) {
                    new_height = *flex_basis;
                }

                match width {
                    Length::Pixels(val) => {
                        new_width = val;
                    }

                    _ => {}
                }

                if new_width < child_min_width {
                    new_width = child_min_width;
                }

                if new_width > child_max_width {
                    new_width = child_max_width;
                }

                if new_height < child_min_height {
                    new_height = child_min_height;
                }

                if new_height > child_max_height {
                    new_height = child_max_height;
                }

                new_width += child_margin_left
                    + child_margin_right
                    + child_padding_left
                    + child_padding_right
                    + 2.0 * child_border_width;
                new_height += child_margin_top
                    + child_margin_bottom
                    + child_padding_top
                    + child_padding_bottom
                    + 2.0 * child_border_width;

                let position = state
                    .style
                    .position
                    .get(*entity)
                    .cloned()
                    .unwrap_or_default();

                match position {
                    Position::Relative => {
                        state.transform.set_child_sum(
                            parent,
                            state.transform.get_child_sum(parent) + new_height,
                        );
                        //state.transform.set_child_max(parent, new_width);
                        state.transform.set_child_max(
                            parent,
                            new_width.max(state.transform.get_child_max(parent)),
                        );
                    }

                    _ => {}
                }
            }
        }

        if let Some(flex_grow) = state.style.flex_grow.get(*entity) {
            state.transform.set_child_grow_sum(
                parent,
                state.transform.get_child_grow_sum(parent) + flex_grow,
            );
        }

        if let Some(flex_shrink) = state.style.flex_shrink.get(*entity) {
            state.transform.set_child_shrink_sum(
                parent,
                state.transform.get_child_shrink_sum(parent) + flex_shrink,
            );
        }
    }

    ////////////////////////
    // Walk down the tree //
    ////////////////////////
    for parent in hierarchy.into_iter() {
        // Parent properties


        // If all of the child widgets have not changed size then we can break out of the loop
        let mut should_continue = false;

        let parent_width = state.transform.get_width(parent);
        let parent_height = state.transform.get_height(parent);

        let parent_border_width = match state
            .style
            .border_width
            .get(parent)
            .cloned()
            .unwrap_or_default() 
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_left = match state
            .style
            .padding_left
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_right = match state
            .style
            .padding_right
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_top = match state
            .style
            .padding_top
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_padding_bottom = match state
            .style
            .padding_bottom
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_posx =
            state.transform.get_posx(parent) + parent_padding_left + parent_border_width;
        let parent_posy =
            state.transform.get_posy(parent) + parent_padding_top + parent_border_width;

        //TEMP - Move to the walk up phase
        let mut num_children = 0;
        for _ in parent.child_iter(&hierarchy) {
            num_children += 1;
        }

        let parent_flex_direction = state
            .style
            .flex_direction
            .get(parent)
            .cloned()
            .unwrap_or_default();

        let mut current_pos = 0.0;
        let mut space_per_widget = 0.0;

        let free_space = match parent_flex_direction {
            FlexDirection::Row => {
                state.transform.get_width(parent)
                    - parent_padding_left
                    - parent_padding_right
                    - 2.0 * parent_border_width
                    - state.transform.get_child_sum(parent)
            }

            FlexDirection::Column => {
                state.transform.get_height(parent)
                    - parent_padding_top
                    - parent_padding_bottom
                    - 2.0 * parent_border_width
                    - state.transform.get_child_sum(parent)
            }
        };

        let justify_content = state
            .style
            .justify_content
            .get(parent)
            .cloned()
            .unwrap_or_default();


        match justify_content {
            JustifyContent::FlexStart => {}
            JustifyContent::FlexEnd => current_pos = free_space,
            JustifyContent::Center => current_pos = (free_space) / 2.0,
            JustifyContent::SpaceBetween => {
                space_per_widget = free_space / (num_children - 1) as f32;
            }
            JustifyContent::SpaceAround => {
                space_per_widget = free_space / num_children as f32;
                current_pos = space_per_widget / 2.0;
            }
            JustifyContent::SpaceEvenly => {
                space_per_widget = free_space / (num_children + 1) as f32;
                current_pos = space_per_widget;
            }
            _ => {}
        }

        for child in parent.child_iter(&hierarchy) {
            // Skip non-displayed widgets
            let display = state.style.display.get(child).cloned().unwrap_or_default();

            if display == Display::None {
                continue;
            }

            // Get the desired width and height
            let width = state.style.width.get(child).cloned().unwrap_or_default();
            let height = state.style.height.get(child).cloned().unwrap_or_default();

            let left = state.style.left.get(child).cloned().unwrap_or_default();
            let top = state.style.top.get(child).cloned().unwrap_or_default();

            let child_border_width = match state
                .style
                .border_width
                .get(child)
                .cloned()
                .unwrap_or_default() 
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_min_width = match state
                .style
                .min_width
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => 0.0,
            };

            let child_max_width = match state
                .style
                .max_width
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => std::f32::INFINITY,
            };

            let child_min_height = match state
                .style
                .min_height
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => 0.0,
            };

            let child_max_height = match state
                .style
                .max_height
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => std::f32::INFINITY,
            };

            let child_margin_left = match state
                .style
                .margin_left
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_margin_right = match state
                .style
                .margin_right
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_margin_top = match state
                .style
                .margin_top
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_height * val,
                _ => 0.0,
            };

            let child_margin_bottom = match state
                .style
                .margin_bottom
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_height * val,
                _ => 0.0,
            };

            let child_padding_left = match state
                .style
                .padding_left
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_padding_right = match state
                .style
                .padding_right
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_padding_top = match state
                .style
                .padding_top
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_height * val,
                _ => 0.0,
            };

            let child_padding_bottom = match state
                .style
                .padding_bottom
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_height * val,
                _ => 0.0,
            };

            // The new size and position of the child
            let mut new_width;
            let mut new_height;
            let mut new_posx;
            let mut new_posy;

            let flex_direction = state
                .style
                .flex_direction
                .get(child)
                .cloned()
                .unwrap_or_default();

            let child_flex_grow = state
                .style
                .flex_grow
                .get(child)
                .cloned()
                .unwrap_or_default();

            let child_flex_shrink = state
                .style
                .flex_shrink
                .get(child)
                .cloned()
                .unwrap_or_default();

            let mut child_grow_sum = state.transform.get_child_grow_sum(parent);
            let mut child_shrink_sum = state.transform.get_child_shrink_sum(parent);

            if child_grow_sum < 1.0 {
                child_grow_sum = 1.0;
            }

            if child_shrink_sum < 1.0 {
                child_shrink_sum = 1.0;
            }

            let flex_grow_fraction = child_flex_grow / child_grow_sum;
            let flex_shrink_fraction = child_flex_shrink / child_shrink_sum;

            let position = state.style.position.get(child).cloned().unwrap_or_default();

            match flex_direction {
                FlexDirection::Row => {
                    new_width = state.transform.get_child_sum(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.transform.get_child_max(child)
                        + child_padding_top
                        + child_padding_bottom
                        + 2.0 * child_border_width;
                }

                FlexDirection::Column => {
                    new_width = state.transform.get_child_max(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.transform.get_child_sum(child)
                        + child_padding_top
                        + child_padding_bottom
                        + 2.0 * child_border_width;
                }
            }

            match position {
                Position::Relative => {
                    match parent_flex_direction {
                        FlexDirection::Row => {
                            new_posx = current_pos;

                            match width {
                                Length::Pixels(val) => {
                                    new_width = val
                                        + child_padding_left
                                        + child_padding_right
                                        + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_width = (parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - parent_border_width
                                        - parent_border_width)
                                        * val;
                                    new_width += child_padding_left
                                        + child_padding_right
                                        + 2.0 * child_border_width;
                                }
                                _ => {}
                            };

                            if let Some(flex_basis) = state.style.flex_basis.get(child) {
                                new_width = *flex_basis + child_padding_left + child_padding_right + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_width
                                - parent_padding_left
                                - parent_padding_right
                                - parent_border_width
                                - parent_border_width
                                - state.transform.get_child_sum(parent);


                            if parent_free_space >= 0.0 {
                                new_width += flex_grow_fraction * parent_free_space;
                            } else {
                                new_width += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(parent)
                                .cloned()
                                .unwrap_or_default();

                            if let Some(align_self) = state.style.align_self.get(child) {
                                if *align_self == AlignSelf::Stretch {
                                    new_height = parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - parent_border_width
                                        - parent_border_width
                                        - child_margin_top
                                        - child_margin_bottom;
                                }
                            } else {
                                if align_items == AlignItems::Stretch {
                                    new_height = parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - parent_border_width
                                        - parent_border_width
                                        - child_margin_top
                                        - child_margin_bottom;
                                }
                            }

                            match height {
                                Length::Pixels(val) => {
                                    new_height = val
                                        + child_padding_top
                                        + child_padding_bottom
                                        + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_height = (parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - 2.0 * parent_border_width)
                                        * val;
                                    new_height += child_padding_top
                                        + child_padding_bottom
                                        + 2.0 * child_border_width;
                                }

                                _ => {}
                            };

                            // Apply size contraints
                            if new_width < child_min_width {
                                new_width = child_min_width;
                            }

                            if new_width > child_max_width {
                                new_width = child_max_width;
                            }

                            if new_height < child_min_height {
                                new_height = child_min_height;
                            }

                            if new_height > child_max_height {
                                new_height = child_max_height;
                            }

                            match left {
                                Length::Pixels(val) => {
                                    new_posx = current_pos + val;
                                }

                                Length::Percentage(val) => {
                                    new_posx = current_pos + val * parent_width;
                                }

                                _ => {}
                            }

                            new_posx = parent_posx + new_posx + child_margin_left;

                            new_posy = match align_items {
                                AlignItems::FlexStart => 0.0,
                                AlignItems::FlexEnd => {
                                    parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - 2.0 * parent_border_width
                                        - new_height
                                }
                                AlignItems::Center => {
                                    (parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - 2.0 * parent_border_width
                                        - new_height
                                        - child_margin_top
                                        - child_margin_bottom)
                                        / 2.0
                                }
                                AlignItems::Stretch => 0.0,
                                //AlignItems::Baseline => 0.0, //TODO
                            };

                            // align-self overrides align-items
                            if let Some(align_self) = state.style.align_self.get(child) {
                                match align_self {
                                    AlignSelf::FlexStart => new_posy = 0.0,
                                    AlignSelf::FlexEnd => new_posy = parent_height - new_height,
                                    AlignSelf::Center => {
                                        new_posy = (parent_height - new_height) / 2.0
                                    }
                                    AlignSelf::Stretch => new_posy = 0.0,
                                }
                            }

                            match top {
                                Length::Pixels(val) => {
                                    new_posy += val;
                                }

                                Length::Percentage(val) => {
                                    new_posy += val * parent_height;
                                }

                                _ => {}
                            }


                            new_posy = parent_posy + new_posy + child_margin_top;

                            // state.transform.set_posy(
                            //     child,
                            //     parent_posy + new_posy + child_margin_top, // + (child_border_width / 2.0),
                            // );

                            current_pos += new_width
                                + space_per_widget
                                + child_margin_left
                                + child_margin_right;
                        }

                        FlexDirection::Column => {
                            new_posy = current_pos;

                            match height {
                                Length::Pixels(val) => {
                                    new_height = val
                                        + child_padding_top
                                        + child_padding_bottom
                                        + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_height = (parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - parent_border_width
                                        - parent_border_width)
                                        * val;
                                    new_height += child_padding_top
                                        + child_padding_bottom
                                        + 2.0 * child_border_width;
                                }
                                _ => {}
                            };

                            if let Some(flex_basis) = state.style.flex_basis.get(child) {
                                new_height = *flex_basis + child_padding_top + child_padding_bottom + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_height
                                - parent_padding_top
                                - parent_padding_bottom
                                - parent_border_width
                                - parent_border_width
                                - state.transform.get_child_sum(parent);

                            if parent_free_space >= 0.0 {
                                new_height += flex_grow_fraction * parent_free_space;
                            } else {
                                new_height += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(parent)
                                .cloned()
                                .unwrap_or_default();

                            if let Some(align_self) = state.style.align_self.get(child) {
                                if *align_self == AlignSelf::Stretch {
                                    new_width = parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - parent_border_width
                                        - parent_border_width
                                        - child_margin_left
                                        - child_margin_right;
                                }
                            } else {
                                if align_items == AlignItems::Stretch {
                                    new_width = parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - parent_border_width
                                        - parent_border_width
                                        - child_margin_left
                                        - child_margin_right;
                                }
                            }

                            match width {
                                Length::Pixels(val) => {
                                    new_width = val
                                        + child_padding_left
                                        + child_padding_right
                                        + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_width = (parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - 2.0 * parent_border_width)
                                        * val;
                                    new_width += child_padding_left
                                        + child_padding_right
                                        + 2.0 * child_border_width;
                                }

                                _ => {}
                            };

                            // Apply size contraints
                            if new_width < child_min_width {
                                new_width = child_min_width;
                            }

                            if new_width > child_max_width {
                                new_width = child_max_width;
                            }

                            if new_height < child_min_height {
                                new_height = child_min_height;
                            }

                            if new_height > child_max_height {
                                new_height = child_max_height;
                            }

                            //state.transform.set_width(child, new_width);
                            //state.transform.set_height(child, new_height);

                            match top {
                                Length::Pixels(val) => {
                                    new_posy = current_pos + val;
                                }

                                Length::Percentage(val) => {
                                    new_posy = current_pos + val * parent_height;
                                }

                                _ => {}
                            }

                            
                            new_posy = parent_posy + new_posy + child_margin_top;
                            
                            // state
                            //     .transform
                            //     .set_posy(child, parent_posy + new_posy + child_margin_top);

                            //let align_items = state.style.align_items.get(parent).cloned().unwrap_or_default();

                            new_posx = match align_items {
                                AlignItems::FlexStart => 0.0,
                                AlignItems::FlexEnd => {
                                    parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - 2.0 * parent_border_width
                                        - new_width
                                }
                                AlignItems::Center => {
                                    (parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - 2.0 * parent_border_width
                                        - new_width
                                        - child_margin_left
                                        - child_margin_right)
                                        / 2.0
                                }
                                AlignItems::Stretch => 0.0,
                                //AlignItems::Baseline => 0.0, //TODO
                            };

                            // align-self overrides align-items
                            if let Some(align_self) = state.style.align_self.get(child) {
                                match align_self {
                                    AlignSelf::FlexStart => new_posx = 0.0,
                                    AlignSelf::FlexEnd => new_posx = parent_width - new_width,
                                    AlignSelf::Center => {
                                        new_posx = (parent_width - new_width) / 2.0
                                    }
                                    AlignSelf::Stretch => new_posx = 0.0,
                                }
                            }

                            match left {
                                Length::Pixels(val) => {
                                    new_posx += val;
                                }

                                Length::Percentage(val) => {
                                    new_posx += val * parent_width;
                                }

                                _ => {}
                            }

                            new_posx = parent_posx + new_posx + child_margin_left;

                            // state
                            //     .transform
                            //     .set_posx(child, parent_posx + new_posx + child_margin_left);

                            current_pos += new_height
                                + space_per_widget
                                + child_margin_top
                                + child_margin_bottom;
                        }
                    }
                }

                Position::Absolute => {
                    let width = state.style.width.get(child).cloned().unwrap_or_default();
                    let height = state.style.height.get(child).cloned().unwrap_or_default();

                    let left = state.style.left.get(child).cloned().unwrap_or_default();
                    let right = state.style.right.get(child).cloned().unwrap_or_default();
                    let top = state.style.top.get(child).cloned().unwrap_or_default();
                    let bottom = state.style.bottom.get(child).cloned().unwrap_or_default();

                    new_posx = parent_posx;
                    new_posy = parent_posy;

                    let r = match right {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => val * parent_width,
                        Length::Initial(val) => val,
                        Length::Auto => 0.0,
                    };

                    let l = match left {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => val * parent_width,
                        Length::Initial(val) => val,
                        Length::Auto => 0.0,
                    };

                    if !right.is_auto() && !left.is_auto() {
                        new_width = parent_width - l - r;
                    }

                    let b = match bottom {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => val * parent_height,
                        Length::Initial(val) => val,
                        Length::Auto => 0.0,
                    };

                    let t = match top {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => val * parent_height,
                        Length::Initial(val) => val,
                        Length::Auto => 0.0,
                    };

                    if !bottom.is_auto() && !top.is_auto() {
                        new_height = parent_height - t - b;
                    }

                    match width {
                        Length::Auto => {}
                        Length::Pixels(val) => new_width = val,
                        Length::Initial(val) => new_width = val,
                        Length::Percentage(val) => new_width = val * parent_width,
                    }

                    match height {
                        Length::Auto => {}
                        Length::Pixels(val) => new_height = val,
                        Length::Initial(val) => new_height = val,
                        Length::Percentage(val) => new_height = val * parent_height,
                    }

                    //state.transform.set_width(child, new_width);
                    //state.transform.set_height(child, new_height);

                    match right {
                        Length::Pixels(val) => {
                            new_posx = parent_posx + parent_width - new_width - val;
                        }

                        Length::Percentage(val) => {
                            new_posx = parent_posx + parent_width
                                - new_width
                                - (val
                                    * (parent_width - parent_padding_left - parent_padding_right));
                        }

                        _ => {}
                    }

                    match left {
                        Length::Pixels(val) => {
                            new_posx = parent_posx + val;
                        }

                        Length::Percentage(val) => {
                            new_posx = parent_posx
                                + (val
                                    * (parent_width - parent_padding_left - parent_padding_right));
                        }

                        _ => {}
                    }

                    match bottom {
                        Length::Pixels(val) => {
                            new_posy = parent_posy + parent_height - new_height - val;
                        }

                        Length::Percentage(val) => {
                            new_posy = parent_posy + parent_height
                                - new_height
                                - (val
                                    * (parent_height - parent_padding_top - parent_padding_bottom));
                        }

                        _ => {}
                    }

                    match top {
                        Length::Pixels(val) => {
                            new_posy = parent_posy + val;
                        }

                        Length::Percentage(val) => {
                            new_posy = parent_posy
                                + (val
                                    * (parent_height - parent_padding_top - parent_padding_bottom));
                        }

                        _ => {}
                    }

                    //state.transform.set_posx(child, new_posx);
                    //state.transform.set_posy(child, new_posy);
                }

                
            }



            
            state.transform.set_posx(child, new_posx);
            state.transform.set_posy(child, new_posy);
            state.transform.set_width(child, new_width);
            state.transform.set_height(child, new_height);



        }




        // Set the transform properties
    }
}
*/





pub fn apply_layout(state: &mut State, hierarchy: &Hierarchy) {
    
    // Reset
    for entity in hierarchy.entities.iter() {
        state.transform.set_child_sum(*entity, 0.0);
        state.transform.set_child_pos(*entity, 0.0);
        state.transform.set_child_grow_sum(*entity, 0.0);
        state.transform.set_child_shrink_sum(*entity, 0.0);
    }

    let mut hierarchy_up_iterator = hierarchy.entities.iter();

    //////////////////////
    // Walk up the tree //
    //////////////////////
    while let Some(entity) = hierarchy_up_iterator.next_back() {
        // Stop before the window
        if *entity == Entity::new(0, 0) {
            break;
        }

        // Skip non-displayed widgets
        let display = state
            .style
            .display
            .get(*entity)
            .cloned()
            .unwrap_or_default();
        if display == Display::None {
            continue;
        }

        let parent = hierarchy.get_parent(*entity).unwrap();

        let parent_width = state.transform.get_width(parent);
        let parent_height = state.transform.get_height(parent);

        let child_min_width = match state
            .style
            .min_width
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_max_width = match state
            .style
            .max_width
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => std::f32::INFINITY,
        };

        let child_min_height = match state
            .style
            .min_height
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_max_height = match state
            .style
            .max_height
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => std::f32::INFINITY,
        };

        let child_margin_left = match state
            .style
            .margin_left
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_margin_right = match state
            .style
            .margin_right
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_margin_top = match state
            .style
            .margin_top
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_margin_bottom = match state
            .style
            .margin_bottom
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_padding_left = match state
            .style
            .padding_left
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_padding_right = match state
            .style
            .padding_right
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let child_padding_top = match state
            .style
            .padding_top
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_padding_bottom = match state
            .style
            .padding_bottom
            .get(*entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let child_border_width = match state
            .style
            .border_width
            .get(*entity)
            .cloned()
            .unwrap_or_default() 
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_border_width = match state
            .style
            .border_width
            .get(parent)
            .cloned()
            .unwrap_or_default() 
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_left = match state
            .style
            .padding_left
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_right = match state
            .style
            .padding_right
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_top = match state
            .style
            .padding_top
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_padding_bottom = match state
            .style
            .padding_bottom
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_flex_direction = state
            .style
            .flex_direction
            .get(parent)
            .cloned()
            .unwrap_or_default();
        let child_flex_direction = state
            .style
            .flex_direction
            .get(*entity)
            .cloned()
            .unwrap_or_default();

        // Get the desired width from the style
        let width = state.style.width.get(*entity).cloned().unwrap_or_default();

        // Get the desired height from the style
        let height = state.style.height.get(*entity).cloned().unwrap_or_default();

        let mut new_width;
        let mut new_height;

        match child_flex_direction {
            FlexDirection::Row => {
                // Set width to the sum of the widths of the children
                new_width = state.transform.get_child_sum(*entity);
                // Set height to the maximum height of the children
                new_height = state.transform.get_child_max(*entity);
            }

            FlexDirection::Column => {
                // Set width to the maximum width of the children
                new_width = state.transform.get_child_max(*entity);
                // Set height to the maximum height of the children
                new_height = state.transform.get_child_sum(*entity);
            }
        }

        match parent_flex_direction {
            FlexDirection::Row => {
                // Start with desired width if specified in pixels
                match width {
                    Length::Pixels(val) => {
                        new_width = val;
                    }

                    Length::Percentage(val) => {
                        new_width = (parent_width
                            - parent_padding_left
                            - parent_padding_right
                            - 2.0 * parent_border_width)
                            * val;
                    }

                    _ => {}
                };

                // Flex basis overrides desired width
                if let Some(flex_basis) = state.style.flex_basis.get(*entity) {
                    new_width = *flex_basis;
                }

                // Set height to desired height if specified in pixels
                match height {
                    Length::Pixels(val) => {
                        new_height = val;
                    }

                    _ => {}
                }

                // Apply size constraints
                if new_width < child_min_width {
                    new_width = child_min_width;
                }

                if new_width > child_max_width {
                    new_width = child_max_width;
                }

                if new_height < child_min_height {
                    new_height = child_min_height;
                }

                if new_height > child_max_height {
                    new_height = child_max_height;
                }

                // Apply margins, padding, and border
                new_width += child_margin_left
                    + child_margin_right
                    + child_padding_left
                    + child_padding_right
                    + 2.0 * child_border_width;
                new_height += child_margin_top
                    + child_margin_bottom
                    + child_padding_top
                    + child_padding_bottom
                    + 2.0 * child_border_width;

                let position = state
                    .style
                    .position
                    .get(*entity)
                    .cloned()
                    .unwrap_or_default();

                match position {
                    Position::Relative => {
                        state.transform.set_child_sum(
                            parent,
                            state.transform.get_child_sum(parent) + new_width,
                        );
                        //state.transform.set_child_max(parent, new_height);
                        state.transform.set_child_max(
                            parent,
                            new_height.max(state.transform.get_child_max(parent)),
                        );
                    }

                    _ => {}
                }
            }

            FlexDirection::Column => {
                match height {
                    Length::Pixels(val) => {
                        new_height = val;
                    }

                    Length::Percentage(val) => {
                        new_height = (parent_height
                            - parent_padding_top
                            - parent_padding_bottom
                            - 2.0 * parent_border_width)
                            * val;
                    }
                    _ => {}
                };

                if let Some(flex_basis) = state.style.flex_basis.get(*entity) {
                    new_height = *flex_basis;
                }

                match width {
                    Length::Pixels(val) => {
                        new_width = val;
                    }

                    _ => {}
                }

                if new_width < child_min_width {
                    new_width = child_min_width;
                }

                if new_width > child_max_width {
                    new_width = child_max_width;
                }

                if new_height < child_min_height {
                    new_height = child_min_height;
                }

                if new_height > child_max_height {
                    new_height = child_max_height;
                }

                new_width += child_margin_left
                    + child_margin_right
                    + child_padding_left
                    + child_padding_right
                    + 2.0 * child_border_width;
                new_height += child_margin_top
                    + child_margin_bottom
                    + child_padding_top
                    + child_padding_bottom
                    + 2.0 * child_border_width;

                let position = state
                    .style
                    .position
                    .get(*entity)
                    .cloned()
                    .unwrap_or_default();

                match position {
                    Position::Relative => {
                        state.transform.set_child_sum(
                            parent,
                            state.transform.get_child_sum(parent) + new_height,
                        );
                        //state.transform.set_child_max(parent, new_width);
                        state.transform.set_child_max(
                            parent,
                            new_width.max(state.transform.get_child_max(parent)),
                        );
                    }

                    _ => {}
                }
            }
        }

        if let Some(flex_grow) = state.style.flex_grow.get(*entity) {
            state.transform.set_child_grow_sum(
                parent,
                state.transform.get_child_grow_sum(parent) + flex_grow,
            );
        }

        if let Some(flex_shrink) = state.style.flex_shrink.get(*entity) {
            state.transform.set_child_shrink_sum(
                parent,
                state.transform.get_child_shrink_sum(parent) + flex_shrink,
            );
        }
        
    }

    let mut hierarchy_down_iterator = state.root.into_iter(hierarchy);
    //let mut hierarchy_down_iterator = hierarchy.into_iter();

    let mut should_continue = false;
    let mut next_sibling = Entity::null();

    ////////////////////////
    // Walk down the tree //
    ////////////////////////
    while let Some(parent) = hierarchy_down_iterator.next() {
        // Parent properties

        let parent_width = state.transform.get_width(parent);
        let parent_height = state.transform.get_height(parent);

        let parent_border_width = match state
            .style
            .border_width
            .get(parent)
            .cloned()
            .unwrap_or_default() 
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_left = match state
            .style
            .padding_left
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_right = match state
            .style
            .padding_right
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let parent_padding_top = match state
            .style
            .padding_top
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_padding_bottom = match state
            .style
            .padding_bottom
            .get(parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_posx =
            state.transform.get_posx(parent) + parent_padding_left + parent_border_width;
        let parent_posy =
            state.transform.get_posy(parent) + parent_padding_top + parent_border_width;

        //TEMP - Move to the walk up phase
        let mut num_children = 0;
        for _ in parent.child_iter(&hierarchy) {
            num_children += 1;
        }

        let parent_flex_direction = state
            .style
            .flex_direction
            .get(parent)
            .cloned()
            .unwrap_or_default();

        let mut current_pos = 0.0;
        let mut space_per_widget = 0.0;

        let free_space = match parent_flex_direction {
            FlexDirection::Row => {
                state.transform.get_width(parent)
                    - parent_padding_left
                    - parent_padding_right
                    - 2.0 * parent_border_width
                    - state.transform.get_child_sum(parent)
            }

            FlexDirection::Column => {
                state.transform.get_height(parent)
                    - parent_padding_top
                    - parent_padding_bottom
                    - 2.0 * parent_border_width
                    - state.transform.get_child_sum(parent)
            }
        };

        let justify_content = state
            .style
            .justify_content
            .get(parent)
            .cloned()
            .unwrap_or_default();


        match justify_content {
            JustifyContent::FlexStart => {}
            JustifyContent::FlexEnd => current_pos = free_space,
            JustifyContent::Center => current_pos = (free_space) / 2.0,
            JustifyContent::SpaceBetween => {
                space_per_widget = free_space / (num_children - 1) as f32;
            }
            JustifyContent::SpaceAround => {
                space_per_widget = free_space / num_children as f32;
                current_pos = space_per_widget / 2.0;
            }
            JustifyContent::SpaceEvenly => {
                space_per_widget = free_space / (num_children + 1) as f32;
                current_pos = space_per_widget;
            }
            _ => {}
        }

        for child in parent.child_iter(&hierarchy) {
            // Skip non-displayed widgets
            let display = state.style.display.get(child).cloned().unwrap_or_default();

            if display == Display::None {
                continue;
            }

            // Get the desired width and height
            let width = state.style.width.get(child).cloned().unwrap_or_default();
            let height = state.style.height.get(child).cloned().unwrap_or_default();

            let left = state.style.left.get(child).cloned().unwrap_or_default();
            let top = state.style.top.get(child).cloned().unwrap_or_default();

            let child_border_width = match state
                .style
                .border_width
                .get(child)
                .cloned()
                .unwrap_or_default() 
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_min_width = match state
                .style
                .min_width
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => 0.0,
            };

            let child_max_width = match state
                .style
                .max_width
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => std::f32::INFINITY,
            };

            let child_min_height = match state
                .style
                .min_height
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => 0.0,
            };

            let child_max_height = match state
                .style
                .max_height
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => std::f32::INFINITY,
            };

            let child_margin_left = match state
                .style
                .margin_left
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_margin_right = match state
                .style
                .margin_right
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_margin_top = match state
                .style
                .margin_top
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_height * val,
                _ => 0.0,
            };

            let child_margin_bottom = match state
                .style
                .margin_bottom
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_height * val,
                _ => 0.0,
            };

            let child_padding_left = match state
                .style
                .padding_left
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_padding_right = match state
                .style
                .padding_right
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => 0.0,
            };

            let child_padding_top = match state
                .style
                .padding_top
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_height * val,
                _ => 0.0,
            };

            let child_padding_bottom = match state
                .style
                .padding_bottom
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_height * val,
                _ => 0.0,
            };

            // The new size and position of the child
            let mut new_width;
            let mut new_height;
            let mut new_posx;
            let mut new_posy;

            let flex_direction = state
                .style
                .flex_direction
                .get(child)
                .cloned()
                .unwrap_or_default();

            let child_flex_grow = state
                .style
                .flex_grow
                .get(child)
                .cloned()
                .unwrap_or_default();

            let child_flex_shrink = state
                .style
                .flex_shrink
                .get(child)
                .cloned()
                .unwrap_or_default();

            let mut child_grow_sum = state.transform.get_child_grow_sum(parent);
            let mut child_shrink_sum = state.transform.get_child_shrink_sum(parent);

            if child_grow_sum < 1.0 {
                child_grow_sum = 1.0;
            }

            if child_shrink_sum < 1.0 {
                child_shrink_sum = 1.0;
            }

            let flex_grow_fraction = child_flex_grow / child_grow_sum;
            let flex_shrink_fraction = child_flex_shrink / child_shrink_sum;

            let position = state.style.position.get(child).cloned().unwrap_or_default();

            match flex_direction {
                FlexDirection::Row => {
                    new_width = state.transform.get_child_sum(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.transform.get_child_max(child)
                        + child_padding_top
                        + child_padding_bottom
                        + 2.0 * child_border_width;
                }

                FlexDirection::Column => {
                    new_width = state.transform.get_child_max(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.transform.get_child_sum(child)
                        + child_padding_top
                        + child_padding_bottom
                        + 2.0 * child_border_width;
                }
            }

            match position {
                Position::Relative => {
                    match parent_flex_direction {
                        FlexDirection::Row => {
                            new_posx = current_pos;

                            match width {
                                Length::Pixels(val) => {
                                    new_width = val
                                        + child_padding_left
                                        + child_padding_right
                                        + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_width = (parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - parent_border_width
                                        - parent_border_width)
                                        * val;
                                    new_width += child_padding_left
                                        + child_padding_right
                                        + 2.0 * child_border_width;
                                }
                                _ => {}
                            };

                            if let Some(flex_basis) = state.style.flex_basis.get(child) {
                                new_width = *flex_basis + child_padding_left + child_padding_right + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_width
                                - parent_padding_left
                                - parent_padding_right
                                - parent_border_width
                                - parent_border_width
                                - state.transform.get_child_sum(parent);


                            if parent_free_space >= 0.0 {
                                new_width += flex_grow_fraction * parent_free_space;
                            } else {
                                new_width += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(parent)
                                .cloned()
                                .unwrap_or_default();

                            if let Some(align_self) = state.style.align_self.get(child) {
                                if *align_self == AlignSelf::Stretch {
                                    new_height = parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - parent_border_width
                                        - parent_border_width
                                        - child_margin_top
                                        - child_margin_bottom;
                                }
                            } else {
                                if align_items == AlignItems::Stretch {
                                    new_height = parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - parent_border_width
                                        - parent_border_width
                                        - child_margin_top
                                        - child_margin_bottom;
                                }
                            }

                            match height {
                                Length::Pixels(val) => {
                                    new_height = val
                                        + child_padding_top
                                        + child_padding_bottom
                                        + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_height = (parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - 2.0 * parent_border_width)
                                        * val;
                                    new_height += child_padding_top
                                        + child_padding_bottom
                                        + 2.0 * child_border_width;
                                }

                                _ => {}
                            };

                            // Apply size contraints
                            if new_width < child_min_width {
                                new_width = child_min_width;
                            }

                            if new_width > child_max_width {
                                new_width = child_max_width;
                            }

                            if new_height < child_min_height {
                                new_height = child_min_height;
                            }

                            if new_height > child_max_height {
                                new_height = child_max_height;
                            }

                            match left {
                                Length::Pixels(val) => {
                                    new_posx = current_pos + val;
                                }

                                Length::Percentage(val) => {
                                    new_posx = current_pos + val * parent_width;
                                }

                                _ => {}
                            }

                            new_posx = parent_posx + new_posx + child_margin_left;

                            new_posy = match align_items {
                                AlignItems::FlexStart => 0.0,
                                AlignItems::FlexEnd => {
                                    parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - 2.0 * parent_border_width
                                        - new_height
                                }
                                AlignItems::Center => {
                                    (parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - 2.0 * parent_border_width
                                        - new_height
                                        - child_margin_top
                                        - child_margin_bottom)
                                        / 2.0
                                }
                                AlignItems::Stretch => 0.0,
                                //AlignItems::Baseline => 0.0, //TODO
                            };

                            // align-self overrides align-items
                            if let Some(align_self) = state.style.align_self.get(child) {
                                match align_self {
                                    AlignSelf::FlexStart => new_posy = 0.0,
                                    AlignSelf::FlexEnd => new_posy = parent_height - new_height,
                                    AlignSelf::Center => {
                                        new_posy = (parent_height - new_height) / 2.0
                                    }
                                    AlignSelf::Stretch => new_posy = 0.0,
                                }
                            }

                            match top {
                                Length::Pixels(val) => {
                                    new_posy += val;
                                }

                                Length::Percentage(val) => {
                                    new_posy += val * parent_height;
                                }

                                _ => {}
                            }


                            new_posy = parent_posy + new_posy + child_margin_top;

                            // state.transform.set_posy(
                            //     child,
                            //     parent_posy + new_posy + child_margin_top, // + (child_border_width / 2.0),
                            // );

                            current_pos += new_width
                                + space_per_widget
                                + child_margin_left
                                + child_margin_right;
                        }

                        FlexDirection::Column => {
                            new_posy = current_pos;

                            match height {
                                Length::Pixels(val) => {
                                    new_height = val
                                        + child_padding_top
                                        + child_padding_bottom
                                        + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_height = (parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - parent_border_width
                                        - parent_border_width)
                                        * val;
                                    new_height += child_padding_top
                                        + child_padding_bottom
                                        + 2.0 * child_border_width;
                                }
                                _ => {}
                            };

                            if let Some(flex_basis) = state.style.flex_basis.get(child) {
                                new_height = *flex_basis + child_padding_top + child_padding_bottom + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_height
                                - parent_padding_top
                                - parent_padding_bottom
                                - parent_border_width
                                - parent_border_width
                                - state.transform.get_child_sum(parent);

                            if parent_free_space >= 0.0 {
                                new_height += flex_grow_fraction * parent_free_space;
                            } else {
                                new_height += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(parent)
                                .cloned()
                                .unwrap_or_default();

                            if let Some(align_self) = state.style.align_self.get(child) {
                                if *align_self == AlignSelf::Stretch {
                                    new_width = parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - parent_border_width
                                        - parent_border_width
                                        - child_margin_left
                                        - child_margin_right;
                                }
                            } else {
                                if align_items == AlignItems::Stretch {
                                    new_width = parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - parent_border_width
                                        - parent_border_width
                                        - child_margin_left
                                        - child_margin_right;
                                }
                            }

                            match width {
                                Length::Pixels(val) => {
                                    new_width = val
                                        + child_padding_left
                                        + child_padding_right
                                        + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_width = (parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - 2.0 * parent_border_width)
                                        * val;
                                    new_width += child_padding_left
                                        + child_padding_right
                                        + 2.0 * child_border_width;
                                }

                                _ => {}
                            };

                            // Apply size contraints
                            if new_width < child_min_width {
                                new_width = child_min_width;
                            }

                            if new_width > child_max_width {
                                new_width = child_max_width;
                            }

                            if new_height < child_min_height {
                                new_height = child_min_height;
                            }

                            if new_height > child_max_height {
                                new_height = child_max_height;
                            }

                            //state.transform.set_width(child, new_width);
                            //state.transform.set_height(child, new_height);

                            match top {
                                Length::Pixels(val) => {
                                    new_posy = current_pos + val;
                                }

                                Length::Percentage(val) => {
                                    new_posy = current_pos + val * parent_height;
                                }

                                _ => {}
                            }

                            
                            new_posy = parent_posy + new_posy + child_margin_top;
                            
                            // state
                            //     .transform
                            //     .set_posy(child, parent_posy + new_posy + child_margin_top);

                            //let align_items = state.style.align_items.get(parent).cloned().unwrap_or_default();

                            new_posx = match align_items {
                                AlignItems::FlexStart => 0.0,
                                AlignItems::FlexEnd => {
                                    parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - 2.0 * parent_border_width
                                        - new_width
                                }
                                AlignItems::Center => {
                                    (parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - 2.0 * parent_border_width
                                        - new_width
                                        - child_margin_left
                                        - child_margin_right)
                                        / 2.0
                                }
                                AlignItems::Stretch => 0.0,
                                //AlignItems::Baseline => 0.0, //TODO
                            };

                            // align-self overrides align-items
                            if let Some(align_self) = state.style.align_self.get(child) {
                                match align_self {
                                    AlignSelf::FlexStart => new_posx = 0.0,
                                    AlignSelf::FlexEnd => new_posx = parent_width - new_width,
                                    AlignSelf::Center => {
                                        new_posx = (parent_width - new_width) / 2.0
                                    }
                                    AlignSelf::Stretch => new_posx = 0.0,
                                }
                            }

                            match left {
                                Length::Pixels(val) => {
                                    new_posx += val;
                                }

                                Length::Percentage(val) => {
                                    new_posx += val * parent_width;
                                }

                                _ => {}
                            }

                            new_posx = parent_posx + new_posx + child_margin_left;

                            // state
                            //     .transform
                            //     .set_posx(child, parent_posx + new_posx + child_margin_left);

                            current_pos += new_height
                                + space_per_widget
                                + child_margin_top
                                + child_margin_bottom;
                        }
                    }
                }

                Position::Absolute => {
                    let width = state.style.width.get(child).cloned().unwrap_or_default();
                    let height = state.style.height.get(child).cloned().unwrap_or_default();

                    let left = state.style.left.get(child).cloned().unwrap_or_default();
                    let right = state.style.right.get(child).cloned().unwrap_or_default();
                    let top = state.style.top.get(child).cloned().unwrap_or_default();
                    let bottom = state.style.bottom.get(child).cloned().unwrap_or_default();

                    new_posx = parent_posx;
                    new_posy = parent_posy;

                    let r = match right {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => val * parent_width,
                        Length::Initial(val) => val,
                        Length::Auto => 0.0,
                    };

                    let l = match left {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => val * parent_width,
                        Length::Initial(val) => val,
                        Length::Auto => 0.0,
                    };

                    if !right.is_auto() && !left.is_auto() {
                        new_width = parent_width - l - r;
                    }

                    let b = match bottom {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => val * parent_height,
                        Length::Initial(val) => val,
                        Length::Auto => 0.0,
                    };

                    let t = match top {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => val * parent_height,
                        Length::Initial(val) => val,
                        Length::Auto => 0.0,
                    };

                    if !bottom.is_auto() && !top.is_auto() {
                        new_height = parent_height - t - b;
                    }

                    match width {
                        Length::Auto => {}
                        Length::Pixels(val) => new_width = val,
                        Length::Initial(val) => new_width = val,
                        Length::Percentage(val) => new_width = val * parent_width,
                    }

                    match height {
                        Length::Auto => {}
                        Length::Pixels(val) => new_height = val,
                        Length::Initial(val) => new_height = val,
                        Length::Percentage(val) => new_height = val * parent_height,
                    }

                    //state.transform.set_width(child, new_width);
                    //state.transform.set_height(child, new_height);

                    match right {
                        Length::Pixels(val) => {
                            new_posx = parent_posx + parent_width - new_width - val;
                        }

                        Length::Percentage(val) => {
                            new_posx = parent_posx + parent_width
                                - new_width
                                - (val
                                    * (parent_width - parent_padding_left - parent_padding_right));
                        }

                        _ => {}
                    }

                    match left {
                        Length::Pixels(val) => {
                            new_posx = parent_posx + val;
                        }

                        Length::Percentage(val) => {
                            new_posx = parent_posx
                                + (val
                                    * (parent_width - parent_padding_left - parent_padding_right));
                        }

                        _ => {}
                    }

                    match bottom {
                        Length::Pixels(val) => {
                            new_posy = parent_posy + parent_height - new_height - val;
                        }

                        Length::Percentage(val) => {
                            new_posy = parent_posy + parent_height
                                - new_height
                                - (val
                                    * (parent_height - parent_padding_top - parent_padding_bottom));
                        }

                        _ => {}
                    }

                    match top {
                        Length::Pixels(val) => {
                            new_posy = parent_posy + val;
                        }

                        Length::Percentage(val) => {
                            new_posy = parent_posy
                                + (val
                                    * (parent_height - parent_padding_top - parent_padding_bottom));
                        }

                        _ => {}
                    }

                    //state.transform.set_posx(child, new_posx);
                    //state.transform.set_posy(child, new_posy);
                }

                
            }



            if state.transform.get_posx(child) != new_posx {
                state.transform.set_posx(child, new_posx);
                should_continue = true;
            }
            

            if state.transform.get_posy(child) != new_posy {
                state.transform.set_posy(child, new_posy);
                should_continue = true;
            }

            if state.transform.get_width(child) != new_width {
                state.transform.set_width(child, new_width);
                should_continue = true;
            }

            if state.transform.get_height(child) != new_height {
                state.transform.set_height(child, new_height);
                should_continue = true;
            }

            // if !should_continue {
            //     if let Some(ns) = hierarchy.get_next_sibling(parent) {
            //         next_sibling = ns;
            //         hierarchy_down_iterator = next_sibling.into_iter(hierarchy);
            //     }
            // } else {
            //     should_continue = false;
            // }


        }




        // Set the transform properties
    }
    


}


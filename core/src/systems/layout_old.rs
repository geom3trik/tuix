use prop::PropGet;

use crate::{Entity, GeometryChanged, Propagation, State};

use crate::tree::*;
use crate::style::*;

use crate::{Event, WindowEvent};

use crate::flexbox::AlignItems;

pub fn apply_z_ordering(state: &mut State, tree: &Tree) {
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        let parent = tree.get_parent(entity).unwrap();

        if let Some(z_order) = state.style.z_order.get(entity) {
            state.data.set_z_order(entity, *z_order);
        } else {
            let parent_z_order = state.data.get_z_order(parent);
            state.data.set_z_order(entity, parent_z_order);
        }
    }
}

pub fn apply_layout(state: &mut State, tree: &Tree) {
    //println!("Relayout");
    // Reset
    for entity in tree.entities.iter() {
        state.data.set_child_sum(*entity, 0.0);
        state.data.set_child_pos(*entity, 0.0);
        state.data.set_child_grow_sum(*entity, 0.0);
        state.data.set_child_shrink_sum(*entity, 0.0);
    }

    //let mut tree_up_iterator = tree.entities.iter();

    let layout_tree = tree.into_iter().collect::<Vec<Entity>>();
    //let layout_tree = tree.entities.clone();

    //////////////////////
    // Walk up the tree //
    //////////////////////
    //while let Some(entity) = layout_tree.iter().next_back() {
    for entity in layout_tree.iter().rev() {
        // Stop before the window
        if *entity == Entity::root() {
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

        let parent = tree.get_parent(*entity).unwrap();

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        //println!("Entity: {} Parent: {} Width: {} Height: {}", entity, parent, parent_width, parent_height);

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

        // Margins
        let child_margin_left = entity.get_margin_left(state).get_value(parent_width);
        let child_margin_right = entity.get_margin_right(state).get_value(parent_width);
        let child_margin_top = entity.get_margin_top(state).get_value(parent_width);
        let child_margin_bottom = entity.get_margin_bottom(state).get_value(parent_width);

        // let child_margin_left = match state
        //     .style
        //     .margin_left
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let child_margin_right = match state
        //     .style
        //     .margin_right
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let child_margin_top = match state
        //     .style
        //     .margin_top
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        // let child_margin_bottom = match state
        //     .style
        //     .margin_bottom
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        // Padding
        let child_padding_left = entity.get_padding_left(state).get_value(parent_width);
        let child_padding_right = entity.get_padding_right(state).get_value(parent_width);
        let child_padding_top = entity.get_padding_top(state).get_value(parent_width);
        let child_padding_bottom = entity.get_padding_bottom(state).get_value(parent_width);

        // let child_padding_left = match state
        //     .style
        //     .padding_left
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let child_padding_right = match state
        //     .style
        //     .padding_right
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let child_padding_top = match state
        //     .style
        //     .padding_top
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        // let child_padding_bottom = match state
        //     .style
        //     .padding_bottom
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        let child_border_width = entity.get_border_width(state).get_value(parent_width);

        // let child_border_width = match state
        //     .style
        //     .border_width
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let parent_border_width = match state
        //     .style
        //     .border_width
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let parent_padding_left = match state
        //     .style
        //     .padding_left
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let parent_padding_right = match state
        //     .style
        //     .padding_right
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let parent_padding_top = match state
        //     .style
        //     .padding_top
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        // let parent_padding_bottom = match state
        //     .style
        //     .padding_bottom
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        let child_flex_direction = entity.get_flex_direction(state);
        let parent_flex_direction = parent.get_flex_direction(state);

        // let parent_flex_direction = state
        //     .style
        //     .flex_direction
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default();
        // let child_flex_direction = state
        //     .style
        //     .flex_direction
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default();

        // Get the desired width from the style
        //let width = state.style.width.get(*entity).cloned().unwrap_or_default();
        let width = entity.get_width(state);

        // Get the desired height from the style
        //let height = state.style.height.get(*entity).cloned().unwrap_or_default();
        let height = entity.get_height(state);

        let mut new_width;
        let mut new_height;

        match child_flex_direction {
            FlexDirection::Row => {
                // Set width to the sum of the widths of the children
                new_width = state.data.get_child_sum(*entity);
                // Set height to the maximum height of the children
                new_height = state.data.get_child_max(*entity);
            }

            FlexDirection::Column => {
                // Set width to the maximum width of the children
                new_width = state.data.get_child_max(*entity);
                // Set height to the maximum height of the children
                new_height = state.data.get_child_sum(*entity);
            }
        }

        match parent_flex_direction {
            FlexDirection::Row => {
                // Start with desired width if specified in pixels
                match width {
                    Length::Pixels(val) => {
                        new_width = val;
                    }

                    // Length::Percentage(val) => {
                    //     new_width = (parent_width
                    //         - parent_padding_left
                    //         - parent_padding_right
                    //         - 2.0 * parent_border_width)
                    //         * val;
                    // }
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
                        state
                            .data
                            .set_child_sum(parent, state.data.get_child_sum(parent) + new_width);
                        //state.data.set_child_max(parent, new_height);
                        state.data.set_child_max(
                            parent,
                            new_height.max(state.data.get_child_max(parent)),
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
                        new_height = (parent_height)
                            // - parent_padding_top
                            // - parent_padding_bottom
                            // - 2.0 * parent_border_width)
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
                        state
                            .data
                            .set_child_sum(parent, state.data.get_child_sum(parent) + new_height);
                        //state.data.set_child_max(parent, new_width);
                        state
                            .data
                            .set_child_max(parent, new_width.max(state.data.get_child_max(parent)));
                    }

                    _ => {}
                }
            }
        }

        if let Some(flex_grow) = state.style.flex_grow.get(*entity) {
            state
                .data
                .set_child_grow_sum(parent, state.data.get_child_grow_sum(parent) + flex_grow);
        }

        if let Some(flex_shrink) = state.style.flex_shrink.get(*entity) {
            state.data.set_child_shrink_sum(
                parent,
                state.data.get_child_shrink_sum(parent) + flex_shrink,
            );
        }
    }

    let mut should_continue = false;

    ////////////////////////
    // Walk down the tree //
    ////////////////////////
    //while let Some(parent) = tree_down_iterator.next() {
    for parent in layout_tree.iter() {
        // Parent properties

        let parent_width = state.data.get_width(*parent);
        let parent_height = state.data.get_height(*parent);

        //println!("Parent: {} Width: {:?} Height: {:?}", parent, parent_width, parent_height);

        let parent_border_width = match state
            .style
            .border_width
            .get(*parent)
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
            .get(*parent)
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
            .get(*parent)
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
            .get(*parent)
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
            .get(*parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_posx = state.data.get_posx(*parent) + parent_padding_left + parent_border_width;
        let parent_posy = state.data.get_posy(*parent) + parent_padding_top + parent_border_width;

        //TEMP - Move to the walk up phase
        let mut num_children = 0;
        for _ in parent.child_iter(&tree) {
            num_children += 1;
        }

        let parent_flex_direction = state
            .style
            .flex_direction
            .get(*parent)
            .cloned()
            .unwrap_or_default();

        let justify_content = state
            .style
            .justify_content
            .get(*parent)
            .cloned()
            .unwrap_or_default();

        let mut current_pos = 0.0;
        let mut space_per_widget = 0.0;

        let free_space = match parent_flex_direction {
            FlexDirection::Row => {
                state.data.get_width(*parent)
                    - parent_padding_left
                    - parent_padding_right
                    - 2.0 * parent_border_width
                    - state.data.get_child_sum(*parent)
            }

            FlexDirection::Column => {
                state.data.get_height(*parent)
                    - parent_padding_top
                    - parent_padding_bottom
                    - 2.0 * parent_border_width
                    - state.data.get_child_sum(*parent)
            }
        };

        match justify_content {
            JustifyContent::FlexStart => current_pos = 0.0,
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

        // Used to calculate the positional proportion of the child
        let mut proportion_numerator = 0.0f32;
        let mut flex_length_sum = 0.0f32;

        for child in parent.child_iter(&tree) {
            // Skip non-displayed widgets
            let display = state.style.display.get(child).cloned().unwrap_or_default();

            if display == Display::None {
                continue;
            }

            // Get the desired width and height
            let width = state.style.width.get(child).cloned().unwrap_or_default();
            let height = state.style.height.get(child).cloned().unwrap_or_default();

            // if *parent == Entity::new(1) {
            //     println!("Child: {} Width: {:?} Height: {:?}", child, width, height);
            // }

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

            let mut child_grow_sum = state.data.get_child_grow_sum(*parent);
            let mut child_shrink_sum = state.data.get_child_shrink_sum(*parent);

            if child_grow_sum < 1.0 {
                child_grow_sum = 1.0;
            }

            if child_shrink_sum < 1.0 {
                child_shrink_sum = 1.0;
            }

            let flex_grow_fraction = child_flex_grow / child_grow_sum;
            let flex_shrink_fraction = child_flex_shrink / child_shrink_sum;

            proportion_numerator += child_flex_grow;
            let positional_proportion = proportion_numerator / child_grow_sum;

            let position = state.style.position.get(child).cloned().unwrap_or_default();

            match flex_direction {
                FlexDirection::Row => {
                    new_width = state.data.get_child_sum(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.data.get_child_max(child)
                        + child_padding_top
                        + child_padding_bottom
                        + 2.0 * child_border_width;
                }

                FlexDirection::Column => {
                    new_width = state.data.get_child_max(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.data.get_child_sum(child)
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
                                new_width = *flex_basis
                                    + child_padding_left
                                    + child_padding_right
                                    + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_width
                                - parent_padding_left
                                - parent_padding_right
                                - parent_border_width
                                - parent_border_width
                                - state.data.get_child_sum(*parent);

                            if parent_free_space >= 0.0 {
                                let flex_width = (proportion_numerator * parent_free_space
                                    / child_grow_sum)
                                    .round()
                                    - flex_length_sum;
                                flex_length_sum += flex_width;
                                new_width += flex_width;
                            //new_width += flex_grow_fraction * parent_free_space;
                            } else {
                                new_width += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(*parent)
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

                            // state.data.set_child_sum(
                            //     *parent,
                            //     state.data.get_child_sum(*parent) + new_width,
                            // );
                            // //state.data.set_child_max(parent, new_height);
                            // state.data.set_child_max(
                            //     *parent,
                            //     new_height.max(state.data.get_child_max(*parent)),
                            // );

                            //current_pos -= new_width;

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

                            // state.data.set_posy(
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
                                new_height = *flex_basis
                                    + child_padding_top
                                    + child_padding_bottom
                                    + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_height
                                - parent_padding_top
                                - parent_padding_bottom
                                - parent_border_width
                                - parent_border_width
                                - state.data.get_child_sum(*parent);

                            if parent_free_space >= 0.0 {
                                let flex_height = (proportion_numerator * parent_free_space
                                    / child_grow_sum)
                                    .round()
                                    - flex_length_sum;
                                flex_length_sum += flex_height;
                                new_height += flex_height;
                            //new_height += flex_grow_fraction * parent_free_space;
                            } else {
                                new_height += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(*parent)
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

                            // state.data.set_child_sum(
                            //     *parent,
                            //     state.data.get_child_sum(*parent) + new_height,
                            // );

                            // state.data.set_child_max(
                            //     *parent,
                            //     new_width.max(state.data.get_child_max(*parent)),
                            // );

                            //current_pos -= new_height;

                            //state.data.set_width(child, new_width);
                            //state.data.set_height(child, new_height);

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
                            //     .data
                            //     .set_posy(child, parent_posy + new_posy + child_margin_top);

                            //let align_items = state.style.align_items.get(*parent).cloned().unwrap_or_default();

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
                            //     .data
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

                    //state.data.set_width(child, new_width);
                    //state.data.set_height(child, new_height);

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

                    //state.data.set_posx(child, new_posx);
                    //state.data.set_posy(child, new_posy);
                }
            }

            let mut geometry_changed = GeometryChanged::default();

            if state.data.get_posx(child) != new_posx {
                state.data.set_posx(child, new_posx);
                should_continue = true;
                geometry_changed.posx = true;
            }

            if state.data.get_posy(child) != new_posy {
                state.data.set_posy(child, new_posy);
                should_continue = true;
                geometry_changed.posy = true;
            }

            if state.data.get_width(child) != new_width {
                state.data.set_width(child, new_width);
                should_continue = true;
                geometry_changed.width = true;
            }

            if state.data.get_height(child) != new_height {
                state.data.set_height(child, new_height);
                should_continue = true;
                geometry_changed.height = true;
            }

            if !should_continue {
                // if let Some(ns) = tree.get_next_sibling(*parent) {
                //     next_sibling = ns;
                //     tree_down_iterator = next_sibling.into_iter(tree);
                // }
            } else {
                state.insert_event(
                    Event::new(WindowEvent::GeometryChanged(geometry_changed))
                        .target(child)
                        .propagate(Propagation::Down),
                );
                should_continue = false;
            }
        }

        // Set the data properties
    }
}

pub fn apply_layout2(state: &mut State, tree: &Tree) {
    //println!("Relayout");
    // Reset
    for entity in tree.entities.iter() {
        state.data.set_child_sum(*entity, 0.0);
        state.data.set_child_pos(*entity, 0.0);
        state.data.set_child_grow_sum(*entity, 0.0);
        state.data.set_child_shrink_sum(*entity, 0.0);
    }

    //let mut tree_up_iterator = tree.entities.iter();

    let layout_tree = tree.into_iter().collect::<Vec<Entity>>();
    //let layout_tree = tree.entities.clone();

    //////////////////////
    // Walk up the tree //
    //////////////////////
    //while let Some(entity) = layout_tree.iter().next_back() {
    for entity in layout_tree.iter().rev() {
        // Stop before the window
        if *entity == Entity::root() {
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

        let parent = tree.get_parent(*entity).unwrap();

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        //println!("Entity: {} Parent: {} Width: {} Height: {}", entity, parent, parent_width, parent_height);

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

        new_width = child_margin_left
            + child_margin_right
            + child_padding_left
            + child_padding_right
            + 2.0 * child_border_width;
        new_height = child_margin_top
            + child_margin_bottom
            + child_padding_top
            + child_padding_bottom
            + 2.0 * child_border_width;

        // new_width += child_margin_left
        //     + child_margin_right
        //     + child_padding_left
        //     + child_padding_right
        //     + 2.0 * child_border_width;
        // new_height += child_margin_top
        //     + child_margin_bottom
        //     + child_padding_top
        //     + child_padding_bottom
        //     + 2.0 * child_border_width;

        match child_flex_direction {
            FlexDirection::Row => {
                // Set width to the sum of the widths of the children
                new_width = state.data.get_child_sum(*entity);
                // Set height to the maximum height of the children
                new_height = state.data.get_child_max(*entity);
            }

            FlexDirection::Column => {
                // Set width to the maximum width of the children
                new_width = state.data.get_child_max(*entity);
                // Set height to the maximum height of the children
                new_height =
                    state.data.get_child_sum(*entity) + child_padding_top + child_padding_bottom;
            }
        }

        //println!("1.  Entity: {} Width: {} Height: {}", entity, new_width, new_height);

        match parent_flex_direction {
            FlexDirection::Row => {
                // Start with desired width if specified in pixels
                match width {
                    Length::Pixels(val) => {
                        new_width = val;
                    }

                    // Length::Percentage(val) => {
                    //     new_width = (parent_width
                    //         - parent_padding_left
                    //         - parent_padding_right
                    //         - 2.0 * parent_border_width)
                    //         * val;
                    // }
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

                let position = state
                    .style
                    .position
                    .get(*entity)
                    .cloned()
                    .unwrap_or_default();

                match position {
                    Position::Relative => {
                        state
                            .data
                            .set_child_sum(parent, state.data.get_child_sum(parent) + new_width);
                        //state.data.set_child_max(parent, new_height);
                        state.data.set_child_max(
                            parent,
                            new_height.max(state.data.get_child_max(parent)),
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

                    // Length::Percentage(val) => {
                    //     new_height = (parent_height
                    //         - parent_padding_top
                    //         - parent_padding_bottom
                    //         - 2.0 * parent_border_width)
                    //         * val;
                    // }
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

                //println!("2.  Entity: {} Width: {} Height: {}", entity, new_width, new_height);

                //println!("3.  Entity: {} Width: {} Height: {}", entity, new_width, new_height);

                let position = state
                    .style
                    .position
                    .get(*entity)
                    .cloned()
                    .unwrap_or_default();

                match position {
                    Position::Relative => {
                        state
                            .data
                            .set_child_sum(parent, state.data.get_child_sum(parent) + new_height);
                        //state.data.set_child_max(parent, new_width);
                        state
                            .data
                            .set_child_max(parent, new_width.max(state.data.get_child_max(parent)));
                    }

                    _ => {}
                }
            }
        }

        if let Some(flex_grow) = state.style.flex_grow.get(*entity) {
            state
                .data
                .set_child_grow_sum(parent, state.data.get_child_grow_sum(parent) + flex_grow);
        }

        if let Some(flex_shrink) = state.style.flex_shrink.get(*entity) {
            state.data.set_child_shrink_sum(
                parent,
                state.data.get_child_shrink_sum(parent) + flex_shrink,
            );
        }
    }

    let mut should_continue = false;

    ////////////////////////
    // Walk down the tree //
    ////////////////////////
    //while let Some(parent) = tree_down_iterator.next() {
    for parent in layout_tree.iter() {
        // Parent properties

        let parent_width = state.data.get_width(*parent);
        let parent_height = state.data.get_height(*parent);

        //println!("Parent: {} Width: {:?} Height: {:?}", parent, parent_width, parent_height);

        let parent_border_width = match state
            .style
            .border_width
            .get(*parent)
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
            .get(*parent)
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
            .get(*parent)
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
            .get(*parent)
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
            .get(*parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_posx = state.data.get_posx(*parent) + parent_padding_left + parent_border_width;
        let parent_posy = state.data.get_posy(*parent) + parent_padding_top + parent_border_width;

        //TEMP - Move to the walk up phase
        let mut num_children = 0;
        for _ in parent.child_iter(&tree) {
            num_children += 1;
        }

        let parent_flex_direction = state
            .style
            .flex_direction
            .get(*parent)
            .cloned()
            .unwrap_or_default();

        let mut current_pos = 0.0;
        let mut space_per_widget = 0.0;

        let free_space = match parent_flex_direction {
            FlexDirection::Row => {
                state.data.get_width(*parent)
                    - parent_padding_left
                    - parent_padding_right
                    - 2.0 * parent_border_width
                    - state.data.get_child_sum(*parent)
            }

            FlexDirection::Column => {
                state.data.get_height(*parent)
                    - parent_padding_top
                    - parent_padding_bottom
                    - 2.0 * parent_border_width
                    - state.data.get_child_sum(*parent)
            }
        };

        let justify_content = state
            .style
            .justify_content
            .get(*parent)
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

        // Used to calculate the positional proportion of the child
        let mut proportion_numerator = 0.0f32;
        let mut flex_length_sum = 0.0f32;

        for child in parent.child_iter(&tree) {
            // Skip non-displayed widgets
            let display = state.style.display.get(child).cloned().unwrap_or_default();

            if display == Display::None {
                continue;
            }

            // Get the desired width and height
            let width = state.style.width.get(child).cloned().unwrap_or_default();
            let height = state.style.height.get(child).cloned().unwrap_or_default();

            // if *parent == Entity::new(1) {
            //     println!("Child: {} Width: {:?} Height: {:?}", child, width, height);
            // }

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

            let mut child_grow_sum = state.data.get_child_grow_sum(*parent);
            let mut child_shrink_sum = state.data.get_child_shrink_sum(*parent);

            if child_grow_sum < 1.0 {
                child_grow_sum = 1.0;
            }

            if child_shrink_sum < 1.0 {
                child_shrink_sum = 1.0;
            }

            let flex_grow_fraction = child_flex_grow / child_grow_sum;
            let flex_shrink_fraction = child_flex_shrink / child_shrink_sum;

            proportion_numerator += child_flex_grow;
            let positional_proportion = proportion_numerator / child_grow_sum;

            let position = state.style.position.get(child).cloned().unwrap_or_default();

            match flex_direction {
                FlexDirection::Row => {
                    new_width = state.data.get_child_sum(child);
                    // + child_padding_left
                    // + child_padding_right
                    // + 2.0 * child_border_width;
                    new_height = state.data.get_child_max(child);
                    // + child_padding_top
                    // + child_padding_bottom
                    // + 2.0 * child_border_width;
                }

                FlexDirection::Column => {
                    new_width = state.data.get_child_max(child);
                    // + child_padding_left
                    // + child_padding_right
                    // + 2.0 * child_border_width;
                    new_height = state.data.get_child_sum(child);
                    // + child_padding_top
                    // + child_padding_bottom
                    // + 2.0 * child_border_width;
                }
            }

            match position {
                Position::Relative => {
                    match parent_flex_direction {
                        FlexDirection::Row => {
                            new_posx = current_pos;

                            match width {
                                Length::Pixels(val) => {
                                    new_width = val;
                                    // + child_padding_left
                                    // + child_padding_right
                                    // + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_width = (parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - parent_border_width
                                        - parent_border_width)
                                        * val;
                                    // new_width += child_padding_left
                                    //     + child_padding_right
                                    //     + 2.0 * child_border_width;
                                }
                                _ => {}
                            };

                            if let Some(flex_basis) = state.style.flex_basis.get(child) {
                                new_width = *flex_basis;
                                // + child_padding_left
                                // + child_padding_right
                                // + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_width
                                - parent_padding_left
                                - parent_padding_right
                                - parent_border_width
                                - parent_border_width
                                - state.data.get_child_sum(*parent);

                            if parent_free_space >= 0.0 {
                                let flex_width = (proportion_numerator * parent_free_space
                                    / child_grow_sum)
                                    .round()
                                    - flex_length_sum;
                                flex_length_sum += flex_width;
                                new_width += flex_width;
                            //new_width += flex_grow_fraction * parent_free_space;
                            } else {
                                new_width += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(*parent)
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
                                    new_height = val;
                                    // + child_padding_top
                                    // + child_padding_bottom
                                    // + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_height = (parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - 2.0 * parent_border_width)
                                        * val;
                                    // new_height += child_padding_top
                                    //     + child_padding_bottom
                                    //     + 2.0 * child_border_width;
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

                            // state.data.set_posy(
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
                                    new_height = val;
                                    // + child_padding_top
                                    // + child_padding_bottom
                                    // + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_height = (parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - parent_border_width
                                        - parent_border_width)
                                        * val;
                                    // new_height += child_padding_top
                                    //     + child_padding_bottom
                                    //     + 2.0 * child_border_width;
                                }
                                _ => {}
                            };

                            if let Some(flex_basis) = state.style.flex_basis.get(child) {
                                new_height = *flex_basis;
                                // + child_padding_top
                                // + child_padding_bottom
                                // + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_height
                                - parent_padding_top
                                - parent_padding_bottom
                                - parent_border_width
                                - parent_border_width
                                - state.data.get_child_sum(*parent);

                            if parent_free_space >= 0.0 {
                                let flex_height = (proportion_numerator * parent_free_space
                                    / child_grow_sum)
                                    .round()
                                    - flex_length_sum;
                                flex_length_sum += flex_height;
                                new_height += flex_height;
                            //new_height += flex_grow_fraction * parent_free_space;
                            } else {
                                new_height += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(*parent)
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
                                    new_width = val;
                                    // + child_padding_left
                                    // + child_padding_right
                                    // + 2.0 * child_border_width;
                                }

                                Length::Percentage(val) => {
                                    new_width = (parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - 2.0 * parent_border_width)
                                        * val;
                                    // new_width += child_padding_left
                                    //     + child_padding_right
                                    //     + 2.0 * child_border_width;
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
                            //     .data
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

                    //state.data.set_width(child, new_width);
                    //state.data.set_height(child, new_height);

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

                    //state.data.set_posx(child, new_posx);
                    //state.data.set_posy(child, new_posy);
                }
            }

            let mut geometry_changed = GeometryChanged::default();

            if state.data.get_posx(child) != new_posx {
                state.data.set_posx(child, new_posx);
                should_continue = true;
                geometry_changed.posx = true;
            }

            if state.data.get_posy(child) != new_posy {
                state.data.set_posy(child, new_posy);
                should_continue = true;
                geometry_changed.posy = true;
            }

            if state.data.get_width(child) != new_width {
                state.data.set_width(child, new_width);
                should_continue = true;
                geometry_changed.width = true;
            }

            if state.data.get_height(child) != new_height {
                state.data.set_height(child, new_height);
                should_continue = true;
                geometry_changed.height = true;
            }

            if !should_continue {
                // if let Some(ns) = tree.get_next_sibling(*parent) {
                //     next_sibling = ns;
                //     tree_down_iterator = next_sibling.into_iter(tree);
                // }
            } else {
                state.insert_event(
                    Event::new(WindowEvent::GeometryChanged(geometry_changed))
                        .target(child)
                        .propagate(Propagation::Down),
                );
                should_continue = false;
            }
        }

        // Set the data properties
    }
}


/* 
pub fn apply_layout3(state: &mut State, tree: &Tree) {
    //println!("Relayout");
    // Reset
    for entity in tree.entities.iter() {
        state.data.set_child_sum(*entity, 0.0);
        state.data.set_child_pos(*entity, 0.0);
        state.data.set_child_grow_sum(*entity, 0.0);
        state.data.set_child_shrink_sum(*entity, 0.0);
    }

    //let mut tree_up_iterator = tree.entities.iter();

    let layout_tree = tree.into_iter().collect::<Vec<Entity>>();
    //let layout_tree = tree.entities.clone();

    //////////////////////
    // Walk up the tree //
    //////////////////////
    //while let Some(entity) = layout_tree.iter().next_back() {
    for entity in layout_tree.iter().rev() {
        // Stop before the window
        if *entity == Entity::root() {
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

        let parent = tree.get_parent(*entity).unwrap();

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        //println!("Entity: {} Parent: {} Width: {} Height: {}", entity, parent, parent_width, parent_height);

        // Size Constraints
        let child_min_width = entity.get_min_width(state).get_value(parent_width);
        let child_max_width = match entity.get_max_width(state) {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => std::f32::INFINITY,
        };
        let child_min_height = entity.get_min_height(state).get_value(parent_width);
        let child_max_height = match entity.get_max_height(state) {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => std::f32::INFINITY,
        };

        // Margins
        let child_margin_left = entity.get_margin_left(state).get_value(parent_width);
        let child_margin_right = entity.get_margin_right(state).get_value(parent_width);
        let child_margin_top = entity.get_margin_top(state).get_value(parent_height);
        let child_margin_bottom = entity.get_margin_bottom(state).get_value(parent_height);

        // Padding
        let child_padding_left = entity.get_padding_left(state).get_value(parent_width);
        let child_padding_right = entity.get_padding_right(state).get_value(parent_width);
        let child_padding_top = entity.get_padding_top(state).get_value(parent_height);
        let child_padding_bottom = entity.get_padding_bottom(state).get_value(parent_height);

        // Border
        let child_border_width = entity.get_border_width(state).get_value(parent_width);

        // Flex Container
        let child_flex_direction = entity.get_flex_direction(state);
        let parent_flex_direction = parent.get_flex_direction(state);

        // Position
        let position = entity.get_position(state);

        // Size
        let width = entity.get_width(state);
        let height = entity.get_height(state);

        let mut new_width;
        let mut new_height;

        match child_flex_direction {
            FlexDirection::Row => {
                // Set width to the sum of the widths of the children
                new_width = state.data.get_child_sum(*entity);
                // Set height to the maximum height of the children
                new_height = state.data.get_child_max(*entity);
            }

            FlexDirection::Column => {
                // Set width to the maximum width of the children
                new_width = state.data.get_child_max(*entity);
                // Set height to the maximum height of the children
                new_height = state.data.get_child_sum(*entity);
            }
        }

        //println!("UP0  Entity: {} Width: {}  Height: {}", entity, new_width, new_height);

        // Set width to desired width if specified in pixels
        match width {
            Length::Pixels(val) => {
                new_width = val;
            }

            _ => {}
        };

        // Set height to desired height if specified in pixels
        match height {
            Length::Pixels(val) => {
                new_height = val;
            }

            _ => {}
        }

        match parent_flex_direction {
            FlexDirection::Row => {


                // Flex basis overrides desired width
                if let Some(flex_basis) = state.style.flex_basis.get(*entity) {
                    new_width = *flex_basis;
                }

                // Apply size constraints
                new_width = new_width.clamp(child_min_width, child_max_width);
                new_height = new_height.clamp(child_min_height, child_max_height);

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



                match position {
                    Position::Relative => {
                        state
                            .data
                            .set_child_sum(parent, state.data.get_child_sum(parent) + new_width);
                        //state.data.set_child_max(parent, new_height);
                        state.data.set_child_max(
                            parent,
                            new_height.max(state.data.get_child_max(parent)),
                        );
                    }

                    _ => {}
                }
            }

            FlexDirection::Column => {

                if let Some(flex_basis) = state.style.flex_basis.get(*entity) {
                    new_height = *flex_basis;
                }

                // Apply size constraints
                new_width = new_width.clamp(child_min_width, child_max_width);
                new_height = new_height.clamp(child_min_height, child_max_height);

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

                match position {
                    Position::Relative => {
                        state
                            .data
                            .set_child_sum(parent, state.data.get_child_sum(parent) + new_height);
                        //state.data.set_child_max(parent, new_width);
                        state
                            .data
                            .set_child_max(parent, new_width.max(state.data.get_child_max(parent)));
                    }

                    _ => {}
                }
            }
        }

        //println!("UP1  Entity: {} Width: {}  Height: {}", entity, new_width, new_height);

        if let Some(flex_grow) = state.style.flex_grow.get(*entity) {
            state
                .data
                .set_child_grow_sum(parent, state.data.get_child_grow_sum(parent) + flex_grow);
        }

        if let Some(flex_shrink) = state.style.flex_shrink.get(*entity) {
            state.data.set_child_shrink_sum(
                parent,
                state.data.get_child_shrink_sum(parent) + flex_shrink,
            );
        }
    }

    let mut should_continue = false;

    ////////////////////////
    // Walk down the tree //
    ////////////////////////
    for parent in layout_tree.iter() {
        // Parent properties

        let parent_width = state.data.get_width(*parent);
        let parent_height = state.data.get_height(*parent);

        //println!("Parent: {} Width: {:?} Height: {:?}", parent, parent_width, parent_height);

        let parent_border_width = parent.get_border_width(state).get_value(parent_width);

        // Padding
        let parent_padding_left = parent.get_padding_left(state).get_value(parent_width);
        let parent_padding_right = parent.get_padding_right(state).get_value(parent_width);
        let parent_padding_top = parent.get_padding_top(state).get_value(parent_height);
        let parent_padding_bottom = parent.get_padding_bottom(state).get_value(parent_height);

        let parent_posx = state.data.get_posx(*parent) + parent_padding_left + parent_border_width;
        let parent_posy = state.data.get_posy(*parent) + parent_padding_top + parent_border_width;

        //TEMP - Move to the walk up phase
        let mut num_children = 0;
        for _ in parent.child_iter(&tree) {
            num_children += 1;
        }

        let parent_flex_direction = state
            .style
            .flex_direction
            .get(*parent)
            .cloned()
            .unwrap_or_default();

        let justify_content = state
            .style
            .justify_content
            .get(*parent)
            .cloned()
            .unwrap_or_default();

        let align_items = state
            .style
            .align_items
            .get(*parent)
            .cloned()
            .unwrap_or_default();

        let mut child_grow_sum = state.data.get_child_grow_sum(*parent);
        let mut child_shrink_sum = state.data.get_child_shrink_sum(*parent);

        //let mut current_pos = 0.0;
        let mut space_per_widget = 0.0;

        // Move to positioning phase?
        let mut free_space = match parent_flex_direction {
            FlexDirection::Row => {
                parent_width
                    - parent_padding_left
                    - parent_padding_right
                    - 2.0 * parent_border_width
                    - state.data.get_child_sum(*parent)
            }

            FlexDirection::Column => {
                parent_height
                    - parent_padding_top
                    - parent_padding_bottom
                    - 2.0 * parent_border_width
                    - state.data.get_child_sum(*parent)
            }
        };

        let mut child_main_sum = free_space;
        
        // Used to calculate the positional proportion of the child
        let mut proportion_numerator = 0.0f32;
        let mut flex_length_sum = 0.0f32;

        // Sort child elements so that inflexible ones are iterated first
        let mut children = parent.child_iter(&tree).collect::<Vec<Entity>>();
        children.sort_by_cached_key(|child| child.get_flex_grow(state) != 0.0);


        for child in children.into_iter() {

            // Skip non-displayed widgets
            let display = state.style.display.get(child).cloned().unwrap_or_default();

            if display == Display::None {
                continue;
            }

            // Get the desired width and height
            let width = state.style.width.get(child).cloned().unwrap_or_default();
            let height = state.style.height.get(child).cloned().unwrap_or_default();

            let child_border_width = child.get_border_width(state).get_value(parent_width);

            // Size Constraints
            let child_min_width = child.get_min_width(state).get_value(parent_width);
            let child_max_width = match child.get_max_width(state) {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_width * val,
                _ => std::f32::INFINITY,
            };
            let child_min_height = child.get_min_height(state).get_value(parent_width);
            let child_max_height = match child.get_max_height(state) {
                Length::Pixels(val) => val,
                Length::Percentage(val) => parent_height * val,
                _ => std::f32::INFINITY,
            };

            // Margins
            let child_margin_left = child.get_margin_left(state).get_value(parent_width);
            let child_margin_right = child.get_margin_right(state).get_value(parent_width);
            let child_margin_top = child.get_margin_top(state).get_value(parent_height);
            let child_margin_bottom = child.get_margin_bottom(state).get_value(parent_height);

            // Padding
            let child_padding_left = child.get_padding_left(state).get_value(parent_width);
            let child_padding_right = child.get_padding_right(state).get_value(parent_width);
            let child_padding_top = child.get_padding_top(state).get_value(parent_height);
            let child_padding_bottom = child.get_padding_bottom(state).get_value(parent_height);

            // The new size and position of the child
            let mut new_width;
            let mut new_height;

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



            if child_grow_sum < 1.0 {
                child_grow_sum = 1.0;
            }

            if child_shrink_sum < 1.0 {
                child_shrink_sum = 1.0;
            }

            let flex_grow_fraction = child_flex_grow / child_grow_sum;
            let flex_shrink_fraction = child_flex_shrink / child_shrink_sum;

            proportion_numerator += child_flex_grow;
            let positional_proportion = proportion_numerator / child_grow_sum;

            let position = state.style.position.get(child).cloned().unwrap_or_default();

            // Set width and height based on sizes of child elements
            match flex_direction {
                FlexDirection::Row => {
                    new_width = state.data.get_child_sum(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.data.get_child_max(child)
                        + child_padding_top
                        + child_padding_bottom
                        + 2.0 * child_border_width;
                }

                FlexDirection::Column => {
                    new_width = state.data.get_child_max(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.data.get_child_sum(child)
                        + child_padding_top
                        + child_padding_bottom
                        + 2.0 * child_border_width;
                }
            }

            //println!("Child: {}  Width: {}  Height: {}", child, new_width, new_height);

            match position {
                Position::Relative => {
                    match parent_flex_direction {
                        FlexDirection::Row => {
                            //new_posx = current_pos;

                            // Set width if specified
                            match width {
                                // Width specified in pixels
                                Length::Pixels(val) => {
                                    new_width = val
                                        + child_padding_left
                                        + child_padding_right
                                        + 2.0 * child_border_width;
                                }

                                // Width specified as a percentage
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

                            // Flex basis overrides width
                            if let Some(flex_basis) = state.style.flex_basis.get(child) {
                                new_width = *flex_basis
                                    + child_padding_left
                                    + child_padding_right
                                    + 2.0 * child_border_width;
                            }

                            //let parent_free_space = free_space - (new_width + child_margin_left + child_margin_right);

                            // println!("Child: {}  Free Space: {}", child, free_space);



                            if free_space >= 0.0 {
                                // let flex_width = (proportion_numerator * free_space
                                //     / child_grow_sum)
                                //     .round()
                                //     - flex_length_sum;
                                // flex_length_sum += flex_width;
                                // new_width += flex_width;
                                new_width += flex_grow_fraction * free_space;
                            } else {
                                new_width += new_width * (flex_shrink_fraction * free_space);
                            }

                            child_grow_sum -= child_flex_grow;
                            child_shrink_sum -= child_flex_shrink;

                            child_grow_sum.max(0.0);
                            child_shrink_sum.max(0.0);


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
                            new_width = new_width.clamp(child_min_width, child_max_width);
                            new_height = new_height.clamp(child_min_height, child_max_height);

                            //free_space -= new_width + child_margin_left + child_margin_right;
                        }

                        FlexDirection::Column => {
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
                                new_height = *flex_basis
                                    + child_padding_top
                                    + child_padding_bottom
                                    + 2.0 * child_border_width;
                            }

                            //let parent_free_space = free_space - new_height;
                                // + child_margin_top + child_margin_bottom);

                            //println!("Child: {} Free Space: {}", child, parent_free_space);

                            if free_space >= 0.0 {
                                // let flex_height = (proportion_numerator * free_space
                                //     / child_grow_sum)
                                //     .round()
                                //     - flex_length_sum;
                                // flex_length_sum += flex_height;
                                // new_height += flex_height;

                                new_height += flex_grow_fraction * free_space;
                            } else {
                                new_height += flex_shrink_fraction * free_space;
                            }

                            //println!("Child: {} Height: {}, Free Space: {}", child, new_height, free_space);

                            child_grow_sum -= child_flex_grow;
                            child_shrink_sum -= child_flex_shrink;

                            child_grow_sum.max(0.0);
                            child_shrink_sum.max(0.0);

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
                            new_width = new_width.clamp(child_min_width, child_max_width);
                            new_height = new_height.clamp(child_min_height, child_max_height);

                            //free_space -= new_height + child_margin_top + child_margin_bottom;
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
                }
            }

            //println!("DOWN  Entity: {} Width: {}  Height: {}", child, new_width, new_height);

            let mut geometry_changed = GeometryChanged::default();


            if state.data.get_width(child) != new_width {
                state.data.set_width(child, new_width);
                should_continue = true;
                geometry_changed.width = true;
            }

            if state.data.get_height(child) != new_height {
                state.data.set_height(child, new_height);
                should_continue = true;
                geometry_changed.height = true;
            }

            // if !should_continue {
            //     // if let Some(ns) = tree.get_next_sibling(*parent) {
            //     //     next_sibling = ns;
            //     //     tree_down_iterator = next_sibling.into_iter(tree);
            //     // }
            // } else {
            if geometry_changed.width || geometry_changed.height {
                state.insert_event(
                    Event::new(WindowEvent::GeometryChanged(geometry_changed))
                        .target(child)
                        .propagate(Propagation::Down),
                );                
            }

                //should_continue = false;
            //}
        }


        let mut current_pos = 0.0;

        match justify_content {
            JustifyContent::FlexStart => current_pos = 0.0,
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

        // Position Children
        // This step has to de done after children have been sized due to percentage sized children
        for child in parent.child_iter(&tree) {
            let mut new_posx = 0.0;
            let mut new_posy = 0.0;

            let position = child.get_position(state);

            let child_margin_left = child.get_margin_left(state).get_value(parent_width);
            let child_margin_right = child.get_margin_right(state).get_value(parent_width);
            let child_margin_top = child.get_margin_top(state).get_value(parent_height);
            let child_margin_bottom = child.get_margin_bottom(state).get_value(parent_height);

            let left = child.get_left(state);
            let right = child.get_right(state);
            let top = child.get_top(state);
            let bottom = child.get_bottom(state);

            let new_width = state.data.get_width(child);
            let new_height = state.data.get_height(child);

            match position {

                Position::Relative => {
                    match parent_flex_direction {
                        FlexDirection::Row => {
                            new_posx = current_pos;

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

                            current_pos += new_width
                                + space_per_widget
                                + child_margin_left
                                + child_margin_right;
                        }

                        FlexDirection::Column => {

                            new_posy = current_pos;

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

                            current_pos += new_height
                                + space_per_widget
                                + child_margin_top
                                + child_margin_bottom;
                        }
                    }
                }

                Position::Absolute => {


                    new_posx = parent_posx;
                    new_posy = parent_posy;

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
                }
            }

            let mut geometry_changed = GeometryChanged::default();

            if state.data.get_posx(child) != new_posx {
                state.data.set_posx(child, new_posx);
                should_continue = true;
                geometry_changed.posx = true;
            }

            if state.data.get_posy(child) != new_posy {
                state.data.set_posy(child, new_posy);
                should_continue = true;
                geometry_changed.posy = true;
            }

            // if !should_continue {
            //     // if let Some(ns) = tree.get_next_sibling(*parent) {
            //     //     next_sibling = ns;
            //     //     tree_down_iterator = next_sibling.into_iter(tree);
            //     // }
            // } else {
            if geometry_changed.posx || geometry_changed.posy {
                state.insert_event(
                    Event::new(WindowEvent::GeometryChanged(geometry_changed))
                        .target(child)
                        .propagate(Propagation::Down),
                );                    
            }

                //should_continue = false;
            //}

        }
    }
}






pub fn apply_layout2(state: &mut State, tree: &Tree) {
    //println!("Relayout");
    // Reset
    for entity in tree.entities.iter() {
        state.data.set_child_sum(*entity, 0.0);
        state.data.set_child_pos(*entity, 0.0);
        state.data.set_child_grow_sum(*entity, 0.0);
        state.data.set_child_shrink_sum(*entity, 0.0);
    }

    //let mut tree_up_iterator = tree.entities.iter();

    let layout_tree = tree.into_iter().collect::<Vec<Entity>>();
    //let layout_tree = tree.entities.clone();

    //////////////////////
    // Walk up the tree //
    //////////////////////
    //while let Some(entity) = layout_tree.iter().next_back() {
    for entity in layout_tree.iter().rev() {
        // Stop before the window
        if *entity == Entity::root() {
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

        let parent = tree.get_parent(*entity).unwrap();

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        //println!("Entity: {} Parent: {} Width: {} Height: {}", entity, parent, parent_width, parent_height);

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

        // Margins
        let child_margin_left = entity.get_margin_left(state).get_value(parent_width);
        let child_margin_right = entity.get_margin_right(state).get_value(parent_width);
        let child_margin_top = entity.get_margin_top(state).get_value(parent_width);
        let child_margin_bottom = entity.get_margin_bottom(state).get_value(parent_width);

        // let child_margin_left = match state
        //     .style
        //     .margin_left
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let child_margin_right = match state
        //     .style
        //     .margin_right
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let child_margin_top = match state
        //     .style
        //     .margin_top
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        // let child_margin_bottom = match state
        //     .style
        //     .margin_bottom
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        // Padding
        let child_padding_left = entity.get_padding_left(state).get_value(parent_width);
        let child_padding_right = entity.get_padding_right(state).get_value(parent_width);
        let child_padding_top = entity.get_padding_top(state).get_value(parent_width);
        let child_padding_bottom = entity.get_padding_bottom(state).get_value(parent_width);

        // let child_padding_left = match state
        //     .style
        //     .padding_left
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let child_padding_right = match state
        //     .style
        //     .padding_right
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let child_padding_top = match state
        //     .style
        //     .padding_top
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        // let child_padding_bottom = match state
        //     .style
        //     .padding_bottom
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        let child_border_width = entity.get_border_width(state).get_value(parent_width);

        // let child_border_width = match state
        //     .style
        //     .border_width
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let parent_border_width = match state
        //     .style
        //     .border_width
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let parent_padding_left = match state
        //     .style
        //     .padding_left
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let parent_padding_right = match state
        //     .style
        //     .padding_right
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_width * val,
        //     _ => 0.0,
        // };

        // let parent_padding_top = match state
        //     .style
        //     .padding_top
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        // let parent_padding_bottom = match state
        //     .style
        //     .padding_bottom
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default()
        // {
        //     Length::Pixels(val) => val,
        //     Length::Percentage(val) => parent_height * val,
        //     _ => 0.0,
        // };

        let child_flex_direction = entity.get_flex_direction(state);
        let parent_flex_direction = parent.get_flex_direction(state);

        // let parent_flex_direction = state
        //     .style
        //     .flex_direction
        //     .get(parent)
        //     .cloned()
        //     .unwrap_or_default();
        // let child_flex_direction = state
        //     .style
        //     .flex_direction
        //     .get(*entity)
        //     .cloned()
        //     .unwrap_or_default();

        // Get the desired width from the style
        //let width = state.style.width.get(*entity).cloned().unwrap_or_default();
        let width = entity.get_width(state);

        // Get the desired height from the style
        //let height = state.style.height.get(*entity).cloned().unwrap_or_default();
        let height = entity.get_height(state);

        let mut new_width;
        let mut new_height;

        match child_flex_direction {
            FlexDirection::Row => {
                // Set width to the sum of the widths of the children
                new_width = state.data.get_child_sum(*entity);
                // Set height to the maximum height of the children
                new_height = state.data.get_child_max(*entity);
            }

            FlexDirection::Column => {
                // Set width to the maximum width of the children
                new_width = state.data.get_child_max(*entity);
                // Set height to the maximum height of the children
                new_height = state.data.get_child_sum(*entity);
            }
        }

        match parent_flex_direction {
            FlexDirection::Row => {
                // Start with desired width if specified in pixels
                match width {
                    Length::Pixels(val) => {
                        new_width = val;
                    }

                    // Length::Percentage(val) => {
                    //     new_width = (parent_width
                    //         - parent_padding_left
                    //         - parent_padding_right
                    //         - 2.0 * parent_border_width)
                    //         * val;
                    // }
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
                        state
                            .data
                            .set_child_sum(parent, state.data.get_child_sum(parent) + new_width);
                        //state.data.set_child_max(parent, new_height);
                        state.data.set_child_max(
                            parent,
                            new_height.max(state.data.get_child_max(parent)),
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
                        new_height = (parent_height)
                            // - parent_padding_top
                            // - parent_padding_bottom
                            // - 2.0 * parent_border_width)
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
                        state
                            .data
                            .set_child_sum(parent, state.data.get_child_sum(parent) + new_height);
                        //state.data.set_child_max(parent, new_width);
                        state
                            .data
                            .set_child_max(parent, new_width.max(state.data.get_child_max(parent)));
                    }

                    _ => {}
                }
            }
        }

        if let Some(flex_grow) = state.style.flex_grow.get(*entity) {
            state
                .data
                .set_child_grow_sum(parent, state.data.get_child_grow_sum(parent) + flex_grow);
        }

        if let Some(flex_shrink) = state.style.flex_shrink.get(*entity) {
            state.data.set_child_shrink_sum(
                parent,
                state.data.get_child_shrink_sum(parent) + flex_shrink,
            );
        }
    }

    let mut should_continue = false;

    ////////////////////////
    // Walk down the tree //
    ////////////////////////
    //while let Some(parent) = tree_down_iterator.next() {
    for parent in layout_tree.iter() {
        // Parent properties

        let parent_width = state.data.get_width(*parent);
        let parent_height = state.data.get_height(*parent);

        //println!("Parent: {} Width: {:?} Height: {:?}", parent, parent_width, parent_height);

        let parent_border_width = match state
            .style
            .border_width
            .get(*parent)
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
            .get(*parent)
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
            .get(*parent)
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
            .get(*parent)
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
            .get(*parent)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let parent_posx = state.data.get_posx(*parent) + parent_padding_left + parent_border_width;
        let parent_posy = state.data.get_posy(*parent) + parent_padding_top + parent_border_width;

        //TEMP - Move to the walk up phase
        let mut num_children = 0;
        for _ in parent.child_iter(&tree) {
            num_children += 1;
        }

        let parent_flex_direction = state
            .style
            .flex_direction
            .get(*parent)
            .cloned()
            .unwrap_or_default();

        let justify_content = state
            .style
            .justify_content
            .get(*parent)
            .cloned()
            .unwrap_or_default();

        let mut current_pos = 0.0;
        let mut space_per_widget = 0.0;

        let free_space = match parent_flex_direction {
            FlexDirection::Row => {
                state.data.get_width(*parent)
                    - parent_padding_left
                    - parent_padding_right
                    - 2.0 * parent_border_width
                    - state.data.get_child_sum(*parent)
            }

            FlexDirection::Column => {
                state.data.get_height(*parent)
                    - parent_padding_top
                    - parent_padding_bottom
                    - 2.0 * parent_border_width
                    - state.data.get_child_sum(*parent)
            }
        };

        match justify_content {
            JustifyContent::FlexStart => current_pos = 0.0,
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

        // Used to calculate the positional proportion of the child
        let mut proportion_numerator = 0.0f32;
        let mut flex_length_sum = 0.0f32;

        for child in parent.child_iter(&tree) {
            // Skip non-displayed widgets
            let display = state.style.display.get(child).cloned().unwrap_or_default();

            if display == Display::None {
                continue;
            }

            // Get the desired width and height
            let width = state.style.width.get(child).cloned().unwrap_or_default();
            let height = state.style.height.get(child).cloned().unwrap_or_default();

            // if *parent == Entity::new(1) {
            //     println!("Child: {} Width: {:?} Height: {:?}", child, width, height);
            // }

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

            let mut child_grow_sum = state.data.get_child_grow_sum(*parent);
            let mut child_shrink_sum = state.data.get_child_shrink_sum(*parent);

            if child_grow_sum < 1.0 {
                child_grow_sum = 1.0;
            }

            if child_shrink_sum < 1.0 {
                child_shrink_sum = 1.0;
            }

            let flex_grow_fraction = child_flex_grow / child_grow_sum;
            let flex_shrink_fraction = child_flex_shrink / child_shrink_sum;

            proportion_numerator += child_flex_grow;
            let positional_proportion = proportion_numerator / child_grow_sum;

            let position = state.style.position.get(child).cloned().unwrap_or_default();

            match flex_direction {
                FlexDirection::Row => {
                    new_width = state.data.get_child_sum(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.data.get_child_max(child)
                        + child_padding_top
                        + child_padding_bottom
                        + 2.0 * child_border_width;
                }

                FlexDirection::Column => {
                    new_width = state.data.get_child_max(child)
                        + child_padding_left
                        + child_padding_right
                        + 2.0 * child_border_width;
                    new_height = state.data.get_child_sum(child)
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
                                new_width = *flex_basis
                                    + child_padding_left
                                    + child_padding_right
                                    + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_width
                                - parent_padding_left
                                - parent_padding_right
                                - parent_border_width
                                - parent_border_width
                                - state.data.get_child_sum(*parent);

                            if parent_free_space >= 0.0 {
                                let flex_width = (proportion_numerator * parent_free_space
                                    / child_grow_sum)
                                    .round()
                                    - flex_length_sum;
                                flex_length_sum += flex_width;
                                new_width += flex_width;
                            //new_width += flex_grow_fraction * parent_free_space;
                            } else {
                                new_width += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(*parent)
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

                            // state.data.set_child_sum(
                            //     *parent,
                            //     state.data.get_child_sum(*parent) + new_width,
                            // );
                            // //state.data.set_child_max(parent, new_height);
                            // state.data.set_child_max(
                            //     *parent,
                            //     new_height.max(state.data.get_child_max(*parent)),
                            // );

                            //current_pos -= new_width;

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

                            // state.data.set_posy(
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
                                new_height = *flex_basis
                                    + child_padding_top
                                    + child_padding_bottom
                                    + 2.0 * child_border_width;
                            }

                            let parent_free_space = parent_height
                                - parent_padding_top
                                - parent_padding_bottom
                                - parent_border_width
                                - parent_border_width
                                - state.data.get_child_sum(*parent);

                            if parent_free_space >= 0.0 {
                                let flex_height = (proportion_numerator * parent_free_space
                                    / child_grow_sum)
                                    .round()
                                    - flex_length_sum;
                                flex_length_sum += flex_height;
                                new_height += flex_height;
                            //new_height += flex_grow_fraction * parent_free_space;
                            } else {
                                new_height += flex_shrink_fraction * parent_free_space;
                            }

                            let align_items = state
                                .style
                                .align_items
                                .get(*parent)
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

                            // state.data.set_child_sum(
                            //     *parent,
                            //     state.data.get_child_sum(*parent) + new_height,
                            // );

                            // state.data.set_child_max(
                            //     *parent,
                            //     new_width.max(state.data.get_child_max(*parent)),
                            // );

                            //current_pos -= new_height;

                            //state.data.set_width(child, new_width);
                            //state.data.set_height(child, new_height);

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
                            //     .data
                            //     .set_posy(child, parent_posy + new_posy + child_margin_top);

                            //let align_items = state.style.align_items.get(*parent).cloned().unwrap_or_default();

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
                            //     .data
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

                    //state.data.set_width(child, new_width);
                    //state.data.set_height(child, new_height);

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

                    //state.data.set_posx(child, new_posx);
                    //state.data.set_posy(child, new_posy);
                }
            }

            let mut geometry_changed = GeometryChanged::default();

            if state.data.get_posx(child) != new_posx {
                state.data.set_posx(child, new_posx);
                should_continue = true;
                geometry_changed.posx = true;
            }

            if state.data.get_posy(child) != new_posy {
                state.data.set_posy(child, new_posy);
                should_continue = true;
                geometry_changed.posy = true;
            }

            if state.data.get_width(child) != new_width {
                state.data.set_width(child, new_width);
                should_continue = true;
                geometry_changed.width = true;
            }

            if state.data.get_height(child) != new_height {
                state.data.set_height(child, new_height);
                should_continue = true;
                geometry_changed.height = true;
            }

            if !should_continue {
                // if let Some(ns) = tree.get_next_sibling(*parent) {
                //     next_sibling = ns;
                //     tree_down_iterator = next_sibling.into_iter(tree);
                // }
            } else {
                state.insert_event(
                    Event::new(WindowEvent::GeometryChanged(geometry_changed))
                        .target(child)
                        .propagate(Propagation::Down),
                );
                should_continue = false;
            }
        }

        // Set the data properties
    }
}
*/
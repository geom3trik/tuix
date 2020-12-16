use crate::{Entity, State};

use crate::hierarchy::*;
use crate::style::*;

use crate::flexbox::AlignItems;

pub fn apply_z_ordering(state: &mut State, hierarchy: &Hierarchy) {
    //println!("Z ORDERING");
    for entity in hierarchy.into_iter() {

        if entity == Entity::new(0,0) {
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

pub fn layout_fun(state: &mut State, hierarchy: &Hierarchy) {
    // Reset
    for entity in hierarchy.entities.iter() {
        state.transform.set_child_sum(*entity, 0.0);
        state.transform.set_child_pos(*entity, 0.0);
        state.transform.set_child_grow_sum(*entity, 0.0);
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

        let min_width = match state
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

        let max_width = match state
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

        let min_height = match state
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

        let max_height = match state
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

        let margin_left = match state.style.margin_left.get(*entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _=> 0.0,
        };

        let margin_right = match state.style.margin_right.get(*entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _=> 0.0,
        };

        let margin_top = match state.style.margin_top.get(*entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _=> 0.0,
        };

        let margin_bottom = match state.style.margin_bottom.get(*entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _=> 0.0,
        };

        let border_width = state
        .style
        .border_width
        .get(*entity)
        .cloned()
        .unwrap_or_default();

        let parent_flex_direction = state
            .style
            .flex_direction
            .get(parent)
            .cloned()
            .unwrap_or_default();
        let flex_direction = state
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

        match flex_direction {
            FlexDirection::Row => {
                new_width = state.transform.get_child_sum(*entity);
                new_height = state.transform.get_child_max(*entity);
            }

            FlexDirection::Column => {
                new_width = state.transform.get_child_max(*entity);
                new_height = state.transform.get_child_sum(*entity);
            }
        }

        match parent_flex_direction {
            FlexDirection::Row => {
                match width {
                    Length::Pixels(val) => {
                        new_width = val;
                    }

                    _ => {}
                };

                if let Some(flex_basis) = state.style.flex_basis.get(*entity) {
                    new_width = *flex_basis;
                }

                match height {
                    Length::Pixels(val) => {
                        new_height = val;
                    }

                    _ => {}
                }

                if new_width < min_width {
                    new_width = min_width;
                }

                if new_width > max_width {
                    new_width = max_width;
                }

                if new_height < min_height {
                    new_height = min_height;
                }

                if new_height > max_height {
                    new_height = max_height;
                }

                new_width += margin_left + margin_right - border_width;
                new_height += margin_top + margin_bottom - border_width;

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

                if new_width < min_width {
                    new_width = min_width;
                }

                if new_width > max_width {
                    new_width = max_width;
                }

                if new_height < min_height {
                    new_height = min_height;
                }

                if new_height > max_height {
                    new_height = max_height;
                }

                new_width += margin_left + margin_right - border_width;
                new_height += margin_top + margin_bottom - border_width;

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
    }

    ////////////////////////
    // Walk down the tree //
    ////////////////////////
    for parent in hierarchy.into_iter() {
        // Parent properties



        let parent_border_width = state
        .style
        .border_width
        .get(parent)
        .cloned()
        .unwrap_or_default();


        let parent_width = state.transform.get_width(parent) - parent_border_width;
        let parent_height = state.transform.get_height(parent) - parent_border_width;

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


        let parent_posx = state.transform.get_posx(parent) + parent_padding_left + (parent_border_width / 2.0);
        let parent_posy = state.transform.get_posy(parent) + parent_padding_top + (parent_border_width / 2.0);

        //TEMP
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
                state.transform.get_width(parent) - state.transform.get_child_sum(parent)
            }

            FlexDirection::Column => {
                state.transform.get_height(parent) - state.transform.get_child_sum(parent)
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

        for (index, child) in parent.child_iter(&hierarchy).enumerate() {

            // Skip non-displayed widgets
            let display = state
            .style
            .display
            .get(child)
            .cloned()
            .unwrap_or_default();
        
            if display == Display::None {
                continue;
            }


            // Get the desired width and height
            let width = state.style.width.get(child).cloned().unwrap_or_default();
            let height = state.style.height.get(child).cloned().unwrap_or_default();

            let left = state.style.left.get(child).cloned().unwrap_or_default();
            let top = state.style.top.get(child).cloned().unwrap_or_default();

            let child_border_width = state
                .style
                .border_width
                .get(child)
                .cloned()
                .unwrap_or_default();

            let min_width = match state
                .style
                .min_width
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => 0.0,
            };

            let max_width = match state
                .style
                .max_width
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => std::f32::INFINITY,
            };

            let min_height = match state
                .style
                .min_height
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => 0.0,
            };

            let max_height = match state
                .style
                .max_height
                .get(child)
                .cloned()
                .unwrap_or_default()
            {
                Length::Pixels(val) => val,
                _ => std::f32::INFINITY,
            };

            let margin_left = match state
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

            let margin_right = match state
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

            let margin_top = match state
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

            let margin_bottom = match state
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

            let mut new_width;
            let mut new_height;
            let mut new_posx = 0.0;
            let mut new_posy = 0.0;

            let flex_direction = state
                .style
                .flex_direction
                .get(child)
                .cloned()
                .unwrap_or_default();

            let flex_grow = state
                .style
                .flex_grow
                .get(child)
                .cloned()
                .unwrap_or_default();
            let mut child_grow_sum = state.transform.get_child_grow_sum(parent);

            if child_grow_sum == 0.0 {
                child_grow_sum = 1.0;
            }

            let flex_fraction = flex_grow / child_grow_sum;

            let position = state.style.position.get(child).cloned().unwrap_or_default();

            match flex_direction {
                FlexDirection::Row => {
                    new_width = state.transform.get_child_sum(child) + child_padding_left + child_padding_right;
                    new_height = state.transform.get_child_max(child);
                }

                FlexDirection::Column => {
                    new_width = state.transform.get_child_max(child);
                    new_height = state.transform.get_child_sum(child) + child_padding_top + child_padding_bottom;
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
                                }

                                Length::Percentage(val) => {
                                    new_width = parent_width * val;
                                }
                                _ => {}
                            };

                            if let Some(flex_basis) = state.style.flex_basis.get(child) {
                                new_width = *flex_basis;
                            }

                            let parent_free_space = parent_width
                                - parent_padding_left
                                - parent_padding_right
                                - state.transform.get_child_sum(parent);

                            new_width +=
                                flex_fraction * parent_free_space;
                            
                            

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
                                        - margin_top
                                        - margin_bottom;
                                }
                            } else {
                                if align_items == AlignItems::Stretch {
                                    new_height = parent_height
                                        - parent_padding_top
                                        - parent_padding_bottom
                                        - margin_top
                                        - margin_bottom;
                                }
                            }

                            match height {
                                Length::Pixels(val) => {
                                    new_height = val;
                                }

                                Length::Percentage(val) => {
                                    new_height = parent_height * val;
                                }

                                _ => {}
                            };

                            // Apply size contraints
                            if new_width < min_width {
                                new_width = min_width;
                            }

                            if new_width > max_width {
                                new_width = max_width;
                            }

                            if new_height < min_height {
                                new_height = min_height;
                            }

                            if new_height > max_height {
                                new_height = max_height;
                            }

                            new_width = new_width - child_border_width;
                            new_height = new_height - child_border_width;

                            // if index % 2 == 0 {
                            //     new_width = new_width.floor();
                            //     new_height = new_height.floor();
                            // } else {
                            //     new_width = new_width.ceil();
                            //     new_height = new_height.ceil();
                            // }

                            state.transform.set_width(child, new_width);
                            state.transform.set_height(child, new_height);

                            match left {
                                Length::Pixels(val) => {
                                    //current_pos += val;
                                    new_posx = current_pos + val;
                                }

                                Length::Percentage(val) => {
                                    //current_pos += val * parent_width;
                                    new_posx = current_pos + (val * parent_width);
                                }

                                _ => {}
                            }

                            // Position
                            state.transform.set_posx(
                                child,
                                parent_posx + new_posx + margin_left + (child_border_width / 2.0),
                            );

                            //let align_items = state.style.align_items.get(parent).cloned().unwrap_or_default();

                            let mut new_posy = match align_items {
                                AlignItems::FlexStart => 0.0,
                                AlignItems::FlexEnd => parent_height - new_height,
                                AlignItems::Center => (parent_height - new_height) / 2.0,
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

                            state.transform.set_posy(
                                child,
                                parent_posy + new_posy + margin_top + (child_border_width / 2.0),
                            );

                            current_pos +=
                                new_width + space_per_widget + margin_left + margin_right;
                        }

                        FlexDirection::Column => {
                            new_posy = current_pos;

                            match height {
                                Length::Pixels(val) => {
                                    new_height = val;
                                }

                                Length::Percentage(val) => {
                                    new_height = parent_height * val;
                                }
                                _ => {}
                            };

                            if let Some(flex_basis) = state.style.flex_basis.get(child) {
                                new_height = *flex_basis;
                            }

                            let parent_free_space = parent_height
                                - parent_padding_top
                                - parent_padding_bottom
                                - state.transform.get_child_sum(parent);

                            new_height +=
                                flex_fraction * parent_free_space;

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
                                        - margin_left
                                        - margin_right;
                                }
                            } else {
                                if align_items == AlignItems::Stretch {
                                    new_width = parent_width
                                        - parent_padding_left
                                        - parent_padding_right
                                        - margin_left
                                        - margin_right;
                                }
                            }

                            match width {
                                Length::Pixels(val) => {
                                    new_width = val;
                                }

                                Length::Percentage(val) => {
                                    new_width = parent_width * val;
                                }

                                _ => {}
                            };

                            // Apply size contraints
                            if new_width < min_width {
                                new_width = min_width;
                            }

                            if new_width > max_width {
                                new_width = max_width;
                            }

                            if new_height < min_height {
                                new_height = min_height;
                            }

                            if new_height > max_height {
                                new_height = max_height;
                            }

                            new_width = new_width - child_border_width;
                            new_height = new_height - child_border_width;

                            state.transform.set_width(child, new_width);
                            state.transform.set_height(child, new_height);

                            match top {
                                Length::Pixels(val) => {
                                    new_posy = current_pos + val;
                                }

                                Length::Percentage(val) => {
                                    new_posy = current_pos + val * parent_height;
                                }

                                _ => {}
                            }

                            // Position
                            state.transform.set_posy(
                                child,
                                parent_posy + new_posy + margin_top + (child_border_width / 2.0),
                            );

                            //let align_items = state.style.align_items.get(parent).cloned().unwrap_or_default();

                            let mut new_posx = match align_items {
                                AlignItems::FlexStart => 0.0,
                                AlignItems::FlexEnd => parent_width - new_width,
                                AlignItems::Center => (parent_width - new_width) / 2.0,
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

                            state.transform.set_posx(
                                child,
                                parent_posx + new_posx + margin_left + (child_border_width / 2.0),
                            );

                            current_pos +=
                                new_height + space_per_widget + margin_top + margin_bottom;
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

                    let mut new_posx = parent_posx;
                    let mut new_posy = parent_posy;

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

                    state.transform.set_width(child, new_width);
                    state.transform.set_height(child, new_height);

                    match right {
                        Length::Pixels(val) => {
                            new_posx = parent_posx + parent_width - new_width - val;
                        }

                        Length::Percentage(val) => {
                            new_posx =
                                parent_posx + parent_width - new_width - (val * parent_width);
                        }

                        _ => {}
                    }

                    match left {
                        Length::Pixels(val) => {
                            new_posx = parent_posx + val;
                        }

                        Length::Percentage(val) => {
                            new_posx = parent_posx + (val * parent_width);
                        }

                        _ => {}
                    }

                    match bottom {
                        Length::Pixels(val) => {
                            new_posy = parent_posy + parent_height - new_height - val;
                        }

                        Length::Percentage(val) => {
                            new_posy =
                                parent_posy + parent_height - new_height - (val * parent_height);
                        }

                        _ => {}
                    }

                    match top {
                        Length::Pixels(val) => {
                            new_posy = parent_posy + val;
                        }

                        Length::Percentage(val) => {
                            new_posy = parent_posy + (val * parent_height);
                        }

                        _ => {}
                    }

                    state
                        .transform
                        .set_posx(child, new_posx + parent_border_width / 2.0);
                    state
                        .transform
                        .set_posy(child, new_posy + parent_border_width / 2.0);
                }
            }
        }
    }
}

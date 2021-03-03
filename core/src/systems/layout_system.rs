#![allow(dead_code)]

use prop::PropGet;

use crate::{Entity, GeometryChanged, State, Event, WindowEvent, Propagation};

use crate::hierarchy::*;
use crate::style::*;

use crate::flexbox::AlignItems;

pub fn apply_z_ordering(state: &mut State, hierarchy: &Hierarchy) {
    for entity in hierarchy.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        let parent = hierarchy.get_parent(entity).unwrap();

        if let Some(z_order) = state.style.z_order.get(entity) {
            state.data.set_z_order(entity, *z_order);
        } else {
            let parent_z_order = state.data.get_z_order(parent);
            state.data.set_z_order(entity, parent_z_order);
        }
    }
}

fn calculate_up(state: &mut State, child: Entity) -> (f32, f32) {
    // Safe to unwrap because every entity in the hierarchy has a parent except window which is skipped
    let parent = child.get_parent(state).unwrap();

    let parent_flex_direction = parent.get_flex_direction(state);

    // Child padding
    let child_padding_left = child.get_padding_left(state).get_value(0.0);
    let child_padding_right = child.get_padding_right(state).get_value(0.0);
    let child_padding_top = child.get_padding_top(state).get_value(0.0);
    let child_padding_bottom = child.get_padding_bottom(state).get_value(0.0);

    let child_border_width = child.get_border_width(state).get_value(0.0);

    let (
        child_padding_main_before,
        child_padding_main_after,
        child_padding_cross_before,
        child_padding_cross_after,
    ) = match parent_flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => (
            child_padding_left,
            child_padding_right,
            child_padding_top,
            child_padding_bottom,
        ),
        FlexDirection::Column | FlexDirection::ColumnReverse => (
            child_padding_top,
            child_padding_bottom,
            child_padding_left,
            child_padding_right,
        ),
    };

    //println!("child: {}  ppmb: {} ppma: {} ppcb: {}, ppca: {}", child, child_padding_main_before, child_padding_main_after, child_padding_cross_before, child_padding_cross_after);

    // Child size constraints
    let child_min_width = match child.get_min_width(state) {
        Length::Pixels(val) => val,
        _ => 0.0,
    };
    let child_max_width = match child.get_max_width(state) {
        Length::Pixels(val) => val,
        _ => std::f32::INFINITY,
    };
    let child_min_height = match child.get_min_height(state) {
        Length::Pixels(val) => val,
        _ => 0.0,
    };
    let child_max_height = match child.get_max_height(state) {
        Length::Pixels(val) => val,
        _ => std::f32::INFINITY,
    };

    let child_flex_direction = child.get_flex_direction(state);

    let mut new_main;
    let mut new_cross;

    if child_flex_direction == parent_flex_direction {
        new_main = state.data.get_child_sum(child);
        new_cross = state.data.get_child_max(child);
    } else {
        new_main = state.data.get_child_max(child);
        new_cross = state.data.get_child_sum(child);
    }

    //let child_position = child.get_position(state);

    // Add padding
    if state.style.flex_grow.get(child).is_none() {
        new_main += child_padding_main_before + child_padding_main_after + 2.0 * child_border_width;
        new_cross +=
            child_padding_cross_before + child_padding_cross_after + 2.0 * child_border_width;
    }

    //println!("New Main: {}, New Cross: {}", new_main, new_cross);

    let (main, cross) = match parent_flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => {
            (child.get_width(state), child.get_height(state))
        }
        FlexDirection::Column | FlexDirection::ColumnReverse => {
            (child.get_height(state), child.get_width(state))
        }
    };

    // A main specified in pixels overrides child sum
    match main {
        Length::Pixels(val) => new_main = val,

        _ => {}
    }

    // A cross specified in pixels overrides child max
    match cross {
        Length::Pixels(val) => new_cross = val,

        _ => {}
    }

    let child_flex_basis = child.get_flex_basis(state);

    // Flex basis overrides main
    match child_flex_basis {
        Length::Pixels(val) => new_main = val,
        _ => {}
    }

    let (min_main, max_main, min_cross, max_cross) = match parent_flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => (
            child_min_width,
            child_max_width,
            child_min_height,
            child_max_height,
        ),
        FlexDirection::Column | FlexDirection::ColumnReverse => (
            child_min_height,
            child_max_height,
            child_min_width,
            child_max_width,
        ),
    };

    // Apply size constraints
    new_main = new_main.clamp(min_main, max_main);
    new_cross = new_cross.clamp(min_cross, max_cross);

    // Main and Cross should be at least as big as padding + border
    // new_main = new_main.max(child_padding_main_before + child_padding_main_after + 2.0 * child_border_width);
    // new_cross = new_cross.max(child_padding_cross_before + child_padding_cross_after + 2.0 * child_border_width);

    (new_main, new_cross)
}

fn calculate_down(state: &mut State, child: Entity) -> (f32, f32) {
    // Safe to unwrap because every entity in the hierarchy has a parent except window which is skipped
    let parent = child.get_parent(state).unwrap();

    let parent_flex_direction = parent.get_flex_direction(state);
    let parent_align_items = parent.get_align_items(state);

    let parent_width = state.data.get_width(parent);
    let parent_height = state.data.get_height(parent);

    // It shoudl probably be grand-parent width and height
    let parent_padding_left = parent.get_padding_left(state).get_value(parent_width);
    let parent_padding_right = parent.get_padding_right(state).get_value(parent_width);
    let parent_padding_top = parent.get_padding_top(state).get_value(parent_height);
    let parent_padding_bottom = parent.get_padding_bottom(state).get_value(parent_height);

    let parent_border_width = parent.get_border_width(state).get_value(parent_width);

    let (parent_main, parent_cross) = match parent_flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => (
            parent_width - parent_padding_left - parent_padding_right - 2.0 * parent_border_width,
            parent_height - parent_padding_top - parent_padding_bottom - 2.0 * parent_border_width,
        ),
        FlexDirection::Column | FlexDirection::ColumnReverse => (
            parent_height - parent_padding_top - parent_padding_bottom - 2.0 * parent_border_width,
            parent_width - parent_padding_left - parent_padding_right - 2.0 * parent_border_width,
        ),
    };

    // Child padding
    let child_padding_left = child.get_padding_left(state).get_value(parent_width);
    let child_padding_right = child.get_padding_right(state).get_value(parent_width);
    let child_padding_top = child.get_padding_top(state).get_value(parent_height);
    let child_padding_bottom = child.get_padding_bottom(state).get_value(parent_height);

    let (
        child_padding_main_before,
        child_padding_main_after,
        child_padding_cross_before,
        child_padding_cross_after,
    ) = match parent_flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => (
            child_padding_left,
            child_padding_right,
            child_padding_top,
            child_padding_bottom,
        ),
        FlexDirection::Column | FlexDirection::ColumnReverse => (
            child_padding_top,
            child_padding_bottom,
            child_padding_left,
            child_padding_right,
        ),
    };

    // Child margins
    let child_margin_left = child.get_margin_left(state).get_value(parent_width);
    let child_margin_right = child.get_margin_right(state).get_value(parent_width);
    let child_margin_top = child.get_margin_top(state).get_value(parent_height);
    let child_margin_bottom = child.get_margin_bottom(state).get_value(parent_height);

    let (
        child_margin_main_before,
        child_margin_main_after,
        child_margin_cross_before,
        child_margin_cross_after,
    ) = match parent_flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => (
            child_margin_left,
            child_margin_right,
            child_margin_top,
            child_margin_bottom,
        ),
        FlexDirection::Column | FlexDirection::ColumnReverse => (
            child_margin_top,
            child_margin_bottom,
            child_margin_left,
            child_margin_right,
        ),
    };

    let child_border_width = child.get_border_width(state).get_value(parent_width);

    // Child size constraints
    let child_min_width = child.get_min_width(state).get_value_or(parent_width, 0.0);
    let child_max_width = child
        .get_max_width(state)
        .get_value_or(parent_width, std::f32::INFINITY);
    let child_min_height = child.get_min_height(state).get_value_or(parent_height, 0.0);
    let child_max_height = child
        .get_max_height(state)
        .get_value_or(parent_height, std::f32::INFINITY);

    let (min_main, max_main, min_cross, max_cross) = match parent_flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => (
            child_min_width,
            child_max_width,
            child_min_height,
            child_max_height,
        ),
        FlexDirection::Column | FlexDirection::ColumnReverse => (
            child_min_height,
            child_max_height,
            child_min_width,
            child_max_width,
        ),
    };

    let child_flex_direction = child.get_flex_direction(state);

    let mut new_main;
    let mut new_cross;

    if child_flex_direction == parent_flex_direction {
        new_main = state.data.get_child_sum(child);
        new_cross = state.data.get_child_max(child);
    } else {
        new_main = state.data.get_child_max(child);
        new_cross = state.data.get_child_sum(child);
    }

    // Add padding
    if state.style.flex_grow.get(child).is_none() {
        new_main += child_padding_main_before + child_padding_main_after + 2.0 * child_border_width;
        new_cross +=
            child_padding_cross_before + child_padding_cross_after + 2.0 * child_border_width;
    }

    let child_position = child.get_position(state);

    match child_position {
        Position::Relative => {
            let (main, cross) = match parent_flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    (child.get_width(state), child.get_height(state))
                }
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    (child.get_height(state), child.get_width(state))
                }
            };

            // A main specified in pixels overrides child sum
            match main {
                Length::Pixels(val) => new_main = val,

                Length::Percentage(val) => new_main = parent_main * val,

                _ => {}
            }

            let child_flex_basis = child.get_flex_basis(state);

            // Flex basis overrides main
            match child_flex_basis {
                Length::Pixels(val) => new_main = val,
                Length::Percentage(val) => new_main = parent_main * val,
                _ => {}
            }

            // Align stretch overrides child max
            if let Some(child_align_self) = state.style.align_self.get(child) {
                if *child_align_self == AlignSelf::Stretch {
                    new_cross = parent_cross - child_margin_cross_before - child_margin_cross_after;
                }
            } else {
                if parent_align_items == AlignItems::Stretch {
                    new_cross = parent_cross - child_margin_cross_before - child_margin_cross_after;
                }
            }

            // A cross specified in pixels overrides align stretch
            match cross {
                Length::Pixels(val) => new_cross = val,

                Length::Percentage(val) => new_cross = parent_cross * val,

                _ => {}
            }
        }

        Position::Absolute => {
            let (mut new_width, mut new_height) = match parent_flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => (new_main, new_cross),

                FlexDirection::Column | FlexDirection::ColumnReverse => (new_cross, new_main),
            };

            // let mut new_width = 0.0;
            // let mut new_height = 0.0;

            let width = child.get_width(state);
            let height = child.get_height(state);

            let left = child.get_left(state);
            let right = child.get_right(state);
            let top = child.get_top(state);
            let bottom = child.get_bottom(state);

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

            match parent_flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    new_main = new_width;
                    new_cross = new_height;
                }

                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    new_main = new_height;
                    new_cross = new_width;
                }
            }
        }
    }

    // Apply size constraints
    new_main = new_main.clamp(min_main, max_main);
    new_cross = new_cross.clamp(min_cross, max_cross);

    // new_main = new_main.max(child_padding_main_before + child_padding_main_after + 2.0 * child_border_width);
    // new_cross = new_cross.max(child_padding_cross_before + child_padding_cross_after + 2.0 * child_border_width);

    (new_main, new_cross)
}

pub fn apply_layout(state: &mut State, hierarchy: &Hierarchy) {
    //println!("RELAYOUT");

    let layout_hierarchy = hierarchy.into_iter().collect::<Vec<Entity>>();

    ///////////
    // Reset //
    ///////////
    for entity in layout_hierarchy.iter() {
        state.data.set_child_sum(*entity, 0.0);
        state.data.set_child_max(*entity, 0.0);
    }

    ///////////////////////////
    // Walk up the hierarchy //
    ///////////////////////////
    for child in layout_hierarchy.iter().rev() {
        // Stop before the window
        if *child == Entity::root() {
            break;
        }

        // Skip non-displayed entities
        let child_display = child.get_display(state);
        if child_display == Display::None {
            continue;
        }

        // Safe to unwrap because every entity in the hierarchy has a parent except window which is skipped
        let parent = child.get_parent(state).unwrap();

        let parent_flex_direction = parent.get_flex_direction(state);

        let child_margin_left = child.get_margin_left(state).get_value(0.0);
        let child_margin_right = child.get_margin_right(state).get_value(0.0);
        let child_margin_top = child.get_margin_top(state).get_value(0.0);
        let child_margin_bottom = child.get_margin_bottom(state).get_value(0.0);

        let (
            child_margin_main_before,
            child_margin_main_after,
            child_margin_cross_before,
            child_margin_cross_after,
        ) = match parent_flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => (
                child_margin_left,
                child_margin_right,
                child_margin_top,
                child_margin_bottom,
            ),
            FlexDirection::Column | FlexDirection::ColumnReverse => (
                child_margin_top,
                child_margin_bottom,
                child_margin_left,
                child_margin_right,
            ),
        };

        let (mut new_main, mut new_cross) = calculate_up(state, *child);

        //println!("UP: {} -> new_main: {} new_cross: {}", child, new_main, new_cross);

        let child_position = child.get_position(state);
        match child_position {
            Position::Relative => {
                new_main += child_margin_main_before + child_margin_main_after;
                state
                    .data
                    .set_child_sum(parent, state.data.get_child_sum(parent) + new_main);

                new_cross += child_margin_cross_before + child_margin_cross_after;
                let child_max = new_cross.max(state.data.get_child_max(parent));
                state.data.set_child_max(parent, child_max);
            }

            _ => {}
        }
    }

    /////////////////////////////
    // Walk down the hierarchy //
    /////////////////////////////
    for parent in layout_hierarchy.iter() {
        // Skip non-displayed entities
        let parent_display = parent.get_display(state);
        if parent_display == Display::None {
            continue;
        }

        let parent_flex_direction = parent.get_flex_direction(state);

        let parent_posx = state.data.get_posx(*parent);
        let parent_posy = state.data.get_posy(*parent);
        let parent_width = state.data.get_width(*parent);
        let parent_height = state.data.get_height(*parent);

        let (parent_main, parent_cross) = match parent_flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => (parent_width, parent_height),
            FlexDirection::Column | FlexDirection::ColumnReverse => (parent_height, parent_width),
        };

        let (parent_pos_main, parent_pos_cross) = match parent_flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                (state.data.get_posx(*parent), state.data.get_posy(*parent))
            }
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (state.data.get_posy(*parent), state.data.get_posx(*parent))
            }
        };

        //let grand_parent = parent.get_parent(state).unwrap();

        //let grand_parent_width = state.data.get_width(grand_parent);
        //let grand_parent_height = state.data.get_height(grand_parent);

        // Parent padding
        let parent_padding_left = parent.get_padding_left(state).get_value(0.0);
        let parent_padding_right = parent.get_padding_right(state).get_value(0.0);
        let parent_padding_top = parent.get_padding_top(state).get_value(0.0);
        let parent_padding_bottom = parent.get_padding_bottom(state).get_value(0.0);

        let (
            parent_padding_main_before,
            parent_padding_main_after,
            parent_padding_cross_before,
            parent_padding_cross_after,
        ) = match parent_flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => (
                parent_padding_left,
                parent_padding_right,
                parent_padding_top,
                parent_padding_bottom,
            ),
            FlexDirection::Column | FlexDirection::ColumnReverse => (
                parent_padding_top,
                parent_padding_bottom,
                parent_padding_left,
                parent_padding_right,
            ),
        };

        let parent_border_width = parent.get_border_width(state).get_value(parent_width);

        let mut main_sum = 0.0;
        let mut flex_grow_sum = 0.0;
        let mut flex_shrink_sum = 0.0;
        let mut num_of_children = 0;

        /////////////////////
        // Resize entities //
        /////////////////////
        for child in parent.child_iter(&hierarchy) {
            // Skip non-displayed entities
            let child_display = child.get_display(state);
            if child_display == Display::None {
                continue;
            }

            state.data.set_prev_width(child, state.data.get_width(child));
            state.data.set_prev_height(child, state.data.get_height(child));

            let (new_main, new_cross) = calculate_down(state, child);

            //println!("DOWN: {} -> new_main: {} new_cross: {}", child, new_main, new_cross);

            match parent_flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    state.data.set_width(child, new_main);
                    state.data.set_height(child, new_cross);
                }

                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    state.data.set_height(child, new_main);
                    state.data.set_width(child, new_cross);
                }
            }

            let child_margin_left = child.get_margin_left(state).get_value(0.0);
            let child_margin_right = child.get_margin_right(state).get_value(0.0);
            let child_margin_top = child.get_margin_top(state).get_value(0.0);
            let child_margin_bottom = child.get_margin_bottom(state).get_value(0.0);

            let (
                child_margin_main_before,
                child_margin_main_after,
                _child_margin_cross_before,
                _child_margin_cross_after,
            ) = match parent_flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => (
                    child_margin_left,
                    child_margin_right,
                    child_margin_top,
                    child_margin_bottom,
                ),
                FlexDirection::Column | FlexDirection::ColumnReverse => (
                    child_margin_top,
                    child_margin_bottom,
                    child_margin_left,
                    child_margin_right,
                ),
            };

            let child_position = child.get_position(state);

            match child_position {
                Position::Relative => {
                    let child_flex_grow = child.get_flex_grow(state);
                    flex_grow_sum += child_flex_grow;

                    let child_flex_shrink = child.get_flex_shrink(state);
                    flex_shrink_sum += child_flex_shrink;

                    num_of_children += 1;
                    main_sum += new_main + child_margin_main_before + child_margin_main_after;
                }

                _ => {}
            }
        }

        let mut free_space = parent_main
            - parent_padding_main_before
            - parent_padding_main_after
            - 2.0 * parent_border_width
            - main_sum;
        //println!("Entity: {}  free_space: {}", parent, free_space);

        // Positive free space so flexible entities can grow to fill
        if free_space > 0.0 && flex_grow_sum > 0.0 {
            // Filter to keep only flexible children
            let mut flexible_children = parent
                .child_iter(&hierarchy)
                .filter(|child| child.get_flex_grow(state) > 0.0)
                .collect::<Vec<_>>();

            // Sort flexible children by max_main
            match parent_flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => {
                    flexible_children.sort_by(|a, b| {
                        a.get_max_width(state)
                            .get_value_or(parent_main, std::f32::INFINITY)
                            .partial_cmp(
                                &b.get_max_width(state)
                                    .get_value_or(parent_main, std::f32::INFINITY),
                            )
                            .unwrap()
                    });
                }

                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    flexible_children.sort_by(|a, b| {
                        a.get_max_height(state)
                            .get_value_or(parent_main, std::f32::INFINITY)
                            .partial_cmp(
                                &b.get_max_height(state)
                                    .get_value_or(parent_main, std::f32::INFINITY),
                            )
                            .unwrap()
                    });
                }
            }

            //////////////////////////////
            // Resize Flexible Entities //
            //////////////////////////////
            for child in flexible_children.iter() {
                // Skip non-displayed entities
                let child_display = child.get_display(state);
                if child_display == Display::None {
                    continue;
                }

                let _child_margin_left = child.get_margin_left(state).get_value(0.0);
                let _child_margin_right = child.get_margin_right(state).get_value(0.0);
                let _child_margin_top = child.get_margin_top(state).get_value(0.0);
                let _child_margin_bottom = child.get_margin_bottom(state).get_value(0.0);

                let child_position = child.get_position(state);

                match child_position {
                    Position::Relative => {
                        //println!("Flexible Child: {}", child);

                        // Child size constraints
                        let _child_min_width = match child.get_min_width(state) {
                            Length::Pixels(val) => val,
                            _ => 0.0,
                        };
                        let child_max_width = match child.get_max_width(state) {
                            Length::Pixels(val) => val,
                            _ => std::f32::INFINITY,
                        };
                        let _child_min_height = match child.get_min_height(state) {
                            Length::Pixels(val) => val,
                            _ => 0.0,
                        };
                        let child_max_height = match child.get_max_height(state) {
                            Length::Pixels(val) => val,
                            _ => std::f32::INFINITY,
                        };

                        let child_flex_grow = child.get_flex_grow(state);
                        let space_per_flex = free_space * child_flex_grow / flex_grow_sum;
                        //println!("child: {} free_space: {} flex_grow: {} flex_grow_sum: {}", child, free_space, child_flex_grow, flex_grow_sum);

                        match parent_flex_direction {
                            FlexDirection::Row | FlexDirection::RowReverse => {
                                let child_width = state.data.get_width(*child);
                                let mut new_width = child_width + space_per_flex.round();

                                // Apply constraint (only max is needed because element is growing)
                                new_width = new_width.min(child_max_width);

                                free_space += child_width - new_width;
                                flex_grow_sum -= child_flex_grow;

                                state.data.set_width(*child, new_width);
                            }

                            FlexDirection::Column | FlexDirection::ColumnReverse => {
                                let child_height = state.data.get_height(*child);
                                let mut new_height = child_height + space_per_flex.round();

                                // Apply constraint (only max is needed because element is growing)
                                new_height = new_height.min(child_max_height);

                                free_space += child_height - new_height;
                                flex_grow_sum -= child_flex_grow;

                                state.data.set_height(*child, new_height);
                            }
                        }
                    }

                    _ => {}
                }

                let mut geometry_changed = GeometryChanged::default();

                let prev_width = state.data.get_prev_width(*child);
                let prev_height = state.data.get_prev_height(*child);
                let new_width = state.data.get_width(*child);
                let new_height = state.data.get_height(*child);
    
                if new_width != prev_width {
                    geometry_changed.width = true;
                }
                if new_height != prev_height {
                    geometry_changed.height = true;
                }
                
    
                if geometry_changed.width || geometry_changed.height {
                    state.insert_event(
                        Event::new(WindowEvent::GeometryChanged(geometry_changed))
                            .target(*child)
                            .propagate(Propagation::Down),
                    );
                }

            }
        } else if free_space < 0.0 && flex_shrink_sum > 0.0 {
            // Do some flex shrinking
        }

        

        ///////////////////////
        // Position Entities //
        ///////////////////////

        let children = match parent_flex_direction {
            FlexDirection::Row | FlexDirection::Column => {
                parent.child_iter(&hierarchy).collect::<Vec<_>>()
            }
            FlexDirection::RowReverse | FlexDirection::ColumnReverse => {
                parent.child_iter(&hierarchy).rev().collect::<Vec<_>>()
            }
        };

        let mut space_per_element = 0.0;
        let mut current_pos = 0.0;

        let parent_justify_content = parent.get_justify_content(state);
        let parent_align_items = parent.get_align_items(state);

        match parent_justify_content {
            JustifyContent::FlexStart => current_pos = 0.0,
            JustifyContent::FlexEnd => current_pos = free_space,
            JustifyContent::Center => current_pos = (free_space) / 2.0,
            JustifyContent::SpaceBetween => {
                space_per_element = free_space / (num_of_children - 1) as f32;
            }
            JustifyContent::SpaceAround => {
                space_per_element = free_space / num_of_children as f32;
                current_pos = space_per_element / 2.0;
            }
            JustifyContent::SpaceEvenly => {
                space_per_element = free_space / (num_of_children + 1) as f32;
                current_pos = space_per_element;
            }
            _ => {}
        }

        for child in children.into_iter() {
            // Skip non-displayed entities
            let child_display = child.get_display(state);
            if child_display == Display::None {
                continue;
            }

            let child_width = state.data.get_width(child);
            let child_height = state.data.get_height(child);

            let left = child.get_left(state);
            let right = child.get_right(state);
            let top = child.get_top(state);
            let bottom = child.get_bottom(state);

            let child_margin_left = child.get_margin_left(state).get_value(0.0);
            let child_margin_right = child.get_margin_right(state).get_value(0.0);
            let child_margin_top = child.get_margin_top(state).get_value(0.0);
            let child_margin_bottom = child.get_margin_bottom(state).get_value(0.0);

            let (
                child_margin_main_before,
                child_margin_main_after,
                child_margin_cross_before,
                child_margin_cross_after,
            ) = match parent_flex_direction {
                FlexDirection::Row | FlexDirection::RowReverse => (
                    child_margin_left,
                    child_margin_right,
                    child_margin_top,
                    child_margin_bottom,
                ),
                FlexDirection::Column | FlexDirection::ColumnReverse => (
                    child_margin_top,
                    child_margin_bottom,
                    child_margin_left,
                    child_margin_right,
                ),
            };

            let position = child.get_position(state);

            let mut new_posx ;
            let mut new_posy ;

            match position {
                Position::Relative => {
                    let (child_main, child_cross) = match parent_flex_direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            (child_width, child_height)
                        }
                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            (child_height, child_width)
                        }
                    };

                    let main_pos = parent_pos_main + current_pos;
                    current_pos += child_main
                        + space_per_element
                        + child_margin_main_before
                        + child_margin_main_after;

                    let mut cross_pos = parent_pos_cross
                        + match parent_align_items {
                            AlignItems::FlexStart => 0.0,
                            AlignItems::FlexEnd => {
                                parent_cross
                                    - parent_padding_cross_before
                                    - parent_padding_cross_after
                                    - 2.0 * parent_border_width
                                    - child_cross
                                    - child_margin_cross_before
                                    - child_margin_cross_after
                            }
                            AlignItems::Center => {
                                (parent_cross
                                    - parent_padding_cross_before
                                    - parent_padding_cross_after
                                    - 2.0 * parent_border_width
                                    - child_cross
                                    - child_margin_cross_before
                                    - child_margin_cross_after)
                                    / 2.0
                            }
                            AlignItems::Stretch => 0.0,
                        };

                    // align-self overrides align-items
                    if let Some(align_self) = state.style.align_self.get(child) {
                        cross_pos = parent_pos_cross
                            + match align_self {
                                AlignSelf::FlexStart => 0.0,
                                AlignSelf::FlexEnd => {
                                    parent_cross
                                        - parent_padding_cross_before
                                        - parent_padding_cross_after
                                        - 2.0 * parent_border_width
                                        - child_cross
                                }
                                AlignSelf::Center => {
                                    (parent_cross
                                        - parent_padding_cross_before
                                        - parent_padding_cross_after
                                        - 2.0 * parent_border_width
                                        - child_cross)
                                        / 2.0
                                }
                                AlignSelf::Stretch => 0.0,
                            }
                    }

                    match parent_flex_direction {
                        FlexDirection::Row | FlexDirection::RowReverse => {
                            new_posx = parent_padding_left + parent_border_width + main_pos;
                            new_posy = parent_padding_top + parent_border_width + cross_pos;
                            //state.data.set_posx(child, parent_padding_left + main_pos);
                            //state.data.set_posy(child, parent_padding_top + cross_pos);
                        }

                        FlexDirection::Column | FlexDirection::ColumnReverse => {
                            new_posy = parent_padding_top + parent_border_width + main_pos;
                            new_posx = parent_padding_left + parent_border_width + cross_pos;
                            //state.data.set_posy(child, parent_padding_top + main_pos);
                            //state.data.set_posx(child, parent_padding_left + cross_pos);
                        }
                    }

                    match left {
                        Length::Pixels(val) => {
                            new_posx += val;
                        }

                        Length::Percentage(val) => {
                            new_posx += val
                                * (parent_width
                                    - parent_padding_left
                                    - parent_padding_right
                                    - 2.0 * parent_border_width);
                        }

                        _ => {}
                    }

                    match top {
                        Length::Pixels(val) => {
                            new_posy += val;
                        }

                        Length::Percentage(val) => {
                            new_posy += val
                                * (parent_height
                                    - parent_padding_top
                                    - parent_padding_bottom
                                    - 2.0 * parent_border_width);
                        }

                        _ => {}
                    }

                    new_posx += child_margin_left;
                    new_posy += child_margin_top;
                }

                Position::Absolute => {
                    new_posx = parent_posx;
                    new_posy = parent_posy;

                    match right {
                        Length::Pixels(val) => {
                            new_posx = parent_posx + parent_width - child_width - val;
                        }

                        Length::Percentage(val) => {
                            new_posx =
                                parent_posx + parent_width - child_width - (val * parent_width);
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
                            new_posy = parent_posy + parent_height - child_height - val;
                        }

                        Length::Percentage(val) => {
                            new_posy =
                                parent_posy + parent_height - child_height - (val * parent_height);
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
                }
            }

            let mut geometry_changed = GeometryChanged::default();

            if state.data.get_posx(child) != new_posx {
                state.data.set_posx(child, new_posx);
                //should_continue = true;
                geometry_changed.posx = true;
            }

            if state.data.get_posy(child) != new_posy {
                state.data.set_posy(child, new_posy);
                //should_continue = true;
                geometry_changed.posy = true;
            }

            if geometry_changed.posx || geometry_changed.posy {
                state.insert_event(
                    Event::new(WindowEvent::GeometryChanged(geometry_changed))
                        .target(child)
                        .propagate(Propagation::Down),
                );
            }
        }
    }
}

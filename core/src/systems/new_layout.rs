use crate::*;

use crate::{Entity, State};


pub fn apply_transform(state: &mut State, tree: &Tree) {
    //println!("Apply Transform");
    for entity in tree.into_iter() {

        //println!("Entity: {}", entity);
        
        if entity == Entity::root() {
            continue;
        }
        
        let parent = tree.get_parent(entity).unwrap();
        //let parent_origin = state.data.get_origin(parent);
        let parent_transform = state.data.get_transform(parent);

        state.data.set_transform(entity, Transform2D::identity());

        state.data.set_transform(entity, parent_transform);

        let bounds = state.data.get_bounds(entity);

        //state.data.set_origin(entity, parent_origin);
        
        if let Some(translate) = state.style.translate.get(entity) {
            state.data.set_translate(entity, *translate);
        }

        if let Some(rotate) = state.style.rotate.get(entity) {
            let x = bounds.x + (bounds.w / 2.0);
            let y = bounds.y + (bounds.h / 2.0);
            state.data.set_translate(entity, (x,y));
            state.data.set_rotate(entity, (*rotate).to_radians());
            state.data.set_translate(entity, (-x,-y));
        }
        //println!("End");

        if let Some(scale) = state.style.scale.get(entity) {
            let x = bounds.x + (bounds.w / 2.0);
            let y = bounds.y + (bounds.h / 2.0);
            state.data.set_translate(entity, (x,y));
            state.data.set_scale(entity, *scale);
            state.data.set_translate(entity, (-x,-y));
        }       

    }
}


#[derive(Debug)]
enum Axis {
    Before,
    Size,
    After,
}

/*
pub fn apply_layout2(state: &mut State, tree: &Tree) {
    //println!("Apply Layout");
    let layout_tree = tree.into_iter().collect::<Vec<Entity>>();

    // for entity in layout_tree.iter() {
    //     state.data.set_child_sum(*entity, 0.0);
    //     state.data.set_child_max(*entity, 0.0);
    // }

    for parent in layout_tree.iter() {

        // Skip non-displayed entities
        let parent_display = parent.get_display(state);
        if parent_display == Display::None {
            continue;
        }
        

        let mut found_first = false;
        let mut last_child = Entity::null();

        state.data.set_child_width_sum(*parent, 0.0);
        state.data.set_child_height_sum(*parent, 0.0);
        state.data.set_child_width_max(*parent, 0.0);
        state.data.set_child_height_max(*parent, 0.0);

        state.data
            .set_prev_width(*parent, state.data.get_width(*parent));
        state
            .data
            .set_prev_height(*parent, state.data.get_height(*parent));

        for child in parent.child_iter(tree) {

            let child_display = child.get_display(state);
            if child_display == Display::None {
                continue;
            }

            state.data.set_stack_first_child(child, false);

            let child_positioning_type = state
                .style
                .positioning_type
                .get(child)
                .cloned()
                .unwrap_or_default();

            match child_positioning_type {
                PositionType::ParentDirected => {
                    if !found_first {
                        found_first = true;
                        state.data.set_stack_first_child(child, true);
                    }
                    last_child = child;
                }

                PositionType::SelfDirected => {
                    state.data.set_stack_first_child(child, true);
                    state.data.set_stack_last_child(child, true);
                }
            }
        }

        state.data.set_stack_last_child(last_child, true);
    }

    // Walk up the tree
    for child in layout_tree.iter().rev() {
        // Stop before the window
        if *child == Entity::root() {
            break;
        }

        // Skip non-displayed entities
        let child_display = child.get_display(state);
        if child_display == Display::None {
            continue;
        }

    

        // Safe to unwrap because every entity in the tree has a parent except window which is skipped
        let parent = child.get_parent(state).unwrap();

        let parent_layout_type = state
            .style
            .layout_type
            .get(parent)
            .cloned()
            .unwrap_or_default();

        let child_left = state
            .style
            .child_left
            .get(parent)
            .cloned()
            .unwrap_or_default();
        let child_right = state
            .style
            .child_right
            .get(parent)
            .cloned()
            .unwrap_or_default();
        let child_top = state
            .style
            .child_top
            .get(parent)
            .cloned()
            .unwrap_or_default();
        let child_bottom = state
            .style
            .child_bottom
            .get(parent)
            .cloned()
            .unwrap_or_default();

        let child_between = state
            .style
            .child_between
            .get(parent)
            .cloned()
            .unwrap_or_default();

        // TODO - support percentage border
        let parent_border_width = parent.get_border_width(state).get_value(0.0);

        let parent_width = state.data.get_width(parent) - 2.0 * parent_border_width;
        let parent_height = state.data.get_height(parent) - 2.0 * parent_border_width;

        let child_border_width = child.get_border_width(state).get_value(parent_width);

        let mut left = state.style.left.get(*child).cloned().unwrap_or_default();
        let width = state
            .style
            .width
            .get(*child)
            .cloned()
            .unwrap_or(Units::Stretch(1.0));
        let mut right = state.style.right.get(*child).cloned().unwrap_or_default();

        let mut top = state.style.top.get(*child).cloned().unwrap_or_default();
        let height = state
            .style
            .height
            .get(*child)
            .cloned()
            .unwrap_or(Units::Stretch(1.0));
        let mut bottom = state.style.bottom.get(*child).cloned().unwrap_or_default();

        let min_left = state
            .style
            .min_left
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_width, -std::f32::INFINITY);
        //let min_width = state.style.min_width.get(*child).cloned().unwrap_or_default().get_value_or(parent_width, 0.0);
        let min_right = state
            .style
            .min_right
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_width, -std::f32::INFINITY);

        let max_left = state
            .style
            .max_left
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_width, std::f32::INFINITY);
        let max_width = state
            .style
            .max_width
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_width, std::f32::INFINITY);
        let max_right = state
            .style
            .max_right
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_width, std::f32::INFINITY);

        let min_top = state
            .style
            .min_top
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_height, -std::f32::INFINITY);
        //let min_height = state.style.min_height.get(*child).cloned().unwrap_or_default().get_value_or(parent_height, 0.0);
        let min_bottom = state
            .style
            .min_bottom
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_height, -std::f32::INFINITY);

        let max_top = state
            .style
            .max_top
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_height, std::f32::INFINITY);
        let max_height = state
            .style
            .max_height
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_height, std::f32::INFINITY);
        let max_bottom = state
            .style
            .max_bottom
            .get(*child)
            .cloned()
            .unwrap_or_default()
            .get_value_or(parent_height, std::f32::INFINITY);

        let child_layout_type = state
            .style
            .layout_type
            .get(*child)
            .cloned()
            .unwrap_or_default();
        let child_positioning_type = state
            .style
            .positioning_type
            .get(*child)
            .cloned()
            .unwrap_or_default();

        let min_width = state
            .style
            .min_width
            .get(*child)
            .cloned()
            .unwrap_or_default();

        let min_width = match min_width {
            Units::Pixels(val) => val,

            Units::Percentage(val) => (val * parent_width).round(),

            Units::Stretch(_) => 0.0,

            Units::Auto => {
                //println!("{} Auto", child);
                match child_layout_type {
                    LayoutType::Column => {
                        state.data.get_child_width_max(*child) + 2.0 * child_border_width
                    }

                    LayoutType::Row => state.data.get_child_width_sum(*child) + 2.0 * child_border_width,

                    LayoutType::Grid => state.data.get_child_width_sum(*child) + 2.0 * child_border_width,

                    LayoutType::None => 0.0,
                }
            }
        };

        let min_height = state
            .style
            .min_height
            .get(*child)
            .cloned()
            .unwrap_or_default();

        let min_height = match min_height {
            Units::Pixels(val) => val,

            Units::Percentage(val) => (val * parent_height).round(),

            Units::Stretch(_) => 0.0,

            Units::Auto => match child_layout_type {
                LayoutType::Column => state.data.get_child_height_sum(*child) + 2.0 * child_border_width,

                LayoutType::Row => state.data.get_child_height_max(*child) + 2.0 * child_border_width,

                _ => state.data.get_child_height_sum(*child) + 2.0 * child_border_width,
            },
        };

        //println!("{} Min Width: {}", child, min_width);

        // Parent overrides
        match parent_layout_type {
            LayoutType::Column => {
                if state.data.get_stack_child(*child).0 {
                    if top == Units::Auto {
                        top = child_top.clone();
                    }
                } else {
                    if top == Units::Auto {
                        top = child_between.clone();
                    }
                }

                if state.data.get_stack_child(*child).1 {
                    if bottom == Units::Auto {
                        bottom = child_bottom.clone();
                    }
                }

                if left == Units::Auto {
                    left = child_left.clone();
                }

                if right == Units::Auto {
                    right = child_right.clone();
                }
            }

            LayoutType::Row => {
                if state.data.get_stack_child(*child).0 {
                    if left == Units::Auto {
                        left = child_left.clone();
                    }
                } else {
                    if left == Units::Auto {
                        left = child_between.clone();
                    }
                }

                if state.data.get_stack_child(*child).1 {
                    if right == Units::Auto {
                        right = child_right.clone();
                    }
                }

                if top == Units::Auto {
                    top = child_top.clone();
                }

                if bottom == Units::Auto {
                    bottom = child_bottom.clone();
                }
            }

            _ => {}
        }

        let mut new_left = 0.0;
        let mut new_width = 0.0;
        let mut new_right = 0.0;

        let mut new_top = 0.0;
        let mut new_height = 0.0;
        let mut new_bottom = 0.0;

        let mut horizontal_used_space = 0.0;
        let mut vertical_used_space = 0.0;

        match left {
            Units::Pixels(val) => {
                new_left = val.clamp(min_left, max_left);
                horizontal_used_space += new_left;
            }

            _ => {}
        }

        match width {
            Units::Pixels(val) => {
                new_width = val.clamp(min_width, max_width);
                horizontal_used_space += new_width;
            }

            Units::Auto => {
                match child_layout_type {
                    LayoutType::Column => {
                        new_width = state.data.get_child_width_max(*child) + 2.0 * child_border_width;
                    }

                    LayoutType::Row => {
                        new_width = state.data.get_child_width_sum(*child) + 2.0 * child_border_width;
                    }

                    LayoutType::Grid => {
                        new_width = state.data.get_child_width_sum(*child) + 2.0 * child_border_width;
                    }

                    _ => {}
                }

                horizontal_used_space += new_width;
            }

            _ => {}
        }

        match right {
            Units::Pixels(val) => {
                new_right = val.clamp(min_right, max_right);
                horizontal_used_space += new_right;
            }

            _ => {}
        }

        match top {
            Units::Pixels(val) => {
                new_top = val.clamp(min_top, max_top);
                vertical_used_space += new_top;
            }

            _ => {}
        }

        match height {
            Units::Pixels(val) => {
                new_height = val.clamp(min_height, max_height);
                vertical_used_space += new_height;
            }

            Units::Auto => {
                match child_layout_type {
                    LayoutType::Column => {
                        new_height = state.data.get_child_height_sum(*child) + 2.0 * child_border_width;
                    }

                    LayoutType::Row => {
                        new_height = state.data.get_child_height_max(*child) + 2.0 * child_border_width;
                    }

                    LayoutType::Grid => {
                        new_height = state.data.get_child_height_sum(*child) + 2.0 * child_border_width;
                    }

                    _ => {}
                }

                vertical_used_space += new_height;
            }

            _ => {}
        }

        match bottom {
            Units::Pixels(val) => {
                new_bottom = val.clamp(min_bottom, max_bottom);
                vertical_used_space += new_bottom;
            }

            _ => {}
        }

        // match child_positioning_type {
        //     PositionType::SelfDirected => {
        //         horizontal_used_space = 0.0;
        //         vertical_used_space = 0.0;
        //     }

        //     _=> {}
        // }

        //println!("{} Row used space {}", child, horizontal_used_space);

        if child_positioning_type == PositionType::ParentDirected {
            state.data.set_child_height_sum(
                parent,
                state.data.get_child_height_sum(parent) + vertical_used_space,
            );

            state.data.set_child_height_max(
                parent,
                vertical_used_space.max(state.data.get_child_height_max(parent)),
            );

            state.data.set_child_width_sum(
                parent,
                state.data.get_child_width_sum(parent) + horizontal_used_space,
            );

            state.data.set_child_width_max(
                parent,
                horizontal_used_space.max(state.data.get_child_width_max(parent)),
            );
        }

        // match parent_layout_type {
        //     LayoutType::Column => {
        //         if child_positioning_type == PositionType::ParentDirected {
        //             state.data.set_child_height_sum(
        //                 parent,
        //                 state.data.get_child_height_sum(parent) + vertical_used_space,
        //             );
        //             state.data.set_child_height_max(
        //                 parent,
        //                 horizontal_used_space.max(state.data.get_child_width_max(parent)),
        //             );
        //         }
        //     }

        //     LayoutType::Row => {
        //         if child_positioning_type == PositionType::ParentDirected {
        //             state.data.set_child_width_sum(
        //                 parent,
        //                 state.data.get_child_width_sum(parent) + horizontal_used_space,
        //             );
        //             state.data.set_child_width_max(
        //                 parent,
        //                 vertical_used_space.max(state.data.get_child_width_max(parent)),
        //             );
        //         }
        //     }

        //     _ => {}
        // }

        state.data.set_height(*child, new_height);
        state.data.set_width(*child, new_width);
        state.data.set_space_top(*child, new_top);
        state.data.set_space_bottom(*child, new_bottom);
        state.data.set_space_left(*child, new_left);
        state.data.set_space_right(*child, new_right);
    }

    // Depth first traversal of all nodes from root
    for parent in layout_tree.into_iter() {


        // Skip non-displayed entities
        let parent_display = parent.get_display(state);
        if parent_display == Display::None {  
            continue;
        }


        let parent_layout_type = state
            .style
            .layout_type
            .get(parent)
            .cloned()
            .unwrap_or_default();

        let child_left = state
            .style
            .child_left
            .get(parent)
            .cloned()
            .unwrap_or_default();
        let child_right = state
            .style
            .child_right
            .get(parent)
            .cloned()
            .unwrap_or_default();
        let child_top = state
            .style
            .child_top
            .get(parent)
            .cloned()
            .unwrap_or_default();
        let child_bottom = state
            .style
            .child_bottom
            .get(parent)
            .cloned()
            .unwrap_or_default();

        let child_between = state
            .style
            .child_between
            .get(parent)
            .cloned()
            .unwrap_or_default();

        // TODO - support percentage border
        let parent_border_width = parent.get_border_width(state).get_value(0.0);

        let parent_width = state.data.get_width(parent) - 2.0 * parent_border_width;
        let parent_height = state.data.get_height(parent) - 2.0 * parent_border_width;

        let (parent_main, parent_cross) = match parent_layout_type {
            LayoutType::Column => (parent_height, parent_width),

            LayoutType::Row | LayoutType::Grid | LayoutType::None => (parent_width, parent_height),
        };

        let mut main_free_space = parent_main;
        let mut main_stretch_sum: f32 = 0.0;

        match parent_layout_type {
            LayoutType::Row | LayoutType::Column => {
                let mut horizontal_axis = Vec::new();
                let mut vertical_axis = Vec::new();

                // ////////////////////////////////
                // Calculate inflexible children //
                ///////////////////////////////////
                for child in parent.child_iter(&tree) {

                    let child_display = child.get_display(state);
                    if child_display == Display::None {
                        continue;
                    }

                    let mut left = state.style.left.get(child).cloned().unwrap_or_default();
                    let width = state
                        .style
                        .width
                        .get(child)
                        .cloned()
                        .unwrap_or(Units::Stretch(1.0));
                    let mut right = state.style.right.get(child).cloned().unwrap_or_default();

                    let mut top = state.style.top.get(child).cloned().unwrap_or_default();
                    let height = state
                        .style
                        .height
                        .get(child)
                        .cloned()
                        .unwrap_or(Units::Stretch(1.0));
                    let mut bottom = state.style.bottom.get(child).cloned().unwrap_or_default();

                    let min_left = state
                        .style
                        .min_left
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_width, -std::f32::INFINITY);
                    //let min_width = state.style.min_width.get(child).cloned().unwrap_or_default().get_value_or(parent_width, 0.0);
                    let min_right = state
                        .style
                        .min_right
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_width, -std::f32::INFINITY);

                    let max_left = state
                        .style
                        .max_left
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_width, std::f32::INFINITY);
                    let max_width = state
                        .style
                        .max_width
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_width, std::f32::INFINITY);
                    let max_right = state
                        .style
                        .max_right
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_width, std::f32::INFINITY);

                    let min_top = state
                        .style
                        .min_top
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_height, -std::f32::INFINITY);
                    //let min_height = state.style.min_height.get(child).cloned().unwrap_or_default().get_value_or(parent_height, 0.0);
                    let min_bottom = state
                        .style
                        .min_bottom
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_height, -std::f32::INFINITY);

                    let max_top = state
                        .style
                        .max_top
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_height, std::f32::INFINITY);
                    let max_height = state
                        .style
                        .max_height
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_height, std::f32::INFINITY);
                    let max_bottom = state
                        .style
                        .max_bottom
                        .get(child)
                        .cloned()
                        .unwrap_or_default()
                        .get_value_or(parent_height, std::f32::INFINITY);

                    let child_border_width = child.get_border_width(state).get_value(parent_width);

                    // Parent overrides
                    match parent_layout_type {
                        LayoutType::Column => {
                            if state.data.get_stack_child(child).0 {
                                if top == Units::Auto {
                                    top = child_top.clone();
                                }
                            } else {
                                if top == Units::Auto {
                                    top = child_between.clone();
                                }
                            }

                            if state.data.get_stack_child(child).1 {
                                if bottom == Units::Auto {
                                    bottom = child_bottom.clone();
                                }
                            }

                            if left == Units::Auto {
                                left = child_left.clone();
                            }

                            if right == Units::Auto {
                                right = child_right.clone();
                            }
                        }

                        LayoutType::Row => {
                            if state.data.get_stack_child(child).0 {
                                if left == Units::Auto {
                                    left = child_left.clone();
                                }
                            } else {
                                if left == Units::Auto {
                                    left = child_between.clone();
                                }
                            }

                            if state.data.get_stack_child(child).1 {
                                if right == Units::Auto {
                                    right = child_right.clone();
                                }
                            }

                            if top == Units::Auto {
                                top = child_top.clone();
                            }

                            if bottom == Units::Auto {
                                bottom = child_bottom.clone();
                            }
                        }

                        _ => {}
                    }

                    let mut new_left = 0.0;
                    let mut new_width = 0.0;
                    let mut new_right = 0.0;

                    let mut new_top = 0.0;
                    let mut new_height = 0.0;
                    let mut new_bottom = 0.0;

                    let mut horizontal_stretch_sum = 0.0;
                    let mut vertical_stretch_sum = 0.0;

                    let mut horizontal_used_space = 0.0;
                    let mut vertical_used_space = 0.0;

                    let mut cross_stretch_sum = 0.0;
                    let mut cross_free_space = 0.0;

                    let child_layout_type = state
                        .style
                        .layout_type
                        .get(child)
                        .cloned()
                        .unwrap_or_default();

                    let min_width = state
                        .style
                        .min_width
                        .get(child)
                        .cloned()
                        .unwrap_or_default();

                    let min_width = match min_width {
                        Units::Pixels(val) => val,

                        Units::Percentage(val) => (val * parent_width).round(),

                        Units::Stretch(_) => 0.0,

                        Units::Auto => {
                            //println!("{} Auto", child);
                            match child_layout_type {
                                LayoutType::Column => {
                                    state.data.get_child_width_max(child) + 2.0 * child_border_width
                                }

                                LayoutType::Row | LayoutType::Grid => {
                                    state.data.get_child_width_sum(child) + 2.0 * child_border_width
                                }

                                _ => 0.0,
                            }
                        }
                    };

                    let min_height = state
                        .style
                        .min_height
                        .get(child)
                        .cloned()
                        .unwrap_or_default();

                    let min_height = match min_height {
                        Units::Pixels(val) => val,

                        Units::Percentage(val) => (val * parent_height).round(),

                        Units::Stretch(_) => 0.0,

                        Units::Auto => match child_layout_type {
                            LayoutType::Column | LayoutType::Grid => {
                                state.data.get_child_height_sum(child) + 2.0 * child_border_width
                            }

                            LayoutType::Row => {
                                state.data.get_child_height_max(child) + 2.0 * child_border_width
                            }

                            _ => 0.0,
                        },
                    };

                    // TODO - replace all these match' with a function
                    match left {
                        Units::Pixels(val) => {
                            new_left = val.clamp(min_left, max_left);
                            horizontal_used_space += new_left;
                        }

                        Units::Percentage(val) => {
                            new_left = (val * parent_width).round();
                            new_left = new_left.clamp(min_left, max_left);
                            horizontal_used_space += new_left;
                        }

                        Units::Stretch(val) => {
                            horizontal_stretch_sum += val;
                            horizontal_axis.push((child, val, min_left, max_left, Axis::Before));
                        }

                        _ => {}
                    }

                    match width {
                        Units::Pixels(val) => {
                            new_width = val.clamp(min_width, max_width);
                            horizontal_used_space += new_width;
                        }

                        Units::Percentage(val) => {
                            new_width = (val * parent_width).round();
                            new_width = new_width.clamp(min_width, max_width);
                            horizontal_used_space += new_width;
                        }

                        Units::Stretch(val) => {
                            horizontal_stretch_sum += val;
                            horizontal_axis.push((child, val, min_width, max_width, Axis::Size));
                        }

                        Units::Auto => {
                            match child_layout_type {
                                LayoutType::Column => {
                                    new_width =
                                        state.data.get_child_width_max(child) + 2.0 * child_border_width;
                                }

                                LayoutType::Row | LayoutType::Grid=> {
                                    new_width =
                                        state.data.get_child_width_sum(child) + 2.0 * child_border_width;
                                }

                                _ => {}
                            }

                            horizontal_used_space += new_width;
                        }
                    }

                    match right {
                        Units::Pixels(val) => {
                            new_right = val.clamp(min_right, max_right);
                            horizontal_used_space += new_right;
                        }

                        Units::Percentage(val) => {
                            new_right = (val * parent_width).round();
                            new_right = new_right.clamp(min_right, max_right);
                            horizontal_used_space += new_right;
                        }

                        Units::Stretch(val) => {
                            horizontal_stretch_sum += val;
                            horizontal_axis.push((child, val, min_right, max_right, Axis::After));
                        }

                        _ => {}
                    }

                    match top {
                        Units::Pixels(val) => {
                            new_top = val.clamp(min_top, max_top);
                            vertical_used_space += new_top;
                        }

                        Units::Percentage(val) => {
                            new_top = (val * parent_height).round();
                            new_top = new_top.clamp(min_top, max_top);
                            vertical_used_space += new_top;
                        }

                        Units::Stretch(val) => {
                            vertical_stretch_sum += val;
                            vertical_axis.push((child, val, min_top, max_top, Axis::Before));
                        }

                        _ => {}
                    }

                    match height {
                        Units::Pixels(val) => {
                            new_height = val.clamp(min_height, max_height);
                            vertical_used_space += new_height;
                        }

                        Units::Percentage(val) => {
                            new_height = (val * parent_height).round();
                            new_height = new_height.clamp(min_height, max_height);
                            vertical_used_space += new_height;
                        }

                        Units::Stretch(val) => {
                            vertical_stretch_sum += val;
                            vertical_axis.push((child, val, min_height, max_height, Axis::Size));
                        }

                        Units::Auto => {
                            match child_layout_type {
                                LayoutType::Column | LayoutType::Grid => {
                                    new_height =
                                        state.data.get_child_height_sum(child) + 2.0 * child_border_width;
                                }

                                LayoutType::Row => {
                                    new_height =
                                        state.data.get_child_height_max(child) + 2.0 * child_border_width;
                                }

                                _ => {}
                            }

                            vertical_used_space += new_height;
                        }
                    }

                    match bottom {
                        Units::Pixels(val) => {
                            new_bottom = val.clamp(min_bottom, max_bottom);
                            vertical_used_space += val;
                        }

                        Units::Percentage(val) => {
                            new_bottom = (val * parent_height).round();
                            new_bottom = new_bottom.clamp(min_bottom, max_bottom);
                            vertical_used_space += new_bottom;
                        }

                        Units::Stretch(val) => {
                            vertical_stretch_sum += val;
                            vertical_axis.push((child, val, min_bottom, max_bottom, Axis::After));
                        }

                        _ => {}
                    }

                    state.data.set_height(child, new_height);
                    state.data.set_width(child, new_width);
                    state.data.set_space_top(child, new_top);
                    state.data.set_space_bottom(child, new_bottom);
                    state.data.set_space_left(child, new_left);
                    state.data.set_space_right(child, new_right);

                    let child_positioning_type = state
                        .style
                        .positioning_type
                        .get(child)
                        .cloned()
                        .unwrap_or_default();

                    //horizontal_stretch_sum = horizontal_stretch_sum.max(1.0);
                    //vertical_stretch_sum = vertical_stretch_sum.max(1.0);

                    state
                        .data
                        .set_horizontal_used_space(child, horizontal_used_space);
                    state
                        .data
                        .set_horizontal_stretch_sum(child, horizontal_stretch_sum);
                    state
                        .data
                        .set_vertical_used_space(child, vertical_used_space);
                    state
                        .data
                        .set_vertical_stretch_sum(child, vertical_stretch_sum);

                    match parent_layout_type {
                        LayoutType::Column => {
                            if child_positioning_type == PositionType::SelfDirected {
                                vertical_used_space = 0.0;
                                vertical_stretch_sum = 0.0;
                            }
                            cross_stretch_sum += horizontal_stretch_sum;
                            main_stretch_sum += vertical_stretch_sum;
                            main_free_space -= vertical_used_space;
                            cross_free_space = parent_cross - horizontal_used_space;
                        }

                        LayoutType::Row => {
                            if child_positioning_type == PositionType::SelfDirected {
                                horizontal_used_space = 0.0;
                                horizontal_stretch_sum = 0.0;
                            }
                            cross_stretch_sum += vertical_stretch_sum;
                            main_stretch_sum += horizontal_stretch_sum;
                            main_free_space -= horizontal_used_space;
                            cross_free_space = parent_cross - vertical_used_space;
                        }

                        _ => {}
                    }

                    if cross_stretch_sum == 0.0 {
                        cross_stretch_sum = 1.0;
                    }

                    state.data.set_cross_stretch_sum(child, cross_stretch_sum);
                    state.data.set_cross_free_space(child, cross_free_space);
                }

                if main_stretch_sum == 0.0 {
                    main_stretch_sum = 1.0;
                }

                horizontal_axis.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
                vertical_axis.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

                let mut horizontal_stretch_sum = 0.0;
                let mut horizontal_free_space = 0.0;
                let mut vertical_stretch_sum = 0.0;
                let mut vertical_free_space = 0.0;

                // Calculate flexible Row space & size
                for (child, value, min_value, max_value, variant) in horizontal_axis.iter() {
                    
                    let cross_stretch_sum = state.data.get_cross_stretch_sum(*child);
                    let cross_free_space = state.data.get_cross_free_space(*child);

                    let child_positioning_type = state
                        .style
                        .positioning_type
                        .get(*child)
                        .cloned()
                        .unwrap_or_default();

                    match child_positioning_type {
                        PositionType::SelfDirected => {
                            horizontal_free_space =
                                parent_width - state.data.get_horizontal_used_space(*child);
                            horizontal_stretch_sum = state.data.get_horizontal_stretch_sum(*child);
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Column => {
                                    horizontal_stretch_sum = cross_stretch_sum;
                                    horizontal_free_space = cross_free_space;
                                }

                                LayoutType::Row => {
                                    horizontal_free_space = main_free_space;
                                    horizontal_stretch_sum = main_stretch_sum;
                                }

                                _ => {}
                            };
                        }
                    }

                    if horizontal_stretch_sum == 0.0 {
                        horizontal_stretch_sum = 1.0;
                    }

                    let mut new_value = horizontal_free_space * value / horizontal_stretch_sum;

                    new_value = new_value.clamp(*min_value, *max_value);

                    match variant {
                        Axis::Before => {
                            state.data.set_space_left(*child, new_value);
                        }

                        Axis::Size => {
                            state.data.set_width(*child, new_value);
                        }

                        Axis::After => {
                            state.data.set_space_right(*child, new_value);
                        }
                    }

                    match child_positioning_type {
                        PositionType::SelfDirected => {
                            state
                                .data
                                .set_horizontal_stretch_sum(*child, horizontal_stretch_sum - value);
                            state.data.set_horizontal_used_space(
                                *child,
                                parent_width - horizontal_free_space + new_value,
                            );
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Column => {
                                    state.data.set_cross_stretch_sum(
                                        *child,
                                        horizontal_stretch_sum - value,
                                    );
                                    state.data.set_cross_free_space(
                                        *child,
                                        horizontal_free_space - new_value,
                                    );
                                }

                                LayoutType::Row => {
                                    main_free_space -= new_value;
                                    main_stretch_sum -= value;
                                }

                                _ => {}
                            };
                        }
                    }
                }

                // Calculate flexible Column space & size
                for (child, value, min_value, max_value, variant) in vertical_axis.iter() {
                    let cross_stretch_sum = state.data.get_cross_stretch_sum(*child);
                    let cross_free_space = state.data.get_cross_free_space(*child);

                    let child_positioning_type = state
                        .style
                        .positioning_type
                        .get(*child)
                        .cloned()
                        .unwrap_or_default();

                    match child_positioning_type {
                        PositionType::SelfDirected => {
                            vertical_free_space =
                                parent_height - state.data.get_vertical_used_space(*child);
                            vertical_stretch_sum = state.data.get_vertical_stretch_sum(*child);
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Row => {
                                    vertical_stretch_sum = cross_stretch_sum;
                                    vertical_free_space = cross_free_space;
                                }

                                LayoutType::Column => {
                                    vertical_free_space = main_free_space;
                                    vertical_stretch_sum = main_stretch_sum;
                                }

                                _ => {}
                            };
                        }
                    }

                    if vertical_stretch_sum == 0.0 {
                        vertical_stretch_sum = 1.0;
                    }

                    let mut new_value = vertical_free_space * value / vertical_stretch_sum;
                    new_value = new_value.clamp(*min_value, *max_value);

                    match variant {
                        Axis::Before => {
                            state.data.set_space_top(*child, new_value);
                        }

                        Axis::Size => {
                            state.data.set_height(*child, new_value);
                        }

                        Axis::After => {
                            state.data.set_space_bottom(*child, new_value);
                        }
                    }

                    match child_positioning_type {
                        PositionType::SelfDirected => {
                            state
                                .data
                                .set_vertical_stretch_sum(*child, vertical_stretch_sum - value);
                            state.data.set_vertical_used_space(
                                *child,
                                parent_height - vertical_free_space + new_value,
                            );
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Row => {
                                    state.data.set_cross_stretch_sum(
                                        *child,
                                        vertical_stretch_sum - value,
                                    );
                                    state.data.set_cross_free_space(
                                        *child,
                                        vertical_free_space - new_value,
                                    );
                                }

                                LayoutType::Column => {
                                    main_free_space -= new_value;
                                    main_stretch_sum -= value;
                                }

                                _ => {}
                            };
                        }
                    }
                }

                let mut current_posx = 0.0;
                let mut current_posy = 0.0;

                // TODO - support percentage border
                let parent_border_width = parent.get_border_width(state).get_value(0.0);

                let parent_posx = state.data.get_posx(parent) + parent_border_width;
                let parent_posy = state.data.get_posy(parent) + parent_border_width;

                ///////////////////////
                // Position Children //
                ///////////////////////
                for child in parent.child_iter(&tree) {

                    let child_display = child.get_display(state);
                    if child_display == Display::None {
                        continue;
                    }

                    let space = state.data.get_space(child);

                    let width = state.data.get_width(child);
                    let height = state.data.get_height(child);

                    let child_positioning_type = state
                        .style
                        .positioning_type
                        .get(child)
                        .cloned()
                        .unwrap_or_default();

                    let (new_posx, new_posy) = match child_positioning_type {
                        PositionType::SelfDirected => {
                            (parent_posx + space.left, parent_posy + space.top)
                        }

                        PositionType::ParentDirected => {
                            let new_posx = parent_posx + current_posx + space.left;
                            let new_posy = parent_posy + current_posy + space.top;

                            match parent_layout_type {
                                LayoutType::Column => {
                                    current_posy += space.top + height + space.bottom;
                                }

                                LayoutType::Row => {
                                    current_posx += space.left + width + space.right;
                                }

                                _ => {}
                            }

                            (new_posx, new_posy)
                        }
                    };

                    state.data.set_posx(child, new_posx);
                    state.data.set_posy(child, new_posy);
                }

                let prev_width = state.data.get_prev_width(parent);
                let prev_height = state.data.get_prev_height(parent);
                let new_width = state.data.get_width(parent);
                let new_height = state.data.get_height(parent);

                let mut geometry_changed = GeometryChanged::default();

                if new_width != prev_width {
                    geometry_changed.width = true;
                }
                if new_height != prev_height {
                    geometry_changed.height = true;
                }

                if geometry_changed.width || geometry_changed.height {
                    state.insert_event(
                        Event::new(WindowEvent::GeometryChanged(geometry_changed))
                            .target(parent)
                            .propagate(Propagation::Down),
                    );
                }
            }

            LayoutType::Grid => {
                let grid_rows = state
                    .style
                    .grid_rows
                    .get(parent)
                    .cloned()
                    .unwrap_or_default();
                let grid_cols = state
                    .style
                    .grid_cols
                    .get(parent)
                    .cloned()
                    .unwrap_or_default();
                
                

                let mut row_heights = vec![(0.0, 0.0, 0.0, 0.0); grid_rows.len() + 1];
                let mut col_widths = vec![(0.0, 0.0, 0.0, 0.0); grid_cols.len() + 1];

                let mut row_free_space = state.data.get_height(parent);
                let mut col_free_space = state.data.get_width(parent);

                let mut row_stretch_sum = 0.0;
                let mut col_stretch_sum = 0.0;

                let row_between = match state.style.row_between
                    .get(parent)
                    .cloned()
                    .unwrap_or_default() {
                        Units::Pixels(val) => val,

                        _=> 0.0,
                    };

                let col_between = match state.style.col_between
                    .get(parent)
                    .cloned()
                    .unwrap_or_default() {
                        Units::Pixels(val) => val,

                        _=> 0.0,
                    };


                for (i, row) in grid_rows.iter().enumerate() {
                    match row {
                        &Units::Pixels(val) => {
                            row_heights[i].1 = val;
                            row_free_space -= val;
                        }

                        &Units::Stretch(val) => {
                            row_stretch_sum += val;
                        }

                        _ => {}
                    }
                }

                for (i, col) in grid_cols.iter().enumerate() {
                    match col {
                        &Units::Pixels(val) => {
                            col_widths[i].1 = val;
                            col_free_space -= val;
                        }

                        &Units::Stretch(val) => {
                            col_stretch_sum += val;
                        }

                        _ => {}
                    }
                }


                if row_stretch_sum == 0.0 {
                    row_stretch_sum = 1.0;
                }
                if col_stretch_sum == 0.0 {
                    col_stretch_sum = 1.0;
                }


                let mut current_row_pos = state.data.get_posy(parent);
                let mut current_col_pos = state.data.get_posx(parent);

                for (i, row) in grid_rows.iter().enumerate() {
                    match row {
                        &Units::Stretch(val) => {
                            row_heights[i].1 = row_free_space * val / row_stretch_sum;
                        }

                        _ => {}
                    }

                    row_heights[i].0 = current_row_pos;
                    current_row_pos += row_heights[i].1;
                }
                let row_heights_len = row_heights.len() - 1;
                row_heights[row_heights_len].0 = current_row_pos;

                for (i, col) in grid_cols.iter().enumerate() {
                    match col {
                        &Units::Stretch(val) => {
                            col_widths[i].1 = col_free_space * val / col_stretch_sum;
                        }

                        _ => {}
                    }

                    col_widths[i].0 = current_col_pos;

                    current_col_pos += col_widths[i].1;
                }

                let col_widths_len = col_widths.len() - 1;
                col_widths[col_widths_len].0 = current_col_pos;

                for child in parent.child_iter(&tree) {

                    let child_display = child.get_display(state);
                    if child_display == Display::None {
                        continue;
                    }

                    let grid_item = state
                        .style
                        .grid_item
                        .get(child)
                        .cloned()
                        .unwrap_or_default();

                    let row_start = grid_item.row_index as usize;
                    let row_end = row_start + grid_item.row_span as usize;

                    let col_start = grid_item.col_index as usize;
                    let col_end = col_start + grid_item.col_span as usize;

                    if col_start == 0 {
                        state.data.set_posx(child, col_widths[col_start].0);
                        state.data.set_width(
                            child,
                            (col_widths[col_end].0 - col_widths[col_start].0)
                                - col_between / 2.0,
                        );
                    } else if col_end + 1 == col_widths.len() {
                        state
                            .data
                            .set_posx(child, col_widths[col_start].0 + (col_between / 2.0));
                        state.data.set_width(
                            child,
                            (col_widths[col_end].0 - col_widths[col_start].0)
                                - col_between / 2.0,
                        );
                    } else {
                        state
                            .data
                            .set_posx(child, col_widths[col_start].0 + (col_between / 2.0));
                        state.data.set_width(
                            child,
                            (col_widths[col_end].0 - col_widths[col_start].0) - col_between,
                        );
                    }

                    if row_start == 0 {
                        state.data.set_posy(child, row_heights[row_start].0);
                        state.data.set_height(
                            child,
                            (row_heights[row_end].0 - row_heights[row_start].0)
                                - row_between / 2.0,
                        );
                    } else if row_end + 1 == row_heights.len() {
                        state
                            .data
                            .set_posy(child, row_heights[row_start].0 + (row_between / 2.0));
                        state.data.set_height(
                            child,
                            (row_heights[row_end].0 - row_heights[row_start].0)
                                - row_between / 2.0,
                        );
                    } else {
                        state
                            .data
                            .set_posy(child, row_heights[row_start].0 + (row_between / 2.0));
                        state.data.set_height(
                            child,
                            (row_heights[row_end].0 - row_heights[row_start].0) - row_between,
                        );
                    }
                }
            }

            _ => {}
        }
    }
}
*/
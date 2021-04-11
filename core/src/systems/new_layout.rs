

use crate::{Entity, State};

use crate::hierarchy::*;
use crate::style::*;

#[derive(Debug)]
enum Axis {
    Before,
    Size,
    After,
}


pub fn apply_layout2(state: &mut State, hierarchy: &Hierarchy) {
    
    let layout_hierarchy = hierarchy.into_iter().collect::<Vec<Entity>>();

    for entity in layout_hierarchy.iter() {
        state.data.set_child_sum(*entity, 0.0);
        state.data.set_child_max(*entity, 0.0);
    }

    // Walk up the hierarchy
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

        let parent_layout_type = state.style.layout_type.get(parent).cloned().unwrap_or_default();

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let mut left = state.style.left.get(*child).cloned().unwrap_or_default();
        let width = state.style.width.get(*child).cloned().unwrap_or_default();
        let mut right = state.style.right.get(*child).cloned().unwrap_or_default();

        let mut top = state.style.top.get(*child).cloned().unwrap_or_default();
        let height = state.style.height.get(*child).cloned().unwrap_or_default();
        let mut bottom = state.style.bottom.get(*child).cloned().unwrap_or_default();

        let min_left = state.style.min_left.get(*child).cloned().unwrap_or_default().get_value_or(parent_width, 0.0);
        let min_width = state.style.min_width.get(*child).cloned().unwrap_or_default().get_value_or(parent_width, 0.0);
        let min_right = state.style.min_right.get(*child).cloned().unwrap_or_default().get_value_or(parent_width, 0.0);

        let max_left = state.style.max_left.get(*child).cloned().unwrap_or_default().get_value_or(parent_width, std::f32::INFINITY);
        let max_width = state.style.max_width.get(*child).cloned().unwrap_or_default().get_value_or(parent_width, std::f32::INFINITY);
        let max_right = state.style.max_right.get(*child).cloned().unwrap_or_default().get_value_or(parent_width, std::f32::INFINITY);

        let min_top = state.style.min_top.get(*child).cloned().unwrap_or_default().get_value_or(parent_height, 0.0);
        let min_height = state.style.min_height.get(*child).cloned().unwrap_or_default().get_value_or(parent_height, 0.0);
        let min_bottom = state.style.min_bottom.get(*child).cloned().unwrap_or_default().get_value_or(parent_height, 0.0);

        let max_top = state.style.max_top.get(*child).cloned().unwrap_or_default().get_value_or(parent_height, std::f32::INFINITY);
        let max_height = state.style.max_height.get(*child).cloned().unwrap_or_default().get_value_or(parent_height, std::f32::INFINITY);
        let max_bottom = state.style.max_bottom.get(*child).cloned().unwrap_or_default().get_value_or(parent_height, std::f32::INFINITY);


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

            _=> {}
        }

        match width {
            Units::Pixels(val) => {
                new_width = val.clamp(min_width, max_width);
                horizontal_used_space += new_width;
            }

            _=> {}
        }

        match right {
            Units::Pixels(val) => {
                new_right = val.clamp(min_right, max_right);
                horizontal_used_space += new_right;
            }

            _=> {}
        }

        match top {
            Units::Pixels(val) => {
                new_top = val.clamp(min_top, max_top);
                vertical_used_space += new_top;
            }

            _=> {}
        }

        match height {
            Units::Pixels(val) => {
                new_height = val.clamp(min_height, max_height);
                vertical_used_space += new_height;
            }

            _=> {}
        }

        match bottom {
            Units::Pixels(val) => {
                new_bottom = val.clamp(min_bottom, max_bottom);
                vertical_used_space += new_bottom;
            }

            _=> {}
        }

        match parent_layout_type {
            LayoutType::Vertical => {
                state.data.set_child_sum(parent, state.data.get_child_sum(parent) + vertical_used_space);
                state.data.set_child_max(parent, horizontal_used_space.max(state.data.get_child_max(parent)));
            }

            LayoutType::Horizontal => {
                state.data.set_child_sum(parent, state.data.get_child_sum(parent) + horizontal_used_space);
                state.data.set_child_max(parent, vertical_used_space.max(state.data.get_child_max(parent)));
            }

            _=> {}
        }

        state.data.set_height(*child, new_height);
        state.data.set_width(*child, new_width);
        state.data.set_space_top(*child, new_top);
        state.data.set_space_bottom(*child, new_bottom);
        state.data.set_space_left(*child, new_left);
        state.data.set_space_right(*child, new_right);

    }
    
    // Depth first traversal of all nodes from root
    for parent in layout_hierarchy.into_iter() {

        let parent_layout_type = state.style.layout_type.get(parent).cloned().unwrap_or_default();


        let main_before_first = state.style.main_before_first.get(parent).cloned().unwrap_or_default();
        let main_between = state.style.main_between.get(parent).cloned().unwrap_or_default();
        let main_after_last = state.style.main_after_last.get(parent).cloned().unwrap_or_default();

        let cross_before_first = state.style.cross_before_first.get(parent).cloned().unwrap_or_default();
        let cross_between = state.style.cross_between.get(parent).cloned().unwrap_or_default();
        let cross_after_last = state.style.cross_after_last.get(parent).cloned().unwrap_or_default();


        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let (parent_main, parent_cross ) = match parent_layout_type {
            LayoutType::Vertical => {
                (parent_height, parent_width)
            }

            LayoutType::Horizontal | LayoutType::Grid | LayoutType::None => {
                (parent_width, parent_height)
            }
        };

        let mut main_free_space = parent_main;
        let mut main_stretch_sum: f32 = 0.0;

        match parent_layout_type {
            LayoutType::Horizontal | LayoutType::Vertical => {

                let mut horizontal_axis = Vec::new();
                let mut vertical_axis = Vec::new();

                // ////////////////////////////////
                // Calculate inflexible children //
                ///////////////////////////////////
                for child in parent.child_iter(&hierarchy) {

                    let mut left = state.style.left.get(child).cloned().unwrap_or_default();
                    let width = state.style.width.get(child).cloned().unwrap_or_default();
                    let mut right = state.style.right.get(child).cloned().unwrap_or_default();

                    let mut top = state.style.top.get(child).cloned().unwrap_or_default();
                    let height = state.style.height.get(child).cloned().unwrap_or_default();
                    let mut bottom = state.style.bottom.get(child).cloned().unwrap_or_default();

                    let min_left = state.style.min_left.get(child).cloned().unwrap_or_default().get_value_or(parent_width, 0.0);
                    let min_width = state.style.min_width.get(child).cloned().unwrap_or_default().get_value_or(parent_width, 0.0);
                    let min_right = state.style.min_right.get(child).cloned().unwrap_or_default().get_value_or(parent_width, 0.0);

                    let max_left = state.style.max_left.get(child).cloned().unwrap_or_default().get_value_or(parent_width, std::f32::INFINITY);
                    let max_width = state.style.max_width.get(child).cloned().unwrap_or_default().get_value_or(parent_width, std::f32::INFINITY);
                    let max_right = state.style.max_right.get(child).cloned().unwrap_or_default().get_value_or(parent_width, std::f32::INFINITY);

                    let min_top = state.style.min_top.get(child).cloned().unwrap_or_default().get_value_or(parent_height, 0.0);
                    let min_height = state.style.min_height.get(child).cloned().unwrap_or_default().get_value_or(parent_height, 0.0);
                    let min_bottom = state.style.min_bottom.get(child).cloned().unwrap_or_default().get_value_or(parent_height, 0.0);

                    let max_top = state.style.max_top.get(child).cloned().unwrap_or_default().get_value_or(parent_height, std::f32::INFINITY);
                    let max_height = state.style.max_height.get(child).cloned().unwrap_or_default().get_value_or(parent_height, std::f32::INFINITY);
                    let max_bottom = state.style.max_bottom.get(child).cloned().unwrap_or_default().get_value_or(parent_height, std::f32::INFINITY);

                    // Parent overrides
                    match parent_layout_type {
                        LayoutType::Vertical => {
                            if hierarchy.get_first_child(parent) == Some(child) {
                                if top == Units::Auto {
                                    top = main_before_first.clone();
                                }
                            } else {
                                if top == Units::Auto {
                                    top = main_between.clone();
                                }
                            }

                            if hierarchy.get_last_child(parent) == Some(child) {
                                if bottom == Units::Auto {
                                    bottom = main_after_last.clone();
                                }
                            }

                            if left == Units::Auto {
                                left = cross_before_first.clone();
                            }

                            if right == Units::Auto {
                                right = cross_after_last.clone();
                            }


                        }

                        LayoutType::Horizontal => {
                            if hierarchy.get_first_child(parent) == Some(child) {
                                if left == Units::Auto {
                                    left = main_before_first.clone();
                                }
                            } else {
                                if left == Units::Auto {
                                    left = main_between.clone();
                                }
                            }

                            if hierarchy.get_last_child(parent) == Some(child) {
                                if right == Units::Auto {
                                    right = main_after_last.clone();
                                }
                            }

                            if top == Units::Auto {
                                top = cross_before_first.clone();
                            }

                            if bottom == Units::Auto {
                                bottom = cross_after_last.clone();
                            }
                        }

                        _=> {}
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
                    let cross_free_space;

                    let child_layout_type = state.style.layout_type.get(child).cloned().unwrap_or_default();

                    // TODO - replace all these match' with a function
                    match left {
                        Units::Pixels(val) => {
                            new_left = val.clamp(min_left, max_left);
                            horizontal_used_space += new_left;
                        }

                        Units::Stretch(val) => {
                            horizontal_stretch_sum += val;
                            horizontal_axis.push((child, val, min_left, max_left, Axis::Before));
                        }

                        _=> {}
                    }

                    match width {
                        Units::Pixels(val) => {
                            new_width = val.clamp(min_width, max_width);
                            horizontal_used_space += new_width;
                        }

                        Units::Stretch(val) => {
                            horizontal_stretch_sum += val;
                            horizontal_axis.push((child, val, min_width, max_width, Axis::Size));
                        }

                        Units::Auto => {
                            match child_layout_type {
                                LayoutType::Vertical => {
                                    new_width = state.data.get_child_max(child);
                                }

                                LayoutType::Horizontal => {
                                    new_width = state.data.get_child_sum(child);
                                }

                                _=> {}
                            }

                            horizontal_used_space += new_width;
                        }

                        _=> {}
                    }

                    match right {
                        Units::Pixels(val) => {
                            new_right = val.clamp(min_right, max_right);
                            horizontal_used_space += new_right;
                        }

                        Units::Stretch(val) => {
                            horizontal_stretch_sum += val;
                            horizontal_axis.push((child, val, min_right, max_right, Axis::After));
                        }

                        _=> {}
                    }

                    match top {
                        Units::Pixels(val) => {
                            new_top = val.clamp(min_top, max_top);
                            vertical_used_space += new_top;
                        }

                        Units::Stretch(val) => {
                            vertical_stretch_sum += val;
                            vertical_axis.push((child, val, min_top, max_top, Axis::Before));
                        }

                        _=> {}
                    }

                    match height {
                        Units::Pixels(val) => {
                            new_height = val.clamp(min_height, max_height);
                            vertical_used_space += new_height;
                        }

                        Units::Stretch(val) => {
                            vertical_stretch_sum += val;
                            vertical_axis.push((child, val, min_height, max_height, Axis::Size));
                        }

                        Units::Auto => {
                        
                            match child_layout_type {
                                LayoutType::Vertical => {
                                    new_height = state.data.get_child_sum(child);
                                }

                                LayoutType::Horizontal => {
                                    new_height = state.data.get_child_max(child);
                                    
                                }

                                _=> {}
                            }

                            vertical_used_space += new_height;
                        }

                        _=> {}
                    }

                    match bottom {
                        Units::Pixels(val) => {
                            new_bottom = val.clamp(min_bottom, max_bottom);
                            vertical_used_space += val;
                        }

                        Units::Stretch(val) => {
                            vertical_stretch_sum += val;
                            vertical_axis.push((child, val, min_bottom, max_bottom, Axis::After));
                        }

                        _=> {}
                    }
                    
                    state.data.set_height(child, new_height);
                    state.data.set_width(child, new_width);
                    state.data.set_space_top(child, new_top);
                    state.data.set_space_bottom(child, new_bottom);
                    state.data.set_space_left(child, new_left);
                    state.data.set_space_right(child, new_right);
                    
                    

                    match parent_layout_type {
                        LayoutType::Vertical => {
                            cross_stretch_sum += horizontal_stretch_sum;
                            main_stretch_sum += vertical_stretch_sum;
                            main_free_space -= vertical_used_space;
                            cross_free_space = parent_cross - horizontal_used_space;
                        }

                        _=> {
                            cross_stretch_sum += vertical_stretch_sum;
                            main_stretch_sum += horizontal_stretch_sum;
                            main_free_space -= horizontal_used_space;
                            cross_free_space = parent_cross - vertical_used_space;
                        }
                    }

                    cross_stretch_sum = cross_stretch_sum.max(1.0);
                    state.data.set_cross_stretch_sum(child, cross_stretch_sum);
                    state.data.set_cross_free_space(child, cross_free_space);
                }

                main_stretch_sum = main_stretch_sum.max(1.0);

                horizontal_axis.sort_by(|a, b| {a.3.partial_cmp(&b.3).unwrap()});
                vertical_axis.sort_by(|a, b| {a.3.partial_cmp(&b.3).unwrap()});


                let (mut horizontal_stretch_sum, 
                    mut horizontal_free_space,
                    mut vertical_stretch_sum,
                    mut vertical_free_space) = match parent_layout_type {
                   LayoutType::Vertical => {
                       (0.0,
                        0.0,
                        main_stretch_sum,
                        main_free_space)
                   }

                   _=> {
                       (main_stretch_sum, 
                        main_free_space,
                        0.0,
                        0.0)
                   }
                };

                // Calculate flexible horizontal space & size 
                for (child, value, min_value, max_value, variant) in horizontal_axis.iter() {

                    let cross_stretch_sum = state.data.get_cross_stretch_sum(*child);
                    let cross_free_space = state.data.get_cross_free_space(*child);

                    match parent_layout_type {
                        LayoutType::Vertical => {
                            horizontal_stretch_sum = cross_stretch_sum;
                            horizontal_free_space = cross_free_space;
                        }

                        _=> {}
                    };


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
                    
                    horizontal_free_space -= new_value;
                    horizontal_stretch_sum -= value;

                    match parent_layout_type {
                        LayoutType::Vertical => {
                            state.data.set_cross_stretch_sum(*child, horizontal_stretch_sum);
                            state.data.set_cross_free_space(*child, horizontal_free_space);
                        }

                        _=> {}
                    };

                }

                // Calculate flexible vertical space & size 
                for (child, value, min_value, max_value, variant) in vertical_axis.iter() {
                    let cross_stretch_sum = state.data.get_cross_stretch_sum(*child);
                    let cross_free_space = state.data.get_cross_free_space(*child);
                
                    match parent_layout_type {
                        LayoutType::Horizontal => {
                            vertical_stretch_sum = cross_stretch_sum;
                            vertical_free_space = cross_free_space;
                        }

                        _=> {}
                    };


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

                    vertical_free_space -= new_value;
                    vertical_stretch_sum -= value;

                    match parent_layout_type {
                        LayoutType::Horizontal => {
                            state.data.set_cross_stretch_sum(*child, vertical_stretch_sum);
                            state.data.set_cross_free_space(*child, vertical_free_space);
                        }

                        _=> {}
                    };



                }

                let mut current_posx = 0.0;
                let mut current_posy = 0.0;

                let parent_posx = state.data.get_posx(parent);
                let parent_posy = state.data.get_posy(parent);
                
                ///////////////////////
                // Position Children //
                ///////////////////////
                for child in parent.child_iter(&hierarchy) {

                    let space = state.data.get_space(child);

                    let width = state.data.get_width(child);
                    let height = state.data.get_height(child);

                    let new_posx = parent_posx + current_posx + space.left;
                    let new_posy = parent_posy + current_posy + space.top;

                    match parent_layout_type {
                        LayoutType::Vertical => {
                            current_posy += space.top + height + space.bottom;
                        }

                        LayoutType::Horizontal => {
                            current_posx += space.left + width + space.right;
                        }

                        _=> {}
                    }
                    
                    state.data.set_posx(child, new_posx);
                    state.data.set_posy(child, new_posy);
                }
            
            }
        
            LayoutType::Grid => {


                let grid_rows = state.style.grid_rows.get(parent).cloned().unwrap_or_default();
                let grid_cols = state.style.grid_cols.get(parent).cloned().unwrap_or_default();

                let mut row_heights = vec![(0.0,0.0,0.0,0.0); grid_rows.items.len() + 1];
                let mut col_widths = vec![(0.0,0.0,0.0,0.0); grid_cols.items.len() + 1];

                let mut row_free_space = state.data.get_height(parent);
                let mut col_free_space = state.data.get_width(parent);

                let mut row_stretch_sum = 0.0;
                let mut col_stretch_sum = 0.0;

                let space_before_first = match grid_rows.align.space_before_first {
                    Units::Pixels(val) => {
                        val
                    }

                    _=> {0.0}
                };

                let row_space_between = match grid_rows.align.space_between {
                    Units::Pixels(val) => {
                        val
                    }

                    _=> {0.0}
                };


                for (i, row) in grid_rows.items.iter().enumerate() {
                    
                    match row {
                        &Units::Pixels(val) => {
                            row_heights[i].1 = val;
                            row_free_space -= val;
                        }

                        &Units::Stretch(val) => {
                            row_stretch_sum += val;
                        }

                        _=> {}
                    }
                }

                let col_space_between = match grid_cols.align.space_between {
                    Units::Pixels(val) => {
                        val
                    }

                    _=> {0.0}
                };

                for (i, col) in grid_cols.items.iter().enumerate() {
                    match col {
                        &Units::Pixels(val) => {
                            col_widths[i].1 = val;
                            col_free_space -= val; 
                        }

                        &Units::Stretch(val) => {
                            col_stretch_sum += val;
                        }

                        _=> {}
                    }
                }

                row_stretch_sum = row_stretch_sum.max(1.0);
                col_stretch_sum = col_stretch_sum.max(1.0);

                let mut current_row_pos = state.data.get_posx(parent);
                let mut current_col_pos = state.data.get_posy(parent);

                for (i, row) in grid_rows.items.iter().enumerate() {
                    match row {
                        &Units::Stretch(val) => {
                            row_heights[i].1 = row_free_space * val / row_stretch_sum;
                        }

                        _=> {}
                    }

                    row_heights[i].0 = current_row_pos;
                    current_row_pos += row_heights[i].1;

                }
                let row_heights_len = row_heights.len() - 1;
                row_heights[row_heights_len].0 = current_row_pos;

                for (i, col) in grid_cols.items.iter().enumerate() {
                    match col {
                        &Units::Stretch(val) => {
                            col_widths[i].1 = col_free_space * val / col_stretch_sum;
                        }

                        _=> {}
                    }


                    col_widths[i].0 = current_col_pos;
                    
                    current_col_pos += col_widths[i].1;
                }

                let col_widths_len = col_widths.len() - 1;
                col_widths[col_widths_len].0 = current_col_pos;


                for child in parent.child_iter(&hierarchy) {
                    let grid_item = state.style.grid_item.get(child).cloned().unwrap_or_default();

                    let row_start = grid_item.row_index as usize;
                    let row_end = row_start + grid_item.row_span as usize;

                    let col_start = grid_item.col_index as usize;
                    let col_end = col_start + grid_item.col_span as usize;

                    if col_start == 0 {
                        state.data.set_posx(child, col_widths[col_start].0);
                        state.data.set_width(child, (col_widths[col_end].0 - col_widths[col_start].0) - col_space_between/2.0);
                    } else if col_end+1 == col_widths.len() {
                        state.data.set_posx(child, col_widths[col_start].0 + (col_space_between / 2.0));
                        state.data.set_width(child, (col_widths[col_end].0 - col_widths[col_start].0) - col_space_between/2.0);
                    } else {
                        state.data.set_posx(child, col_widths[col_start].0 + (col_space_between / 2.0));
                        state.data.set_width(child, (col_widths[col_end].0 - col_widths[col_start].0) - col_space_between);
                    }

                    if row_start == 0 {
                        state.data.set_posy(child, row_heights[row_start].0);
                        state.data.set_height(child, (row_heights[row_end].0 - row_heights[row_start].0) - row_space_between/2.0);
                    } else if row_end+1 == row_heights.len() {
                        state.data.set_posy(child, row_heights[row_start].0 + (row_space_between / 2.0));
                        state.data.set_height(child, (row_heights[row_end].0 - row_heights[row_start].0) - row_space_between/2.0);
                    } else {
                        state.data.set_posy(child, row_heights[row_start].0 + (row_space_between / 2.0));
                        state.data.set_height(child, (row_heights[row_end].0 - row_heights[row_start].0) - row_space_between);
                    }
                }

            }  
            
            _=> {}
        }


    
    }
}



use prop::PropGet;

use crate::{Entity, Event, GeometryChanged, Propagation, State, WindowEvent};

use crate::hierarchy::*;
use crate::style::*;

use crate::flexbox::AlignItems;


pub fn apply_layout2(state: &mut State, hierarchy: &Hierarchy) {
    
    let layout_hierarchy = hierarchy.into_iter().collect::<Vec<Entity>>();

    
    for parent in layout_hierarchy.into_iter() {
        //let parent_flex_direction = parent.get_flex_direction(state);

        let parent_layout_type = state.style.layout_type.get(parent).cloned().unwrap_or_default();

        let main_axis_align = state.style.main_axis_align.get(parent).cloned().unwrap_or_default();
        let cross_axis_align = state.style.cross_axis_align.get(parent).cloned().unwrap_or_default();

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let (mut main_free_space, mut cross_free_space) = match parent_layout_type {
            LayoutType::Vertical => {
                (parent_height, parent_width)
            }

            LayoutType::Horizontal | LayoutType::Grid => {
                (parent_width, parent_height)
            }
        };

        let mut main_stretch_sum = 0.0;

        match parent_layout_type {
            LayoutType::Horizontal | LayoutType::Vertical => {
                // Calculate inflexible items
                for child in parent.child_iter(&hierarchy) {
                    let mut main_axis = state.style.main_axis.get(child).cloned().unwrap_or_default();
                    let mut cross_axis = state.style.cross_axis.get(child).cloned().unwrap_or_default();



                    if hierarchy.get_first_child(parent) == Some(child) {
                        if main_axis.space_before == Units::Inherit {
                            main_axis.space_before = main_axis_align.space_before_first.clone();
                        }
                    } else {
                        if main_axis.space_before == Units::Inherit {
                            main_axis.space_before = main_axis_align.space_between.clone();
                        }
                    } 


                    if hierarchy.get_last_child(parent) == Some(child) {
                        if main_axis.space_after == Units::Inherit {
                            main_axis.space_after = main_axis_align.space_after_last.clone();
                        }
                    } 


                    let mut cross_stretch_sum = 0.0;

                    let mut new_main = 0.0;
                    let mut new_cross = 0.0;

                
                    let mut main_space_before = 0.0;
                    let mut main_space_after = 0.0;
                    let mut cross_space_before = 0.0;
                    let mut cross_space_after = 0.0;


                    match main_axis.space_before {
                        Units::Pixels(val) => {
                            main_free_space -= val;
                            main_space_before = val;

                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match main_axis.size {
                        Units::Pixels(val) => {
                            new_main = val;
                            main_free_space -= new_main;
                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match main_axis.space_after {
                        Units::Pixels(val) => {
                            main_free_space -= val;
                            main_space_after = val;
                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match cross_axis.space_before {
                        Units::Pixels(val) => {
                            //cross_free_space -= val;
                            cross_space_before = val;
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match cross_axis.size {
                        Units::Pixels(val) => {
                            new_cross = val;
                            //cross_free_space -= new_width;
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match cross_axis.space_after {
                        Units::Pixels(val) => {
                            cross_space_after = val;
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                        }

                        _=> {}
                    }

                    
                
                    match parent_layout_type {
                        LayoutType::Vertical => {
                            state.data.set_height(child, new_main);
                            state.data.set_width(child, new_cross);

                            state.data.set_space_top(child, main_space_before);
                            state.data.set_space_bottom(child, main_space_after);
                            state.data.set_space_left(child, cross_space_before);
                            state.data.set_space_right(child, cross_space_after);
                        }

                        LayoutType::Horizontal => {
                            state.data.set_height(child, new_cross);
                            state.data.set_width(child, new_main);

                            state.data.set_space_top(child, cross_space_before);
                            state.data.set_space_bottom(child, cross_space_after);
                            state.data.set_space_left(child, main_space_before);
                            state.data.set_space_right(child, main_space_after);
                        }

                        _=> {}
                    }

                    cross_stretch_sum = cross_stretch_sum.max(1.0);

                    state.data.set_cross_stretch_sum(child, cross_stretch_sum);

                    //println!("nw: {}  nh: {}", new_width, new_height);
                }
            
                main_stretch_sum = main_stretch_sum.max(1.0);
                

                //println!("cross_free_space: {}", cross_free_space);

                // Calculate flexible items
                for child in parent.child_iter(&hierarchy) {
                    let mut main_axis = state.style.main_axis.get(child).cloned().unwrap_or_default();
                    let cross_axis = state.style.cross_axis.get(child).cloned().unwrap_or_default();

                    //let mut new_width = state.data.get_width(child);
                    //let mut new_height = state.data.get_height(child);

                    let cross_stretch_sum = state.data.get_cross_stretch_sum(child);

                    if hierarchy.get_first_child(parent) == Some(child) {
                        if main_axis.space_before == Units::Inherit {
                            main_axis.space_before = main_axis_align.space_before_first.clone();
                        }
                    } else {
                        if main_axis.space_before == Units::Inherit {
                            main_axis.space_before = main_axis_align.space_between.clone();
                        }
                    } 


                    if hierarchy.get_last_child(parent) == Some(child) {
                        if main_axis.space_after == Units::Inherit {
                            main_axis.space_after = main_axis_align.space_after_last.clone();
                        }
                    } 

                    let (   mut new_main, 
                            mut new_cross,
                            mut main_space_before,
                            mut main_space_after,
                            mut cross_space_before,
                            mut cross_space_after,
                        ) = match parent_layout_type {
                        LayoutType::Vertical => {
                            (   
                                state.data.get_height(child), 
                                state.data.get_width(child),
                                state.data.get_space_top(child),
                                state.data.get_space_bottom(child),
                                state.data.get_space_left(child),
                                state.data.get_space_right(child),
                            
                            )
                        }

                        LayoutType::Horizontal | LayoutType::Grid => {
                            (   
                                state.data.get_width(child), 
                                state.data.get_height(child),
                                state.data.get_space_left(child),
                                state.data.get_space_right(child),
                                state.data.get_space_top(child),
                                state.data.get_space_bottom(child),
                            )
                        }
                    };

                    match main_axis.space_before {
                        Units::Stretch(val) => {
                            main_space_before = main_free_space * val / main_stretch_sum;
                        }

                        _=> {}
                    }

                    match main_axis.size {
                        Units::Stretch(val) => {
                            new_main = main_free_space * val / main_stretch_sum;
                        }

                        _=> {}
                    }

                    match main_axis.space_after {
                        Units::Stretch(val) => {
                            main_space_after = main_free_space * val / main_stretch_sum;
                        }

                        _=> {}
                    }

                    match cross_axis.space_before {
                        Units::Stretch(val) => {
                            cross_space_before = cross_free_space * val / cross_stretch_sum;
                            
                        }

                        _=> {}
                    }

                    match cross_axis.size {
                        Units::Stretch(val) => {
                            new_cross = cross_free_space * val / cross_stretch_sum;
                        }

                        _=> {}
                    }

                    match cross_axis.space_after {
                        Units::Stretch(val) => {
                            cross_space_after = cross_free_space * val / cross_stretch_sum;
                        }

                        _=> {}
                    }
                        

                    match parent_layout_type {
                        LayoutType::Vertical => {
                            state.data.set_height(child, new_main);
                            state.data.set_width(child, new_cross);

                            state.data.set_space_top(child, main_space_before);
                            state.data.set_space_bottom(child, main_space_after);
                            state.data.set_space_left(child, cross_space_before);
                            state.data.set_space_right(child, cross_space_after);
                        }

                        LayoutType::Horizontal => {
                            state.data.set_height(child, new_cross);
                            state.data.set_width(child, new_main);

                            state.data.set_space_top(child, cross_space_before);
                            state.data.set_space_bottom(child, cross_space_after);
                            state.data.set_space_left(child, main_space_before);
                            state.data.set_space_right(child, main_space_after);
                        }

                        _=> {}
                    }
                    

                    //println!("nw: {}  nh: {}", new_width, new_height);

                    //state.data.set_posx(child, 0.0);
                    //state.data.set_width(child, new_width);
                    //state.data.set_posy(child, 0.0);
                    //state.data.set_height(child, new_height);

                }

                let mut current_posx = 0.0;
                let mut current_posy = 0.0;

                let parent_posx = state.data.get_posx(parent);
                let parent_posy = state.data.get_posy(parent);

                for child in parent.child_iter(&hierarchy) {

                    let main_axis = state.style.main_axis.get(child).cloned().unwrap_or_default();
                    let cross_axis = state.style.cross_axis.get(child).cloned().unwrap_or_default();

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
                //let mut row_heights = Vec::new();
                //let mut column_widths = Vec::new();

                let grid_rows = state.style.grid_rows.get(parent).cloned().unwrap_or_default();
                let grid_cols = state.style.grid_cols.get(parent).cloned().unwrap_or_default();



                // let mut row_heights = Vec::with_capacity(grid_rows.items.len());
                // let mut col_widths = Vec::with_capacity(grid_cols.items.len());

                // (posx width space_before space_after)
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

                //println!("{} {:?}", parent, row_heights);
                println!("{} {:?}", parent, col_widths);

                for child in parent.child_iter(&hierarchy) {
                    let grid_item = state.style.grid_item.get(child).cloned().unwrap_or_default();

                    let row_start = grid_item.row_index as usize;
                    let row_end = row_start + grid_item.row_span as usize;

                    let col_start = grid_item.col_index as usize;
                    let col_end = col_start + grid_item.col_span as usize;

                    //println!("Child: {:?} {} {} {} {}", child, row_start, row_end, col_start, col_end);
                    //println!("{} {}", row_space_between, col_space_between);

                    //println!("Child: {:?} {} {} {} {}", child, col_widths[col_start].0, row_heights[row_start].0, col_widths[col_start].1, row_heights[row_start].1);

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

                    // state.data.set_posx(child, col_widths[col_start].0 + (col_space_between / 2.0 * col_start as f32));
                    // state.data.set_posy(child, row_heights[row_start].0 + (row_space_between / 2.0 * row_start as f32));
                    // if col_end + 1 == col_widths.len() {
                    //     state.data.set_width(child, col_widths[col_end].0 - col_widths[col_start].0);
                    // } else {
                    //     state.data.set_width(child, (col_widths[col_end].0 - col_widths[col_start].0) - col_space_between);
                    // }
                    // if row_end + 1 == row_heights.len() {
                    //     state.data.set_height(child, row_heights[row_end].0 - row_heights[row_start].0);
                    // } else {
                    //     state.data.set_height(child, (row_heights[row_end].0 - row_heights[row_start].0) - row_space_between);
                    // }
                    
                }

            }   
        }


    
    }
}
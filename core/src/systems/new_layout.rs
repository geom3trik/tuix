use std::num;

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

        //let main_axis_align = state.style.main_axis_align.get(parent).cloned().unwrap_or_default();
        //let cross_axis_align = state.style.cross_axis_align.get(parent).cloned().unwrap_or_default();


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
        //let mut cross_free_space = parent_cross;

        let mut main_stretch_sum: f32 = 0.0;
        //let mut cross_stretch_sum: f32 = 1.0;

        let mut temp_main_pos = 0.0;

        let mut num_of_wraps = 1;

        let mut wraps: Vec<f32> = Vec::new();
        wraps.push(0.0);

        match parent_layout_type {
            LayoutType::Horizontal | LayoutType::Vertical => {
                // Calculate inflexible items
                for (index, child) in parent.child_iter(&hierarchy).enumerate() {
                    //let mut main_axis = state.style.main_axis.get(child).cloned().unwrap_or_default();
                    //let mut cross_axis = state.style.cross_axis.get(child).cloned().unwrap_or_default();

                    let mut main_before = state.style.main_before.get(child).cloned().unwrap_or_default();
                    let main_size = state.style.main_size.get(child).cloned().unwrap_or_default();
                    let mut main_after = state.style.main_after.get(child).cloned().unwrap_or_default();

                    let mut cross_before = state.style.cross_before.get(child).cloned().unwrap_or_default();
                    let cross_size = state.style.cross_size.get(child).cloned().unwrap_or_default();
                    let mut cross_after = state.style.cross_after.get(child).cloned().unwrap_or_default();


                    // Override child space_before with parent space_before_first if set to Inherit
                    if hierarchy.get_first_child(parent) == Some(child) {
                        if main_before == Units::Auto {
                            main_before = main_before_first.clone();
                        }
                    } else {
                        if main_before == Units::Auto {
                            main_before = main_between.clone();
                        }
                    } 

                    // Override child space_after with parent space_after_last if set to Inherit
                    if hierarchy.get_last_child(parent) == Some(child) {
                        if main_after == Units::Auto {
                            main_after = main_after_last.clone();
                        }
                    }

                    if cross_before == Units::Auto {
                        cross_before = cross_before_first.clone();
                    }

                    if cross_after == Units::Auto {
                        cross_after = cross_after_last.clone();
                    }
                    

                    let mut new_main = 0.0;
                    let mut new_cross = 0.0;

                
                    let mut main_space_before = 0.0;
                    let mut main_space_after = 0.0;
                    let mut cross_space_before = 0.0;
                    let mut cross_space_after = 0.0;

                    let mut cross_stretch_sum = 0.0;
                    let mut cross_used_space = 0.0;


                    match main_before {
                        Units::Pixels(val) => {
                            main_free_space -= val;
                            main_space_before = val;

                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match main_size {
                        Units::Pixels(val) => {
                            new_main = val;
                            main_free_space -= new_main;
                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match main_after {
                        Units::Pixels(val) => {
                            main_free_space -= val;
                            main_space_after = val;
                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                        }

                        _=> {}
                    }

                    //println!("Child {} {}", index, temp_free_space_main);


                    match cross_before {
                        Units::Pixels(val) => {
                            cross_used_space += val;
                            cross_space_before = val;
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match cross_size {
                        Units::Pixels(val) => {
                            new_cross = val;
                            cross_used_space += val;
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += 1.0;
                        }

                        _=> {}
                    }

                    match cross_after {
                        Units::Pixels(val) => {
                            cross_space_after = val;
                            cross_used_space += val;
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                        }

                        _=> {}
                    }

                    if temp_main_pos + main_space_before + new_main + main_space_after >= parent_main {
                        temp_main_pos = 0.0;
                        num_of_wraps += 1;
                        wraps.push(cross_space_before + new_cross + cross_space_after);
                    } else {
                        wraps[num_of_wraps as usize - 1] = wraps[num_of_wraps as usize - 1].max(cross_space_before + new_cross + cross_space_after);
                    }

                    temp_main_pos += main_space_before + new_main + main_space_after;

                    
                
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

                //println!("Number of Rows: {} {:?}", num_of_wraps, wraps);
            
                let mut current_wrap = 0;
                temp_main_pos = 0.0;



                let num_of_stretch_rows = wraps.iter().filter(|&x| *x <= (parent_cross/num_of_wraps as f32)).count();
                let used_space = wraps.iter().filter(|&x| *x > (parent_cross/num_of_wraps as f32)).sum::<f32>();
                
                println!("Num stretch rows: {:?}  Used space: {}", num_of_stretch_rows, used_space);

                wraps.iter_mut().for_each(|x| if *x <= (parent_cross/num_of_wraps as f32) { *x = (parent_cross - used_space)/num_of_stretch_rows as f32 });
                
                println!("Wraps: {:?}", wraps);

                // Calculate flexible items
                for (index, child) in parent.child_iter(&hierarchy).enumerate() {
                    // let mut main_axis = state.style.main_axis.get(child).cloned().unwrap_or_default();
                    // let mut cross_axis = state.style.cross_axis.get(child).cloned().unwrap_or_default();

                    let mut main_before = state.style.main_before.get(child).cloned().unwrap_or_default();
                    let main_size = state.style.main_size.get(child).cloned().unwrap_or_default();
                    let mut main_after = state.style.main_after.get(child).cloned().unwrap_or_default();

                    let mut cross_before = state.style.cross_before.get(child).cloned().unwrap_or_default();
                    let cross_size = state.style.cross_size.get(child).cloned().unwrap_or_default();
                    let mut cross_after = state.style.cross_after.get(child).cloned().unwrap_or_default();

                    //let mut new_width = state.data.get_width(child);
                    //let mut new_height = state.data.get_height(child);

                    let cross_stretch_sum = state.data.get_cross_stretch_sum(child);

                    if hierarchy.get_first_child(parent) == Some(child) {
                        if main_before == Units::Auto {
                            main_before = main_before_first.clone();
                        }
                    } else {
                        if main_before == Units::Auto {
                            main_before = main_between.clone();
                        }
                    } 


                    if hierarchy.get_last_child(parent) == Some(child) {
                        if main_after == Units::Auto {
                            main_after = main_after_last.clone();
                        }
                    } 

                    if cross_before == Units::Auto {
                        cross_before = cross_before_first.clone();
                    }

                    if cross_after == Units::Auto {
                        cross_after = cross_after_last.clone();
                    }

                    let (   mut new_main, 
                            mut new_cross,
                            mut main_space_before,
                            mut main_space_after,
                            mut cross_space_before,
                            mut cross_space_after,
                        ) = match parent_layout_type {
                        LayoutType::Vertical => {
                            
                            //cross_free_space = state.data.get_width(parent);
                            
                            (   
                                state.data.get_height(child), 
                                state.data.get_width(child),
                                state.data.get_space_top(child),
                                state.data.get_space_bottom(child),
                                state.data.get_space_left(child),
                                state.data.get_space_right(child),
                            )
                        }

                        LayoutType::Horizontal | LayoutType::Grid | LayoutType::None => {
                            
                            //cross_free_space = state.data.get_height(parent);
                            
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



                    
                    //println!("Do This: {} {} {}", index, temp_main_pos + main_space_before + new_main + main_space_after, parent_main);
                    if temp_main_pos + main_space_before + new_main + main_space_after >= parent_main {
                        
                        temp_main_pos = 0.0;
                        current_wrap += 1;
                    }

                    temp_main_pos += main_space_before + new_main + main_space_after;

                    //println!("Child: {} {}", index, current_wrap);

                    let cross_free_space = wraps[current_wrap] - new_cross - cross_space_before - cross_space_after;
                    

                    //let cross_used_space = new_cross + cross_space_before + cross_space_after;
                    //let cross_free_space = (parent_cross / num_of_wraps as f32) - cross_used_space;

                    let cross_free_space = parent_cross - new_cross - cross_space_before - cross_space_after;

                    match main_before {
                        Units::Stretch(val) => {
                            main_space_before = main_free_space * val / main_stretch_sum;
                        }

                        _=> {}
                    }

                    match main_size {
                        Units::Stretch(val) => {
                            new_main = main_free_space * val / main_stretch_sum;
                        }

                        _=> {}
                    }

                    match main_after {
                        Units::Stretch(val) => {
                            main_space_after = main_free_space * val / main_stretch_sum;
                        }

                        _=> {}
                    }

                    match cross_before {
                        Units::Stretch(val) => {
                            cross_space_before = cross_free_space * val / cross_stretch_sum;
                            
                        }

                        _=> {}
                    }

                    match cross_size {
                        Units::Stretch(val) => {
                            new_cross = cross_free_space * val / cross_stretch_sum;
                        }

                        _=> {}
                    }

                    match cross_after {
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

                let mut current_wrap = 0;

                // Position Children
                for child in parent.child_iter(&hierarchy) {

                    //let main_axis = state.style.main_axis.get(child).cloned().unwrap_or_default();
                    //let cross_axis = state.style.cross_axis.get(child).cloned().unwrap_or_default();

                    let space = state.data.get_space(child);

                    let width = state.data.get_width(child);
                    let height = state.data.get_height(child);

                    // match parent_layout_type {
                    //     LayoutType::Vertical => {
                    //         current_posy += space.top + height + space.bottom;
                    //     }

                    //     LayoutType::Horizontal => {
                    //         if current_posx + space.left + width + space.right >= parent_posx + parent_width {
                    //             current_posx = 0.0;
                    //             // current_posy += space.top + height + space.bottom;
                    //             current_posy += wraps[current_wrap];
                    //             current_wrap += 1;
                    //         }
                    //     }

                    //     _=> {}
                    // }

                    let new_posx = parent_posx + current_posx + space.left;
                    let new_posy = parent_posy + current_posy + space.top;

                    match parent_layout_type {
                        LayoutType::Vertical => {
                            current_posy += space.top + height + space.bottom;
                        }

                        LayoutType::Horizontal => {
                            current_posx += space.left + width + space.right;
                            //println!("Current PosX: {}", current_posx);
                            // if current_posx >= parent_posx + parent_width {
                            //     //println!("Do This");
                            //     current_posx = 0.0;
           
                            //     current_posy += height;
                            // }
                        }

                        _=> {}
                    }
                    
                    
                    //println!("posx: {} posy: {}", new_posx, new_posy);
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
            
            _=> {}
        }


    
    }
}
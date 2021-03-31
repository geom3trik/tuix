


use prop::PropGet;

use crate::{Entity, Event, GeometryChanged, Propagation, State, WindowEvent};

use crate::hierarchy::*;
use crate::style::*;

use crate::flexbox::AlignItems;


pub fn apply_layout2(state: &mut State, hierarchy: &Hierarchy) {
    
    let layout_hierarchy = hierarchy.into_iter().collect::<Vec<Entity>>();

    
    for parent in layout_hierarchy.into_iter() {
        let parent_flex_direction = parent.get_flex_direction(state);

        let main_axis_align = state.style.main_axis_align.get(parent).cloned().unwrap_or_default();
        let cross_axis_align = state.style.cross_axis_align.get(parent).cloned().unwrap_or_default();

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let (mut main_free_space, mut cross_free_space) = match parent_flex_direction {
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (parent_height, parent_width)
            }

            FlexDirection::Row | FlexDirection::RowReverse => {
                (parent_width, parent_height)
            }
        };

        let mut main_stretch_sum = 0.0;

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




            let mut new_width = 0.0;
            let mut new_height = 0.0;

            let mut cross_stretch_sum = 0.0;

            match parent_flex_direction {
                FlexDirection::Column | FlexDirection::ColumnReverse => {

                    match main_axis.space_before {
                        Units::Pixels(val) => {
                            main_free_space -= val;
                            state.data.set_space_top(child, val);

                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match main_axis.size {
                        Units::Pixels(val) => {
                            new_height = val;
                            main_free_space -= new_height;
                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match main_axis.space_after {
                        Units::Pixels(val) => {
                            main_free_space -= val;
                            state.data.set_space_bottom(child, val);
                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match cross_axis.space_before {
                        Units::Pixels(val) => {
                            //cross_free_space -= val;
                            state.data.set_space_left(child, val);
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match cross_axis.size {
                        Units::Pixels(val) => {
                            new_width = val;
                            //cross_free_space -= new_width;
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                        }

                        _=> {}
                    }

                    match cross_axis.space_after {
                        Units::Pixels(val) => {
                            //cross_free_space -= val;
                            state.data.set_space_right(child, val);
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                        }

                        _=> {}
                    }
                }

                FlexDirection::Row | FlexDirection::RowReverse => {
                    match main_axis.size {
                        Units::Pixels(val) => {
                            new_width = val;
                            main_free_space -= new_width;
                        }

                        _=> {}
                    }

                    match cross_axis.size {
                        Units::Pixels(val) => {
                            new_height = val;
                        }

                        _=> {}
                    }
                }
            }

            state.data.set_posx(child, 0.0);
            state.data.set_width(child, new_width);
            state.data.set_posy(child, 0.0);
            state.data.set_height(child, new_height);

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
        

            match parent_flex_direction {
                FlexDirection::Column | FlexDirection::ColumnReverse => {

                    match main_axis.space_before {
                        Units::Stretch(val) => {
                            state.data.set_space_top(child, main_free_space * val / main_stretch_sum);
                        }

                        _=> {}
                    }

                    match main_axis.size {
                        Units::Stretch(val) => {
                            state.data.set_height(child, main_free_space * val / main_stretch_sum);
                        }

                        _=> {}
                    }

                    match main_axis.space_after {
                        Units::Stretch(val) => {
                            state.data.set_space_bottom(child, main_free_space * val / main_stretch_sum);
                        }

                        _=> {}
                    }

                    match cross_axis.space_before {
                        Units::Stretch(val) => {
                            state.data.set_space_left(child, cross_free_space * val / cross_stretch_sum);
                        }

                        _=> {}
                    }

                    match cross_axis.size {
                        Units::Stretch(val) => {
                            state.data.set_width(child, cross_free_space * val / cross_stretch_sum);
                        }

                        _=> {}
                    }

                    match cross_axis.space_after {
                        Units::Stretch(val) => {
                            state.data.set_space_right(child, cross_free_space * val / cross_stretch_sum);
                        }

                        _=> {}
                    }
                }

                FlexDirection::Row | FlexDirection::RowReverse => {
                    // match main_axis.size {
                    //     Units::Pixels(val) => {
                    //         new_width = val;
                    //     }

                    //     _=> {}
                    // }

                    // match cross_axis.size {
                    //     Units::Pixels(val) => {
                    //         new_height = val;
                    //     }

                    //     _=> {}
                    // }
                }
            }

            //println!("nw: {}  nh: {}", new_width, new_height);

            //state.data.set_posx(child, 0.0);
            //state.data.set_width(child, new_width);
            //state.data.set_posy(child, 0.0);
            //state.data.set_height(child, new_height);

        }

        let mut current_posx = 0.0;
        let mut current_posy = 0.0;

        for child in parent.child_iter(&hierarchy) {

            let main_axis = state.style.main_axis.get(child).cloned().unwrap_or_default();
            let cross_axis = state.style.cross_axis.get(child).cloned().unwrap_or_default();

            let space = state.data.get_space(child);

            let width = state.data.get_width(child);
            let height = state.data.get_height(child);

            let new_posx = current_posx + space.left;
            let new_posy = current_posy + space.top;

            match parent_flex_direction {
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    current_posy += space.top + height + space.bottom;
                }

                FlexDirection::Row | FlexDirection::RowReverse => {
                    current_posx += space.left + width + space.right;
                }
            }
            
            

            state.data.set_posx(child, new_posx);
            state.data.set_posy(child, new_posy);
        }
    }
}
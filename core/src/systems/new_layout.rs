


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



        for child in parent.child_iter(&hierarchy) {
            let main_axis = state.style.main_axis.get(child).cloned().unwrap_or_default();
            let cross_axis = state.style.cross_axis.get(child).cloned().unwrap_or_default();


            let mut new_width = 0.0;
            let mut new_height = 0.0;

            match parent_flex_direction {
                FlexDirection::Column | FlexDirection::ColumnReverse => {
                    match main_axis.size {
                        Units::Pixels(val) => {
                            new_height = val;
                        }

                        _=> {}
                    }
                }

                FlexDirection::Row | FlexDirection::RowReverse => {
                    match main_axis.size {
                        Units::Pixels(val) => {
                            new_width = val;
                        }

                        _=> {}
                    }
                }
            }

            state.data.set_width(entity, val)
        }
    }
}
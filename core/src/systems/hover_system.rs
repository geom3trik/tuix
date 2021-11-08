use morphorm::Cache;

use crate::{Display, Entity, Event, PropGet, PropSet, Propagation, State, Units, Visibility, WindowEvent};

/// Determines the hovered entity based on the mouse cursor position
pub fn apply_hover(state: &mut State) {
    //println!("Apply Hover");
    let mut draw_tree: Vec<Entity> = state.tree.into_iter().collect();

    // This should be cached somewhere probably
    draw_tree.sort_by_cached_key(|entity| state.data.get_z_index(*entity));

    let cursorx = state.mouse.cursorx;
    let cursory = state.mouse.cursory;

    let mut hovered_widget = Entity::root();

    for entity in draw_tree.into_iter() {

        //println!("Entity: {} Display: {:?}", entity, state.data.display.get(entity));
        // Skip invisible widgets
        if state.data.get_visibility(entity) == Visibility::Invisible {
            continue;
        }

        // This shouldn't be here but there's a bug if it isn't
        if state.data.get_opacity(entity) == 0.0 {
            continue;
        }

        // Skip non-displayed widgets
        if state.data.get_display(entity) == Display::None {
            continue;
        }

        // Skip non-hoverable widgets
        if state.data.get_hoverable(entity) != true {
            continue;
        }

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            //Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let posx = state.data.get_posx(entity) - (border_width / 2.0);
        let posy = state.data.get_posy(entity) - (border_width / 2.0);
        let width = state.data.get_width(entity) + (border_width);
        let height = state.data.get_height(entity) + (border_width);

        //let clip_widget = state.data.get_clip_widget(entity);

        let clip_region = state.data.get_clip_region(entity);

        let mut transform = state.data.get_transform(entity);

        //let scale = state.data.get_scale(entity);
        //let mut scale_transform = Transform2D::identity();
        //scale_transform.scale(scale, scale);
        //scale_transform.inverse();

        //transform.premultiply(&scale_transform);

        //transform.translate(-posx - width / 2.0, -posy - height / 2.0);
        transform.inverse();
        //transform.translate(-origin.0, -origin.1);
        //transform.translate(posx + width / 2.0, posy + height / 2.0);
        
        let (cx, cy) = transform.transform_point(cursorx, cursory);

        //println!("cx {} cy {}", cx, cy);
        //transform.inverse();
        //let (clip_x, clip_y) = transform.transform_point(clip_region.x, clip_region.y);
        //let (clip_w, clip_h) = transform.transform_point(clip_region.x + clip_region.w, clip_region.y + clip_region.h);
        // let clip_posx = state.data.get_posx(clip_widget);
        // let clip_posy = state.data.get_posy(clip_widget);
        // let clip_width = state.data.get_width(clip_widget);
        // let clip_height = state.data.get_height(clip_widget);

        //println!("entity: {} {} {} {} {}", entity, posx, posy, cx, cy);
        //println!("entity: {} clip: {:?} tclip: BoundingBox {{ x: {}, y: {}, w: {}, h: {} }}", entity, clip_region, clip_x, clip_y, clip_w, clip_h);

        if cx >= posx
            && cx >= clip_region.x
            && cx < (posx + width)
            && cx < (clip_region.x + clip_region.w)
            && cy >= posy
            && cy >= clip_region.y
            && cy < (posy + height)
            && cy < (clip_region.y + clip_region.h)
        {
            hovered_widget = entity;
            if entity.is_over(state) == false {
                //println!("Over: {}", entity);
                state.insert_event(
                    Event::new(WindowEvent::MouseOver)
                        .target(entity)
                        .propagate(Propagation::Direct),
                );
                
                entity.set_over(state, true);
            }
        } else {
            if entity.is_over(state) == true {
                state.insert_event(
                    Event::new(WindowEvent::MouseOut)
                        .target(entity)
                        .propagate(Propagation::Direct),
                );

                entity.set_over(state, false);
            }
        }
    }

    if hovered_widget != state.hovered {
        // Useful for debugging

        #[cfg(debug_assertions)]
        println!(
            "Hover changed to {:?} parent: {:?}, posx: {}, posy: {} width: {} height: {} z_order: {} {} {}",
            hovered_widget,
            state.tree.get_parent(hovered_widget),
            state.data.get_posx(hovered_widget),
            state.data.get_posy(hovered_widget),
            state.data.get_width(hovered_widget),
            state.data.get_height(hovered_widget),
            state.data.get_z_index(hovered_widget),
            state.data.stack_first_child(hovered_widget),
            state.data.stack_last_child(hovered_widget),
        );

        let cursor = state.style.cursor.get(hovered_widget).cloned().unwrap_or_default();

        hovered_widget.emit(state, WindowEvent::SetCursor(cursor));

        hovered_widget.set_hover(state, true);
        state.hovered.set_hover(state, false);

        // if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(hovered_widget) {
        //     pseudo_classes.set_hover(true);
        // }

        // if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(state.hovered) {
        //     pseudo_classes.set_hover(false);
        // }

        // if state.captured != Entity::null() {
        //     state.insert_event(
        //         Event::new(WindowEvent::MouseOver)
        //             .target(state.captured)
        //             .propagate(Propagation::Direct),
        //     );
        // } else if state.hovered != Entity::new(0, 0) {
        //     state.insert_event(
        //         Event::new(WindowEvent::MouseOver)
        //             .target(state.hovered),
        //     );
        // }

        state.insert_event(Event::new(WindowEvent::MouseEnter).target(hovered_widget));
        state.insert_event(Event::new(WindowEvent::MouseLeave).target(state.hovered));

        Entity::root().restyle(state);

        state.hovered = hovered_widget;
        state.active = Entity::null();

        //state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        //state.needs_restyle = true;
        // state.needs_redraw = true;
    }
}

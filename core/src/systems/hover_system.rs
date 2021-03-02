


use crate::{Entity, State, Event, WindowEvent, Length, Visibility};

/// Determines the hovered entity based on the mouse cursor position
pub fn apply_hover(state: &mut State) {

    let mut draw_hierarchy: Vec<Entity> = state.hierarchy.into_iter().collect();

    // This should be cached somewhere
    draw_hierarchy.sort_by_cached_key(|entity| state.data.get_z_order(*entity));

    let cursorx = state.mouse.cursorx;
    let cursory = state.mouse.cursory;

    let mut hovered_widget = Entity::root();

    for entity in draw_hierarchy.into_iter() {
        // Skip invisible widgets
        if state.data.get_visibility(entity) == Visibility::Invisible {
            continue;
        }

        // This shouldn't be here but there's a bug if it isn't
        if state.data.get_opacity(entity) == 0.0 {
            continue;
        }

        // Skip non-hoverable widgets
        if state.data.get_hoverability(entity) != true {
            continue;
        }

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            //Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let posx = state.data.get_posx(entity) - (border_width / 2.0);
        let posy = state.data.get_posy(entity) - (border_width / 2.0);
        let width = state.data.get_width(entity) + (border_width);
        let height = state.data.get_height(entity) + (border_width);

        let clip_widget = state.data.get_clip_widget(entity);

        let clip_posx = state.data.get_posx(clip_widget);
        let clip_posy = state.data.get_posy(clip_widget);
        let clip_width = state.data.get_width(clip_widget);
        let clip_height = state.data.get_height(clip_widget);

        if cursorx >= posx
            && cursorx >= clip_posx
            && cursorx < (posx + width)
            && cursorx < (clip_posx + clip_width)
            && cursory >= posy
            && cursory >= clip_posy
            && cursory < (posy + height)
            && cursory < (clip_posy + clip_height)
        {
            hovered_widget = entity;
            if let Some(pseudo_classes) =
                state.style.pseudo_classes.get_mut(hovered_widget)
            {
                pseudo_classes.set_over(true);
            }
        } else {
            if let Some(pseudo_classes) =
                state.style.pseudo_classes.get_mut(hovered_widget)
            {
                pseudo_classes.set_over(false);
            }
        }
    }

    if hovered_widget != state.hovered {
        // Useful for debugging

        println!(
            "Hover changed to {:?} parent: {:?}, posx: {}, posy: {} width: {} height: {} z_order: {}",
            hovered_widget,
            state.hierarchy.get_parent(hovered_widget),
            state.data.get_posx(hovered_widget),
            state.data.get_posy(hovered_widget),
            state.data.get_width(hovered_widget),
            state.data.get_height(hovered_widget),
            state.data.get_z_order(hovered_widget),
        );

        if let Some(pseudo_classes) =
            state.style.pseudo_classes.get_mut(hovered_widget)
        {
            pseudo_classes.set_hover(true);
        }

        if let Some(pseudo_classes) =
            state.style.pseudo_classes.get_mut(state.hovered)
        {
            pseudo_classes.set_hover(false);
        }

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

        state.insert_event(
            Event::new(WindowEvent::MouseOver).target(hovered_widget),
        );
        state.insert_event(
            Event::new(WindowEvent::MouseOut).target(state.hovered),
        );

        state.insert_event(
            Event::new(WindowEvent::Restyle).target(Entity::root()),
        );

        state.hovered = hovered_widget;
        state.active = Entity::null();

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
    }
}
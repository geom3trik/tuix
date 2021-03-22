use crate::{BoundingBox, Entity, Event, HierarchyTree, IntoParentIterator, State, WindowEvent};

use crate::hierarchy::*;
use crate::state::animation::*;

pub fn apply_clipping(state: &mut State, hierarchy: &Hierarchy) {
    //println!("Apply Clipping");
    for entity in hierarchy.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        let parent = hierarchy.get_parent(entity).unwrap();

        let parent_clip_region = state.data.get_clip_region(parent);

        if let Some(clip_widget) = state.style.clip_widget.get(entity) {
            let clip_x = state.data.get_posx(*clip_widget);
            let clip_y = state.data.get_posy(*clip_widget);
            let clip_w = state.data.get_width(*clip_widget);
            let clip_h = state.data.get_height(*clip_widget);

            let mut intersection = BoundingBox::default();
            intersection.x = clip_x.max(parent_clip_region.x);
            intersection.y = clip_y.max(parent_clip_region.y);

            intersection.w = if clip_x + clip_w < parent_clip_region.x + parent_clip_region.w {
                clip_x + clip_w - intersection.x
            } else {
                parent_clip_region.x + parent_clip_region.w - -intersection.x
            };

            intersection.h = if clip_y + clip_h < parent_clip_region.y + parent_clip_region.h {
                clip_y + clip_h - intersection.y
            } else {
                parent_clip_region.y + parent_clip_region.h - intersection.y
            };

            state.data.set_clip_region(entity, intersection);
        } else {
            state.data.set_clip_region(entity, parent_clip_region);
        }
    }
}

pub fn apply_visibility(state: &mut State, hierarchy: &Hierarchy) {
    //println!("Apply Visibility");
    let mut draw_hierarchy: Vec<Entity> = hierarchy.into_iter().collect();
    draw_hierarchy.sort_by_cached_key(|entity| state.data.get_z_order(*entity));

    for widget in draw_hierarchy.into_iter() {
        let visibility = state
            .style
            .visibility
            .get(widget)
            .cloned()
            .unwrap_or_default();
        state.data.set_visibility(widget, visibility);

        let opacity = state.style.opacity.get(widget).cloned().unwrap_or_default();

        state.data.set_opacity(widget, opacity.0);

        let display = state.style.display.get(widget).cloned().unwrap_or_default();

        if display == Display::None {
            state.data.set_visibility(widget, Visibility::Invisible);
        }

        if let Some(parent) = widget.parent(hierarchy) {
            let parent_visibility = state.data.get_visibility(parent);
            if parent_visibility == Visibility::Invisible {
                state.data.set_visibility(widget, Visibility::Invisible);
            }
            let parent_display = state.style.display.get(parent).cloned().unwrap_or_default();
            if parent_display == Display::None {
                state.data.set_visibility(widget, Visibility::Invisible);
            }

            let parent_opacity = state.data.get_opacity(parent);

            let opacity = state.style.opacity.get(widget).cloned().unwrap_or_default();

            state.data.set_opacity(widget, opacity.0 * parent_opacity);
        }
    }
}

// Returns true if the widget matches the selector
fn check_match(state: &State, entity: Entity, selector: &Selector) -> bool {
    // Construct the entity selector
    let mut entity_selector = Selector::new();

    // Get the entity id from state
    //entity_selector.id = state.style.ids.get(entity).cloned();
    // let mut s = DefaultHasher::new();
    // entity_selector.id = state.style.ids.get_by_right(&entity).map(|f| {
    //     f.hash(&mut s);
    //     s.finish()
    // });

    // Get the entity element from state
    entity_selector.element = state.style.elements.get(entity).cloned();

    // Get the entity class list from state
    if let Some(class_list) = state.style.classes.get(entity) {
        entity_selector.classes = class_list.clone();
    }

    // Set the pseudoclass selectors
    entity_selector.pseudo_classes = state
        .style
        .pseudo_classes
        .get(entity)
        .cloned()
        .unwrap_or_default();

    if state.active == entity {
        entity_selector.pseudo_classes.set_active(true);
    }

    return selector.matches(&entity_selector);
}

pub fn apply_styles(state: &mut State, hierarchy: &Hierarchy) {
    //println!("RESTYLE");
    // Loop through all entities
    for entity in hierarchy.into_iter() {
        // Skip the root
        if entity == Entity::root() {
            continue;
        }

        // Create a list of style rules that match this entity
        let mut matched_rules: Vec<usize> = Vec::new();

        // Loop through all of the style rules
        'rule_loop: for (index, rule) in state.style.rules.iter().enumerate() {
            let mut relation_entity = entity;
            // Loop through selectors (Should be from right to left)
            // All the selectors need to match for the rule to apply
            'selector_loop: for rule_selector in rule.selectors.iter().rev() {
                // Get the relation of the selector
                match rule_selector.relation {
                    Relation::None => {
                        if !check_match(state, entity, rule_selector) {
                            continue 'rule_loop;
                        }
                    }

                    Relation::Parent => {
                        // Get the parent
                        // Contrust the selector for the parent
                        // Check if the parent selector matches the rule_seletor
                        if let Some(parent) = relation_entity.parent(hierarchy) {
                            if !check_match(state, parent, rule_selector) {
                                continue 'rule_loop;
                            }

                            relation_entity = parent;
                        } else {
                            continue 'rule_loop;
                        }
                    }

                    Relation::Ancestor => {
                        // Walk up the hierarchy
                        // Check if each entity matches the selector
                        // If any of them match, move on to the next selector
                        // If none of them do, move on to the next rule
                        for ancestor in relation_entity.parent_iter(hierarchy) {
                            if ancestor == relation_entity {
                                continue;
                            }

                            if check_match(state, ancestor, rule_selector) {
                                relation_entity = ancestor;

                                continue 'selector_loop;
                            }
                        }

                        continue 'rule_loop;
                    }
                }
            }

            // If all the selectors match then add the rule to the matched rules list
            matched_rules.push(index);
        }

        //println!("Entity: {}, Matched Rules: {:?}", entity, &matched_rules);

        if matched_rules.len() == 0 {
            continue;
        }

        let mut should_relayout = false;
        let mut should_redraw = false;

        // Display
        if state.style.display.link_rule(entity, &matched_rules) {
            //println!("1");
            should_relayout = true;
            should_redraw = true;
        }
        if state.style.visibility.link_rule(entity, &matched_rules) {
            //println!("2");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.z_order.link_rule(entity, &matched_rules) {
            //println!("3");
            should_relayout = true;
            should_redraw = true;
        }

        // Currently doesn't do anything - TODO
        state.style.overflow.link_rule(entity, &matched_rules);

        // Opacity
        if state.style.opacity.link_rule(entity, &matched_rules) {
            //println!("4");
            should_relayout = true;
            should_redraw = true;
        }

        // Positioning
        if state.style.position.link_rule(entity, &matched_rules) {
            //println!("5");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.left.link_rule(entity, &matched_rules) {
            //println!("6");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.right.link_rule(entity, &matched_rules) {
            //println!("7");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.top.link_rule(entity, &matched_rules) {
            //println!("8");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.bottom.link_rule(entity, &matched_rules) {
            //println!("9");
            should_relayout = true;
            should_redraw = true;
        }

        // Size
        if state.style.width.link_rule(entity, &matched_rules) {
            //println!("10");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.height.link_rule(entity, &matched_rules) {
            //println!("11");
            should_relayout = true;
            should_redraw = true;
        }

        // Size Constraints
        if state.style.max_width.link_rule(entity, &matched_rules) {
            //println!("12");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.min_width.link_rule(entity, &matched_rules) {
            //println!("13");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.max_height.link_rule(entity, &matched_rules) {
            //println!("14");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.min_height.link_rule(entity, &matched_rules) {
            //println!("15");
            should_relayout = true;
            should_redraw = true;
        }

        // Margin
        if state.style.margin_left.link_rule(entity, &matched_rules) {
            //println!("16");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.margin_right.link_rule(entity, &matched_rules) {
            //println!("17");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.margin_top.link_rule(entity, &matched_rules) {
            //println!("18");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.margin_bottom.link_rule(entity, &matched_rules) {
            //println!("19");
            should_relayout = true;
            should_redraw = true;
        }

        // Padding
        if state.style.padding_left.link_rule(entity, &matched_rules) {
            //println!("20");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.padding_right.link_rule(entity, &matched_rules) {
            //println!("21");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.padding_top.link_rule(entity, &matched_rules) {
            //println!("22");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.padding_bottom.link_rule(entity, &matched_rules) {
            //println!("23");
            should_relayout = true;
            should_redraw = true;
        }

        // Border
        if state.style.border_width.link_rule(entity, &matched_rules) {
            //println!("24");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.border_color.link_rule(entity, &matched_rules) {
            //println!("25");
            should_redraw = true;
        }

        if state
            .style
            .border_radius_top_left
            .link_rule(entity, &matched_rules)
        {
            //println!("26");
            should_redraw = true;
        }

        if state
            .style
            .border_radius_top_right
            .link_rule(entity, &matched_rules)
        {
            //println!("27");
            should_redraw = true;
        }

        if state
            .style
            .border_radius_bottom_left
            .link_rule(entity, &matched_rules)
        {
            //println!("28");
            should_redraw = true;
        }

        if state
            .style
            .border_radius_bottom_right
            .link_rule(entity, &matched_rules)
        {
            //println!("29");
            should_redraw = true;
        }

        // Flex Container
        if state.style.flex_direction.link_rule(entity, &matched_rules) {
            //println!("30");
            should_relayout = true;
            should_redraw = true;
        }

        if state
            .style
            .justify_content
            .link_rule(entity, &matched_rules)
        {
            //println!("31");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.align_content.link_rule(entity, &matched_rules) {
            //println!("32");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.align_items.link_rule(entity, &matched_rules) {
            //println!("33");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.align_self.link_rule(entity, &matched_rules) {
            //println!("34");
            should_relayout = true;
            should_redraw = true;
        }

        // Flex Item
        if state.style.flex_basis.link_rule(entity, &matched_rules) {
            //println!("35");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.flex_grow.link_rule(entity, &matched_rules) {
            //println!("36");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.flex_shrink.link_rule(entity, &matched_rules) {
            //println!("37");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.align_self.link_rule(entity, &matched_rules) {
            //println!("38");
            should_relayout = true;
            should_redraw = true;
        }

        // Text Alignment
        if state.style.text_align.link_rule(entity, &matched_rules) {
            //println!("39");
            should_redraw = true;
        }

        if state.style.text_justify.link_rule(entity, &matched_rules) {
            //println!("40");
            should_redraw = true;
        }

        // Background
        if state
            .style
            .background_color
            .link_rule(entity, &matched_rules)
        {
            //println!("41");
            should_redraw = true;
        }

        if state
            .style
            .background_image
            .link_rule(entity, &matched_rules)
        {
            //println!("42");
            should_redraw = true;
        }

        // Font
        if state.style.font_color.link_rule(entity, &matched_rules) {
            //println!("43");
            should_redraw = true;
        }

        if state.style.font_size.link_rule(entity, &matched_rules) {
            //println!("44");
            should_redraw = true;
        }

        // Outer Shadow
        if state
            .style
            .outer_shadow_h_offset
            .link_rule(entity, &matched_rules)
        {
            //println!("45");
            should_redraw = true;
        }

        if state
            .style
            .outer_shadow_v_offset
            .link_rule(entity, &matched_rules)
        {
            //println!("46");
            should_redraw = true;
        }

        if state
            .style
            .outer_shadow_blur
            .link_rule(entity, &matched_rules)
        {
            //println!("47");
            should_redraw = true;
        }

        if state
            .style
            .outer_shadow_color
            .link_rule(entity, &matched_rules)
        {
            //println!("48");
            should_redraw = true;
        }

        // Inner Shadow
        if state
            .style
            .inner_shadow_h_offset
            .link_rule(entity, &matched_rules)
        {
            //println!("45");
            should_redraw = true;
        }

        if state
            .style
            .inner_shadow_v_offset
            .link_rule(entity, &matched_rules)
        {
            //println!("46");
            should_redraw = true;
        }

        if state
            .style
            .inner_shadow_blur
            .link_rule(entity, &matched_rules)
        {
            //println!("47");
            should_redraw = true;
        }

        if state
            .style
            .inner_shadow_color
            .link_rule(entity, &matched_rules)
        {
            //println!("48");
            should_redraw = true;
        }

        if should_relayout {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
            //state.needs_relayout = true;
        }

        if should_redraw {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
            //state.needs_redraw = true;
        }
    }
}

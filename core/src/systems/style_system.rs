use crate::{Entity, Event, HierarchyTree, IntoParentIterator, State, WindowEvent};

use crate::hierarchy::*;
use crate::state::animator::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::cell::RefCell;
use std::rc::Rc;

pub fn apply_clipping(state: &mut State, hierarchy: &Hierarchy) {
    //let state = state.shared_state.borrow_mut();
    for entity in hierarchy.into_iter() {
        if entity == Entity::new(0, 0) {
            continue;
        }

        let parent = hierarchy.get_parent(entity).unwrap();

        if let Some(clip_widget) = state.style.borrow_mut().clip_widget.get(entity) {
            state.transform.set_clip_widget(entity, *clip_widget);
        } else {
            let parent_clip_widget = state.transform.get_clip_widget(parent);
            state.transform.set_clip_widget(entity, parent_clip_widget);
        }
    }
}

pub fn apply_visibility(state: &mut State, hierarchy: &Hierarchy) {
    //let state = state.shared_state.borrow_mut();
    let mut draw_hierarchy: Vec<Entity> = hierarchy.into_iter().collect();
    draw_hierarchy.sort_by_cached_key(|entity| state.transform.get_z_order(*entity));

    for widget in draw_hierarchy.into_iter() {
        let visibility = state
            .style
            .borrow_mut()
            .visibility
            .get(widget)
            .cloned()
            .unwrap_or_default();
        state.transform.set_visibility(widget, visibility);

        let opacity = state
            .style
            .borrow_mut()
            .opacity
            .get(widget)
            .cloned()
            .unwrap_or_default();

        state.transform.set_opacity(widget, opacity.0);

        let display = state
            .style
            .borrow_mut()
            .display
            .get(widget)
            .cloned()
            .unwrap_or_default();

        if display == Display::None {
            state
                .transform
                .set_visibility(widget, Visibility::Invisible);
        }

        if let Some(parent) = widget.parent(hierarchy) {
            let parent_visibility = state.transform.get_visibility(parent);
            if parent_visibility == Visibility::Invisible {
                state
                    .transform
                    .set_visibility(widget, Visibility::Invisible);
            }
            let parent_display = state
                .style
                .borrow_mut()
                .display
                .get(parent)
                .cloned()
                .unwrap_or_default();
            if parent_display == Display::None {
                state
                    .transform
                    .set_visibility(widget, Visibility::Invisible);
            }

            let parent_opacity = state.transform.get_opacity(parent);

            let opacity = state
                .style
                .borrow_mut()
                .opacity
                .get(widget)
                .cloned()
                .unwrap_or_default();

            state
                .transform
                .set_opacity(widget, opacity.0 * parent_opacity);
        }
    }
}

// Returns true if the widget matches the selector
fn check_match(state: &mut State, widget: Entity, selector: &Selector) -> bool {
    //let state = state.shared_state.borrow_mut();
    // Construct the widget selector
    let mut widget_selector = Selector::new();

    // Get the widget id from state
    //widget_selector.id = state.style.ids.get(widget).cloned();
    let mut s = DefaultHasher::new();
    widget_selector.id = state.style.borrow_mut().ids.get_by_right(&widget).map(|f| {
        f.hash(&mut s);
        s.finish()
    });

    // Get the widget element from state
    widget_selector.element = state.style.borrow_mut().elements.get(widget).cloned();

    // Get the widget class list from state
    if let Some(class_list) = state.style.borrow_mut().classes.get(widget) {
        widget_selector.classes = class_list.clone();
    }

    // Set the pseudoclass selectors
    widget_selector.pseudo_classes = state
        .style
        .borrow_mut()
        .pseudo_classes
        .get(widget)
        .cloned()
        .unwrap_or_default();

    return selector.matches(&widget_selector);
}

pub fn apply_styles(state: &mut State, hierarchy: &Hierarchy) {
    // Loop through all entities
    for entity in hierarchy.into_iter() {
        if entity == Entity::new(0, 0) {
            continue;
        }

        // Possible point to add Cascading

        // Create a list of style rules that match this widget
        let mut matched_rules: Vec<usize> = Vec::new();

        let rule_selectors = state.style.borrow().rule_selectors.clone();

        // Loop through all of the style rules
        'rule_loop: for (index, selectors) in rule_selectors.iter().enumerate() {
            let mut relation_entity = entity;
            // Loop through selectors (Should be from right to left)
            // All the selectors need to match for the rule to apply
            'selector_loop: for rule_selector in selectors.iter().rev() {
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

        //let mut state = state.shared_state.borrow_mut();

        // Display
        if state
            .style
            .borrow_mut()
            .display
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }
        if state
            .style
            .borrow_mut()
            .visibility
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .z_order
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Currently doesn't do anything - TODO
        state
            .style
            .borrow_mut()
            .overflow
            .link_rule(entity, &matched_rules);

        // Opacity
        if state
            .style
            .borrow_mut()
            .opacity
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Positioning
        if state
            .style
            .borrow_mut()
            .position
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .left
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .right
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .top
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .bottom
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Size
        if state
            .style
            .borrow_mut()
            .width
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .height
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Size Constraints
        if state
            .style
            .borrow_mut()
            .max_width
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .min_width
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .max_height
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .min_height
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Margin
        if state
            .style
            .borrow_mut()
            .margin_left
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .margin_right
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .margin_top
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .margin_bottom
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Padding
        if state
            .style
            .borrow_mut()
            .padding_left
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .padding_right
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .padding_top
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .padding_bottom
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Border
        if state
            .style
            .borrow_mut()
            .border_width
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .border_color
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .border_radius_top_left
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .border_radius_top_right
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .border_radius_bottom_left
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .border_radius_bottom_right
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Flex Container
        if state
            .style
            .borrow_mut()
            .flex_direction
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .justify_content
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .align_content
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .align_items
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .align_self
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Flex Item
        if state
            .style
            .borrow_mut()
            .flex_basis
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .flex_grow
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .flex_shrink
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .align_self
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Text Alignment
        if state
            .style
            .borrow_mut()
            .text_align
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .text_justify
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Background
        if state
            .style
            .borrow_mut()
            .background_color
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .background_image
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Font
        if state
            .style
            .borrow_mut()
            .font_color
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .borrow_mut()
            .font_size
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }
    }
}

/*
pub fn apply_styles2(state: &mut State, hierarchy: &Hierarchy, mut style_entity: Entity) {



    if style_entity == Entity::null() {
        style_entity = Entity::new(0, 0);
    }

    // Iterate down the hierarchy
    for entity in style_entity.into_iter(hierarchy) {

        if entity == Entity::new(0, 0) {
            continue;
        }

        // Create a list of style rules that match this widget
        let mut matched_rules: Vec<usize> = Vec::new();

        // Loop through all of the style rules
        'rule_loop: for (index, selectors) in state.style.rule_selectors.iter().enumerate() {
            let mut relation_entity = entity;
            // Loop through selectors (Should be from right to left)
            // All the selectors need to match for the rule to apply
            'selector_loop: for rule_selector in selectors.iter().rev() {
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
            return;
        }

        // Display
        if state.style.display.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }
        if state.style.visibility.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Currently doesn't do anything - TODO
        state.style.overflow.link_rule(entity, &matched_rules);

        // Opacity
        if state.style.opacity.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Positioning
        if state.style.position.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.left.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.right.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.top.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.bottom.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Size
        if state.style.width.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.height.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Size Constraints
        if state.style.max_width.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.min_width.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.max_height.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.min_height.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Margin
        if state.style.margin_left.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.margin_right.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.margin_top.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.margin_bottom.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Padding
        if state.style.padding_left.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.padding_right.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.padding_top.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.padding_bottom.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Border
        if state.style.border_width.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.border_color.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.border_radius_top_left.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.border_radius_top_right.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.border_radius_bottom_left.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.border_radius_bottom_right.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Flex Container
        if state.style.flex_direction.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .justify_content
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.align_content.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.align_items.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.align_self.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Flex Item
        if state.style.flex_basis.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.flex_grow.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.flex_shrink.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.align_self.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Text Alignment
        if state.style.text_align.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.text_justify.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Background
        if state
            .style
            .background_color
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state
            .style
            .background_image
            .link_rule(entity, &matched_rules)
        {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        // Font
        if state.style.font_color.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }

        if state.style.font_size.link_rule(entity, &matched_rules) {
            state.insert_event(Event::new(WindowEvent::Redraw));
        }
    }



}
*/

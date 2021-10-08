use crate::{BoundingBox, Display, Entity, Overflow, PropGet, PropSet, Property, SelectorRelation, Rule, Selector, State, Tree, TreeExt, Visibility};


pub fn apply_z_ordering(state: &mut State, tree: &Tree) {
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        let parent = tree.get_parent(entity).unwrap();

        if let Some(z_order) = state.style.z_order.get(entity) {
            state.data.set_z_index(entity, *z_order);
        } else {
            let parent_z_order = state.data.get_z_index(parent);
            state.data.set_z_index(entity, parent_z_order);
        }
    }
}

pub fn apply_clipping(state: &mut State, tree: &Tree) {
    //println!("Apply Clipping");
    for entity in tree.into_iter() {
        if entity == Entity::root() {
            continue;
        }

        let parent = tree.get_parent(entity).unwrap();

        let mut parent_clip_region = state.data.get_clip_region(parent);
        let parent_border_width = state.style.border_width.get(parent).cloned().unwrap_or_default().value_or(0.0, 0.0);

        //println!("Parent border width: {}", parent_border_width);
        parent_clip_region.x += parent_border_width / 2.0;
        parent_clip_region.y += parent_border_width / 2.0;
        parent_clip_region.w -= parent_border_width;
        parent_clip_region.h -= parent_border_width;



        let root_clip_region = state.data.get_clip_region(Entity::root());

        if entity.get_overflow(state) == Overflow::Hidden {
            if let Some(clip_widget) = state.style.clip_widget.get(entity).cloned() {
                let clip_widget_border_width = state.style.border_width.get(clip_widget).cloned().unwrap_or_default().value_or(0.0, 0.0);
                let clip_x = state.data.get_posx(clip_widget) + clip_widget_border_width;
                let clip_y = state.data.get_posy(clip_widget) + clip_widget_border_width;
                let clip_w = state.data.get_width(clip_widget) - 2.0 * clip_widget_border_width;
                let clip_h = state.data.get_height(clip_widget) - 2.0 * clip_widget_border_width;

                let mut intersection = BoundingBox::default();
                intersection.x = clip_x.max(parent_clip_region.x);
                intersection.y = clip_y.max(parent_clip_region.y);

                intersection.w = if clip_x + clip_w < parent_clip_region.x + parent_clip_region.w {
                    clip_x + clip_w - intersection.x
                } else {
                    parent_clip_region.x + parent_clip_region.w - intersection.x
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
        } else {
            state.data.set_clip_region(entity, root_clip_region);
        }

        //let clip_region = state.data.get_clip_region(entity);
        //println!("Entity: {}  Clip Region: {:?}", entity, clip_region);
    }
}

pub fn apply_visibility(state: &mut State, tree: &Tree) {
    //println!("Apply Visibility");
    let mut draw_tree: Vec<Entity> = tree.into_iter().collect();
    draw_tree.sort_by_cached_key(|entity| state.data.get_z_index(*entity));

    for widget in draw_tree.into_iter() {
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

        if let Some(parent) = widget.parent(tree) {
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

    // Universal selector always matches
    if selector.asterisk {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get(entity) {
            if !pseudo_classes.is_empty() && !pseudo_classes.intersects(*pseudo_classes)
            {
                return false;
            } else {
                return true;
            }            
        } else {
            return true;
        }
    }

    // Check for ID match TODO
    // if selector.id.is_some() && selector.id != entity_selector.id {
    //     return false;
    // }


    // Check for element name match
    if let Some(selector_element) = &selector.element {
        if let Some(element) = state.style.elements.get(entity) {
            if selector_element != element {
                return false;
            }
        } else {
            return false;
        }
    }

    // Check for classes match
    if let Some(classes) = state.style.classes.get(entity) {
        if !selector.classes.is_subset(classes) {
            return false;
        }        
    } else if !selector.classes.is_empty() {
        return false;
    }

    // Check for pseudo-class match
    if let Some(pseudo_classes) = state.style.pseudo_classes.get(entity) {
        if !selector.pseudo_classes.is_empty() && !selector.pseudo_classes.intersects(*pseudo_classes)
        {
            return false;
        }
    }

    return true;
}

pub fn apply_styles(state: &mut State, tree: &Tree) {
    //println!("RESTYLE");
    // Loop through all entities
    for entity in tree.into_iter() {
        // Skip the root
        if entity == Entity::root() {
            continue;
        }

        // Create a list of style rules that match this entity
        let mut matched_rules: Vec<Rule> = Vec::new();

        // Loop through all of the style rules
        'rule_loop: for rule in state.style.rules.iter() {
            let mut relation_entity = entity;
            // Loop through selectors (Should be from right to left)
            // All the selectors need to match for the rule to apply
            'selector_loop: for rule_selector in rule.selectors.iter().rev() {
                // Get the relation of the selector
                match rule_selector.relation {
                    SelectorRelation::None => {
                        if !check_match(state, entity, rule_selector) {
                            continue 'rule_loop;
                        }
                    }

                    SelectorRelation::Parent => {
                        // Get the parent
                        // Contrust the selector for the parent
                        // Check if the parent selector matches the rule_seletor
                        if let Some(parent) = relation_entity.parent(tree) {
                            if !check_match(state, parent, rule_selector) {
                                continue 'rule_loop;
                            }

                            relation_entity = parent;
                        } else {
                            continue 'rule_loop;
                        }
                    }

                    SelectorRelation::Ancestor => {
                        // Walk up the tree
                        // Check if each entity matches the selector
                        // If any of them match, move on to the next selector
                        // If none of them do, move on to the next rule
                        for ancestor in relation_entity.parent_iter(tree) {
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
            matched_rules.push(rule.id);
        }

        //println!("Entity: {}, Matched Rules: {:?}", entity, &matched_rules);

        if matched_rules.len() == 0 {
            continue;
        }

        let mut should_relayout = false;
        let mut should_redraw = false;

        // Display
        if state.style.display.link(entity, &matched_rules) {
            //println!("1");
            should_relayout = true;
            should_redraw = true;
        }
        if state.style.visibility.link(entity, &matched_rules) {
            //println!("2");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.z_order.link(entity, &matched_rules) {
            //println!("3");
            should_relayout = true;
            should_redraw = true;
        }

        // Currently doesn't do anything - TODO
        state.style.overflow.link(entity, &matched_rules);

        // Opacity
        if state.style.opacity.link(entity, &matched_rules) {
            //println!("4");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.left.link(entity, &matched_rules) {
            //println!("6");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.right.link(entity, &matched_rules) {
            //println!("7");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.top.link(entity, &matched_rules) {
            //println!("8");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.bottom.link(entity, &matched_rules) {
            //println!("9");
            should_relayout = true;
            should_redraw = true;
        }

        // Size
        if state.style.width.link(entity, &matched_rules) {
            //println!("10");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.height.link(entity, &matched_rules) {
            //println!("11");
            should_relayout = true;
            should_redraw = true;
        }

        // Size Constraints
        if state.style.max_width.link(entity, &matched_rules) {
            //println!("12");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.min_width.link(entity, &matched_rules) {
            //println!("13");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.max_height.link(entity, &matched_rules) {
            //println!("14");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.min_height.link(entity, &matched_rules) {
            //println!("15");
            should_relayout = true;
            should_redraw = true;
        }

        // Border
        if state.style.border_width.link(entity, &matched_rules) {
            //println!("24");
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.border_color.link(entity, &matched_rules) {
            //println!("25");
            should_redraw = true;
        }

        if state.style.border_shape_top_left.link(entity, &matched_rules) {
            should_redraw = true;
        }

        if state.style.border_shape_top_right.link(entity, &matched_rules) {
            should_redraw = true;
        }

        if state.style.border_shape_bottom_left.link(entity, &matched_rules) {
            should_redraw = true;
        }

        if state.style.border_shape_bottom_right.link(entity, &matched_rules) {
            should_redraw = true;
        }

        if state
            .style
            .border_radius_top_left
            .link(entity, &matched_rules)
        {
            //println!("26");
            should_redraw = true;
        }

        if state
            .style
            .border_radius_top_right
            .link(entity, &matched_rules)
        {
            //println!("27");
            should_redraw = true;
        }

        if state
            .style
            .border_radius_bottom_left
            .link(entity, &matched_rules)
        {
            //println!("28");
            should_redraw = true;
        }

        if state
            .style
            .border_radius_bottom_right
            .link(entity, &matched_rules)
        {
            //println!("29");
            should_redraw = true;
        }

        if state.style.layout_type.link(entity, &matched_rules) {
            //println!("30");
            should_relayout = true;
            should_redraw = true;
        }

        if state
            .style
            .positioning_type
            .link(entity, &matched_rules)
        {
            //println!("30");
            should_relayout = true;
            should_redraw = true;
        }

        // Background
        if state
            .style
            .background_color
            .link(entity, &matched_rules)
        {
            //println!("41");
            should_redraw = true;
        }

        if state
            .style
            .background_image
            .link(entity, &matched_rules)
        {
            //println!("42");
            should_redraw = true;
        }

        // Font
        if state.style.font_color.link(entity, &matched_rules) {
            //println!("43");
            should_redraw = true;
        }

        if state.style.font_size.link(entity, &matched_rules) {
            //println!("44");
            should_redraw = true;
        }

        if state.style.font.link(entity, &matched_rules) {
            //println!("44");
            should_redraw = true;
        }

        // Outer Shadow
        if state
            .style
            .outer_shadow_h_offset
            .link(entity, &matched_rules)
        {
            //println!("45");
            should_redraw = true;
        }

        if state
            .style
            .outer_shadow_v_offset
            .link(entity, &matched_rules)
        {
            //println!("46");
            should_redraw = true;
        }

        if state
            .style
            .outer_shadow_blur
            .link(entity, &matched_rules)
        {
            //println!("47");
            should_redraw = true;
        }

        if state
            .style
            .outer_shadow_color
            .link(entity, &matched_rules)
        {
            //println!("48");
            should_redraw = true;
        }

        // Inner Shadow
        if state
            .style
            .inner_shadow_h_offset
            .link(entity, &matched_rules)
        {
            //println!("45");
            should_redraw = true;
        }

        if state
            .style
            .inner_shadow_v_offset
            .link(entity, &matched_rules)
        {
            //println!("46");
            should_redraw = true;
        }

        if state
            .style
            .inner_shadow_blur
            .link(entity, &matched_rules)
        {
            //println!("47");
            should_redraw = true;
        }

        if state
            .style
            .inner_shadow_color
            .link(entity, &matched_rules)
        {
            //println!("48");
            should_redraw = true;
        }

        if state.style.child_left.link(entity, &matched_rules) {
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.child_right.link(entity, &matched_rules) {
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.child_top.link(entity, &matched_rules) {
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.child_bottom.link(entity, &matched_rules) {
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.row_between.link(entity, &matched_rules) {
            should_relayout = true;
            should_redraw = true;
        }

        if state.style.col_between.link(entity, &matched_rules) {
            should_relayout = true;
            should_redraw = true;
        }


        for rule_id in matched_rules.iter() {
            // TODO - remove cloned
            if let Some(rule_index) = state.style.rules.iter().position(|rule| rule.id == *rule_id) {
                if let Some(rule) = state.style.rules.get(rule_index).cloned() {
                    for property in rule.properties.iter() {
                        match property {
                            Property::Unknown(ident, prop) => {
                                if let Some(mut event_handler) = state.event_handlers.remove(&entity) {
                                    event_handler.on_style(state, entity, (ident.clone(), prop.clone()));
    
                                    state.event_handlers.insert(entity, event_handler);
                                }
                            }
    
                            _=> {}
                        }
                    }
                }
            }
        }

        if should_relayout {
            Entity::root().relayout(state);
            //state.needs_relayout = true;
        }

        if should_redraw {
            Entity::root().redraw(state);
            //state.needs_redraw = true;
        }
    }

}

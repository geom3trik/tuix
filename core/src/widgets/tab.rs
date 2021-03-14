#![allow(dead_code)]

use std::usize;

use crate::{CheckboxEvent, Entity, HierarchyTree, MouseButton, Propagation, Radio, List, State, PropGet, AnimationState};

use crate::events::{BuildHandler, Event, EventHandler};

use crate::widgets::Element;

use crate::widgets::*;

use crate::state::style::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TabEvent {
    SwitchTab(String),
    CloseTab(String),
}

pub struct TabBar {
    list: List,
}

impl TabBar {
    pub fn new() -> Self {
        Self { 
            list: List::new(),
        }
    }
}

impl BuildHandler for TabBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.list.on_build(state, entity);

        //entity.set_flex_direction(state, FlexDirection::Row);

        state.style.insert_element(entity, "tab_bar");

        entity
    }
}

impl EventHandler for TabBar {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.list.on_event(state, entity, event);
    }
}

pub struct Tab {
    pub name: String,
    check: Checkable,
}

impl Tab {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            check: Checkable::new(false).on_checked(Event::new(TabEvent::SwitchTab(name.to_string())).propagate(Propagation::DownUp)),
            //.check_on_press()
        }
    }
}

impl BuildHandler for Tab {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity.set_element(state, "tab")
    }
}

impl EventHandler for Tab {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.check.on_event(state, entity, event);

        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(name) => {
                    if name == &self.name && event.origin != entity {
                        state.insert_event(
                            Event::new(CheckboxEvent::Checked)
                                .target(entity)
                                .origin(entity),
                        );
                    }
                }

                TabEvent::CloseTab(name) => {
                    if name == &self.name {
                        state.remove(entity);
                    }
                }
            }
        }
    }
}



pub struct TabManager {
    pub tab_bar: Entity,
    pub viewport: Entity,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tab_bar: Entity::default(),
            viewport: Entity::default(),
        }
    }
}

impl BuildHandler for TabManager {
    type Ret = (Entity, Entity);
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.tab_bar = TabBar2::new().build(state, entity, |builder| builder);

        self.viewport = Element::new().build(state, entity, |builder| builder.class("viewport"));

        entity.set_element(state, "tab_manager");

        (self.tab_bar, self.viewport)
    }
}

impl EventHandler for TabManager {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(name) => {
                    if event.origin.is_descendant_of(&state.hierarchy, self.tab_bar) || event.target == self.tab_bar {
                        state.insert_event(Event::new(TabEvent::SwitchTab(name.clone())).target(entity).propagate(Propagation::Fall).origin(event.origin));
                    }          
                    
                    event.consume();
                }

                TabEvent::CloseTab(name) => {
                    if event.origin.is_descendant_of(&state.hierarchy, self.tab_bar) || event.target == self.tab_bar {
                        state.insert_event(Event::new(TabEvent::CloseTab(name.clone())).target(entity).propagate(Propagation::Fall).origin(event.origin));
                    }

                    event.consume();
                }
            }
        }
    }
}


pub struct TabContainer {
    pub name: String,
}

impl TabContainer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl BuildHandler for TabContainer {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "tab_container")
    }
}

impl EventHandler for TabContainer {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(name) => {
                    if name == &self.name {
                        entity.set_display(state, Display::Flexbox);
                    } else {
                        entity.set_display(state, Display::None);
                    }
                }

                TabEvent::CloseTab(name) => {
                    if name == &self.name {
                        state.remove(entity);
                    }
                }
            }
        }
    }
}

// Movable Tabs

#[derive(Debug, Clone, PartialEq)]
pub enum MovableTabEvent {
    StartMove(Entity),
    StopMove(Entity),
    Moving(f32),
    Switch(bool),
}

pub struct TabBar2 {
    phantom_tab1: Entity,
    phantom_tab2: Entity,
    shrink_animation: usize,
    grow_animation: usize,
    tab_moving: bool,
    list: List,
}

impl TabBar2 {
    pub fn new() -> Self {
        Self {
            phantom_tab1: Entity::default(),
            phantom_tab2: Entity::default(),
            shrink_animation: std::usize::MAX,
            grow_animation: std::usize::MAX,
            tab_moving: false,
            list: List::new(),
        }
    }
}

impl BuildHandler for TabBar2 {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.list.on_build(state, entity);

        self.phantom_tab1 = Tab::new("phantom1").build(state, entity, |builder| builder
            .set_display(Display::None)
            .set_width(Length::Pixels(30.0))
            //.set_background_color(Color::rgb(90,90,90))
        );
        self.phantom_tab2 = Tab::new("phantom2").build(state, entity, |builder| builder
            .set_display(Display::None)
            //.set_background_color(Color::rgb(90,90,90))
        );

        // Animation to shrink one of the phantom tracks
        let shrink_animation_state = AnimationState::new()
        .with_duration(std::time::Duration::from_millis(100))
        .with_keyframe((0.0, Length::Pixels(100.0)))
        .with_keyframe((1.0, Length::Pixels(0.0)));

        self.shrink_animation = state.style.width.insert_animation(shrink_animation_state);

        // Animation to grow one of the phantom tracks
        let grow_animation_state = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(100.0)));

        self.grow_animation = state.style.width.insert_animation(grow_animation_state);



        entity.set_element(state, "tab_bar")
    }
}

impl EventHandler for TabBar2 {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {

        self.list.on_event(state, entity, event);

        if let Some(movable_tab_event) = event.message.downcast::<MovableTabEvent>() {
            match movable_tab_event {
                MovableTabEvent::StartMove(tab) => {

                    self.tab_moving = true;
                    self.phantom_tab1.set_display(state, Display::Flexbox);
                    self.phantom_tab2.set_display(state, Display::Flexbox);

                    state.hierarchy.set_prev_sibling(*tab,self.phantom_tab1);


                    //let tab_width = tab.get_width(state);
                    //let tab_height = tab.get_height(state);

                    let tab_width = state.data.get_width(*tab) + tab.get_margin_left(state).get_value(0.0) + tab.get_margin_right(state).get_value(0.0);
                    let tab_height = state.data.get_width(*tab) + tab.get_margin_top(state).get_value(0.0) + tab.get_margin_bottom(state).get_value(0.0);

                    self.phantom_tab1.set_height(state, Length::Pixels(tab_height));
                    self.phantom_tab1.set_width(state, Length::Pixels(tab_width));

                    self.phantom_tab2.set_height(state, Length::Pixels(tab_height));
                    self.phantom_tab2.set_width(state, Length::Pixels(0.0));

                    // Move the tab to the end unless already at the end
                    if let Some(last_child) = state.hierarchy.get_last_child(entity) {
                        if last_child != *tab {
                            state.hierarchy.set_next_sibling(last_child, *tab);
                        }
                    }

                    state.hierarchy.set_next_sibling(*tab,self.phantom_tab2);

                    event.consume();
                }

                MovableTabEvent::StopMove(tab) => {

                    self.tab_moving = false;
                    // Because the phantom tracks swap places while moving, 
                    // need to check which one is active before moving the track before it.
                    // This can be done by checking which one has a non-zero width (for row)
                    if state.data.get_width(self.phantom_tab1) > 0.0 {
                        state.hierarchy.set_prev_sibling(self.phantom_tab1, *tab);
                    } else if state.data.get_width(self.phantom_tab2) > 0.0 {
                        state.hierarchy.set_prev_sibling(self.phantom_tab2, *tab);
                    }

                    self.phantom_tab1.set_display(state, Display::None);
                    self.phantom_tab2.set_display(state, Display::None);
                    event.consume();
                }

                MovableTabEvent::Switch(position_state) => {
                    if self.tab_moving {
                        if *position_state {
                            if state.hierarchy.get_next_sibling(event.target) == Some(self.phantom_tab1) {
                                state.hierarchy.set_prev_sibling(event.target, self.phantom_tab2);


                                state.style.width.play_animation(self.phantom_tab1, self.shrink_animation);
                                state.style.width.play_animation(self.phantom_tab2, self.grow_animation);

                                self.phantom_tab1.set_width(state, Length::Pixels(0.0));
                                self.phantom_tab2.set_width(state, Length::Pixels(100.0));
                            } else if state.hierarchy.get_next_sibling(event.target) == Some(self.phantom_tab2) {
                                state.hierarchy.set_prev_sibling(event.target, self.phantom_tab1);


                                state.style.width.play_animation(self.phantom_tab2, self.shrink_animation);
                                state.style.width.play_animation(self.phantom_tab1, self.grow_animation);

                                self.phantom_tab2.set_width(state, Length::Pixels(0.0));
                                self.phantom_tab1.set_width(state, Length::Pixels(100.0));
                            }
                        } else {
                            if state.hierarchy.get_prev_sibling(event.target) == Some(self.phantom_tab1) {
                                state.hierarchy.set_next_sibling(event.target, self.phantom_tab2);


                                state.style.width.play_animation(self.phantom_tab1, self.shrink_animation);
                                state.style.width.play_animation(self.phantom_tab2, self.grow_animation);

                                self.phantom_tab1.set_width(state, Length::Pixels(0.0));
                                self.phantom_tab2.set_width(state, Length::Pixels(100.0));
                            } else if state.hierarchy.get_prev_sibling(event.target) == Some(self.phantom_tab2) {
                                state.hierarchy.set_next_sibling(event.target, self.phantom_tab1);


                                state.style.width.play_animation(self.phantom_tab2, self.shrink_animation);
                                state.style.width.play_animation(self.phantom_tab1, self.grow_animation);

                                self.phantom_tab2.set_width(state, Length::Pixels(0.0));
                                self.phantom_tab1.set_width(state, Length::Pixels(100.0));
                            }
                        }                        
                    }

                }

                _=> {}
            }
        }
    }
    
} 

pub struct MovableTab {
    moving: bool,
    dragging: bool,
    pos_down_x: f32,
    pos_down_y: f32,
    previous_height: Length,
    previous_width: Length,
    position_state: bool,
    tab: Tab,
}

impl MovableTab {
    pub fn new(name: &str) -> Self {
        Self {
            moving: false,
            dragging: false,
            pos_down_x: 0.0,
            pos_down_y: 0.0,
            previous_height: Length::default(),
            previous_width: Length::default(),
            position_state: false,
            tab: Tab::new(name),
        }
    }
}

impl BuildHandler for MovableTab {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.tab.on_build(state, entity)
    }
}

impl EventHandler for MovableTab {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.tab.on_event(state, entity, event);

    
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        self.moving = true;

                        self.pos_down_x = state.data.get_posx(entity);
                        self.pos_down_y = state.data.get_posy(entity);

                        state.data.set_hoverability(entity, false);

                        self.previous_height = entity.get_height(state);
                        self.previous_width = entity.get_width(state);

                        

                        entity.set_height(state, Length::Pixels(state.data.get_height(entity)));
                        entity.set_width(state, Length::Pixels(state.data.get_width(entity)));

                        let parent = state.hierarchy.get_parent(entity).unwrap();
                        let parent_posx = state.data.get_posx(parent);
                        let parent_posy = state.data.get_posy(parent);

                        entity.set_left(state, Length::Pixels(self.pos_down_x - parent_posx));
                        entity.set_top(state, Length::Pixels(self.pos_down_y - parent_posy));

                        entity.set_position(state, Position::Absolute);
                        entity.set_z_order(state, 10);
                        state.capture(entity);
                        state.insert_event(Event::new(MovableTabEvent::StartMove(entity)).target(entity));
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        self.moving = false;
                        self.dragging = false;
                        entity.set_height(state, self.previous_height);
                        entity.set_width(state, self.previous_width);
                        entity.set_position(state, Position::Relative);
                        state.data.set_hoverability(entity, true);
                        entity.set_left(state, Length::Auto);
                        entity.set_top(state, Length::Auto);
                        entity.set_z_order(state, 0);
                        state.release(entity);
                        entity.set_left(state, Length::Auto);
                        entity.set_top(state, Length::Auto);
                        state.insert_event(Event::new(MovableTabEvent::StopMove(entity)).target(entity));
                    }
                }

                WindowEvent::MouseMove(x,y) => {
                    if self.moving {

                        let parent = state.hierarchy.get_parent(entity).unwrap();
                        let parent_posx = state.data.get_posx(parent);
                        let parent_posy = state.data.get_posy(parent);

                        let dist = *x - state.mouse.left.pos_down.0;

                        //println!("dist: {}", dist);

                        if dist.abs() > 5.0 {
                            self.dragging = true;
                        }

                        if self.dragging {
                            entity.set_left(state, Length::Pixels(self.pos_down_x - parent_posx + dist));
                            //entity.set_top(state, Length::Pixels(self.pos_down_y - parent_posy + (*y - state.mouse.left.pos_down.1)));                            
                        }

                        
                        if !state.hovered.is_descendant_of(&state.hierarchy, entity) {
                            state.insert_event(Event::new(WindowEvent::MouseMove(*x,*y)).target(state.hovered));
                        }
                        

                    } else {
                        //println!("Entity Hovered: {}", entity);
                        if *x >= state.data.get_posx(entity) + state.data.get_width(entity)/2.0 {
                            if self.position_state {
                                self.position_state = false;
                                state.insert_event(Event::new(MovableTabEvent::Switch(self.position_state)).target(entity));
                            }
                            
                        } else {
                            if !self.position_state {
                                self.position_state = true;
                                state.insert_event(Event::new(MovableTabEvent::Switch(self.position_state)).target(entity));
                            }
                        }
                    }
                }

                _=> {}
            }
        }
    
    }
}
use std::time::Duration;

use tuix_core::TreeExt;

use crate::{ButtonEvent, CheckboxEvent, common::*};

use crate::{List};

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
        Self { list: List::new() }
    }
}

impl Widget for TabBar {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.list.on_build(state, entity);

        entity.set_element(state, "tab_bar")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.list.on_event(state, entity, event);
    }
}

pub struct Tab {
    pub name: String,
    checked: bool,

    on_checked: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    on_unchecked: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,

    key: Code,
}

impl Tab {
    pub fn new(name: &str) -> Self {
        
        let name = name.to_owned();
        
        Self {
            name: name.clone(),
            checked: false,

            on_checked: None,
            on_unchecked: None,

            key: Code::Space,
        }
    }

    pub fn on_checked<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_checked = Some(Box::new(callback));

        self
    }

    pub fn on_unchecked<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_unchecked = Some(Box::new(callback));

        self
    }
}

impl Widget for Tab {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "tab")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(name) => {
                    if name == &self.name && event.origin != entity {
                        println!("Switch the tab!");
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

        if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
            match checkbox_event {
                CheckboxEvent::Switch => {
                    if event.target == entity {
                        if self.checked {

                            //entity.set_checked(state, false);

                            state.insert_event(
                                Event::new(CheckboxEvent::Unchecked)
                                    .target(entity)
                                    .origin(entity),
                            );
                        } else {

                            //entity.set_checked(state, true);

                            state.insert_event(
                                Event::new(CheckboxEvent::Checked)
                                    .target(entity)
                                    .origin(entity),
                            );
                        }
                    }
                }

                CheckboxEvent::Check => {
                    self.checked = true;
                    entity.set_checked(state, true);
                }

                CheckboxEvent::Uncheck => {
                    self.checked = false;
                    entity.set_checked(state, false);
                }

                CheckboxEvent::Checked => {
                    self.checked = true;

                    entity.set_checked(state, true);

                    if let Some(callback) = self.on_checked.take() {
                        (callback)(self, state, entity);
                        self.on_checked = Some(callback);
                    }
                }

                CheckboxEvent::Unchecked => {
                    self.checked = false;

                    entity.set_checked(state, false);

                    if let Some(callback) = self.on_unchecked.take() {
                        (callback)(self, state, entity);
                        self.on_unchecked = Some(callback);
                    }
                }
            }
        }
        
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if entity == event.target && !entity.is_disabled(state) {
                        state.capture(entity);
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if entity == event.target && state.mouse.left.pressed == entity {
                        state.release(entity);
                        entity.set_active(state, false);
                        if !entity.is_disabled(state) {
                            if state.hovered == entity {
                                state.insert_event(
                                    Event::new(CheckboxEvent::Switch)
                                        .target(entity)
                                        .origin(entity),
                                );
                            }

                            state.insert_event(
                                Event::new(TabEvent::SwitchTab(self.name.clone())).propagate(Propagation::Up).target(entity).origin(entity),
                            )
                        }
                    }
                }

                WindowEvent::KeyDown(code, _) if *code == self.key => {
                    if state.focused == entity && !entity.is_disabled(state) {
                        state.insert_event(
                            Event::new(ButtonEvent::Pressed)
                                .target(entity)
                                .origin(entity),
                        );

                        state.insert_event(
                            Event::new(CheckboxEvent::Switch)
                                .target(entity)
                                .origin(entity),
                        );
                    }
                }

                WindowEvent::KeyUp(code, _) if *code == self.key => {
                    state.insert_event(
                        Event::new(ButtonEvent::Released)
                            .target(entity)
                            .origin(entity),
                    );
                }

                _ => {}
            }
        }
    }
}

pub struct TabView {
    pub tab_bar: Entity,
    pub tab_page: Entity,
}

impl TabView {
    pub fn new() -> Self {
        Self {
            tab_bar: Entity::null(),
            tab_page: Entity::null(),
        }
    }
}

impl Widget for TabView {
    type Ret = (Entity, Entity);
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.tab_bar = TabBar2::new().build(state, entity, |builder| builder);

        self.tab_page = Element::new().build(state, entity, |builder| builder.class("viewport"));

        entity.set_element(state, "tab_manager");

        (self.tab_bar, self.tab_page)
    }

    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) {
        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(name) => {
                    if event
                        .origin
                        .is_descendant_of(&state.tree, self.tab_bar)
                        || event.target == self.tab_bar
                    {
                        println!("Received request to switch tab: {}", name);
                        for child in self.tab_page.child_iter(&state.tree.clone()) {
                            state.insert_event(
                                Event::new(TabEvent::SwitchTab(name.clone()))
                                    .target(child)
                                    .propagate(Propagation::Direct)
                                    .origin(event.origin),
                            );
                        }

                        // event.consume();
                    }
                }

                TabEvent::CloseTab(name) => {
                    if event
                        .origin
                        .is_descendant_of(&state.tree, self.tab_bar)
                        || event.target == self.tab_bar
                    {
                        for child in self.tab_page.child_iter(&state.tree.clone()) {
                            state.insert_event(
                                Event::new(TabEvent::CloseTab(name.clone()))
                                    .target(child)
                                    .propagate(Propagation::Direct)
                                    .origin(event.origin),
                            );
                        }
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

impl Widget for TabContainer {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "tab_container")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(name) => {
                    //println!("Switch Tab: {}", name);

                    if name == &self.name {
                        entity.set_display(state, Display::Flex);
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
    Moving(Entity, f32),
    Switch(bool),
}

pub struct TabBar2 {
    phantom_tab1: Entity,
    phantom_tab2: Entity,
    shrink_animation: Animation,
    grow_animation: Animation,
    tab_moving: Entity,

    list: List,
}

impl TabBar2 {
    pub fn new() -> Self {
        Self {
            phantom_tab1: Entity::default(),
            phantom_tab2: Entity::default(),
            shrink_animation: Animation::default(),
            grow_animation: Animation::default(),
            tab_moving: Entity::null(),
            list: List::new(),
        }
    }
}

impl Widget for TabBar2 {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.list.on_build(state, entity);

        self.phantom_tab1 = Tab::new("phantom1").build(
            state,
            entity,
            |builder| {
                builder
                    .set_display(Display::None)
                    .set_width(Units::Pixels(30.0))
                    .set_background_color(Color::red())
            },      
        );
        self.phantom_tab2 = Tab::new("phantom2").build(
            state,
            entity,
            |builder| 
                builder
                    .set_display(Display::None) 
                    .set_background_color(Color::green())
        );

        // // Animation to shrink one of the phantom tracks
        // let shrink_animation_state = AnimationState::new()
        //     .with_duration(std::time::Duration::from_millis(100))
        //     .with_keyframe((0.0, Units::Pixels(100.0)))
        //     .with_keyframe((1.0, Units::Pixels(0.0)));

        // self.shrink_animation = state.style.width.insert_animation(shrink_animation_state);

        // // Animation to grow one of the phantom tracks
        // let grow_animation_state = AnimationState::new()
        //     .with_duration(std::time::Duration::from_millis(100))
        //     .with_keyframe((0.0, Units::Pixels(0.0)))
        //     .with_keyframe((1.0, Units::Pixels(100.0)));

        // self.grow_animation = state.style.width.insert_animation(grow_animation_state);

        entity.set_element(state, "tab_bar")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.list.on_event(state, entity, event);

        if let Some(movable_tab_event) = event.message.downcast::<MovableTabEvent>() {
            match movable_tab_event {
                MovableTabEvent::StartMove(tab) => {
                    self.tab_moving = *tab;
                    self.phantom_tab1.set_display(state, Display::Flex);
                    self.phantom_tab2.set_display(state, Display::Flex);

                    let _ = state
                        .tree
                        .set_prev_sibling(*tab, self.phantom_tab1);

                    //let tab_width = tab.get_width(state);
                    //let tab_height = tab.get_height(state);

                    let tab_width = state.data.get_width(*tab);
                        // + tab.get_left(state).value_or(0.0, 0.0)
                        // + tab.get_right(state).value_or(0.0, 0.0);
                    // let tab_height = state.data.get_height(*tab)
                    //     + tab.get_top(state).value_or(0.0, 0.0)
                    //     + tab.get_bottom(state).value_or(0.0, 0.0);

                    println!("Tab Width: {}", tab_width);

                    self.phantom_tab1
                        .set_height(state, Units::Pixels(30.0));
                    self.phantom_tab1.set_width(state, Units::Pixels(tab_width));

                    self.phantom_tab2
                        .set_height(state, Units::Pixels(30.0));
                    self.phantom_tab2.set_width(state, Units::Pixels(0.0));

                    if let Some(grow_animation) = state.style.width.get_animation_mut(self.grow_animation) {
                        grow_animation.keyframes.last_mut().unwrap().1 = Pixels(tab_width);
                        grow_animation.duration = Duration::from_millis(tab_width as u64);
                    }

                    if let Some(shrink_animation) = state.style.width.get_animation_mut(self.shrink_animation) {
                        shrink_animation.keyframes.first_mut().unwrap().1 = Pixels(tab_width);
                        shrink_animation.duration = Duration::from_millis(tab_width as u64);
                    }

                    // Move the tab to the end unless already at the end
                    if let Some(last_child) = state.tree.get_last_child(entity) {
                        if last_child != *tab {
                            state.tree.set_next_sibling(last_child, *tab).unwrap();
                        }
                    }

                    state
                        .tree
                        .set_next_sibling(*tab, self.phantom_tab2)
                        .unwrap();

                    event.consume();
                }

                MovableTabEvent::StopMove(tab) => {
                    println!("STOP MOVE");
                    self.tab_moving = Entity::null();
                    // Because the phantom tracks swap places while moving,
                    // need to check which one is active before moving the track before it.
                    // This can be done by checking which one has a non-zero width (for row)
                    if state.data.get_width(self.phantom_tab1) > 0.0 {
                        state
                            .tree
                            .set_prev_sibling(self.phantom_tab1, *tab)
                            .unwrap();
                    } else if state.data.get_width(self.phantom_tab2) > 0.0 {
                        state
                            .tree
                            .set_prev_sibling(self.phantom_tab2, *tab)
                            .unwrap();
                    }

                    self.phantom_tab1.set_display(state, Display::None).set_width(state, Pixels(0.0)).set_height(state, Pixels(0.0));
                    self.phantom_tab2.set_display(state, Display::None).set_width(state, Pixels(0.0)).set_height(state, Pixels(0.0));
                    event.consume();
                }

                MovableTabEvent::Switch(position_state) => {
                    if self.tab_moving != Entity::null() {
                        //if !state.style.width.is_animating(self.phantom_tab1) && !state.style.width.is_animating(self.phantom_tab2) {
                            if *position_state {
                                if state.tree.get_next_sibling(event.target)
                                    == Some(self.phantom_tab1)
                                {
                                    let _ = state
                                        .tree
                                        .set_prev_sibling(event.target, self.phantom_tab2);

                                    // if state.data.get_width(self.phantom_tab1) != 0.0 {
                                    //     state
                                    //     .style
                                    //     .width
                                    //     .play_animation(self.phantom_tab1, self.shrink_animation);
                                    // }
                                    
                                    // if state.data.get_width(self.phantom_tab2) == 0.0 {
                                    //     state
                                    //     .style
                                    //     .width
                                    //     .play_animation(self.phantom_tab2, self.grow_animation);
                                    // }
                                    

                                    let tab_width = state.data.get_width(self.tab_moving);
                                    self.phantom_tab1.set_width(state, Units::Pixels(0.0));
                                    self.phantom_tab2.set_width(state, Units::Pixels(tab_width));
                                } else if state.tree.get_next_sibling(event.target)
                                    == Some(self.phantom_tab2)
                                {
                                    let _ = state
                                        .tree
                                        .set_prev_sibling(event.target, self.phantom_tab1);

                                    // if state.data.get_width(self.phantom_tab2) != 0.0 {
                                    //     state
                                    //         .style
                                    //         .width
                                    //         .play_animation(self.phantom_tab2, self.shrink_animation);
                                    // }

                                    // if state.data.get_width(self.phantom_tab1) == 0.0 {
                                    //     state
                                    //         .style
                                    //         .width
                                    //         .play_animation(self.phantom_tab1, self.grow_animation);
                                    // }

                                    let tab_width = state.data.get_width(self.tab_moving);
                                    self.phantom_tab2.set_width(state, Units::Pixels(0.0));
                                    self.phantom_tab1.set_width(state, Units::Pixels(tab_width));
                                }
                            } else {
                                if state.tree.get_prev_sibling(event.target)
                                    == Some(self.phantom_tab1)
                                {
                                    let _ = state
                                        .tree
                                        .set_next_sibling(event.target, self.phantom_tab2);

                                    // if state.data.get_width(self.phantom_tab1) != 0.0 {
                                    //     state
                                    //         .style
                                    //         .width
                                    //         .play_animation(self.phantom_tab1, self.shrink_animation);
                                    // }

                                    // if state.data.get_width(self.phantom_tab2) == 0.0 {
                                    //     state
                                    //         .style
                                    //         .width
                                    //         .play_animation(self.phantom_tab2, self.grow_animation);
                                    // }

                                    let tab_width = state.data.get_width(self.tab_moving);
                                    self.phantom_tab1.set_width(state, Units::Pixels(0.0));
                                    self.phantom_tab2.set_width(state, Units::Pixels(tab_width));
                                } else if state.tree.get_prev_sibling(event.target)
                                    == Some(self.phantom_tab2)
                                {
                                    let _ = state
                                        .tree
                                        .set_next_sibling(event.target, self.phantom_tab1);

                                    // if state.data.get_width(self.phantom_tab2) != 0.0 {
                                    //     state
                                    //         .style
                                    //         .width
                                    //         .play_animation(self.phantom_tab2, self.shrink_animation);
                                    // }

                                    // if state.data.get_width(self.phantom_tab1) == 0.0 {
                                    //     state
                                    //         .style
                                    //         .width
                                    //         .play_animation(self.phantom_tab1, self.grow_animation);
                                    // }

                                    let tab_width = state.data.get_width(self.tab_moving);
                                    self.phantom_tab2.set_width(state, Units::Pixels(0.0));
                                    self.phantom_tab1.set_width(state, Units::Pixels(tab_width));
                                }
                            }    
                        //}
                        
                    }
                }

                _ => {}
            }
        }
    }
}

pub struct MovableTab {
    moving: bool,
    dragging: bool,
    pos_down_x: f32,
    pos_down_y: f32,
    previous_height: Units,
    previous_width: Units,
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
            previous_height: Units::default(),
            previous_width: Units::default(),
            position_state: false,
            tab: Tab::new(name),
        }
    }
}

impl Widget for MovableTab {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {


        entity.add_listener(state, |tab: &mut Self, state, entity, event|{
            if let Some(window_event) = event.message.downcast() {
                match window_event {
                    MovableTabEvent::Moving(moving_tab, _) => {
                        if *moving_tab != entity {

                            let px = state.data.get_posx(*moving_tab);
                            let w = state.data.get_width(*moving_tab);

                            let xx = px + w;
                            let ww = w / 2.0;

                            let ww2 = state.data.get_width(entity)/2.0;

                            //println!("{} {} {} {}", px,  state.data.get_posx(entity), xx, state.data.get_posx(entity) + state.data.get_width(entity) / 2.0);
   
                            if xx < state.data.get_posx(entity) + state.data.get_width(entity) && xx > state.data.get_posx(entity) + state.data.get_width(entity) - ww.min(ww2) {
                                //if tab.position_state {
                                    tab.position_state = false;
                                    state.insert_event(
                                        Event::new(MovableTabEvent::Switch(tab.position_state))
                                            .target(entity),
                                    );
                                //}
                            } else if px > state.data.get_posx(entity) && px < state.data.get_posx(entity) + ww.min(ww2) {
                                //if !tab.position_state {
                                    tab.position_state = true;
                                    state.insert_event(
                                        Event::new(MovableTabEvent::Switch(tab.position_state))
                                            .target(entity),
                                    );
                                //}
                            }
                            //if *dist > 0.0 {
                                //println!("Entity Hovered: {}", state.mouse.left.pressed);
                                // if xx + *dist >= state.data.get_posx(entity) + state.data.get_width(entity) / 2.0 {
                                //     if tab.position_state {
                                //         tab.position_state = false;
                                //         state.insert_event(
                                //             Event::new(MovableTabEvent::Switch(tab.position_state))
                                //                 .target(entity),
                                //         );
                                //     }
                                // } else {
                                //     if !tab.position_state {
                                //         tab.position_state = true;
                                //         state.insert_event(
                                //             Event::new(MovableTabEvent::Switch(tab.position_state))
                                //                 .target(entity),
                                //         );
                                //     }
                                // }
                            //} else {
                                //println!("Entity Hovered: {}", state.mouse.left.pressed);
                            //     if px <= state.data.get_posx(entity) + state.data.get_width(entity) / 2.0 {
                            //         if tab.position_state {
                            //             tab.position_state = true;
                            //             state.insert_event(
                            //                 Event::new(MovableTabEvent::Switch(tab.position_state))
                            //                     .target(entity),
                            //             );
                            //         }
                            //     } else {
                            //         if !tab.position_state {
                            //             tab.position_state = false;
                            //             state.insert_event(
                            //                 Event::new(MovableTabEvent::Switch(tab.position_state))
                            //                     .target(entity),
                            //             );
                            //         }
                            //     }
                            // }
                            
                        }
                    }

                    _=> {}
                }
            }
        });

        self.tab.on_build(state, entity)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.tab.on_event(state, entity, event);

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        self.moving = true;

                        self.pos_down_x = state.data.get_posx(entity);
                        self.pos_down_y = state.data.get_posy(entity);

                        entity.set_hoverable(state, false);

                        self.previous_height = entity.get_height(state);
                        self.previous_width = entity.get_width(state);

                        entity.set_height(state, Units::Pixels(state.data.get_height(entity)));
                        entity.set_width(state, Units::Pixels(state.data.get_width(entity)));

                        let parent = state.tree.get_parent(entity).unwrap();
                        let parent_posx = state.data.get_posx(parent);
                        let parent_posy = state.data.get_posy(parent);

                        entity.set_left(state, Units::Pixels(self.pos_down_x - parent_posx));
                        entity.set_top(state, Units::Pixels(self.pos_down_y - parent_posy));

                        entity.set_position_type(state, PositionType::SelfDirected);
                        entity.set_z_order(state, 10);
                        state.capture(entity);
                        state.insert_event(
                            Event::new(MovableTabEvent::StartMove(entity)).target(entity),
                        );
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        self.moving = false;
                        self.dragging = false;
                        //entity.set_height(state, self.previous_height);
                        //entity.set_width(state, self.previous_width);
                        entity.set_position_type(state, PositionType::ParentDirected);
                        entity.set_hoverable(state, true);
                        entity.set_left(state, Units::Auto);
                        entity.set_top(state, Units::Auto);
                        entity.set_z_order(state, 0);
                        state.release(entity);
                        entity.set_left(state, Units::Auto);
                        entity.set_top(state, Units::Auto);
                        state.insert_event(
                            Event::new(MovableTabEvent::StopMove(entity)).target(entity),
                        );
                    }
                }

                WindowEvent::MouseMove(x, _) => {
                    if self.moving {
                        let parent = state.tree.get_parent(entity).unwrap();
                        let parent_posx = state.data.get_posx(parent);
                        //let parent_posy = state.data.get_posy(parent);

                        let dist = *x - state.mouse.left.pos_down.0;

                        //println!("dist: {}", dist);

                        if dist.abs() > 5.0 {
                            self.dragging = true;
                        }

                        if self.dragging {
                            entity.set_left(
                                state,
                                Units::Pixels(self.pos_down_x - parent_posx + dist),
                            );
                            //entity.set_top(state, Units::Pixels(self.pos_down_y - parent_posy + (*y - state.mouse.left.pos_down.1)));
                        }

                        entity.emit(state, MovableTabEvent::Moving(entity, *x));

                        // if !state.hovered.is_descendant_of(&state.tree, entity) {
                        //     state.insert_event(
                        //         Event::new(WindowEvent::MouseMove(*x, *y)).target(state.hovered),
                        //     );
                        // }
                    } 
                    
                    // else {

                    //     //println!("Entity Hovered: {}", state.mouse.left.pressed);
                    //     if *x >= state.data.get_posx(entity) + state.data.get_width(entity) / 2.0 {
                    //         if self.position_state {
                    //             self.position_state = false;
                    //             state.insert_event(
                    //                 Event::new(MovableTabEvent::Switch(self.position_state))
                    //                     .target(entity),
                    //             );
                    //         }
                    //     } else {
                    //         if !self.position_state {
                    //             self.position_state = true;
                    //             state.insert_event(
                    //                 Event::new(MovableTabEvent::Switch(self.position_state))
                    //                     .target(entity),
                    //             );
                    //         }
                    //     }
                    // }
                }

                _ => {}
            }
        }
    }
}





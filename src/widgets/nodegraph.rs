#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;

use crate::{Align, Color, FlexDirection, Justify, JustifyContent, Length, PropSet, State};
use crate::{BuildHandler, Event, EventHandler, Propagation, Visibility, WindowEvent};

use crate::widgets::Button;

use crate::state::style::AlignItems;

#[derive(Clone, Debug, PartialEq)]
pub enum NodeEvent {
    NewConnection,
}

pub struct NodeGraph {
    connecting: bool,
    cursorx: (f32, f32),
}

impl NodeGraph {
    pub fn new() -> Self {
        NodeGraph {
            connecting: false,
            cursorx: (0.0, 0.0),
        }
    }
}

impl BuildHandler for NodeGraph {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.style.insert_element(entity, "nodegraph");

        entity
    }
}

impl EventHandler for NodeGraph {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(node_event) = event.is_type::<NodeEvent>() {
            match node_event {
                NodeEvent::NewConnection => {
                    println!("CONNECT");
                    self.connecting = true;
                    state.capture(entity);
                }
            }
        }

        if let Some(window_event) = event.is_type::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseMove(x, y) => {
                    if self.connecting {
                        println!("Connecting");
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        self.connecting = false;
                        state.release();
                    }
                }

                _ => {}
            }
        }

        false
    }
}

// NODE

pub struct Node {
    name: String,
    header: Entity,
    pressed: (f32, f32),
    moving: bool,
    input_socket: Entity,
    output_socket: Entity,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            header: Entity::null(),
            pressed: (0.0, 0.0),
            moving: false,
            input_socket: Entity::null(),
            output_socket: Entity::null(),
        }
    }
}

impl BuildHandler for Node {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_border_width(state, 1.0)
            .set_border_color(state, Color::rgb(0, 0, 0));

        self.header = Button::with_label("TEXT").build(state, entity, |builder| {
            builder.set_text_justify(Justify::Center).class("header")
        });

        let row = Button::new().build(state, entity, |builder| {
            builder
                .set_width(Length::Percentage(1.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50, 100, 50))
                .set_flex_direction(FlexDirection::Row)
                .set_justify_content(JustifyContent::Center)
                .set_align_items(AlignItems::Center)
        });

        self.input_socket = Button::new().build(state, row, |builder| {
            builder
                .set_left(Length::Pixels(-70.0))
                .set_width(Length::Pixels(11.0))
                .set_height(Length::Pixels(11.0))
                .set_background_color(Color::rgb(50, 50, 100))
                .set_border_radius(Length::Pixels(5.5))
                .set_border_width(1.0)
                .set_border_color(Color::rgb(0, 0, 0))
        });

        self.output_socket = Button::new().build(state, row, |builder| {
            builder
                .set_left(Length::Pixels(69.0))
                .set_width(Length::Pixels(11.0))
                .set_height(Length::Pixels(11.0))
                .set_background_color(Color::rgb(50, 50, 100))
                .set_border_radius(Length::Pixels(5.5))
                .set_border_width(1.0)
                .set_border_color(Color::rgb(0, 0, 0))
        });

        state.style.insert_element(entity, "node");

        entity
    }
}

impl EventHandler for Node {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.is_type::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    let button = button.clone();

                    if button == MouseButton::Left && event.target == self.header {
                        let parent = entity.get_parent(state).unwrap();

                        let posx = (state.transform.get_posx(entity)
                            - state.transform.get_posx(parent))
                        .round();
                        let posy = (state.transform.get_posy(entity)
                            - state.transform.get_posy(parent))
                        .round();

                        self.pressed = (state.mouse.cursorx - posx, state.mouse.cursory - posy);
                        self.moving = true;

                        state.capture(entity);
                    }

                    if button == MouseButton::Left && event.target == self.input_socket {
                        state.insert_event(
                            Event::new(NodeEvent::NewConnection).target(self.input_socket),
                        );
                    }

                    if button == MouseButton::Left && event.target == self.output_socket {
                        state.insert_event(
                            Event::new(NodeEvent::NewConnection).target(self.output_socket),
                        );
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        self.moving = false;
                        state.release();
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    let distx = *x - self.pressed.0;
                    let disty = *y - self.pressed.1;
                    if self.moving {
                        entity
                            .set_left(state, Length::Pixels(distx))
                            .set_top(state, Length::Pixels(disty));
                    }
                }

                _ => {}
            }
        }

        false
    }
}

/*
use crate::component::entity::Entity;
use crate::component::state::WidgetState;
use crate::component::storage::Storage;
use crate::events::{EventHandler, EventQueue, WidgetEvent, WidgetList};
use crate::mouse::*;
use crate::widget::{Widget, WidgetBuilder};

use crate::component::style::display::*;
use crate::widget_system::WidgetSystem;

use crate::component::style::flexbox::*;
use crate::component::style::layout::*;
use crate::component::style::text::*;
use crate::widget::button::Button;
use crate::widget::checkbox::Checkbox;
use crate::widget::intbox::IntBox;
use crate::widget::scrollbar::{Direction, ScrollBar, Slot};
use crate::widget::slider::Slider;

use nanovg::{
    Alignment, BasicCompositeOperation, Clip, Color, CompositeOperation, Context, Font, Frame,
    Gradient, Image, ImagePattern, LineCap, LineJoin, PathOptions, Scissor, StrokeOptions,
    TextOptions,
};

use crate::node::basic_nodes::*;
use crate::node::*;

pub struct SocketObj {
    id: Entity,
    index: u16,
    connecting: bool,
}

impl SocketObj {
    pub fn new(
        state: &mut WidgetState,
        widget_list: &mut WidgetList,
        parent: Entity,
        graph: &Graph,
        index: u16,
    ) -> Self {
        let id = state.add(parent).unwrap();
        id.set_width(state, 10.0)
            .set_height(state, 10.0)
            .set_border_width(state, 1.0)
            .set_align(state, AlignSelf::Center);

        SocketObj {
            id: id,
            index: index,
            connecting: false,
        }
    }

    pub fn get_entity(&self) -> Entity {
        self.id
    }
}

impl EventHandler for SocketObj {
    fn handle_event(
        &mut self,
        state: &mut WidgetState,
        event: &WidgetEvent,
        event_handlers: &mut Vec<Box<EventHandler>>,
        event_queue: &mut EventQueue,
    ) {
    }
    fn get_entity(&self) -> Entity {
        self.id
    }
}

pub struct NodeObj {
    id: Entity,
    title: Entity,

    node: u16,

    moving: bool,
    pressed_x: f32,
    pressed_y: f32,
}

impl NodeObj {
    pub fn new(
        state: &mut WidgetState,
        widget_list: &mut WidgetList,
        parent: Entity,
        graph: &Graph,
        node: u16,
        input_sockets: &mut Vec<(u16, Entity)>,
        output_sockets: &mut Vec<(u16, Entity)>,
    ) -> Self {
        let id = state.add(parent).unwrap();

        id.set_width(state, 150.0)
            .set_height(state, 100.0)
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Column)
            .set_border_width(state, 1.0);

        let node_name = graph.get_name(node);
        //let node_name = "test";

        let title = state.add(id).unwrap();
        title
            .set_text(state, node_name.to_string())
            .set_text_horizontal_align(state, HorizontalAlign::Center)
            .set_flex_grow(state, 0.0)
            .set_flex_basis(state, 30.0)
            .set_border_width(state, 1.0)
            .set_background(state, nanovg::Color::from_rgb(0, 100, 100));

        let node_meta_data = graph.get_meta_data(node);

        //TODO
        // if (get_group_index(node) as u32) < GROUP_START {
        //     // Do stuff here
        // }

        let num_inputs = graph.get_num_inputs(node);
        let num_outputs = graph.get_num_outputs(node);

        // println!(
        //     "Name: {}, Binary: {:#034b}, Num_Inputs: {}, Num_Outputs: {}",
        //     node_name, node, num_inputs, num_outputs
        // );

        let sockets_container = state.add(id).unwrap();
        sockets_container
            .set_display(state, Display::Flexbox)
            .set_background(state, nanovg::Color::from_rgb(0, 100, 0));

        let input_container = state.add(sockets_container).unwrap();
        input_container
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Column)
            .set_justify_content(state, JustifyContent::Center)
            .set_background(state, nanovg::Color::from_rgb(100, 50, 50));

        let output_container = state.add(sockets_container).unwrap();
        output_container
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Column)
            .set_background(state, nanovg::Color::from_rgb(50, 100, 50));

        id.set_height(state, (num_inputs.max(num_outputs) + 1) as f32 * 30.0);

        //println!("index: {}", (num_sockets + 1) as f32 * 30.0);
        //println!("num_sockets: {} - {}", node_name, num_outputs);

        for i in (1..num_inputs + 1) {
            let socket = node + (i as u16);

            let socket_meta_data = graph.get_meta_data(socket);
            let connected_socket = graph.get_connected_socket(socket);
            let data_index = graph.get_data_index(socket);

            let socket_name = graph.get_name(socket);

            // println!(
            //     "socket_name: {} socket_offset: {} data_offset: {}",
            //     socket_name, socket_index, data_index
            // );

            let container = state.add(input_container).unwrap();
            container
                .set_height(state, 30.0)
                .set_flex_basis(state, 30.0)
                .set_flex_grow(state, 0.0)
                .set_border_width(state, 1.0);
            let label = state.add(container).unwrap();
            label
                .set_posx(state, 5.0)
                .set_width(state, 50.0)
                .set_height(state, 20.0)
                .set_font_size(state, 12.0)
                .set_text(state, socket_name.to_string())
                .set_background(state, nanovg::Color::from_rgb(0, 100, 100))
                .set_align(state, AlignSelf::Center);
            let socket_obj =
                SocketObj::new(state, widget_list, container, graph, node + (i as u16));
            let socket_obj = widget_list.push(socket_obj);
            socket_obj
                .set_justify(state, JustifySelf::Start)
                .set_posx(state, -5.0);

            input_sockets.push((connected_socket, socket_obj));
            match data_index {
                NIL => {
                    println!("nill input");

                    container.set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                }
                BOOL => {
                    println!("bool input");

                    container.set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                }
                INT => {
                    println!("int input");
                    container.set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                }
                REAL => {
                    println!("real input");

                    container.set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                }
                AUDIO => {
                    println!("audio input");

                    container.set_background(state, nanovg::Color::from_rgb(40, 40, 90));
                }
                _ => {
                    if (data_index as usize) < graph.node_data.data.len() {
                        let data = &graph.node_data.data[data_index as usize];

                        println!("Data: {:?}", data);

                        match data {
                            DataType::Nil => {
                                println!("nill output");
                            }
                            DataType::Bool(val) => {
                                println!("bool output");
                            }
                            DataType::Int(val) => {
                                println!("int output");
                            }
                            DataType::Real(val) => {
                                println!("real output");
                            }
                            DataType::Audio(val) => {
                                println!("audio output");
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        for i in (1..num_outputs + 1) {
            let socket = node + (num_inputs as u16) + (i as u16);

            let socket_meta_data = graph.get_meta_data(socket);
            let data_index = graph.get_data_index(socket);
            let connected_socket = graph.get_connected_socket(socket);

            let socket_name = graph.get_name(socket);

            // println!(
            //     "socket_name: {} socket_offset: {} data_offset: {}",
            //     socket_name, socket_index, data_index
            // );
            let container = state.add(output_container).unwrap();
            container
                .set_height(state, 30.0)
                .set_flex_basis(state, 30.0)
                .set_flex_grow(state, 0.0)
                .set_border_width(state, 1.0);
            let label = state.add(container).unwrap();
            label
                .set_justify(state, JustifySelf::End)
                .set_posx(state, -5.0)
                .set_width(state, 50.0)
                .set_height(state, 20.0)
                .set_font_size(state, 12.0)
                .set_text_horizontal_align(state, HorizontalAlign::Right)
                .set_text(state, socket_name.to_string())
                .set_background(state, nanovg::Color::from_rgb(0, 100, 100))
                .set_align(state, AlignSelf::Center)
                .set_text_margin_right(state, 5.0);
            let socket_obj =
                SocketObj::new(state, widget_list, container, graph, node + (i as u16));
            let socket_obj = widget_list.push(socket_obj);
            socket_obj
                .set_justify(state, JustifySelf::End)
                .set_posx(state, 5.0);
            output_sockets.push((socket, socket_obj));
            match data_index {
                NIL => {
                    println!("nill input");

                    container.set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                }
                BOOL => {
                    println!("bool input");

                    container.set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                }
                INT => {
                    println!("int input");
                    container.set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                }
                REAL => {
                    println!("real input");

                    container.set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                }
                AUDIO => {
                    println!("audio input");

                    container.set_background(state, nanovg::Color::from_rgb(40, 40, 90));
                }
                _ => {
                    if (data_index as usize) < graph.node_data.data.len() {
                        let data = &graph.node_data.data[data_index as usize];

                        println!("Data: {:?}", data);

                        match data {
                            DataType::Nil => {
                                println!("nill output");
                            }
                            DataType::Bool(val) => {
                                println!("bool output");
                            }
                            DataType::Int(val) => {
                                println!("int output");
                            }
                            DataType::Real(val) => {
                                println!("real output");
                            }
                            DataType::Audio(val) => {
                                println!("audio output");
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        NodeObj {
            id: id,
            title: title,
            node: node,

            moving: false,
            pressed_x: 0.0,
            pressed_y: 0.0,
        }
    }

    pub fn get_entity(&self) -> Entity {
        self.id
    }

    pub fn add_socket(state: &mut WidgetState, widget_list: &mut WidgetList) {}
}

impl EventHandler for NodeObj {
    fn handle_event(
        &mut self,
        state: &mut WidgetState,
        event: &WidgetEvent,
        event_handlers: &mut Vec<Box<EventHandler>>,
        event_queue: &mut EventQueue,
    ) {
        match event {
            WidgetEvent::MouseButton(button, action, mods) => match button {
                MouseButton::Left => match action {
                    MouseButtonState::Pressed => {
                        if state.hovered == self.id || state.hovered == self.title {
                            self.pressed_x =
                                state.mouse.cursorx - state.transform.get_local_x(self.id);
                            self.pressed_y =
                                state.mouse.cursory - state.transform.get_local_y(self.id);
                            self.moving = true;
                        }
                    }

                    MouseButtonState::Released => {
                        self.moving = false;
                    }
                },

                _ => {}
            },

            WidgetEvent::MouseMotion(x, y) => {
                if self.moving {
                    let dx = x - self.pressed_x;
                    let dy = y - self.pressed_y;

                    state.transform.set_local_x(self.id, dx);
                    state.transform.set_local_y(self.id, dy);
                }
            }

            _ => {}
        }
    }

    fn get_entity(&self) -> Entity {
        self.id
    }
}

/*
pub struct NodeGroup {
    id: Entity,
    nodes: Vec<u16>,
}

impl NodeGroup {
    pub fn new(
        state: &mut WidgetState,
        widget_list: &mut WidgetList,
        parent: Entity,
        graph: &Graph,
        nodes: Vec<u16>,
    ) -> Self {
        let id = state.add(parent).unwrap();

        for node in nodes.iter() {
            let num_sockets = graph.sockets[*node as usize];
            for socket in 1..num_sockets + 1 {
                let socket = graph.sockets[(*node + socket) as usize];
                let socket_obj =
                    SocketObj::new(state, widget_list, id, graph, *node + socket).get_entity();
                match socket {
                    NIL => {
                        println!("nill input");
                        socket_obj.set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                    }
                    BOOL => {
                        println!("bool input");
                        socket_obj.set_background(state, nanovg::Color::from_rgb(0, 0, 90));
                    }
                    INT => {
                        println!("int input");
                        socket_obj.set_background(state, nanovg::Color::from_rgb(0, 90, 0));
                    }
                    REAL => {
                        println!("real input");
                        socket_obj.set_background(state, nanovg::Color::from_rgb(90, 0, 0));
                    }
                    AUDIO => {
                        println!("audio input");
                        socket_obj.set_background(state, nanovg::Color::from_rgb(0, 0, 90));
                    }
                    _ => {
                        if (socket as usize) < graph.data.len() {
                            let data = &graph.data[socket as usize];

                            println!("Data: {:?}", data);

                            socket_obj.set_posx(state, 40.0);

                            match data {
                                DataType::Nil => {
                                    println!("nill output");
                                    socket_obj
                                        .set_background(state, nanovg::Color::from_rgb(90, 90, 90));
                                }
                                DataType::Bool(val) => {
                                    println!("bool output");
                                    socket_obj
                                        .set_background(state, nanovg::Color::from_rgb(0, 90, 90));
                                }
                                DataType::Int(val) => {
                                    println!("int output");
                                    socket_obj
                                        .set_background(state, nanovg::Color::from_rgb(0, 90, 0));
                                }
                                DataType::Real(val) => {
                                    println!("real output");
                                    socket_obj
                                        .set_background(state, nanovg::Color::from_rgb(90, 0, 0));
                                }
                                DataType::Audio(val) => {
                                    println!("audio output");
                                    socket_obj
                                        .set_background(state, nanovg::Color::from_rgb(0, 0, 90));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        NodeGroup {
            id: id,
            nodes: nodes,
        }
    }
}
*/

pub struct NodeGraph {
    id: Entity,

    graph: Graph,

    pressed_x: f32,
    pressed_y: f32,

    panning: bool,

    output_sockets: Vec<(u16, Entity)>,
    input_sockets: Vec<(u16, Entity)>,

    connecting_socket: (u16, Entity),

    connections: Vec<(Entity, Entity)>,
}

impl NodeGraph {
    pub fn new(state: &mut WidgetState, widget_list: &mut WidgetList, parent: Entity) -> Self {
        //let state = &mut gui.state;
        let id = state.add(parent).unwrap();

        let mut graph = Graph::new();

        let int = graph.create_node::<IntNode>();
        let print = graph.create_node::<PrintNode>();

        graph.connect(int.get_id() + 1, print.get_id() + 1);

        //graph.exec();

        let test_node = graph.create_node::<TestNode>();

        //id.set_width(state, 200.0).set_height(state, 200.0);

        let mut input_sockets: Vec<(u16, Entity)> = Vec::new();
        let mut output_sockets: Vec<(u16, Entity)> = Vec::new();
        let mut connections: Vec<(Entity, Entity)> = Vec::new();

        for node in graph.graph_data.nodes.iter() {
            let node_id = node.get_id();

            let node = NodeObj::new(
                state,
                widget_list,
                id,
                &graph,
                node_id,
                &mut input_sockets,
                &mut output_sockets,
            );
            let node = widget_list.push(node);
        }

        for socket in &input_sockets {
            let socket_offset = socket.0;
            let socket_entity = socket.1;

            println!("Input Socket Offset: {}", socket_offset);

            if socket_offset != DISCONNECTED {
                for connected_socket in &output_sockets {
                    let connected_socket_offset = connected_socket.0;
                    let connected_socket_entity = connected_socket.1;

                    println!("Output Socket Offset: {}", connected_socket_offset);

                    if socket_offset == connected_socket_offset {
                        connections.push((connected_socket_entity, socket_entity));
                    }
                }
            }
        }

        // println!("num connections: {}", connections.len());

        NodeGraph {
            id: id,

            graph: graph,

            pressed_x: 0.0,
            pressed_y: 0.0,
            panning: false,

            connecting_socket: (0, Entity::new()),

            output_sockets: output_sockets,
            input_sockets: input_sockets,

            connections: connections,
        }
    }
    // pub fn add_node(&self, state: &mut WidgetState) {
    //     let node = NodeObj::new(state, self.id);
    // }
}

impl EventHandler for NodeGraph {
    fn handle_event(
        &mut self,
        state: &mut WidgetState,
        event: &WidgetEvent,
        event_handlers: &mut Vec<Box<EventHandler>>,
        event_queue: &mut EventQueue,
    ) {
        match event {
            WidgetEvent::MouseButton(button, action, mods) => match button {
                MouseButton::Left => match action {
                    MouseButtonState::Pressed => {
                        for socket in &self.output_sockets {
                            if state.hovered == socket.1 {
                                self.connecting_socket = *socket;
                            }
                        }
                    }

                    MouseButtonState::Released => {
                        if self.connecting_socket.1 != Entity::new() {
                            for socket in &self.input_sockets {
                                if state.hovered == socket.1 {
                                    state.graph.connect(self.connecting_socket.0, socket.0);
                                    self.connections.push((self.connecting_socket.1, socket.1));
                                }
                            }
                        }

                        self.connecting_socket = (0, Entity::new());
                    }
                },
                MouseButton::Middle => match action {
                    MouseButtonState::Pressed => {
                        if state.hovered == self.id {
                            self.pressed_x =
                                state.mouse.cursorx - state.transform.get_local_x(self.id);
                            self.pressed_y =
                                state.mouse.cursory - state.transform.get_local_y(self.id);
                            self.panning = true;
                        }
                        // let button = Button::new(state, self.id);
                        // button
                        //     .get_entity()
                        //     .set_width(state, 100.0)
                        //     .set_height(state, 100.0);
                        // event_handlers.push(Box::new(button));
                    }

                    MouseButtonState::Released => {
                        self.panning = false;
                    }
                },

                _ => {}
            },

            WidgetEvent::MouseMotion(x, y) => {}

            _ => {}
        }
    }

    fn get_entity(&self) -> Entity {
        self.id
    }

    fn draw_func(&self, state: &WidgetState, frame: &Frame) {
        for socket_pair in &self.connections {
            let output_socket = socket_pair.0;
            let input_socket = socket_pair.1;

            let output_x = state.transform.get_global_x(output_socket)
                + state.transform.get_global_width(output_socket) / 2.0;
            let output_y = state.transform.get_global_y(output_socket)
                + state.transform.get_global_height(output_socket) / 2.0;
            let input_x = state.transform.get_global_x(input_socket)
                + state.transform.get_global_width(input_socket) / 2.0;
            let input_y = state.transform.get_global_y(input_socket)
                + state.transform.get_global_height(input_socket) / 2.0;

            if input_x > output_x {
                let mid_x = output_x + (input_x - output_x) / 2.0;

                frame.path(
                    |path| {
                        path.move_to((output_x, output_y));
                        path.line_to((mid_x, output_y));
                        path.line_to((mid_x, input_y));
                        path.line_to((input_x, input_y));

                        path.stroke(
                            Color::from_rgba(255, 255, 255, 160),
                            StrokeOptions {
                                width: 5.0,
                                line_cap: LineCap::Square,
                                line_join: LineJoin::Miter,
                                ..Default::default()
                            },
                        );
                    },
                    Default::default(),
                );
            } else {
                let mid_y = output_y + (input_y - output_y) / 2.0;
                frame.path(
                    |path| {
                        path.move_to((output_x, output_y));
                        path.line_to((output_x + 10.0, output_y));
                        path.line_to((output_x + 10.0, mid_y));
                        path.line_to((input_x - 10.0, mid_y));
                        path.line_to((input_x - 10.0, input_y));
                        path.line_to((input_x, input_y));

                        path.stroke(
                            Color::from_rgba(255, 255, 255, 160),
                            StrokeOptions {
                                width: 5.0,
                                line_cap: LineCap::Square,
                                line_join: LineJoin::Miter,
                                ..Default::default()
                            },
                        );
                    },
                    Default::default(),
                );
            }
        }
    }
}
*/

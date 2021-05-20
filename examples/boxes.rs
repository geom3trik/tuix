use tuix::*;
use fnv::FnvHashMap;
#[derive(Default)]
pub struct BoxState {
    posx: f32,
    posy: f32,
}

impl Node for BoxState {}

#[derive(Default)]
pub struct SelectedState {
    selected: Vec<Entity>,
}

impl Node for SelectedState {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppEvent {
    AddBox,
}

struct CanvasWidget {
    // Widgets
    canvas: Entity,
    controls: Entity,

    bounding_box: Entity,

    // Shared data
    selected_state: Entity,
}

impl CanvasWidget {
    pub fn new() -> Self {
        Self {
            canvas: Entity::null(),
            controls: Entity::null(),

            bounding_box: Entity::null(),

            selected_state: Entity::null(),
        }
    }
}

impl Widget for CanvasWidget {
    type Ret = Entity;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        // Create some new selected state to share 
        self.selected_state = SelectedState::default().build(state, entity);
        

        let row = Row::new().build(state, entity, |builder| builder.set_width(Stretch(1.0)).set_height(Stretch(1.0)));

        self.canvas = Element::new().build(state, row, |builder| builder);

        // Add button for adding boxes
        Button::with_label("Add")
            .on_press(Event::new(AppEvent::AddBox))
            .build(state, self.canvas, |builder| 
                builder
                    .set_width(Pixels(50.0))
                    .set_height(Pixels(20.0))
                    .set_child_space(Stretch(1.0))
                    .set_background_color(Color::rgb(50, 50, 50))
            );
        
        // Create a nw controls widget and bind it to the selected state
        self.controls = ControlsWidget::new().build(state, row, |builder| 
            builder
                .set_width(Pixels(300.0))
                .set_background_color(Color::red())
        ).bind(state, self.selected_state);

        self.bounding_box = BoundingBoxWidget::new()
            .build(state, self.canvas, |builder| builder.set_z_order(10))
            .bind(state, self.selected_state);

        // Add two boxes by calling the AddBox event
        state.insert_event(Event::new(AppEvent::AddBox).direct(entity));
        state.insert_event(Event::new(AppEvent::AddBox).direct(entity));

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::AddBox => {
                    // Create some new box state
                    let box_state = BoxState::default().build(state, entity);
                    
                    // Create a new box
                    // Bind the box state and the selected state to the box
                    BoxWidget::new(box_state)
                        .build(state, self.canvas, |builder| builder)
                        .bind(state, box_state)
                        .bind(state, self.selected_state);
                    
                    self.bounding_box.bind(state, box_state);

                    event.consume();
                }

                _=> {}
            }
        }
    }
}

struct BoxWidget {
    dragging: bool,
    px: f32,
    py: f32,
    data: Entity,
}

impl BoxWidget {
    pub fn new(data: Entity) -> Self {
        Self {
            dragging: false,
            px: 0.0,
            py: 0.0,
            data,
        }
    }
}

impl Widget for BoxWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity
            .set_left(state, Pixels(0.0))
            .set_top(state, Pixels(0.0))
            .set_width(state, Pixels(150.0))
            .set_height(state, Pixels(100.0))
            .set_background_color(state, Color::rgb(100,100,200))
            .set_position_type(state, PositionType::SelfDirected)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &Box<dyn Node>, nodes: &FnvHashMap<Entity, Box<dyn Node>>) {
        // React to box state change
        if let Some(box_state) = node.downcast_ref::<BoxState>() {
            entity.set_left(state, Pixels(box_state.posx)).set_top(state, Pixels(box_state.posy));
        }

        // React to selected state change
        if let Some(selected_state) = node.downcast_ref::<SelectedState>() {
            if selected_state.selected.contains(&self.data) {
                entity.set_background_color(state, Color::rgb(150,150,250));
            } else {
                entity.set_background_color(state, Color::rgb(100,100,200));
            }
        }
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == entity {
                        state.capture(entity);
                        self.dragging = true;
                        self.px = state.data.get_posx(entity);
                        self.py = state.data.get_posy(entity);

                        let shift = state.modifiers.shift;

                        // Set the selected box state to this box
                        let data = self.data;
                        state.insert_update(Update::new(entity, move |selected_state: &mut SelectedState| {

                            if shift {
                                if !selected_state.selected.contains(&data) {
                                    selected_state.selected.push(data);
                                }
                            } else {
                                selected_state.selected.clear();
                                selected_state.selected.push(data);
                            }
                        }))
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if event.target == entity {
                        state.release(entity);
                        self.dragging = false;
                    }
                }

                WindowEvent::MouseMove(x,y) => {
                    if self.dragging {
                        // Mutate box state bound to this widget
                        let x = *x - state.mouse.left.pos_down.0 + self.px;
                        let y = *y - state.mouse.left.pos_down.1 + self.py;

                        state.insert_update(Update::new(entity, move |box_state: &mut BoxState| {
                            box_state.posx = x;
                            box_state.posy = y;
                        }));
                    }
                }

                _=> {}
            }
        }
    }
}

struct ControlsWidget {
    posx_label: Entity,
    posy_label: Entity,
}

impl ControlsWidget {
    pub fn new() -> Self {
        Self {
            posx_label: Entity::null(),
            posy_label: Entity::null(),            
        }
    }
}

impl Widget for ControlsWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.posx_label = Label::new("0").build(state, entity, |builder| builder.set_height(Pixels(30.0)));
        self.posy_label = Label::new("0").build(state, entity, |builder| builder.set_height(Pixels(30.0)));
    
        entity
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &Box<dyn Node>, nodes: &FnvHashMap<Entity, Box<dyn Node>>) {
        // React to box state change
        if let Some(box_state) = node.downcast_ref::<BoxState>() {
            self.posx_label.set_text(state, &box_state.posx.to_string());
            self.posy_label.set_text(state, &box_state.posy.to_string());
        }

        // React to selected state change
        if let Some(selected_state) = node.downcast_ref::<SelectedState>() {
            // Bind the selected box state to the controls
            // If already bound then it won't bind again
            for data in selected_state.selected.iter() {
                entity.bind(state, *data);
                
                // Send an empty update event to trigger an on_update so the labels get updated
                // There should probably be a better way to do this
                state.insert_update(Update::new(entity, |box_state: &mut BoxState| {}).target(*data));
            }
            

            
        }
    }
}

#[derive(Default)]
struct BoundingBoxWidget {
    min_left: f32,
    min_top: f32,
    max_right: f32,
    max_bottom: f32,

    selected: Vec<Entity>,
}

impl BoundingBoxWidget {
    pub fn new() -> Self {
        Self {
            min_left: 0.0,
            min_top: 0.0,
            max_right: 0.0,
            max_bottom: 0.0,

            selected: Vec::new(),
        }
    }
}

impl Widget for BoundingBoxWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_width(state, Pixels(200.0))
            .set_height(state, Pixels(200.0))
            .set_border_color(state, Color::black())
            .set_border_width(state, Pixels(5.0))
            .set_hoverability(state, false)
            .set_position_type(state, PositionType::SelfDirected)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &Box<dyn Node>, nodes: &FnvHashMap<Entity, Box<dyn Node>>) {
        // React to a box state change
        if let Some(_) = node.downcast_ref::<BoxState>() {

            // Reset the bounds
            self.min_left = 1000.0;
            self.min_top = 1000.0;
            self.max_bottom = 0.0;
            self.max_right = 0.0;

            // Loop over the selected data nodes, query them, and then use them to compute the bounds
            for data in self.selected.iter() {
                if let Some(box_state) = nodes.get(data) {
                    if let Some(box_state) = box_state.downcast_ref::<BoxState>() {
                        self.min_left = self.min_left.min(box_state.posx);
                        self.min_top = self.min_top.min(box_state.posy);
                        self.max_bottom = self.max_bottom.max((box_state.posy + 100.0));
                        self.max_right = self.max_right.max((box_state.posx + 150.0));
                    }
                }
            }

            // Set the bounding box widget position and dimensions
            entity
                .set_left(state, Pixels(self.min_left))
                .set_top(state, Pixels(self.min_top))
                .set_width(state, Pixels(self.max_right - self.min_left))
                .set_height(state, Pixels(self.max_bottom - self.min_top));
        }

        // React to a selected state change
        if let Some(selected_state) = node.downcast_ref::<SelectedState>() {
            // Copy the list of selected data nodes
            self.selected = selected_state.selected.clone();
        }
    }
}

fn main() {
    // Create the app
    let window_description = WindowDescription::new().with_title("Boxes");
    let app = Application::new(window_description, |state, window| {
        CanvasWidget::new()
            .build(state, window, |builder| builder);
        
    });

    app.run();
}

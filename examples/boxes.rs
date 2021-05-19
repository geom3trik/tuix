use tuix::*;

#[derive(Default)]
pub struct BoxState {
    posx: f32,
    posy: f32,
}

impl Node for BoxState {}

#[derive(Default)]
pub struct SelectedState {
    selected: Entity,
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

    // Shared data
    selected_state: Entity,
}

impl CanvasWidget {
    pub fn new() -> Self {
        Self {
            canvas: Entity::null(),
            controls: Entity::null(),

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
        
        // Create a nw controls widget and bind it to the selected state
        self.controls = ControlsWidget::new().build(state, row, |builder| 
            builder
                .set_width(Pixels(300.0))
                .set_background_color(Color::red())
        ).bind(state, self.selected_state);

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

                    event.consume();
                }

                _=> {}
            }
        }
    }
}

struct BoxWidget {
    dragging: bool,
    data: Entity,
}

impl BoxWidget {
    pub fn new(data: Entity) -> Self {
        Self {
            dragging: false,
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
            .set_width(state, Pixels(100.0))
            .set_height(state, Pixels(150.0))
            .set_background_color(state, Color::blue())
            .set_position_type(state, PositionType::SelfDirected)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &Box<dyn Node>) {
        // React to box state change
        if let Some(box_state) = node.downcast_ref::<BoxState>() {
            entity.set_left(state, Pixels(box_state.posx)).set_top(state, Pixels(box_state.posy));
        }

        // React to selected state change
        if let Some(selected_state) = node.downcast_ref::<SelectedState>() {
            if selected_state.selected == self.data {
                entity.set_background_color(state, Color::green());
            } else {
                entity.set_background_color(state, Color::red());
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

                    // Set the selected box state to this box
                    let data = self.data;
                    state.insert_update(Update::new(entity, move |selected_state: &mut SelectedState| selected_state.selected = data))
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
                    let x = *x;
                    let y = *y;
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

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &Box<dyn Node>) {
        // React to box state change
        if let Some(box_state) = node.downcast_ref::<BoxState>() {
            self.posx_label.set_text(state, &box_state.posx.to_string());
            self.posy_label.set_text(state, &box_state.posy.to_string());
        }

        // React to selected state change
        if let Some(selected_state) = node.downcast_ref::<SelectedState>() {
            // Bind the selected box state to the controls
            // If already bound then it won't bind again
            entity.bind(state, selected_state.selected);

            // Send an empty update event to trigger an on_update so the labels get updated
            // There should probably be a better way to do this
            state.insert_update(Update::new(entity, |box_state: &mut BoxState| {}).target(selected_state.selected));
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

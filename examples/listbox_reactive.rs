use tuix::*;

const STYLE: &str = r#"
    list {
        border-width: 1px;
        border-color: #555555;
    }

    list>check_button {
        height: 30px;
        child-space: 1s;
        background-color: white;
    }

    list>check_button:checked {
        background-color: #AAAAFF;
    }

    list>check_button:focus {
        border-width: 1px;
        border-color: black;
    }

    list label {
        child-top: 1s;
        child-bottom: 1s;
        child-left: 10px;
        color: black;
    }

"#;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomEvent {
    ChangeUser,
}

#[derive(Debug, Clone, Lens)]
pub struct User {
    name: String,
    age: i32,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Lens)]
pub struct UserData {
    users: Vec<User>,
}

impl UserData {
    pub fn new() -> Self {

        let mut users = Vec::new();
        users.push(User{name: "John Doe".to_string(), age: 42});
        users.push(User{name: "Jane Doe".to_string(), age: 39});
        users.push(User{name: "Sammy Doe".to_string(), age: 13});

        Self {
            users,
        }
    }
}

impl Model for UserData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeUser => {
                    if let Some(first) = self.users.first_mut() {
                        first.name = "Testy".to_string();
                        first.age = 27;
                        entity.emit(state, BindEvent::Update);
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct UserWidget {

}

impl Widget for UserWidget {
    type Ret = Entity;
    type Data = User;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        Label::new("name")
            .bind(User::name, |name| name.to_owned())
            .build(state, entity, |builder| 
                builder
                    .set_hoverable(false)
                );
        Label::new("age")
            .bind(User::age, |age| age.to_string())
            .build(state, entity, |builder| 
                builder
                    .set_hoverable(false)
                );
        
        entity.set_layout_type(state, LayoutType::Row).set_height(state, Pixels(50.0))
    }
}


#[derive(Default)]
struct Container {
    listview: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        self.listview = ListView::with_template(|_, _| UserWidget::default())
        .bind(UserData::users, |users| users.to_vec())
        .build(state, container, |builder| {
            builder
                .set_width(Pixels(210.0))
                .set_height(Auto)
                .set_space(Stretch(1.0))
        });

        state.set_focus(container);

        container.set_background_color(state, Color::white()).set_focusable(state, false)
    }
    
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::KeyDown(code, key) => {
                    println!("Pressed: {:?} {:?}", code, key);
                    match key {
                        Some(Key::Enter) => {
                            entity.emit(state, CustomEvent::ChangeUser);
                        }

                        _=> {}
                    }
                }

                _=> {}
            }
        }
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new().with_title("ListView").with_inner_size(300, 300),
    |state, window| {

            state.add_theme(STYLE);

            let data = UserData::new().build(state, window);
            
            Container::default().build(state, data, |builder| builder);

        },
    );

    app.run();
}
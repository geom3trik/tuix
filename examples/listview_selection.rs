use image::Pixel;
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

#[derive(Debug, Clone, Data, Lens)]
pub struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.first_name)
    }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct UserData {
    users: Vec<User>,
}

impl UserData {
    pub fn new() -> Self {

        let mut users = Vec::new();
        users.push(User{first_name: "John".to_string(), last_name: "Doe".to_string(), age: 42});
        users.push(User{first_name: "Jane".to_string(), last_name: "Doe".to_string(), age: 39});
        users.push(User{first_name: "Sammy".to_string(), last_name: "Doe".to_string(), age: 13});

        Self {
            users,
        }
    }
}

impl Widget for UserData {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    // fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
    //     if let Some(custom_event) = event.message.downcast() {
    //         match custom_event {
    //             CustomEvent::ChangeUser => {
    //                 if let Some(first) = self.users.first_mut() {
    //                     first.name = "Testy".to_string();
    //                     first.age = 27;
    //                     entity.emit(state, BindEvent::Update);
    //                 }
    //             }
    //         }
    //     }
    // }
}

#[derive(Default)]
struct UserWidget {

}

impl Widget for UserWidget {
    type Ret = Entity;
    type Data = User;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity
            .set_layout_type(state, LayoutType::Grid)
            .set_grid_cols(state, vec![Stretch(1.0), Stretch(1.0)])
            .set_grid_rows(state, vec![Pixels(30.0), Pixels(30.0), Pixels(30.0)]);
        
        Label::new("First Name: ")
            .build(state, entity, |builder| 
                builder
                    .set_hoverability(false)
                    .set_col(0)
                    .set_row(0)
                );

        Label::new("")
            .bind(User::first_name, |first_name| first_name.to_owned())
            .build(state, entity, |builder| 
                builder
                    .set_hoverability(false)
                    .set_col(1)
                    .set_row(0)
                );

        Label::new("Last Name: ")
            .build(state, entity, |builder| 
                builder
                    .set_hoverability(false)
                    .set_col(0)
                    .set_row(0)
                );
    
        Label::new("")
            .bind(User::last_name, |last_name| last_name.to_owned())
            .build(state, entity, |builder| 
                builder
                    .set_hoverability(false)
                    .set_col(1)
                    .set_row(0)
                );

        Label::new("Age: ")
            .build(state, entity, |builder| 
                builder
                    .set_hoverability(false)
                    .set_col(0)
                    .set_row(0)
                );
        
        Label::new("")
            .bind(User::age, |age| age.to_string())
            .build(state, entity, |builder| 
                builder
                    .set_hoverability(false)
                    .set_col(1)
                    .set_row(0)
                );
            
        
        entity
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

        self.listview = ListView::new(|item: &User| CheckButton::with_label(&item.to_string()) )
            .bind(UserData::users, |users| users.to_vec())
            .build(state, container, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Auto)
                    .set_space(Stretch(1.0))
            });

        UserWidget::default().build(state, container, |builder| builder.set_width(Pixels(250.0)).set_height(Auto));
        
        

        state.set_focus(container);

        container.set_background_color(state, Color::white()).set_focusability(state, false)
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

            let data = Store::new(UserData::new()).build(state, window, |builder| builder);
            
            Container::default().build(state, data, |builder| builder);

        },
    );

    app.run();
}
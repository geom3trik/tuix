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

    label {
        child-space: 1s;
        color: black;
    }

"#;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomEvent {
    SelectUser(usize),
}

#[derive(Debug, Clone, Lens)]
pub struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

impl Default for User {
    fn default() -> Self {
        Self {
            first_name: "Unknown".to_string(),
            last_name: "Unknown".to_string(),
            age: 0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.first_name)
    }
}

// TODO - Move this to a utilities folder
#[derive(Debug, Clone)]
pub struct CheckList<T> {
    pub list: Vec<T>,
    pub selected: usize,
}

impl<T> CheckList<T> {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            selected: 0,
        }
    }

    pub fn get_selected(&self) -> Option<&T> {
        self.list.get(self.selected)
    }
}

// Widget to manage the data store
#[derive(Debug, Clone, Lens)]
pub struct UserData {
    users: CheckList<User>,
}

impl UserData {
    pub fn new() -> Self {

        let mut users = Vec::new();
        users.push(User{first_name: "John".to_string(), last_name: "Doe".to_string(), age: 42});
        users.push(User{first_name: "Jane".to_string(), last_name: "Doe".to_string(), age: 39});
        users.push(User{first_name: "Sammy".to_string(), last_name: "Doe".to_string(), age: 13});

        Self {
            users: CheckList {
                list: users,
                selected: 0,
            },
        }
    }
}

impl Model for UserData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::SelectUser(index) => {
                    self.users.selected = *index;
                    // Would be better to have this sent automatically somehow
                    entity.emit(state, BindEvent::Update);
                }
            }
        }
    }
}

// Widget describing the table view of the data
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
                    .set_hoverable(false)
                    .set_col(0)
                    .set_row(0)
                );

        Label::new("")
            .bind(User::first_name, |first_name| first_name.to_owned())
            .build(state, entity, |builder| 
                builder
                    .set_hoverable(false)
                    .set_col(1)
                    .set_row(0)
                );

        Label::new("Last Name: ")
            .build(state, entity, |builder| 
                builder
                    .set_hoverable(false)
                    .set_col(0)
                    .set_row(1)
                );
    
        Label::new("")
            .bind(User::last_name, |last_name| last_name.to_owned())
            .build(state, entity, |builder| 
                builder
                    .set_hoverable(false)
                    .set_col(1)
                    .set_row(1)
                );

        Label::new("Age: ")
            .build(state, entity, |builder| 
                builder
                    .set_hoverable(false)
                    .set_col(0)
                    .set_row(2)
                );
        
        Label::new("")
            .bind(User::age, |age| age.to_string())
            .build(state, entity, |builder| 
                builder
                    .set_hoverable(false)
                    .set_col(1)
                    .set_row(2)
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
            .on_change(|list_data, state, list|{
                list.emit(state, CustomEvent::SelectUser(list_data.selected));
            })
            .bind(UserData::users, |users| users.list.to_vec())
            .build(state, container, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Auto)
                    .set_space(Stretch(1.0))
                    .set_background_color(Color::blue())
            });

        UserWidget::default()
            .bind(UserData::users, |users| users.get_selected().cloned().unwrap_or_default())
            .build(state, container, |builder| 
                builder
                    .set_width(Pixels(250.0))
                    .set_height(Pixels(90.0))
                    .set_space(Stretch(1.0))
            );

        container.set_background_color(state, Color::white()).set_focusable(state, false)
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
use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"
    textbox {
        height: 30px;
        border-width: 2px;
        border-color: black;
        child-space: 1s;
        child-left: 5px;
    }

    button.add {
        background-color: #4040AA;
        color: white;
        height: 30px;
        width: 100px;
        border-radius: 15px;
        child-space: 1s;
    }

    button.add:hover {
        background-color: #4040DD;
    }

    button.delete {
        opacity: 0.5;
    }

    button.delete:hover {
        opacity: 1.0;
    }
"#;

const ICON_TRASH: &str = "\u{e729}";

#[derive(Debug, Clone, Lens)]
pub struct TodoItem {
    description: String,
    completed: bool,
}

#[derive(Debug, Clone, Lens)]
pub struct AppData {
    todo_items: Vec<TodoItem>,
}

impl AppData {
    pub fn new() -> Self {

        let mut todo_items = Vec::new();

        todo_items.push(TodoItem {
            description: "Finish making this example".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Make example editable".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Restyle counter examples".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Update images in quick start guide".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Change default window background to white".to_string(),
            completed: true,
        });

        todo_items.push(TodoItem {
            description: "Clean up the code".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Change default font color to black".to_string(),
            completed: true,
        });

        todo_items.push(TodoItem {
            description: "Propagate binding data to children".to_string(),
            completed: true,
        });

        todo_items.push(TodoItem {
            description: "Add Drag and Drop functionality".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Add ability to add Timers".to_string(),
            completed: false,
        });

        Self {
            todo_items,
        }
    }
}

#[derive(PartialEq)]
pub enum TodoEvent {
    Add(String),
    Remove(usize),
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(todo_event) = event.message.downcast() {
            match todo_event {
                TodoEvent::Add(todo) => {
                    self.todo_items.push(TodoItem {
                        description: todo.clone(),
                        completed: false,
                    });
                    entity.emit(state, BindEvent::Update);
                }

                TodoEvent::Remove(index) => {
                    self.todo_items.remove(*index);
                    entity.emit(state, BindEvent::Update);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct TodoItemWidget {

}

impl Widget for TodoItemWidget {
    type Ret = Entity;
    type Data = TodoItem;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        // Row for laying out the checkbox and label horizontally and for adding space around and between
        let row = Row::new().build(state, entity, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Pixels(5.0))
                .set_col_between(Pixels(5.0))
        );

        // Checkbox to show whether the todo item is done or not
        Checkbox::new(false)
            // Bind the checkbox to the TodoItem's completed flag
            .bind(TodoItem::completed, |completed| *completed)
            .build(state, row, |builder| builder);

        // Label to show the todo description
        Label::new("")
            // Bind the label to the TodoItem's description field
            .bind(TodoItem::description, |desc| desc.clone())
            .build(state, row, |builder| builder);
        
        let index = state.tree.get_child_index(entity).unwrap();
        
        Button::with_label(ICON_TRASH)
        .on_press(move |_, state, button| {
            button.emit(state, TodoEvent::Remove(index));
        })
        .build(state, row, |builder| 
            builder
                .set_width(Pixels(30.0))
                .set_right(Pixels(0.0))
                .set_child_space(Stretch(1.0))
                .set_font("icons")
                .set_color(Color::rgb(220, 50, 50))
                .class("delete")
        );

        entity
            .set_border_color(state, Color::black())
            .set_border_width(state, Pixels(2.0))
            .set_height(state, Auto)
    }
}


fn main() {
    let window_description = WindowDescription::new().with_title("Todos");
    let app = Application::new(window_description, |state, window|{

        state.add_theme(STYLE);

        // Add the app data to the root of the application 
        let app_data = AppData::new().build(state, window);

        let col = Column::new().build(state, app_data, |builder| 
            builder
                .set_height(Pixels(100.0))
                .set_col_between(Pixels(10.0))
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_child_left(Pixels(10.0))
                .set_child_right(Pixels(10.0))
        );

        Label::new("TODOS APP").build(state, col, |builder| 
            builder
                .set_height(Pixels(50.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
                .set_font_size(20.0)
        );

        Textbox::new("What do you want todo?")
            .on_submit(|data, state, textbox|{
                textbox.emit(state, TodoEvent::Add(data.text.clone()));
            })
            .build(state, col, |builder| builder);
        
        Element::new().build(state, app_data, |builder| 
            builder
                .set_background_color(Color::black())
                .set_height(Pixels(2.0))
        );

        let scroll = ScrollContainer::new().build(state, app_data, |builder| builder);

        // Create a new list view and specify a widget to use to show the list item
        ListView::with_template(|_,_| TodoItemWidget::default())
            // Bind the ListView to the list data
            .bind(AppData::todo_items, |items| items.clone())
            // Build the ListView into the app
            .build(state, scroll, |builder| 
                builder
                    .set_child_space(Pixels(10.0))
                    .set_row_between(Pixels(10.0))
                    .set_height(Auto)
            );
    });

    app.run();
}
use tuix::*;
use tuix::widgets::*;

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

impl Model for AppData {}

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

        entity
            .set_border_color(state, Color::black())
            .set_border_width(state, Pixels(2.0))
            .set_height(state, Auto)
    }
}


fn main() {
    let window_description = WindowDescription::new().with_title("Basic Todos");
    let app = Application::new(window_description, |state, window|{

        // Add the app data to the root of the application 
        let app_data = AppData::new().build(state, window);

        // Create a new list view and specify a widget to use to show the list item
        ListView::with_template(|_,_| TodoItemWidget::default())
            // Bind the ListView to the list data
            .bind(AppData::todo_items, |items| items.clone())
            // Build the ListView into the app
            .build(state, app_data, |builder| 
                builder
                    .set_child_space(Pixels(10.0))
                    .set_row_between(Pixels(10.0))
                    .set_height(Auto)
            );
    });

    app.run();
}
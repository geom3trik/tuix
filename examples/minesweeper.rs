

use tuix::*;

const STYLE: &str = r#"
    cell {
        background-color: #737373;
    }

    cell:hover {
        background-color: #909090;
    }

    cell:checked {
        background-color: #CCCCCC;
    }

    cell:selected {
        background-color: #FF0000;
    }

    cell:disabled {
        background-color: #000000;
    }
"#;

fn main() {
    let window_description = WindowDescription::new()
        .with_title("Minesweeper")
        .with_inner_size(600, 600);

    Application::new(window_description, |state, window|{
        state.add_theme(STYLE);

        let app_data = AppData::new().build(state, window);

        AppWidget::new().build(state, app_data, |builder| builder);

    }).run();
}

// Data
#[derive(Default, Clone, Copy, Lens)]
pub struct CellState {
    pub visible: bool,
    pub mine: bool,
    pub flagged: bool,
}

#[derive(Lens)]
pub struct AppData {
    cells: Vec<CellState>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            cells: vec![CellState::default(); 100],
        }
    }
}

// Events
enum AppEvent {
    Reveal(usize),
    Flag(usize),
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::Reveal(index) => {
                    self.cells[*index].visible = true;
                    entity.emit(state, BindEvent::Update);
                }

                AppEvent::Flag(index) => {
                    if self.cells[*index].flagged == true {
                        self.cells[*index].flagged = false;
                    } else {
                        self.cells[*index].flagged = true;
                    }
                    entity.emit(state, BindEvent::Update);
                }
            }
        }
    }
}

pub struct AppWidget {

}

impl AppWidget {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Widget for AppWidget {

    type Ret = Entity;
    type Data = AppData;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_layout_type(state, LayoutType::Grid)
            .set_grid_rows(state, vec![Stretch(1.0); 10])
            .set_grid_cols(state, vec![Stretch(1.0); 10])
            .set_row_between(state, Pixels(1.0))
            .set_col_between(state, Pixels(1.0))
            .set_background_color(state, Color::rgb(50, 50, 50));

        for i in 0..10 {
            for j in 0..10 {
                let index = 10 * i + j;
                Cell::new(index)
                .bind(AppData::cells, move |data| data[index])
                .build(state, entity, |builder| 
                    builder
                        .set_row_index(i)
                        .set_col_index(j)
                );
            }
        }

        entity
    }
}

pub struct Cell {
    index: usize,
}  

impl Cell {
    pub fn new(index: usize) -> Self {
        Self {
            index,
        }
    }
}

impl Widget for Cell {
    type Ret = Entity;
    type Data = CellState;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "cell")
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        if data.visible && data.mine {
            entity.set_disabled(state, true);
        } else if data.visible {
            entity.set_checked(state, true);
        } else if data.flagged {
            entity.set_selected(state, true);
        } else {
            entity.set_disabled(state, false);
            entity.set_checked(state, false);
            entity.set_selected(state, false);
        }
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if state.hovered == entity {
                        entity.emit(state, AppEvent::Reveal(self.index));
                    }
                }

                WindowEvent::MouseDown(button) if *button == MouseButton::Right => {
                    if state.hovered == entity {
                        entity.emit(state, AppEvent::Flag(self.index));
                    }
                }

                _=> {}
            }
        }
    }
}
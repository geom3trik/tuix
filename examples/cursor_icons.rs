use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"
    hover_button {
        width: 150px;
        height: 1s;
        background-color: #CCCCCC;
        child-space: 1s;
    }
"#;

fn main() {
    let window_description = WindowDescription::new()
        .with_title("Cursor Icons")
        .with_inner_size(560, 600);
    let app = Application::new(window_description, |state, window|{

        state.add_theme(STYLE);

        let row = Row::new().build(state, window, |builder| 
            builder
                .set_space(Pixels(5.0))
                .set_col_between(Pixels(50.0))
        );

        let column = Column::new().build(state, row, |builder| builder.set_row_between(Pixels(5.0)));
        HoverButton::new("Default", CursorIcon::Default).build(state, column, |builder| builder);
        HoverButton::new("Crosshair", CursorIcon::Crosshair).build(state, column, |builder| builder);
        HoverButton::new("Hand", CursorIcon::Hand).build(state, column, |builder| builder);
        HoverButton::new("Arrow", CursorIcon::Arrow).build(state, column, |builder| builder);
        HoverButton::new("Move", CursorIcon::Move).build(state, column, |builder| builder);
        HoverButton::new("Text", CursorIcon::Text).build(state, column, |builder| builder);
        HoverButton::new("Wait", CursorIcon::Wait).build(state, column, |builder| builder);
        HoverButton::new("Help", CursorIcon::Help).build(state, column, |builder| builder);
        HoverButton::new("Progress", CursorIcon::Progress).build(state, column, |builder| builder);
        HoverButton::new("NotAllowed", CursorIcon::NotAllowed).build(state, column, |builder| builder);
        HoverButton::new("ContextMenu", CursorIcon::ContextMenu).build(state, column, |builder| builder);
        HoverButton::new("Cell", CursorIcon::Cell).build(state, column, |builder| builder);
        
        let column = Column::new().build(state, row, |builder| builder.set_row_between(Pixels(5.0)));
        HoverButton::new("VerticalText", CursorIcon::VerticalText).build(state, column, |builder| builder);
        HoverButton::new("Alias", CursorIcon::Alias).build(state, column, |builder| builder);
        HoverButton::new("Copy", CursorIcon::Copy).build(state, column, |builder| builder);
        HoverButton::new("NoDrop", CursorIcon::NoDrop).build(state, column, |builder| builder);
        HoverButton::new("Grab", CursorIcon::Grab).build(state, column, |builder| builder);
        HoverButton::new("Grabbing", CursorIcon::Grabbing).build(state, column, |builder| builder);
        HoverButton::new("AllScroll", CursorIcon::AllScroll).build(state, column, |builder| builder);
        HoverButton::new("ZoomIn", CursorIcon::ZoomIn).build(state, column, |builder| builder);
        HoverButton::new("ZoomOut", CursorIcon::ZoomOut).build(state, column, |builder| builder);
        HoverButton::new("EResize", CursorIcon::EResize).build(state, column, |builder| builder);
        HoverButton::new("NResize", CursorIcon::NResize).build(state, column, |builder| builder);
        HoverButton::new("NeResize", CursorIcon::NeResize).build(state, column, |builder| builder);
        
        let column = Column::new().build(state, row, |builder| builder.set_row_between(Pixels(5.0)));
        HoverButton::new("NwResize", CursorIcon::NwResize).build(state, column, |builder| builder);
        HoverButton::new("SResize", CursorIcon::SResize).build(state, column, |builder| builder);
        HoverButton::new("SeResize", CursorIcon::SeResize).build(state, column, |builder| builder);
        HoverButton::new("SwResize", CursorIcon::SwResize).build(state, column, |builder| builder);
        HoverButton::new("WResize", CursorIcon::WResize).build(state, column, |builder| builder);
        HoverButton::new("EwResize", CursorIcon::EwResize).build(state, column, |builder| builder);
        HoverButton::new("NsResize", CursorIcon::NsResize).build(state, column, |builder| builder);
        HoverButton::new("NeswResize", CursorIcon::NeswResize).build(state, column, |builder| builder);
        HoverButton::new("NwseResize", CursorIcon::NwseResize).build(state, column, |builder| builder);
        HoverButton::new("ColResize", CursorIcon::ColResize).build(state, column, |builder| builder);
        HoverButton::new("RowResize", CursorIcon::RowResize).build(state, column, |builder| builder);
        HoverButton::new("None", CursorIcon::None).build(state, column, |builder| builder);
    
    });

    app.run();
}


pub struct HoverButton {
    text: String,
    cursor_icon: CursorIcon,
}

impl HoverButton {
    pub fn new(text: &str, cursor_icon: CursorIcon) -> Self {
        Self {
            text: text.to_owned(),
            cursor_icon,
        }
    }
}

impl Widget for HoverButton {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity.set_element(state, "hover_button").set_text(state, &self.text)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseOver => {
                    entity.emit(state, WindowEvent::SetCursor(self.cursor_icon));
                }

                _=> {}
            }
        }
    }
}

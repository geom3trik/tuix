use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"
    button {
        width: 150px;
        height: 1s;
        background-color: #CCCCCC;
        child-space: 1s;
    }

    .default {cursor: default}
    .crosshair {cursor: crosshair}
    .hand {cursor: hand}
    .arrow {cursor: arrow}
    .move {cursor: move}
    .text {cursor: text}
    .wait {cursor: wait}
    .help {cursor: help}
    .progress {cursor: progress}
    .not-allowed {cursor: not-allowed}
    .context-menu {cursor: context-menu}
    .cell {cursor: cell}
    .vertical-text {cursor: vertical-text}
    .alias {cursor: alias}
    .copy {cursor: copy}
    .no-drop {cursor: no-drop}
    .grab {cursor: grab}
    .grabbing {cursor: grabbing}
    .all-scroll {cursor: all-scroll}
    .zoom-in {cursor: zoom-in}
    .zoom-out {cursor: zoom-out}
    .e-resize {cursor: e-resize}
    .n-resize {cursor: n-resize}
    .ne-resize {cursor: ne-resize}
    .nw-resize {cursor: nw-resize}
    .s-resize {cursor: s-resize}
    .se-resize {cursor: se-resize}
    .sw-resize {cursor: sw-resize}
    .w-resize {cursor: w-resize}
    .ew-resize {cursor: ew-resize}
    .ns-resize {cursor: ns-resize}
    .nesw-resize {cursor: nesw-resize}
    .nwse-resize {cursor: nwse-resize}
    .col-resize {cursor: col-resize}
    .row-resize {cursor: row-resize}
    .none {cursor: none}
"#;

fn main() {
    let window_description = WindowDescription::new()
        .with_title("Cursor Icons CSS")
        .with_inner_size(560, 600);
    let app = Application::new(window_description, |state, window|{

        state.add_theme(STYLE);

        let row = Row::new().build(state, window, |builder| 
            builder
                .set_space(Pixels(5.0))
                .set_col_between(Pixels(50.0))
        );

        let column = Column::new().build(state, row, |builder| builder.set_row_between(Pixels(5.0)));
        Button::with_label("Default").build(state, column, |builder| builder.class("default"));
        Button::with_label("Crosshair").build(state, column, |builder| builder.class("crosshair"));
        Button::with_label("Hand").build(state, column, |builder| builder.class("hand"));
        Button::with_label("Arrow").build(state, column, |builder| builder.class("arrow"));
        Button::with_label("Move").build(state, column, |builder| builder.class("move"));
        Button::with_label("Text").build(state, column, |builder| builder.class("text"));
        Button::with_label("Wait").build(state, column, |builder| builder.class("wait"));
        Button::with_label("Help").build(state, column, |builder| builder.class("help"));
        Button::with_label("Progress").build(state, column, |builder| builder.class("progress"));
        Button::with_label("NotAllowed").build(state, column, |builder| builder.class("not-allowed"));
        Button::with_label("ContextMenu").build(state, column, |builder| builder.class("context-menu"));
        Button::with_label("Cell").build(state, column, |builder| builder.class("cell"));
        
        let column = Column::new().build(state, row, |builder| builder.set_row_between(Pixels(5.0)));
        Button::with_label("VerticalText").build(state, column, |builder| builder.class("vertical-text"));
        Button::with_label("Alias").build(state, column, |builder| builder.class("alias"));
        Button::with_label("Copy").build(state, column, |builder| builder.class("copy"));
        Button::with_label("NoDrop").build(state, column, |builder| builder.class("no-drop"));
        Button::with_label("Grab").build(state, column, |builder| builder.class("grab"));
        Button::with_label("Grabbing").build(state, column, |builder| builder.class("grabbing"));
        Button::with_label("AllScroll").build(state, column, |builder| builder.class("all-scroll"));
        Button::with_label("ZoomIn").build(state, column, |builder| builder.class("zoom-in"));
        Button::with_label("ZoomOut").build(state, column, |builder| builder.class("zoom-out"));
        Button::with_label("EResize").build(state, column, |builder| builder.class("e-resize"));
        Button::with_label("NResize").build(state, column, |builder| builder.class("n-resize"));
        Button::with_label("NeResize").build(state, column, |builder| builder.class("ne-resize"));
        
        let column = Column::new().build(state, row, |builder| builder.set_row_between(Pixels(5.0)));
        Button::with_label("NwResize").build(state, column, |builder| builder.class("nw-resize"));
        Button::with_label("SResize").build(state, column, |builder| builder.class("s-resize"));
        Button::with_label("SeResize").build(state, column, |builder| builder.class("se-resize"));
        Button::with_label("SwResize").build(state, column, |builder| builder.class("sw-resize"));
        Button::with_label("WResize").build(state, column, |builder| builder.class("w-resize"));
        Button::with_label("EwResize").build(state, column, |builder| builder.class("ew-resize"));
        Button::with_label("NsResize").build(state, column, |builder| builder.class("ns-resize"));
        Button::with_label("NeswResize").build(state, column, |builder| builder.class("nesw-resize"));
        Button::with_label("NwseResize").build(state, column, |builder| builder.class("nwse-resize"));
        Button::with_label("ColResize").build(state, column, |builder| builder.class("col-resize"));
        Button::with_label("RowResize").build(state, column, |builder| builder.class("row-resize"));
        Button::with_label("None").build(state, column, |builder| builder.class("none"));
    
    });

    app.run();
}

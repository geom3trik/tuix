extern crate tuix;

use tuix::*;

static THEME: &'static str = include_str!("themes/eq8_theme.css");

const ICON_FLOPPY: &str = "\u{1f4be}";
const ICON_PLUS: &str = "\u{2b}";

const frequencies: [f32; 27] = [
    1.477121, 1.60206, 1.69897, 1.778151, 1.845098, 1.90309, 1.954243, 2.0, 2.30103, 2.477121,
    2.60206, 2.69897, 2.778151, 2.845098, 2.90309, 2.954243, 3.0, 3.30103, 3.477121, 3.60206,
    3.69897, 3.778151, 3.845098, 3.90309, 3.954243, 4.0, 4.30103,
];

// TODO's

//  - Figure out how events should be propagated without lots of forwarding
//    I like the name lookup idea but the question is how to give entities names when they inaccessible
//  - Add the ability for children to be notified of parent size change
//  - Add units to a textbox. Possibly should be a new widget?
//  - Figure out how to plot filter responses
//  - Propagate "disabled" flag?
//  - The graph doesn't update correctly if the window is resized while on another tab
//    I think it's because invisible widgets don't receive layout events.

fn main() {
    // Create the app
    let mut app = Application::new(|win_desc, state, window| {
        state.insert_theme(THEME);

        let eq8 = EQ8::new().build(state, window, |builder| builder.set_flex_grow(1.0));

        win_desc.with_title("Eq8").with_inner_size(800, 600)
    });

    app.run();
}

#[derive(Debug, Clone, PartialEq)]
pub enum EqChannelEvent {
    Enabled,
    Disabled,
    FreqChanged(f32),
    GainChanged(f32),
}

pub struct EQ8 {
    channel1: Entity,
    channel2: Entity,
    channel3: Entity,
    channel4: Entity,
    channel5: Entity,
    channel6: Entity,
    channel7: Entity,
    channel8: Entity,

    control_point1: Entity,
    control_point2: Entity,
    control_point3: Entity,
    control_point4: Entity,
    control_point5: Entity,
    control_point6: Entity,
    control_point7: Entity,
    control_point8: Entity,
}

impl EQ8 {
    pub fn new() -> Self {
        EQ8 {
            channel1: Entity::null(),
            channel2: Entity::null(),
            channel3: Entity::null(),
            channel4: Entity::null(),
            channel5: Entity::null(),
            channel6: Entity::null(),
            channel7: Entity::null(),
            channel8: Entity::null(),

            control_point1: Entity::null(),
            control_point2: Entity::null(),
            control_point3: Entity::null(),
            control_point4: Entity::null(),
            control_point5: Entity::null(),
            control_point6: Entity::null(),
            control_point7: Entity::null(),
            control_point8: Entity::null(),
        }
    }
}

impl BuildHandler for EQ8 {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        //Header
        let header = HBox::new().build(state, entity, |builder| builder.class("header"));
        //Enabled Checkbox
        let enabled = Checkbox::new(true).build(state, header, |builder| builder.class("enable"));
        let label = Label::new("EQ EIGHT").build(state, header, |builder| builder);
        let preset_dropdown = Dropdown::new("Preset 1")
            .build(state, header, |builder| builder)
            .2;

        // Example Presets
        let first = Dimension::new("Preset 1")
            .build(state, preset_dropdown, |builder| builder.class("item"));
        let second = Dimension::new("Preset 2")
            .build(state, preset_dropdown, |builder| builder.class("item"));
        let third = Dimension::new("Preset 3")
            .build(state, preset_dropdown, |builder| builder.class("item"));

        let enabled =
            Checkbox::new(true)
                .with_icon_checked(ICON_FLOPPY)
                .build(state, header, |builder| builder.class("save_preset"));
        let enabled =
            Checkbox::new(true)
                .with_icon_checked(ICON_PLUS)
                .build(state, header, |builder| builder.class("save_preset"));

        // Body
        let body = VBox::new().build(state, entity, |builder| builder.class("body"));
        let (tab_bar, tab_container) = TabContainer::new().build(state, body, |builder| builder);

        Button::with_label("Graph")
            .on_press(Event::new(TabEvent::SwitchTab(0)))
            .build(state, tab_bar, |builder| builder.set_checked(true));
        //let graph_view = Element::new().build(state, tab_container, |builder| builder.class("item1"));
        let graph = FreqGraph::new().build(state, tab_container, |builder| builder);

        self.control_point1 = ControlPoint::new("1").build(state, graph, |builder| builder);
        self.control_point2 = ControlPoint::new("2").build(state, graph, |builder| builder);
        self.control_point3 = ControlPoint::new("3").build(state, graph, |builder| builder);
        self.control_point4 = ControlPoint::new("4").build(state, graph, |builder| builder);
        self.control_point5 = ControlPoint::new("5").build(state, graph, |builder| builder);
        self.control_point6 = ControlPoint::new("6").build(state, graph, |builder| builder);
        self.control_point7 = ControlPoint::new("7").build(state, graph, |builder| builder);
        self.control_point8 = ControlPoint::new("8").build(state, graph, |builder| builder);

        Button::with_label("Control")
            .on_press(Event::new(TabEvent::SwitchTab(1)))
            .build(state, tab_bar, |builder| builder);
        let control_view = Button::new().build(state, tab_container, |builder| {
            builder
                .class("item2")
                .set_display(Display::None)
                .set_flex_direction(FlexDirection::Row)
        });

        //let row = HBox::new().build(state, control_view, |builder| builder.set_flex_grow(1.0));
        self.channel1 = EQChannel::new(1).build(state, control_view, |builder| builder);
        self.channel2 = EQChannel::new(2).build(state, control_view, |builder| builder);
        self.channel3 = EQChannel::new(3).build(state, control_view, |builder| builder);
        self.channel4 = EQChannel::new(4).build(state, control_view, |builder| builder);
        self.channel5 = EQChannel::new(5).build(state, control_view, |builder| builder);
        self.channel6 = EQChannel::new(6).build(state, control_view, |builder| builder);
        self.channel7 = EQChannel::new(7).build(state, control_view, |builder| builder);
        self.channel8 = EQChannel::new(8).build(state, control_view, |builder| builder);
        let channel_output = ChannelOutput::new().build(state, control_view, |builder| builder);

        // Button::with_label("Second Button").build(state, second, |builder| builder.class("test"));

        state.style.insert_element(entity, "eqeight");

        entity
    }
}

impl EventHandler for EQ8 {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                // WindowEvent::GeometryChanged => {
                //     // Prevents infinite recursion (except when there are multiple control points)
                //     if event.origin != entity {
                //         let parent = state.hierarchy.get_parent(entity).unwrap();
                //         let parent_width = state.transform.get_width(parent);
                //         let parent_height = state.transform.get_height(parent);
                //         let width = state.transform.get_width(entity);
                //         let height = state.transform.get_height(entity);

                //         let min = 1.477121;
                //         let max = 4.3013;
                //         let range = max - min;

                //         let new_left = 40.0 + (self.frequency.log10() - min) * ((parent_width - 80.0)/range);
                //         let new_top = 40.0 + (-self.gain + 12.0) * ((parent_height - 80.0)/24.0);

                //         entity.set_left(state, Length::Pixels(new_left - width/2.0));
                //         entity.set_top(state, Length::Pixels(new_top - height/2.0));
                //     }

                // }
                _ => {}
            }
        }

        if let Some(filter_event) = event.message.downcast::<FilterEvent>() {
            match filter_event {
                FilterEvent::FreqChange(channel, freq) => {
                    state.insert_event(
                        Event::new(FilterEvent::FreqChange(*channel, *freq))
                            .target(entity)
                            .propagate(Propagation::Fall),
                    );
                    return true;
                }

                FilterEvent::GainChange(channel, gain) => {
                    state.insert_event(
                        Event::new(FilterEvent::GainChange(*channel, *gain))
                            .target(entity)
                            .propagate(Propagation::Fall),
                    );
                    return true;
                }

                _ => {}
            }
        }

        false
    }
}

pub struct EQChannel {
    active_switch: Entity,
    frequency_knob: Entity,
    gain_knob: Entity,
    q_knob: Entity,
    response_dropdown: Entity,

    channel_number: u32,
}

impl EQChannel {
    pub fn new(channel_number: u32) -> Self {
        EQChannel {
            active_switch: Entity::null(),
            frequency_knob: Entity::null(),
            gain_knob: Entity::null(),
            q_knob: Entity::null(),
            response_dropdown: Entity::null(),

            channel_number,
        }
    }
}

impl BuildHandler for EQChannel {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        println!("E: {}", entity);

        self.active_switch = Checkbox::new(true)
            .on_checked(Event::new(EqChannelEvent::Enabled).target(entity))
            .on_unchecked(Event::new(EqChannelEvent::Disabled).target(entity))
            .with_icon_checked(&self.channel_number.to_string())
            .with_icon_unchecked(&self.channel_number.to_string())
            .build(state, entity, |builder| builder);
        self.response_dropdown = Dropdown::new("")
            .build(state, entity, |builder| {
                builder.set_margin_bottom(Length::Pixels(20.0))
            })
            .2;

        self.frequency_knob = ValueKnob::new("Freq", 30.0, 30.0, 20000.0)
            .on_change(move |val| Event::new(EqChannelEvent::FreqChanged(val)).target(entity))
            .with_log_scale()
            .build(state, entity, |builder| builder.id("channel1_freq_knob"));
        self.gain_knob =
            ValueKnob::new("Gain", 0.0, -12.0, 12.0).build(state, entity, |builder| builder);
        self.q_knob = ValueKnob::new("Q", 0.7, 0.0, 5.0).build(state, entity, |builder| builder);

        state.style.insert_element(entity, "eqchannel");

        entity
    }
}

impl EventHandler for EQChannel {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(eqchannel_event) = event.message.downcast::<EqChannelEvent>() {
            match eqchannel_event {
                EqChannelEvent::Enabled => {
                    if event.target == entity {
                        entity.set_disabled(state, false);
                        self.response_dropdown.set_disabled(state, false);
                        self.frequency_knob.set_disabled(state, false);
                        self.gain_knob.set_disabled(state, false);
                        self.q_knob.set_disabled(state, false);
                    }
                }

                EqChannelEvent::Disabled => {
                    if event.target == entity {
                        entity.set_disabled(state, true);
                        self.response_dropdown.set_disabled(state, true);
                        self.frequency_knob.set_disabled(state, true);
                        self.gain_knob.set_disabled(state, true);
                        self.q_knob.set_disabled(state, true);
                    }
                }

                EqChannelEvent::FreqChanged(val) => {
                    if event.target == entity {
                        println!("Freq Value: {}", val);
                    }
                }

                _ => {}
            }
        }

        false
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        // Skip window
        if entity == Entity::new(0, 0) {
            return;
        }

        // Skip invisible widgets
        if state.transform.get_visibility(entity) == Visibility::Invisible {
            return;
        }

        if state.transform.get_opacity(entity) == 0.0 {
            return;
        }

        let posx = state.transform.get_posx(entity);
        let posy = state.transform.get_posy(entity);
        let width = state.transform.get_width(entity);
        let height = state.transform.get_height(entity);

        let padding_left = match state
            .style
            .padding_left
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_right = match state
            .style
            .padding_right
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_top = match state.style.padding_top.get(entity).unwrap_or(&Length::Auto) {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_bottom = match state
            .style
            .padding_bottom
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let background_color = state
            .style
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let font_color = state
            .style
            .font_color
            .get(entity)
            .cloned()
            .unwrap_or(tuix::Color::rgb(255, 255, 255));

        let border_color = state
            .style
            .border_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let shadow_color = state
            .style
            .shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let parent = state
            .hierarchy
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.transform.get_width(parent);
        let parent_height = state.transform.get_height(parent);

        let border_radius_top_left = match state
            .style
            .border_radius_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state
            .style
            .border_radius_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state
            .style
            .border_radius_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state
            .style
            .border_radius_bottom_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let opacity = state.transform.get_opacity(entity);

        let mut background_color: Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let mut shadow_color: Color = shadow_color.into();
        shadow_color.set_alphaf(shadow_color.a * opacity);

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        // Skip widgets with no width or no height
        if width + 2.0 * border_width + padding_left + padding_right == 0.0
            || height + 2.0 * border_width + padding_top + padding_bottom == 0.0
        {
            return;
        }

        // Apply transformations
        let rotate = state.style.rotate.get(entity).unwrap_or(&0.0);
        let scaley = state.style.scaley.get(entity).cloned().unwrap_or_default();

        canvas.save();
        canvas.translate(posx + width / 2.0, posy + height / 2.0);
        canvas.rotate(rotate.to_radians());
        canvas.translate(-(posx + width / 2.0), -(posy + height / 2.0));

        //let pt = canvas.transform().inversed().transform_point(posx + width / 2.0, posy + height / 2.0);
        //canvas.translate(posx + width / 2.0, posy + width / 2.0);
        // canvas.translate(pt.0, pt.1);
        // canvas.scale(1.0, scaley.0);
        // canvas.translate(-pt.0, -pt.1);

        // Apply Scissor
        let clip_entity = state.transform.get_clip_widget(entity);

        let clip_posx = state.transform.get_posx(clip_entity);
        let clip_posy = state.transform.get_posy(clip_entity);
        let clip_width = state.transform.get_width(clip_entity);
        let clip_height = state.transform.get_height(clip_entity);

        canvas.scissor(clip_posx, clip_posy, clip_width, clip_height);
        //canvas.scissor(0.0, 0.0, 100.0, 100.0);

        let shadow_h_offset = match state
            .style
            .shadow_h_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let shadow_v_offset = match state
            .style
            .shadow_v_offset
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let shadow_blur = match state
            .style
            .shadow_blur
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_height * val,
            _ => 0.0,
        };

        let shadow_color = state
            .style
            .shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let mut shadow_color: Color = shadow_color.into();
        shadow_color.set_alphaf(shadow_color.a * opacity);

        // Draw shadow (TODO)
        let mut path = Path::new();
        path.rect(
            posx + (border_width / 2.0) - shadow_blur + shadow_h_offset,
            posy + (border_width / 2.0) - shadow_blur + shadow_v_offset,
            width - border_width + 2.0 * shadow_blur,
            height - border_width + 2.0 * shadow_blur,
        );
        // path.rounded_rect_varying(
        //     posx + (border_width / 2.0),
        //     posy + (border_width / 2.0),
        //     width - border_width,
        //     height - border_width,
        //     border_radius_top_left,
        //     border_radius_top_right,
        //     border_radius_bottom_right,
        //     border_radius_bottom_left,
        // );
        // path.solidity(Solidity::Hole);
        //let mut paint = Paint::color(shadow_color);

        let mut paint = Paint::box_gradient(
            posx + (border_width / 2.0) + shadow_h_offset,
            posy + (border_width / 2.0) + shadow_v_offset,
            width - border_width,
            height - border_width,
            border_radius_top_left,
            shadow_blur,
            shadow_color,
            Color::rgba(0, 0, 0, 0),
        );

        canvas.fill_path(&mut path, paint);

        // Draw rounded rect
        let mut path = Path::new();
        path.rounded_rect_varying(
            posx + (border_width / 2.0),
            posy + (border_width / 2.0),
            width - border_width,
            height - border_width,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        let mut paint = Paint::color(background_color);
        canvas.fill_path(&mut path, paint);

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        canvas.stroke_path(&mut path, paint);

        // Draw text
        if let Some(text) = state.style.text.get_mut(entity) {
            let font_id = match text.font.as_ref() {
                "Sans" => state.fonts.regular.unwrap(),
                "Icons" => state.fonts.icons.unwrap(),
                _ => state.fonts.regular.unwrap(),
            };

            let mut x = posx + (border_width / 2.0);
            let mut y = posy + (border_width / 2.0);

            let text_string = text.text.to_owned();

            let text_align = state
                .style
                .text_align
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let text_justify = state
                .style
                .text_justify
                .get(entity)
                .cloned()
                .unwrap_or_default();

            let align = match text_justify {
                Justify::Start => {
                    x += padding_left;
                    femtovg::Align::Left
                }
                Justify::Center => {
                    x += 0.5 * width;
                    femtovg::Align::Center
                }
                Justify::End => {
                    x += width - padding_right;
                    femtovg::Align::Right
                }
            };

            let baseline = match text_align {
                Align::Start => {
                    y += padding_top;
                    Baseline::Top
                }
                Align::Center => {
                    y += 0.5 * height;
                    Baseline::Middle
                }
                Align::End => {
                    y += height - padding_bottom;
                    Baseline::Bottom
                }
            };

            let mut font_color: Color = font_color.into();
            font_color.set_alphaf(font_color.a * opacity);

            let font_size = state.style.font_size.get(entity).cloned().unwrap_or(16.0);

            let mut paint = Paint::color(font_color);
            paint.set_font_size(font_size);
            paint.set_font(&[font_id]);
            paint.set_text_align(align);
            paint.set_text_baseline(baseline);
            paint.set_anti_alias(false);

            canvas.fill_text(x, y, &text_string, paint);
        }

        canvas.restore();

        /*
        window.context.borrow_mut().frame(
            (
                state.transform.get_width(state.root),
                state.transform.get_height(state.root),
            ),
            1.0 as f32,
            |mut frame| {

                let zoom = Transform::new().scale(state.transform.get_zoom_scale(entity), state.transform.get_zoom_scale(entity));
                frame.transformed(Transform::new(), |frame| {
                    if entity == Entity::new(0, 0) {
                        return;
                    }

                    // Skip invisible widgets
                    if state.transform.get_visibility(entity) == Visibility::Invisible {
                        //println!("Entity: {} is invisible", entity);
                        return;
                    }

                    if state.transform.get_opacity(entity) == 0.0 {
                        //println!("Entity: {} has 0 opacity", entity);
                        return;
                    }

                    let posx = state.transform.get_posx(entity);
                    let posy = state.transform.get_posy(entity);
                    let width = state.transform.get_width(entity);
                    let height = state.transform.get_height(entity);

                    //println!("DRAW: {} {} {} {} {}", entity, posx, posy, width, height);

                    // Skip widgets with no width or no height
                    if width == 0.0 || height == 0.0 {
                        return;
                    }

                    let parent = state.hierarchy.get_parent(entity).unwrap();

                    let parent_width = state.transform.get_width(parent);

                    // let clip_entity = state
                    //     .style
                    //     .clip_widget
                    //     .get(entity)
                    //     .cloned()
                    //     .unwrap_or_default();

                    let clip_entity = state.transform.get_clip_widget(entity);

                    //let clip_entity = state.root;

                    let clip_posx = state.transform.get_posx(clip_entity);
                    let clip_posy = state.transform.get_posy(clip_entity);
                    let clip_width = state.transform.get_width(clip_entity);
                    let clip_height = state.transform.get_height(clip_entity);

                    //let mut path_opts: PathOptions = Default::default();

                    let padding_left = match state
                        .style
                        .padding_left
                        .get(entity)
                        .unwrap_or(&Length::Auto)
                    {
                        Length::Pixels(val) => val,
                        _ => &0.0,
                    };

                    let padding_right = match state
                        .style
                        .padding_right
                        .get(entity)
                        .unwrap_or(&Length::Auto)
                    {
                        Length::Pixels(val) => val,
                        _ => &0.0,
                    };

                    let padding_top = match state.style.padding_top.get(entity).unwrap_or(&Length::Auto)
                    {
                        Length::Pixels(val) => val,
                        _ => &0.0,
                    };

                    let padding_bottom = match state
                        .style
                        .padding_bottom
                        .get(entity)
                        .unwrap_or(&Length::Auto)
                    {
                        Length::Pixels(val) => val,
                        _ => &0.0,
                    };

                    let rotate = state.style.rotate.get(entity).unwrap_or(&0.0);

                    //let rotate = &10.0;

                    let trans1 = Transform::new().translate(-posx - width / 2.0, -posy - height / 2.0);
                    let rotation = Transform::new().rotate((*rotate as f32).to_radians());
                    let trans2 = Transform::new().translate(posx + width / 2.0, posy + height / 2.0);

                    let transform = trans1 * rotation * trans2;
                    //let rotation = Transform::new().translate(50.0, 0.0);

                    let path_opts = PathOptions {
                        clip: Clip::Scissor(Scissor {
                            x: clip_posx,
                            y: clip_posy,
                            width: clip_width,
                            height: clip_height,
                            transform: None,

                        }),
                        transform: Some(transform),
                        ..Default::default()
                    };

                    let background_color = state
                        .style
                        .background_color
                        .get(entity)
                        .cloned()
                        .unwrap_or_default();

                    let border_color = state
                        .style
                        .border_color
                        .get(entity)
                        .cloned()
                        .unwrap_or_default();

                    let border_radius = state
                        .style
                        .border_radius
                        .get(entity)
                        .cloned()
                        .unwrap_or_default();

                    let border_radius_top_left = match border_radius.top_left {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => parent_width * val,
                        _ => 0.0,
                    };

                    let border_radius_top_right = match border_radius.top_right {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => parent_width * val,
                        _ => 0.0,
                    };

                    let border_radius_bottom_left = match border_radius.bottom_left {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => parent_width * val,
                        _ => 0.0,
                    };

                    let border_radius_bottom_right = match border_radius.bottom_right {
                        Length::Pixels(val) => val,
                        Length::Percentage(val) => parent_width * val,
                        _ => 0.0,
                    };

                    let opacity = state.transform.get_opacity(entity);

                    let mut background_color: nanovg::Color = background_color.into();
                    //let mut background_color: nvg::Color = background_color.into();
                    background_color.set_alpha(background_color.alpha() * opacity);
                    //background_color.a = background_color.a * opacity;

                    let mut border_color: nanovg::Color = border_color.into();
                    //let mut border_color: nvg::Color = border_color.into();
                    border_color.set_alpha(border_color.alpha() * opacity);
                    //border_color.a = border_color.a * opacity;

                    let border_width = state
                        .style
                        .border_width
                        .get(entity)
                        .cloned()
                        .unwrap_or_default();

                    frame.path(
                        |path| {
                            path.rounded_rect_varying(
                                (posx, posy),
                                (width, height),
                                (border_radius_top_left, border_radius_top_right),
                                (border_radius_bottom_left, border_radius_bottom_right),
                            );
                            // if let Some(background_image) = state.style.background_image.get(entity) {
                            //     let image = images.get(background_image).unwrap();
                            //     path.fill(
                            //         ImagePattern {
                            //             image: &image,
                            //             origin: (posx, posy),
                            //             size: (width, height),
                            //             angle: 0.0,
                            //             alpha: opacity,
                            //         },
                            //         Default::default(),
                            //     );
                            // } else {
                                path.fill(background_color, Default::default());
                            //}

                            path.stroke(
                                border_color,
                                StrokeOptions {
                                    width: border_width,
                                    ..Default::default()
                                },
                            );
                        },
                        path_opts,
                    );

                    if let Some(text) = state.style.text.get_mut(entity) {
                        let sans =
                            Font::find(frame.context(), "Roboto-Regular").expect("Failed to load font");
                        let icons = Font::find(frame.context(), "Icons").expect("Failed to load font");

                        let font = match text.font.as_ref() {
                            "Sans" => sans,
                            "Icons" => icons,
                            _ => sans,
                        };
                        let mut align = Alignment::new();

                        let mut x = posx;
                        let mut y = posy;

                        let text_string = text.text.to_owned();

                        let text_align = state
                            .style
                            .text_align
                            .get(entity)
                            .cloned()
                            .unwrap_or_default();
                        let text_justify = state
                            .style
                            .text_justify
                            .get(entity)
                            .cloned()
                            .unwrap_or_default();

                        match text_justify {
                            Justify::Start => {
                                align = align.left();
                                x += padding_left;
                            }
                            Justify::Center => {
                                align = align.center();
                                x += 0.5 * width;
                            }
                            Justify::End => {
                                align = align.right();
                                x += width - padding_right;
                            }
                        }

                        match text_align {
                            crate::Align::Start => {
                                align = align.top();
                                y += padding_top;
                            }
                            crate::Align::Center => {
                                align = align.middle();
                                y += 0.5 * height;
                            }
                            crate::Align::End => {
                                align = align.bottom();
                                y += height - padding_bottom;
                            }
                        }

                        //x += text.indent;

                        let mut font_color: nanovg::Color = text.font_color.into();
                        font_color.set_alpha(font_color.alpha() * opacity);

                        let text_options = TextOptions {
                            color: font_color,
                            size: text.font_size,
                            align: align,
                            clip: Clip::Scissor(Scissor {
                                x: clip_posx,
                                y: clip_posy,
                                width: clip_width,
                                height: clip_height,
                                transform: None,
                            }),
                            transform: Some(transform),
                            //line_height: 1.0,
                            ..Default::default()
                        };

                        frame.text(font, (x, y), &text_string, text_options);
                    }
                });




                //     context.begin_path();
                //     context.reset_transform();
                //     context.translate(posx+width/2.0, posy+height/2.0);
                //     context.rotate(rotate * std::f32::consts::PI / 180.0);
                //     context.translate(-posx-width/2.0,-posy-height/2.0);
                //     context.rounded_rect_varying((posx, posy, width, height), border_radius_top_left, border_radius_top_right, border_radius_bottom_right, border_radius_bottom_left);
                //     context.fill_paint(background_color);
                //     context.stroke_width(border_width);
                //     context.stroke_paint(border_color);
                //     context.fill().unwrap();
                //     context.stroke().unwrap();

                //     if let Some(text) = state.style.text.get_mut(entity) {

                //         let mut font_color: nvg::Color = text.font_color.into();
                //         font_color.a = font_color.a * opacity;

                //         context.fill_paint(font_color);
                //         match text.font.as_ref() {
                //             "Sans" => {context.font("roboto");}
                //             "Icons" => {context.font("entypo");}
                //             _=> {}
                //         }
                //         //context.reset_transform();
                //         //context.rotate(45.0 * std::f32::consts::PI / 180.0);
                //         context.font_size(text.font_size);
                //         context.begin_path();

                //         let text_align = state.style.text_align.get(entity).cloned().unwrap_or_default();
                //         let text_justify = state.style.text_justify.get(entity).cloned().unwrap_or_default();

                //         let mut alignment = Align::empty();

                //         let mut x = posx;
                //         let mut y = posy;

                //         match text_align {
                //             crate::Align::Start => {
                //                 alignment.insert(Align::TOP);
                //                 y += padding_top;
                //             }
                //             crate::Align::Center => {
                //                 alignment.insert(Align::MIDDLE);
                //                 y += 0.5 * height;
                //             }
                //             crate::Align::End => {
                //                 alignment.insert(Align::BOTTOM);
                //                 y += height - padding_bottom;
                //             }
                //         }

                //         match text_justify {
                //             crate::Justify::Start => {
                //                 alignment.insert(Align::LEFT);
                //                 x += padding_left;
                //             }
                //             crate::Justify::Center => {
                //                 alignment.insert(Align::CENTER);
                //                 x += 0.5 * width;
                //             }
                //             crate::Justify::End => {
                //                 alignment.insert(Align::RIGHT);
                //                 x += width - padding_right;
                //             }
                //         }

                //         context.text_align(alignment);
                //         context.text((x, y), &text.text);

                //         context.fill().unwrap();
                //     }
            },
        );
        */
    }
}

pub struct FreqGraph {}

impl FreqGraph {
    pub fn new() -> Self {
        FreqGraph {}
    }
}

impl BuildHandler for FreqGraph {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        let min = 1.477121;
        let max = 3.3013;

        state.style.insert_element(entity, "freq_graph");

        entity
    }
}

use femtovg::{
    renderer::OpenGl, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId, LineCap,
    LineJoin, Paint, Path, Renderer, Solidity,
};

impl EventHandler for FreqGraph {
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        // Skip window
        if entity == Entity::new(0, 0) {
            return;
        }

        // Skip invisible widgets
        if state.transform.get_visibility(entity) == Visibility::Invisible {
            return;
        }

        if state.transform.get_opacity(entity) == 0.0 {
            return;
        }

        let posx = state.transform.get_posx(entity);
        let posy = state.transform.get_posy(entity);
        let width = state.transform.get_width(entity);
        let height = state.transform.get_height(entity);

        //println!("entity: {} posx: {} posy: {} width: {} height: {}", entity, posx, posy, width, height);

        // Skip widgets with no width or no height
        // if width == 0.0 || height == 0.0 {
        //     return;
        // }

        let background_color = state
            .style
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let font_color = state
            .style
            .font_color
            .get(entity)
            .cloned()
            .unwrap_or(tuix::Color::rgb(255, 255, 255));

        let border_color = state
            .style
            .border_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let shadow_color = state
            .style
            .shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let parent = state
            .hierarchy
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.transform.get_width(parent);

        let border_radius_top_left = match state
            .style
            .border_radius_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state
            .style
            .border_radius_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state
            .style
            .border_radius_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state
            .style
            .border_radius_bottom_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let opacity = state.transform.get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let mut shadow_color: femtovg::Color = shadow_color.into();
        shadow_color.set_alphaf(shadow_color.a * opacity);

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        //println!("Border Width: {}", border_width);

        // Apply transformations
        let rotate = state.style.rotate.get(entity).unwrap_or(&0.0);
        let scaley = state.style.scaley.get(entity).cloned().unwrap_or_default();

        canvas.save();
        // canvas.translate(posx + width / 2.0, posy + height / 2.0);
        // canvas.rotate(rotate.to_radians());
        // canvas.translate(-(posx + width / 2.0), -(posy + height / 2.0));

        let pt = canvas
            .transform()
            .inversed()
            .transform_point(posx + width / 2.0, posy + height / 2.0);
        //canvas.translate(posx + width / 2.0, posy + width / 2.0);
        canvas.translate(pt.0, pt.1);
        canvas.scale(1.0, scaley.0);
        canvas.translate(-pt.0, -pt.1);

        // Apply Scissor
        let clip_entity = state.transform.get_clip_widget(entity);

        let clip_posx = state.transform.get_posx(clip_entity);
        let clip_posy = state.transform.get_posy(clip_entity);
        let clip_width = state.transform.get_width(clip_entity);
        let clip_height = state.transform.get_height(clip_entity);

        canvas.scissor(clip_posx, clip_posy, clip_width, clip_height);

        let shadow_h_offset = state
            .style
            .shadow_h_offset
            .get(entity)
            .cloned()
            .unwrap_or_default();

        // Draw shadow
        // let mut path = Path::new();
        // path.rounded_rect_varying(posx, posy, width, height, border_radius_top_left, border_radius_top_right, border_radius_bottom_right, border_radius_bottom_left);
        // let mut paint = Paint::color(background_color);
        // canvas.fill_path(&mut path, paint);

        // Draw rounded rect
        let mut path = Path::new();
        path.rounded_rect_varying(
            posx + (border_width / 2.0),
            posy + (border_width / 2.0),
            width - border_width,
            height - border_width,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        let mut paint = Paint::color(background_color);
        canvas.fill_path(&mut path, paint);

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        //paint.set_anti_alias(false);
        canvas.stroke_path(&mut path, paint);
        //println!("posx: {}", posx);

        // Draw Vertical Lines
        // Convert value to pixel position
        // 30 -

        let min = 1.477121;
        let max = 4.3013;
        let range = max - min;

        for f in &frequencies {
            let t = (f - min) * (width - 80.0) / range;
            let mut path = Path::new();
            path.move_to(posx + 40.5 + t.ceil(), posy);
            path.line_to(posx + 40.5 + t.ceil(), posy + height);
            let mut paint = Paint::color(Color::rgb(80, 80, 80));
            paint.set_line_width(1.0);
            canvas.stroke_path(&mut path, paint);
        }

        for g in 0..5 {
            let t = g as f32 * (height - 80.0) / 4.0;
            let mut path = Path::new();
            path.move_to(posx, posy + 40.5 + t.ceil());
            path.line_to(posx + width, posy + 40.5 + t.ceil());
            let mut paint = Paint::color(Color::rgb(80, 80, 80));
            paint.set_line_width(1.0);
            canvas.stroke_path(&mut path, paint);
        }

        // 30 Hz Label
        //let t = (width - 40.0) / range;
        let mut path = Path::new();
        path.rect(posx + 30.0, posy + height - 27.0, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 49.0, posy + height - 20.0, "30 Hz", label_paint);

        // 100 Hz Label
        let t = (2.0 - min) * (width - 80.0) / range;
        let mut path = Path::new();
        path.rect(posx + 30.0 + t, posy + height - 27.0, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 49.0 + t, posy + height - 20.0, "100 Hz", label_paint);

        // 1 KHz Label
        let t = (3.0 - min) * (width - 80.0) / range;
        let mut path = Path::new();
        path.rect(posx + 30.0 + t, posy + height - 27.0, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 49.0 + t, posy + height - 20.0, "1 kHz", label_paint);

        // 10 KHz Label
        let t = (4.0 - min) * (width - 80.0) / range;
        let mut path = Path::new();
        path.rect(posx + 30.0 + t, posy + height - 27.0, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 49.0 + t, posy + height - 20.0, "10 kHz", label_paint);

        // 20 KHz Label
        let t = width - 80.0;
        let mut path = Path::new();
        path.rect(posx + 30.0 + t, posy + height - 27.0, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 49.0 + t, posy + height - 20.0, "20 kHz", label_paint);

        // -12 dB Label
        let t = 0.0 * (height - 80.0) / 4.0;
        let mut path = Path::new();
        path.rect(posx, posy + height - 47.0, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 20.0, posy + height - 40.0, "-12 dB", label_paint);

        // -6 dB Label
        let t = 1.0 * (height - 80.0) / 4.0;
        let mut path = Path::new();
        path.rect(posx, posy + height - 47.0 - t, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 20.0, posy + height - 40.0 - t, "-6 dB", label_paint);

        // 0 dB Label
        let t = 2.0 * (height - 80.0) / 4.0;
        let mut path = Path::new();
        path.rect(posx, posy + height - 47.0 - t, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 20.0, posy + height - 40.0 - t, "0 dB", label_paint);

        // 6 dB Label
        let t = 3.0 * (height - 80.0) / 4.0;
        let mut path = Path::new();
        path.rect(posx, posy + height - 47.0 - t, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 20.0, posy + height - 40.0 - t, "6 dB", label_paint);

        // 12 dB Label
        let t = 4.0 * (height - 80.0) / 4.0;
        let mut path = Path::new();
        path.rect(posx, posy + height - 47.0 - t, 40.0, 14.0);
        let mut paint = Paint::color(Color::rgb(32, 32, 32));
        canvas.fill_path(&mut path, paint);
        let mut label_paint = Paint::color(Color::rgb(80, 80, 80));
        label_paint.set_text_align(femtovg::Align::Center);
        label_paint.set_text_baseline(Baseline::Middle);
        label_paint.set_font_size(12.0);
        canvas.fill_text(posx + 20.0, posy + height - 40.0 - t, "12 dB", label_paint);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterEvent {
    TypeChange(u32, u8),
    FreqChange(u32, f32),
    GainChange(u32, f32),
    Disabled(u32),
    Enabled(u32),
}

pub struct ControlPoint {
    moving: bool,
    px: f32,
    py: f32,
    label: String,

    frequency: f32,
    gain: f32,
}

impl ControlPoint {
    pub fn new(label: &str) -> Self {
        ControlPoint {
            moving: false,
            px: 0.0,
            py: 0.0,
            label: label.to_string(),

            frequency: 30.0,
            gain: 0.0,
        }
    }
}

impl BuildHandler for ControlPoint {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_text(state, &self.label);

        state.style.insert_element(entity, "control_point");

        entity
    }
}

impl EventHandler for ControlPoint {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                // Currently this gets stuck in a recursion because other control points trigger a relayout event
                // Need a better way to figure out if a parent has been resized
                // Current thinking is a GeometryChanged event that is passed directly to the widget changing size
                // And then optionally propagated to the children
                WindowEvent::Relayout => {
                    // Prevents infinite recursion (except when there are multiple control points)
                    if event.origin != entity && !state.hierarchy.is_sibling(event.origin, entity) {
                        let parent = state.hierarchy.get_parent(entity).unwrap();
                        let parent_width = state.transform.get_width(parent);
                        let parent_height = state.transform.get_height(parent);
                        let width = state.transform.get_width(entity);
                        let height = state.transform.get_height(entity);

                        let min = 1.477121;
                        let max = 4.3013;
                        let range = max - min;

                        let new_left =
                            40.0 + (self.frequency.log10() - min) * ((parent_width - 80.0) / range);
                        let new_top = 40.0 + (-self.gain + 12.0) * ((parent_height - 80.0) / 24.0);

                        entity.set_left(state, Length::Pixels(new_left - width / 2.0));
                        entity.set_top(state, Length::Pixels(new_top - height / 2.0));
                    }
                }

                WindowEvent::MouseDown(button) => {
                    if event.target == entity && *button == MouseButton::Left {
                        self.moving = true;
                        self.px = state.transform.get_posx(entity);
                        self.py = state.transform.get_posy(entity);
                        state.capture(entity);
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if event.target == entity && *button == MouseButton::Left {
                        self.moving = false;
                        state.release(entity);
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    if self.moving {
                        let parent = state.hierarchy.get_parent(entity).unwrap();

                        let parent_posx = state.transform.get_posx(parent);
                        let parent_posy = state.transform.get_posy(parent);
                        let parent_width = state.transform.get_width(parent);
                        let parent_height = state.transform.get_height(parent);

                        let width = state.transform.get_width(entity);
                        let height = state.transform.get_height(entity);

                        let ddx = state.mouse.left.pos_down.0 - self.px;
                        let ddy = state.mouse.left.pos_down.1 - self.py;

                        let dx = *x - parent_posx;
                        let dy = *y - parent_posy;

                        // Convert to frequency and gain
                        let min = 1.477121;
                        let max = 4.3013;
                        let range = max - min;

                        let mut new_left = dx - ddx;
                        let mut new_top = dy - ddy;

                        let mut f = (((new_left + (width / 2.0) - 40.0) * range)
                            / (parent_width - 80.0))
                            + min;

                        if f <= min {
                            new_left = 40.0 - width / 2.0;
                            f = min;
                        }

                        if f >= max {
                            new_left = parent_width - 40.0 - width / 2.0;
                            f = max;
                        }

                        let mut g = -((((new_top + height / 2.0) - 40.0) * 24.0
                            / (parent_height - 80.0))
                            - 12.0);

                        if g <= -12.0 {
                            new_top = parent_height - 40.0 - height / 2.0;
                            g = -12.0;
                        }

                        if g >= 12.0 {
                            new_top = 40.0 - height / 2.0;
                            g = 12.0;
                        }

                        //println!("Freq: {}, Gain: {}", 10.0f32.powf(f), g);

                        self.frequency = 10.0f32.powf(f);
                        self.gain = g;

                        state.insert_event(
                            Event::new(SliderEvent::SetValue(self.frequency))
                                .target(state.id2entity("channel1_freq_knob").unwrap()),
                        );
                        //state.insert_event(Event::new(FilterEvent::GainChange(1,self.gain)));

                        entity.set_left(state, Length::Pixels(new_left));
                        entity.set_top(state, Length::Pixels(new_top));

                        //println!("dx: {} dy: {}", dx, dy);
                    }
                }

                _ => {}
            }
        }

        false
    }
}

pub struct ChannelOutput {}

impl ChannelOutput {
    pub fn new() -> Self {
        ChannelOutput {}
    }
}

impl BuildHandler for ChannelOutput {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        let channels = HBox::new().build(state, entity, |builder| builder);

        let audio_level = AudioLevelBar::new().build(state, channels, |builder| {
            builder
                .set_height(Length::Pixels(180.0))
                .set_width(Length::Pixels(10.0))
        });

        let audio_level = AudioLevelBar::new().build(state, channels, |builder| {
            builder
                .set_height(Length::Pixels(180.0))
                .set_width(Length::Pixels(10.0))
        });

        ValueKnob::new("Gain", 0.0, -12.0, 12.0).build(state, entity, |builder| builder);
        ValueKnob::new("Mix", 1.0, 0.0, 1.0).build(state, entity, |builder| builder);

        state.style.insert_element(entity, "channel_output");

        entity
    }
}

impl EventHandler for ChannelOutput {}

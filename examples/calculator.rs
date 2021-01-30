extern crate tuix;

use image::GenericImageView;

use tuix::*;

static LIGHT_THEME: &'static str = include_str!("themes/calculator_light_theme.css");

#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorEvent {
    Digit(char),
    Operator(char),
}

// impl Message for CalculatorEvent {}

#[derive(Default)]
pub struct CalculatorState {
    display: Entity,

    zero: Entity,
    one: Entity,
    two: Entity,
    three: Entity,
    four: Entity,
    five: Entity,
    six: Entity,
    seven: Entity,
    eight: Entity,
    nine: Entity,

    clear: Entity,
    multiply: Entity,
    divide: Entity,
    subtract: Entity,
    add: Entity,
    percent: Entity,
    plus_minus: Entity,
    decimal_point: Entity,
    equals: Entity,

    input: String,
    left_side: f64,
    right_side: Option<f64>,
    operator: Option<char>,
}

impl CalculatorState {

    pub fn update_display(&self, state: &mut State) {
        self.display.set_text(state, &self.left_side.to_string());

        // if self.right_side > 0.0 {
        //     self.display.set_text(state, &self.right_side.to_string());
        // } else if self.left_side > 0.0 {
        //     self.display.set_text(state, &self.left_side.to_string());
        // } else {
        //     self.display.set_text(state, "0");
        // }
    }

    pub fn clear_all(&mut self, state: &mut State) {
        self.input.clear();
        self.left_side = 0.0;
        self.right_side = None;
        self.operator = None;
        self.update_display(state);
    }
}

#[derive(Default)]
pub struct Calculator {

}

impl BuildHandler for Calculator {
    type Ret = Handle;
    fn on_build(&mut self, state: &mut State, handle: Handle) -> Self::Ret {


        let mut calculator_state = CalculatorState::default();

        let container = Element::new().build(state, &handle).add_class("container");

        // Change to label that can be copied but not edited at some point
        calculator_state.display = Button::new().build(state, &container).set_text("0").add_class("display").entity();

        // Currently using flexbox to create the layout but would be good to use grid when working

        let row1 = Button::new().build(state, &container).add_class("row");

        calculator_state.clear = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('C')))
            .build(state, &row1)
            .set_text("AC")
            .add_class("digit")
            .entity();

        calculator_state.plus_minus = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('¬')))
            .build(state, &row1)
            .set_text("\u{00B1}")
            .add_class("digit")
            .entity();

        // Percentage
        calculator_state.percent = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('%')))
            .build(state, &row1)
            .set_text("\u{0025}")
            .add_class("digit")
            .entity();

        // Divide
        calculator_state.divide = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('/')))
            .build(state, &row1)
            .set_text("\u{00F7}")
            .add_class("operator")
            .entity();

        // Second Row
        let row2 = Button::new()
            .build(state, &container)
            .add_class("row");

        // Digit Seven
        calculator_state.seven = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('7')))
            .build(state, &row2)
            .set_text("7")
            .add_class("digit")
            .entity();

        // Digit Eight
        calculator_state.eight = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('8')))
            .build(state, &row2)
            .set_text("8")
            .add_class("digit")
            .entity();

        // Digit Nine
        calculator_state.nine = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('9')))
            .build(state, &row2)
            .set_text("9")
            .add_class("digit")
            .entity();

        // Multiply
        calculator_state.multiply = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('*')))
            .build(state, &row2)
            .set_text("\u{00D7}")
            .add_class("operator")
            .entity();

        // Third Row
        let row3 = Button::new().build(state, &container).add_class("row");

        // Digit Four
        calculator_state.four = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('4')))
            .build(state, &row3)
            .set_text("4")
            .add_class("digit")
            .entity();

        // Digit Five
        calculator_state.five = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('5')))
            .build(state, &row3)
            .set_text("5")
            .add_class("digit")
            .entity();

        // Digit Six
        calculator_state.six = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('6')))
            .build(state, &row3)
            .set_text("6")
            .add_class("digit")
            .entity();

        // Subtract
        calculator_state.subtract = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('-')))
            .build(state, &row3)
            .set_text("\u{002D}")
            .add_class("operator")
            .entity();

        // Fourth Row
        let row4 = Button::new().build(state, &container).add_class("row");

        // Digit One
        calculator_state.one = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('1')))
            .build(state, &row4)
            .set_text("1")
            .add_class("digit")
            .entity();

        // Digit Two
        calculator_state.two = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('2')))
            .build(state, &row4)
            .set_text("2")
            .add_class("digit")
            .entity();

        // Digit Three
        calculator_state.three = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('3')))
            .build(state, &row4)
            .set_text("3")
            .add_class("digit")
            .entity();

        // Add
        calculator_state.add = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('+')))
            .build(state, &row4)
            .set_text("\u{002B}")
            .add_class("operator")
            .entity();

        // Fifth Row
        let row5 = Button::new().build(state, &container).add_class("last_row");

        // Digit Zero
        calculator_state.zero = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('0')))
            .build(state, &row5)
            .set_text("0")
            //.set_flex_grow(2.0)
            .add_class("digit")
            .entity();

        // Decimal Point
        calculator_state.decimal_point = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('.')))
            .build(state, &row5)
            .set_text(".")
            .add_class("digit")
            .entity();

        // Equals
        calculator_state.equals = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('=')))
            .build(state, &row5)
            .set_text("\u{003D}")
            .add_class("operator")
            .entity();

        state.focused = calculator_state.display;

        // self.display
        //     .set_focus_order(state, self.clear, self.decimal_point);
        // self.clear
        //     .set_focus_order(state, self.plus_minus, self.display);
        // self.plus_minus
        //     .set_focus_order(state, self.percent, self.clear);
        // self.percent
        //     .set_focus_order(state, self.divide, self.plus_minus);
        // self.divide
        //     .set_focus_order(state, self.multiply, self.percent);
        // self.multiply
        //     .set_focus_order(state, self.subtract, self.divide);
        // self.subtract
        //     .set_focus_order(state, self.add, self.multiply);
        // self.add.set_focus_order(state, self.equals, self.subtract);
        // self.equals.set_focus_order(state, self.zero, self.add);
        // self.zero.set_focus_order(state, self.one, self.equals);
        // self.one.set_focus_order(state, self.two, self.zero);
        // self.two.set_focus_order(state, self.three, self.one);
        // self.three.set_focus_order(state, self.four, self.two);
        // self.four.set_focus_order(state, self.five, self.three);
        // self.five.set_focus_order(state, self.six, self.four);
        // self.six.set_focus_order(state, self.seven, self.five);
        // self.seven.set_focus_order(state, self.eight, self.six);
        // self.eight.set_focus_order(state, self.nine, self.seven);
        // self.nine
        //     .set_focus_order(state, self.decimal_point, self.eight);
        // self.decimal_point
        //     .set_focus_order(state, self.display, self.nine);

        handle.add_component(calculator_state).add_event_handler(calculator_event_handler).add_event_handler(calculator_event_handler2)
    }
}

pub fn calculator_event_handler(calculator_state: &mut CalculatorState, state: &mut State, handle: &Handle, event_data: &EventData, calculator_event: &mut CalculatorEvent) -> bool {
    match calculator_event {
        CalculatorEvent::Digit(num) => {
            if *num == '¬' {
                if calculator_state.input.len() > 0 {
                    if calculator_state.input.contains("-") {
                        calculator_state.input.remove(0);
                    } else {
                        calculator_state.input.insert(0, '-');
                    }
                } else {
                    calculator_state.input = (calculator_state.left_side * -1.0).to_string();
                }
            } else if *num == '%' {
                if let Some(right_side) = calculator_state.right_side {
                    if let Some(operator) = calculator_state.operator {
                        calculator_state.right_side = match operator {
                            '+' | '-' => Some(calculator_state.left_side * 0.01 * right_side),
                            '*' | '/' => Some(0.01 * right_side),
                            _ => Some(right_side),
                        }
                    }

                    calculator_state.input = calculator_state.right_side.unwrap().to_string();
                }
            } else if *num == '.' {
                if calculator_state.input.len() == 0 {
                    calculator_state.input.push('0');
                    calculator_state.input.push('.');
                } else {
                    calculator_state.input.push('.');
                }
            } else {
                if calculator_state.input.len() < 15 {
                    calculator_state.input.push(*num);
                }
            }

            println!("input: {}", calculator_state.input);

            calculator_state.right_side = match calculator_state.input.parse::<f64>() {
                Ok(val) => Some(val),
                Err(_) => {
                    calculator_state.input.pop();
                    calculator_state.right_side
                }
            };

            if !calculator_state.input.is_empty() {
                calculator_state.display.set_text(state, &calculator_state.input);
            } else {
                calculator_state.display.set_text(state, "0");
            }
        }

        CalculatorEvent::Operator(op) => {
            if let Some(right_side) = calculator_state.right_side {
                match calculator_state.operator {
                    Some(operator) => {
                        calculator_state.left_side = match operator {
                            '+' => calculator_state.left_side + right_side,
                            '-' => calculator_state.left_side - right_side,
                            '*' => calculator_state.left_side * right_side,
                            '/' => calculator_state.left_side / right_side,
                            '%' => calculator_state.left_side,
                            _ => right_side,
                        };
                    }

                    None => calculator_state.left_side = right_side,
                }

                calculator_state.right_side = None;
            }

            calculator_state.input.clear();
            calculator_state.update_display(state);

            match op {
                '+' => {
                    calculator_state.operator = Some('+');
                }

                '-' => {
                    calculator_state.operator = Some('-');
                }

                '*' => {
                    calculator_state.operator = Some('*');
                }

                '/' => {
                    calculator_state.operator = Some('/');
                }

                '=' => {
                    calculator_state.operator = Some('=');
                }

                'C' => {
                    calculator_state.clear_all(state);
                }

                _ => {}
            }
        }
    }

    false
}

pub fn calculator_event_handler2(calculator_state: &mut CalculatorState, state: &mut State, handle: &Handle, event_data: &EventData, window_event: &mut WindowEvent) -> bool {
    match window_event {
        WindowEvent::KeyDown(code, key) => {
            match code {
                Code::Digit0 => {
                    state.active = calculator_state.zero;
                    state.insert_event(
                        Event::new(CalculatorEvent::Digit('0')).target(handle.entity),
                    );
                }

                Code::Digit1 => {
                    state.active = calculator_state.one;
                    state.insert_event(
                        Event::new(CalculatorEvent::Digit('1')).target(handle.entity),
                    );
                }

                Code::Digit2 => {
                    state.active = calculator_state.two;
                    state.insert_event(
                        Event::new(CalculatorEvent::Digit('2')).target(handle.entity),
                    );
                }

                Code::Digit3 => {
                    state.active = calculator_state.three;
                    state.insert_event(
                        Event::new(CalculatorEvent::Digit('3')).target(handle.entity),
                    );
                }

                Code::Digit4 => {
                    state.active = calculator_state.four;
                    state.insert_event(
                        Event::new(CalculatorEvent::Digit('4')).target(handle.entity),
                    );
                }

                Code::Digit5 => {
                    if state.modifiers.shift {
                        state.active = calculator_state.percent;
                        state.insert_event(
                            Event::new(CalculatorEvent::Digit('%')).target(handle.entity),
                        );
                    } else {
                        state.active = calculator_state.five;
                        state.insert_event(
                            Event::new(CalculatorEvent::Digit('5')).target(handle.entity),
                        );
                    }
                }

                Code::Digit6 => {
                    state.active = calculator_state.six;
                    state.insert_event(
                        Event::new(CalculatorEvent::Digit('6')).target(handle.entity),
                    );
                }

                Code::Digit7 => {
                    state.active = calculator_state.seven;
                    state.insert_event(
                        Event::new(CalculatorEvent::Digit('7')).target(handle.entity),
                    );
                }

                Code::Digit8 => {
                    if state.modifiers.shift {
                        state.active = calculator_state.multiply;
                        state.insert_event(
                            Event::new(CalculatorEvent::Operator('*')).target(handle.entity),
                        );
                    } else {
                        state.active = calculator_state.eight;
                        state.insert_event(
                            Event::new(CalculatorEvent::Digit('8')).target(handle.entity),
                        );
                    }
                }

                Code::Digit9 => {
                    state.active = calculator_state.nine;
                    state.insert_event(
                        Event::new(CalculatorEvent::Digit('9')).target(handle.entity),
                    );
                }

                Code::Escape => {
                    state.active = calculator_state.clear;
                    calculator_state.clear_all(state);
                }

                Code::NumpadMultiply => {
                    state.active = calculator_state.multiply;
                    state.insert_event(
                        Event::new(CalculatorEvent::Operator('*')).target(handle.entity),
                    );
                }

                Code::NumpadSubtract => {
                    state.active = calculator_state.subtract;
                    state.insert_event(
                        Event::new(CalculatorEvent::Operator('-')).target(handle.entity),
                    );
                }

                Code::NumpadAdd => {
                    state.active = calculator_state.add;
                    state.insert_event(
                        Event::new(CalculatorEvent::Operator('+')).target(handle.entity),
                    );
                }

                Code::NumpadDivide => {
                    state.active = calculator_state.divide;
                    state.insert_event(
                        Event::new(CalculatorEvent::Operator('/')).target(handle.entity),
                    );
                }

                Code::NumpadDecimal => {
                    state.active = calculator_state.decimal_point;
                    state.insert_event(
                        Event::new(CalculatorEvent::Digit('.')).target(handle.entity),
                    );
                }

                Code::Equal => {
                    if state.modifiers.shift {
                        state.active = calculator_state.add;
                        state.insert_event(
                            Event::new(CalculatorEvent::Operator('+')).target(handle.entity),
                        );
                    } else {
                        state.active = calculator_state.equals;
                        state.insert_event(
                            Event::new(CalculatorEvent::Operator('=')).target(handle.entity),
                        );
                    }
                }

                Code::NumpadEnter | Code::Enter => {
                    state.active = calculator_state.equals;
                    state.insert_event(
                        Event::new(CalculatorEvent::Operator('=')).target(handle.entity),
                    );
                }

                _ => {}
            }

            state.insert_event(Event::new(WindowEvent::Restyle).target(state.root));
        }

        WindowEvent::KeyUp(_, _) => {
            state.active = Entity::null();
            state.insert_event(Event::new(WindowEvent::Restyle).target(state.root));
        }

        _ => {}
    }

    false
}

// impl EventHandler for Calculator {
//     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
//         if let Some(calculator_event) = event.message.downcast::<CalculatorEvent>() {
            
//         }

//         if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            
//         }

//         false
//     }
// }

pub fn main() {
    // Replace this with icon loading using resource manager when working
    let icon = image::open("resources/icons/calculator_dark-128.png").unwrap();

    let mut app = Application::new(|win_desc, state, window| {
        state.insert_theme(LIGHT_THEME);

        Calculator::default().build(state, &window).add_class("calculator");

        win_desc
            .with_title("Calculator")
            .with_inner_size(300, 400)
            .with_min_inner_size(200, 300)
            .with_icon(icon.to_bytes(), icon.width(), icon.height())
    });

    app.run();
}

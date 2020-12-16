// extern crate tuix;

// use tuix::Application;
// use tuix::widgets::Button;

// use tuix::events::BuildHandler;

// use tuix::style::{Color, Length};

// fn main() {
//     let mut app = Application::new(|window| window.with_title("Hello GUI"));

//     // let state = app.get_state();
//     // let window = state.root;

//     // Button::new().build(state, window, |builder| {
//     //     builder
//     //         .set_width(Length::Pixels(100.0))
//     //         .set_height(Length::Pixels(30.0))
//     //         .set_border_width(2.0)
//     //         .set_border_color(Color::rgb(0,0,0))
//     //         .set_background_color(Color::rgb(50,50,100))
//     //         .set_border_radius(Length::Pixels(5.0))
//     //         .set_text("TEST")
//     // });

//     app.run();
// }

use femtovg::{
    //CompositeOperation,
    renderer::OpenGl,
    Align,
    Baseline,
    Canvas,
    Color,
    FillRule,
    FontId,
    ImageFlags,
    ImageId,
    LineCap,
    LineJoin,
    Paint,
    Path,
    Renderer,
    Solidity,
};

use std::time::Duration;

use rtrb::{RingBuffer, Consumer};

use baseview::{Event, Window, WindowHandler, WindowScalePolicy};

use raw_gl_context::GlContext;

#[derive(Debug, Clone)]
enum Message {
    Hello
}

struct OpenWindowExample {
    rx: Consumer<Message>,
    context: GlContext,
    canvas: Canvas<OpenGl>,
}

impl WindowHandler for OpenWindowExample {
    fn on_frame(&mut self) {
        while let Ok(message) = self.rx.pop() {
            println!("Message: {:?}", message);
        }

        self.context.make_current();

        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }



        self.canvas.set_size(512, 512, 1.0);
        self.canvas.clear_rect(0, 0, 512, 512, Color::rgbf(0.3, 0.3, 0.32));

        draw_colorwheel(&mut self.canvas, 200.0, 200.0, 200.0, 200.0, 0.0);

        self.canvas.flush();
        self.context.swap_buffers();
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) {
        match event {
            Event::Mouse(e) => println!("Mouse event: {:?}", e),
            Event::Keyboard(e) => println!("Keyboard event: {:?}", e),
            Event::Window(e) => println!("Window event: {:?}", e),
        }
    }
}

fn main() {
    let window_open_options = baseview::WindowOpenOptions {
        title: "baseview".into(),
        size: baseview::Size::new(512.0, 512.0),
        scale: WindowScalePolicy::SystemScaleFactor,
        parent: baseview::Parent::None,
    };

    let (mut tx, rx) = RingBuffer::new(128).split();

    let opt_app_runner = Window::open(
        window_open_options,
        |window| {
            let context = GlContext::create(window).unwrap();
            context.make_current();
            gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
            let renderer = OpenGl::new(|symbol| context.get_proc_address(symbol) as *const _).expect("Cannot create renderer");
            let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

            OpenWindowExample { rx, context, canvas }
        } 
    );

    ::std::thread::spawn(move || {
        loop {
            ::std::thread::sleep(Duration::from_secs(5));

            if let Err(_) = tx.push(Message::Hello) {
                println!("Failed sending message");
            }
        }
    });

    opt_app_runner.unwrap().app_run_blocking();
}

fn draw_colorwheel<T: Renderer>(canvas: &mut Canvas<T>, x: f32, y: f32, w: f32, h: f32, t: f32) {
    
    let PI = 3.141592;
    let t = 0.0f32;
    let hue = (t * 0.12).sin();

    canvas.save();

    let cx = x + w * 0.5;
    let cy = y + h * 0.5;
    let r1 = if w < h { w } else { h } * 0.5 - 5.0;
    let r0 = r1 - 20.0;
    let aeps = 0.5 / r1;

    for i in 0..6 {
        let a0 = i as f32 / 6.0 * PI * 2.0 - aeps;
        let a1 = (i as f32 + 1.0) / 6.0 * PI * 2.0 + aeps;

        let mut path = Path::new();
        path.arc(cx, cy, r0, a0, a1, Solidity::Hole);
        path.arc(cx, cy, r1, a1, a0, Solidity::Solid);
        path.close();

        let ax = cx + a0.cos() * (r0 + r1) * 0.5;
        let ay = cy + a0.sin() * (r0 + r1) * 0.5;
        let bx = cx + a1.cos() * (r0 + r1) * 0.5;
        let by = cy + a1.sin() * (r0 + r1) * 0.5;

        let paint = Paint::linear_gradient(
            ax,
            ay,
            bx,
            by,
            Color::hsla(a0 / (PI * 2.0), 1.0, 0.55, 1.0),
            Color::hsla(a1 / (PI * 2.0), 1.0, 0.55, 1.0),
        );

        canvas.fill_path(&mut path, paint);
    }

    let mut path = Path::new();
    path.circle(cx, cy, r0 - 0.5);
    path.circle(cx, cy, r1 + 0.5);
    let mut paint = Paint::color(Color::rgba(0, 0, 0, 64));
    paint.set_line_width(1.0);
    canvas.stroke_path(&mut path, paint);

    // Selector
    canvas.save();
    canvas.translate(cx, cy);
    canvas.rotate(hue * PI * 2.0);

    // Marker on
    let mut path = Path::new();
    path.rect(r0 - 1.0, -3.0, r1 - r0 + 2.0, 6.0);
    paint = Paint::color(Color::rgba(255, 255, 255, 192));
    paint.set_line_width(2.0);
    canvas.stroke_path(&mut path, paint);

    paint = Paint::box_gradient(
        r0 - 3.0,
        -5.0,
        r1 - r0 + 6.0,
        10.0,
        2.0,
        4.0,
        Color::rgba(0, 0, 0, 128),
        Color::rgba(0, 0, 0, 0),
    );
    let mut path = Path::new();
    path.rect(r0 - 2.0 - 10.0, -4.0 - 10.0, r1 - r0 + 4.0 + 20.0, 8.0 + 20.0);
    path.rect(r0 - 2.0, -4.0, r1 - r0 + 4.0, 8.0);
    path.solidity(Solidity::Hole);
    canvas.fill_path(&mut path, paint);

    // Center triangle
    let r = r0 - 6.0;
    let ax = (120.0 / 180.0 * PI).cos() * r;
    let ay = (120.0 / 180.0 * PI).sin() * r;
    let bx = (-120.0 / 180.0 * PI).cos() * r;
    let by = (-120.0 / 180.0 * PI).sin() * r;

    let mut path = Path::new();
    path.move_to(r, 0.0);
    path.line_to(ax, ay);
    path.line_to(bx, by);
    path.close();
    paint = Paint::linear_gradient(
        r,
        0.0,
        ax,
        ay,
        Color::hsla(hue, 1.0, 0.5, 1.0),
        Color::rgba(255, 255, 255, 255),
    );
    canvas.fill_path(&mut path, paint);
    paint = Paint::linear_gradient(
        (r + ax) * 0.5,
        ay * 0.5,
        bx,
        by,
        Color::rgba(0, 0, 0, 0),
        Color::rgba(0, 0, 0, 255),
    );
    canvas.fill_path(&mut path, paint);
    paint = Paint::color(Color::rgba(0, 0, 0, 64));
    canvas.stroke_path(&mut path, paint);

    // Select circle on triangle
    let ax = (120.0 / 180.0 * PI).cos() * r * 0.3;
    let ay = (120.0 / 180.0 * PI).sin() * r * 0.4;
    paint = Paint::color(Color::rgba(255, 255, 255, 192));
    paint.set_line_width(2.0);
    let mut path = Path::new();
    path.circle(ax, ay, 5.0);
    canvas.stroke_path(&mut path, paint);

    paint = Paint::radial_gradient(ax, ay, 7.0, 9.0, Color::rgba(0, 0, 0, 64), Color::rgba(0, 0, 0, 0));
    let mut path = Path::new();
    path.rect(ax - 20.0, ay - 20.0, 40.0, 40.0);
    path.circle(ax, ay, 7.0);
    path.solidity(Solidity::Hole);
    canvas.fill_path(&mut path, paint);

    canvas.restore();

    canvas.restore();
}
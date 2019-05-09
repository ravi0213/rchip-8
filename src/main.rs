extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod cpu;
use std::fs::File;
use std::io::Read;
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use std::env;
use graphics::*;

//struct App {
//    gl: GlGraphics,
//    rotation: f64
//}
//
//impl App {
//    fn render(&mut self, args: &RenderArgs) {
//        use graphics::*;
//
//        const BLACK:[f32;4] = [0.0, 0.0, 0.0,1.0];
//        const WHITE:[f32;4] = [1.0,1.0,1.0,1.0];
//
//        let square = rectangle::square(0.0,0.0,8.0);
//        let rotation = self.rotation;
//        let (x,y) = (args.width/2.0, args.height / 2.0);
//
//        self.gl.draw(args.viewport(), |c,gl| {
//           clear(BLACK, gl) ;
//            let transform = c.transform.trans(x,y).rot_rad(rotation).trans(-25.0,-25.0);
//            rectangle(WHITE,square,transform,gl);
//        });
//    }
//
//    fn update(&mut self, args: &UpdateArgs) {
//        self.rotation += 2.0 * args.dt;
//    }
//}


fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    println!("{:?}", env::current_dir()?);
    let mut file = File::open("resources/PONG").unwrap();

    let mut buffer = Vec::new();
    let _size = file.read_to_end(&mut buffer);
    println!("{:?}",buffer);


//
    let opengl = OpenGL::V3_2;

    let mut window:Window = WindowSettings::new("rchip-8",[64 ,32])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut cpu = cpu::CPU::new(&buffer, opengl);
//    println!("{:?}", opengl);
// //   let mut window = GlutinWindow::new()
//    let mut app = App{
//        gl:GlGraphics::new(opengl),
//        rotation:0.0
//    };
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args() {
            cpu.render(&r);
        }
        if let Some(u) = e.update_args() {
            cpu.update(&u);
        }
    }



    Ok(())
}

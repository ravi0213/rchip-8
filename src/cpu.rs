extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;

use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use multiarray::*;

pub struct CPU<'a> {
    register: [u8; 16],
    instruction: u16,
    delay_timer:u8,
    sound_timer:u8,
    pc:u16, //program counter
    stack_pointer:u8,
    stack:[u16;16],
    gl: GlGraphics,
    data: &'a Vec<u8>,
    grid: Array2D<i32>,
}

impl<'a> CPU<'a> {

    pub fn new(file: &std::vec::Vec<u8>) -> CPU {
        let opengl = OpenGL::V3_2;
        let displayGrid = Array2D::new([64,32],0);

//        let mut windowGl:Window = WindowSettings::new("rchip-8",[64 ,32])
//            .opengl(opengl)
//            .exit_on_esc(true)
//            .build()
//            .unwrap();
        CPU {
            register: [0; 16],
            instruction: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0,
            stack_pointer:0,
            stack:[0;16],
            gl: GlGraphics::new(opengl),
            data: file,
            grid: displayGrid
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK:[f32;4] = [0.0, 0.0, 0.0,1.0];
        const WHITE:[f32;4] = [1.0,1.0,1.0,1.0];

        let square = rectangle::square(0.0,0.0,8.0);

        self.gl.draw(args.viewport(), |c,gl| {
            clear(BLACK, gl) ;

            for i in 0..64 {
                for j in 0..32 {
                    let transform = c.transform.trans(i as f64,j as f64);
                    rectangle(WHITE, square, transform, gl);
                }
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.parse_operation_code(self.data[self.pc]);
    }

    fn parse_operation_code(&mut self, opcode: u16) {
        match opcode & 0xf000 {
            0x0000 => {
                self.pc = opcode & 0x0fff;
            },
            0x1000 => {
                self.pc = opcode & 0x0fff;
            },
            0x2000 => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.pc;
                self.pc = opcode & 0x0fff;
            },
            0x3000 => {
                if self.register[((opcode & 0x0f00) >> 8) as usize] == (opcode & 0x00ff) as u8 {
                    self.pc += 2;
                }
            },
            0x4000 => {
                if self.register[((opcode & 0x0f00) >> 8) as usize] != (opcode & 0x00ff) as u8 {
                    self.pc += 2;
                }
            },
            0x5000 => {
                if self.register[((opcode & 0x0f00) >> 8) as usize] == self.register[((opcode & 0x00f0) >> 4) as usize] {
                    self.pc += 2;
                }
            },
            0x6000 => {
                self.register[((opcode & 0x0f00) >> 8) as usize] = (opcode & 0x00ff) as u8;
            },
            0x7000 => {
                self.register[((opcode & 0x0f00) >> 8) as usize] += (opcode & 0x00ff) as u8;
            },
            0x8000 => {
                match opcode & 0x000f {
                    0x0000 => {
                        self.register[((opcode & 0x0f00) >> 8) as usize] = self.register[((opcode & 0x00f0) >> 4) as usize];
                    },
                    0x0001 => {
                        self.register[((opcode & 0x0f00) >> 8) as usize] |= self.register[((opcode & 0x00f0) >> 4) as usize];
                    },
                    0x0002 => {
                        self.register[((opcode & 0x0f00) >> 8) as usize] &= self.register[((opcode & 0x00f0) >> 4) as usize];
                    },
                    0x0003 => {
                        self.register[((opcode & 0x0f00) >> 8) as usize] ^= self.register[((opcode & 0x00f0) >> 4) as usize];
                    },
                    0x0004 => {
                        self.register[0x000f as usize] = if (self.register[((opcode & 0x0f00) >> 8) as usize] as u16 + self.register[((opcode & 0x00f0) >> 4) as usize] as u16) > 0x00ff {
                            0x0001
                        } else {
                            0x0000
                        };
                        self.register[((opcode & 0x0f00) >> 8) as usize] += self.register[((opcode & 0x00f0) >> 4) as usize];

                    },
                    0x0005 => {
                        self.register[0x000f] = if self.register[((opcode & 0x0f00) >> 8) as usize] > self.register[((opcode & 0x00f0) >> 4) as usize] {
                            0x0001
                        } else {
                            0x0000
                        };
                        self.register[((opcode & 0x0f00) >> 8) as usize] -= self.register[((opcode & 0x00f0) >> 4) as usize];

                    },
                    0x0006 => {
                        self.register[0x000f] = self.register[((opcode & 0x0f00) >> 8) as usize] & 0x0001;
                        self.register[((opcode & 0x0f00) >> 8) as usize] /= 0x0002;
                    },
                    0x0007 => {
                        self.register[0x000f] = if self.register[((opcode & 0x0f00) >> 8) as usize] < self.register[((opcode & 0x00f0) >> 4) as usize] {
                            0x0001
                        } else {
                            0x0000
                        };
                        self.register[((opcode & 0x0f00) >> 8) as usize] = self.register[((opcode & 0x00f0) >> 4) as usize] - self.register[((opcode & 0x0f00) >> 8) as usize];
                    },
                    0x000e => {
                        self.register[0x000f] = self.register[((opcode & 0x0f00) >> 8) as usize] & ((0x0e00 >> 8) as u8);
                        self.register[0x0f00] *= 0x0002;
                    }
                    _ => {
                        println!("unsupported opcode");
                    }
                }
            },
            0x9000 => {
                if self.register[0x0f00] != self.register[0x00f0] {
                    self.pc += 0x0002;
                }
            },
            0xa000 => {


                self.instruction = opcode & 0x0fff;
            },
            0xb000 => {
                self.pc = (opcode & 0x0fff) + (self.register[0x0000] as u16);
            },
            0xc000 => {
                self.register[(opcode & 0x0f00) as usize] = (opcode & 0x00ff) as u8;
            },
            0xd000 => {
                //display
                let numberOfBytes = (opcode & 0x000f);
                let mut x = opcode & 0x0f00;
                let mut y = opcode & 0x00f0;
                self.register[15] = 0;
                for xi in 0..numberOfBytes {
                    let mut yi = 0;
                    while yi < 8 {
                        let prev = self.grid[[(x+xi)%64,(y+yi)%32]];
                        self.grid[[(x+xi)%64, (y+yi)%32]] = self.grid[[(x+xi)%64,(y+yi)%32]] ^ ((1<<(7-yi)) & self.instruction);
                        if prev == 1 && self.grid[[(x+xi)%64,(y+yi)%32]] == 0 {
                            self.register[15] = 1;
                        }
                        yi = yi+1;
                    }
                    self.instruction = self.instruction+1;
                }
            },
            0xe000 => {
                //keyboard
                println!("unsupported opcode");
            },
            0xf000 => {
                match opcode & 0x00ff {
                    0x0007 => {
                        self.register[((opcode & 0x0f00) >> 8) as usize] = self.delay_timer;
                    },
                    0x000a => {
                        //keyboard
                        println!("unsupported opcode");
                    },
                    0x0015 => {
                        self.delay_timer = self.register[((opcode & 0x0f00) >> 8) as usize];
                    },
                    0x0018 => {
                        self.sound_timer = self.register[((opcode & 0x0f00) >> 8) as usize];
                    },
                    0x001e => {
                        self.instruction += self.register[((opcode & 0x0f00) >> 8) as usize] as u16;
                    },
                    0x0029 => {
                        //display
                        println!("unsupported opcode");
                    },
                    0x0033 => {
                        println!("unsupported opcode");
                    },
                    0x0055 => {
                        println!("unsupported opcode");
                    },
                    0x0065 => {
                        println!("unsupported opcode");
                    },
                    _ => {
                        println!("unsupported opcode");
                    }
                }
            }
            _ => {
                println!("unsupported opcode");
            }

        }

    }
}

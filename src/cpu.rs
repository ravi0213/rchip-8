extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;

use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use multiarray::*;
use graphics::*;

pub struct CPU {
    register: [u8; 16],
    instruction: u16,
    delay_timer:u8,
    sound_timer:u8,
    pc:u16, //program counter
    stack_pointer:u8,
    stack:[u16;16],
    opengl:OpenGL,
    gl: GlGraphics,
    data: Vec<u8>,
    grid: Array2D<i32>,
}

impl CPU {

    pub fn new(file: std::vec::Vec<u8>, open_gl: OpenGL) -> CPU {

        let display_grid = Array2D::new([64,32],0);

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
            pc: 0x200,
            stack_pointer:0,
            stack:[0;16],
            opengl: open_gl,
            gl: GlGraphics::new(open_gl),
            data: file,
            grid: display_grid
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {


        const BLACK:[f32;4] = [0.0, 0.0, 0.0,1.0];
        const WHITE:[f32;4] = [1.0,1.0,1.0,1.0];

        let square = rectangle::square(0.0,0.0,8.0);
        let temp_grid = &self.grid;
        self.gl.draw(args.viewport(), |c,gl| {
            clear(BLACK, gl) ;

            for i in 0..64 {
                for j in 0..32 {
                    let transform = c.transform.trans(i as f64,j as f64);
                    if temp_grid[[i as usize, j as usize]] == 1 {
                        rectangle(WHITE, square, transform, gl);
                    }
                }
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        println!("update pc {}", self.pc);
        self.parse_operation_code((self.data[self.pc as usize] as u16) <<8 | (self.data[(self.pc+1) as usize ] as u16));
    }

    fn parse_operation_code(&mut self, opcode: u16) {
        println!("current opcode {:#x?}", opcode);
        match opcode & 0xf000 {
//            0x0000 => {
//                println!("step 0");
//                self.pc = (opcode & 0x0fff)*2;
//            },
            0x0000 => {
                match opcode & 0x000f {
                    0x0000 => {
                        println!("step 0x00e0 clear screen");
                        for i in 0..64 {
                            for j in 0..32 {
                                self.grid[[i,j]] = 0;
                            }
                        }
                    },
                    0x000e => {
                        println!("step 0x00ee return subroutine");
                        self.pc = self.stack[self.stack_pointer as usize];
                        self.stack_pointer -= 1;
                    },
                    _ => {
                        println!("unsupported op 0x0000");
                    }
                }
            },
            0x1000 => {
                println!("step 0x1nnn jump to nnn");
                self.pc = (opcode & 0x0fff);
            },
            0x2000 => {
                println!("step 0x2nnn call addr nnn");
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.pc;
                self.pc = (opcode & 0x0fff);
            },
            0x3000 => {
                println!("step 0x3xkk if Vx==kk then increment pc+=4");
                if self.register[((opcode & 0x0f00) >> 8) as usize] == (opcode & 0x00ff) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4000 => {
                println!("step 0x4xkk if Vx!=kk then increment pc+=4");
                if self.register[((opcode & 0x0f00) >> 8) as usize] != (opcode & 0x00ff) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x5000 => {
                println!("step 0x5xy0 if Vx==Vy then increment pc+=4");
                if self.register[((opcode & 0x0f00) >> 8) as usize] == self.register[((opcode & 0x00f0) >> 4) as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6000 => {
                println!("step 0x6xkk set Vx=kk");
                self.register[((opcode & 0x0f00) >> 8) as usize] = (opcode & 0x00ff) as u8;
                self.pc += 2;
            },
            0x7000 => {
                println!("step 0x7xkk set Vx+=kk");
                self.register[((opcode & 0x0f00) >> 8) as usize] += (opcode & 0x00ff) as u8;
                self.pc += 2;
            },
            0x8000 => {
                match opcode & 0x000f {
                    0x0000 => {
                        println!("step 0x8xy0 set Vx=Vy");
                        self.register[((opcode & 0x0f00) >> 8) as usize] = self.register[((opcode & 0x00f0) >> 4) as usize];
                        self.pc += 2;
                    },
                    0x0001 => {
                        println!("step 0x8xy1 set Vx = Vx OR Vy");
                        self.register[((opcode & 0x0f00) >> 8) as usize] |= self.register[((opcode & 0x00f0) >> 4) as usize];
                        self.pc += 2;
                    },
                    0x0002 => {
                        println!("step 0x8xy2 set Vx = Vx AND Vy");
                        self.register[((opcode & 0x0f00) >> 8) as usize] &= self.register[((opcode & 0x00f0) >> 4) as usize];
                        self.pc += 2;
                    },
                    0x0003 => {
                        println!("step 0x8xy3 set Vx = Vx XOR Vy");
                        self.register[((opcode & 0x0f00) >> 8) as usize] ^= self.register[((opcode & 0x00f0) >> 4) as usize];
                        self.pc += 2;
                    },
                    0x0004 => {
                        println!("step 0x8xy4 set Vx = Vx + Vy");
                        self.register[0x000f as usize] = if (self.register[((opcode & 0x0f00) >> 8) as usize] as u16 + self.register[((opcode & 0x00f0) >> 4) as usize] as u16) > 0x00ff {
                            0x0001
                        } else {
                            0x0000
                        };
                        self.register[((opcode & 0x0f00) >> 8) as usize] += self.register[((opcode & 0x00f0) >> 4) as usize];
                        self.pc += 2;
                    },
                    0x0005 => {
                        println!("step 0x8xy5 set Vx = Vx - Vy");
                        self.register[0x000f] = if self.register[((opcode & 0x0f00) >> 8) as usize] > self.register[((opcode & 0x00f0) >> 4) as usize] {
                            0x0001
                        } else {
                            0x0000
                        };
                        self.register[((opcode & 0x0f00) >> 8) as usize] -= self.register[((opcode & 0x00f0) >> 4) as usize];
                        self.pc += 2;
                    },
                    0x0006 => {
                        println!("step 0x8xy6 set Vx = Vx >> 1");
                        self.register[0x000f] = self.register[((opcode & 0x0f00) >> 8) as usize] & 0x0001;
                        self.register[((opcode & 0x0f00) >> 8) as usize] /= 0x0002;
                        self.pc += 2;
                    },
                    0x0007 => {
                        println!("step 0x8xy7 set Vx = Vy - Vx");
                        self.register[0x000f] = if self.register[((opcode & 0x0f00) >> 8) as usize] < self.register[((opcode & 0x00f0) >> 4) as usize] {
                            0x0001
                        } else {
                            0x0000
                        };
                        self.register[((opcode & 0x0f00) >> 8) as usize] = self.register[((opcode & 0x00f0) >> 4) as usize] - self.register[((opcode & 0x0f00) >> 8) as usize];
                        self.pc += 2;
                    },
                    0x000e => {
                        println!("step 0x8xyE set Vx = Vx << 1");
                        self.register[0x000f] = self.register[((opcode & 0x0f00) >> 8) as usize] & ((0x0e00 >> 8) as u8);
                        self.register[(0x0f00 >> 8) as usize] *= 0x0002;
                        self.pc += 2;
                    }
                    _ => {
                        println!("unsupported opcode");
                    }
                }
            },
            0x9000 => {
                println!("step 9");
                if self.register[(0x0f00 >> 8) as usize] != self.register[(0x00f0 >> 4) as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0xa000 => {

                println!("step a");
                self.instruction = opcode & 0x0fff;
                self.pc += 2;
            },
            0xb000 => {
                println!("step b");
                self.pc = (opcode & 0x0fff) + (self.register[0x0000] as u16) *2;
            },
            0xc000 => {
                println!("step c");
                self.register[(opcode & 0x0f00) as usize] = (opcode & 0x00ff) as u8;
                self.pc += 2;
            },
            0xd000 => {
                //display
                println!("step d");
                let number_of_bytes = opcode & 0x000f;
                let  x = opcode & 0x0f00;
                let  y = opcode & 0x00f0;
                self.register[15] = 0;
                for xi in 0..number_of_bytes {
                    let mut yi = 0;
                    while yi < 8 {
                        let prev = self.grid[[((x+xi)%64) as usize,((y+yi)%32) as usize]];
                        self.grid[[((x+xi)%64) as usize, ((y+yi)%32) as usize]] = self.grid[[((x+xi)%64) as usize,((y+yi)%32) as usize]] ^ ((1<<(7-yi)) & self.instruction) as i32;
                        if prev == 1 && self.grid[[((x+xi)%64) as usize,((y+yi)%32) as usize]] == 0 {
                            self.register[15] = 1;
                        }
                        yi = yi+1;
                    }
                    self.instruction = self.instruction+1;
                }
                self.pc += 2;
            },
            0xe000 => {
                //keyboard
                println!("unsupported opcode");
                self.pc += 2;
            },
            0xf000 => {
                match opcode & 0x00ff {
                    0x0007 => {
                        println!("step f 7");
                        self.register[((opcode & 0x0f00) >> 8) as usize] = self.delay_timer;
                        self.pc += 2;
                    },
                    0x000a => {
                        //keyboard
                        println!("unsupported opcode");
                        //self.pc += 2;
                    },
                    0x0015 => {
                        println!("step f 15");
                        self.delay_timer = self.register[((opcode & 0x0f00) >> 8) as usize];
                        self.pc += 2;
                    },
                    0x0018 => {
                        println!("step f 18");
                        self.sound_timer = self.register[((opcode & 0x0f00) >> 8) as usize];
                        self.pc += 2;
                    },
                    0x001e => {
                        println!("step f 1e");
                        self.instruction += self.register[((opcode & 0x0f00) >> 8) as usize] as u16;
                        self.pc += 2;
                    },
                    0x0029 => {
                        //display
                        println!("not tested opcode");
                        self.instruction = self.register[((opcode & 0x0f00) >> 8) as usize] as u16;
                        self.pc += 2;
                    },
                    0x0033 => {
                        println!("unsupported opcode");
                        self.pc += 2;
                    },
                    0x0055 => {
                        let count = (opcode & 0x0f00) >> 8 as usize;
//                        let mut i = 0;
//                        for x in self.data.iter_mut() {
//                            if i < self.instruction || i > self.instruction+count {
//                                *x = self.register[(i-self.instruction) as usize];
//                            }
//                            i += 1;
//                        }
                        for i in 0..count {
                            self.data[(self.instruction + i) as usize] = self.register[i as usize] ;
                        }
                        self.pc += 2;
                    },
                    0x0065 => {
                        let count = (opcode & 0x0f00) >> 8 as usize;
                        for i in 0..count {
                            self.register[i as usize] = self.data[(self.instruction + i) as usize];
                        }
                        self.pc += 2;
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

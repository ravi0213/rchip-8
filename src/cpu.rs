
struct cpu {
    register: [i8; 16],
    instruction: i16,
    delay_timer:i8,
    sound_timer:i8,
    pc:i16, //program counter
    stack_pointer:i8,
    stack:[i16;16]
}

impl cpu {
    fn parseOperationCode(&mut self, opcode: i16) {
        match opcode & 0xf000 {
            0x0000 => {
                self.pc = opcode & 0x0fff;
            },
            0x1000 => {
                self.pc = opcode & 0x0fff;
            },
            0x2000 => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.pc;
                self.pc = opcode & 0x0fff;
            },
            0x3000 => {
                if self.register[opcode & 0x0f00] == (opcode & 0x00ff) {
                    self.pc += 2;
                }
            },
            0x4000 => {
                if self.register[opcode & 0x0f00] != (opcode & 0x00ff) {
                    self.pc += 2;
                }
            },
            0x5000 => {
                if self.register[opcode & 0x0f00] == self.register[opcode & 0x00f0] {
                    self.pc += 2;
                }
            },
            0x6000 => {
                self.register[opcode & 0x0f00] = (opcode & 0x00ff);
            },
            0x7000 => {
                self.register[opcode & 0x0f00] += (opcode & 0x00ff);
            },
            0x8000 => {
                match opcode & 0x000f {
                    0x0000 => {
                        self.register[opcode & 0x0f00] = self.register[opcode & 0x00f0];
                    },
                    0x0001 => {
                        self.register[opcode & 0x0f00] |= self.register[opcode & 0x00f0];
                    },
                    0x0002 => {
                        self.register[opcode & 0x0f00] &= self.register[opcode & 0x00f0];
                    },
                    0x0003 => {
                        self.register[opcode & 0x0f00] ^= self.register[opcode & 0x00f0];
                    },
                    0x0004 => {
                        self.register[0x000f] = if (self.register[opcode & 0x0f00] as i16 + self.register[opcode & 0x00f0] as i16) > 0x00ff {
                            0x0001
                        } else {
                            0x0000
                        };
                        self.register[opcode & 0x0f00] += self.register[opcode & 0x00f0];

                    },
                    0x0005 => {
                        self.register[0x000f] = if (self.register[opcode & 0x0f00] > self.register[opcode & 0x00f0]){
                            0x0001
                        } else {
                            0x0000
                        };
                        self.register[opcode & 0x0f00] -= self.register[opcode & 0x00f0];

                    },
                    0x0006 => {
                        self.register[0x000f] = self.register[opcode & 0x0f00] & 0x0001;
                        self.register[opcode & 0x0f00] /= 0x0002;
                    },
                    0x0007 => {
                        self.register[0x000f] = if (self.register[opcode & 0x0f00] < self.register[opcode & 0x00f0]){
                            0x0001
                        } else {
                            0x0000
                        };
                        self.register[opcode & 0x0f00] = self.register[opcode & 0x00f0] - self.register[opcode & 0x0f00];
                    },
                    0x000e => {
                        self.register[0x000f] = self.register[opcode & 0x0f00] & 0x0e00
                        self.register[0x0f00] *= 0x0002;
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
                self.pc = (opcode & 0x0fff) + self.register[0x0000];
            },
            0xc000 => {
                self.register[opcode & 0x0f00] = opcode & 0x00ff;
            },
            0xd000 => {
                //display
            },
            0xe000 => {
                //keyboard
            },
            0xf000 => {
                match opcode & 0x00ff {
                    0x0007 => {
                        self.register[0x0f00 & opcode] = self.delay_timer;
                    },
                    0x000a => {
                        //keyboard
                    },
                    0x0015 => {
                        self.delay_timer = self.register[0x0f00 & opcode];
                    },
                    0x0018 => {
                        self.sound_timer = self.register[0x0f00 & opcode];
                    },
                    0x001e => {
                        self.instruction += self.register[0x0f00 & opcode];
                    },
                    0x0029 => {
                        //display
                    },
                    0x0033 => {

                    },
                    0x0055 => {

                    },
                    0x0065 => {

                    }
                }
            }

        }

    }
}

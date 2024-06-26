use std::collections::VecDeque;
use std::ops::{Not, Sub};
use rand::{self, Rng};
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use device_query::{DeviceQuery, DeviceState, Keycode};


fn main() {    
    println!("Hello, world!");
    let mut cpu = CPU::init();

    cpu.start("./SpaceInvaders.ch8".to_owned());

    cpu.printScreen();
}


fn get_xbytes(x:u8,opcode:u16) -> u16 {
    match x {
        1  => return (opcode & 0xF000)>>12,
        2  => return (opcode & 0x0F00)>>8,
        3  => return (opcode & 0x00F0)>>4,
        4  => return  opcode & 0x000F,
        12 => return (opcode & 0xFF00)>>8,
        23 => return (opcode & 0x0FF0)>>4,
        34 => return  opcode & 0x00FF,
        123=> return  opcode & 0xFFF0,
        234=> return  opcode & 0x0FFF,
        _  => return  0x00,
    }
}

struct CPU{
    v0:u8,
    v1:u8,
    v2:u8,
    v3:u8,
    v4:u8,
    v5:u8,
    v6:u8,
    v7:u8,
    v8:u8,
    v9:u8,
    vA:u8,
    vB:u8,
    vC:u8,
    vD:u8,
    vE:u8,
    vF:u8,
    I:u16,
    PC:u16,
    SP:u8,
    DT:u8,
    ST:u8,
    keys:[bool;16],
    STACK:[u16;16],
    RAM:[u8; 4095],
    SCREEN:[[bool;64];32]
}

impl CPU {

    const FONT: [u8;80] =
            [
                    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                    0x20, 0x60, 0x20, 0x20, 0x70, // 1
                    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
                    ];

    fn init() -> CPU{
        let mut cpu = CPU { v0: 0, v1: 0, v2: 0, v3: 0, v4: 0, v5: 0, v6: 0, v7: 0, v8: 0, v9: 0, vA: 0, vB: 0, vC: 0, vD: 0, vE: 0, vF: 0, I: 0, PC: 0, SP: 0, STACK: [0;16], RAM: [0;4095], SCREEN: [[false;64];32], DT: 0 ,ST: 0, keys:[false;16] };
        for i in 0x50..0xA0 {
            cpu.RAM[i] = CPU::FONT[i-0x50];
        }
        return cpu;
    }

    fn start(&mut self,path:String) {
        let mut device_state = DeviceState::new();
        let cpu = Self::init();
        let file = File::open(path);
        let mut contents: Vec<u8> = Vec::new();
        let _ = file.unwrap().read_to_end(&mut contents);

    if contents.len() % 2 != 0 {
        contents.push(0);
    }

        let result: Vec<u16> = contents.chunks(2)
            .map(|chunk| u16::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        for (i,el) in contents.iter().enumerate() {    
            self.RAM[0x200+i] = *el;
        }
        self.PC = 0x200;

        while true {

            self.keys = [false;16];
            device_state = DeviceState::new();
            let keys: Vec<Keycode> = device_state.get_keys();
            for key in keys.iter() {
                self.keys[self.keyToIndex(*key)] = true;
                println!("Pressed key: {:?}", key);
            }

            let nextop = (self.RAM[self.PC as usize] as u16) << 8 | (self.RAM[(self.PC + 1) as usize] as u16);
            println!("opcode : {:#04x} from RAM[{:#04x}]",nextop,self.PC);
            self.printScreen();
            thread::sleep(time::Duration::from_millis(6));
            self.execute(nextop);
            self.PC +=2;
            if self.DT > 0 {
                self.DT -=1;
            }
        }

    }

    fn fetch(&mut self) -> u8 {
        return self.RAM[self.PC as usize];
    }

    fn getXreg(&mut self,x:u8) -> &u8 {
        match x {
            0x0=>return &self.v0,
            0x1=>return &self.v1,
            0x2=>return &self.v2,
            0x3=>return &self.v3,
            0x4=>return &self.v4,
            0x5=>return &self.v5,
            0x6=>return &self.v6,
            0x7=>return &self.v7,
            0x8=>return &self.v8,
            0x9=>return &self.v9,
            0xA=>return &self.vA,
            0xB=>return &self.vB,
            0xC=>return &self.vC,
            0xD=>return &self.vD,
            0xE=>return &self.vE,
            0xF=>return &self.vF,
            _=>return &self.vF
        }
    }

    fn setXreg(&mut self,x:u8,value:u8) {
        match x {
            0x0=>self.v0 = value,
            0x1=>self.v1 = value,
            0x2=>self.v2 = value,
            0x3=>self.v3 = value,
            0x4=>self.v4 = value,
            0x5=>self.v5 = value,
            0x6=>self.v6 = value,
            0x7=>self.v7 = value,
            0x8=>self.v8 = value,
            0x9=>self.v9 = value,
            0xA=>self.vA = value,
            0xB=>self.vB = value,
            0xC=>self.vC = value,
            0xD=>self.vD = value,
            0xE=>self.vE = value,
            0xF=>self.vF = value,
            _=>print!("")
        }
    }

    fn registersList(&self) -> [&u8; 16] {
        let ret: [&u8; 16] = [
        &self.v0,
        &self.v1,
        &self.v2,
        &self.v3,
        &self.v4,
        &self.v5,
        &self.v6,
        &self.v7,
        &self.v8,
        &self.v9,
        &self.vA,
        &self.vB,
        &self.vC,
        &self.vD,
        &self.vE,
        &self.vF,
        ];

        return ret;
    }

    fn keyToIndex(&self,key:Keycode) -> usize {

        match key {
            Keycode::Key1=>{return 1},
            Keycode::Key2=>{return 2},
            Keycode::Key3=>{return 3},
            Keycode::Key4=>{return 0xC},

            Keycode::A=>   {return 4},
            Keycode::Z=>   {return 5},
            Keycode::E=>   {return 6},
            Keycode::R=>   {return 0xD},

            Keycode::Q=>   {return 7},
            Keycode::S=>   {return 8},
            Keycode::D=>   {return 9},
            Keycode::F=>   {return 0xE},

            Keycode::W=>   {return 0xA},
            Keycode::X=>   {return 0},
            Keycode::C=>   {return 0xB},
            Keycode::V=>   {return 0xF},
            _=>return 0xFF
        }

    }
    
    fn execute(&mut self,opcode:u16) {
        //i know its a weird way to parse opcodes but it works so
        let mut instruction = get_xbytes(1, opcode);
        let x2   = get_xbytes(2, opcode)  as u8;
        let x3   = get_xbytes(3, opcode)  as u8;
        let x4   = get_xbytes(4, opcode)  as u8;
        let x12  = get_xbytes(12, opcode) as u8;
        let x23  = get_xbytes(23, opcode) as u8;
        let x34  = get_xbytes(34, opcode) as u8;
        let x123 = get_xbytes(123, opcode);
        let x234 = get_xbytes(234, opcode);

        match instruction{
             0=>{
                if x3 == 0xE {
                    if x4 == 0x0 {
                        self.SCREEN = [[false;64];32];
                    }
                    else {
                        self.SP -=1;
                        self.PC = self.STACK[(self.SP) as usize];
                        //self.PC -=2;
                    }
                }
                else {
                    //SYS addr 0nnn
                }
             },
             1=> { 
                //JP addr 1nnn
                self.PC = x234;
                self.PC -=2;

            },
             2=>{
                //CALL addr 2nnn
                 self.STACK[self.SP as usize] = self.PC;
                 self.PC = x234;
                 self.PC -=2;
                 self.SP +=1;
             }        
             3=>{
                //SE Vx, byte 3xkk
                if *self.getXreg(x2) == x34 {
                    self.PC +=2
                } 
             },
             4=>{
                //SNE Vx, byte 4xkk
                if *self.getXreg(x2) != x34 {
                    self.PC +=2
                } 
             },
             5=>{
                //SE Vx, Vy 5xy0
                if *self.getXreg(x2) == *self.getXreg(x3) {
                    self.PC +=2
                } 
             },
             6=>{
                //LD Vx, byte 6xkk
                self.setXreg(x2,x34);
             },
             7=>{
                //ADD Vx, byte 7xkk
                let value = (*self.getXreg(x2)).wrapping_add(x34);
                self.setXreg(x2, value);
             }
             8=>{
                instruction = get_xbytes(4, opcode);
                let yvalue = *self.getXreg(x3);
                let xvalue = *self.getXreg(x2);
                
                match instruction {
                    0=>{
                        self.setXreg(x2, yvalue);
                    },
                    1=>{
                        self.setXreg(x2, xvalue | yvalue);
                    },
                    2=>{
                        self.setXreg(x2, xvalue & yvalue);
                    },
                    3=>{
                        self.setXreg(x2, xvalue ^ yvalue);
                    },
                    4=>{
                        let(res,over) = xvalue.overflowing_add(yvalue);
                        if over {
                            self.setXreg(0xF, 0x1)
                        }
                        self.setXreg(x2, res);
                    },
                    5=>{
                        let(res,over) = xvalue.overflowing_sub(yvalue);
                        if over {
                            self.setXreg(0xF, 0x0)
                        }else {
                            self.setXreg(0xF, 0x1)
                        }
                        self.setXreg(x2, res);
                    },
                    6=>{
                        self.setXreg(0xF, yvalue & 0b0000_0001);
                        self.setXreg(x2, yvalue>>1);
                    },
                    7=>{
                        let(res,over) = yvalue.overflowing_sub(xvalue);
                        if over {
                            self.setXreg(0xF, 0x0)
                        }else {
                            self.setXreg(0xF, 0x1)
                        }
                        self.setXreg(x2, res);
                    },
                    0xE=>{
                        self.setXreg(0xF, yvalue & 0b1000_0000);
                        self.setXreg(x2, yvalue<<1);
                    },
                    _=>print!("Unknown instruction")
                    }

                },
             9=>{
                //SNE Vx, Vy 9xy0
                if *self.getXreg(x2) != *self.getXreg(x3) {
                        self.PC +=2
                } 
             },
             0xA=>{
                //LD I, addr Annn
                self.I = x234;
             },
             0xB=>{
                //JP V0, addr Bnnn
                self.PC = *self.getXreg(0) as u16 + x234;
                self.PC -=2;
             },
             0xC=>{
                //RND Vx, byte Cxkk
                let mut rng = rand::thread_rng();
                self.setXreg(x2, rng.gen_range(0x00..0xFF) & x34);
             },
             0xD=>{
                //DRW Vx, Vy, nibble Dxyn
                for i in 0..x4 {
                    for j in 0..8 {
                        //okay so we have to XOR the screen[x][y] and I. so we take I, and it with 1<<7-j (to transforme it in 1 only bit on j pos) 
                        //then we check if it bigger than 0 to transform any bit into bool and then XOR it with screen[x][y]
                        //idk if it optimized but thats how i see this :O 
                        self.SCREEN[((*self.getXreg(x3)+i) % 32) as usize][((self.getXreg(x2) + j) % 64) as usize]
                         = self.SCREEN[((*self.getXreg(x3)+i) % 32) as usize][((self.getXreg(x2) + j) % 64) as usize] 
                         ^ ((self.RAM[(self.I + i as u16) as usize] & 1<<7-j) > 0);
                    }         
                }
            },
             0xE=>{
                    if (self.keys[(*self.getXreg(x2)% 0xF ) as usize] && x3 == 9) ||
                        (!self.keys[(*self.getXreg(x2)% 0xF ) as usize] && x3 != 9){
                        self.PC +=2;
                    }
             },
             0xF=>{
                match x34 {
                    7=>self.setXreg(x2, self.DT),
                    0xA=>{
                        //LD Vx, K Fx0A

                        let device_state = DeviceState::new();
                        let mut key = 0;
                        loop {
                            let keys = device_state.get_keys();
                            if !keys.is_empty() {
                                println!("Pressed key: {:?}", *keys.first().unwrap());
                                self.keys[self.keyToIndex(*keys.first().unwrap())] = true;
                                break;
                            }
                        }
                        self.setXreg(x2, key as u8);

                    },
                    0x15=>self.DT = *self.getXreg(x2),
                    0x18=>self.ST = *self.getXreg(x2),
                    0x1E=>self.I = self.I + *self.getXreg(x2) as u16,
                    0x29=>{
                        //LD F, Vx Fx29
                        self.I = (*self.getXreg(x2) as u16 * 5) + 0x50;
                    },
                    0x33=>{
                        //LD B, Vx Fx33
                        self.RAM[self.I as usize] = *self.getXreg(x2) / 100;
                        self.RAM[(self.I + 1) as usize] = (*self.getXreg(x2) / 10) % 10;
                        self.RAM[(self.I + 2) as usize] = *self.getXreg(x2) % 10;
                    },
                    0x55=>{ 
                        for i in 0..0xF {
                            self.RAM[(self.I + i as u16) as usize] = *self.getXreg(i);   
                        }
                    },
                    0x65=>{ 
                        for i in 0..0xF {
                            self.setXreg(i, self.RAM[(self.I + i as u16) as usize]);
                        }
                    },
                    _=>print!("tjrpas")
                }
                 
             }


                _=>print!("y'a rien")
            }
        }

        fn print_ERROR(&self) {
            println!("Erreur !!");
            println!("All registers : ");
            print!("V0 :{:#04x} ,", self.v0);
            print!("V1 :{:#04x} ,", self.v1);
            print!("V2 :{:#04x} ,", self.v2);
            print!("V3 :{:#04x} ,", self.v3);
            print!("V4 :{:#04x} ,", self.v4);
            print!("V5 :{:#04x} ,", self.v5);
            print!("V6 :{:#04x} ,", self.v6);
            print!("V7 :{:#04x} ,", self.v7);
            print!("V8 :{:#04x} ,", self.v8);
            print!("V9 :{:#04x} ,", self.v9);
            print!("VA :{:#04x} ,", self.vA);
            print!("VB :{:#04x} ,", self.vB);
            print!("VC :{:#04x} ,", self.vC);
            print!("VD :{:#04x} ,", self.vD);
            print!("VE :{:#04x} ,", self.vE);
            print!("VF :{:#04x} ,", self.vF);

            print!("I  :{:#04x} ,", self.I);
            print!("PC :{:#04x} ,", self.PC);
            print!("SP :{:#04x} ,", self.SP);
            print!("DT :{:#04x} ,", self.DT);
            print!("ST :{:#04x} ,", self.ST);


            print!("Keys : ");
            for i in self.keys {
                print!("{},", i);
            }
            print!("\n");

            print!("Stack : ");
            for i in self.STACK {
                print!("{},", i);
            }
            print!("\n");

            print!("RAM : ");
            for (i,el) in self.RAM.iter().enumerate() {
                print!("{:#04x},", el);
                if i % 0x10 == 0 {
                    print!("\n");
                    print!("{:#04x} : ", i);
                }
            }
        }

        fn printScreen(&mut self) {
            for i in self.SCREEN{
                print!("_");
                for _j in i {
                    print!("_");
                }
                print!("__");

                break;
            }
            print!("\n");

            for i in self.SCREEN {
                print!("｜");
                for j in i{
                    if j {
                        print!("*");
                    }else {
                        print!(" ");
                    }
                }
                print!("｜\n");
            }

            for i in self.SCREEN{
                print!("‾");
                for _j in i {
                    print!("‾");
                }
                print!("‾‾");
                break;
            }
            print!("\n\n");
        }
            
            
    }

//   struct Stack<T>{
//       stack:Vec<T>,
//       last:usize
//   }
//
//   impl<T> Stack<T> {
//       fn init() -> Stack<T>{
//           
//       }
//   }
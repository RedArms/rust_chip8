use std::{collections::VecDeque, ops::{BitAnd, Not}, ptr::null, sync::Arc};
use rand::{self, Rng};

fn main() {
    let RAM: [u16; 4095] = [0x00;0xFFF];
    let SCREEN : [[bool;31];63] = [[false;31];63];
    
    println!("Hello, world!");
    let mut cpu = CPU::init();
    println!("i before: {}", cpu.I);
    cpu.execute(0xAFFE);
    println!("i after: {}", cpu.I);


    println!("i before: {}", cpu.v2);
    cpu.execute(0xC20F);
    println!("i after: {}", cpu.v2);
}


fn getXBytes(x:u8,opcode:u16) -> u16 {
    match x {
        1=>return ((opcode & 0xF000)>>12),
        2=>return ((opcode & 0x0F00)>>8) ,
        3=>return ((opcode & 0x00F0)>>4) ,
        4=>return (opcode & 0x000F),
        12=>return ((opcode & 0xFF00)>>8),
        23=>return ((opcode & 0x0FF0)>>4),
        34=>return ((opcode & 0x00FF)),
        123=>return ((opcode & 0xFFF0)),
        234=>return ((opcode & 0x0FFF)),
        _=>return 0x00,
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
    STACK:VecDeque<u16>,
    RAM:[u16; 4095],
    SCREEN:[[bool;31];63]
}

impl CPU {

    fn init() -> CPU{
        let cpu = CPU { v0: 0, v1: 0, v2: 0, v3: 0, v4: 0, v5: 0, v6: 0, v7: 0, v8: 0, v9: 0, vA: 0, vB: 0, vC: 0, vD: 0, vE: 0, vF: 0, I: 0, PC: 0, SP: 0, STACK: VecDeque::new(), RAM: [0;4095], SCREEN: [[false;31];63] };
        return cpu;
    }

    fn fetch(&mut self) -> u16 {
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
    

    fn execute(&mut self,opcode:u16) {
        let mut instruction = getXBytes(1, opcode);
        let x1   = getXBytes(1, opcode)  as u8;
        let x2   = getXBytes(2, opcode)  as u8;
        let x3   = getXBytes(3, opcode)  as u8;
        let x4   = getXBytes(4, opcode)  as u8;
        let x12  = getXBytes(12, opcode) as u8;
        let x23  = getXBytes(23, opcode) as u8;
        let x34  = getXBytes(34, opcode) as u8;
        let x123 = getXBytes(123, opcode);
        let x234 = getXBytes(234, opcode);

        match instruction{
             0=>{
                if x3 == 0xE {
                    if x4 == 0x0 {
                        self.SCREEN = [[false;31];63];
                    }
                    else {
                        self.PC = self.STACK.pop_front().unwrap();
                        self.SP-=1;
                    }
                }
                else {
                    //SYS addr 0nnn
                }
             },
             2=>{
                 self.STACK.push_front(self.PC);
                 self.PC = x234;
             }
             1=>self.PC = opcode & 0b0000_1111_1111_1111,
             
             3=>{
                if *self.getXreg(x2) == x34 {
                    self.PC +=1
                } 
             },
             4=>{
                if *self.getXreg(x2) != x34 {
                    self.PC +=1
                } 
             },
             5=>{
                if *self.getXreg(x2) == *self.getXreg(x3) {
                    self.PC +=1
                } 
             },
             6=>{
                self.setXreg(x2,x34);
             },
             7=>{
                let xvalue = *self.getXreg(x2) as u8;
                self.setXreg(x2, xvalue + x34);
             }
             8=>{
                instruction = getXBytes(4, opcode);
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
                    _=>print!("he")
                    }

                },
             9=>{
                if *self.getXreg(x2) != *self.getXreg(x3) {
                        self.PC +=1
                } 
             },
             0xA=>{
                self.I = x234;
             },
             0xB=>{
                self.PC = *self.getXreg(0) as u16 + x234;
             },
             0xC=>{
                let mut rng = rand::thread_rng();
                self.setXreg(x2, rng.gen_range(0x00..0xFF) & x34);
             }

                _=>print!("y'a rien")
            }
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
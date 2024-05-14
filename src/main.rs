use std::{ops::{BitAnd, Not}, ptr::null};


fn main() {
    let RAM: [u16; 4095] = [0x00;0xFFF];
    let SCREEN : [[bool;31];63] = [[false;31];63];

    println!("Hello, world!");
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
    STACK:[u16; 16],
    RAM:[u16; 4095],
    SCREEN:[[bool;31];63]
}

impl CPU {

    fn getXreg(&mut self,x:u8) -> &u8 {
        match x {
            0=>return &self.v0,
            1=>return &self.v1,
            2=>return &self.v2,
            3=>return &self.v3,
            4=>return &self.v4,
            5=>return &self.v5,
            6=>return &self.v6,
            7=>return &self.v7,
            8=>return &self.v8,
            9=>return &self.v9,
            0xA=>return &self.vA,
            0xB=>return &self.vB,
            0xC=>return &self.vC,
            0xD=>return &self.vD,
            0xE=>return &self.vE,
            0xF=>return &self.vF,
            _=>return &self.vF
        }
    }

    fn getXBytes(&mut self,x:u8,opcode:u16) -> u8 {
        match x {
            1=>return ((opcode & 0xF000)>>12).try_into().unwrap(),
            2=>return ((opcode & 0x0F00)>>8).try_into().unwrap(),
            3=>return ((opcode & 0x00F0)>>4).try_into().unwrap(),
            4=>return (opcode & 0x000F).try_into().unwrap(),
            _=>return 0x00,
        }
    }
    fn execute(&mut self,opcode:u16) {
        self.v1 = 0;

        match opcode>>12{

            1=>self.PC = opcode & 0b0000_1111_1111_1111, //0x1NNN & 0x0FFFF => 0xNNN
            2=>print!(""),
            3=>{
                if *self.getXreg(3) == (opcode & 0x00FF) as u8 {
                    
                }
            }
            
            _ => print!("ya rien ")
        }
    }



    
}
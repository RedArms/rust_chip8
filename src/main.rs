use std::{ops::{BitAnd, Not}, ptr::null};


fn main() {
    let RAM: [u16; 4095] = [0x00;0xFFF];
    let SCREEN : [[bool;31];63] = [[false;31];63];

    println!("Hello, world!");
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

    fn setXreg(&mut self,x:u8,value:u8) {
        match x {
            0=>self.v0  = value,
            1=>self.v1  = value,
            2=>self.v2  = value,
            3=>self.v3  = value,
            4=>self.v4  = value,
            5=>self.v5  = value,
            6=>self.v6  = value,
            7=>self.v7  = value,
            8=>self.v8  = value,
            9=>self.v9  = value,
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
             1=>self.PC = opcode & 0b0000_1111_1111_1111,
             6=>{
                self.setXreg(x2,x34);
             },
             7=>{
                let xvalue = *self.getXreg(x2) as u8;
                self.setXreg(x2, xvalue + x34);
             }
             8=>{
                instruction = getXBytes(4, opcode);
                match instruction {
                    0=>{
                        let yvalue = *self.getXreg(x3);
                        self.setXreg(x2, yvalue);
                    },
                    1=>{

                    }

                    _=>print!("y'a rien")
                }
             }
            

            //0x0001=>self.PC = opcode & 0b0000_1111_1111_1111, //0x1NNN & 0x0FFFF => 0xNNN
            //0x0002=>print!(""),
            //0x0003=>{
            //},
            //0x0005=>{
//
            //},
            //0x0006=>{
            //    self.setXreg(self.getXBytes(2, opcode), (opcode & 0b0000_0000_1111_1111) as u8)
            //}
            //
            _ => print!("ya rien ")
        }
    }



    
}
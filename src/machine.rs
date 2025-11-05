use crate::instruction::{Address, Data8, Data16, Register8, Register16};

static MEMORY_SIZE_BYTES: usize = 2 << 16;
pub struct Memory([u8; MEMORY_SIZE_BYTES]);

impl Memory {
    pub fn new() -> Self {
        Self([0; MEMORY_SIZE_BYTES])
    }

    pub fn read_u8(&self, address: Address) -> Data8 {
        self.0[address.value() as usize]
    }
    pub fn read_u16(&self, address: Address) -> Option<Data16> {
        let low = self.0[address.value() as usize];
        let high = *self.0.get(address.value() as usize + 1)?;
        Some(Data16::new(low, high))
    }

    pub fn as_raw(&self) -> &[u8; MEMORY_SIZE_BYTES] {
        &self.0
    }
}

// Struct containing program addressable registers.
pub struct RegisterMap {
    a: Data8,
    b: Data8,
    c: Data8,
    d: Data8,
    e: Data8,
    h: Data8,
    l: Data8,
    sp: Data16,
}

impl RegisterMap {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: Data16::ZERO,
        }
    }

    pub fn get_8(&self, register: Register8, memory: &Memory) -> Data8 {
        match register {
            Register8::B => {
                return self.b;
            }
            Register8::C => {
                return self.c;
            }
            Register8::D => {
                return self.d;
            }
            Register8::E => {
                return self.e;
            }
            Register8::H => {
                return self.h;
            }
            Register8::L => {
                return self.l;
            }
            Register8::M => {
                let address = self.get_16(Register16::Hl);

                return memory.read_u8(address);
            }
            Register8::A => {
                return self.a;
            }
        }
    }

    pub fn get_16(&self, register: Register16) -> Data16 {
        match register {
            Register16::Bc => Data16::new(self.c, self.b),
            Register16::De => Data16::new(self.e, self.d),
            Register16::Hl => Data16::new(self.l, self.h),
            Register16::Sp => self.sp,
        }
    }
}

pub struct Machine {
    memory: Box<Memory>,
    registers: RegisterMap,
    pc: Data16,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            memory: Box::new(Memory::new()),
            registers: RegisterMap::new(),
            pc: Data16::ZERO,
        }
    }

    pub fn registers(&self) -> &RegisterMap {
        &self.registers
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn register_8(&self, register: Register8) -> Data8 {
        self.registers().get_8(register, self.memory())
    }

    pub fn register_16(&self, register: Register16) -> Data16 {
        self.registers().get_16(register)
    }

    pub fn pc(&self) -> Data16 {
        self.pc
    }
}

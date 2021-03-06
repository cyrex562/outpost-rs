use crate::win_struct::{HICON16, HMENU16};

pub type U32Ptr = u32;

pub struct StructA {
    pub field_0x4: u16,
    pub field_0x16: u16,
    pub field_0x36: u32,
    pub field_0x3e: u16,
    pub field_0x40: u16,
    pub field_0xa: Struct13_1,
}

pub struct Struct1 {}

pub struct Struct13_1 {
    pub field_0x0: u16,
    pub field_0x4: u16,
    pub field_0x6: u16,
    pub field_0xa: u16,
}

pub struct Struct_1050_0012 {
    pub field_0x0: String,
    pub field_0x2: u16,
    pub field_0xc: u32,
}

pub struct StructB {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0xf8: Struct18,
}

impl StructA {
    pub fn new() -> StructA {
        StructA {
            field_0x4: 0,
            field_0x16: 0,
            field_0x36: 0,
            field_0x3e: 0,
            field_0x40: 0,
            field_0xa: Struct13_1 {
                field_0x4: 0,
                field_0x6: 0,
                field_0xa: 0,
                field_0x0: 0,
            },
        }
    }
}

pub struct Struct18 {
    pub field_0x0: u16,
    pub field_0x1: u16,
    pub field_0x2: U32Ptr,
    pub field_0x6: i16,
    pub field_0x12: U32Ptr,
    pub field_0x16: u16,
    pub field_0x18: u16,
    pub field_0x2a: u16,
    pub field_0x8e: u16,
    pub field_0x92: U32Ptr,
    pub field_0x94: u16,
}

impl Struct18 {
    pub fn new() -> Struct18 {
        Struct18 {
            field_0x0: 0,
            field_0x1: 0,
            field_0x2: 0,
            field_0x6: 0,
            field_0x12: 0,
            field_0x16: 0,
            field_0x18: 0,
            field_0x2a: 0,
            field_0x8e: 0,
            field_0x92: 0,
            field_0x94: 0,
        }
    }
}

pub struct Struct10 {}

pub struct Struct27 {
    pub field_0x16: u16,
}

pub struct Struct65 {}

pub struct Struct11 {}

pub struct Struct13 {}

pub struct Struct28 {}

pub struct Struct194 {
    pub field_0xe: u16,
}

pub struct Struct29 {
    pub field_0x0: u16,
    pub field_0x188: Struct18,
}

pub struct Struct57 {
    pub field_0x0: u16,
    pub field_0x2: U32Ptr,
    pub field_0x8e: Option<Struct19>,
    pub field_0x94: u16,
    pub field_0x96: Option<Struct19>,
}

pub struct Struct19 {
    pub field_0x0: u16,
    pub field_0xa: i16,
    pub field_0xc: i16,
    pub field_0xe: u16,
    pub field_0x14: i16,
    pub field_0x16: i16,
    pub field_0x18: bool,
    pub field_0x1a: u16,
}

pub struct Struct20 {}

pub struct Struct76 {
    pub field_0x1e: u16,
    pub field_0x22: u16,
    pub field_0x24: u16,
    pub field_0x0: String,
    pub field_0x2: u16,
}

pub struct Struct79 {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0x4: u16,
    pub field_0x8: u16,
    pub field_0xa: u16,
    pub field_0xc: u16,
    pub field_0xe: u16,
    pub field_0x12: u16,
    pub field_0x14: u16,
    pub field_0x16: u16,
}

pub struct Struct80 {}

pub struct Struct449 {
    pub field_0x0: u16,
    pub field_0x2: u16,
}

pub struct Struct648 {}

pub struct Struct_1008_09ba {
    pub field_0xec: HMENU16,
}

pub struct Struct_1008_0a3c {
    pub field_0xde: bool,
}

pub struct Struct_1008_496c {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0x4: u16,
    pub field_0x6: u16,
    pub field_0x8: Struct18,
    pub field_0xc: u16,
    pub field_0x1a: Struct18,
}

pub struct Struct_1008_49e8 {
    pub field_0x4: u16,
    pub field_0x8: u16,
    pub field_0xc: u16,
    pub field_0xe: u16,
    pub field_0x1a: u16,
    pub field_0x1c: u16,
    pub field_0x1e: u16,
    pub field_0x12: u16,
}

pub struct Struct_1008_4cdc {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0x4: Struct18,
    pub field_0xe: Struct18,
    pub field_0x12: u16,
}

pub struct Struct_1008_4d26 {
    pub field_0x2: u16,
    pub field_0x4: u16,
    pub field_0xc: u16,
}

pub struct Struct_1008_4d84 {}

pub struct Struct_1000_05e2 {}

pub struct Struct_1040_8b92 {
    pub field_0x90: u8,
    pub field_0x8e: HICON16,
}

pub struct Struct17 {}

pub struct Struct161 {}

pub struct Struct37 {}

pub struct Struct99 {}

pub struct Struct110 {}

pub struct Struct_1000_2cb0 {
    pub field_0x0: u16,
    pub field_0x1: u16,
    pub field_0x2: u16,
    pub field_0x3: Struct18,
    pub field_0x4: u16,
    pub field_0x5: u16,
    pub field_0xb: u16,
    pub field_0x78: u16,
}

pub struct Struct_1000_34cf {
    pub field_0xe: Struct197,
}

pub struct Struct197 {
    pub field_0x2: u16,
}

pub struct Struct_1000_09ca {
    pub field_0x0: Struct_211,
    pub field_0x1: u16,
    pub field_0x2: u16,
    pub field_0x7: U32Ptr,
    pub field_0x8: Struct_160,
    pub field_0x9: u16,
    pub field_0xa: u16,
}

pub struct Struct_211 {}

pub struct Struct_160 {}

pub struct Struct_1000_0c32 {
    pub field_0xe: u16,
}

// pub struct Pointer<T> {}

// impl Pointer<T> {
//     pub fn new<T>() -> Pointer<T> {
//         Pointer::<T> {}
//     }
// }

pub struct Struct87 {
    pub field_0x680: i16,
    pub field_0x67c: u16,
    pub field_0x67e: Struct18,
}

pub struct Struct_1010_2fa0 {}

pub struct Struct_1008_628e {
    pub field_0x2: u16,
    pub field_0x8: u16,
    pub field_0xd2: Struct_257,
}

pub struct Struct_257 {
    pub field_0x14: U32Ptr,
}

pub struct Struct_1010_7b26 {}

pub struct Struct_1000_30a4 {}

pub struct Struct183 {}

pub struct Struct_1028_00cc {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0x20: u16,
    pub field_0x22: u16,
}

pub struct Struct_1028_0138 {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0x22: u16,
    pub field_0x24: u16,
}

pub struct Struct_1028_0176 {}

pub struct Struct_1040_0a1a {
    pub field_0x8e: Struct_311,
    pub field_0x70: u16,
    pub field_0x72: u16,
}

pub struct Struct_311 {
    pub field_0xa: u32,
    pub field_0xc: u16,
}

pub struct Struct_1040_98c0 {}

pub struct Struct_1010_4b3e {
    pub field_0x0: u16,
    pub field_0x2: u16,
    pub field_0x12: u16,
    pub field_0x2a: u16,
}

pub struct Struct_1008_4274 {
    pub field_0x6: u32,
}

pub struct Struct_1028_b514 {
    pub field_0x12: u16,
    pub field_0x18: u16,
    pub field_0x14: Option<Struct18>,
}

pub struct Struct_1010_4e08 {
    pub field_0x20: u16,
    pub field_0x22: u16,
    pub field_0x24: u16,
    pub field_0x30: u16,
}

pub struct Struct_1050_11ca {
    pub field_0x0: String,
    pub field_0x8: u16,
}

impl Struct_1050_11ca {
    pub fn new() -> Struct_1050_11ca {
        Struct_1050_11ca {
            field_0x0: "".to_string(),
            field_0x8: 0,
        }
    }
}

pub struct Struct_1008_9664 {
    pub field_0xa: String,
}

pub struct Struct30 {}

pub struct Struct43 {}

pub struct Struct3 {}

// pub struct Struct1{}

pub struct Struct34 {}

pub struct Struct31 {}

pub struct Struct35 {}

pub struct Struct23 {}

pub struct Struct5 {}

pub struct Struct4 {}

pub struct Struct8 {}

pub struct Struct9 {}

pub struct Struct58 {}

pub struct Struct36 {}

pub struct Struct24 {}

pub struct Struct16 {}

pub struct Struct64 {}

pub struct Struct7 {}

pub struct Struct6 {}

pub struct Struct14 {}

pub struct Struct15 {}

pub struct Struct71 {}

pub struct Struct199 {}

pub struct Struct200 {}

pub struct Struct229 {}

pub struct Struct228 {}

pub struct Struct729 {}

pub struct Struct45 {}

pub struct Struct44 {}

pub struct Struct368 {}

pub struct Struct369 {}

pub struct Struct370 {}

pub struct Struct430 {}

pub struct Struct381 {}

pub struct Struct380 {}

pub struct Struct387 {}

pub struct Struct389 {}

pub struct Struct391 {}

pub struct Struct388 {}

pub struct Struct390 {}

pub struct Struct307 {}

pub struct Struct169 {}

pub struct Struct171 {}

pub struct Struct450 {}

pub struct Struct452 {}

pub struct Struct451 {}

pub struct Struct456 {}

pub struct Struct160 {}

pub struct Struct649 {}

pub struct Struct83 {}

pub struct Struct109 {}

pub struct Struct111 {}

pub struct Struct651 {}

pub struct Struct457 {}

pub struct Struct458 {}

pub struct Struct459 {}

pub struct Struct362 {}

pub struct Struct460 {}

pub struct Struct461 {}

pub struct Struct86 {}

pub struct Struct462 {}

pub struct Struct112 {}

pub struct Struct463 {}

pub struct Struct107 {}

pub struct Struct108 {}

pub struct Struct106 {}

pub struct Struct464 {}

pub struct Struct134 {}

pub struct Struct465 {}

pub struct Struct195 {}

pub struct Struct196 {}

pub struct Struct198 {}

pub struct Struct208 {}

pub struct Struct21 {}

pub struct Struct647 {}

pub struct Struct732 {}

pub struct Struct471 {}

pub struct Struct210 {}

pub struct Struct211 {}

pub struct Struct215 {}

pub struct Struct214 {}

pub struct Struct216 {}

pub struct Struct213 {}

pub struct Struct102 {}

pub struct Struct103 {}

pub struct Struct219 {}

pub struct Struct218 {}

pub struct Struct645 {}

pub struct Struct707 {}

pub struct Struct225 {}

pub struct Struct226 {}

pub struct Struct227 {}

pub struct Struct239 {}

pub struct Struct234 {}

pub struct Struct238 {}

pub struct Struct455 {}

pub struct Struct241 {}

pub struct Struct242 {}

pub struct Struct498 {}

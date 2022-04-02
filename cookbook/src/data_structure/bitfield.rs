use bitflags::bitflags;
use std::fmt;

bitflags! {
    struct Myflags:u32 {
        const FLAG_A    = 0b00000001;
        const FLAG_B    = 0b00000010;
        const FLAG_C    = 0b00000100;
        const FLAG_ABC  = Self::FLAG_A.bits
                        | Self::FLAG_B.bits
                        |Self::FLAG_C.bits;
        const FLAG_D    = 0b00001000;
    }
}

impl Myflags {
    pub fn clear(&mut self) -> &mut Self {
        self.bits = 0;
        self
    }
}

impl fmt::Display for Myflags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits)
    }
}

pub fn bit_op() {
    let e1 = Myflags::FLAG_A | Myflags::FLAG_C;
    let e2 = Myflags::FLAG_B | Myflags::FLAG_C;
    assert_eq!((e1|e2), Myflags::FLAG_ABC);
    assert_eq!((e1&e2), Myflags::FLAG_C);
    assert_eq!((e1-e2), Myflags::FLAG_A);
    assert_eq!((e2-e1), Myflags::FLAG_B);
    assert_eq!((e1-Myflags::FLAG_ABC), Myflags::from_bits(0).unwrap());
    assert_eq!(!e2, Myflags::FLAG_A|Myflags::FLAG_D);
    assert_eq!(!e1, Myflags::FLAG_B|Myflags::FLAG_D);

    let mut flags = Myflags::FLAG_ABC;
    assert_eq!(format!("{}", flags), "00000000000000000000000000000111");
    assert_eq!(format!("{}", flags.clear()),"00000000000000000000000000000000");
    assert_eq!(format!("{:?}", Myflags::FLAG_A), "FLAG_A");
    assert_eq!(format!("{:?}", Myflags::FLAG_B|Myflags::FLAG_A), "FLAG_A | FLAG_B");
    assert_ne!(format!("{:?}", Myflags::FLAG_B|Myflags::FLAG_A), "FLAG_B | FLAG_A");
    assert_ne!(format!("{:?}", Myflags::FLAG_B|Myflags::FLAG_A), "FLAG_A|FLAG_B");
}
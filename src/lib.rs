pub struct Crc8 {
    table: [u8; 256],
}

impl Crc8 {
    pub fn with_msb(poly: u8) -> Crc8 {
        const MSB: u8 = 0x80;

        let mut c = unsafe {
            Self {
                table: std::mem::uninitialized(),
            }
        };
        c.table[0] = 0;

        let mut t = MSB;
        let mut i = 1;
        while i < c.table.len() {
            t = (t << 1) ^ (if t & MSB != 0 { poly } else { 0 });
            for j in 0..i {
                c.table[i + j] = c.table[j] ^ t;
            }
            i <<= 1;
        }
        c
    }

    pub fn with_lsb(poly: u8) -> Crc8 {
        let mut c = unsafe {
            Self {
                table: std::mem::uninitialized(),
            }
        };
        c.table[0] = 0;

        let mut t = 1u8;
        let mut i = c.table.len() >> 1;
        while i > 0 {
            t = (t >> 1) ^ (if t & 1 != 0 { poly } else { 0 });

            let mut j = 0;
            while j < c.table.len() {
                c.table[i + j] = c.table[j] ^ t;
                j += i << 1;
            }
            i >>= 1;
        }
        c
    }


    pub fn calc(&self, buffer: &[u8], crc: u8) -> u8 {
        let mut crc_tmp = crc;
        for byte in buffer {
            crc_tmp = self.table[(crc_tmp ^ byte) as usize];
        }
        crc_tmp
    }
}


#[test]
fn test_msb() {
    assert_eq!(Crc8::with_msb(0x7).calc(b"test", 0), 0xB9);
}
#[test]
fn test_lsb() {
    assert_eq!(Crc8::with_lsb(0x7).calc(b"test", 0), 0x07);
}

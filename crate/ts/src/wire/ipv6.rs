pub const ADDR_SIZE: usize = 16;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Address(pub [u8; ADDR_SIZE]);

impl Address {
  pub const fn new(
    a0: u16,
    a1: u16,
    a2: u16,
    a3: u16,
    a4: u16,
    a5: u16,
    a6: u16,
    a7: u16,
  ) -> Address {
    Address([
      (a0 >> 8) as u8,
      a0 as u8,
      (a1 >> 8) as u8,
      a1 as u8,
      (a2 >> 8) as u8,
      a2 as u8,
      (a3 >> 8) as u8,
      a3 as u8,
      (a4 >> 8) as u8,
      a4 as u8,
      (a5 >> 8) as u8,
      a5 as u8,
      (a6 >> 8) as u8,
      a6 as u8,
      (a7 >> 8) as u8,
      a7 as u8,
    ])
  }
}

use super::ip::IPAddress;
pub const ADDR_SIZE: usize = 4;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Address(pub [u8; ADDR_SIZE]);

impl Address {
  pub const fn new(a0: u8, a1: u8, a2: u8, a3: u8) -> Address {
    Address([a0, a1, a2, a3])
  }
}

impl IPAddress for Address {
  const VERSION: u8 = 4;
  const UNSPECIFIED: Address = Address([0x00; ADDR_SIZE]);
  const BROADCAST: Address = Address([0xff; ADDR_SIZE]);
  const MULTICAST_ALL_SYSTEMS: Address = Address([224, 0, 0, 1]);
  const MULTICAST_ALL_ROUTERS: Address = Address([224, 0, 0, 2]);

  fn from_bytes(data: &[u8]) -> Address {
    let mut bytes = [0; ADDR_SIZE];
    bytes.copy_from_slice(data);
    Address(bytes)
  }

  fn as_bytes(&self) -> &[u8] {
    &self.0
  }

  fn is_unicast(&self) -> bool {
    !(self.is_broadcast() || self.is_multicast() || self.is_unspecified())
  }

  fn is_multicast(&self) -> bool {
    self.0[0] & 0xf0 == 224
  }

  fn is_broadcast(&self) -> bool {
    self.0[0..4] == Address::BROADCAST.0
  }

  fn is_unspecified(&self) -> bool {
    self.0[0..4] == Address::UNSPECIFIED.0
  }

  fn is_loopback(&self) -> bool {
    self.0[0] == 127
  }

  fn is_link_local(&self) -> bool {
    self.0[0..2] == [169, 254]
  }
}

impl core::fmt::Display for Address {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let bytes = self.0;
    write!(f, "{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let addr = Address::new(192, 168, 0, 1);
    assert_eq!(addr.0, [192, 168, 0, 1]);
  }

  #[test]
  fn test_from_bytes() {
    let bytes = [10, 0, 0, 1];
    let addr = Address::from_bytes(&bytes);
    assert_eq!(addr.0, bytes);
  }

  #[test]
  fn test_as_bytes() {
    let addr = Address::new(172, 16, 0, 1);
    assert_eq!(addr.as_bytes(), &[172, 16, 0, 1]);
  }

  #[test]
  fn test_constants() {
    assert_eq!(Address::VERSION, 4);
    assert_eq!(Address::UNSPECIFIED, Address([0, 0, 0, 0]));
    assert_eq!(Address::BROADCAST, Address([255, 255, 255, 255]));
    assert_eq!(Address::MULTICAST_ALL_SYSTEMS, Address([224, 0, 0, 1]));
    assert_eq!(Address::MULTICAST_ALL_ROUTERS, Address([224, 0, 0, 2]));
  }

  #[test]
  fn test_is_unicast() {
    assert!(Address::new(192, 168, 1, 1).is_unicast());
    assert!(!Address::UNSPECIFIED.is_unicast());
    assert!(!Address::BROADCAST.is_unicast());
    assert!(!Address::MULTICAST_ALL_SYSTEMS.is_unicast());
  }

  #[test]
  fn test_is_multicast() {
    assert!(Address::MULTICAST_ALL_SYSTEMS.is_multicast());
    assert!(Address::new(224, 0, 0, 5).is_multicast());
    assert!(!Address::new(192, 168, 1, 1).is_multicast());
  }

  #[test]
  fn test_is_broadcast() {
    assert!(Address::BROADCAST.is_broadcast());
    assert!(!Address::new(192, 168, 1, 255).is_broadcast());
  }

  #[test]
  fn test_is_unspecified() {
    assert!(Address::UNSPECIFIED.is_unspecified());
    assert!(!Address::new(0, 0, 0, 1).is_unspecified());
  }

  #[test]
  fn test_is_loopback() {
    assert!(Address::new(127, 0, 0, 1).is_loopback());
    assert!(!Address::new(192, 168, 1, 1).is_loopback());
  }

  #[test]
  fn test_is_link_local() {
    assert!(Address::new(169, 254, 0, 1).is_link_local());
    assert!(!Address::new(192, 168, 1, 1).is_link_local());
  }

  #[test]
  fn test_display() {
    let addr = Address::new(192, 168, 0, 1);
    assert_eq!(format!("{}", addr), "192.168.0.1");
  }

  #[test]
  fn test_ord() {
    let addr1 = Address::new(10, 0, 0, 1);
    let addr2 = Address::new(10, 0, 0, 2);
    let addr3 = Address::new(192, 168, 1, 1);

    assert!(addr1 < addr2);
    assert!(addr2 < addr3);
    assert!(addr1 < addr3);
  }

  #[test]
  fn test_clone_and_eq() {
    let addr1 = Address::new(192, 168, 1, 1);
    let addr2 = addr1.clone();

    assert_eq!(addr1, addr2);
  }

  #[test]
  fn test_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(Address::new(192, 168, 1, 1));
    set.insert(Address::new(10, 0, 0, 1));

    assert!(set.contains(&Address::new(192, 168, 1, 1)));
    assert!(!set.contains(&Address::new(172, 16, 0, 1)));
  }
}

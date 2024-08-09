use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::fmt::Debug;
use core::fmt::Display;
use core::hash::Hash;

pub trait IPAddress:
  Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Clone + Copy + Display
{
  const VERSION: u8;

  /// An unspecified address.
  const UNSPECIFIED: Self;

  /// The broadcast address.
  const BROADCAST: Self;

  /// All multicast-capable nodes
  const MULTICAST_ALL_SYSTEMS: Self;

  /// All multicast-capable routers
  const MULTICAST_ALL_ROUTERS: Self;

  /// Creates a new address from the given bytes.
  fn from_bytes(data: &[u8]) -> Self;

  /// Returns the address as a byte slice.
  fn as_bytes(&self) -> &[u8];

  /// Returns true if the address is a unicast address.
  fn is_unicast(&self) -> bool;

  /// Returns true if the address is a multicast address.
  fn is_multicast(&self) -> bool;

  /// Returns true if the address is a broadcast address.
  fn is_broadcast(&self) -> bool;

  /// Returns true if the address is an unspecified address.
  fn is_unspecified(&self) -> bool;

  /// Returns true if the address is a loopback address.
  fn is_loopback(&self) -> bool;

  /// Returns true if the address is a link-local address.
  fn is_link_local(&self) -> bool;

  /// Returns the prefix length of the address.
  fn prefix_len(&self) -> Option<u8> {
    let mut ones = true;
    let mut prefix_len = 0;
    for byte in self.as_bytes() {
      let mut mask = 0x80;
      for _ in 0..8 {
        let one = *byte & mask != 0;
        if ones {
          if one {
            prefix_len += 1;
          } else {
            ones = false;
          }
        } else if one {
          return None;
        }
        mask >>= 1;
      }
    }
    Some(prefix_len)
  }
}

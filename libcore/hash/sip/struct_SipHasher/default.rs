#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::hash::SipHasher;
    use core::hash::Hasher;

    use core::default::Default;

    // pub struct SipHasher {
    //     k0: u64,
    //     k1: u64,
    //     length: usize, // how many bytes we've processed
    //     v0: u64,      // hash state
    //     v1: u64,
    //     v2: u64,
    //     v3: u64,
    //     tail: u64, // unprocessed bytes le
    //     ntail: usize,  // how many bytes in tail are valid
    // }

    // macro_rules! u8to64_le {
    //     ($buf:expr, $i:expr) =>
    //     ($buf[0+$i] as u64 |
    //      ($buf[1+$i] as u64) << 8 |
    //      ($buf[2+$i] as u64) << 16 |
    //      ($buf[3+$i] as u64) << 24 |
    //      ($buf[4+$i] as u64) << 32 |
    //      ($buf[5+$i] as u64) << 40 |
    //      ($buf[6+$i] as u64) << 48 |
    //      ($buf[7+$i] as u64) << 56);
    //     ($buf:expr, $i:expr, $len:expr) =>
    //     ({
    //         let mut t = 0;
    //         let mut out = 0;
    //         while t < $len {
    //             out |= ($buf[t+$i] as u64) << t*8;
    //             t += 1;
    //         }
    //         out
    //     });
    // }

    // macro_rules! rotl {
    //     ($x:expr, $b:expr) =>
    //     (($x << $b) | ($x >> (64_i32.wrapping_sub($b))))
    // }

    // macro_rules! compress {
    //     ($v0:expr, $v1:expr, $v2:expr, $v3:expr) =>
    //     ({
    //         $v0 = $v0.wrapping_add($v1); $v1 = rotl!($v1, 13); $v1 ^= $v0;
    //         $v0 = rotl!($v0, 32);
    //         $v2 = $v2.wrapping_add($v3); $v3 = rotl!($v3, 16); $v3 ^= $v2;
    //         $v0 = $v0.wrapping_add($v3); $v3 = rotl!($v3, 21); $v3 ^= $v0;
    //         $v2 = $v2.wrapping_add($v1); $v1 = rotl!($v1, 17); $v1 ^= $v2;
    //         $v2 = rotl!($v2, 32);
    //     })
    // }

    // impl SipHasher {
    //     /// Creates a new `SipHasher` with the two initial keys set to 0.
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn new() -> SipHasher {
    //         SipHasher::new_with_keys(0, 0)
    //     }
    //
    //     /// Creates a `SipHasher` that is keyed off the provided keys.
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn new_with_keys(key0: u64, key1: u64) -> SipHasher {
    //         let mut state = SipHasher {
    //             k0: key0,
    //             k1: key1,
    //             length: 0,
    //             v0: 0,
    //             v1: 0,
    //             v2: 0,
    //             v3: 0,
    //             tail: 0,
    //             ntail: 0,
    //         };
    //         state.reset();
    //         state
    //     }
    //
    //     #[inline]
    //     fn reset(&mut self) {
    //         self.length = 0;
    //         self.v0 = self.k0 ^ 0x736f6d6570736575;
    //         self.v1 = self.k1 ^ 0x646f72616e646f6d;
    //         self.v2 = self.k0 ^ 0x6c7967656e657261;
    //         self.v3 = self.k1 ^ 0x7465646279746573;
    //         self.ntail = 0;
    //     }
    //
    //     #[inline]
    //     fn write(&mut self, msg: &[u8]) {
    //         let length = msg.len();
    //         self.length += length;
    //
    //         let mut needed = 0;
    //
    //         if self.ntail != 0 {
    //             needed = 8 - self.ntail;
    //             if length < needed {
    //                 self.tail |= u8to64_le!(msg, 0, length) << 8*self.ntail;
    //                 self.ntail += length;
    //                 return
    //             }
    //
    //             let m = self.tail | u8to64_le!(msg, 0, needed) << 8*self.ntail;
    //
    //             self.v3 ^= m;
    //             compress!(self.v0, self.v1, self.v2, self.v3);
    //             compress!(self.v0, self.v1, self.v2, self.v3);
    //             self.v0 ^= m;
    //
    //             self.ntail = 0;
    //         }
    //
    //         // Buffered tail is now flushed, process new input.
    //         let len = length - needed;
    //         let end = len & (!0x7);
    //         let left = len & 0x7;
    //
    //         let mut i = needed;
    //         while i < end {
    //             let mi = u8to64_le!(msg, i);
    //
    //             self.v3 ^= mi;
    //             compress!(self.v0, self.v1, self.v2, self.v3);
    //             compress!(self.v0, self.v1, self.v2, self.v3);
    //             self.v0 ^= mi;
    //
    //             i += 8;
    //         }
    //
    //         self.tail = u8to64_le!(msg, i, left);
    //         self.ntail = left;
    //     }
    // }

    // impl Hasher for SipHasher {
    //     #[inline]
    //     fn write(&mut self, msg: &[u8]) {
    //         self.write(msg)
    //     }
    //
    //     #[inline]
    //     fn finish(&self) -> u64 {
    //         let mut v0 = self.v0;
    //         let mut v1 = self.v1;
    //         let mut v2 = self.v2;
    //         let mut v3 = self.v3;
    //
    //         let b: u64 = ((self.length as u64 & 0xff) << 56) | self.tail;
    //
    //         v3 ^= b;
    //         compress!(v0, v1, v2, v3);
    //         compress!(v0, v1, v2, v3);
    //         v0 ^= b;
    //
    //         v2 ^= 0xff;
    //         compress!(v0, v1, v2, v3);
    //         compress!(v0, v1, v2, v3);
    //         compress!(v0, v1, v2, v3);
    //         compress!(v0, v1, v2, v3);
    //
    //         v0 ^ v1 ^ v2 ^ v3
    //     }
    // }

    // impl Default for SipHasher {
    //     fn default() -> SipHasher {
    //         SipHasher::new()
    //     }
    // }

    #[test]
    fn default_test1() {
	let siphasher: SipHasher = SipHasher::default();

	let finish: u64 = siphasher.finish();
	assert_eq!(finish, 0x1e924b9d737700d7);
    }

    #[test]
    fn default_test2() {
	let siphasher: SipHasher = Default::default();

	let finish: u64 = siphasher.finish();
	assert_eq!(finish, 0x1e924b9d737700d7);
    }
}

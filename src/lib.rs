//! ```rust
//! use unzip_iter::*;
//! fn iters_work() {
//!     let it = (0u8..4).zip(0u16..4);
//!     let (mut u8s, mut u16s) = it.unzip_iter();
//!
//!     assert_eq!(u8s.next(), Some(0));
//!     assert_eq!(u16s.next(), Some(0));
//!
//!     assert_eq!(u8s.next(), Some(1));
//!     assert_eq!(u8s.next(), Some(2));
//!
//!     assert_eq!(u16s.next(), Some(1));
//!     assert_eq!(u16s.next(), Some(2));
//! }
//! ```

mod ext;
pub use ext::UnzipExt;

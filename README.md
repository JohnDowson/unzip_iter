# unzip_iter
## Just like `Iterator::unzip`, but without collecting!
### For that once-in-a-lifetime occasion where you really need to unzip an iterator without evaluating all of it at once

# Usage
```rust
use unzip_iter::UnzipExt;

let it = (0u8..4).zip(0u16..4);
let (mut u8s, mut u16s) = it.unzip_iter();

// Type specifiers here are just for demonstration, type inference mostly works
assert_eq!(u8s.next(), Some(0u8));
assert_eq!(u16s.next(), Some(0u16));
```

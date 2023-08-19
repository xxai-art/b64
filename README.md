# b64 : urlsafe base64 encode / decode

[→ tests/main.rs](tests/main.rs)

```rust
use b64::{b64_u64, b64d, b64e, u64_b64};

#[test]
fn main() {
  let bin = [1, 2, 3];
  let b64 = b64e(bin);
  dbg!(&b64);
  dbg!(b64d(b64).unwrap());
  let n = 9876543210;
  let u64_b64 = u64_b64(n);
  dbg!(&u64_b64);
  dbg!(b64_u64(u64_b64));
}
```


run

[→ out.txt](out.txt)

```txt
```


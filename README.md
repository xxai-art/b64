# ub64 : urlsafe base64 encode / decode

[→ tests/main.rs](tests/main.rs)

```rust
use ub64::{b64_u64, b64d, b64e, u64_b64};

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
+ cargo test --all-features -- --nocapture
     Running tests/main.rs (target/debug/deps/main-8af48d1761c45bbe)
[tests/main.rs:7] &b64 = "AQID"
[tests/main.rs:8] b64d(b64).unwrap() = [
    1,
    2,
    3,
]
[tests/main.rs:11] &u64_b64 = "6hawTAI"
[tests/main.rs:12] b64_u64(u64_b64) = 9876543210
```


# Credit Card

Zero-dependency credit card number validation and parsing for Rust.

## Basic example

```rust
use creditcard::{CreditCard, CreditCardKind};

let card = "4111111111111111".parse::<CreditCard>().unwrap();

assert_eq!(card.kind(), CreditCardKind::Visa);
assert_eq!(card.pan(), 4111111111111111);
```

## Benchmarks

See the [`benches`](./benches) directory for basic micro-benchmarks.

| benchmark | `creditcard` | `card-validate` |
|-----------|--------------|-----------------|
| `too short` | **27.740ns** | 280.77ns |
| `too long` | **23.787ns** | 280.06ns |
| `invalid` | **18.113ns** | 279.38ns |
| `valid` | **28.651ns** | 139.38ns |


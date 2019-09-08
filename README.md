# Middle Square Weyl Sequence

[Middle Square Weyl Sequence][1] pseudorandom number generator (`no_std`).

[1]: https://en.wikipedia.org/wiki/Middle-square_method#Middle_Square_Weyl_Sequence_PRNG

## Example

```rust
use msws::Rand;

// This will always return the same seed.
msws::seed(0); // => 0xb5ad4eceda1ce2a9

let seed = 0xb5ad4eceda1ce2a9;
let mut r = Rand::new(seed).expect("invalid seed");
r.rand(); // => 0xb5ad4ece
```

## Crypto

Pseudorandom number generators should not be used for crypto.

### License

MIT

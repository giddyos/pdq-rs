# pdq-rs

Rust bindings for [Meta's PDQ perceptual hash algorithm](https://github.com/facebook/ThreatExchange/tree/main/pdq/cpp).

PDQ is a 256-bit perceptual image hash. Two images that are visually similar
(different compression, minor crops, resizes, slight color adjustments) will
produce hashes with a low Hamming distance. Two unrelated images will be ~128
bits apart. It was designed for detecting near-duplicate harmful content at
scale and is used in production by Meta and partners in the
[ThreatExchange](https://developers.facebook.com/programs/threat-exchange/)
program.

This crate wraps the upstream C++ library via a thin `extern "C"` shim and
[bindgen](https://github.com/rust-lang/rust-bindgen). The C++ source is
vendored so you don't need anything installed beyond a C++ compiler.

---

## What it does

- Compute a 256-bit PDQ hash from an image::DynamicImage
- Compute all 8 dihedral variants (rotations + flips) in a single pass —
  useful when you can't assume the image orientation
- Compute Hamming distance between two hashes with match classification helpers
- Parse hashes from hex strings

What it doesn't do: decode bytes for you. Bring your own decoder
([`image`](https://crates.io/crates/image) works well) and hand this crate
an `image::DynamicImage`.

---

## Usage

```toml
[dependencies]
pdq-rs = "1.0.0"
image = "0.25"  # or whatever decoder you prefer
```

```rust
use pdq_rs::{hamming_distance, pdq_dihedral_hash_rgb, pdq_hash_rgb};

let img = image::open("photo.jpg").unwrap();

let (hash, quality) = pdq_hash_rgb(&img).expect("image is too small to hash");
println!("{}", hash.to_hex());  // e.g. f8f8f0cce0f4e84d...
println!("quality: {}/100", quality);

let other = image::open("other-photo.jpg").unwrap();
let (hash2, _) = pdq_hash_rgb(&other).expect("image is too small to hash");
let distance = hamming_distance(&hash, &hash2);

println!("distance: {}", distance.distance());
println!("kind: {}", distance.kind());
println!("similarity: {:.2}%", distance.similarity_percent());

if distance.is_near_duplicate() {
    println!("likely the same image");
}
```

### Dihedral hashing

If you're matching user-uploaded images and can't assume they haven't been
rotated or flipped, compute all 8 variants at once. It's one C++ call
internally so it's faster than calling `hash_rgb` eight times.

```rust
let (variants, quality) = pdq_dihedral_hash_rgb(&img).expect("image is too small to hash");

// variants.original, .rotate90, .rotate180, .rotate270,
// .flip_x, .flip_y, .flip_plus1, .flip_minus1
```

### Already have a greyscale buffer?

```rust
let (hash, quality) = pdq_hash_grey(&img).expect("image is too small to hash");
let (variants, quality) = pdq_dihedral_hash_grey(&img).expect("image is too small to hash");
```

If you need exact full-resolution behavior without the built-in 512px cap, use
`pdq_hash_rgb_full`, `pdq_hash_grey_full`, `pdq_dihedral_hash_rgb_full`, or
`pdq_dihedral_hash_grey_full`.

Note: Meta's official implementation downscales images larger than 512px for more efficient calculation.

---

## How the matching threshold works

PDQ hashes are 256 bits. The Hamming distance between two hashes is how many
of those bits differ.

| Distance | Kind | Interpretation |
|----------|------|----------------|
| 0 | `Exact` | Identical hash |
| 1-31 | `NearDuplicate` | Same image with compression, crop, resize, or minor edits |
| 32-71 | `Similar` | Visually related, but outside the standard near-duplicate threshold |
| 72-127 | `Different` | Likely distinct images |
| 128-256 | `Unrelated` | Random or effectively unrelated hashes; ~128 is the expected midpoint |

The standard production threshold is still `< 32`, which is exposed as
`distance.is_near_duplicate()`. The returned `HammingDistance` value also gives
you `distance.kind()`, `distance.distance()`, `distance.matching_bits()`, and
`distance.similarity_percent()` if you want to build your own heuristics.

---

## Building

Requires a C++11 compiler. On macOS that's Xcode Command Line Tools
(`xcode-select --install`). On Linux, `g++` or `clang++`.

The C++ source is vendored under `vendor/` so there's nothing to install
separately. `cargo build` handles everything.

```bash
cargo build
cargo run --example hash_image -- /path/to/photo.jpg
```

---

## Quality score

Every successful hash call returns a quality score from 0 to 100 alongside the hash.
Low quality (below ~50) means the image didn't have enough gradient information
for the DCT to produce a reliable hash — think solid-color images, very small
images, or heavily blurred frames. The hash is still returned but you should
treat low-quality matches with more skepticism.

The minimum hashable dimension is 5×5 pixels. Below that, the hashing APIs
return `None`.

The default hashing APIs also downsample any image larger than 512 pixels in
either dimension before hashing. Use the `_full` variants if you do not want
that behavior.

---

## License

The Rust wrapper code in this crate is MIT licensed. The vendored PDQ C++ code
is copyright Meta Platforms, Inc. and licensed under the
[BSD license](https://github.com/facebook/ThreatExchange/blob/main/LICENSE).

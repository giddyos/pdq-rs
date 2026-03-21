#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
use std::borrow::Cow;

use image::DynamicImage;

include!(concat!(env!("OUT_DIR"), "/pdq_bindings.rs"));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HammingDistanceKind {
    /// 0 bits different, identical hashes
    Exact,
    /// 1-31 bits different, visually near-duplicates
    NearDuplicate,
    /// 32-71 bits different, visually similar but not near-duplicates
    Similar,
    /// 72-127 bits different, visually different but not completely unrelated
    Different,
    /// 128-256 bits different, visually unrelated
    Unrelated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HammingDistance {
    bits: u16,
}

impl HammingDistance {
    pub const MAX_BITS: u16 = 256;
    pub const NEAR_DUPLICATE_THRESHOLD: u16 = 32;
    pub const SIMILAR_THRESHOLD: u16 = 72;
    pub const UNRELATED_THRESHOLD: u16 = 128;

    pub fn from_bits(bits: u16) -> Self {
        assert!(
            bits <= Self::MAX_BITS,
            "PDQ Hamming distance must be in 0..=256"
        );
        Self { bits }
    }

    pub fn distance(self) -> u16 {
        self.bits
    }

    pub fn matching_bits(self) -> u16 {
        Self::MAX_BITS - self.bits
    }

    pub fn similarity_ratio(self) -> f32 {
        self.matching_bits() as f32 / Self::MAX_BITS as f32
    }

    pub fn similarity_percent(self) -> f32 {
        self.similarity_ratio() * 100.0
    }

    pub fn kind(self) -> HammingDistanceKind {
        match self.bits {
            0 => HammingDistanceKind::Exact,
            1..=31 => HammingDistanceKind::NearDuplicate,
            32..=71 => HammingDistanceKind::Similar,
            72..=127 => HammingDistanceKind::Different,
            _ => HammingDistanceKind::Unrelated,
        }
    }

    pub fn is_exact(self) -> bool {
        self.kind() == HammingDistanceKind::Exact
    }

    pub fn is_near_duplicate(self) -> bool {
        self.bits < Self::NEAR_DUPLICATE_THRESHOLD
    }
}

impl std::fmt::Display for HammingDistanceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            HammingDistanceKind::Exact => "exact",
            HammingDistanceKind::NearDuplicate => "near-duplicate",
            HammingDistanceKind::Similar => "similar",
            HammingDistanceKind::Different => "different",
            HammingDistanceKind::Unrelated => "unrelated",
        };

        f.write_str(label)
    }
}

impl std::fmt::Display for HammingDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bits ({})", self.distance(), self.kind())
    }
}

impl PdqHash256 {
    /// Parse a 64-char lowercase hex string into a PdqHash.
    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() != 64 {
            return None;
        }

        let mut bytes = [0u8; 32];

        for i in 0..32 {
            let byte_str = &hex[2 * i..2 * i + 2];
            match u8::from_str_radix(byte_str, 16) {
                Ok(b) => bytes[i] = b,
                Err(_) => return None,
            }
        }

        Some(PdqHash256 { bits: bytes })
    }

    pub fn to_hex(&self) -> String {
        self.bits.iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn to_binary(&self) -> String {
        self.bits.iter().map(|b| format!("{:08b}", b)).collect()
    }

    pub fn to_f32_vec(&self) -> Vec<f32> {
        self.bits.iter().map(|b| *b as f32).collect()
    }
}

pub struct PdqDihedralHashes {
    pub original: PdqHash256,
    pub rotate90: PdqHash256,
    pub rotate180: PdqHash256,
    pub rotate270: PdqHash256,
    pub flip_x: PdqHash256,
    pub flip_y: PdqHash256,
    pub flip_plus1: PdqHash256,
    pub flip_minus1: PdqHash256,
}

const fn new_c_pdq_hash() -> PdqHash256 {
    PdqHash256 { bits: [0u8; 32] }
}

const MIN_HASHABLE_DIM: u32 = 5;
const DOWNSAMPLE_DIMS: u32 = 512;

fn prepare_image(image: &DynamicImage) -> Option<Cow<'_, DynamicImage>> {
    if image.width() < MIN_HASHABLE_DIM || image.height() < MIN_HASHABLE_DIM {
        return None;
    }

    if image.width() > DOWNSAMPLE_DIMS || image.height() > DOWNSAMPLE_DIMS {
        Some(Cow::Owned(image.thumbnail_exact(
            DOWNSAMPLE_DIMS.min(image.width()),
            DOWNSAMPLE_DIMS.min(image.height()),
        )))
    } else {
        Some(Cow::Borrowed(image))
    }
}

/// hash an interleaved RGB image. Returns (hash, quality 0-100).
pub fn pdq_hash_rgb(image: &DynamicImage) -> Option<(PdqHash256, i32)> {
    let prepared = prepare_image(image)?;
    let img_ref: &DynamicImage = match prepared {
        Cow::Borrowed(b) => b,
        Cow::Owned(ref o) => o,
    };

    pdq_hash_rgb_full(img_ref)
}

/// hash an interleaved RGB image without downsampling. Returns (hash, quality 0-100).
pub fn pdq_hash_rgb_full(image: &DynamicImage) -> Option<(PdqHash256, i32)> {
    if image.width() < MIN_HASHABLE_DIM || image.height() < MIN_HASHABLE_DIM {
        return None;
    }

    let rgb = image.to_rgb8();
    let num_rows = image.height() as i32;
    let num_cols = image.width() as i32;
    let mut out = new_c_pdq_hash();

    let quality = unsafe { pdq_hash_from_rgb(rgb.as_ptr(), num_rows, num_cols, &mut out) };

    Some((out, quality))
}

/// hash a greyscale image. Returns (hash, quality 0-100).
pub fn pdq_hash_grey(image: &DynamicImage) -> Option<(PdqHash256, i32)> {
    let prepared = prepare_image(image)?;
    let img_ref: &DynamicImage = match prepared {
        Cow::Borrowed(b) => b,
        Cow::Owned(ref o) => o,
    };

    pdq_hash_grey_full(img_ref)
}

/// hash a greyscale image without downsampling. Returns (hash, quality 0-100).
pub fn pdq_hash_grey_full(image: &DynamicImage) -> Option<(PdqHash256, i32)> {
    if image.width() < MIN_HASHABLE_DIM || image.height() < MIN_HASHABLE_DIM {
        return None;
    }

    let grey = image.to_luma8();
    let num_rows = image.height() as i32;
    let num_cols = image.width() as i32;
    let mut out = new_c_pdq_hash();

    let quality = unsafe { pdq_hash_from_grey(grey.as_ptr(), num_rows, num_cols, &mut out) };

    Some((out, quality))
}

/// compute all 8 dihedral hashes in one pass. Returns (hashes, quality 0-100).
pub fn pdq_dihedral_hash_rgb(image: &DynamicImage) -> Option<(PdqDihedralHashes, i32)> {
    let prepared = prepare_image(image)?;
    let img_ref: &DynamicImage = match prepared {
        Cow::Borrowed(b) => b,
        Cow::Owned(ref o) => o,
    };

    pdq_dihedral_hash_rgb_full(img_ref)
}

/// compute all 8 dihedral hashes in one pass without downsampling. Returns (hashes, quality 0-100).
pub fn pdq_dihedral_hash_rgb_full(image: &DynamicImage) -> Option<(PdqDihedralHashes, i32)> {
    if image.width() < MIN_HASHABLE_DIM || image.height() < MIN_HASHABLE_DIM {
        return None;
    }

    let rgb = image.to_rgb8();
    let num_rows = image.height() as i32;
    let num_cols = image.width() as i32;
    let mut h0 = new_c_pdq_hash();
    let mut h90 = new_c_pdq_hash();
    let mut h180 = new_c_pdq_hash();
    let mut h270 = new_c_pdq_hash();
    let mut hfx = new_c_pdq_hash();
    let mut hfy = new_c_pdq_hash();
    let mut hfp1 = new_c_pdq_hash();
    let mut hfm1 = new_c_pdq_hash();

    let quality = unsafe {
        pdq_dihedral_hash_from_rgb(
            rgb.as_ptr(),
            num_rows,
            num_cols,
            &mut h0,
            &mut h90,
            &mut h180,
            &mut h270,
            &mut hfx,
            &mut hfy,
            &mut hfp1,
            &mut hfm1,
        )
    };

    Some((
        PdqDihedralHashes {
            original: h0,
            rotate90: h90,
            rotate180: h180,
            rotate270: h270,
            flip_x: hfx,
            flip_y: hfy,
            flip_plus1: hfp1,
            flip_minus1: hfm1,
        },
        quality,
    ))
}

/// compute all 8 dihedral hashes for a greyscale image. Returns (hashes, quality 0-100).
pub fn pdq_dihedral_hash_grey(image: &DynamicImage) -> Option<(PdqDihedralHashes, i32)> {
    let prepared = prepare_image(image)?;
    let img_ref: &DynamicImage = match prepared {
        Cow::Borrowed(b) => b,
        Cow::Owned(ref o) => o,
    };

    pdq_dihedral_hash_grey_full(img_ref)
}

/// compute all 8 dihedral hashes for a greyscale image without downsampling. Returns (hashes, quality 0-100).
pub fn pdq_dihedral_hash_grey_full(image: &DynamicImage) -> Option<(PdqDihedralHashes, i32)> {
    if image.width() < MIN_HASHABLE_DIM || image.height() < MIN_HASHABLE_DIM {
        return None;
    }

    let grey = image.to_luma8();
    let num_rows = image.height() as i32;
    let num_cols = image.width() as i32;
    let mut h0 = new_c_pdq_hash();
    let mut h90 = new_c_pdq_hash();
    let mut h180 = new_c_pdq_hash();
    let mut h270 = new_c_pdq_hash();
    let mut hfx = new_c_pdq_hash();
    let mut hfy = new_c_pdq_hash();
    let mut hfp1 = new_c_pdq_hash();
    let mut hfm1 = new_c_pdq_hash();

    let quality = unsafe {
        pdq_dihedral_hash_from_grey(
            grey.as_ptr(),
            num_rows,
            num_cols,
            &mut h0,
            &mut h90,
            &mut h180,
            &mut h270,
            &mut hfx,
            &mut hfy,
            &mut hfp1,
            &mut hfm1,
        )
    };

    Some((
        PdqDihedralHashes {
            original: h0.into(),
            rotate90: h90.into(),
            rotate180: h180.into(),
            rotate270: h270.into(),
            flip_x: hfx.into(),
            flip_y: hfy.into(),
            flip_plus1: hfp1.into(),
            flip_minus1: hfm1.into(),
        },
        quality,
    ))
}

/// hamming distance between two hashes with helper methods for classification.
pub fn hamming_distance(a: &PdqHash256, b: &PdqHash256) -> HammingDistance {
    let ca = PdqHash256 { bits: a.bits };
    let cb = PdqHash256 { bits: b.bits };

    let bits = unsafe { pdq_hamming_distance(&ca, &cb) } as u16;

    HammingDistance::from_bits(bits)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::LazyLock};

    use super::*;

    const DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/data")));
    
    #[test]
    fn test_hash_and_distance() {
        let hash1 = PdqHash256 { bits: [0b10101010; 32] };
        let hash2 = PdqHash256 { bits: [0b11110000; 32] };

        let dist = hamming_distance(&hash1, &hash2);
        assert_eq!(dist.distance(), 128);
        assert_eq!(dist.kind(), HammingDistanceKind::Unrelated);
    }

    #[test]
    fn test_hash_image() {
        let img_path = DATA_PATH.join("bridge-1-original.jpg");
        let img: image::DynamicImage = image::open(&img_path).expect("Failed to open image");
        let (hash, quality) = pdq_hash_rgb_full(&img).expect("Expected image to be hashable");
        assert!(quality > 49, "Expected positive quality score");
        assert_eq!(hash.bits.len(), 32, "Expected hash length of 32 bytes");
    }

    #[test]
    fn test_dihedral_hashes() {
        let img_path = DATA_PATH.join("bridge-1-original.jpg");
        let img: image::DynamicImage = image::open(&img_path).expect("Failed to open image");
        let (dihedral, quality) =
            pdq_dihedral_hash_rgb_full(&img).expect("Expected image to be hashable");
        assert!(quality > 49, "Expected positive quality score");
        assert_eq!(dihedral.original.bits.len(), 32, "Expected hash length of 32 bytes");

        let expected_hashes = [
            "f8f8f0cee0f4a84f06370a22038f63f0b36e2ed596621e1d33e6b39c4e9c9b22",
            "30a10efd71c83f429013d48d0ffffc52e34e0e17ada952a9d29685211ea9e5af",
            "adad5a64b5a102e55b62a08856dacd5ae63b847fc337b4b766b319361bc93188",
            "a5f0b457a49995e8c1065c275aaa54d8b61ba49df8fcfc0383c32f8b0bfc4f05",
            "f8f80f31e0f457b00637f5d5028f980fb36ed12a9622e1e233e64c634e9c64dd",
            "0dad2599b1a1bd1a5362576752da32a5e63b7380c2374b4866b346c91bc9ce77",
            "f0a5e102f1ccc0bd945308720fff038de34ef1e8ada9a956d2967ade5ea91a50",
            "a5f04ba8a4996a17c906a3d85aaaa927b61b5b42f8fc03fc87c3d0740bfcb0fa",
        ];

        let actual_hashes = [
            dihedral.original.to_hex(),
            dihedral.rotate90.to_hex(),
            dihedral.rotate180.to_hex(),
            dihedral.rotate270.to_hex(),
            dihedral.flip_x.to_hex(),
            dihedral.flip_y.to_hex(),
            dihedral.flip_plus1.to_hex(),
            dihedral.flip_minus1.to_hex(),
        ];

        for (actual, expected) in actual_hashes.iter().zip(expected_hashes.iter()) {
            assert_eq!(actual, expected, "Dihedral hash does not match expected value");
        }
    }

    #[test]
    fn test_load() {
        fn load(data: &[u8]) -> String {
            let img = image::load_from_memory(data).expect("Failed to load image from memory");
            let (hash, quality) = pdq_hash_rgb_full(&img).expect("Expected image to be hashable");
            assert!(quality > 49, "Expected positive quality score");
            hash.to_hex()
        }

        assert_eq!(
            "f8f8f0cee0f4a84f06370a22038f63f0b36e2ed596621e1d33e6b39c4e9c9b22",
            load(include_bytes!("../data/bridge-1-original.jpg"))
        );
                
         assert_eq!(
            "30a10efd71cc3d429013d48d0ffffc52e34e0e17ada952a9d29685211ea9e5af",
            load(include_bytes!("../data/bridge-2-rotate-90.jpg"))
        );

        assert_eq!(
            "adad5a64b5a142e75b62a09857da895ae63b847fc23794b766b319361bc93188",
            load(include_bytes!("../data/bridge-3-rotate-180.jpg"))
        );
        
        assert_eq!(
            "a5f0a457a48995e8c9065c275aaa5498b61ba4bdf8fcf80387c32f8b1bfc4f05",
            load(include_bytes!("../data/bridge-4-rotate-270.jpg"))
        );
        
        assert_eq!(
            "f8f80f31e0f417b20e37f5cd028f980fb36ed02a9662c1e233e64c634e9c64dd",
            load(include_bytes!("../data/bridge-5-flipx.jpg"))
        );

        assert_eq!(
            "0dad2599b1a1bd1a5362576742da32a5e63b7380c2374b4866b366c91bc9ce77",
            load(include_bytes!("../data/bridge-6-flipy.jpg"))
        );
        
        assert_eq!(
            "f0a5e102f1ccc0bd945308720fff038de34ef1e8ada9a956d2967ade5ea91a50",
            load(include_bytes!("../data/bridge-7-flip-plus-1.jpg"))
        );

        assert_eq!(
            "a5f05aa8a4896a17c906a2d85aaaab07b61b5b42f8fc07fc87c3d0741bfcb0fa",
            load(include_bytes!("../data/bridge-8-flip-minus-1.jpg"))
        );

    }

    #[test]
    fn test_hash_too_small_returns_none() {
        let img = image::DynamicImage::new_rgb8(MIN_HASHABLE_DIM - 1, MIN_HASHABLE_DIM);

        assert!(pdq_hash_rgb(&img).is_none());
        assert!(pdq_hash_rgb_full(&img).is_none());
        assert!(pdq_hash_grey(&img).is_none());
        assert!(pdq_hash_grey_full(&img).is_none());
        assert!(pdq_dihedral_hash_rgb(&img).is_none());
        assert!(pdq_dihedral_hash_rgb_full(&img).is_none());
        assert!(pdq_dihedral_hash_grey(&img).is_none());
        assert!(pdq_dihedral_hash_grey_full(&img).is_none());
    }

}
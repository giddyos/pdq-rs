use image::{DynamicImage, load_from_memory};

use crate::wit::wit_util::*;
use crate::{HammingDistance, HammingDistanceKind, PdqDihedralHashes, PdqHash256};

wit_bindgen::generate!({
    path: "wit",
    world: "pdq",
    pub_export_macro: true,
});

struct PdqComponent;

impl crate::wit::Guest for PdqComponent {
    #[allow(async_fn_in_trait)]
    fn hash_from_hex(hex: _rt::String) -> Option<PdqHash> {
        PdqHash256::from_hex(&hex).map(|hash| to_wit_hash(&hash))
    }

    #[allow(async_fn_in_trait)]
    fn hash_to_hex(hash: PdqHash) -> Result<_rt::String, _rt::String> {
        Ok(from_wit_hash(hash)?.to_hex())
    }

    #[allow(async_fn_in_trait)]
    fn hash_to_binary(hash: PdqHash) -> Result<_rt::String, _rt::String> {
        Ok(from_wit_hash(hash)?.to_binary())
    }

    #[allow(async_fn_in_trait)]
    fn hash_rgb(image_bytes: _rt::Vec<u8>) -> Result<Option<HashResult>, _rt::String> {
        let image = decode_image(&image_bytes)?;
        Ok(crate::pdq_hash_rgb(&image).map(to_wit_hash_result))
    }

    #[allow(async_fn_in_trait)]
    fn hash_rgb_full(image_bytes: _rt::Vec<u8>) -> Result<Option<HashResult>, _rt::String> {
        let image = decode_image(&image_bytes)?;
        Ok(crate::pdq_hash_rgb_full(&image).map(to_wit_hash_result))
    }

    #[allow(async_fn_in_trait)]
    fn hash_grey(image_bytes: _rt::Vec<u8>) -> Result<Option<HashResult>, _rt::String> {
        let image = decode_image(&image_bytes)?;
        Ok(crate::pdq_hash_grey(&image).map(to_wit_hash_result))
    }

    #[allow(async_fn_in_trait)]
    fn hash_grey_full(image_bytes: _rt::Vec<u8>) -> Result<Option<HashResult>, _rt::String> {
        let image = decode_image(&image_bytes)?;
        Ok(crate::pdq_hash_grey_full(&image).map(to_wit_hash_result))
    }

    #[allow(async_fn_in_trait)]
    fn dihedral_hash_rgb(
        image_bytes: _rt::Vec<u8>,
    ) -> Result<Option<DihedralHashResult>, _rt::String> {
        let image = decode_image(&image_bytes)?;
        Ok(crate::pdq_dihedral_hash_rgb(&image).map(to_wit_dihedral_hash_result))
    }

    #[allow(async_fn_in_trait)]
    fn dihedral_hash_rgb_full(
        image_bytes: _rt::Vec<u8>,
    ) -> Result<Option<DihedralHashResult>, _rt::String> {
        let image = decode_image(&image_bytes)?;
        Ok(crate::pdq_dihedral_hash_rgb_full(&image).map(to_wit_dihedral_hash_result))
    }

    #[allow(async_fn_in_trait)]
    fn dihedral_hash_grey(
        image_bytes: _rt::Vec<u8>,
    ) -> Result<Option<DihedralHashResult>, _rt::String> {
        let image = decode_image(&image_bytes)?;
        Ok(crate::pdq_dihedral_hash_grey(&image).map(to_wit_dihedral_hash_result))
    }

    #[allow(async_fn_in_trait)]
    fn dihedral_hash_grey_full(
        image_bytes: _rt::Vec<u8>,
    ) -> Result<Option<DihedralHashResult>, _rt::String> {
        let image = decode_image(&image_bytes)?;
        Ok(crate::pdq_dihedral_hash_grey_full(&image).map(to_wit_dihedral_hash_result))
    }

    #[allow(async_fn_in_trait)]
    fn hamming_distance(a: PdqHash, b: PdqHash) -> Result<DistanceInfo, _rt::String> {
        let a = from_wit_hash(a)?;
        let b = from_wit_hash(b)?;
        Ok(to_wit_distance_info(crate::hamming_distance(&a, &b)))
    }
}

crate::wit::export!(PdqComponent);

pub mod wit_util {
    use super::*;

    pub fn decode_image(image_bytes: &[u8]) -> Result<DynamicImage, String> {
        load_from_memory(image_bytes).map_err(|e| format!("failed to decode image: {e}"))
    }

    pub fn to_wit_hash(hash: &PdqHash256) -> crate::wit::PdqHash {
        crate::wit::PdqHash {
            bits: hash.bits.to_vec(),
        }
    }

    pub fn from_wit_hash(hash: crate::wit::PdqHash) -> Result<PdqHash256, String> {
        let bits: [u8; 32] = hash
            .bits
            .try_into()
            .map_err(|v: Vec<u8>| format!("expected 32 hash bytes, got {}", v.len()))?;

        Ok(PdqHash256 { bits })
    }

    pub fn to_wit_hash_result(value: (PdqHash256, i32)) -> crate::wit::HashResult {
        let (hash, quality) = value;
        crate::wit::HashResult {
            hash: to_wit_hash(&hash),
            quality,
        }
    }

    pub fn to_wit_dihedral_hashes(value: PdqDihedralHashes) -> crate::wit::DihedralHashes {
        crate::wit::DihedralHashes {
            original: to_wit_hash(&value.original),
            rotate90: to_wit_hash(&value.rotate90),
            rotate180: to_wit_hash(&value.rotate180),
            rotate270: to_wit_hash(&value.rotate270),
            flip_x: to_wit_hash(&value.flip_x),
            flip_y: to_wit_hash(&value.flip_y),
            flip_plus1: to_wit_hash(&value.flip_plus1),
            flip_minus1: to_wit_hash(&value.flip_minus1),
        }
    }

    pub fn to_wit_dihedral_hash_result(
        value: (PdqDihedralHashes, i32),
    ) -> crate::wit::DihedralHashResult {
        let (hashes, quality) = value;
        crate::wit::DihedralHashResult {
            hashes: to_wit_dihedral_hashes(hashes),
            quality,
        }
    }

    pub fn to_wit_distance_kind(kind: HammingDistanceKind) -> crate::wit::DistanceKind {
        match kind {
            HammingDistanceKind::Exact => crate::wit::DistanceKind::Exact,
            HammingDistanceKind::NearDuplicate => crate::wit::DistanceKind::NearDuplicate,
            HammingDistanceKind::Similar => crate::wit::DistanceKind::Similar,
            HammingDistanceKind::Different => crate::wit::DistanceKind::Different,
            HammingDistanceKind::Unrelated => crate::wit::DistanceKind::Unrelated,
        }
    }

    pub fn to_wit_distance_info(value: HammingDistance) -> crate::wit::DistanceInfo {
        crate::wit::DistanceInfo {
            bits: value.distance(),
            matching_bits: value.matching_bits(),
            similarity_ratio: value.similarity_ratio(),
            similarity_percent: value.similarity_percent(),
            kind: to_wit_distance_kind(value.kind()),
            exact: value.is_exact(),
            near_duplicate: value.is_near_duplicate(),
        }
    }
}

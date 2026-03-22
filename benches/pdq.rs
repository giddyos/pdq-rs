use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::{hint::black_box, sync::LazyLock};

use image::{DynamicImage, imageops::FilterType};
use pdq_rs::{
    HammingDistance, PdqHash256, hamming_distance, pdq_dihedral_hash_grey, pdq_dihedral_hash_rgb,
    pdq_hash_grey, pdq_hash_rgb, pdq_hash_rgb_full,
};

static BASE_IMAGE: LazyLock<DynamicImage> = LazyLock::new(|| {
    image::load_from_memory(include_bytes!("../data/bridge-1-original.jpg"))
        .expect("benchmark image should decode")
});

static LARGE_IMAGE: LazyLock<DynamicImage> = LazyLock::new(|| {
    let resized = image::imageops::resize(&BASE_IMAGE.to_rgb8(), 1024, 1024, FilterType::Lanczos3);
    DynamicImage::ImageRgb8(resized)
});

static SMALL_IMAGE: LazyLock<DynamicImage> = LazyLock::new(|| {
    image::load_from_memory(include_bytes!("../data/bridge-256px.jpg"))
        .expect("benchmark image should decode")
});

static HAMMING_INPUTS: LazyLock<(PdqHash256, PdqHash256)> = LazyLock::new(|| {
    let (left, _) = pdq_hash_rgb_full(&BASE_IMAGE).expect("base image should hash");
    let (right, _) = pdq_hash_rgb_full(&LARGE_IMAGE).expect("large image should hash");
    (left, right)
});

fn benchmark_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashing");

    group.throughput(Throughput::Elements(1));

    group.bench_function("pdq_hash_rgb/full", |b| {
        b.iter(|| pdq_hash_rgb_full(black_box(&BASE_IMAGE)).expect("image should hash"))
    });

    group.bench_function("pdq_hash_rgb/downsampled", |b| {
        b.iter(|| pdq_hash_rgb(black_box(&LARGE_IMAGE)).expect("image should hash"))
    });

    group.bench_function("pdq_hash_grey", |b| {
        b.iter(|| pdq_hash_grey(black_box(&BASE_IMAGE)).expect("image should hash"))
    });

    group.bench_function("pdq_dihedral_hash_rgb", |b| {
        b.iter(|| pdq_dihedral_hash_rgb(black_box(&BASE_IMAGE)).expect("image should hash"))
    });

    group.bench_function("pdq_dihedral_hash_grey", |b| {
        b.iter(|| pdq_dihedral_hash_grey(black_box(&BASE_IMAGE)).expect("image should hash"))
    });

    group.bench_function("pdq_hash_rgb/small_image", |b| {
        b.iter(|| pdq_hash_rgb(black_box(&SMALL_IMAGE)).expect("image should hash"))
    });

    group.finish();
}

fn benchmark_hamming_distance(c: &mut Criterion) {
    let (left, right) = *HAMMING_INPUTS;
    let mut group = c.benchmark_group("distance");

    group.throughput(Throughput::Elements(1));
    group.bench_function("hamming_distance", |b| {
        b.iter(|| -> HammingDistance { hamming_distance(black_box(&left), black_box(&right)) })
    });

    group.finish();
}

criterion_group!(benches, benchmark_hashing, benchmark_hamming_distance);
criterion_main!(benches);

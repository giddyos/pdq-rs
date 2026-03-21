use pdq_rs::{PdqHash256, hamming_distance, pdq_dihedral_hash_rgb, pdq_hash_rgb};

fn main() {
    let img_path = std::env::args().nth(1).unwrap_or_else(|| {
        let default_path = "data/bridge-1-original.jpg";
        println!("No image path provided, using default: {}", default_path);
        format!("{}/{}", env!("CARGO_MANIFEST_DIR"), default_path)
    });

    let img: image::DynamicImage = image::open(&img_path)
        .unwrap_or_else(|e| panic!("Failed to open image '{}': {}", img_path, e));

    let (width, height) = (img.width(), img.height());

    println!("Image: {} ({}x{})", img_path, width, height);

    // we compute a single hash for the original image
    let (hash, quality) = pdq_hash_rgb(&img).expect("Image is too small to hash");

    println!("\n--- Single hash ---");
    println!("Hash:    {}", &hash.to_hex());
    println!("Quality: {}/100", quality);

    // we compute all 8 dihedral variants in one efficient pass
    // (useful for matching images regardless of rotation/flip)
    let (dihedral, _) = pdq_dihedral_hash_rgb(&img).expect("Image is too small to hash");

    println!("\n--- Dihedral hashes ---");
    println!("Original:     {}", &dihedral.original.to_hex());
    println!("Rotate 90°:   {}", &dihedral.rotate90.to_hex());
    println!("Rotate 180°:  {}", &dihedral.rotate180.to_hex());
    println!("Rotate 270°:  {}", &dihedral.rotate270.to_hex());
    println!("Flip X:       {}", &dihedral.flip_x.to_hex());
    println!("Flip Y:       {}", &dihedral.flip_y.to_hex());
    println!("Flip +diag:   {}", &dihedral.flip_plus1.to_hex());
    println!("Flip -diag:   {}", &dihedral.flip_minus1.to_hex());

    let dist_orig_vs_rot90 = hamming_distance(&dihedral.original, &dihedral.rotate90);
    let dist_orig_vs_self = hamming_distance(&dihedral.original, &dihedral.original);

    println!("\n--- Hamming distances ---");
    println!(
        "original vs itself:     {} | similarity: {:.2}%",
        dist_orig_vs_self,
        dist_orig_vs_self.similarity_percent()
    );
    println!(
        "original vs rotate90:   {} | similarity: {:.2}%",
        dist_orig_vs_rot90,
        dist_orig_vs_rot90.similarity_percent()
    );

    let hex = hash.to_hex();
    let parsed = PdqHash256::from_hex(&hex).expect("Failed to parse hex hash");
    let roundtrip_dist = hamming_distance(&hash, &parsed);

    println!("\n--- Hex round-trip ---");
    println!("Original:   {}", &hash.to_hex());
    println!("Parsed:     {}", &parsed.to_hex());
    println!("Binary:     {}", &hash.to_binary());
    println!(
        "Distance:   {} | similarity: {:.2}%",
        roundtrip_dist,
        roundtrip_dist.similarity_percent()
    );
}

// Alternative implementation with compression
// Add to Cargo.toml: flate2 = "1.0"

use flate2::read::GzDecoder;
use std::io::Read;

// Compress words.txt with gzip first: gzip -k words.txt
// Then include the compressed version
const COMPRESSED_WORDS: &[u8] = include_bytes!("../../words.txt.gz");

pub fn load_words_compressed() -> Result<Vec<String>, std::io::Error> {
    let mut decoder = GzDecoder::new(COMPRESSED_WORDS);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;
    
    let words: Vec<String> = decompressed
        .lines()
        .map(|word| word.trim().to_uppercase())
        .filter(|word| word.len() == 5)
        .collect();

    Ok(words)
}

// Size comparison:
// - Uncompressed: ~13 KB in binary
// - Compressed: ~5 KB in binary + decompression code
// - Savings: ~8 KB (not significant for this use case)
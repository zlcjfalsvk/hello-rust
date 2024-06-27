use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;

fn roundtrip(data: &[u8]) {
    println!("data: {:?}", data);
    // Compress the input
    let compressed = compress_to_vec(data, 6);
    println!("compressed data: {:?}", &compressed);

    // Decompress the compressed input and limit max output size to avoid going out of memory on large/malformed input.
    let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");
    println!("decompressed data: {:?}", &decompressed);

    // Check roundtrip succeeded
    assert_eq!(data, decompressed);
}

fn main() {
    let message_bytes = "Hello, world!".as_bytes();
    roundtrip(message_bytes);
}

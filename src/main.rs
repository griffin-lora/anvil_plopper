use fastanvil::{CurrentJavaChunk, Region, Chunk, Block, Result};
use fastnbt::error::Error;
use fastnbt::{Value};
use fastnbt::from_bytes;
use fastnbt::to_bytes;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let file: File = File::options().read(true).write(true).open("regions/upper.mca").unwrap();

    let mut region: Region<File> = Region::from_stream(file).unwrap();
    let data = region.read_chunk(0, 0).unwrap().unwrap();

    let chunk: HashMap<String, Value> = from_bytes(data.as_slice()).unwrap();

    match chunk["DataVersion"] {
        Value::Int(ver) => println!("Version: {}", ver),
        _ => {},
    }

    let bytes = to_bytes(&chunk).unwrap();

    match region.write_chunk(0, 0, &bytes) {
        Ok(_) => {}
        Err(error) => panic!("Error writing chunk: {:?}", error)
    };
}
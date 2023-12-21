use fastanvil::{CurrentJavaChunk, Region, Chunk, Block, Result};
use fastnbt::error::Error;
use fastnbt::{Value};
use fastnbt::from_bytes;
use fastnbt::to_bytes;
use std::fs::File;

//
// This loads a region file, extracts a chunk from it, and uses serde to
// deserialize it into a `anvil::Chunk` object and print it.
//

fn main() {
    let file: File = File::options().read(true).write(true).open("regions/vanilla.mca").unwrap();

    let mut region: Region<File> = Region::from_stream(file).unwrap();
    let data = region.read_chunk(0, 0).unwrap().unwrap();

    let chunk: Value = from_bytes(data.as_slice()).unwrap();

    let bytes = to_bytes(&chunk).unwrap();

    match region.write_chunk(0, 0, &bytes) {
        Ok(_) => {}
        Err(error) => panic!("Error writing chunk: {:?}", error)
    };
}
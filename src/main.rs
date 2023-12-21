use fastanvil::{CurrentJavaChunk, Region};
use fastnbt::from_bytes;

//
// This loads a region file, extracts a chunk from it, and uses serde to
// deserialize it into a `anvil::Chunk` object and print it.
//

fn main() {
    let file = std::fs::File::open("regions/vanilla.mca").unwrap();

    let mut region = Region::from_stream(file).unwrap();
    let data = region.read_chunk(0, 0).unwrap().unwrap();

    let chunk: CurrentJavaChunk = from_bytes(data.as_slice()).unwrap();
    chunk.block();

    println!("{:?}", chunk);
}
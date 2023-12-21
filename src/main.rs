use fastanvil::{CurrentJavaChunk, Region, Chunk, Block};
use fastnbt::{Value};
use fastnbt::from_bytes;
use fastnbt::to_bytes;

//
// This loads a region file, extracts a chunk from it, and uses serde to
// deserialize it into a `anvil::Chunk` object and print it.
//

fn main() {
    let mut file: std::fs::File = std::fs::File::open("regions/vanilla.mca").unwrap();

    let mut region: Region<std::fs::File> = Region::from_stream(file).unwrap();
    let data = region.read_chunk(0, 0).unwrap().unwrap();

    let chunk: Value = from_bytes(data.as_slice()).unwrap();

    let bytes = to_bytes(&chunk).unwrap();

    region.write_chunk(0, 0, &bytes);
}
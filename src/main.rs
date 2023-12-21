use fastanvil::{Region};
use fastnbt::{Value};
use fastnbt::from_bytes;
use fastnbt::to_bytes;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let file: File = File::options().read(true).write(true).open("regions/vanilla.mca").unwrap();

    let mut region: Region<File> = Region::from_stream(file).unwrap();
    let data = region.read_chunk(0, 0).unwrap().unwrap();

    let mut chunk: HashMap<String, Value> = from_bytes(data.as_slice()).unwrap();

    let sections: &mut Vec<Value> = match chunk.get_mut("sections").unwrap() {
        Value::List(list) => list,
        _ => panic!("Failed to get sections")
    };

    println!("Num sections: {:?}", sections.len());

    for section_value in sections {
        let section: &mut HashMap<String, Value> = match section_value {
            Value::Compound(compound) => compound,
            _ => panic!("Failed to get section")
        };

        let section_y: i32 = match section["Y"] {
            Value::Byte(num) => num.into(),
            Value::Int(num) => num,
            _ => panic!("Failed to get Y value")
        };

        if section_y != -4 {
            continue;
        }

        let block_states: &mut HashMap<String, Value> = match section.get_mut("block_states").unwrap() {
            Value::Compound(compound) => compound,
            _ => panic!("Failed to get block states")
        };

        let palette: &mut Vec<Value> = match block_states.get_mut("palette").unwrap() {
            Value::List(list) => list,
            _ => panic!("Failed to get palette")
        };

        for block_value in palette {
            let block: &mut HashMap<String, Value> = match block_value {
                Value::Compound(compound) => compound,
                _ => panic!("Failed to get block")
            };

            let name: &mut String = match block.get_mut("Name").unwrap() {
                Value::String(string) => string,
                _ => panic!("Failed to get block name")
            };

            if name == "minecraft:bedrock" {
                name.replace_range(.., "minecraft:dirt");
            }
        }

        println!("{:?}", section);
    }

    let bytes = to_bytes(&chunk).unwrap();

    match region.write_chunk(0, 0, &bytes) {
        Ok(_) => {}
        Err(error) => panic!("Error writing chunk: {:?}", error)
    };
}
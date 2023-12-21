use fastanvil::{Region};
use fastnbt::{Value};
use fastnbt::from_bytes;
use fastnbt::to_bytes;
use std::fs::File;
use std::collections::HashMap;

fn get_section_by_height(sections: &mut Vec<Value>, height: i32) -> &mut HashMap<String, Value> {
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

        if section_y == height {
            return section;
        }
    }
    panic!("Failed to get section with height");
}

fn plop_chunk(upper_region: &mut Region<File>, lower_region: &mut Region<File>, x: usize, z: usize) {
    let data = upper_region.read_chunk(x, z).unwrap().unwrap();
    let mut chunk: HashMap<String, Value> = from_bytes(data.as_slice()).unwrap();

    let lower_data = lower_region.read_chunk(x, z).unwrap().unwrap();
    let mut lower_chunk: HashMap<String, Value> = from_bytes(lower_data.as_slice()).unwrap();

    let sections: &mut Vec<Value> = match chunk.get_mut("sections").unwrap() {
        Value::List(list) => list,
        _ => panic!("Failed to get sections")
    };

    // println!("Num sections: {:?}", sections.len());

    let lower_sections: &mut Vec<Value> = match lower_chunk.get_mut("sections").unwrap() {
        Value::List(list) => list,
        _ => panic!("Failed to get sections")
    };

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

        // println!("Section y: {:?}", section_y);

        if section_y < 0 {
            let lower_section: &mut HashMap<String, Value> = get_section_by_height(lower_sections, section_y);
            std::mem::swap(section, lower_section);
        } else if section_y == 0 {
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
                    name.replace_range(.., "minecraft:stone");
                }
            }
        } else {
            continue;
        }

        // println!("{:?}", section);
    }

    let bytes = to_bytes(&chunk).unwrap();

    match upper_region.write_chunk(x, z, &bytes) {
        Ok(_) => {}
        Err(error) => panic!("Error writing chunk: {:?}", error)
    };
}

fn plop_region(upper: String, lower: String) {
    let file: File = File::options().read(true).write(true).open(upper).unwrap();

    let mut region: Region<File> = Region::from_stream(file).unwrap();

    let lower_file: File = File::open(lower).unwrap();
    let mut lower_region: Region<File> = Region::from_stream(lower_file).unwrap();

    for z in 0..32 {
        for x in 0..32 {
            plop_chunk(&mut region, &mut lower_region, x, z);
        }
    }
}

fn main() {
    plop_region("regions/upper.mca".into(), "regions/lower.mca".into());
}
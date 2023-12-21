use std::collections::HashMap;
use std::fs;

fn get_section_by_height(sections: &mut Vec<fastnbt::Value>, height: i32) -> &mut HashMap<String, fastnbt::Value> {
    for section_value in sections {
        let section: &mut HashMap<String, fastnbt::Value> = match section_value {
            fastnbt::Value::Compound(compound) => compound,
            _ => panic!("Failed to get section")
        };

        let section_y: i32 = match section["Y"] {
            fastnbt::Value::Byte(num) => num.into(),
            fastnbt::Value::Int(num) => num,
            _ => panic!("Failed to get Y value")
        };

        if section_y == height {
            return section;
        }
    }
    panic!("Failed to get section with height");
}

fn plop_chunk(upper_region: &mut fastanvil::Region<fs::File>, lower_region: &mut fastanvil::Region<fs::File>, x: usize, z: usize) {    
    // Read the raw, serialized chunk data for the lower chunk
    let lower_data = lower_region.read_chunk(x, z).unwrap().unwrap();
    // Parse the lower chunk nbt into a HashMap
    let mut lower_chunk: HashMap<String, fastnbt::Value> = fastnbt::from_bytes(lower_data.as_slice()).unwrap();
    
    // ----------------

    // Read the raw, serialized chunk data for the upper chunk
    let upper_data = upper_region.read_chunk(x, z).unwrap().unwrap();
    // Parse the upper chunk nbt into a HashMap
    let mut upper_chunk: HashMap<String, fastnbt::Value> = fastnbt::from_bytes(upper_data.as_slice()).unwrap();


    // ----------------

    // Get the section


    let lower_sections: &mut Vec<fastnbt::Value> = match lower_chunk.get_mut("sections").unwrap() {
        fastnbt::Value::List(list) => list,
        _ => panic!("Failed to get sections")
    };

    let upper_sections: &mut Vec<fastnbt::Value> = match upper_chunk.get_mut("sections").unwrap() {
        fastnbt::Value::List(list) => list,
        _ => panic!("Failed to get sections")
    };

    // Loop over all the sections in upper region file
    for upper_section_value in upper_sections {

        let upper_section: &mut HashMap<String, fastnbt::Value> = match upper_section_value {
            fastnbt::Value::Compound(compound) => compound,
            _ => panic!("Failed to get section")
        };
        let upper_section_y = match upper_section["Y"] {
            fastnbt::Value::Byte(num) => num.into(),
            fastnbt::Value::Int(num) => num,
            _ => panic!("Failed to get Y value")
        };

        // println!("Section y: {:?}", section_y);

        if upper_section_y < 0 {
            let lower_section: &mut HashMap<String, fastnbt::Value> = get_section_by_height(lower_sections, upper_section_y);
            std::mem::swap(upper_section, lower_section);

        } else if upper_section_y == 0 {
            let block_states: &mut HashMap<String, fastnbt::Value> = match upper_section.get_mut("block_states").unwrap() {
                fastnbt::Value::Compound(compound) => compound,
                _ => panic!("Failed to get block states")
            };
    
            let palette: &mut Vec<fastnbt::Value> = match block_states.get_mut("palette").unwrap() {
                fastnbt::Value::List(list) => list,
                _ => panic!("Failed to get palette")
            };
    
            for block_value in palette {
                let block: &mut HashMap<String, fastnbt::Value> = match block_value {
                    fastnbt::Value::Compound(compound) => compound,
                    _ => panic!("Failed to get block")
                };
    
                let name: &mut String = match block.get_mut("Name").unwrap() {
                    fastnbt::Value::String(string) => string,
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

    let bytes = fastnbt::to_bytes(&upper_chunk).unwrap();

    match upper_region.write_chunk(x, z, &bytes) {
        Ok(_) => {}
        Err(error) => panic!("Error writing chunk: {:?}", error)
    };
}

pub fn plop_region(upper: &str, lower: &str) {
    let file = fs::File::options().read(true).write(true).open(upper).unwrap();

    let mut region = fastanvil::Region::from_stream(file).unwrap();

    let lower_file = fs::File::open(lower).unwrap();
    let mut lower_region = fastanvil::Region::from_stream(lower_file).unwrap();

    for z in 0..32 {
        for x in 0..32 {
            plop_chunk(&mut region, &mut lower_region, x, z);
        }
    }
}
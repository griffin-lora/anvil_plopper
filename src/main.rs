use fastanvil::{Region};
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

    let sections: &Vec<Value> = match &chunk["sections"] {
        Value::List(list) => list,
        _ => panic!("Failed to get sections")
    };

    println!("Num sections: {:?}", sections.len());

    for section_value in sections.iter() {
        let section: &HashMap<String, Value> = match section_value {
            Value::Compound(compound) => compound,
            _ => panic!("Failed to get section")
        };

        let section_y: i32 = match section["Y"] {
            Value::Byte(num) => num.into(),
            Value::Int(num) => num,
            _ => panic!("Failed to get Y value")
        };
        println!("Section");
        println!("Y position: {:?}", section_y);
    }

    let bytes = to_bytes(&chunk).unwrap();

    match region.write_chunk(0, 0, &bytes) {
        Ok(_) => {}
        Err(error) => panic!("Error writing chunk: {:?}", error)
    };
}
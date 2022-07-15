mod data;
mod input;
pub use data::*;
use dialoguer::Select;

const CHOICES: [&str; 4] = [
    "New Stop",
    "New Switch",
    "New Train",
    "Print Map As Debug Text",
];

const FILE_LOCATION: &'static str = "map.freds";

fn main() {
    loop {
        use freds::Reader;
        let reader: Reader<Map> = Reader::from_file(FILE_LOCATION).await.unwrap();
        let mut map: Map = reader.get(reader.core).await.unwrap();

        input(&mut map);

        use {
            freds::Write,
            std::{fs::File, io::Write as IoWrite},
        };
        let writer = map.write();
        let mut buffer = File::create(FILE_LOCATION).unwrap();
        buffer.write(&writer).unwrap();
    }
}
fn input(map: &mut Map) {
    println!();
    match CHOICES[Select::new().items(&CHOICES).default(0).interact().unwrap()] {
        "New Stop" => {
            let mut stop = Stop::default();
            stop.input();
            map.stops.push(stop);
        }
        "New Switch" => unimplemented!(),
        "New Train" => unimplemented!(),
        "Print Map As Debug Text" => println!("{:?}", map),
        _ => unreachable!(),
    }
}

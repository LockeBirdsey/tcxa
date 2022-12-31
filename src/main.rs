use std::fs;
use std::fs::File;
use std::io::prelude::*;

use std::str;

use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::io::Cursor;

fn main() {
    println!("Hello, world!");

    // dev vars
    let dist = 7512;

    // load file
    let file_path = "test-mini.tcx";
    let mut file = match File::create("test-mini-out.tcx") {
        Err(why) => panic!("couldn't create {}: {}", "test-mini-out.tcx", why),
        Ok(file) => file,
    };
    let xml = fs::read_to_string(file_path).expect("File file_path could not be read");

    // deser the xml file
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    // States
    let mut ignore: bool = false;
    let mut trackpoint: bool = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"Position" => {
                ignore = true;
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"Position" => {
                ignore = false;
            }

            Ok(Event::Start(e)) if e.name().as_ref() == b"DistanceMeters" => {
                ignore = true;
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"DistanceMeters" => {
                ignore = false;
            }
            Ok(Event::Eof) => break,

            Ok(e) => {
                if trackpoint {}
                if !ignore {
                    assert!(writer.write_event(e).is_ok())
                }
            }
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
    }
    let result = writer.into_inner().into_inner();
    let _s = match str::from_utf8(result.as_ref()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    match file.write_all(result.as_ref()) {
        Err(why) => panic!("Couldn't write {}", why),
        Ok(_) => println!("Written successfully"),
    }
    // println!("result: {}", s);
}

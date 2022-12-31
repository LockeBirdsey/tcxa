use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::Writer;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::str;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DistanceMeters {
    #[serde(rename = "$text")]
    pub body: String,
}
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
    let mut distanceMeters:bool = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"Position" => {
                ignore = true;
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"Position" => {
                ignore = false;
            }

            Ok(Event::Start(e)) if e.name().as_ref() == b"Trackpoint" => {
                trackpoint = true;
                assert!(writer.write_event(Event::Start(e.into_owned())).is_ok());
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"Trackpoint" => {
                trackpoint = false;
                assert!(writer.write_event(Event::End(e.into_owned())).is_ok());
            }

            Ok(Event::Start(e)) if e.name().as_ref() == b"DistanceMeters" => {
                if trackpoint {
                    // ignore
                    distanceMeters = true;
                } else {
                    assert!(writer.write_event(Event::Start(e.into_owned())).is_ok());
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"DistanceMeters" => {
                if trackpoint {
                    // ignore
                    distanceMeters = false;
                } else {
                    assert!(writer.write_event(Event::End(e.into_owned())).is_ok());
                }
            }
            Ok(Event::Text(e)) => {
                if (distanceMeters || ignore){
                    // skip
                }else {
                    assert!(writer.write_event(Event::Text(e.into_owned())).is_ok());
                }
            }
            Ok(Event::Eof) => break,

            Ok(e) => {
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

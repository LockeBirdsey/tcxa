use clap::Parser;
use quick_xml::events::{BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::Writer;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::str;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to input file
    #[arg(short, long)]
    input_path: std::path::PathBuf,

    /// Path to output file
    #[arg(short, long)]
    output_path: std::path::PathBuf,

    /// Manually inputted distance
    #[arg(short, long, default_value_t = 0)]
    distance: u32,

    /// Calculate total distance
    #[arg(short, long, default_value_t = false)]
    sum_distance: bool,

    /// calculate mean heart rate
    #[arg(short, long, default_value_t = false)]
    mhr: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DistanceMeters {
    #[serde(rename = "$text")]
    pub body: String,
}
fn main() {
    let args = Args::parse();

    let dist = args.distance;

    // TODO validate input and output files
    let outpath = match args.output_path.to_str() {
        Some(x) => x,
        None => panic!("No output path specified")
    };

    // load file
    let file_path = args.input_path;
    let mut file = match File::create(outpath) {
        Err(why) => panic!("couldn't create {}: {}", outpath, why),
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
    let mut distance_meters: bool = false;

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
                    ignore = true;
                } else {
                    // new val
                    distance_meters = true;
                    let dstr = dist.to_string(); // lazy owning
                    let x = BytesText::new(&dstr);
                    assert!(writer.write_event(Event::Start(e.into_owned())).is_ok());
                    assert!(writer.write_event(Event::Text(x)).is_ok());
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"DistanceMeters" => {
                if trackpoint {
                    // ignore
                    ignore = false;
                } else {
                    distance_meters = false;
                    assert!(writer.write_event(Event::End(e.into_owned())).is_ok());
                }
            }
            Ok(Event::Text(e)) => {
                if distance_meters || ignore {
                    // skip
                } else {
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

    match file.write_all(result.as_ref()) {
        Err(why) => panic!("Couldn't write {}", why),
        Ok(_) => println!("Written successfully"),
    }

    // if you want it to stdout
    let _s = match str::from_utf8(result.as_ref()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    // println!("result: {}", s);
}

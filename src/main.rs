#![allow(non_snake_case)]
use std::io::BufReader;
use std::path::Path;
use std::{fs::File, io::Read};
mod aimless;
use aimless::{Aimless, AimlessFields, CCP4Table};
use quick_xml::de::from_reader;
use std::time::Instant;
fn main() {
    let aimless_xml: &str =
        "/Users/aaronfinke/Documents/edna2report_rust/example_files/aimless_temp.xml";
    let _pointless_xml: &str =
        "/Users/aaronfinke/Documents/edna2report_rust/example_files/pointless.xml";
    let _ctruncate_log: &str =
        "/Users/aaronfinke/Documents/edna2report_rust/example_files/ctruncate.log";
    let _integrate_lp: &str =
        "/Users/aaronfinke/Documents/edna2report_rust/example_files/INTEGRATE.LP";
    let _xdsstat_lp: &str = "/Users/aaronfinke/Documents/edna2report_rust/example_files/XDSSTAT.LP";
    let _xtriage_json: &str =
        "/Users/aaronfinke/Documents/edna2report_rust/example_files/outDataMmtbxXtriageTask.json";
    let t1 = Instant::now();
    let path = Path::new(aimless_xml);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // let x = &mut quick_xml::de::Deserializer::from_reader(reader);
    // let result: Result<Aimless, _> = serde_path_to_error::deserialize(x);
    let result: Result<Aimless, _> = from_reader(reader);
    let aimless = match result {
        Ok(c) => c,
        Err(err) => {
            panic!("{}", err);
        }
    };
    // dbg!(aimless.fields);
    let ccp4tables: Vec<&CCP4Table> = aimless
        .fields
        .iter()
        .filter_map(|f| match f {
            AimlessFields::CCP4Table(m) => Some(m),
            _ => None,
        })
        .collect();
    dbg!(&aimless);
    println!("number of ccp4tables: {}", ccp4tables.len());
    println!("time: {:?}", t1.elapsed());
}

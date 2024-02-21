#![allow(non_snake_case)]
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
mod aimless;
use aimless::Aimless;
use quick_xml::de::DeError;
fn main() {
    let aimless_xml :&str = "/Users/aaronfinke/Documents/edna2report_rust/example_files/aimless.xml";
    let _pointless_xml :&str = "/Users/aaronfinke/Documents/edna2report_rust/example_files/pointless.xml";
    let _ctruncate_log :&str = "/Users/aaronfinke/Documents/edna2report_rust/example_files/ctruncate.log";
    let _integrate_lp :&str = "/Users/aaronfinke/Documents/edna2report_rust/example_files/INTEGRATE.LP";
    let _xdsstat_lp :&str = "/Users/aaronfinke/Documents/edna2report_rust/example_files/XDSSTAT.LP";
    let _xtriage_json :&str = "/Users/aaronfinke/Documents/edna2report_rust/example_files/outDataMmtbxXtriageTask.json";

    let path = Path::new(aimless_xml);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let x = &mut quick_xml::de::Deserializer::from_reader(reader);
    let result: Result<Aimless, _> = serde_path_to_error::deserialize(x);
    let y = match result {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{}",err);
            let path = err.path().to_string();
            panic!("{}",path);
        }
    };
    dbg!(y);
}

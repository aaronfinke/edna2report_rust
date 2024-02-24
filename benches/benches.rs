use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/aimless.rs"]
mod aimless;
use aimless::{Aimless, AimlessFields};

use quick_xml::de::from_str;
use std::path::Path;
use std::{fs::File, io::Read};
fn criterion_benchmark(c: &mut Criterion) {
    let aimless_xml: &str =
        "/Users/aaronfinke/Documents/edna2report_rust/example_files/aimless.xml";
    let path = Path::new(aimless_xml);
    let mut buffer = String::new();
    let mut file = File::open(aimless_xml).unwrap();
    file.read_to_string(&mut buffer).unwrap();
    buffer = buffer.replace('\n', "");
    let file_str = std::fs::read_to_string(path).unwrap();

    c.bench_function("roxmltree", |b| {
        b.iter(|| {
            black_box(roxmltree::Document::parse(&buffer).unwrap());
        })
    });
    c.bench_function("serde quick_xml", |b| {
        b.iter(|| {
            let x: Aimless = black_box(from_str(&file_str).unwrap());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/aimless.rs"] mod aimless;
use aimless::Aimless;
use std::path::Path;
use quick_xml::de::from_str;
fn criterion_benchmark(c: &mut Criterion) {
	let aimless_xml :&str = "/Users/aaronfinke/Documents/edna2report_rust/example_files/aimless.xml";

    c.bench_function("convert to tuple", |b| b.iter(|| {
            let path = Path::new(aimless_xml);
            let file = std::fs::read_to_string(path).unwrap();
            let x:Aimless = black_box(from_str(&file).unwrap());
            x
    }));


}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use dxf2image::dxf2svg;

fn main() {
    let start = std::time::Instant::now();

    println!("dxf2svg started");
    let dxf = "./examples/dxf/sample.dxf";
    let svg = "./examples/hoge.svg";
    dxf2svg(dxf, svg).unwrap();

    let end = start.elapsed();
    println!("finish dxf2svg. took {}ms", end.as_millis());
}

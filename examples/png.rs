use dxf2image::dxf2png;

fn main() {
    let start = std::time::Instant::now();

    println!("dxf2png started");
    let dxf = "./examples/dxf/sample.dxf";
    let png = "./examples/hoge.png";
    dxf2png(dxf, png).unwrap();

    let end = start.elapsed();
    println!("finish dxf2png. took {} ms", end.as_millis());
}

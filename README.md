# dxf2image

dxf2image is a fast and efficient dxf to image converter!

## Installation

Put the following in your `Cargo.toml`
``` toml
[dependencies]
dxf2image = "0.1"
```

If you want png, please add `png` to the features
``` toml
[dependencies]
dxf2image = { version = "0.1", features = ["png"] }
```

## Usage

``` rust
use dxf2image::{dxf2svg, dxf2png};

fn main() {
    let dxf = "sample.dxf";

    // Convert to svg
    let svg = "sample.svg";
    dxf2svg(dxf, svg).unwrap();

    // Convert to png
    let png = "sample.png";
    dxf2png(dxf, png).unwrap();
}
```

## Note

For supported dxf, see [dxf-rs](https://github.com/ixmilia/dxf-rs).

## License

Distributed under the Apache License. See LICENSE for more information.

use crate::color::DxfColor;
use crate::coord::{Coord, PointConverter};
use dxf::entities::EntityType;
use svg::{
    document::Document,
    nodes::{Circle, Line, Polyline},
};

/// Convert dxf to svg
pub fn dxf2svg(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    let drawing = dxf::Drawing::load_file(input_path)?;

    let extmax = drawing.header.maximum_drawing_extents.clone();
    let extmin = drawing.header.minimum_drawing_extents.clone();
    let coord = Coord::new(extmax, extmin);

    let mut document = Document::new().viewbox(
        coord.origin().0,
        coord.origin().1,
        coord.width(),
        coord.height(),
    );

    for entity in drawing.entities() {
        let common = entity.common.clone();
        let color = hex_color(&drawing, &common);

        match &entity.specific {
            EntityType::LwPolyline(polyline) => {
                let polyline = polyline.clone();
                let points: Vec<(f64, f64)> = polyline
                    .vertices
                    .into_iter()
                    .map(|vertex| (coord.relative_to((vertex.x.clone(), vertex.y.clone()))))
                    .collect();
                let node = Polyline::new().points(points).stroke(&color);
                document.add(node);
            }
            EntityType::Line(line) => {
                let line = line.clone();
                let from = coord.relative_to(line.p1);
                let to = coord.relative_to(line.p2);
                let node = Line::new().points(from, to).stroke(&color);
                document.add(node);
            }
            EntityType::Leader(leader) => {
                // TODO: use_arrowheads, spline
                let leader = leader.clone();
                let points = leader
                    .vertices
                    .into_iter()
                    .map(|p| coord.relative_to(p))
                    .collect();
                let node = Polyline::new().points(points).stroke(&color);
                document.add(node);
            }
            EntityType::Circle(circle) => {
                let circle = circle.clone();
                let node = Circle::new()
                    .center(coord.relative_to(circle.center))
                    .radius(coord.relative_to(circle.radius))
                    .stroke(&color);
                document.add(node);
            }
            _ => {
                // TODO: support more entities;
            }
        }
    }

    document.save(output_path)
}

fn hex_color(drawing: &dxf::Drawing, common: &dxf::entities::EntityCommon) -> String {
    let color_idx = if common.color.is_by_layer() {
        drawing
            .layers()
            .find(|layer| layer.name == common.layer)
            .map(|layer| layer.color.index())
            .flatten()
    } else {
        common.color.index()
    };
    DxfColor::from(color_idx.unwrap_or(0)).as_hex()
}

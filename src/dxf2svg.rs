use crate::color::DxfColor;
use crate::coord::{Coord, PointConverter};
use dxf::entities::{Entity, EntityType};
use dxf::enums::AngleDirection;
use svgx::{
    document::Document,
    nodes::{Circle, Line, Path, Polyline},
};

/// Convert dxf to svg
pub fn dxf2svg(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    let drawing = dxf::Drawing::load_file(input_path)?;

    let extmax = drawing.header.maximum_drawing_extents.clone();
    let extmin = drawing.header.minimum_drawing_extents.clone();
    let coord = Coord::new(extmax, extmin, Some(3000.0));

    let mut document = Document::new().viewbox(0.0, 0.0, coord.width(), coord.height());

    for entity in drawing.entities() {
        entity_to_node(&mut document, &drawing, &coord, &entity);
    }

    document.save(output_path)
}

fn entity_to_node(document: &mut Document, drawing: &dxf::Drawing, coord: &Coord, entity: &Entity) {
    let common = entity.common.clone();

    let should_draw = drawing
        .layers()
        .find(|layer| layer.name == common.layer)
        .map(|layer| layer.is_layer_plotted && layer.is_layer_on)
        .unwrap_or(false);
    if !should_draw {
        return;
    }

    let color = hex_color(&drawing, &common);
    let line_weight = coord.relative_to(line_weight(drawing, &common) as f64);

    match &entity.specific {
        EntityType::LwPolyline(polyline) => {
            let polyline = polyline.clone();
            let points: Vec<(f64, f64)> = polyline
                .vertices
                .into_iter()
                .map(|vertex| (coord.relative_to((vertex.x.clone(), vertex.y.clone()))))
                .collect();
            let node = Polyline::new().points(points).stroke(&color).stroke_width(line_weight);
            document.add(node);
        }
        EntityType::Line(line) => {
            let line = line.clone();
            let from = coord.relative_to(line.p1);
            let to = coord.relative_to(line.p2);
            let node = Line::new().points(from, to).stroke(&color).stroke_width(line_weight);
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
            let node = Polyline::new().points(points).stroke(&color).stroke_width(line_weight);
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
        EntityType::Arc(arc) => {
            let arc = arc.clone();
            let center = coord.relative_to(arc.center);
            let radius = coord.relative_to(arc.radius);
            let is_clockwise = drawing.header.angle_direction == AngleDirection::Clockwise;
            let node = Path::arc(center, radius, arc.start_angle, arc.end_angle, is_clockwise)
                .stroke(&color)
                .stroke_width(line_weight);
            document.add(node);
        }
        EntityType::Insert(insert) => {
            if let Some(block) = drawing.blocks().find(|block| block.name == insert.name) {
                let mut coord = coord.clone();
                let base_point = (insert.location.x, insert.location.y);
                coord.set_base_point(base_point);
                for entity in block.entities.clone() {
                    entity_to_node(document, &drawing, &coord, &entity);
                }
            }
        }
        _ => {}
    }
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

fn line_weight(drawing: &dxf::Drawing, common: &dxf::entities::EntityCommon) -> i16 {
    let by_layer = common.lineweight_enum_value == 0;
    let line_weight = if by_layer {
        drawing
            .layers()
            .find(|layer| layer.name == common.layer)
            .map(|layer| layer.line_weight.to_owned().get_raw_value())
            .unwrap_or_default()
    } else {
        common.lineweight_enum_value
    };
    line_weight.max(1)
}

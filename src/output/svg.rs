use image::GrayImage;
use image::Luma;

use output::png::draw_maze;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::node::element::Style;

use svg::Document;
use types::xy::Xy;

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

fn draw_paths<T>(
    document: Document,
    grid: &Grid<T>,
    cell_size: u32,
    wall_size: u32,
    external_doors: bool,
) -> Document
where
    T: Cell + Clone,
{
    let img_x = (grid.x() as u32 * cell_size) + (grid.x() as u32 + 1) * wall_size;
    let img_y = (grid.y() as u32 * cell_size) + (grid.y() as u32 + 1) * wall_size;

    let mut img = GrayImage::new(img_x, img_y);
    let background_color = Luma([0]);
    let wall_color = Luma([1]);

    draw_maze(
        &mut img,
        grid,
        cell_size,
        wall_size,
        background_color,
        wall_color,
        external_doors,
    );

    let mut bits = Vec::<Vec<i8>>::with_capacity(img.height() as usize);
    for row in img.rows() {
        bits.push(row.map(|p| (p == &wall_color) as i8).collect());
    }

    let path = contour_tracing::bits_to_paths(bits, true);
    let data = Data::parse(&path).unwrap();
    let path = Path::new().set("class", "wall").set("d", data);

    document.add(path)
}

/// Adds multiple SVG Rectangles to the document to draw the maze.
fn draw_rects<T>(
    mut document: Document,
    grid: &Grid<T>,
    cell_size: u32,
    wall_size: u32,
    external_doors: bool,
) -> Document
where
    T: Cell + Clone,
{
    // Draw Top Wall
    if external_doors && grid.start.y() == 0 {
        // Draw left of the starting door
        if grid.start.x() > 0 {
            let end_x = (grid.start.x() as u32) * (cell_size + wall_size);
            document = document.add(
                Rectangle::new()
                    .set("x", 0)
                    .set("y", 0)
                    .set("width", end_x)
                    .set("height", wall_size)
                    .set("class", "wall"),
            );
        }

        if grid.start.x() < grid.x() - 1 {
            // Draw right of the starting door
            let start_x = (grid.start.x() as u32 + 1) * (cell_size + wall_size);
            let end_x = (grid.x() as u32) * (cell_size + wall_size);
            document = document.add(
                Rectangle::new()
                    .set("x", start_x)
                    .set("y", 0)
                    .set("width", end_x - start_x + wall_size)
                    .set("height", wall_size)
                    .set("class", "wall"),
            );
        }
    } else {
        let img_x = (grid.x() as u32 * cell_size) + (grid.x() as u32 + 1) * wall_size;
        document = document.add(
            Rectangle::new()
                .set("x", 0)
                .set("y", 0)
                .set("width", img_x)
                .set("height", wall_size)
                .set("class", "wall"),
        );
    }

    // Left Wall
    let img_y = (grid.y() as u32 * cell_size) + (grid.y() as u32 + 1) * wall_size;
    document = document.add(
        Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", wall_size)
            .set("height", img_y)
            .set("class", "wall"),
    );

    // Draw all the vertical lines (as one long line between the maze doors)
    for x in 0..grid.x() {
        let start_x = (x as u32 + 1) * (cell_size + wall_size);

        let mut y = 0;
        while y < grid.y() {
            let right = grid.is_linked_indices(x, y, x + 1, y);
            if !right {
                let start_y = y as u32 * (cell_size + wall_size);

                // Draw until we get to the gap
                let mut last_y = grid.y();
                while y < last_y {
                    y += 1;
                    if grid.is_linked_indices(x, y, x + 1, y) {
                        last_y = y;
                        break;
                    }
                }

                let end_y = last_y as u32 * (cell_size + wall_size);
                document = document.add(
                    Rectangle::new()
                        .set("x", start_x)
                        .set("y", start_y)
                        .set("width", wall_size)
                        .set("height", end_y - start_y + wall_size)
                        .set("class", "wall"),
                );
            }

            y += 1;
        }
    }

    // Draw all the horiztonal lines (as one long line between the maze doors)
    for y in 0..grid.y() {
        let start_y = (y as u32 + 1) * (cell_size + wall_size);

        let mut x = 0;
        while x < grid.x() {
            let bottom = grid.is_linked_indices(x, y, x, y + 1)
                || (external_doors && grid.end == Xy::new(x, y));
            if !bottom {
                let start_x = x as u32 * (cell_size + wall_size);

                // Draw until we get to the gap
                let mut last_x = grid.x();
                while x < last_x {
                    x += 1;
                    if grid.is_linked_indices(x, y, x, y + 1)
                        || (external_doors && grid.end == Xy::new(x, y))
                    {
                        last_x = x;
                        break;
                    }
                }

                let end_x = last_x as u32 * (cell_size + wall_size);
                document = document.add(
                    Rectangle::new()
                        .set("x", start_x)
                        .set("y", start_y)
                        .set("width", end_x - start_x + wall_size)
                        .set("height", wall_size)
                        .set("class", "wall"),
                );
            }

            x += 1;
        }
    }
    document
}

pub fn format<T>(
    grid: &Grid<T>,
    cell_size: u32,
    wall_size: u32,
    color_cell: &[u8; 3],
    color_wall: &[u8; 3],
    external_doors: bool,
    output_filename: &'static str,
) -> std::io::Result<()>
where
    T: Cell + Clone,
{
    let img_x = (grid.x() as u32 * cell_size) + (grid.x() as u32 + 1) * wall_size;
    let img_y = (grid.y() as u32 * cell_size) + (grid.y() as u32 + 1) * wall_size;

    let style = format!(
        r#"
        svg {{
            background-color: rgba({},{},{},1.0);
        }}

        .wall {{
            background-color: rgba({},{},{},1.0);
        }}
    "#,
        color_cell[0], color_cell[1], color_cell[2], color_wall[0], color_wall[1], color_wall[2]
    );

    let mut document = Document::new()
        .set("width", img_x)
        .set("height", img_y)
        .set("viewBox", (0, 0, img_x, img_y))
        .add(Style::new(style));

    info!(
        "Generating {:?}, size: {}x{} px",
        output_filename, img_x, img_y
    );

    // Two options
    // 1. Use a pure SVG solution (that produces lots of rects)
    // 2. Use a path tracing approach (that produces a couple of paths)
    if cfg!(feature = "fast-svg") {
        document = draw_rects(document, grid, cell_size, wall_size, external_doors);
    } else {
        document = draw_paths(document, grid, cell_size, wall_size, external_doors);
    }

    svg::save(output_filename, &document)
}

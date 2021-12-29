extern crate image;
extern crate imageproc;

use image::Pixel;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use std::ops::Deref;
use std::ops::DerefMut;
use types::xy::Xy;

// use rand;
// use rand::distributions::{Sample, Range};

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub(crate) fn draw_maze<T, P: Pixel, Container>(
    img: &mut image::ImageBuffer<P, Container>,
    grid: &Grid<T>,
    cell_size: u32,
    wall_size: u32,
    background_color: P,
    wall_color: P,
    external_doors: bool,
) where
    T: Cell + Clone,
    P: Pixel + 'static,
    P::Subpixel: 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    let img_x = img.width();
    let img_y = img.height();

    // Background
    draw_filled_rect_mut(img, Rect::at(0, 0).of_size(img_x, img_y), background_color);

    // Left
    for y in 0..grid.y() {
        let left = grid.is_linked_indices(0, y, grid.x() - 1, y);
        if !left {
            let start_y = y as i32 * cell_size as i32 + y as i32 * wall_size as i32;
            let size_x = wall_size;
            let size_y = (cell_size + 2 * wall_size) as u32;

            debug!(
                "left: ({}, {}), start: ({}, {}), size({}, {})",
                0, y, 0, start_y, size_x, size_y
            );
            draw_filled_rect_mut(
                img,
                Rect::at(0, start_y).of_size(size_x, size_y),
                wall_color,
            );
        }
    }

    // Top
    for x in 0..grid.x() {
        let top = grid.is_linked_indices(x, 0, x, grid.y() - 1) || (external_doors && grid.end == Xy::new(x, 0));
        if !top {
            let start_x = x as i32 * cell_size as i32 + x as i32 * wall_size as i32;
            let size_x = (cell_size + 2 * wall_size) as u32;
            let size_y = wall_size;
            debug!(
                "top: ({}, {}), start: ({}, {}), size({}, {})",
                x, 0, start_x, 0, size_x, size_y
            );
            draw_filled_rect_mut(
                img,
                Rect::at(start_x, 0).of_size(size_x, size_y),
                wall_color,
            );
        }
    }

    // Cells
    for x in 0..grid.x() {
        for y in 0..grid.y() {
            let cell = &grid[x][y];

            // Right - Vertical
            let right = grid.is_linked_indices(cell.x(), cell.y(), (cell.x() + 1).rem_euclid(grid.x()), cell.y());
            if !right {
                let start_x = (x + 1) as i32 * cell_size as i32 + (x + 1) as i32 * wall_size as i32;
                let start_y = y as i32 * cell_size as i32 + y as i32 * wall_size as i32;
                let size_x = wall_size;
                let size_y = (cell_size + 2 * wall_size) as u32;
                debug!(
                    "right: ({}, {}), start: ({}, {}), size({}, {})",
                    x, y, start_x, start_y, size_x, size_y
                );
                draw_filled_rect_mut(
                    img,
                    Rect::at(start_x, start_y).of_size(size_x, size_y),
                    wall_color,
                );
            }

            // Bottom - Horizontal
            let bottom = grid.is_linked_indices(cell.x(), cell.y(), cell.x(), (cell.y() + 1).rem_euclid(grid.y()))
                || (external_doors && grid.end == Xy::new(x, y));
            if !bottom {
                let start_x = x as i32 * cell_size as i32 + x as i32 * wall_size as i32;
                let start_y = (y + 1) as i32 * cell_size as i32 + (y + 1) as i32 * wall_size as i32;
                let size_x = (cell_size + 2 * wall_size) as u32;
                let size_y = wall_size;
                debug!(
                    "bottom: ({}, {}), start: ({}, {}), size({}, {})",
                    x, y, start_x, start_y, size_x, size_y
                );
                draw_filled_rect_mut(
                    img,
                    Rect::at(start_x, start_y).of_size(size_x, size_y),
                    wall_color,
                );
            }
        }
    }
}

pub fn format<T>(
    grid: &Grid<T>,
    cell_size: u32,
    wall_size: u32,
    color_cell: &[u8; 3],
    color_wall: &[u8; 3],
    external_doors: bool,
    output_filename: &'static str,
) where
    T: Cell + Clone,
{
    let img_x = (grid.x() as u32 * cell_size) + (grid.x() as u32 + 1) * wall_size;
    let img_y = (grid.y() as u32 * cell_size) + (grid.y() as u32 + 1) * wall_size;

    info!(
        "Generating {:?}, size: {}x{} px",
        output_filename, img_x, img_y
    );

    let mut img = RgbImage::new(img_x, img_y);
    let background_color = Rgb(*color_cell);
    let wall_color = Rgb(*color_wall);

    draw_maze(
        &mut img,
        grid,
        cell_size,
        wall_size,
        background_color,
        wall_color,
        external_doors,
    );
    img.save(output_filename).unwrap();
}

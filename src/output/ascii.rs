use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn format<T>(grid: &Grid<T>) -> String
where
    T: Cell + Clone,
{
    let mut res = String::new();

    // Top line
    res += "+";
    for x in 0..grid.x() {
        match grid.is_linked_indices(x, 0, x, grid.y() - 1) {
            true => res += "   +",
            false => res += "---+",
        };
    }
    res += "\n";

    for y in 0..grid.y() {
        let mut top = match grid.is_linked_indices(0, y, grid.x() - 1, y) {
            true => " ",
            false => "|",
        }
        .to_string();

        let mut bottom = "+".to_string();

        for x in 0..grid.x() {
            top += &grid.cells[x][y].to_string()[..];

            match grid.is_linked_indices(x, y, (x + 1).rem_euclid(grid.x()), y) {
                true => top += " ",
                false => top += "|",
            }

            match grid.is_linked_indices(x, y, x, (y + 1).rem_euclid(grid.y())) {
                true => bottom += "   +",
                false => bottom += "---+",
            }
        }

        res += &top[..];
        res += "\n";

        res += &bottom[..];
        res += "\n";
    }

    res
}

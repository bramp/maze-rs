use serde::{Deserialize, Serialize};

pub trait Cell {
    fn new(x: usize, y: usize) -> Self;
    fn to_string(&self) -> String;
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BaseCell {
    x: usize,
    y: usize,
}

const EMPTY_CELL: &str = "   ";

impl Cell for BaseCell {
    fn new(x: usize, y: usize) -> BaseCell {
        BaseCell { x, y }
    }

    fn to_string(&self) -> String {
        EMPTY_CELL.to_string()
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

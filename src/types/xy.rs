use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Xy {
    pub x: usize,
    pub y: usize,
}

impl Xy {
    pub fn new(x: usize, y: usize) -> Self {
        Xy { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl fmt::Display for Xy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl FromStr for Xy {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let x_fromstr = coords[0].parse::<usize>()?;
        let y_fromstr = coords[1].parse::<usize>()?;

        Ok(Xy {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use types::xy::Xy;

    #[test]
    fn from_str() {
        let p = Xy::from_str("(1,2)");
        assert_eq!(p.unwrap(), Xy::new(1, 2))
    }
}

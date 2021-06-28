use std::fmt;
use std::str::FromStr;

use DocumentError::InvalidAttribute;
use DocumentError::InvalidPoint;
use DocumentError::MissingAttribute;

use crate::colors::Color;
use crate::DocumentError;

use super::FromAttributes;

#[derive(PartialEq, Clone, Copy)]
pub struct Point(pub f32, pub f32, pub f32);

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{}:{})", self.0, self.1, self.2)
    }
}

#[derive(Debug)]
pub struct Line {
    pub color: Color,
    pub width: f32,
    pub points: Vec<Point>,
}

impl FromAttributes for Line {
    fn from_attributes(
        attributes: std::collections::HashMap<String, svg::node::Value>,
    ) -> Result<Self, crate::DocumentError> {
        Ok(Line {
            color: {
                let color: &str = attributes
                    .get("stroke")
                    .ok_or(MissingAttribute("stroke".to_owned()))?;
                Color::from_str(color)
                    .map_err(|_| InvalidAttribute("stroke".to_owned(), color.to_owned()))?
            },
            points: {
                let points: &str = attributes
                    .get("svgnote:points")
                    .ok_or(MissingAttribute("svgnote:points".to_owned()))?;
                points
                    .split_ascii_whitespace()
                    .map(|s| {
                        let a: Vec<&str> = s.split(',').collect();
                        if a.len() == 3 {
                            Ok(Point(
                                f32::from_str(a[0]).map_err(|_| InvalidPoint(s.to_owned()))?,
                                f32::from_str(a[1]).map_err(|_| InvalidPoint(s.to_owned()))?,
                                f32::from_str(a[2]).map_err(|_| InvalidPoint(s.to_owned()))?,
                            ))
                        } else {
                            Err(InvalidPoint(s.to_owned()))
                        }
                    })
                    .collect::<Result<_, _>>()?
            },
            width: {
                let width: &str = attributes
                    .get("svgnote:width")
                    .ok_or(MissingAttribute("svgnote:width".to_owned()))?;
                f32::from_str(width)
                    .map_err(|_| InvalidAttribute("svgnote:width".to_owned(), width.to_owned()))?
            },
        })
    }
}

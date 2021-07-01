use std::fmt;
use std::str::FromStr;

use svg::node::element;
use DocumentError::InvalidAttribute;
use DocumentError::InvalidPoint;
use DocumentError::MissingAttribute;

use crate::colors::Color;
use crate::elems_eq;
use crate::DocumentError;

use super::FromAttributes;

#[derive(PartialEq, Clone, Copy)]
pub struct Point(pub f32, pub f32, pub f32);

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{}:{})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}

impl From<Point> for (f32, f32) {
    fn from(val: Point) -> Self {
        (val.0, val.1)
    }
}

#[derive(Debug)]
pub struct Line {
    pub color: Color,
    pub width: f32,
    pub points: Vec<Point>,
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.width == other.width
            && elems_eq(&self.points, &other.points)
    }
}

impl From<&Line> for element::Path {
    fn from(line: &Line) -> Self {
        let d = line.points.iter().skip(1).fold(
            element::path::Data::new().move_to(
                line.points
                    .first()
                    .map(|p| -> (f32, f32) { (*p).into() })
                    .unwrap_or((0.0, 0.0)),
            ),
            |d, &p| d.line_to::<(f32, f32)>(p.into()),
        );
        element::Path::new()
            .set("stroke", line.color.to_string_na())
            .set("stroke-opacity", line.color.opacity())
            .set("stroke-width", line.width)
            .set("svgnote:width", line.width)
            .set(
                "svgnote:points",
                line.points
                    .iter()
                    .map(Point::to_string)
                    .collect::<Vec<String>>(),
            )
            // Static
            .set("svgnote:tool", "pen")
            .set("fill-opacity", "0")
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            // Generated
            .set("d", d)
    }
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
                    .map_err(|_| InvalidAttribute("stroke".to_owned(), color.to_owned()))
                    .map(|c| {
                        // TODO Give an Error on a malformed opacity maybe
                        if let Some(Ok(value)) =
                            attributes.get("stroke-opacity").map(|s| f32::from_str(s))
                        {
                            c.with_opacity(value)
                        } else {
                            c
                        }
                    })?
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

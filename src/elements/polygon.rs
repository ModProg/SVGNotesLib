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
pub struct PolylinePoint(pub f32, pub f32);

impl fmt::Debug for PolylinePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl PolylinePoint {
    pub fn distance_to(&self, other: Self) -> f32 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

impl fmt::Display for PolylinePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

impl From<PolylinePoint> for (f32, f32) {
    fn from(val: PolylinePoint) -> Self {
        (val.0, val.1)
    }
}

#[derive(Debug, Clone)]
pub struct Polyline {
    pub stroke: Color,
    pub fill: Color,
    pub width: f32,
    pub points: Vec<PolylinePoint>,
}

impl PartialEq for Polyline {
    fn eq(&self, other: &Self) -> bool {
        (self.stroke, self.fill, self.width) == (other.stroke, other.fill, other.width)
            && elems_eq(&self.points, &other.points)
    }
}

impl From<&Polyline> for element::Polyline {
    fn from(polygon: &Polyline) -> Self {
                element::Polyline::new()
                    .set("stroke", polygon.stroke.to_string_na())
                    .set("fill", polygon.fill.to_string_na())
                    .set("stroke-opacity", polygon.stroke.opacity())
                    .set("fill-opacity", polygon.fill.opacity())
                    .set("stroke-width", polygon.width)
                    .set(
                        "points",
                        polygon
                            .points
                            .iter()
                            .map(PolylinePoint::to_string)
                            .collect::<Vec<String>>(),
                    )
                    // Static
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
    }
}

impl FromAttributes for Polyline{
    fn from_attributes(
        attributes: std::collections::HashMap<String, svg::node::Value>,
    ) -> Result<Self, crate::DocumentError> {
        Ok(Polyline {
            stroke: {
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
            fill: {
                let color: &str = attributes
                    .get("fill")
                    .ok_or(MissingAttribute("fill".to_owned()))?;
                Color::from_str(color)
                    .map_err(|_| InvalidAttribute("fill".to_owned(), color.to_owned()))
                    .map(|c| {
                        // TODO Give an Error on a malformed opacity maybe
                        if let Some(Ok(value)) =
                            attributes.get("fill-opacity").map(|s| f32::from_str(s))
                        {
                            c.with_opacity(value)
                        } else {
                            c
                        }
                    })?
            },
            points: {
                let points: &str = attributes
                    .get("points")
                    .ok_or(MissingAttribute("points".to_owned()))?;
                points
                    .split_ascii_whitespace()
                    .map(|s| {
                        let a: Vec<&str> = s.split(',').collect();
                        if a.len() == 2 {
                            Ok(PolylinePoint(
                                f32::from_str(a[0]).map_err(|_| InvalidPoint(s.to_owned()))?,
                                f32::from_str(a[1]).map_err(|_| InvalidPoint(s.to_owned()))?,
                            ))
                        } else {
                            Err(InvalidPoint(s.to_owned()))
                        }
                    })
                    .collect::<Result<_, _>>()?
            },
            width: {
                let width: &str = attributes
                    .get("stroke-width")
                    .ok_or(MissingAttribute("stroke-width".to_owned()))?;
                f32::from_str(width)
                    .map_err(|_| InvalidAttribute("stroke-width".to_owned(), width.to_owned()))?
            },
        })
    }
}

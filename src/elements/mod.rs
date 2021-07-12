mod line;
mod polygon;

use std::collections::HashMap;
use std::f32::consts::PI;
use std::str::FromStr;

use crate::colors::Color;
use crate::DocumentError;

use derivative::Derivative;
use svg::node::element::{self, tag};
use svg::node::Value;

use svg::parser::Event;
use DocumentError::InvalidAttribute;
use DocumentError::MissingAttribute;

pub use self::line::Line;
pub use self::line::LinePoint;
pub use self::polygon::Polyline;
pub use self::polygon::PolylinePoint;

#[derive(Debug, PartialEq, Clone)]
pub struct Ngon {
    pub position: (f32, f32),
    pub stroke: Color,
    pub fill: Color,
    pub width: f32,
    pub angle: f32,
    pub n: u8,
    pub radius: f32,
}

impl FromAttributes for Ngon {
    fn from_attributes(attributes: HashMap<String, Value>) -> Result<Self, DocumentError> {
        Ok(Ngon {
            position: {
                let value: &str = attributes
                    .get("svgnote:position")
                    .ok_or(MissingAttribute("svgnote:position".to_owned()))?;
                value
                    .split_once(',')
                    .ok_or(())
                    .and_then(|v| {
                        if let (Ok(x), Ok(y)) = (f32::from_str(v.0), f32::from_str(v.1)) {
                            Ok((x, y))
                        } else {
                            Err(())
                        }
                    })
                    .map_err(|_| {
                        InvalidAttribute("svgnote:position".to_owned(), value.to_owned())
                    })?
            },
            width: {
                let value: &str = attributes
                    .get("stroke-width")
                    .ok_or(MissingAttribute("stroke-width".to_owned()))?;
                f32::from_str(value)
                    .map_err(|_| InvalidAttribute("stroke-width".to_owned(), value.to_owned()))?
            },
            radius: {
                let value: &str = attributes
                    .get("svgnote:radius")
                    .ok_or(MissingAttribute("svgnote:radius".to_owned()))?;
                f32::from_str(value)
                    .map_err(|_| InvalidAttribute("svgnote:radius".to_owned(), value.to_owned()))?
            },
            n: {
                let value: &str = attributes
                    .get("svgnote:n")
                    .ok_or(MissingAttribute("svgnote:n".to_owned()))?;
                u8::from_str(value)
                    .map_err(|_| InvalidAttribute("svgnote:n".to_owned(), value.to_owned()))?
            },
            angle: {
                let value: &str = attributes
                    .get("svgnote:angle")
                    .ok_or(MissingAttribute("svgnote:angle".to_owned()))?;
                f32::from_str(value)
                    .map_err(|_| InvalidAttribute("svgnote:angle".to_owned(), value.to_owned()))?
            },
            fill: {
                let value: &str = attributes
                    .get("fill")
                    .ok_or(MissingAttribute("fill".to_owned()))?;
                Color::from_str(value)
                    .map_err(|_| InvalidAttribute("fill".to_owned(), value.to_owned()))
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
            stroke: {
                let value: &str = attributes
                    .get("stroke")
                    .ok_or(MissingAttribute("stroke".to_owned()))?;
                Color::from_str(value)
                    .map_err(|_| InvalidAttribute("stroke".to_owned(), value.to_owned()))
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
        })
    }
}

impl From<&Ngon> for element::Polygon {
    fn from(n: &Ngon) -> Self {
        Self::new()
            .set(
                "svgnote:position",
                format!("{},{}", n.position.0, n.position.1),
            )
            .set("stroke", n.stroke.to_string_na())
            .set("fill", n.fill.to_string_na())
            .set("stroke-opacity", n.stroke.opacity())
            .set("fill-opacity", n.fill.opacity())
            .set("stroke-width", n.width)
            .set("svgnote:angle", n.angle)
            .set("svgnote:n", n.n)
            .set("svgnote:radius", n.radius)
            // Static
            .set("svgnote:tool", "ngon")
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            // Generated
            .set(
                "points",
                n.points()
                    .iter()
                    .map(|(x, y)| format!("{},{}", x, y))
                    .collect::<Vec<String>>(),
            )
    }
}

impl Ngon {
    fn points(&self) -> Vec<(f32, f32)> {
        let mut points = vec![];
        let angle = 2. * PI / self.n as f32;
        let offset_angle = PI / 2. + angle / 2.;
        for i in 0..self.n {
            points.push((
                self.position.0
                    + self.radius * (i as f32 * angle + offset_angle + self.angle).cos(),
                self.position.1
                    + self.radius * (i as f32 * angle + offset_angle + self.angle).sin(),
            ))
        }
        points
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ellipse {
    pub position: (f32, f32),
    pub stroke: Color,
    pub fill: Color,
    pub width: f32,
    pub radius: f32,
}
impl From<&Ellipse> for element::Ellipse {
    fn from(n: &Ellipse) -> Self {
        Self::new()
            .set("stroke", n.stroke.to_string_na())
            .set("stroke-opacity", n.stroke.opacity())
            .set("fill", n.fill.to_string_na())
            .set("fill-opacity", n.fill.opacity())
            .set("stroke-width", n.width)
            .set("cx", n.position.0)
            .set("cy", n.position.1)
            .set("rx", n.radius)
            .set("ry", n.radius)
    }
}

impl FromAttributes for Ellipse {
    fn from_attributes(attributes: HashMap<String, Value>) -> Result<Self, DocumentError> {
        Ok(Ellipse {
            position: {
                (
                    {
                        let value = attributes
                            .get("cx")
                            .ok_or(MissingAttribute("cx".to_owned()))?;
                        f32::from_str(value)
                            .map_err(|_| InvalidAttribute("cx".to_owned(), value.to_string()))?
                    },
                    {
                        let value = attributes
                            .get("cy")
                            .ok_or(MissingAttribute("cy".to_owned()))?;
                        f32::from_str(value)
                            .map_err(|_| InvalidAttribute("cy".to_owned(), value.to_string()))?
                    },
                )
            },
            width: {
                let value: &str = attributes
                    .get("stroke-width")
                    .ok_or(MissingAttribute("stroke-width".to_owned()))?;
                f32::from_str(value)
                    .map_err(|_| InvalidAttribute("stroke-width".to_owned(), value.to_owned()))?
            },
            radius: {
                let value: &str = attributes
                    .get("rx")
                    .ok_or(MissingAttribute("rx".to_owned()))?;
                f32::from_str(value)
                    .map_err(|_| InvalidAttribute("rx".to_owned(), value.to_owned()))?
            },
            fill: {
                let value: &str = attributes
                    .get("fill")
                    .ok_or(MissingAttribute("fill".to_owned()))?;
                Color::from_str(value)
                    .map_err(|_| InvalidAttribute("fill".to_owned(), value.to_owned()))
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
            stroke: {
                let value: &str = attributes
                    .get("stroke")
                    .ok_or(MissingAttribute("stroke".to_owned()))?;
                Color::from_str(value)
                    .map_err(|_| InvalidAttribute("stroke".to_owned(), value.to_owned()))
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
        })
    }
}

#[derive(Derivative, PartialEq, Clone)]
#[derivative(Debug)]
pub enum Element {
    Line(Line, i32),
    Ngon(Ngon, i32),
    Ellipse(Ellipse, i32),
    Polyline(Polyline, i32),
}

pub trait FromAttributes: Sized {
    fn from_attributes(attributes: HashMap<String, Value>) -> Result<Self, DocumentError>;
}

impl Element {
    pub fn from_event(e: Event, id: i32) -> Result<Self, DocumentError> {
        match e {
            Event::Tag(tag::Path, _, attributes) => {
                let tool: &str = attributes
                    .get("svgnote:tool")
                    .ok_or(MissingAttribute("svgnote:tool".to_owned()))?;
                match tool {
                    "pen" => Ok(Element::Line(Line::from_attributes(attributes)?, id)),
                    _ => Err(InvalidAttribute("svgnote:tool".to_owned(), tool.to_owned()))?,
                }
            }
            Event::Tag(tag::Polygon, _, attributes) => {
                let tool: &str = attributes
                    .get("svgnote:tool")
                    .ok_or(MissingAttribute("svgnote:tool".to_owned()))?;
                match tool {
                    "ngon" => Ok(Element::Ngon(Ngon::from_attributes(attributes)?, id)),
                    _ => Err(InvalidAttribute("svgnote:tool".to_owned(), tool.to_owned()))?,
                }
            }
            Event::Tag(tag::Polyline, _, attributes) => Ok(Element::Polyline(
                Polyline::from_attributes(attributes)?,
                id,
            )),
            Event::Tag(tag::Ellipse, _, attributes) => {
                Ok(Element::Ellipse(Ellipse::from_attributes(attributes)?, id))
            }
            _ => Err(DocumentError::UnknownEvent),
        }
    }
}

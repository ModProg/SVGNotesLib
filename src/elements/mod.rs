mod line;

use std::collections::HashMap;
use std::str::FromStr;

use crate::colors::Color;
use crate::DocumentError;

use derivative::Derivative;
use svg::node::element::tag;
use svg::node::Value;
use svg::parser::Event;
use DocumentError::InvalidAttribute;
use DocumentError::MissingAttribute;

pub use self::line::Line;
pub use self::line::Point;

#[derive(Debug)]
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
                    .map_err(|_| InvalidAttribute("fill".to_owned(), value.to_owned()))?
            },
            stroke: {
                let value: &str = attributes
                    .get("stroke")
                    .ok_or(MissingAttribute("stroke".to_owned()))?;
                Color::from_str(value)
                    .map_err(|_| InvalidAttribute("stroke".to_owned(), value.to_owned()))?
            },
        })
    }
}

#[derive(Debug)]
pub struct Ellipse {
    pub position: (f32, f32),
    pub stroke: Color,
    pub fill: Color,
    pub width: f32,
    pub radius: f32,
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
                    .map_err(|_| InvalidAttribute("fill".to_owned(), value.to_owned()))?
            },
            stroke: {
                let value: &str = attributes
                    .get("stroke")
                    .ok_or(MissingAttribute("stroke".to_owned()))?;
                Color::from_str(value)
                    .map_err(|_| InvalidAttribute("stroke".to_owned(), value.to_owned()))?
            },
        })
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Element {
    Line(Line),
    Ngon(Ngon),
    Ellipse(Ellipse),
}

pub trait FromAttributes: Sized {
    fn from_attributes(attributes: HashMap<String, Value>) -> Result<Self, DocumentError>;
}

impl Element {
    pub fn from_event(e: Event) -> Result<Self, DocumentError> {
        match e {
            Event::Tag(tag::Path, _, attributes) => {
                let tool: &str = attributes
                    .get("svgnote:tool")
                    .ok_or(MissingAttribute("svgnote:tool".to_owned()))?;
                match tool {
                    "pen" => Ok(Element::Line(Line::from_attributes(attributes)?)),
                    _ => Err(InvalidAttribute("svgnote:tool".to_owned(), tool.to_owned()))?,
                }
            }
            Event::Tag(tag::Polygon, _, attributes) => {
                let tool: &str = attributes
                    .get("svgnote:tool")
                    .ok_or(MissingAttribute("svgnote:tool".to_owned()))?;
                match tool {
                    "ngon" => Ok(Element::Ngon(Ngon::from_attributes(attributes)?)),
                    _ => Err(InvalidAttribute("svgnote:tool".to_owned(), tool.to_owned()))?,
                }
            }
            Event::Tag(tag::Ellipse, _, attributes) => {
                Ok(Element::Ellipse(Ellipse::from_attributes(attributes)?))
            }
            _ => Err(DocumentError::UnknownEvent),
        }
    }
}

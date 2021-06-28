#![feature(assert_matches)]

use std::str::FromStr;

use thiserror::Error;

use crate::elements::Element;

pub mod colors;
pub mod elements;

pub struct Document {
    pub elements: Vec<Element>,
}

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("Invalid {0}: `{1}`")]
    InvalidAttribute(String, String),
    #[error("Missing {0}")]
    MissingAttribute(String),
    #[error("Invalid Point: `{0}`")]
    InvalidPoint(String),
    #[error("Unknown Event")]
    UnknownEvent,
}

impl FromStr for Document {
    type Err = DocumentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self {
            elements: svg::read(s)
                .unwrap()
                .map(Element::from_event)
                .filter(|e| match e {
                    Err(DocumentError::UnknownEvent) => false,
                    _ => true,
                })
                .collect::<Result<_, _>>()?,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::colors::Color;
    use crate::elements::Line;
    use crate::elements::Ngon;
    use crate::elements::Point;
    use crate::elements::{Element, Ellipse};
    use crate::Document;

    #[test]
    fn parse() {
        let s = r##"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- Created with SVGNotes (https://github.com/ModProg/SVGNotesLib) -->

<svg
   width="100mm"
   height="100mm"
   viewBox="0 0 100 100"
   version="1.1"
   xmlns="http://www.w3.org/2000/svg"
   xmlns:svg="http://www.w3.org/2000/svg"
   xmlns:svgnote="https://github.com/ModProg/SVGNotesLib"
   svgnote:version="0.1"
>
  <g id="foreground">
    <path
       fill="#00000000"
       stroke="#000000"
       stroke-width="4"
       stroke-linecap="round"
       stroke-linejoin="round"
       d="M 10,10 60,30 50,60 90,10"
       svgnote:tool="pen"
       svgnote:width="4"
       svgnote:points="10,10,1 60,30,4 50,60,3 90,10,2"
    />
    <polygon 
       fill="#00000000"
       stroke="#FF0000FF"
       stroke-width="3"
       stroke-linecap="round"
       stroke-linejoin="round"
       points="50,50 80,50 80,80 50,80"
       svgnote:tool="ngon"
       svgnote:n="4"
       svgnote:angle="0"
       svgnote:position="65,65"
       svgnote:radius="15"
    />
    <ellipse 
       fill="#FFFF0055"
       stroke="#FFFF00FF"
       stroke-width="2"
       cx="65"
       cy="65"
       rx="10"
       ry="10"
    />
  </g>
</svg>
        "##;
        let d = Document::from_str(s).unwrap();
        assert_eq!(
            d.elements.len(),
            3,
            "There should be {} elements in {:?}",
            3,
            d.elements
        );
        assert_matches!(
            &d.elements[0],
            Element::Line(Line {
                width,
                points,
                color: Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0xFF
                }
            })
        if *width == 4.0  &&  elems_eq(&points, &[(10.0,10.0,1.0),(60.0,30.0,4.0),(50.0, 60.0,3.0),(90.0, 10.0,2.0)].iter().map(|&(x,y,w)| Point( x,y,w)).collect::<Vec<_>>()));
        assert_matches!(
            &d.elements[1],
            Element::Ngon(Ngon {
                position,
                width,
                angle,
                n: 4,
                radius,
                fill: Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0
                },
                stroke: Color {
                    r: 0xFF,
                    g: 0,
                    b: 0,
                    a: 0xFF
                }
            })
            if
                *position == (65.0,65.0) &&
                *width == 3.0 &&
                *angle == 0.0 &&
                *radius == 15.0

        );
        assert_matches!(
            &d.elements[2],
            Element::Ellipse(Ellipse {
                position,
                width,
                radius,
                fill: Color {
                    r: 0xFF,
                    g: 0xFF,
                    b: 0,
                    a: 0x55
                },
                stroke: Color {
                    r: 0xFF,
                    g: 0xFF,
                    b: 0,
                    a: 0xFF
                }
            })
            if
                *position == (65.0,65.0) &&
                *width == 2.0 &&
                *radius == 10.0
        );
    }

    fn elems_eq<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        a.len() == b.len() && a.iter().zip(b).filter(|&(a, b)| a == b).count() == a.len()
    }
}

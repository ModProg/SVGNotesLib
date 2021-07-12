#![feature(assert_matches, const_fn_floating_point_arithmetic)]
use std::fmt::Display;
use std::str::FromStr;

use indoc::writedoc;
use thiserror::Error;

use crate::elements::Element;

pub mod colors;
pub mod elements;

pub fn elems_eq<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.len() == b.len() && a.iter().zip(b).filter(|&(a, b)| a == b).count() == a.len()
}

#[derive(Debug)]
pub struct Document {
    pub elements: Vec<Element>,
}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        elems_eq(&self.elements, &other.elements)
    }
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
        let mut id = 1;
        return Ok(Self {
            elements: svg::read(s)
                .unwrap()
                .map(|e| {
                    Element::from_event(e, {
                        id += 1;
                        id
                    })
                })
                .filter(|e| match e {
                    Err(DocumentError::UnknownEvent) => false,
                    _ => true,
                })
                .collect::<Result<_, _>>()?,
        });
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut doc = svg::Document::new()
            .set("viewBox", (0, 0, 2000, 2000))
            .set("width", "100mm")
            .set("height", "100mm")
            .set("xmlns:svgnote", "https://github.com/ModProg/SVGNotesLib")
            .set("svgnote:version", "0.1");
        doc = self
            .elements
            .iter()
            .fold(doc, |doc, element| match element {
                Element::Line(e, _) => doc.add::<svg::node::element::Path>(e.into()),
                Element::Ngon(e, _) => doc.add::<svg::node::element::Polygon>(e.into()),
                Element::Ellipse(e, _) => doc.add::<svg::node::element::Ellipse>(e.into()),
                Element::Polyline(e, _) => doc.add::<svg::node::element::Polyline>(e.into()),
            });
        writedoc!(
            f,
            r##"
            <?xml version="1.0" encoding="UTF-8" standalone="no"?>
            <!-- Created with SVGNotes (https://github.com/ModProg/SVGNotesLib) -->

            {}"##,
            doc
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::str::FromStr;

    use crate::colors::Color;
    use crate::elements::Line;
    use crate::elements::LinePoint;
    use crate::elements::Ngon;
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
       fill="#000000"
       stroke="#000000"
       fill-opacity="0"
       stroke-opacity="1"
       stroke-width="4"
       stroke-linecap="round"
       stroke-linejoin="round"
       d="M 10,10 60,30 50,60 90,10"
       svgnote:tool="pen"
       svgnote:width="4"
       svgnote:points="10,10,1 60,30,4 50,60,3 90,10,2"
    />
    <polygon
       fill="#000000"
       stroke="#FF0000"
       stroke-width="3"
       fill-opacity="0"
       stroke-opacity="1"
       stroke-linecap="round"
       stroke-linejoin="round"
       points="15"
       svgnote:tool="ngon"
       svgnote:n="4"
       svgnote:angle="0"
       svgnote:position="65,65"
       svgnote:radius="20"
    />
    <ellipse
       fill="#FFFF00"
       stroke="#FFFF00"
       fill-opacity="0.3333333333"
       stroke-opacity="1"
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
        if *width == 4.0  &&  elems_eq(&points, &[(10.0,10.0,1.0),(60.0,30.0,4.0),(50.0, 60.0,3.0),(90.0, 10.0,2.0)].iter().map(|&(x,y,w)| LinePoint( x,y,w)).collect::<Vec<_>>()));
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
                *radius == 20.0

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

        if Path::new("o.svg").is_file() {
            let mut file = File::create("o.svg").unwrap();
            write!(file, "{}", d).unwrap();
        }
    }

    #[test]
    fn encoding() {
        let doc = Document {
            elements: vec![
                Element::Line(Line {
                    color: Color::rgb(0xFF, 0, 0),
                    width: 5.0,
                    points: vec![
                        LinePoint(0., 0., 0.),
                        LinePoint(2., 10., 1.),
                        LinePoint(1.2313, 10.213, 1.123),
                    ],
                }),
                Element::Ngon(Ngon {
                    position: (3.0, 12.0),
                    stroke: Color::rgba(13, 24, 51, 123),
                    fill: Color::rgba(0xFF, 0xFF, 0xFF, 0),
                    width: 15.,
                    angle: PI / 4.0,
                    n: 9,
                    radius: 5.,
                }),
                Element::Ellipse(Ellipse {
                    position: (10., 2.),
                    stroke: Color::rgb(0xFF, 0xFF, 12),
                    fill: Color::rgba(0xFF, 0, 0, 0xFE),
                    width: 13.2,
                    radius: 12.2,
                }),
            ],
        };

        let string = doc.to_string();

        println!("{}", string);

        let parsed = Document::from_str(&string).unwrap();

        assert_eq!(doc, parsed);
    }

    fn elems_eq<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        a.len() == b.len() && a.iter().zip(b).filter(|&(a, b)| a == b).count() == a.len()
    }
}

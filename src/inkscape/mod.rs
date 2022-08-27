mod object;
mod parse;

use anyhow::Context;
use anyhow::Result;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug)]
pub struct Inkscape {
    leading_events: Vec<Event<'static>>,
    layers: Vec<Group>,
    trailing_events: Vec<Event<'static>>,
}

#[derive(Debug)]
struct Group {
    header: Event<'static>,
    content: Vec<object::Object>,
    footer: Event<'static>,
}


/// Export an [`Inkscape`] object to a file
impl Inkscape {
    pub fn write_svg<'a, W: Write>(self, writer: W) -> Result<()> {
        let mut writer = quick_xml::Writer::new(writer);

        for event in self.leading_events {
            writer.write_event(&event)
                .with_context(|| format!("failed to write a leading event: {:?}", event))?;
        }

        for layer in self.layers {
            writer.write_event(&layer.header)
                .with_context(|| format!("failed to write header for layer : {:?}", layer.header))?;

            for object in layer.content {
                let event = object.into_event();
                writer.write_event(&event)
                    .with_context(|| format!("failed to write inner object for layer : {:?}", event))?;
            }

            writer.write_event(&layer.footer)
                .with_context(|| format!("failed to write footer for layer : {:?}", layer.footer))?;
        }

        for event in self.trailing_events {
            writer.write_event(&event)
                .with_context(|| format!("failed to write a trailing event: {:?}", event))?;
        }


        Ok(())
    }

    pub fn parse_svg<'a, R: BufRead>(reader: R, buffer: &'a mut Vec<u8>) -> Result<Self> {
        let mut reader = quick_xml::Reader::from_reader(reader);

        let (leading_events, first_group) = parse::leading_events(&mut reader, buffer)?;

        //println!("\n\n\n\n\n\n");
        //dbg!(&leading_events);
        //dbg!(&first_group);

        // read the inner layers
        let (layers, first_trailing) = if let Some(first_group) = first_group {
            let (layers, first_trailing) = parse::layers(&mut reader, buffer, first_group)?;
            (layers, Some(first_trailing))
        } else {
            (vec![], None)
        };

        //dbg!(&layers, &first_trailing);

        let trailing_events = if let Some(first_trailing) = first_trailing {
            parse::trailing_events(&mut reader, buffer, first_trailing)?
        } else {
            Vec::new()
        };

        //dbg!(&trailing_events);

        let inkscape = Inkscape {
            leading_events,
            layers,
            trailing_events,
        };
        Ok(inkscape)
    }
}


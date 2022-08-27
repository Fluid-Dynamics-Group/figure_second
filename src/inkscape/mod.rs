mod object;
mod parse;

use object::EncodedImage;

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

impl Group {
    #[cfg(test)]
    fn eof_group_test(content: Vec<object::Object>) -> Self {
        Self {
            header: Event::Eof,
            content,
            footer: Event::Eof,
        }
    }
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

    pub fn id_to_image(&mut self, image: EncodedImage) -> Result<()> {
        for layer in &self.layers {
        }

        anyhow::bail!("id is not contained in the document");
    }
}

struct IdIterator<'a> {
    curr_group_idx: usize,
    curr_group_object_idx: usize,
    groups: &'a [Group]
}

impl <'a> IdIterator <'a>  {
    fn new(groups: &'a [Group]) -> IdIterator<'a>  {
        Self {
            groups,
            curr_group_idx: 0,
            curr_group_object_idx: 0
        }
    }
}

impl <'a> Iterator for IdIterator<'a>  {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let group = if let Some(grp) = self.groups.get(self.curr_group_idx) {
            grp
        } else {
            // we have exhaused all the groups we can
            return None
        };

        if let Some(group_object) = group.content.get(self.curr_group_object_idx) {
            // match on the group object to see if this element is a rectangle or image,
            // and therefore contains `Identifier` information we can return from the iterator
            match group_object {
                object::Object::Rectangle(rect) => {
                    self.curr_group_object_idx += 1;
                    return Some(&rect.ident.id)
                }
                object::Object::Image(image) => {
                    self.curr_group_object_idx += 1;
                    return Some(&image.ident.id)
                }
                object::Object::Other(_) => {
                    // we HAVE a valid object, but since its not an object we normally care
                    // about, we have not parsed the identifiers for it
                    self.curr_group_object_idx += 1;
                    return self.next()
                }
            }
        } else {
            // there are no more objects in this layer, go to the next layer
            // group and return anything from there
            self.curr_group_idx += 1;
            self.curr_group_object_idx = 0;
            return self.next()
        }
    }
}


#[test]
fn id_iterator() {
    use object::{Object, Image, Rectangle, Identifiers};
    let groups = vec![
        Group::eof_group_test(vec![]),
        Group::eof_group_test(vec![
            Object::Rectangle(Rectangle::from_ident(Identifiers::zeros_with_id("1"))),
            Object::Rectangle(Rectangle::from_ident(Identifiers::zeros_with_id("2"))),
            Object::Image(Image::from_ident(Identifiers::zeros_with_id("3"))),
        ]),
        Group::eof_group_test(vec![]),
        Group::eof_group_test(vec![
            Object::Rectangle(Rectangle::from_ident(Identifiers::zeros_with_id("4"))),
            Object::Other(Event::Empty(BytesStart::owned_name(b"doesnt_matter".to_vec()))),
            Object::Other(Event::Empty(BytesStart::owned_name(b"doesnt_matter2".to_vec()))),
            Object::Rectangle(Rectangle::from_ident(Identifiers::zeros_with_id("5"))),
        ]),
        Group::eof_group_test(vec![]),
    ];

    let iter = IdIterator::new(&groups);
    let ids = iter.collect::<Vec<_>>();
    assert_eq!(&["1", "2", "3", "4", "5"], ids.as_slice());
}

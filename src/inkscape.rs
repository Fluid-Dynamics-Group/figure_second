use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use std::io::BufRead;
use anyhow::Result;
use anyhow::Context;
use quick_xml::events::Event;
use quick_xml::events::BytesStart;


#[derive(Debug)]
pub struct Inkscape {
    leading_events: Vec<Event<'static>>,
    layers: Vec<Group>,
    trailing_events: Vec<Event<'static>>
}

#[derive(Debug)]
struct Group {
    header: Event<'static>,
    content: Vec<Object>,
    footer: Event<'static>,
}

#[derive(Debug)]
enum Object {
    Rectangle(Rectangle),
    Image(Image),
    /// other does not necessarily have to be a image or geometrical event,
    /// it could also be spacing events
    Other(Event<'static>)
}

#[derive(Debug)]
struct Rectangle {
    ident:  Identifiers,
    element: BytesStart<'static>
}

#[derive(Debug)]
struct Image {
    ident:  Identifiers,
    element: BytesStart<'static>
}

#[derive(Debug)]
struct Identifiers {
    id: String,
    width: f64,
    height: f64,
}

impl Identifiers {
    fn from_elem(elem: &BytesStart<'static>) -> Result<Self> {
        let atts = elem.attributes()
            .filter_map(Result::ok)
            .filter(|att| 
                 att.key == b"width" ||
                 att.key == b"height" ||
                 att.key == b"id"
             );

        let mut width = None;
        let mut height = None;
        let mut id = None;

        for att in atts {
            if att.key == b"width" {
                let number = String::from_utf8(att.value.to_vec())
                    .with_context(|| format!("failed to convert `width` parameter to utf8 string"))?; 
                width = Some(number.parse().with_context(|| format!("failed to parse `width` paramter `{number}` to float"))?);
            } else if att.key == b"height" {
                let number = String::from_utf8(att.value.to_vec())
                    .with_context(|| format!("failed to convert `height` parameter to utf8 string"))?; 
                height = Some(number.parse().with_context(|| format!("failed to parse `height` paramter `{number}` to float"))?);
            }
            else if att.key == b"id" {
                let id_utf8 = String::from_utf8(att.value.to_vec())
                    .with_context(|| format!("failed to convert `id` parameter to utf8 string"))?; 
                id = Some(id_utf8)
            }
        }

        let out = match (width,height,id)  {
            (Some(width), Some(height), Some(id)) => {
                Identifiers {id, width, height }
            }
            (w, h, id) => anyhow::bail!("one of width / height / id was missing from element. Width: `{w:?}` height: `{h:?}` id `{id:?}`")
        };

        Ok(out)
    }
}

pub fn parse_svg<'a, R: BufRead>(reader: R, buffer: &'a mut Vec<u8>) -> Result<Inkscape> {
    let mut reader = quick_xml::Reader::from_reader(reader);

    let (leading_events, first_group) = leading_events(&mut reader, buffer)?;

    println!("\n\n\n\n\n\n");
    dbg!(&leading_events);
    dbg!(&first_group);

    // read the inner layers
    let (layers, first_trailing) = if let Some(first_group) = first_group {
        let (layers, first_trailing) = layers(&mut reader, buffer, first_group)?;
        (layers, Some(first_trailing))
    } else {
        (vec![], None)
    };

    dbg!(&layers, &first_trailing);

    let trailing_events = if let Some(first_trailing) = first_trailing {
        trailing_events(&mut reader, buffer, first_trailing)?
    } else {
        Vec::new()
    };

    let inkscape = Inkscape {leading_events, layers, trailing_events};
    Ok(inkscape)
}

fn leading_events<R: BufRead>(reader: &mut quick_xml::Reader<R>, buffer: &mut Vec<u8>) -> Result<(Vec<Event<'static>>, Option<BytesStart<'static>>)> {
    let mut out = Vec::new();

    while let Ok(event) = reader.read_event(buffer) {
        let event = event.into_owned();

        dbg!(&event);
        if let Event::Start(element) = event {
            if element.name() == b"g" {
                // we are at the first layer event, leave this function

                return Ok((out, Some(element)))
            }
        } else {
            out.push(event);
        }
    }

    Ok((out, None))
}

fn trailing_events<R: BufRead>(reader: &mut quick_xml::Reader<R>, buffer: &mut Vec<u8>, first_layer: Event<'static>) -> Result<Vec<Event<'static>>> {

    todo!()
}

fn layers<R: BufRead>(reader: &mut quick_xml::Reader<R>, buffer: &mut Vec<u8>, first_layer_start: BytesStart<'static>) -> Result<(Vec<Group>,Event<'static>)> {

    let mut out = Vec::new();

    let first_group = group(first_layer_start, reader, buffer)?;
    out.push(first_group);

    while let Ok(event) = reader.read_event(buffer) {
        let event = event.into_owned();

        if let Event::Start(ref element) = event {
            if element.name() == b"g" {
                // we are at the first layer event, leave this function

                //return Ok((out, Some(event)))
            }
        } else {
            return Ok((out, event))
        }

    }

    unreachable!()


    //Ok((out, None))
}

fn group<R: BufRead>(start_event: BytesStart<'static>, reader: &mut quick_xml::Reader<R>, buffer: &mut Vec<u8>) -> Result<Group> {
    let mut content = Vec::new();

    let mut footer = None;

    while let Ok(event) = reader.read_event(buffer) {
        let event = event.into_owned();

        match event {
            Event::Empty(xml_object) => {
                // parse the object
                let object = object(xml_object)
                    .with_context(|| {
                        let name = layer_name(&start_event).unwrap();
                        format!("failed to parse object in layer {name}")
                    })?;

                content.push(object);
            }
            Event::End(end) => {
                footer = Some(Event::End(end));
                break
            }
            other_event => {
                content.push(Object::Other(other_event));
                //let name = layer_name(&start_event)?;
                //anyhow::bail!("unknown XML attribute occured while parsing layer {}: {:?}", name, other_event);
            }
        }
    }

    let footer = if let Some(inner_footer) = footer {
        inner_footer
    } else {
        let name = layer_name(&start_event)?;
        anyhow::bail!("failed to find end of group attribute for layer {}", name)
    };

    let grp = Group {
        header: Event::Start(start_event),
        content,
        footer,
    };

    Ok(grp)
}

fn object(element: BytesStart<'static>) -> Result<Object> {

    let obj = match element.name() {
        b"image" => {
            // parse as an image
            let ident = Identifiers::from_elem(&element)
                .with_context(|| format!("failed to parse image id / width / height from element {:?}", element))?;

            Object::Image(Image{ident, element})

        }
        b"rect" => {
            // parse as a rectangle
            let ident = Identifiers::from_elem(&element)
                .with_context(|| format!("failed to parse rectangle id / width / height from element {:?}", element))?;

            Object::Rectangle(Rectangle{ident, element})
        }
        _unknown => {
            Object::Other(Event::Empty(element))
        }
    };

    Ok(obj)
}

fn layer_name(layer_start_event: &BytesStart<'static>) -> Result<String> {
    let (_, name_id) = layer_start_event.attributes().into_iter()
        .filter_map(|x| x.ok())
        .map(|att| {
            (att.key, att.value)
        })
        .find(|(key, _)| key == &b"id".as_slice()).unwrap();

    Ok(String::from_utf8(name_id.to_vec())?)
}

fn utf8_name(event: BytesStart<'_>) -> Result<String> {
    String::from_utf8(event.name().to_vec())
        .with_context(|| format!("failed to convert bytes sequence to UTF8 name: {:?}", event.name()))
}

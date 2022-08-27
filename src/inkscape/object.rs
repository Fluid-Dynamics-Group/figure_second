use anyhow::Context;
use anyhow::Result;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;

#[derive(Debug)]
pub(crate) enum Object {
    Rectangle(Rectangle),
    Image(Image),
    /// other does not necessarily have to be a image or geometrical event,
    /// it could also be spacing events
    Other(Event<'static>),
}

impl Object {
    pub(crate) fn into_event(self) -> Event<'static> {
        match self {
            Self::Rectangle(rect) => Event::Empty(rect.element),
            Self::Image(image) => Event::Empty(image.element),
            Self::Other(object) => object,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Rectangle {
    pub(crate) ident: Identifiers,
    pub(crate) element: BytesStart<'static>,
}

#[derive(Debug)]
/// an image with base64 encoding in inkscape
///
/// actual content of the image is stored in the xlink:href attribute
/// of the element field.
pub(crate) struct Image {
    pub(crate) ident: Identifiers,
    pub(crate) element: BytesStart<'static>,
}

#[derive(Debug)]
pub(crate) struct Identifiers {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

impl Identifiers {
    pub(crate) fn from_elem(elem: &BytesStart<'static>) -> Result<Self> {
        let atts = elem
            .attributes()
            .filter_map(Result::ok)
            .filter(|att| att.key == b"width" || att.key == b"height" || att.key == b"id");

        let mut width = None;
        let mut height = None;
        let mut id = None;

        for att in atts {
            if att.key == b"width" {
                let number = String::from_utf8(att.value.to_vec()).with_context(|| {
                    format!("failed to convert `width` parameter to utf8 string")
                })?;
                width = Some(number.parse().with_context(|| {
                    format!("failed to parse `width` paramter `{number}` to float")
                })?);
            } else if att.key == b"height" {
                let number = String::from_utf8(att.value.to_vec()).with_context(|| {
                    format!("failed to convert `height` parameter to utf8 string")
                })?;
                height = Some(number.parse().with_context(|| {
                    format!("failed to parse `height` paramter `{number}` to float")
                })?);
            } else if att.key == b"id" {
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

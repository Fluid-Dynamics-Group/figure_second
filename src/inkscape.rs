use serde::{Serialize, Deserialize};
use std::borrow::Cow;

type CowStr<'a> = Cow<'a, str>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename="svg")]
pub struct Svg<'a> {
    width: CowStr<'a>,
    height: CowStr<'a>,
    #[serde(rename="viewBox")]
    view: CowStr<'a>,
    id: CowStr<'a>,
    // not present in 1.2, present in 1.1?
    #[serde(rename="xml:space")]
    space: Option<CowStr<'a>>,
    #[serde(rename="inkscape:version")]
    version: CowStr<'a>,
    #[serde(rename="sodipodi:docname")]
    sodipodi_docname: CowStr<'a>,
    //
    // export settings
    //
    #[serde(rename="inkscape:export-filename")]
    export_filename: Option<CowStr<'a>>,
    #[serde(rename="inkscape:export-xdpi")]
    export_xdpi: Option<CowStr<'a>>,
    #[serde(rename="inkscape:export-ydpi")]
    export_ydpi: Option<CowStr<'a>>,
    //
    // end export settings
    //
    #[serde(rename="xmlns:inkscape")]
    xmlns_inkscape: CowStr<'a>,
    #[serde(rename="xmlns:sodipodi")]
    xmlns_soidpodi: CowStr<'a>,
    #[serde(rename="xmlns:xlink")]
    xmlns_xlink: CowStr<'a>,
    xmlns: CowStr<'a>,
    #[serde(rename="xmlns:svg")]
    xmlns_svg: CowStr<'a>,
    // for some reason this element is namespaced, its unclear 
    // if the writing of this element will be correct (preliminary results 
    // show that it is fine), but im not sure what the broader implications are
    // of this element
    #[serde(rename(serialize="sodipodi:namedview"))]
    namedview: NamedView<'a>,
    defs: Defs<'a>,
    g: Vec<Global<'a>>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NamedView<'a> {
    id: CowStr<'a>,
    pagecolor: CowStr<'a>,
    bordercolor: CowStr<'a>,
    borderopacity: CowStr<'a>,
    //#[serde(rename="inkscape:pageshadow")]
    //page_shadow: Option<CowStr<'a>>,
    #[serde(rename="inkscape:showpageshadow")]
    #[serde(alias="inkscape:pageshadow")]
    show_page_shadow: Option<CowStr<'a>>,
    #[serde(rename="inkscape:pageopacity")]
    page_opacity: CowStr<'a>,
    #[serde(rename="inkscape:pagecheckerboard")]
    page_checkboard: CowStr<'a>,
    // this seems to be missing form inkscape 1.2 documents, but it is present
    // in inkscape 1.1 documents
    #[serde(rename="inkscape:deskcolor")]
    desk_color: Option<CowStr<'a>>,
    #[serde(rename="inkscape:document-units")]
    document_units: CowStr<'a>,
    #[serde(rename="showgrid")]
    show_grid: CowStr<'a>,
    #[serde(rename="inkscape:zoom")]
    zoom: CowStr<'a>,
    #[serde(rename="inkscape:cx")]
    cx: CowStr<'a>,
    #[serde(rename="inkscape:cy")]
    cy: CowStr<'a>,
    #[serde(rename="inkscape:window-height")]
    window_height: CowStr<'a>,
    #[serde(rename="inkscape:window-width")]
    window_width: CowStr<'a>,
    #[serde(rename="inkscape:window-x")]
    window_x: CowStr<'a>,
    #[serde(rename="inkscape:window-y")]
    window_y: CowStr<'a>,
    #[serde(rename="inkscape:window-maximized")]
    window_maximized: CowStr<'a>,
    #[serde(rename="inkscape:current-layer")]
    current_layer: CowStr<'a>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Defs<'a> {
    id: CowStr<'a>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "g")]
pub struct Global<'a> {
    #[serde(rename="inkscape:label")]
    label: CowStr<'a>,
    #[serde(rename="inkscape:groupmode")]
    group: CowStr<'a>,
    #[serde(rename="id")]
    id: CowStr<'a>,
    // can appear in inkscape 1.2 (?) 
    transform: Option<CowStr<'a>>,
    // can appear in inkscape 1.2 
    #[serde(rename="sodipodi:insensitive")]
    insensitive: Option<CowStr<'a>>,

    //#[serde(rename="$value")]
    //image: Vec<Content<'a>>
    #[serde(rename="$value")]
    content: Vec<Content<'a>>
    //content: Vec<Text<'a>>
}

#[derive(Deserialize, Serialize, Debug)]
//#[serde(untagged)]
//#[serde(deny_unknown_fields)]
enum Content<'a> {
    Image(Image<'a>),
    #[serde(rename="rect")]
    Rectangle(Rectangle<'a>),
    //#[serde(rename="text")]
    Text(Text<'a>),
    Path(Path<'a>),
}

#[derive(Deserialize, Serialize, Debug)]
//#[serde(deny_unknown_fields)]
pub struct Image<'a> {
    width: CowStr<'a>,
    height: CowStr<'a>,
    #[serde(rename="preserveAspectRatio")]
    preserve_aspect: CowStr<'a>,
    #[serde(rename="xlink:href")]
    inline_data: CowStr<'a>,
    id: CowStr<'a>,
    x: CowStr<'a>,
    y: CowStr<'a>,
}

#[derive(Deserialize, Serialize, Debug)]
//#[serde(deny_unknown_fields)]
pub struct Rectangle<'a> {
    width: CowStr<'a>,
    height: CowStr<'a>,
    x: CowStr<'a>,
    y: CowStr<'a>,
    id: CowStr<'a>,
    style: CowStr<'a>,
}

#[derive(Deserialize, Serialize, Debug)]
//#[serde(deny_unknown_fields)]
pub struct Path<'a> {
    style: CowStr<'a>,
    d: CowStr<'a>,
    id: CowStr<'a>,
    transform: Option<CowStr<'a>>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Text <'a> {
    // quick-xml *really* does not like us renaming with a namespace
    // here for some reason, its a mystery
    //#[serde(rename(serialize="xml:space"))]
    #[serde(rename="xml:space")]
    space: CowStr<'a>,
    style: CowStr<'a>,
    x: CowStr<'a>,
    y: CowStr<'a>,
    id: CowStr<'a>,
    transform: Option<CowStr<'a>>,
    #[serde(rename="$value")]
    tspan: Option<Vec<Tspan<'a>>>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Tspan <'a> {
    x: CowStr<'a>,
    y: CowStr<'a>,
    id: CowStr<'a>,
    #[serde(rename="$value")]
    text: CowStr<'a>
}

use serde::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Debug)]
pub struct Svg<'a> {
    pub id: Cow<'a, str>,
    //#[serde(flatten)]
    //other: quick_xml::events::Event<'a>
}

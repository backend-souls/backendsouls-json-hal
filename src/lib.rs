/// The JSON Hypertext Application Language (HAL) is a standard which
/// establishes conventions for expressing hypermedia controls, such as
/// links.
///
/// HAL is a generic media type with which Web APIs can be developed and
/// exposed as series of links. Clients of these APIs can select links
/// by their link relation type and traverse them in order to progress
/// through the application.
///
/// Reference: [draft-kelly-json-hal-10](https://datatracker.ietf.org/doc/html/draft-kelly-json-hal-10)
use std::collections::HashMap;

use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

pub fn get_default_response<T: Clone + Default>(link: String, data: T) -> HalResource<T> {
    let _self = LinkBuilder::default().href(link).build().unwrap();
    let links = HalLinksBuilder::default()._self(_self).build().unwrap();

    HalResourceBuilder::default()._links(links).data(data).build().unwrap()
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize)]
#[builder(setter(into), default)]
pub struct HalResource<T: Clone + Default> {
    pub _links: Option<HalLinks>,
    pub _embedded: Option<HashMap<String, HalResource<T>>>,

    #[serde(flatten)]
    pub data: T,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize)]
#[builder(setter(into), default)]
pub struct HalLinks {
    #[serde(rename = "self")]
    pub _self: Link,
    pub related: Option<Link>,
    pub next: Option<Link>,
    pub search: Option<Link>,
    pub previous: Option<Link>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize)]
#[builder(setter(into), default)]
pub struct Link {
    pub href: String,
    pub templated: Option<bool>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub deprecation: Option<String>,
    pub name: Option<String>,
    pub profile: Option<String>,
    pub title: Option<String>,
    pub hreflang: Option<String>,
}

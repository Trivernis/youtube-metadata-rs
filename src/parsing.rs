use crate::error::{Parsing, Result};
use scraper::{ElementRef, Html, Selector};

pub mod video_information;

/// Tries selecting one element or fails if the element can't be found
fn try_select_one<'a>(document: &'a Html, selector: &Selector) -> Result<ElementRef<'a>> {
    document
        .select(selector)
        .next()
        .ok_or_else(|| Parsing::MissingElement(format!("{:?}", selector)).into())
}

/// Tries to select a given attribute
fn try_select_attribute<'a>(
    document: &'a Html,
    selector: &Selector,
    attribute: &str,
) -> Result<&'a str> {
    let element = try_select_one(document, selector)?;
    element
        .value()
        .attr(attribute)
        .ok_or_else(|| Parsing::MissingAttribute(attribute.to_string()).into())
}

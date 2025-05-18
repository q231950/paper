use crate::html_resource::HTMLResource;
use scraper::{ElementRef, Html, Selector};

pub(crate) trait TextProvider {
    fn get_text(&self, selector: &str) -> Option<String>;
    fn get_attribute(&self, attribute: &str, selector: &str) -> Option<String>;
}

impl TextProvider for Html {
    fn get_text(&self, selector: &str) -> Option<String> {
        Selector::parse(selector)
            .ok()
            .map(|item_selector| {
                self.select(&item_selector)
                    .map(|item_ref| HTMLResource::get_text_from_element(item_ref))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .filter(|s| !s.is_empty())
    }

    fn get_attribute(&self, attribute: &str, selector: &str) -> Option<String> {
        Selector::parse(selector).map_or(None, |s| {
            self.select(&s)
                .next()
                .map_or(None, |input| input.attr(attribute).map(|f| f.to_string()))
        })
    }
}

impl<'a> TextProvider for ElementRef<'_> {
    fn get_text(&self, selector: &str) -> Option<String> {
        Selector::parse(selector)
            .ok()
            .map(|item_selector| {
                self.select(&item_selector)
                    .map(|item_ref| HTMLResource::get_text_from_element(item_ref))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .filter(|s| !s.is_empty())
    }

    fn get_attribute(&self, attribute: &str, selector: &str) -> Option<String> {
        Selector::parse(selector).map_or(None, |s| {
            self.select(&s)
                .next()
                .map_or(None, |input| input.attr(attribute).map(|f| f.to_string()))
        })
    }
}

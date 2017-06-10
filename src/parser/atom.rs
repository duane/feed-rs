use chrono::prelude::*;
use xml5ever::rcdom::{NodeData, Handle};
use feed::Feed;
use entry::{Entry, Link};
use super::{attr, text, uuid_gen, timestamp_from_rfc3339};

pub fn handle_atom(handle: Handle) -> Option<Feed> {
    let node = handle;
    let mut feed = Feed::new();
    for child in node.children.borrow().iter() {
        match child.data {
            NodeData::Element { ref name, ref attrs, .. } => {
                let tag_name = name.local.as_ref();
                match tag_name {
                    "id"    => feed.id = text(child.clone()).unwrap_or(uuid_gen()),
                    "title" => feed.title = text(child.clone()),
                    "subtitle" => feed.description = text(child.clone()),
                    "updated" => feed.last_updated = timestamp_from_rfc3339(child.clone()),
                    "link" => {
                        // rel
                        //    self
                        let attributes = &attrs.borrow();
                        let rel = attr("rel", attributes).unwrap_or("".to_string());
                        let href = attr("href", attributes);
                        match (rel.as_ref(), href) {
                            ("self", Some(href)) => feed.id = format!("feed/{}", href),
                            (_, Some(href)) => feed.website = Some(href),
                            _ => (),
                        }
                    },
                    //"author" => (),
                    "logo" => feed.visual_url = text(child.clone()),
                    "icon" => feed.icon_url = text(child.clone()),
                    "generator" => (),
                    "contributor" => (),
                    "category" => {},
                    "rights" => (),
                    "entry" => {
                        if let Some(entry) = handle_entry(child.clone()) {
                            feed.entries.push(entry)
                        }
                    },
                    _ => (),
                }
            },
            _ => {},
        }
    }
    Some(feed)
}

pub fn handle_entry(handle: Handle) -> Option<Entry> {
    let node = handle;
    let mut entry = Entry::new();
    for child in node.children.borrow().iter() {
        match child.data {
            NodeData::Element { ref name, ref attrs, .. } => {
                let tag_name = name.local.as_ref();
                match tag_name {
                    "id" => entry.id = text(child.clone()).unwrap_or(uuid_gen()),
                    "title" => entry.title = text(child.clone()),
                    "summary" => entry.summary = text(child.clone()),
                    "content" => {
                        //entry.content = text(child.clone()),
                        let attributes = &attrs.borrow();
                        let content_type = attr("type", attributes).unwrap_or("text".to_string());
                        let src = attr("src", attributes);
                        match content_type.as_ref() {
                            "text" => (),
                            "html" => (),
                            "xhtml" => (),
                            _ => (),
                        }
                    },
                    "author" => {},
                    "link" => {
                        // rel alternate
                    },
                    "published" => entry.published = timestamp_from_rfc3339(child.clone()).unwrap_or(UTC::now().naive_utc()),
                    "updated" => entry.updated = timestamp_from_rfc3339(child.clone()),
                    "category" => {
                        let attributes = &attrs.borrow();
                        let term   = attr("term", attributes);
                        let scheme = attr("schema", attributes);
                        let label  = attr("label", attributes);
                        match (term, scheme, label) {
                            (Some(term), _, _) => entry.keywords.push(term),
                            _ => (),
                        }
                    },
                    "contributor" => (),
                    "rights" => (),
                    "source" => (),
                    _ => (),
                }
            },
            _ => (),
        }
    }
    Some(entry)
}
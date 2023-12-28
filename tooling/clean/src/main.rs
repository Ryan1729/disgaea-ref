type Res<A> = Result<A, Box<dyn std::error::Error>>;

fn main() -> Res<()> {
    let input_path = "input.html";

    let html_string = std::fs::read_to_string(input_path)?;

    let mut dom = tl::parse(&html_string, Default::default())?;

    for node in dom.nodes_mut() {
        use tl::Node::*;
        match node {
            Tag(tag) => {
                if tag.name() == "a" {
                    let attributes = tag.attributes_mut();
                    match attributes.get("href") {
                        // Id links are probably worth keeping.
                        Some(Some(value)) if value.as_bytes().starts_with(b"#") => {}
                        // Most links are probably internal ones that probably break
                        // So remove the link only since the text may be useful
                        Some(Some(_)) => {
                            attributes.remove("href");
                        }
                        // Seems like a tags with no href are nav things that are 
                        // broken when copying just the HTML
                        None | Some(None) => {
                            *node = Raw(tl::Bytes::new());
                            continue
                        }
                    }                    
                }
                
                let attributes = tag.attributes_mut();
                // Clear off the attributes we know we don't care about
                for key in [
                    "class",
                    "tabindex",
                    "aria-expanded",
                    "data-action",
                    "data-tracking-label",
                    "xlink:href",
                ] {
                    attributes.remove(key);
                }

                // Remove tags with attrs that we know make the tag useless
                if let Some(Some(bytes)) = attributes.get("role") {
                    if bytes.as_bytes() == b"button" {
                        *node = Raw(tl::Bytes::new());
                    }
                }
            },
            Raw(bytes) => {
                // Keep it, unless it seems useless.
                match bytes.as_bytes() {
                    // Seems like length one things are all for nav stuff that is 
                    // doesn't work when copying the HTML anyway.
                    b if b.len() <= 1 => {
                        *node = Raw(tl::Bytes::new());
                    }
                    _ => {}
                }
            },
            Comment(_bytes) => {
                // We don't need comments so effectively remove it.
                *node = Raw(tl::Bytes::new());
            },
        }
    }

    println!("{}", dom.outer_html());

    Ok(())
}

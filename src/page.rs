use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use yaml_front_matter::YamlFrontMatter;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub layout: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Page {
    pub raw: String,
    pub data: FrontMatter,
    pub path: PathBuf,
    pub contents: String,
}

impl Page {
    pub fn read(path: &PathBuf) -> Page {
        let file_contents = fs::read_to_string(path).unwrap();

        if let Ok(result) = YamlFrontMatter::parse::<FrontMatter>(&file_contents) {
            Page {
                raw: String::from(&file_contents),
                data: result.metadata,
                path: path.clone(),
                contents: result.content,
            }
        } else {
            Page {
                raw: String::from(&file_contents),
                data: FrontMatter {
                    layout: None,
                    title: None,
                },
                path: path.clone(),
                contents: String::from(&file_contents),
            }
        }
    }

    pub fn render(&self) -> Option<String> {
        match self.path.extension().unwrap().to_str().unwrap() {
            "html" => {
                if let Ok(template) = liquid::ParserBuilder::with_stdlib()
                    .build()
                    .unwrap()
                    .parse(self.raw.as_str())
                {
                    let globals = liquid::object!({});
                    return Some(template.render(&globals).unwrap());
                }
                None
            }
            "md" => {
                let parser = Parser::new_ext(&self.contents, Options::all());
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                Some(html_output)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::{FrontMatter, Page};

    #[test]
    fn reads_raw_data_from_file() {
        let path = Path::new("/tmp/file.md");
        let contents = String::from(
            "
		# My heading

		This is my content

		## Subheading

		This is my second-level content

		* List item 1
		* List item 2
		* List item 3
		",
        );

        assert!(fs::write(&path, &contents).is_ok());

        let expected = Page {
            raw: String::from(&contents),
            data: FrontMatter {
                layout: None,
                title: None,
            },
            path: path.to_path_buf(),
            contents: String::from(&contents),
        };

        assert_eq!(expected, Page::read(&path.to_path_buf()));

        assert!(fs::remove_file(path).is_ok());
    }

    #[test]
    fn separates_frontmatter_from_data() {
        let path = Path::new("/tmp/file_2.md");

        let contents = "---
title: My page title
layout: a-layout.liquid
---
# My heading

This is my content

## Subheading

This is my second-level content

* List item 1
* List item 2
* List item 3";

        assert!(fs::write(&path, &contents).is_ok());

        let expected = Page {
            raw: String::from(contents),
            data: FrontMatter {
                title: Some(String::from("My page title")),
                layout: Some(String::from("a-layout.liquid")),
            },
            path: path.to_path_buf(),
            contents: String::from(
                "# My heading

This is my content

## Subheading

This is my second-level content

* List item 1
* List item 2
* List item 3",
            ),
        };

        assert_eq!(expected, Page::read(&path.to_path_buf()));
        assert!(fs::remove_file(path).is_ok());
    }
}

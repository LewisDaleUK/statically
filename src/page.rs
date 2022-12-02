use pulldown_cmark::{html, Options, Parser};
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{ Path, PathBuf };
use yaml_front_matter::YamlFrontMatter;

#[derive(Debug)]
pub struct GlobalData {
    content: Option<String>,
}

impl GlobalData {
    pub fn empty() -> GlobalData {
        GlobalData { content: None }
    }
}

#[derive(Debug)]
pub struct Dirs {
    pub output: PathBuf,
    pub input: PathBuf,
    pub includes: PathBuf,
}

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
        let matter = Matter::<YAML>::new();

        if let Some(result) = matter.parse_with_struct::<FrontMatter>(&file_contents) {
            Page {
                raw: String::from(&file_contents),
                data: result.data,
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

    fn parse_liquid(&self, globals: GlobalData) -> Option<String> {
        if let Ok(template) = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(&self.contents.as_str())
        {
            let data = liquid::object!({
                "content": globals.content.unwrap_or(String::from(""))
            });
            return Some(template.render(&data).unwrap());
        }
        None
    }

    pub fn render(&self, dirs: &Dirs, globals: GlobalData) -> Option<String> {
        let content = match self.path.extension().unwrap().to_str().unwrap() {
            "html" => self.parse_liquid(globals),
            "liquid" => self.parse_liquid(globals),
            "md" => {
                let parser = Parser::new_ext(&self.contents, Options::all());
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                Some(html_output)
            },
            _ => None,
        };

        match &self.data.layout {
            Some(layout) => {
                let path = dirs.includes.join(Path::new(layout));
                println!("Attempting to read layout {:#?}", path);
                let layout_page = Page::read(&path);
                layout_page.render(dirs, GlobalData { content: content })
            },
            _ => content,
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

use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct FrontMatter {
    title: Option<String>,
    layout: Option<String>,
}

impl FrontMatter {
    pub fn from_str(contents: &str) -> FrontMatter {    
        if let Ok(result) = YamlFrontMatter::parse::<FrontMatter>(contents) {
            result.metadata
        } else {
            FrontMatter { title: None, layout: None }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrontMatter;
    #[test]
    fn it_can_be_created_from_an_empty_str() {
        let expected = FrontMatter { title: None, layout: None };
        assert_eq!(expected, FrontMatter::from_str(""))
    }

    #[test]
    fn it_parses_title_and_layout() {
        let expected = FrontMatter {
            title: Some(String::from("My title")),
            layout: Some(String::from("a-layout.liquid"))
        };
        let contents: &str = "---
        title: My title
        layout: a-layout.liquid
        ---
        
        # My heading";

        assert_eq!(expected, FrontMatter::from_str(contents))
    }

    #[test]
    fn it_parses_only_title_and_defaults_layout() {
        let expected = FrontMatter {
            title: Some(String::from("My title")),
            layout: None
        };
        let contents = "---
        title: My title
        ---
        
        # A heading";

        assert_eq!(expected, FrontMatter::from_str(contents))
    }

    #[test]
    fn it_discards_data_we_dont_care_about() {
        let expected = FrontMatter {
            title: Some(String::from("A title")),
            layout: Some(String::from("a-different-layout.liquid")),
        };
        let contents = "---
        title: A title
        layout: a-different-layout.liquid
        extra: Data i dont care about
        ---
        
        # My heading";

        assert_eq!(expected, FrontMatter::from_str(contents))
    }
}
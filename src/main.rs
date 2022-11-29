use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::{Path, PathBuf};

mod frontmatter;

fn read_file(path: &PathBuf) -> Option<String> {
    let contents = fs::read_to_string(path).unwrap();

    let extension = path.extension().unwrap();

    // TODO: The frontmatter library also gives us document contents as a parameter,
    // This flow should be rewritten so the parse happens at the top-level
    let data = frontmatter::FrontMatter::from_str(&contents);
    println!("{:#?}", data);

    match extension.to_str().unwrap() {
        "html" => {
            let template = liquid::ParserBuilder::with_stdlib()
                .build()
                .unwrap()
                .parse(&contents)
                .unwrap();
            let globals = liquid::object!({
            "content": "<p>This is a liquid-rendered website</p>"
            });
            Some(template.render(&globals).unwrap())
        }
        "md" => {
            let parser = Parser::new_ext(&contents, Options::all());
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            Some(html_output)
        }
        _ => None,
    }
}

fn read_dir(dir: PathBuf, out_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !out_dir.exists() {
        fs::create_dir(out_dir)?;
    }
    for entry in dir
        .read_dir()
        .expect("Reading source directory failed - does it exist?")
        .flatten()
    {
        let path = entry.path();
        if path.is_file() {
            let output = read_file(&path);

            if let Some(output) = output {
                let mut out_file = out_dir.join(path.file_name().unwrap());
                out_file.set_extension("html");
                println!("Writing {}", out_file.to_str().unwrap());
                fs::write(out_file, output)?;
            }
        } else {
            read_dir(
                entry.path(),
                &out_dir.join(entry.path().file_name().unwrap()),
            )?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = Path::new("./website/");

    let source_dir = root_dir.join("src");
    let out_dir = root_dir.join("dist");

    if !out_dir.exists() {
        fs::create_dir(&out_dir)?;
    }

    read_dir(source_dir, &out_dir)?;
    Ok(())
}

use std::fs;
use std::path::{Path, PathBuf};

mod page;

fn read_file(path: &PathBuf, dirs: &page::Dirs) -> Option<String> {
    let page = page::Page::read(path);
    page.render(dirs, page::GlobalData::empty())
}

fn read_dir(
    dir: &PathBuf,
    out_dir: &PathBuf,
    dirs: &page::Dirs,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.starts_with(&dirs.includes.as_path()) {
        return Ok(());
    }

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
            let output = read_file(&path, dirs);

            if let Some(output) = output {
                let mut out_file = out_dir.join(path.file_name().unwrap());
                out_file.set_extension("html");
                println!("Writing {}", out_file.to_str().unwrap());
                fs::write(out_file, output)?;
            } else {
                println!("Skipping {}", path.to_str().unwrap());
            }
        } else {
            read_dir(
                &entry.path(),
                &out_dir.join(entry.path().file_name().unwrap()),
                dirs,
            )?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = Path::new("./website/");

    let source_dir = root_dir.join("src");
    let out_dir = root_dir.join("dist");
    let includes_dir = source_dir.join("_includes");

    let dirs = page::Dirs {
        includes: includes_dir,
        output: out_dir,
        input: source_dir,
    };

    if !dirs.output.exists() {
        fs::create_dir(&dirs.output)?;
    }

    read_dir(&dirs.input, &dirs.output, &dirs)?;
    Ok(())
}

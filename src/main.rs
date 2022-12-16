use std::{
    env, error, fs,
    path::{Path, PathBuf, StripPrefixError},
};

use handlebars::Handlebars;
use serde_json::json;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut registry = Handlebars::new();

    registry.register_template_file("layout", "templates/layout.html.hbs")?;

    let input = Path::new("pages");
    let output = Path::new("build");

    let html_base = env::current_dir().unwrap().join(output);

    fs::remove_dir_all(output)?;

    for entry in WalkDir::new(input).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());

        let new_path = replace_prefix(entry.path(), input, output)?;

        if entry.path().is_dir() {
            fs::create_dir_all(new_path)?
        } else {
            let extension = entry.path().extension().unwrap_or_default();

            if extension == "hbs" {
                let template_name = new_path.to_str().unwrap().replace(".hbs", "");

                registry.register_template_file(&template_name, entry.path())?;
                let mut output_file = fs::File::create(template_name.clone())?;
                registry.render_to_write(
                    &template_name,
                    &json!({
                        "base": format!("{}\\", html_base.to_str().unwrap()),
                        "layout": "layout",
                    }),
                    &mut output_file,
                )?;
            } else {
                fs::copy(entry.path(), new_path)?;
            }
        };
    }

    Ok(())
}

fn replace_prefix(
    p: impl AsRef<Path>,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
) -> Result<PathBuf, StripPrefixError> {
    p.as_ref().strip_prefix(from).map(|p| to.as_ref().join(p))
}

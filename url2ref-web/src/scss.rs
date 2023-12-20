use std::env::VarError;
use std::fs::File;
use std::io::Write;
use std::env;
use std::path::Path;

use dotenv;

use grass;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilationError {
    #[error("Environment variable not set")]
    EnvironmentError(#[from] VarError),

    #[error("Could not create file")]
    FileCreateError(#[source] std::io::Error),
    #[error("Could not write to file")]
    FileWriteError(#[source] std::io::Error),

    #[error("{}", .0.to_string())]
    Failed(#[from] Box<grass::Error>)
}

// No native support for SCSS in Rocket.
// Paths are also used in Tera templates, so environment variables are
// used to share state (see .cargo/config.toml).
pub fn compile() -> Result<(), CompilationError> {
    // Loading environment vars
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dotenv_path = Path::new(&manifest_dir).join(".env");
    dotenv::from_path(&dotenv_path).ok();

    let css_path = Path::new(&manifest_dir).join(env::var("MAIN_CSS_PATH")?);
    let scss_path = Path::new(&manifest_dir).join(env::var("MAIN_SCSS_PATH")?);

    use CompilationError::{FileCreateError, FileWriteError};

    let mut output_file = File::create(css_path)
            .map_err(FileCreateError)?;

    let css = grass::from_path(scss_path, &grass::Options::default())?;
    write!(output_file, "{}", css).map_err(FileWriteError)?;

    Ok(())
}


use thiserror::Error;
use std::env::{VarError, self};
use std::fs::File;
use std::io::Write;
use grass;

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
pub fn compile(scss_path_key: &str, css_path_key: &str) -> Result<(), CompilationError> {
    use CompilationError::{FileCreateError, FileWriteError};

    let css_path = env::var(css_path_key)?;
    let scss_path = env::var(scss_path_key)?;
    let mut output_file = File::create(css_path)
            .map_err(FileCreateError)?;

    let css = grass::from_path(scss_path, &grass::Options::default())?;
    write!(output_file, "{}", css).map_err(FileWriteError)?;

    Ok(())
}


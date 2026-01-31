use std::env::VarError;
use std::fs::File;
use std::io::Write;
use std::env;
use std::path::Path;

use grass;
use thiserror::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    paths: PathsConfig,
}

#[derive(Deserialize)]
struct PathsConfig {
    main_css: String,
    main_scss: String,
}

#[derive(Error, Debug)]
pub enum CompilationError {
    #[error("Environment variable not set")]
    EnvironmentError(#[from] VarError),

    #[error("Could not read config file")]
    ConfigReadError(#[source] std::io::Error),
    #[error("Could not parse config file")]
    ConfigParseError(#[from] toml::de::Error),

    #[error("Could not create file")]
    FileCreateError(#[source] std::io::Error),
    #[error("Could not write to file")]
    FileWriteError(#[source] std::io::Error),

    #[error("{}", .0.to_string())]
    Failed(#[from] Box<grass::Error>)
}

// No native support for SCSS in Rocket.
// Paths are defined in config.toml
pub fn compile() -> Result<(), CompilationError> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    
    // Load config from config.toml
    let config_path = Path::new(&manifest_dir).join("config.toml");
    let config_content = std::fs::read_to_string(&config_path)
        .map_err(CompilationError::ConfigReadError)?;
    let config: Config = toml::from_str(&config_content)?;

    let css_path = Path::new(&manifest_dir).join(&config.paths.main_css);
    let scss_path = Path::new(&manifest_dir).join(&config.paths.main_scss);
    
    // Set environment variables for Tera templates
    env::set_var("MAIN_CSS_PATH", &config.paths.main_css);

    use CompilationError::{FileCreateError, FileWriteError};

    let mut output_file = File::create(css_path)
            .map_err(FileCreateError)?;

    let css = grass::from_path(scss_path, &grass::Options::default())?;
    write!(output_file, "{}", css).map_err(FileWriteError)?;

    Ok(())
}


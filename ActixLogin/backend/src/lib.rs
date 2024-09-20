pub mod routes;
pub mod settings;
pub mod startup;
pub mod telemetry;
pub mod utils;
pub mod types;
pub mod database;

use std::{path::Path, fs};
use once_cell::sync::Lazy;
use minijinja::Environment;

fn load_templates_from_directory(env: &mut Environment<'static>, dir: &Path) -> Result<(), std::io::Error> {
    tracing::event!(target: "backend", tracing::Level::INFO, "Getting DIRs");
    for entry in fs::read_dir(dir)? {
        tracing::event!(target: "backend", tracing::Level::INFO, "ENTRY: {:?}", entry);
        let entry = entry?;
        println!("{:?}", entry);
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                let name = Box::leak(Box::new(name.to_string()));
                let content = fs::read_to_string(&path)?;
                let content = Box::leak(Box::new(content));
                tracing::event!(target: "backend", tracing::Level::INFO, "NAME: {:?}", name);
                tracing::event!(target: "backend", tracing::Level::INFO, "CONTENT: {:?}", content);
                env.add_template(name, content).expect("Failed to add template");
            }
        }
    }
    Ok(())
}

pub static ENV: Lazy<minijinja::Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    load_templates_from_directory(&mut env, Path::new("templates")).unwrap();
    env
});
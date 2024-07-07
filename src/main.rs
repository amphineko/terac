mod args;
mod compile;
mod json;

use crate::args::IncludedTemplate;
use crate::compile::compile;
use clap::{arg, Parser};
use serde_json::Value;
use std::{collections::HashMap, error::Error, io::Read, path::PathBuf};

#[derive(Parser)]
struct Args {
    /// Files containing values to be used in the template
    #[arg(short = 'a', long = "values")]
    value_files: Vec<String>,

    /// Included templates to be rendered and used in the main template as `.includes`
    #[arg(short = 'i', long = "include")]
    included_template_files: Vec<IncludedTemplate>,

    /// Output file of the rendered template
    #[arg(short='o', long="output", default_value=None)]
    output_file: Option<PathBuf>,

    /// Template file to render
    template_file: Option<PathBuf>,
}

fn load_file(file: Option<PathBuf>) -> Result<String, Box<dyn Error>> {
    match &file {
        Some(file) => {
            let content = std::fs::read_to_string(file)?;
            Ok(content)
        }
        None => {
            let mut content = String::new();
            std::io::stdin().read_to_string(&mut content)?;
            Ok(content)
        }
    }
}

fn load_values_from_file(file: PathBuf) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_str(&std::fs::read_to_string(file)?)?)
}

fn load_values_from_files(files: Vec<String>) -> Result<Vec<Value>, (String, Box<dyn Error>)> {
    files
        .into_iter()
        .map(|file| match load_values_from_file(file.clone().into()) {
            Ok(value) => Ok(value),
            Err(err) => Err((file, err)),
        })
        .collect()
}

fn write_output(output_file: Option<PathBuf>, output: String) -> Result<(), Box<dyn Error>> {
    let mut output_writer: Box<dyn std::io::Write> = match output_file {
        Some(file) => Box::new(std::fs::File::create(file)?),
        None => Box::new(std::io::stdout()),
    };

    output_writer.write_all(output.as_bytes())?;
    output_writer.flush()?;

    Ok(())
}

fn parse_args_and_compile() -> Result<(), String> {
    let Args {
        included_template_files,
        output_file,
        template_file,
        value_files,
    } = Args::parse();

    let template =
        load_file(template_file).map_err(|err| format!("Failed to load template: {}", err))?;

    let includes: HashMap<String, String> = included_template_files
        .into_iter()
        .map(
            |IncludedTemplate { name, path }| match load_file(Some(path)) {
                Ok(content) => Ok((name, content)),
                Err(err) => Err(format!("Failed to load included template: {}", err)),
            },
        )
        .collect::<Result<HashMap<String, String>, _>>()?;

    let values: Vec<serde_json::Value> = load_values_from_files(value_files)
        .map_err(|(filename, err)| format!("Failed to load values from {}: {}", filename, err))?;

    let output = compile(&template, &includes, &values)
        .map_err(|err| format!("Failed to compile template: {}", err))?;

    write_output(output_file, output).map_err(|err| format!("Failed to write output: {}", err))?;

    Ok(())
}

fn main() {
    if let Err(err) = parse_args_and_compile() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

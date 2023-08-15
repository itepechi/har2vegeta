mod har;
mod json;

use base64::{engine::general_purpose, Engine as _};
use clap::{Parser, ValueEnum};
use har::Har;
use std::{
    collections::HashMap,
    io::{self, stdout, Read, Write},
};
use tempfile::NamedTempFile;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    /// Human readable, easy to edit, but may create temporary files
    Http,
    /// Less human readable, but works without temporary files
    Json,
    /// Same as http
    H,
    /// Same as json
    J,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_enum, default_value_t = Format::Http)]
    format: Format,
}

fn main() {
    let cli = Cli::parse();

    let mut input = String::new();
    let mut stdin = io::stdin().lock();
    let mut stdout = stdout().lock();

    stdin.read_to_string(&mut input).unwrap();

    let har: Har = serde_json::from_str(&input).unwrap();

    let entries = har.log.entries.into_iter();

    match cli.format {
        Format::Json | Format::J => {
            for entry in entries {
                let json = json::Json {
                    method: entry.request.method,
                    url: entry.request.url,
                    header: HashMap::from_iter(
                        entry
                            .request
                            .headers
                            .into_iter()
                            .map(|header| (header.name, vec![header.value])),
                    ),
                    body: entry
                        .request
                        .post_data
                        .map(|data| general_purpose::STANDARD.encode(data.text)),
                };

                writeln!(stdout, "{}", serde_json::to_string(&json).unwrap()).unwrap();
            }
        }
        Format::Http | Format::H => {
            for (i, entry) in entries.enumerate() {
                if i != 0 {
                    write!(stdout, "\n").unwrap();
                }

                writeln!(
                    stdout,
                    "{} {}{}{}",
                    entry.request.method,
                    entry.request.url,
                    entry
                        .request
                        .headers
                        .into_iter()
                        .fold(String::new(), |prev, header| format!(
                            "{}\n{}: {}",
                            prev, header.name, header.value
                        )),
                    match entry.request.post_data {
                        Some(body) => {
                            let file = NamedTempFile::new().unwrap();
                            let location = format!("\n@{}", &file.path().to_string_lossy());

                            writeln!(&file, "{}", body.text).unwrap();
                            file.keep().unwrap();

                            location
                        }
                        None => String::from(""),
                    }
                )
                .unwrap();
            }
        }
    }
}

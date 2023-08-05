use ::anyhow::{bail, Context, Result};
use ::clap::{Parser, Subcommand};
use ::std::io;
use ::toml;
use ::toml_edit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Override {
        #[arg(short, long)]
        in_toml: Option<String>,
        #[arg(short, long)]
        override_toml: Option<String>,
        #[arg(long)]
        override_toml_dot: Option<String>,
        #[arg(long)]
        override_toml_dot_type: Option<String>,
        #[arg(long)]
        to_stdout: Option<bool>,
    },
    Debug {
        #[arg(short, long)]
        fizz: Option<String>,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Override {
            in_toml,
            override_toml,
            override_toml_dot: toml_dot,
            override_toml_dot_type: toml_dot_type,
            to_stdout,
        } => {
            let base_toml_str;
            match in_toml {
                Some(x) => {
                    base_toml_str = x;
                }
                None => {
                    // get from stdin
                    let mut buffer = String::new();
                    for line in io::stdin().lines() {
                        buffer.push_str(
                            &line.with_context(|| "Failed to read from stdin".to_string())?,
                        );
                        buffer.push_str("\n");
                    }
                    base_toml_str = buffer;
                }
            }

            let base_toml = toml::from_str::<toml::Value>(&base_toml_str)
                .with_context(|| format!("Failed to parse base_toml: {:?}", base_toml_str))?;

            let mut out_toml = base_toml_str
                .parse::<toml_edit::Document>()
                .with_context(|| {
                    format!(
                        "Failed to parse toml as toml_edit::Document: {:?}",
                        base_toml_str
                    )
                })?;
            match (override_toml, toml_dot, toml_dot_type) {
                (Some(toml_path), None, None) => {
                    let override_toml = std::fs::read_to_string(&toml_path)
                        .with_context(|| format!("Failed to read toml: {:?}", toml_path))?;
                    let override_toml = toml::from_str::<toml::Value>(&override_toml)
                        .with_context(|| {
                            format!("Failed to parse override_toml: {:?}", override_toml)
                        })?;
                    println!("override_toml: {:?}", override_toml);

                    // println!("{:?}", out_toml);
                    println!("{:?}", out_toml["disabled_plugins"]);
                    out_toml["disabled_plugins"] =
                        toml_edit::value(toml_edit::Array::from_iter(vec!["foo"]));

                    // for (k, v) in override_toml.as_table().unwrap() {
                    //     println!("k: {:?}, v: {:?}", k, v);
                    //     out_toml[k] = toml_edit::value(v.clone());
                    // }
                }
                (None, Some(_), Some(_)) => {
                    bail!("not implemented yet")
                }
                _ => {
                    bail!("'toml' or ('toml_dot' and 'toml_dot_type') is required")
                }
            }

            let to_stdout = to_stdout.unwrap_or(true); // default to stdout
            if to_stdout {
                println!("{}", out_toml.to_string());
            } else {
                // write to file
                bail!("not implemented yet")
            }
            Ok(())
        }
        Commands::Debug { fizz } => {
            println!("fizz: {:?}", fizz);
            Ok(())
        }
    }
}

use ::anyhow::{bail, Context, Result};
use ::clap::{Parser, Subcommand};
use ::std::io;
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
                    // let override_toml = toml::from_str::<toml::Value>(&override_toml)
                    //     .with_context(|| {
                    //         format!("Failed to parse override_toml: {:?}", override_toml)
                    //     })?;
                    let override_toml =
                        override_toml
                            .parse::<toml_edit::Document>()
                            .with_context(|| {
                                format!(
                                    "Failed to parse toml as toml_edit::Document: {:?}",
                                    override_toml
                                )
                            })?;

                    // override_toml =
                    for (k, v) in override_toml.iter() {
                        out_toml[k] = override_toml_values(
                            &out_toml.as_item()[k],
                            v,
                            &default_override_toml_options(),
                        )?;
                    }
                    println!("override_toml: {:?}", override_toml);

                    for (k, v) in override_toml.iter() {
                        println!("item: {:?}, {:?}", k, v);
                    }

                    out_toml["disabled_plugins"] =
                        toml_edit::value(toml_edit::Array::from_iter(vec!["foo"]));
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

fn is_same_enum(a: &toml_edit::Item, b: &toml_edit::Item) -> bool {
    match (a, b) {
        (toml_edit::Item::Value(_), toml_edit::Item::Value(_)) => true,
        (toml_edit::Item::Table(_), toml_edit::Item::Table(_)) => true,
        (toml_edit::Item::ArrayOfTables(_), toml_edit::Item::ArrayOfTables(_)) => true,
        _ => false,
    }
}

fn override_toml_values(
    orig_item: &toml_edit::Item,
    override_item: &toml_edit::Item,
    options: &OverrideTomlOptions,
) -> Result<toml_edit::Item> {
    let mut out_toml = orig_item.clone();

    // if the section does not exist in original, just copy the override
    if orig_item.is_none() {
        out_toml = override_item.clone();
    }

    if !is_same_enum(orig_item, override_item) {
        match options.allow_override_type {
            true => {
                // just copy the override
                out_toml = override_item.clone();
                return Ok(out_toml);
            }
            false => {
                bail!(
                    "override_toml type does not match original toml: {:?} vs {:?}",
                    orig_item,
                    override_item
                )
            }
        }
    }

    match override_item {
        toml_edit::Item::Table(v) => {
            for (k, v) in v.iter() {
                // if out_toml[k].is_none() {
                //     out_toml[k] = v.clone();
                // } else {
                out_toml[k] = override_toml_values(&out_toml[k], v, options)?;
                // }
                // out_toml[k] = override_toml_values(out_toml[k], v)
                //     .with_context(format!("Failed to override toml: {:?}", v))?
            }
        }
        _ => {
            out_toml = override_item.clone();
        }
    }

    Ok(out_toml)
}

struct OverrideTomlOptions {
    allow_override_type: bool,
}

fn default_override_toml_options() -> OverrideTomlOptions {
    OverrideTomlOptions {
        allow_override_type: false,
    }
}

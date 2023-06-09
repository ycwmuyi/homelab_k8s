use crate::{arguments::OutputType, helpers, kind, operator, operator::Operator, CliArgs};
use cached::proc_macro::cached;
use clap::{ArgGroup, Parser, ValueHint};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    Cell, ContentArrangement, Table,
};
use indexmap::IndexMap;
use lazy_static::lazy_static;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::{error::Error, ops::Deref, process::exit, sync::Mutex};

lazy_static! {
    pub static ref RELEASE_FILES: Mutex<Vec<String>> = Mutex::new(vec![
        "https://raw.githubusercontent.com/stackabletech/release/main/releases.yaml".to_string()
    ]);
}

#[derive(Parser)]
pub enum CliCommandRelease {
    /// List all the available releases
    #[command(alias("ls"))]
    List {
        #[arg(short, long, value_enum, default_value = "text")]
        output: OutputType,
    },
    /// Show details of a specific release
    #[command(alias("desc"))]
    Describe {
        /// Name of the release to describe
        #[arg(required = true, value_hint = ValueHint::Other)]
        release: String,

        #[arg(short, long, value_enum, default_value = "text")]
        output: OutputType,
    },
    /// Install a specific release
    #[command(alias("in"))]
    #[command(group(
        ArgGroup::new("list_of_products")
            .required(false)
            .args(&["include_products", "exclude_products"]),
    ))]
    Install {
        /// Name of the release to install
        #[arg(required = true, value_hint = ValueHint::Other)]
        release: String,

        /// Whitelist of product operators to install.
        /// Mutually exclusive with `--exclude-products`
        #[arg(short, long, value_hint = ValueHint::Other)]
        include_products: Vec<String>,

        /// Blacklist of product operators to install.
        /// Mutually exclusive with `--include-products`
        #[arg(short, long, value_hint = ValueHint::Other)]
        exclude_products: Vec<String>,

        /// If specified, a local Kubernetes cluster consisting of 4 nodes (1 for control-plane and 3 workers) for testing purposes will be created.
        /// Kind is a tool to spin up a local Kubernetes cluster running on Docker on your machine.
        /// You need to have `docker` and `kind` installed.
        /// Have a look at our documentation on how to install `kind` at <https://docs.stackable.tech/home/getting_started.html#_installing_kubernetes_using_kind>
        #[arg(short, long)]
        kind_cluster: bool,

        /// Name of the kind cluster created if `--kind-cluster` is specified
        #[arg(
            long,
            default_value = "stackable-data-platform",
            requires = "kind_cluster",
            value_hint = ValueHint::Other,
        )]
        kind_cluster_name: String,
    },
    /// Uninstall a release
    #[command(alias("un"))]
    Uninstall {
        /// Name of the release to uninstall
        #[arg(required = true, value_hint = ValueHint::Other)]
        release: String,
    },
}

impl CliCommandRelease {
    pub async fn handle(&self) -> Result<(), Box<dyn Error>> {
        match self {
            CliCommandRelease::List { output } => list_releases(output).await?,
            CliCommandRelease::Describe { release, output } => {
                describe_release(release, output).await?
            }
            CliCommandRelease::Install {
                release,
                include_products,
                exclude_products,
                kind_cluster,
                kind_cluster_name,
            } => {
                kind::handle_cli_arguments(*kind_cluster, kind_cluster_name)?;
                install_release(release, include_products, exclude_products).await?;
            }
            CliCommandRelease::Uninstall { release } => uninstall_release(release).await,
        }

        Ok(())
    }
}

pub fn handle_common_cli_args(args: &CliArgs) {
    let mut release_files = RELEASE_FILES.lock().unwrap();
    release_files.extend_from_slice(&args.additional_releases_file);
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Releases {
    releases: IndexMap<String, Release>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Release {
    release_date: String,
    description: String,
    products: IndexMap<String, ReleaseProduct>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReleaseProduct {
    operator_version: String,
}

async fn list_releases(output_type: &OutputType) -> Result<(), Box<dyn Error>> {
    let output = get_releases().await;
    match output_type {
        OutputType::Text => {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("Release"),
                    Cell::new("Release date"),
                    Cell::new("Description"),
                ]);
            for (release_name, release) in output.releases {
                table.add_row(vec![
                    Cell::new(release_name),
                    Cell::new(release.release_date),
                    Cell::new(release.description),
                ]);
            }
            println!("{table}");
        }
        OutputType::Json => {
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputType::Yaml => {
            println!("{}", serde_yaml::to_string(&output)?);
        }
    }

    Ok(())
}

async fn describe_release(
    release_name: &str,
    output_type: &OutputType,
) -> Result<(), Box<dyn Error>> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Output {
        release: String,
        release_date: String,
        description: String,
        products: IndexMap<String, ReleaseProduct>,
    }

    let release = get_release(release_name).await;
    let output = Output {
        release: release_name.to_string(),
        release_date: release.release_date,
        description: release.description,
        products: release.products,
    };

    match output_type {
        OutputType::Text => {
            let mut table = Table::new();
            table
                .load_preset(NOTHING)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .add_row(vec![Cell::new("Release"), Cell::new(output.release)])
                .add_row(vec![
                    Cell::new("Release date"),
                    Cell::new(output.release_date),
                ])
                .add_row(vec![
                    Cell::new("Description"),
                    Cell::new(output.description),
                ])
                .add_row(vec![Cell::new("Included products")])
                .add_row(vec![""]);
            println!("{table}");

            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![Cell::new("Product"), Cell::new("Operator version")]);
            for (product_name, product) in output.products {
                table.add_row(vec![
                    Cell::new(product_name),
                    Cell::new(product.operator_version),
                ]);
            }
            println!("{table}");
        }
        OutputType::Json => {
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputType::Yaml => {
            println!("{}", serde_yaml::to_string(&output)?);
        }
    }

    Ok(())
}

/// If include_operators is an non-empty list only the whitelisted product operators will be installed.
/// If exclude_operators is an non-empty list the blacklisted product operators will be skipped.
pub async fn install_release(
    release_name: &str,
    include_products: &[String],
    exclude_products: &[String],
) -> Result<(), Box<dyn Error>> {
    info!("Installing release {release_name}");
    let release = get_release(release_name).await;

    for (product_name, product) in release.products {
        let included = include_products.is_empty() || include_products.contains(&product_name);
        let excluded = exclude_products.contains(&product_name);

        if included && !excluded {
            Operator::new(product_name, Some(product.operator_version))
                .expect("Failed to construct operator definition")
                .install()?;
        }
    }

    Ok(())
}

async fn uninstall_release(release_name: &str) {
    info!("Uninstalling release {release_name}");
    let release = get_release(release_name).await;

    operator::uninstall_operators(&release.products.into_keys().collect());
}

/// Cached because of potential slow network calls
#[cached]
async fn get_releases() -> Releases {
    let mut all_releases = IndexMap::new();
    let release_files = RELEASE_FILES.lock().unwrap().deref().clone();
    for release_file in release_files {
        let yaml = helpers::read_from_url_or_file(&release_file).await;
        match yaml {
            Ok(yaml) => match serde_yaml::from_str::<Releases>(&yaml) {
                Ok(releases) => all_releases.extend(releases.releases),
                Err(err) => warn!("Failed to parse release list from {release_file}: {err}"),
            },
            Err(err) => {
                warn!("Could not read from releases file \"{release_file}\": {err}");
            }
        }
    }

    Releases {
        releases: all_releases,
    }
}

async fn get_release(release_name: &str) -> Release {
    get_releases()
    .await
        .releases
        .remove(release_name) // We need to remove to take ownership
        .unwrap_or_else(|| {
            error!("Release {release_name} not found. Use `stackablectl release list` to list the available releases.");
            exit(1);
        })
}

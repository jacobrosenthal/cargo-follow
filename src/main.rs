//! Note crates only allows 1 second per request so this tends to take several+
//! minutes.
//! ideally also generate an index.js turned into star and watch github
//! buttons

use cargo_lock::Lockfile;
use crates_io_api::SyncClient;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::process::Command;

fn main() {
    generate();
    let lockfile = Lockfile::load("Cargo.lock").unwrap();

    let client = SyncClient::new(
        "cargo-follow (jacobrosenthal@gmail.com)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();

    let mut packages = lockfile.packages;
    packages.sort_by(|a, b| b.name.cmp(&a.name));

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(80);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.blue} {msg}"),
    );
    pb.set_message("pinging crates.io api at 1 req/s");

    let mut packages: Vec<String> = packages
        .iter()
        //dedup multiple versions of dep
        .dedup_by(|x, y| x.name == y.name)
        //look up crate on crates.io
        .map(|p| {
            let c = client.full_crate(p.name.as_str(), false).unwrap();
            pb.inc(1);
            c
        })
        //just return unwrapped repository
        .filter_map(|c|c.repository).collect();

    pb.finish();

    //often repos have several packages, so need to sort and dedup again
    packages.sort();
    packages.iter().dedup().for_each(|repo| {
        println!("{}", repo);
    });
}

pub fn generate() {
    let status = Command::new("cargo")
        .arg("generate-lockfile")
        .status()
        .unwrap();

    if !status.success() {
        if let Some(code) = status.code() {
            panic!(
                "non-zero exit status running `cargo generate-lockfile`: {}",
                code
            );
        } else {
            panic!("no exit status running `cargo generate-lockfile`!");
        }
    }
}

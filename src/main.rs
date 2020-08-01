use cargo_lock::Lockfile;
use crates_io_api::SyncClient;
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

    packages
        .iter()
        .dedup_by(|x, y| x.name == y.name)
        .for_each(|p| {
            let dep = client.full_crate(p.name.as_str(), false).unwrap();
            println!("{}", dep.repository.unwrap_or_default());
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

mod query;

use std::{env, path::Path};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside the workspace");

    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["query", "pull"] => query::pull(root),
        ["query", "update"] => query::update(root),
        ["--help" | "-h"] => {
            println!("Usage: cargo xtask query <pull|update>");
            Ok(())
        }
        _ => bail!("usage: cargo xtask query <pull|update>"),
    }
}

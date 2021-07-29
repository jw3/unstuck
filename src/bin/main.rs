use clap::Clap;

use unstuck::{api, find, fix, yolo};
use std::str::FromStr;

#[derive(Clap)]
#[clap(name = "unstuck", version="0.0.0")]
/// Unstick a stuck namespace
struct Opts {
    /// The namespace
    namespace: String,

    /// The action to take (find, fix, force)
    action: Op,
}

#[derive(Clap)]
enum Op {
    Find,
    Fix,
    Force,
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "find" => Ok(Op::Find),
            "fix" => Ok(Op::Fix),
            "force" => Ok(Op::Force),
            _ => Err("no match"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), api::Error> {
    let opts: Opts = Opts::parse();
    match opts.action {
        Op::Find => find::whats_stuck(&opts.namespace).await,
        Op::Fix => fix::surgical_strike(&opts.namespace).await,
        Op::Force => yolo::brute_force_fix(&opts.namespace).await,
    }
}

use clap::{AppSettings, Parser};
use cli::{Args, RunCmd};

const BUILD_TIME: &str = include!(concat!(env!("OUT_DIR"), "/compi
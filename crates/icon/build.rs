use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::{read_to_string, File};
use std::io::{LineWriter, Write};
use std::path::Path;

use itertools::Itertools;

fn build_raw_line<S: AsRef<OsStr> + ?Sized>(p: &S, const_name: &str) -> String {
    let json_file_path = Path::new(p);
    let json_file_str = read_to_string(json_file_path).expect("file not found");
    let icon_map: HashMap<String, char> =
        serde_json::from_str(&json_file_str).expect("error while reading json");

    let sorted_icon_tuples = icon_map
        .keys()
        .sorted()
        .map(|k| format!("(\"{}\", '{}')", k, icon_map[k]))
        .join(",");

    format!("pub const {const_name}: &[(&str, char)] = &[{sorted_icon_tuples}];",)
}

fn main() {
    let current_dir = std::env::current_dir().unwrap();

    let file_under_current_dir = |filename: &str| {
        let mut icon_path = current_dir.clone();
        icon_path.push(filename);
        icon_path
    };

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let file = File::create(dest_path).expect("can not create file");
    let mut file = LineWriter::new(file);

    let build_line = |filename: &str, const_name: &str| {
    
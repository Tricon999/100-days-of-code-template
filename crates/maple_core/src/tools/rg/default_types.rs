// https://github.com/BurntSushi/ripgrep/blob/20534fad04/crates/ignore/src/default_types.rs

#[rustfmt::skip]
pub const DEFAULT_TYPES: &[(&str, &[&str])] = &[
    ("agda", &["*.agda", "*.lagda"]),
    ("aidl", &["*.aidl"]),
    ("amake", &["*.mk", "*.bp"]),
    ("asciidoc", &["*.adoc", "*.asc", "*.asciidoc"]),
    ("asm", &["*.asm", "*.s", "*.S"]),
    ("asp", &[
        "*.aspx", "*.aspx.cs", "*.aspx.cs", "*.ascx", "*.ascx.cs", "*.ascx.vb",
    ]),
    ("ats", &["*.ats", "*.dats", "*.sats", "*.hats"]),
    ("avro", &["*.avdl", "*.avpr", "*.avsc"]),
    ("awk", &["*.awk"]),
    ("bazel", &["*.bzl", "WORKSPACE", "BUILD", "BUILD.bazel"]),
    ("bitbake", &["*.bb", "*.bbappend", "*.bbclass", "*.conf", "*.inc"]),
    ("brotli", &["*.br"]),
    ("buildstream", &["*.bst"]),
    ("bzip2", &["*.bz2", "*.tbz2"]),
    ("c", &["*.[chH]", "*.[chH].in", "*.cats"]),
    ("cabal", &["*.cabal"]),
    ("cbor", &["*.cbor"]),
    ("ceylon", &["*.ceylon"]),
    ("clojure", &["*.clj", "*.cljc", "*.cljs", "*.cljx"]),
    ("cmake", &["*.cmake", "CMakeLists.txt"]),
    ("coffeescript"
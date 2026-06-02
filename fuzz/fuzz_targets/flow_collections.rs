#![no_main]

mod common;

use common::parse_with_both_inputs;
use libfuzzer_sys::fuzz_target;

// Exercise flow-style sequences and mappings through granit-parser directly.
fuzz_target!(|data: &[u8]| {
    if data.len() > 16 * 1024 {
        return;
    }

    let s = String::from_utf8_lossy(data);

    let yaml_seq = format!("[{s}]");
    let yaml_map = format!("{{{s}}}");
    let yaml_doc = format!("root: {{{s}}}\narray: [{s}]\n");

    for yaml in [&yaml_seq, &yaml_map, &yaml_doc] {
        parse_with_both_inputs(yaml);
    }
});

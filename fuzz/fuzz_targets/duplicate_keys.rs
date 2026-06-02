#![no_main]

mod common;

use common::parse_with_both_inputs;
use libfuzzer_sys::fuzz_target;

// Construct mappings with intentional duplicate keys across block and flow styles.
fuzz_target!(|data: &[u8]| {
    if data.len() > 16 * 1024 {
        return;
    }

    let s = String::from_utf8_lossy(data);

    let yaml_top = format!("a: 1\na: 2\nkey: {s}\nkey: {s}\n");
    let yaml_nested = format!("outer:\n  inner: {{x: 1, x: 2}}\n  arr: [{{k: {s}}}, {{k: {s}}}]\n");
    let yaml_flow = format!("{{'{s}': 1, '{s}': 2}}\n");

    for yaml in [&yaml_top, &yaml_nested, &yaml_flow] {
        parse_with_both_inputs(yaml);
    }
});

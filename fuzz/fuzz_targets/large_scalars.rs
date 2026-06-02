#![no_main]

mod common;

use common::parse_with_both_inputs;
use libfuzzer_sys::fuzz_target;

// Stress plain and block scalar scanning. Generated YAML is capped to keep the
// target useful under CI and sanitizer memory limits.
fuzz_target!(|data: &[u8]| {
    if data.len() < 256 {
        return;
    }

    let cap: usize = 1 << 20;
    let fragment = String::from_utf8_lossy(data);
    let mut plain = String::new();

    while plain.len() < cap {
        if plain.len() + fragment.len() > cap {
            break;
        }
        plain.push_str(&fragment);
    }

    let yaml_plain = format!("{plain}\n");
    let yaml_block = format!("|\n  {plain}\n  {plain}\n");

    for yaml in [&yaml_plain, &yaml_block] {
        parse_with_both_inputs(yaml);
    }
});

#![no_main]

mod common;

use common::parse_with_both_inputs;
use libfuzzer_sys::fuzz_target;

// Bias inputs toward anchors, aliases, and YAML merge-key syntax while exercising
// granit-parser directly. Invalid YAML is acceptable; parser panics are findings.
fuzz_target!(|data: &[u8]| {
    if data.len() > 16 * 1024 {
        return;
    }

    let s = String::from_utf8_lossy(data);

    let yaml_alias = format!("a: &A {s}\nb: *A\nseq: &S [1, 2, 3]\nseq_alias: *S\n");
    let yaml_merge = format!(
        "base1: &B1 {{k: 1, v: {s}}}\nbase2: &B2 {{k: 2, w: {s}}}\nmerged: {{<<: [*B1, *B2], extra: 3}}\n"
    );

    for yaml in [&yaml_alias, &yaml_merge] {
        parse_with_both_inputs(yaml);
    }
});

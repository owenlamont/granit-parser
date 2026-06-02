use granit_parser::{Parser, ScanError};

pub fn parse_with_both_inputs(input: &str) {
    let _ = drain_str_parser(input);
    let _ = drain_iter_parser(input);
}

fn drain_str_parser(input: &str) -> Result<(), ScanError> {
    for event in Parser::new_from_str(input) {
        event?;
    }
    Ok(())
}

fn drain_iter_parser(input: &str) -> Result<(), ScanError> {
    for event in Parser::new_from_iter(input.chars()) {
        event?;
    }
    Ok(())
}

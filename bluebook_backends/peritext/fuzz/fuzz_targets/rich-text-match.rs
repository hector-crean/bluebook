#![no_main]
use libfuzzer_sys::fuzz_target;
use rich_text::rich_text::test_utils::{fuzzing_line_break, LineBreakFuzzAction};

fuzz_target!(|actions: Vec<LineBreakFuzzAction>| { fuzzing_line_break(actions) });

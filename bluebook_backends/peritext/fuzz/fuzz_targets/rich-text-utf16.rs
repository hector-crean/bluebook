#![no_main]
use libfuzzer_sys::fuzz_target;
use rich_text::rich_text::test_utils::{fuzzing_utf16, Action};

fuzz_target!(|actions: Vec<Action>| { fuzzing_utf16(5, actions) });

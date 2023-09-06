#![no_main]
use libfuzzer_sys::fuzz_target;
use rich_text::rich_text::test_utils::{fuzzing, Action};

fuzz_target!(|actions: Vec<Action>| { fuzzing(5, actions) });

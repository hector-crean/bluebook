#![no_main]
use libfuzzer_sys::fuzz_target;
use rich_text::legacy::test_utils::{fuzzing, Action};

fuzz_target!(|actions: Vec<Action>| { fuzzing(2, actions) });

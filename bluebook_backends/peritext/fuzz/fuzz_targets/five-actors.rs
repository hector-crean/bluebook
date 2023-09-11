#![no_main]
use libfuzzer_sys::fuzz_target;
use rich_text::legacy::test_utils::{fuzzing, Action};

fuzz_target!(|actions: [Action; 100]| { fuzzing(5, actions.to_vec()) });

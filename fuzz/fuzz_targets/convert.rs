#![no_main]
use libfuzzer_sys::fuzz_target;
use base100::{char_to_emoji, emoji_to_char};

fuzz_target!(|data: &[u8]| {
    let mut copy = data.to_vec();
    copy.fill(0);
    let _ = char_to_emoji(data, &mut copy);
    copy.fill(0);
    let _ = emoji_to_char(data, &mut copy);
});

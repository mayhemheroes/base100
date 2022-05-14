#![no_main]
use libfuzzer_sys::fuzz_target;
use base100::{char_to_emoji, emoji_to_char};

fuzz_target!(|data: &[u8]| {
    let mut copy = Vec::with_capacity(data.len()*4);
    copy.resize(data.len() * 4, 0);
    
    let _ = char_to_emoji(data, &mut copy);

    copy.fill(0);
    let mut emoji_copy = data.to_vec();
    while emoji_copy.len() % 4 != 0 {
        emoji_copy.push(0);
    }
    let _ = emoji_to_char(&emoji_copy, &mut copy);
});

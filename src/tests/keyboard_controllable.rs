use crate::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;

#[test]
// Try entering various texts that were selected to test edge cases.
// Because it is hard to test if they succeed,
// we assume it worked as long as there was no panic
fn test_key_sequence() {
    thread::sleep(super::get_delay());
    let mut enigo = Enigo::new();

    let sequences = vec![
        "",  // Empty string
        "a", // Simple character
        "z", /* Simple character
             TODO: This enters "y" on my computer */
        "9",     // Number
        "%",     // Special character
        "𝕊",     // Special char which needs two u16s to be encoded
        "❤️",     // Single emoji
        "abcde", // Simple short character string (shorter than 20 chars)
        "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz", /* Simple long character string (longer than 20 chars to test the restrictions of the macOS implementation) */
        "اَلْعَرَبِيَّةُ", // Short arabic string (meaning "Arabic")
        "中文",    // Short chinese string (meaning "Chinese")
        "日本語",  /* Short japanese string (meaning "Japanese") // TODO: On my computer "日" is
                    * not entered */
        "aaaaaaaaaaaaaaaaaaa𝕊𝕊", /* Long character string where a character starts at the 19th
                                  * byte and ends at the 20th byte */
        "aaaaaaaaaaaaaaaaaaa❤️❤️", /* Long character string where an emoji starts at the 19th byte
                                  * and ends at the 20th byte */
        "𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊𝕊", /* Long string where all 22 characters have a length of two in
                                   * the utf-16 encoding */
        "اَلْعَرَبِيَّةُاَلْعَرَبِيَّةُاَلْعَرَبِيَّةُاَلْعَرَبِيَّةُاَلْعَرَبِيَّةُاَلْعَرَبِيَّةُ", /* Long arabic string (longer than 20
                                                       * chars to test the restrictions of the
                                                       * macOS implementation)
                                                       * // TODO: This is missing the character on the very right
                                                       */
        "中文中文中文中文中文中文", // Long chinese string
        "日本語日本語日本語日本語日本語日本語日本語", // Long japanese string
        "H3llo World ❤️💯. What'𝕊 üp {}#𝄞\\日本語اَلْعَرَبِيَّةُ", /* Long string including characters
                                     * from various languages, emoji and
                                     * complex characters */
    ];

    for sequence in sequences {
        enigo.key_sequence(sequence);
    }
}

#[ignore] // TODO: Currently ignored because not all chars are valid CStrings
#[test]
// Try entering all UTF-16 chars with the key_sequence function.
// Because it is hard to test if they succeed,
// we assume it worked as long as there was no panic
fn test_key_sequence_all_utf16() {
    thread::sleep(Duration::from_secs(2));
    let mut enigo = Enigo::new();
    for c in 0x0000..0x0010_FFFF {
        if let Some(character) = char::from_u32(c) {
            println!("{character}");
            enigo.key_sequence(&character.to_string());
        };
    }
}

#[test]
// Test all the keys, make sure none of them panic
fn test_key_click() {
    use strum::IntoEnumIterator;

    let mut enigo = Enigo::new();
    for key in Key::iter() {
        //println!("{key:?}");
        enigo.key_down(key);
        enigo.key_up(key);
        enigo.key_click(key);
    }

    // TODO: Add tests for Key::Raw and Key::Layout
}

#[test]
// Try entering all chars with Key::Layout and make sure none of them panic
fn test_key_layout_all_utf16() {
    thread::sleep(Duration::from_secs(2));
    let mut enigo = Enigo::new();
    for c in 0x0000..0x0010_FFFF {
        if let Some(character) = char::from_u32(c) {
            println!("{character}");
            enigo.key_down(Key::Layout(character));
            enigo.key_up(Key::Layout(character));
            enigo.key_click(Key::Layout(character));
        };
    }
}

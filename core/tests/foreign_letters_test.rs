//! Per-letter choice for foreign initials: allow z (zui, zẻ) without turning "for" into "fỏ".

use gonhanh_core::engine::Engine;
use gonhanh_core::utils::type_word;

const Z: u8 = 1;
const W: u8 = 2;
const J: u8 = 4;
const F: u8 = 8;

fn run(allow: bool, letters: u8, input: &str) -> String {
    let mut e = Engine::new();
    e.set_english_auto_restore(true);
    e.set_allow_foreign_consonants(allow);
    e.set_foreign_letters(letters);
    type_word(&mut e, input)
}

#[test]
fn z_allowed_f_not() {
    // The user's case: "zui zẻ" works, "for" stays "for"
    assert_eq!(run(true, Z, "zui zer "), "zui zẻ ");
    assert_eq!(run(true, Z, "for "), "for ");
    assert_eq!(run(true, Z, "fa "), "fa ");
}

#[test]
fn all_letters_keeps_old_behaviour() {
    // Mask 15 must behave exactly like the plain on/off option did
    for input in ["zui zer ", "for ", "fa ", "jo ", "ja "] {
        let mut old = Engine::new();
        old.set_english_auto_restore(true);
        old.set_allow_foreign_consonants(true);
        assert_eq!(
            run(true, 0b1111, input),
            type_word(&mut old, input),
            "{input}"
        );
    }
}

#[test]
fn option_off_ignores_the_mask() {
    for letters in [0, Z, 0b1111] {
        assert_eq!(run(false, letters, "zer "), run(false, 0b1111, "zer "));
    }
}

#[test]
fn letters_are_independent() {
    // f allowed, z not: the reverse of the user's setup
    let z_off = run(true, F, "zer ");
    let z_on = run(true, Z, "zer ");
    assert_ne!(z_off, z_on, "z must depend on its own bit");
    assert_eq!(z_on, "zẻ ");
    // j and w bits only matter for words starting with j / w
    assert_eq!(run(true, Z, "zer "), run(true, Z | J | W, "zer "));
}

#[test]
fn normal_vietnamese_unaffected_by_any_mask() {
    for letters in [0, Z, F, 0b1111] {
        assert_eq!(run(true, letters, "daasu "), "dấu ");
        assert_eq!(run(true, letters, "tooi "), "tôi ");
    }
}

#[test]
fn english_words_that_only_look_vietnamese_stay_english() {
    // "just"->"jút", "fist"->"fít" with all foreign letters on: real English words whose Vietnamese
    // reading is not a word. Also inside an English sentence.
    for letters in [0b1111, J | F, F] {
        assert_eq!(run(true, letters, "just "), "just ", "letters={letters}");
        assert_eq!(run(true, letters, "fist "), "fist ", "letters={letters}");
        assert_eq!(
            run(true, letters, "user code just fist text "),
            "user code just fist text ",
            "letters={letters}"
        );
    }
    // ...while words the user really means as Vietnamese slang keep working
    assert_eq!(run(true, 0b1111, "zui zer "), "zui zẻ ");
    assert_eq!(run(true, 0b1111, "ddoiwjiiiiiii "), "đợiiiiiiii ");
}

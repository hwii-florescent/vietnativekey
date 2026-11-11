//! English mode (menu bar "E"): keys pass through, but a finished word that is real Vietnamese
//! typed in Telex is rewritten on Space. English stays the default for everything else.

use gonhanh_core::engine::Engine;
use gonhanh_core::utils::type_word;

fn en_engine() -> Engine {
    let mut e = Engine::new();
    e.set_enabled(false); // English mode
    e.set_detect_vn_in_en_mode(true);
    e
}

fn en(input: &str) -> String {
    type_word(&mut en_engine(), input)
}

#[test]
fn vietnamese_typed_in_english_mode_is_converted_on_space() {
    assert_eq!(en("truowfng "), "trường ");
    assert_eq!(en("nguwowif "), "người ");
    assert_eq!(en("ddi ddaau "), "đi đâu ");
    assert_eq!(en("tooi ddang hocj "), "tôi đang học ");
    assert_eq!(en("Vieetj Nam "), "Việt Nam ");
}

#[test]
fn english_stays_english() {
    for s in [
        "hello world ",
        "this is just a test ",
        "cos las ",
        "been how now ",
        "window system ",
    ] {
        assert_eq!(en(s), s, "{s}");
    }
}

#[test]
fn nothing_changes_until_space_and_never_with_symbols_or_digits() {
    assert_eq!(en("truowfng"), "truowfng"); // no space yet: still raw
    for s in [
        "truowfng, ",
        "truowfng1 ",
        "a/truowfng ",
        "user@truowfng ",
        "x.truowfng ",
        "truowfng.com ",
    ] {
        assert_eq!(en(s), s, "{s}");
    }
    assert_eq!(en("truowfng\n"), "truowfng\n");
}

#[test]
fn code_like_words_are_not_converted() {
    for s in [
        "vars args refs sys ",
        "sudo ls -la ",
        "git status ",
        "cd src ",
        "npm run dev ",
    ] {
        assert_eq!(en(s), s, "{s}");
    }
}

#[test]
fn off_by_default_and_vietnamese_mode_unchanged() {
    let mut off = Engine::new();
    off.set_enabled(false); // detection not switched on
    assert_eq!(type_word(&mut off, "truowfng "), "truowfng ");

    let mut vn = Engine::new();
    vn.set_english_auto_restore(true);
    vn.set_detect_vn_in_en_mode(true); // must have no effect in Vietnamese mode
    assert_eq!(type_word(&mut vn, "truowfng "), "trường ");
    assert_eq!(type_word(&mut vn, "hello "), "hello ");
}

#[test]
fn flip_converts_an_english_looking_word_and_teaches_it() {
    let mut e = en_engine();
    assert_eq!(type_word(&mut e, "cos "), "cos "); // English by default
    e.flip_last_ambiguous(); // user taps left Option: it was Vietnamese
    assert_eq!(type_word(&mut e, "cos "), "có "); // learned
    e.flip_last_ambiguous(); // and back
    assert_eq!(type_word(&mut e, "cos "), "cos ");
}

#[test]
fn flip_undoes_a_conversion_and_teaches_the_word_stays_raw() {
    let mut e = en_engine();
    assert_eq!(type_word(&mut e, "tooi "), "tôi ");
    e.flip_last_ambiguous();
    assert_eq!(type_word(&mut e, "tooi "), "tooi ");
}

#[test]
fn a_clearly_vietnamese_line_converts_english_looking_words() {
    // two Vietnamese words in a row make the line Vietnamese: "cos" is "có" there
    assert_eq!(en("tooi ddang hocj cos "), "tôi đang học có ");
    assert_eq!(en("cos "), "cos ");
    assert_eq!(en("tooi cos "), "tôi cos "); // one vote is not enough
}

#[test]
fn enter_starts_a_new_line() {
    // Enter itself never converts (only Space does), and it resets the line context
    assert_eq!(en("tooi ddang\ncos "), "tôi ddang\ncos ");
    assert_eq!(en("tooi ddang \ncos "), "tôi đang \ncos ");
}

#[test]
fn backspace_right_after_a_conversion_reverts_to_the_typed_english() {
    // '<' is Backspace in type_word. The Backspace also eats the space, like Backspace after a word does elsewhere.
    assert_eq!(en("truowfng <"), "truowfng");
    assert_eq!(en("ddi ddaau <"), "đi ddaau"); // only the last word reverts
    assert_eq!(en("ddi ddaau <<"), "đi ddaa"); // then it is ordinary backspacing
                                               // The word stays English: typing on continues it, and it is not re-converted on the next Space.
    assert_eq!(en("truowfng <x "), "truowfngx ");
    assert_eq!(en("truowfng < "), "truowfng ");
    // Any key in between means the conversion is final.
    assert_eq!(en("truowfng a<"), "trường ");
    // A word that was never converted: Backspace only deletes the space.
    assert_eq!(en("hello <"), "hello");
}

/// Type `input`, press the flip key, and return the screen. `engine` picks the mode and settings.
fn typed_then_flipped(mut e: Engine, input: &str) -> String {
    let mut screen = type_word(&mut e, input);
    let r = e.flip_last_ambiguous();
    if r.action == gonhanh_core::engine::Action::Send as u8 {
        for _ in 0..r.backspace {
            screen.pop();
        }
        screen.extend((0..r.count as usize).filter_map(|i| char::from_u32(r.chars[i])));
    }
    screen
}

#[test]
fn flip_works_on_the_word_left_of_the_cursor_in_english_mode() {
    // Being typed, or one space after, whether or not the word was converted on its own
    assert_eq!(typed_then_flipped(en_engine(), "truowfng"), "trường");
    assert_eq!(typed_then_flipped(en_engine(), "truowfng "), "truowfng "); // was converted: flips back
    assert_eq!(typed_then_flipped(en_engine(), "hello cos "), "hello có "); // English kept: flips to Vietnamese
    assert_eq!(typed_then_flipped(en_engine(), "vars "), "vá "); // code-like: never auto-converted, still flippable
                                                                 // Detection off: nothing is converted by itself, the flip key still can
    let mut off = Engine::new();
    off.set_enabled(false);
    assert_eq!(typed_then_flipped(off, "tooi "), "tôi ");
}

#[test]
fn flip_ignores_what_it_cannot_convert_or_cannot_reach() {
    for s in [
        "hello",
        "hello ",
        "x",
        "truowfng  ",
        "truowfng a",
        "truowfng,",
        "truowfng1",
        "a/truowfng",
    ] {
        let mut e = en_engine();
        type_word(&mut e, s);
        assert_eq!(e.flip_last_ambiguous().action, 0, "{s}");
    }
}

#[test]
fn flip_toggles_and_a_word_can_flip_again_after_backspace_reverts_it() {
    let mut e = en_engine();
    assert_eq!(type_word(&mut e, "truowfng"), "truowfng");
    let mut screen = String::from("truowfng");
    for want in ["trường", "truowfng", "trường"] {
        let r = e.flip_last_ambiguous();
        for _ in 0..r.backspace {
            screen.pop();
        }
        screen.extend((0..r.count as usize).filter_map(|i| char::from_u32(r.chars[i])));
        assert_eq!(screen, want);
    }
    // converted on Space, Backspace reverts it, flip converts it again (no space left to touch)
    assert_eq!(typed_then_flipped(en_engine(), "truowfng <"), "trường");
}

#[test]
fn flip_works_in_vietnamese_mode_for_any_word() {
    let vn = || {
        let mut e = Engine::new();
        e.set_english_auto_restore(true);
        e
    };
    assert_eq!(typed_then_flipped(vn(), "Vieetj"), "Vieetj"); // being typed: back to the keys
    assert_eq!(typed_then_flipped(vn(), "Vieetj "), "Vieetj "); // finished, one space: same
    assert_eq!(typed_then_flipped(vn(), "hello "), "hello "); // nothing to flip
    assert_eq!(typed_then_flipped(vn(), "Vieetj  "), "Việt  "); // two spaces: out of reach
}

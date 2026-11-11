//! Words taught with the flip key, and the English-context rules for Vietnamese-looking English.

use gonhanh_core::engine::Engine;
use gonhanh_core::utils::type_word;

const EN: &str = "hello world about "; // three plain English words: English context
const VN: &str = "tooi ddang hocj "; // three Vietnamese words: Vietnamese context

fn engine() -> Engine {
    let mut e = Engine::new();
    e.set_english_auto_restore(true);
    e
}

fn run(prefix: &str, word: &str) -> String {
    let mut e = engine();
    let all = type_word(&mut e, &format!("{prefix}{word} "));
    all[type_word(&mut engine(), prefix).len()..]
        .trim_end()
        .to_string()
}

fn temp_file(name: &str) -> String {
    let p = std::env::temp_dir().join(format!("vnk_learn_{name}_{}", std::process::id()));
    let _ = std::fs::remove_file(&p);
    p.to_string_lossy().into_owned()
}

#[test]
fn flip_teaches_the_word_and_it_survives_a_restart() {
    let file = temp_file("restart");
    let mut e = engine();
    e.set_learn_file(&file);
    assert_eq!(type_word(&mut e, "cuts "), "cút "); // no context: Vietnamese
    e.flip_last_ambiguous(); // the user taps left Option: it should have been English
    assert_eq!(type_word(&mut e, "cuts "), "cuts "); // learned in the same session

    let mut fresh = engine(); // "restart": a new engine reading the same file
    fresh.set_learn_file(&file);
    assert_eq!(type_word(&mut fresh, "cuts "), "cuts ");
    assert_eq!(fresh.learned_count(), 1);

    fresh.flip_last_ambiguous(); // flip back: latest choice wins
    assert_eq!(type_word(&mut fresh, "cuts "), "cút ");
    let mut again = engine();
    again.set_learn_file(&file);
    assert_eq!(type_word(&mut again, "cuts "), "cút ");

    again.clear_learned();
    assert_eq!(again.learned_count(), 0);
    let mut empty = engine();
    empty.set_learn_file(&file);
    assert_eq!(empty.learned_count(), 0);
    let _ = std::fs::remove_file(&file);
}

#[test]
fn learned_word_beats_context() {
    let mut e = engine();
    type_word(&mut e, "nuts ");
    e.flip_last_ambiguous(); // teach: nuts = English
                             // Even in a clearly Vietnamese line it stays English now
    assert_eq!(
        type_word(&mut e, "tooi ddang hocj nuts "),
        "tooi ddang hocj nuts ".replace("tooi ddang hocj", "tôi đang học")
    );
}

#[test]
fn only_english_dictionary_words_are_stored() {
    let file = temp_file("filter");
    let mut e = engine();
    e.set_learn_file(&file);
    type_word(&mut e, "tooi "); // Vietnamese, not an English word: nothing to flip or teach
    e.flip_last_ambiguous();
    assert_eq!(e.learned_count(), 0);
    assert!(std::fs::read_to_string(&file)
        .unwrap_or_default()
        .is_empty());
    let _ = std::fs::remove_file(&file);
}

#[test]
fn non_word_vietnamese_reading_follows_the_line() {
    assert_eq!(run(EN, "trays"), "trays"); // English line: not "tráy"
    assert_eq!(run(VN, "trays"), "tráy"); // Vietnamese line: unchanged
    assert_eq!(run("", "trays"), "tráy"); // no context, desktop default: unchanged
    let mut e = engine();
    e.set_ambiguous_default_english(true); // terminal default
    assert_eq!(type_word(&mut e, "trays "), "trays ");
}

#[test]
fn vietnamese_word_sharing_english_keys_needs_a_fully_english_line() {
    assert_eq!(run(EN, "been"), "been"); // English line: not "bên"
    assert_eq!(run(VN, "been"), "bên");
    assert_eq!(run("", "been"), "bên"); // no context: unchanged
    assert_eq!(run("hello world ", "been"), "been"); // two English votes already make an English line
    assert_eq!(run("hello ", "been"), "bên"); // one vote is not enough
}

#[test]
fn clash_words_can_be_flipped_by_hand() {
    // "how" -> "hơ" has a horn, so it used to be unflippable
    let mut e = engine();
    assert_eq!(type_word(&mut e, "how "), "hơ ");
    let r = e.flip_last_ambiguous();
    assert!(
        r.action != 0,
        "flip must work on horn/circumflex clashes too"
    );
}

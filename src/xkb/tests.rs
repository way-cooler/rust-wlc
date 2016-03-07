#![cfg(test)] // Module only builds if tests are enabled

use super::*;

#[test]
fn sanity_test() {
    assert!(true);
    assert_eq!(1, 1);
}

#[test]
fn from_name_get_name() {
    // Test a small (but most likely to be used) subset of keysyms
    let names = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
        "BackSpace", "Tab", "Return", "Escape", "Delete",
        "Left", "Right", "Up", "Down", "Home", "End",
        "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9",
        "Shift_L", "Shift_R", "Super_L", "Meta_R", "Caps_Lock",
        "space", "exclam", "dollar", "ampersand", "apostrophe"];
    for name in names {
        let msym = Keysym::from_name(name.to_string(), NameFlags::None);
        assert!(msym.is_some());
        let sym = msym.unwrap();
        assert!(sym.is_valid());
        let mname = sym.get_name();
        assert!(mname.is_some());
        let sym_name = mname.unwrap();
        assert_eq!(sym_name, name.to_string());
    }
}

#[test]
fn some_unexpected_keysyms() {
    let non_names = vec![
        "soda", "caps lock", "Hypermeta_L", "key",
        "keyboard", "aa", "kk"];
    for name in non_names {
        let sym = Keysym::from_name(name.to_string(), NameFlags::None);
        assert!(sym.is_none());
    }
}

#[test]
// keysym.is_valid() implies keysym.get_name().is_some()
fn valitity_implications() {
    for val in 0u32..999999u32 {
        let sym = Keysym::from(val);
        if sym.is_valid() {
            let name = sym.get_name();
            assert!(name.is_some());
            let sym2 = Keysym::from_name(name.unwrap(), NameFlags::None);
            assert!(sym2.is_some());
            assert_eq!(sym, sym2.unwrap());
        }
    }
}

#[test]
fn case_insensitive_from_name() {
    let names = vec![
        "caps_lock", "super_l", "escape",
        "ESCAPE", "RETURN", "SELECT"];
    for name in names {
        let sym = Keysym::from_name(name.to_string(), NameFlags::CaseInsensitive);
        let none_sym = Keysym::from_name(name.to_string(), NameFlags::None);
        assert!(sym.is_some());
        assert!(none_sym.is_none());
    }
}

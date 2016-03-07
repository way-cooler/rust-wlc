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
        "space", "exclam", "dollar", "ampersand", "apostrophe"
        ];
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

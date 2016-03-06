#![cfg(test)] // Module only builds if tests are enabled

use super::*;

#[test]
fn sanity_test() {
    assert!(true);
    assert_eq!(1, 1);
}

fn name_and_get_name(name: &str) {
    let name_string = name.to_string();
    let sym = Keysym::from_name(name, NameFlags::None);
    assert!(sym.is_some());
    assert_eq!(sym.unwrap().get_name(), Some(name_string));
}

#[test]
fn from_name_to_get_name() {
    // TODO Implement this with a macro
    name_and_get_name("a");
    //name_and_get_name("b");
    //name_and_get_name("c");
    //name_and_get_name("d");
    // name_and_get_name("e");
    // name_and_get_name("f");
    // name_and_get_name("g");
    // name_and_get_name("h");
    // name_and_get_name("i");
    // name_and_get_name("j");
    // name_and_get_name("k");
    // name_and_get_name("l");
    // name_and_get_name("m");
    // name_and_get_name("n");
    // name_and_get_name("o");
    // name_and_get_name("p");
    // name_and_get_name("q");
    // name_and_get_name("r");
}

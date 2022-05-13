#[allow(unused)]
use crate::language::{get_default_ui_language,get_system_default_ui_language,get_language_by_number};

#[test]
fn get_default_ui_language_test() {
    assert_eq!(get_default_ui_language(),"zh-CN");
}

#[test]
fn get_system_default_ui_language_test() {
    assert_eq!(get_system_default_ui_language(),"zh-CN");
}
#[test]
fn get_language_by_number_test() {
    assert_eq!(get_language_by_number(2052),"zh-CN");
}
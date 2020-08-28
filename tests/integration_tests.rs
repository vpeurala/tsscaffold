extern crate tsscaffold;

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fs;
use std::io;
use std::io::Write;
use std::rc::Rc;

#[test]
pub fn test_insert() {
    let input: fs::File = fs::File::open("testdata/table.yml").unwrap();
    let tables = tsscaffold::parse_yaml(input).unwrap();
    let expected = fs::read_to_string("testdata/insert_1.sql").unwrap();

    let mut writer: Vec<u8> = Vec::new();
    tsscaffold::insert(tables, &mut writer).unwrap();
    let actual = String::from_utf8(writer.to_vec()).unwrap();
    assert_eq!(expected, actual);
}
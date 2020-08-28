extern crate tsscaffold;

use pretty_assertions::assert_eq;
use std::fs;

#[test]
pub fn insert_smoke() {
    let input: fs::File = fs::File::open("testdata/table.yml").unwrap();
    let tables = tsscaffold::parse_yaml(input).unwrap();
    let expected = fs::read_to_string("testdata/insert_2.sql").unwrap();

    let mut writer: Vec<u8> = Vec::new();
    tsscaffold::insert(tables, &mut writer).unwrap();
    let actual = String::from_utf8(writer).unwrap();
    assert_eq!(expected, actual);
}

#[test]
pub fn create_table_smoke() {
    let input: fs::File = fs::File::open("testdata/table.yml").unwrap();
    let tables = tsscaffold::parse_yaml(input).unwrap();
    let expected = fs::read_to_string("testdata/create_table_1.sql").unwrap();

    let mut writer: Vec<u8> = Vec::new();
    tsscaffold::create_table(tables, &mut writer).unwrap();
    let actual = String::from_utf8(writer).unwrap();
    assert_eq!(expected, actual);
}

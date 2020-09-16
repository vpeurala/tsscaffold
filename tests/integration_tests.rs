extern crate tsscaffold;

use pretty_assertions::assert_eq;
use std::fs;

use tsscaffold::commands::create_table;
use tsscaffold::commands::insert_sql;
use tsscaffold::commands::insert_interface;
use tsscaffold::parse_yaml;

#[test]
pub fn insert_smoke() {
    let input: fs::File = fs::File::open("testdata/table_1.yml").unwrap();
    let tables = parse_yaml(input).unwrap();
    let expected = fs::read_to_string("testdata/insert_2.sql").unwrap();

    let mut writer: Vec<u8> = Vec::new();
    insert_sql(tables, &mut writer).unwrap();
    let actual = String::from_utf8(writer).unwrap();
    assert_eq!(expected, actual);
}

#[test]
pub fn create_table_smoke() {
    let input: fs::File = fs::File::open("testdata/table_1.yml").unwrap();
    let tables = parse_yaml(input).unwrap();
    let expected = fs::read_to_string("testdata/create_table_1.sql").unwrap();

    let mut writer: Vec<u8> = Vec::new();
    create_table(tables, &mut writer).unwrap();
    let actual = String::from_utf8(writer).unwrap();
    assert_eq!(expected, actual);
}

#[test]
pub fn create_table_supports_nullable_keyword() {
    let input: fs::File = fs::File::open("testdata/table_2.yml").unwrap();
    let tables = parse_yaml(input).unwrap();
    let expected = fs::read_to_string("testdata/create_table_2.sql").unwrap();

    let mut writer: Vec<u8> = Vec::new();
    create_table(tables, &mut writer).unwrap();
    let actual = String::from_utf8(writer).unwrap();
    assert_eq!(expected, actual);
}

#[test]
pub fn insert_interface_smoke() {
    let input: fs::File = fs::File::open("testdata/table_2.yml").unwrap();
    let tables = parse_yaml(input).unwrap();
    let expected = fs::read_to_string("testdata/insert_interface_2.ts").unwrap();

    let mut writer: Vec<u8> = Vec::new();
    insert_interface(tables, &mut writer).unwrap();
    let actual = String::from_utf8(writer).unwrap();
    assert_eq!(expected, actual);
}
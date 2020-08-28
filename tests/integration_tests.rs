extern crate tsscaffold;

use std::fs;
use std::io::Write;
use pretty_assertions::assert_eq;

#[test]
pub fn test_insert() {
    let input: fs::File = fs::File::open("testdata/table.yml").unwrap();
    let tables = tsscaffold::parse_yaml(input).unwrap();
    let expected = fs::read_to_string("testdata/insert_2.sql").unwrap();

    let mut writer: Vec<u8> = Vec::new();
    tsscaffold::insert(tables, &mut writer).unwrap();
    let actual = String::from_utf8(writer).unwrap();
    assert_eq!(expected, actual);
}
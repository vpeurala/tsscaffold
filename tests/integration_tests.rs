extern crate tsscaffold;

use pretty_assertions::assert_eq;
use std::fs;

use tsscaffold::commands::create_table;
use tsscaffold::commands::insert_sql;
use tsscaffold::commands::insert_interface;
use tsscaffold::parse_yaml;
use assert_cmd::Command;

#[test]
pub fn main_smoke() {
    let mut cmd = Command::cargo_bin("tsscaffold").unwrap();
    let assert = cmd
        .arg("testdata/table_1.yml")
        .arg("create-table")
        .assert();
    assert
        .success()
        .stdout("CREATE TABLE share_classes (
    portfolio_id INTEGER NOT NULL,
    share_class VARCHAR NOT NULL,
    isin VARCHAR NOT NULL,
    currency VARCHAR NOT NULL,
    as_of_date DATE NOT NULL,
    sales_status VARCHAR NOT NULL,
    hex_code VARCHAR NOT NULL,
    complexity VARCHAR NOT NULL,
    subscription_fee VARCHAR NOT NULL,
    redemption_fee VARCHAR NOT NULL,
    management_fee VARCHAR NOT NULL,
    running_costs VARCHAR NOT NULL,
    ter_number VARCHAR NOT NULL,
    currency_exchange_costs VARCHAR NOT NULL,
    performance_renumeration VARCHAR NOT NULL,
    fund_transaction_costs VARCHAR NOT NULL,
    instrument_management_costs VARCHAR NOT NULL,
    rules_fi VARCHAR NOT NULL,
    rules_sv VARCHAR NOT NULL,
    brochure_fi VARCHAR NOT NULL,
    brochure_sv VARCHAR NOT NULL
);

ALTER TABLE share_classes ADD CONSTRAINT share_classes_pk PRIMARY KEY (portfolio_id, share_class);
");
}

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
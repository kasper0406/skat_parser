use serde::Deserialize;

use web_sys::console;
use yew::Html;
use yew::prelude::*;

use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub enum FieldFormatter {
    Raw,
    Integer,
    Date,
    CprNumber,
    Time,
    Enum(HashMap<String, String>),
}

impl FieldFormatter {
    pub fn format(&self, raw_value: String) -> Result<Html, ()> {
        match self {
            FieldFormatter::Integer => {
                let parsed = raw_value.parse::<i64>();
                parsed.map(|value| html! { format!("{}", value) }).map_err(|_| ())
            },
            FieldFormatter::Date => {
                if raw_value.len() != 8 {
                    return Err(());
                }
                Ok(html! { format!("{}-{}-{}", &raw_value[6..8], &raw_value[4..6], &raw_value[0..4]) })
            },
            FieldFormatter::Time => {
                if raw_value.len() != 6 {
                    return Err(());
                }
                Ok(html! { format!("{}:{}:{}", &raw_value[0..2], &raw_value[2..4], &raw_value[4..6]) })
            },
            FieldFormatter::CprNumber => {
                if raw_value.len() != 10 {
                    return Err(());
                }
                Ok(html! { format!("{}-{}", &raw_value[0..6], &raw_value[6..10]) })
            },
            FieldFormatter::Enum(entries) => {
                Ok(html! { entries.get(&raw_value).unwrap_or(&format!("Unknown: {}", raw_value)) })
            }
            _ => Ok(html! { raw_value }),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RecordField {
    field_number: usize,
    start: usize,
    length: usize,
    name: String,
    formatter: FieldFormatter,
}

impl RecordField {
    pub fn key(start: usize, length: usize) -> Self {
        RecordField {
            start, length,
            field_number: 999,
            name: "".to_string(),
            formatter: FieldFormatter::Raw,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn formatter(&self) -> &FieldFormatter {
        &self.formatter
    }
}

impl RecordField {
    pub fn parse(&self, line: &str) -> ParsedField {
        let raw_value = line[(self.start - 1)..(self.start + self.length - 1)].to_string();
        ParsedField::of(raw_value, self)
    }
}

#[derive(Clone, Deserialize)]
pub struct RecordType {
    key: String,
    fields: Vec<RecordField>,
}

#[derive(Clone, Deserialize)]
pub struct RecordSpec {
    key_field: RecordField,
    records: Vec<RecordType>,
}

impl RecordSpec {
    pub fn get_record_type(&self, line: &str) -> Option<&RecordType> {
        let key = self.key_field.parse(line).raw_value;
        self.records.iter().find(|record| record.key == key)
    }
}

#[derive(Debug)]
pub struct ParsedRecord {
//    title: String,
    fields: Vec<ParsedField>,
}

impl ParsedRecord {
    pub fn of(fields: Vec<ParsedField>) -> Self {
        ParsedRecord { fields }
    }

    pub fn unknown(line: String) -> Self {
        ParsedRecord { fields: vec![
            ParsedField::unknown(&line),
        ] }
    }

    pub fn fields(&self) -> &[ParsedField] {
        &self.fields
    }
}

#[derive(Debug)]
pub struct ParsedField {
    raw_value: String,
    field_spec: Option<RecordField>,
}

impl ParsedField {
    pub fn of(raw_value: String, field_spec: &RecordField) -> Self {
        ParsedField {
            raw_value: raw_value.to_string(),
            field_spec: Some(field_spec.clone()),
        }
    }

    pub fn unknown(raw_value: &str) -> Self {
        ParsedField {
            raw_value: raw_value.to_string(),
            field_spec: None,
        }
    }

    pub fn raw(&self) -> &String {
        &self.raw_value
    }

    pub fn spec(&self) -> &Option<RecordField> {
        &self.field_spec
    }
}

pub fn parse_records(content: &str, spec: RecordSpec) -> Vec<ParsedRecord> {
    let mut records = vec![];

    for line in content.lines() {
        records.push(if let Some(record_type) = spec.get_record_type(line) {
            let mut fields = vec![];

            let mut idx = 1;

            for field in &record_type.fields {
                // console::log_1(&format!("Idx: {}, Start: {}, field: {}, record: {}", idx, field.start, field.field_number, record_type.key).into());
                assert![ field.start >= idx, "Start must be smaller than index" ];
                if field.start != idx {
                    fields.push(ParsedField::unknown(&line[(idx - 1)..field.start]));
                }
                fields.push(field.parse(line));
                idx = field.start + field.length;
            }
            if line.len() != idx {
                fields.push(ParsedField::unknown(&line[(idx - 1)..]));
            }

            ParsedRecord::of(fields)
        } else {
            ParsedRecord::unknown(line.to_string())
        });
    }

    records
}

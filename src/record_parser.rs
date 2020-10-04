use serde::Deserialize;

use web_sys::console;
use yew::Html;
use yew::prelude::*;

use std::collections::HashMap;
use std::rc::Rc;

fn empty_string(len: usize) -> String {
    let mut s = String::with_capacity(len);
    for i in 0 .. len {
        s.push(' ');
    }
    s
}

fn safe_slice(string: &str, start: usize, end: usize) -> String {
    if end <= string.len() {
        string[start .. end].to_string()
    } else if start >= string.len() {
        empty_string(end - start)
    } else {
        string[start .. string.len()].to_owned() + &empty_string(end - string.len())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum FieldFormatter {
    Raw,
    Integer,
    Date,
    CprNumber,
    Time,
    Enum(HashMap<String, String>),
    MoneyAmount,
}

fn trim_right_zeros(value: &str) -> String {
    let trimmed = value.trim();
    let mut len = trimmed.len();
    while len > 0 && trimmed.chars().nth(len - 1).unwrap() == '0' {
        len -= 1;
    }
    trimmed[0..len].to_string()
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
            FieldFormatter::MoneyAmount => {
                let sigs = raw_value[0..10].parse::<i64>().unwrap_or(-1);
                let decs = trim_right_zeros(&raw_value[10..16]).parse::<i64>().unwrap_or(-1);
                let sign = match raw_value.chars().nth(16) {
                    Some('-') => '-',
                    _ => ' '
                };

                Ok(html! { format!("{}{}.{},-", sign, sigs, decs) })
            },
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

    pub fn field_number(&self) -> usize {
        self.field_number
    }
}

impl RecordField {
    pub fn parse(&self, line: &str) -> ParsedField {
        let raw_value = safe_slice(line, self.start - 1, self.start + self.length - 1).to_string();
        ParsedField::of(raw_value, self)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct RecordType {
    name: String,
    key: String,
    fields: Vec<RecordField>,
}

impl RecordType {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct RecordSpec {
    key_field: RecordField,
    records: Vec<RecordType>,
    hierarchy: Vec<HierarchySpec>,
}

impl RecordSpec {
    pub fn get_record_type(&self, line: &str) -> Option<&RecordType> {
        let key = self.key_field.parse(line).raw_value;
        self.records.iter().find(|record| record.key == key)
    }
}

#[derive(Clone, Debug)]
pub struct ParsedRecord {
//    title: String,
    key: String,
    fields: Vec<ParsedField>,
    spec: Option<Rc<RecordType>>,
}

impl ParsedRecord {
    pub fn of(spec: Rc<RecordType>, key: String, fields: Vec<ParsedField>) -> Self {
        ParsedRecord {
            key,
            fields,
            spec: Some(spec),
        }
    }

    pub fn unknown(line: String) -> Self {
        ParsedRecord {
            key: "UNKNOWN".to_string(),
            fields: vec![
                ParsedField::unknown(&line),
            ],
            spec: None,
        }
    }

    pub fn fields(&self) -> &[ParsedField] {
        &self.fields
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn spec(&self) -> Option<Rc<RecordType>> {
        self.spec.as_ref().map(|spec| spec.clone())
    }

    pub fn field(&self, fieldnr: usize) -> Option<String> {
        self.fields.iter()
            .filter(|field| field.field_spec.is_some())
            .find(|field| field.field_spec.as_ref().unwrap().field_number == fieldnr)
            .map(|field| field.raw_value.clone())
    }
}

#[derive(Clone, Debug)]
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

pub fn parse_records(content: &str, spec: Rc<RecordSpec>) -> Vec<ParsedRecord> {
    let mut records = vec![];

    for line in content.lines() {
        records.push(if let Some(record_type) = spec.get_record_type(line) {
            let mut fields = vec![];

            let mut idx = 1;

            for field in &record_type.fields {
                // console::log_1(&format!("Idx: {}, Start: {}, field: {}, record: {}", idx, field.start, field.field_number, record_type.key).into());
                assert![ field.start >= idx, "Start must be smaller than index" ];
                if field.start != idx {
                    fields.push(ParsedField::unknown(&safe_slice(line, idx - 1, field.start)));
                }
                fields.push(field.parse(line));
                idx = field.start + field.length;
            }
            if line.len() > idx {
                fields.push(ParsedField::unknown(&line[(idx - 1)..]));
            }

            ParsedRecord::of(Rc::new(record_type.clone()), spec.key_field.parse(line).raw_value, fields)
        } else {
            ParsedRecord::unknown(line.to_string())
        });
    }

    records
}

pub struct RecordHierarchy {
    record: ParsedRecord,
    children: Vec<Rc<RecordHierarchy>>,
}

impl RecordHierarchy {
    pub fn of(record: ParsedRecord) -> RecordHierarchy {
        RecordHierarchy {
            record,
            children: vec![],
        }
    }

    pub fn record(&self) -> &ParsedRecord {
        &self.record
    }

    pub fn children(&self) -> Vec<Rc<RecordHierarchy>> {
        self.children.clone()
    }

    pub fn contains_error(&self, maybe_errors: &Option<Rc<HashMap<usize, Error>>>) -> bool {
        match maybe_errors {
            Some(errors) => {
                if let Some(raw_linenr) = self.record.field(1) {
                    let linenr = raw_linenr.parse::<usize>().unwrap();

                    let mut child_contains_errors = false;

                    for child in &self.children {
                        child_contains_errors |= child.contains_error(&maybe_errors);
                    }

                    errors.contains_key(&linenr) || child_contains_errors
                } else {
                    false
                }
            },
            None => false
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct HierarchySpec {
    key: String,
    children: Option<Vec<HierarchySpec>>,
}

pub fn build_hierarchy(records: &[ParsedRecord], record_spec: &RecordSpec) -> Vec<Rc<RecordHierarchy>> {
    let mut result = vec![];

    let empty_list = vec![];

    // TODO(knielsen): Consider unifying these two stacks
    let mut hierarchy_stack: Vec<Rc<RecordHierarchy>> = vec![ ];
    let mut spec_stack: Vec<&Vec<HierarchySpec>> = vec![ &record_spec.hierarchy ];

    for record in records {
        assert! [ hierarchy_stack.len() + 1 == spec_stack.len(), "Stack should be the same size!" ];
        while !spec_stack.is_empty() {
            if let Some(matching_spec) = spec_stack.last().unwrap()
                    .iter()
                    .find(|spec| spec.key == record.key())
            {
                spec_stack.push(matching_spec.children.as_ref().unwrap_or(&empty_list));
                hierarchy_stack.push(Rc::new(RecordHierarchy::of(record.clone())));
                break;
            } else {
                if spec_stack.len() > 1 {
                    spec_stack.pop();
                    let element = hierarchy_stack.pop().unwrap();
                    if let Some(current_node) = hierarchy_stack.last_mut() {
                        Rc::get_mut(current_node).unwrap().children.push(element);
                    } else {
                        result.push(element);
                    }
                } else {
                    break;
                }
            }
        }

        if hierarchy_stack.is_empty() {
            result.push(Rc::new(RecordHierarchy::of(record.clone())));
        }
    }

    // Make sure to transfer ownership to the resulting structure in case things are still being built
    while let Some(element) = hierarchy_stack.pop() {
        if let Some(current_node) = hierarchy_stack.last_mut() {
            Rc::get_mut(current_node).unwrap().children.push(element);
        } else {
            result.push(element);
        }
    }

    result
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub severity: ErrorType,
    pub line: usize,
    pub field: usize,
    pub error_id: String,
    pub error_text: String,
}

pub fn extract_errors(records: &[ParsedRecord]) -> HashMap<usize, Error> {
    let mut result = HashMap::new();

    let extract = |severity, record: &ParsedRecord| {
        Error {
            severity,
            line: record.field(2).unwrap().parse::<usize>().unwrap(),
            field: record.field(3).unwrap().parse::<usize>().unwrap(),
            error_id: record.field(6).unwrap(),
            error_text: record.field(7).unwrap(),
        }
    };

    for record in records {
        match record.key.as_str() {
            "0002" => {
                let error = extract(ErrorType::Error, record);
                result.insert(error.line, error);
            },
            "0003" => {
                let error = extract(ErrorType::Warning, record);
                result.insert(error.line, error);
            },
            _ => {},
        }
    }

    result
}

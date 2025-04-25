use std::fmt;

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
    Array(Vec<JsonValue<'a>>),
    Object(Vec<(&'a str, JsonValue<'a>)>),
}

impl<'a> JsonValue<'a> {
    pub fn get(&'a self, key: &str) -> Option<&'a JsonValue<'a>> {
        if let JsonValue::Object(obj) = self {
            obj.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
        } else {
            None
        }
    }

    pub fn index(&'a self, i: usize) -> Option<&'a JsonValue<'a>> {
        if let JsonValue::Array(arr) = self {
            arr.get(i)
        } else {
            None
        }
    }

    pub fn query(&'a self, path: &str) -> Option<&'a JsonValue<'a>> {
        let mut current = self;
        for part in path.split('.') {
            if let Ok(index) = part.parse::<usize>() {
                current = current.index(index)?;
            } else {
                current = current.get(part)?;
            }
        }
        Some(current)
    }
}

impl<'a> fmt::Display for JsonValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Null => write!(f, "null"),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

pub fn parse_json(input: &str) -> Result<JsonValue, &'static str> {
    parse_value(input.trim()).map(|(v, _)| v)
}

fn parse_value(input: &str) -> Result<(JsonValue, &str), &'static str> {
    let input = input.trim_start();
    if input.starts_with('"') {
        parse_string(input).map(|(s, r)| (JsonValue::String(s), r))
    } else if input.starts_with('{') {
        parse_object(input)
    } else if input.starts_with('[') {
        parse_array(input)
    } else if input.starts_with("true") {
        Ok((JsonValue::Boolean(true), &input[4..]))
    } else if input.starts_with("false") {
        Ok((JsonValue::Boolean(false), &input[5..]))
    } else if input.starts_with("null") {
        Ok((JsonValue::Null, &input[4..]))
    } else {
        parse_number(input).map(|(n, r)| (JsonValue::Number(n), r))
    }
}

fn parse_object(input: &str) -> Result<(JsonValue, &str), &'static str> {
    let mut members = Vec::new();
    let mut cursor = &input[1..];

    loop {
        cursor = cursor.trim_start();
        if cursor.starts_with('}') {
            return Ok((JsonValue::Object(members), &cursor[1..]));
        }

        let (key, rest) = parse_string(cursor)?;
        let rest = rest.trim_start();
        if !rest.starts_with(':') {
            return Err("Expected ':'");
        }

        let rest = &rest[1..];
        let (value, rest) = parse_value(rest)?;
        members.push((key, value));

        let rest = rest.trim_start();
        if rest.starts_with(',') {
            cursor = &rest[1..];
        } else if rest.starts_with('}') {
            return Ok((JsonValue::Object(members), &rest[1..]));
        } else {
            return Err("Expected ',' or '}'");
        }
    }
}

fn parse_array(input: &str) -> Result<(JsonValue, &str), &'static str> {
    let mut elements = Vec::new();
    let mut cursor = &input[1..];

    loop {
        cursor = cursor.trim_start();
        if cursor.starts_with(']') {
            return Ok((JsonValue::Array(elements), &cursor[1..]));
        }

        let (value, rest) = parse_value(cursor)?;
        elements.push(value);
        let rest = rest.trim_start();

        if rest.starts_with(',') {
            cursor = &rest[1..];
        } else if rest.starts_with(']') {
            return Ok((JsonValue::Array(elements), &rest[1..]));
        } else {
            return Err("Expected ',' or ']'");
        }
    }
}

fn parse_string(input: &str) -> Result<(&str, &str), &'static str> {
    if !input.starts_with('"') {
        return Err("Expected string");
    }
    let rest = &input[1..];
    if let Some(pos) = rest.find('"') {
        Ok((&rest[..pos], &rest[pos + 1..]))
    } else {
        Err("Unterminated string")
    }
}

fn parse_number(input: &str) -> Result<(f64, &str), &'static str> {
    let mut end = 0;
    for (i, c) in input.char_indices() {
        if !(c.is_ascii_digit() || c == '.' || c == '-') {
            break;
        }
        end = i + 1;
    }
    let num_str = &input[..end];
    num_str.parse::<f64>()
        .map(|n| (n, &input[end..]))
        .map_err(|_| "Invalid number")
}

pub fn pretty_print_json(value: &JsonValue, indent: usize) -> String {
    let mut result = String::new();
    let indent_str = "  ".repeat(indent);

    match value {
        JsonValue::String(s) => result.push_str(&format!("\"{}\"", s)),
        JsonValue::Number(n) => result.push_str(&n.to_string()),
        JsonValue::Boolean(b) => result.push_str(&b.to_string()),
        JsonValue::Null => result.push_str("null"),
        JsonValue::Array(arr) => {
            result.push_str("[\n");
            for (i, val) in arr.iter().enumerate() {
                result.push_str(&"  ".repeat(indent + 1));
                result.push_str(&pretty_print_json(val, indent + 1));
                if i < arr.len() - 1 {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&indent_str);
            result.push(']');
        }
        JsonValue::Object(obj) => {
            result.push_str("{\n");
            for (i, (k, v)) in obj.iter().enumerate() {
                result.push_str(&"  ".repeat(indent + 1));
                result.push_str(&format!("\"{}\": {}", k, pretty_print_json(v, indent + 1)));
                if i < obj.len() - 1 {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&indent_str);
            result.push('}');
        }
    }

    result
}

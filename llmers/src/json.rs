/// Find the value (after colon) for a given key at the **current nesting level**
/// (depth 1 from the outermost object). Returns a slice pointing to the start
/// of the value (still inside the original json).
pub(crate) fn find_json_key<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{}\"", key);
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut chars = json.char_indices();
    while let Some((idx, c)) = chars.next() {
        if in_string {
            match c {
                '\\' => {
                    chars.next();
                } // skip escaped char
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }
        if c == '"' {
            // Check if we are at depth 1 (inside top-level object)
            if depth == 1 {
                let rest = &json[idx..];
                if rest.starts_with(&needle) {
                    let after_key = &rest[needle.len()..];
                    // skip whitespace then colon
                    let mut it = after_key.char_indices();
                    for (_, c) in it.by_ref() {
                        if c == ':' {
                            let val_start = after_key[it.offset()..].trim_start();
                            return Some(val_start);
                        } else if !c.is_whitespace() {
                            break;
                        }
                    }
                }
            }
            in_string = true;
            continue;
        }
        match c {
            '{' | '[' => depth += 1,
            '}' | ']' => depth -= 1,
            _ => {}
        }
    }
    None
}

/// Escape a string for JSON (without surrounding quotes)
fn json_escape_body(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            _ => out.push(c),
        }
    }
    out
}

/// Produce a quoted, escaped JSON string.
pub fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    out.push_str(&json_escape_body(s));
    out.push('"');
    out
}

fn to_alternating_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}

/// Unescape a JSON string **value** (input without surrounding quotes, or
/// including quotes we strip). Returns decoded string.
pub fn json_unescape(mut s: &str) -> String {
    if s.starts_with('"') && s.ends_with('"') {
        s = &s[1..s.len() - 1];
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some('/') => out.push('/'),
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('b') => out.push('\x08'),
                Some('f') => out.push('\x0C'),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    if let Ok(cp) = u32::from_str_radix(&hex, 16) {
                        if let Some(c) = char::from_u32(cp) {
                            out.push(c);
                        }
                    }
                }
                Some(c) => out.push(c),
                None => break,
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Extract a JSON string value for the given key. Returns `None` if not found
/// or not a string
pub fn json_get_string(json: &str, key: &str) -> Option<String> {
    let val_start = find_json_key(json, key)?;
    let trimmed = val_start.trim_start();
    if trimmed.is_empty() || !trimmed.starts_with('"') {
        return None;
    }
    // Find the end of the string (handling escapes)
    let mut end = None;
    let mut chars = trimmed[1..].char_indices();
    while let Some((i, c)) = chars.next() {
        if c == '\\' {
            chars.next(); // skip escaped
        } else if c == '"' {
            end = Some(i);
            break;
        }
    }
    end.map(|i| json_unescape(&trimmed[..=i + 1]))
}

/// Extract an integer value.
pub fn json_get_int(json: &str, key: &str) -> i64 {
    if let Some(val_start) = find_json_key(json, key) {
        let trimmed = val_start.trim_start();
        // parse until non-digit or comma/end
        trimmed
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    } else {
        0
    }
}

/// Extract a nested object or array as a slice (still borrowing the original).
pub(crate) fn json_get_nested<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let val_start = find_json_key(json, key)?;
    let trimmed = val_start.trim_start();
    let first_char = trimmed.chars().next()?;
    if first_char != '{' && first_char != '[' {
        return None;
    }
    let open = first_char;
    let close = if open == '{' { '}' } else { ']' };
    let mut depth = 0i32;
    let mut in_string = false;
    for (i, c) in trimmed.char_indices() {
        if in_string {
            if c == '\\' { /* skip next */
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        if c == '"' {
            in_string = true;
            continue;
        }
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(&trimmed[..=i]);
            }
        }
    }
    None
}

/// Count elements in a JSON array (slice of the form "[...]").
pub fn json_array_len(arr: &str) -> usize {
    if arr.is_empty() || !arr.starts_with('[') {
        return 0;
    }
    let inner = arr[1..arr.len() - 1].trim();
    if inner.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut depth = 0;
    let mut in_string = false;
    for c in inner.chars() {
        if in_string {
            if c == '\\' { /* skip next */
            } else if c == '"' {
                in_string == false;
            }
            continue;
        }
        if c == '"' {
            in_string = true;
            continue;
        }
        match c {
            '{' | '[' => depth += 1,
            '}' | ']' => depth -= 1,
            ',' if depth == 0 => count += 1,
            _ => {}
        }
    }
    count + 1
}

/// Get the i-th element of a JSON array as a slice (still borrowing).
pub(crate) fn json_array_get<'a>(arr: &'a str, index: usize) -> Option<&'a str> {
    if arr.is_empty() || !arr.starts_with('[') {
        return None;
    }
    let inner = &arr[1..arr.len() - 1];
    let mut start = 0;
    let mut depth = 0;
    let mut in_string = false;
    let mut current_idx = 0;
    for (i, c) in inner.char_indices() {
        if in_string {
            if c == '\\' { /* skip next */
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        if c == '"' {
            in_string = true;
            continue;
        }
        match c {
            '{' | '[' => depth += 1,
            '}' | ']' => depth -= 1,
            ',' if depth == 0 => {
                if current_idx == index {
                    return Some(inner[start..i].trim());
                }
                start = i + 1;
                current_idx += 1;
            }
            _ => {}
        }
    }
    if current_idx == index {
        Some(inner[start..].trim())
    } else {
        None
    }
}

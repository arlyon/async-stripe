use std::fmt::Write as _;

use heck::SnakeCase;
use lazy_static::lazy_static;
use openapiv3::{IntegerFormat, Schema, VariantOrUnknownOrEmpty};
use regex::Regex;

use crate::file_generator::FileGenerator;

pub fn write_out_field(out: &mut String, var_name: &str, var_type: &str, required: bool) {
    if required {
        writeln!(out, "    pub {var_name}: {var_type},").unwrap();
    } else {
        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
        writeln!(out, "    pub {var_name}: Option<{var_type}>,").unwrap();
    }
}

pub fn write_serde_rename(out: &mut String, rename: &str) {
    writeln!(out, r#"    #[serde(rename = "{rename}")]"#).unwrap();
}

pub fn print_doc_from_schema(out: &mut String, schema: &Schema, print_level: u8) {
    if let Some(description) = &schema.schema_data.description {
        print_doc_comment(out, description, print_level);
    }
}

pub fn print_doc_comment(out: &mut String, description: &str, depth: u8) {
    if description.trim().is_empty() {
        return;
    }

    let doc = format_doc_comment(description);
    let mut doc_parts = doc.splitn(2, ". ");
    let head = doc_parts.next().unwrap().trim();
    for (i, line) in head.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        print_indent(out, depth);
        if !line.is_empty() {
            out.push_str("/// ");
            out.push_str(line);
        } else {
            out.push_str("///");
        }
    }
    if !head.ends_with('.') {
        out.push('.');
    }
    out.push('\n');
    if let Some(tail) = doc_parts.next() {
        print_indent(out, depth);
        out.push_str("///\n");
        for part in tail.split(". ") {
            print_indent(out, depth);
            out.push_str("/// ");
            out.push_str(part.replace('\n', " ").trim());
            if !part.ends_with('.') {
                out.push('.');
            }
            out.push('\n');
        }
    }
}

fn print_indent(out: &mut String, depth: u8) {
    for _ in 0..depth {
        out.push_str("    ");
    }
}

fn format_doc_comment(doc: &str) -> String {
    lazy_static! {
        static ref P_TAG: Regex = Regex::new("<p>|</p>").unwrap();
        static ref BR_TAG: Regex = Regex::new("<br ?/?>").unwrap();
        static ref A_DOC_TAG: Regex = Regex::new("<a href=\"/docs/([^\"]+)\">([^<]+)</a>").unwrap();
        static ref A_HASH_TAG: Regex = Regex::new("<a href=\"#([^\"]+)\">([^<]+)</a>").unwrap();
        static ref A_HTTP_TAG: Regex =
            Regex::new("<a href=\"(https?://[^\"]+)\">([^<]+)</a>").unwrap();
        static ref CODE_TAG: Regex = Regex::new("<code>|</code>").unwrap();
        static ref EM_TAG: Regex = Regex::new("<em>|</em>|<i>|</i>").unwrap();
        static ref STRONG_TAG: Regex = Regex::new("<strong>|</strong>|<b>|</b>").unwrap();
        static ref AMOUNT_OPEN_TAG: Regex = Regex::new("<amount>").unwrap();
        static ref AMOUNT_CLOSE_TAG: Regex = Regex::new("</amount>").unwrap();
        static ref CURRENCY_OPEN_TAG: Regex = Regex::new("<currency>").unwrap();
        static ref CURRENCY_CLOSE_TAG: Regex = Regex::new("</currency>").unwrap();
        static ref HYPERLINK: Regex = Regex::new(r#"([^\(])(https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*))([^\)])"#).unwrap();
    }
    let doc = P_TAG.replace_all(doc, "");
    let doc = BR_TAG.replace_all(&doc, "\n");
    let doc = A_DOC_TAG.replace_all(&doc, "[${2}](https://stripe.com/docs/${1})");
    let doc = A_HASH_TAG.replace_all(&doc, "[${2}](https://stripe.com/docs/api#${1})");
    let doc = A_HTTP_TAG.replace_all(&doc, "[$2]($1)");
    let doc = CODE_TAG.replace_all(&doc, "`");
    let doc = EM_TAG.replace_all(&doc, "_");
    let doc = STRONG_TAG.replace_all(&doc, "**");
    let doc = AMOUNT_OPEN_TAG.replace_all(&doc, "");
    let doc = AMOUNT_CLOSE_TAG.replace_all(&doc, "00"); // add cents to get correct "integer" argument
    let doc = CURRENCY_OPEN_TAG.replace_all(&doc, "$"); // add locale formatting (we can only support one easily in our rust docs...)
    let doc = CURRENCY_CLOSE_TAG.replace_all(&doc, "");
    let doc = HYPERLINK.replace_all(&doc, "$1<$2>$5"); // replace all hyperlinks that are not already in markdown with rust doc links
    doc.trim().into()
}

pub fn infer_integer_type(
    state: &mut FileGenerator,
    name: &str,
    int_fmt: &VariantOrUnknownOrEmpty<IntegerFormat>,
) -> String {
    let is_unix_time_fmt = match int_fmt {
        VariantOrUnknownOrEmpty::Unknown(val) => val == "unix-time",
        _ => false,
    };
    let name_snake = name.to_snake_case();
    let name_words = name_snake.split('_').collect::<Vec<_>>();
    if is_unix_time_fmt || name_words.contains(&"date") {
        state.use_params.insert("Timestamp");
        "Timestamp".into()
    } else if name == "monthly_anchor" {
        "u8".into()
    } else if name_words.contains(&"days") {
        "u32".into()
    } else if name_words.contains(&"count")
        || name_words.contains(&"size")
        || name_words.contains(&"quantity")
    {
        "u64".into()
    } else {
        "i64".into()
    }
}

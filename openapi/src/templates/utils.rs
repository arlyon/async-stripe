use std::fmt::{Display, Formatter, Write};

use indoc::writedoc;
use lazy_static::lazy_static;
use regex_lite::Regex;

use crate::types::RustIdent;

lazy_static! {
    /// Regex for a period followed by 1 or 2 whitespaces
    static ref PERIOD_THEN_WHITESPACE: Regex = Regex::new(r"\.\s[\s]?").unwrap();
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SerdeDeriveState {
    serialize: Option<ShouldDerive>,
    deserialize: Option<ShouldDerive>,
}

impl SerdeDeriveState {
    pub fn serialize(&mut self, kind: ShouldDerive) -> &mut Self {
        self.serialize = Some(kind);
        self
    }

    pub fn deserialize(&mut self, kind: ShouldDerive) -> &mut Self {
        self.deserialize = Some(kind);
        self
    }

    pub fn write_derives(&self, out: &mut String) {
        if let Some(derive) = self.serialize {
            write_derive(derive, SerOrDeser::Ser, out);
        }
        if let Some(derive) = self.deserialize {
            write_derive(derive, SerOrDeser::Deser, out);
        }
    }

    pub fn maybe_write_tag(&self, out: &mut String, tag: impl Display) {
        let feature_gate = match self.usage() {
            SerdeUsage::Never => {
                return;
            }
            SerdeUsage::Always => {
                let _ = writeln!(out, r#"#[serde({tag})]"#);
                return;
            }
            SerdeUsage::SerGated => SerFeatureGate::Ser,
            SerdeUsage::DeserGated => SerFeatureGate::Deser,
            SerdeUsage::EitherGated => SerFeatureGate::Either,
        };
        let _ = writeln!(out, r#"#[cfg_attr({feature_gate}, serde({tag}))]"#);
    }

    pub fn maybe_write_rename(&self, out: &mut String, rename: &str) {
        self.maybe_write_tag(out, format!(r#"rename = "{rename}""#));
    }

    fn usage(&self) -> SerdeUsage {
        match (self.serialize, self.deserialize) {
            (None, None) => SerdeUsage::Never,
            (Some(state), None) => match state {
                ShouldDerive::Always => SerdeUsage::Always,
                ShouldDerive::Gated => SerdeUsage::SerGated,
            },
            (None, Some(state)) => match state {
                ShouldDerive::Always => SerdeUsage::Always,
                ShouldDerive::Gated => SerdeUsage::DeserGated,
            },
            (Some(state_ser), Some(state_deser)) => match (state_ser, state_deser) {
                (ShouldDerive::Always, _) => SerdeUsage::Always,
                (_, ShouldDerive::Always) => SerdeUsage::Always,
                (ShouldDerive::Gated, ShouldDerive::Gated) => SerdeUsage::EitherGated,
            },
        }
    }
}

fn write_derive(derive: ShouldDerive, ser: SerOrDeser, out: &mut String) {
    match derive {
        ShouldDerive::Always => {
            let _ = writeln!(out, "#[derive(serde::{ser})]");
        }
        ShouldDerive::Gated => {
            let _ = writeln!(
                out,
                r#"#[cfg_attr(feature = "{}", derive(serde::{ser}))]"#,
                ser.feature_gate()
            );
        }
    }
}

enum SerdeUsage {
    SerGated,
    DeserGated,
    EitherGated,
    Always,
    Never,
}

#[derive(Debug, Copy, Clone)]
pub enum ShouldDerive {
    Always,
    Gated,
}

#[derive(Debug, Copy, Clone)]
enum SerFeatureGate {
    Ser,
    Deser,
    Either,
}

impl Display for SerFeatureGate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SerFeatureGate::Ser => r#"feature = "serialize""#,
            SerFeatureGate::Deser => r#"feature = "deserialize""#,
            SerFeatureGate::Either => r#"any(feature = "deserialize", feature = "serialize")"#,
        })
    }
}

enum SerOrDeser {
    Ser,
    Deser,
}

impl SerOrDeser {
    fn feature_gate(&self) -> &'static str {
        match self {
            Self::Ser => "serialize",
            Self::Deser => "deserialize",
        }
    }
}

pub fn write_default_impl(ident: &RustIdent, lifetime_str: &str, out: &mut String) {
    let _ = writedoc!(
        out,
        r"
        impl{lifetime_str} Default for {ident}{lifetime_str} {{
            fn default() -> Self {{
                Self::new()
            }}
        }}"
    );
}

impl Display for SerOrDeser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Ser => "Serialize",
            Self::Deser => "Deserialize",
        })
    }
}

/// Write a formatted doc comment.
///
/// Where Stripe's description has reasonable line length, we follow their newline usage (happy, simple case).
/// When Stripe does not include newlines, we manually insert newlines on each sentence end.
pub fn write_doc_comment(description: &str, depth: u8) -> String {
    if description.trim().is_empty() {
        return String::new();
    }
    let mut out = String::with_capacity(description.len());
    let doc = format_doc_comment(description);

    for (i, line) in doc.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        print_indent(&mut out, depth);
        if line.is_empty() {
            out.push_str("///");
            continue;
        }
        // If a reasonable line, follow Stripe
        if line.len() < 100 {
            out.push_str("/// ");
            out.push_str(line);
            // If an unreasonable line, inject newlines after each sentence
        } else {
            for (i, part) in PERIOD_THEN_WHITESPACE.split(line).enumerate() {
                if i > 0 {
                    out.push('\n');
                }
                print_indent(&mut out, depth);
                out.push_str("/// ");
                out.push_str(part.trim());
                if !part.is_empty() && !out.ends_with('.') {
                    out.push('.');
                }
            }
        }
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

pub fn write_top_level_doc_comment(description: &str) -> String {
    let doc = format_doc_comment(description);

    let mut out = String::new();
    for (i, line) in doc.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        if !line.is_empty() {
            out.push_str("//! ");
            out.push_str(line);
        } else {
            out.push_str("//!");
        }
    }
    out.push('\n');
    out
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
        static ref A_HTTP_TAG: Regex = Regex::new("<a href=\"(https?://[^\"]+)\">([^<]+)</a>").unwrap();
        static ref CODE_TAG: Regex = Regex::new("<code>|</code>").unwrap();
        static ref EM_TAG: Regex = Regex::new("<em>|</em>|<i>|</i>").unwrap();
        static ref STRONG_TAG: Regex = Regex::new("<strong>|</strong>|<b>|</b>").unwrap();
        static ref AMOUNT_OPEN_TAG: Regex = Regex::new("<amount>").unwrap();
        static ref AMOUNT_CLOSE_TAG: Regex = Regex::new("</amount>").unwrap();
        static ref CURRENCY_OPEN_TAG: Regex = Regex::new("<currency>").unwrap();
        static ref CURRENCY_CLOSE_TAG: Regex = Regex::new("</currency>").unwrap();
        static ref HYPERLINK: Regex = Regex::new(r"([^\(])(https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*))([^\)])").unwrap();
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
    let doc = doc.replace("[Deprecated]", r"\[Deprecated\]"); // Otherwise rustdoc creates broken intra-doc links
    doc.trim().into()
}

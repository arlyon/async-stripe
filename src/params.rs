use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: Option<u64>,
    pub url: String,
}

pub type Metadata = HashMap<String, String>;
pub type Timestamp = i64;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RangeBounds<T> {
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
}

impl<T> Default for RangeBounds<T> {
    fn default() -> Self {
        RangeBounds {
            gt: None,
            gte: None,
            lt: None,
            lte: None,
        }
    }
}

/// A set of generic request parameters that can be used on
/// list endpoints to filter their results by some timestamp.
#[derive(Serialize)]
#[serde(untagged)]
pub enum RangeQuery<T> {
    Exact(T),
    Bounds(RangeBounds<T>),
}

impl<T> RangeQuery<T> {
    /// Filter results to exactly match a given value
    pub fn eq(value: T) -> RangeQuery<T> {
        RangeQuery::Exact(value)
    }

    /// Filter results to be after a given value
    pub fn gt(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gt = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be after or equal to a given value
    pub fn gte(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gte = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be before to a given value
    pub fn lt(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gt = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be before or equal to a given value
    pub fn lte(value: T) -> RangeQuery<T> {
        let mut bounds = RangeBounds::default();
        bounds.gte = Some(value);
        RangeQuery::Bounds(bounds)
    }
}

// NOTE: Only intended to handle conversion from ASCII CamelCase to SnakeCase
//   This function is used to convert static Rust identifiers to snakecase
// TODO: pub(crate) fn
pub fn to_snakecase(camel: &str) -> String {
    let mut i = 0;
    let mut snake = String::new();
    let mut chars = camel.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if i > 0 && !chars.peek().unwrap_or(&'A').is_uppercase() {
                snake.push('_');
            }
            snake.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            snake.push(ch);
        }
        i += 1;
    }

    snake
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_snakecase() {
        use super::to_snakecase;

        assert_eq!(to_snakecase("snake_case").as_str(), "snake_case");
        assert_eq!(to_snakecase("CamelCase").as_str(), "camel_case");
        assert_eq!(to_snakecase("XMLHttpRequest").as_str(), "xml_http_request");
        assert_eq!(to_snakecase("UPPER").as_str(), "upper");
        assert_eq!(to_snakecase("lower").as_str(), "lower");
    }
}

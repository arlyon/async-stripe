#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TokenId(String);

impl ::std::fmt::Display for TokenId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for TokenId {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("tok_") {
            Err(ParseIdError { prefix: "tok_" })
        } else {
            Ok(TokenId(s.to_owned()))
        }
    }
}

impl ::serde::Serialize for TokenId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::ser::Serializer
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> ::serde::Deserialize<'de> for TokenId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::de::Deserializer<'de>
    {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        s.parse::<Self>().map_err(|e| ::serde::de::Error::custom(e))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SourceId(String);

impl ::std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::str::FromStr for SourceId {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("src_") {
            Err(ParseIdError { prefix: "src_" })
        } else {
            Ok(SourceId(s.to_owned()))
        }
    }
}

impl ::serde::Serialize for SourceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::ser::Serializer
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> ::serde::Deserialize<'de> for SourceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::de::Deserializer<'de>
    {
        let s: String = ::serde::Deserialize::deserialize(deserializer)?;
        s.parse::<Self>().map_err(|e| ::serde::de::Error::custom(e))
    }
}

#[derive(Debug)]
pub struct ParseIdError { prefix: &'static str }

impl ::std::fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "expected id to start with '{}'", self.prefix)
    }
}

impl ::std::error::Error for ParseIdError {
    fn description(&self) -> &str {
        "error parsing id"
    }
}

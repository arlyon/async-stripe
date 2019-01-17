macro_rules! id {
    ($newtype_name:ident, $prefix:expr) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $newtype_name(String);

        impl $newtype_name {
            #[inline]
            pub fn prefix() -> &'static str {
                $prefix
            }
        }

        impl ::std::fmt::Display for $newtype_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl ::std::str::FromStr for $newtype_name {
            type Err = ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if !s.starts_with($prefix) {
                    Err(ParseIdError {
                        typename: stringify!($newtype_name),
                        expected: stringify!(id to start with $prefix),
                    })
                } else {
                    Ok($newtype_name(s.to_owned()))
                }
            }
        }

        impl ::serde::Serialize for $newtype_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::ser::Serializer
            {
                self.to_string().serialize(serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $newtype_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::de::Deserializer<'de>
            {
                let s: String = ::serde::Deserialize::deserialize(deserializer)?;
                s.parse::<Self>().map_err(::serde::de::Error::custom)
            }
        }
    };

    ($enum_name:ident { $( $variant_name:ident($($variant_type:tt)*) ),* $(,)* }) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum $enum_name {$(
            $variant_name($($variant_type)*),
        )*}

        impl ::std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {$(
                    $enum_name::$variant_name(ref id) => id.fmt(f),
                )*}
            }
        }

        impl ::std::str::FromStr for $enum_name {
            type Err = ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let prefix = s.find('_')
                    .map(|i| &s[0..=i])
                    .ok_or_else(|| ParseIdError {
                        typename: stringify!($enum_name),
                        expected: "id to start with a prefix (as in 'prefix_')"
                    })?;

                match prefix {
                    $(_ if prefix == $($variant_type)*::prefix() => {
                        Ok($enum_name::$variant_name(s.parse()?))
                    })*
                    _ => {
                        Err(ParseIdError {
                            typename: stringify!($enum_name),
                            expected: "unknown id prefix",
                        })
                    }
                }
            }
        }

        impl ::serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::ser::Serializer
            {
                self.to_string().serialize(serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::de::Deserializer<'de>
            {
                let s: String = ::serde::Deserialize::deserialize(deserializer)?;
                s.parse::<Self>().map_err(::serde::de::Error::custom)
            }
        }

        $(
            impl From<$($variant_type)*> for $enum_name {
                fn from(id: $($variant_type)*) -> Self {
                    $enum_name::$variant_name(id)
                }
            }
        )*
    };
}

#[derive(Clone, Debug)]
pub struct ParseIdError {
    typename: &'static str,
    expected: &'static str,
}

impl ::std::fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "invalid `{}`, expected {}", self.typename, self.expected)
    }
}

impl ::std::error::Error for ParseIdError {
    fn description(&self) -> &str {
        "error parsing an id"
    }
}

id!(BankAccountId, "ba_");
id!(BankTokenId, "btok_");
id!(CardId, "card_");
id!(CardTokenId, "tok_");
id!(SourceId, "src_");
id!(TokenId {
    Card(CardTokenId),
    Bank(BankTokenId),
});
id!(PaymentSourceId {
    BankAcccount(BankAccountId),
    Card(CardId),
    Source(SourceId),
});

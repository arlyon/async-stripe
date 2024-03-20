#[doc(hidden)]
#[macro_export]
macro_rules! def_id_serde_impls {
    ($struct_name:ident) => {
        impl serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                self.as_str().serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
                s.parse::<Self>().map_err(::serde::de::Error::custom)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! def_id {
    ($struct_name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub struct $struct_name(smol_str::SmolStr);

        impl $struct_name {
            /// Extracts a string slice containing the entire id.
            #[inline]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            /// Obtain a reference to the underlying `SmolStr`.
            #[inline]
            pub fn inner(&self) -> &smol_str::SmolStr {
                &self.0
            }
        }

        impl stripe_types::AsCursor for $struct_name {
            fn as_cursor(&self) -> &str {
                self.0.as_str()
            }
        }

        impl PartialEq<str> for $struct_name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $struct_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for $struct_name {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other
            }
        }

        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $struct_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $struct_name {
            type Err = $crate::ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($struct_name(s.into()))
            }
        }

        $crate::def_id_serde_impls!($struct_name);
    };
    ($struct_name:ident, $prefix:literal $(| $alt_prefix:literal)*) => {
        /// An id for the corresponding object type.
        ///
        /// This type _typically_ will not allocate and
        /// therefore is usually cheaply clonable.
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub struct $struct_name(smol_str::SmolStr);

        impl $struct_name {
            /// The valid prefixes of the id type (e.g. [`ch_`, `py_`\ for a `ChargeId`).
            #[inline]
            pub fn prefixes() -> &'static [&'static str] {
                &[$prefix$(, $alt_prefix)*]
            }

            /// Extracts a string slice containing the entire id.
            #[inline]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            /// Obtain a reference to the underlying `SmolStr`.
            #[inline]
            pub fn inner(&self) -> &smol_str::SmolStr {
                &self.0
            }

            /// Check is provided prefix would be a valid prefix for id's of this type
            pub fn is_valid_prefix(prefix: &str) -> bool {
                prefix == $prefix $( || prefix == $alt_prefix )*
            }
        }

        impl stripe_types::AsCursor for $struct_name {
            fn as_cursor(&self) -> &str {
                self.0.as_str()
            }
        }

        impl PartialEq<str> for $struct_name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $struct_name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for $struct_name {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other
            }
        }

        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $struct_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $struct_name {
            type Err = $crate::ParseIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if !s.starts_with($prefix) $(
                    && !s.starts_with($alt_prefix)
                )* {
                    Err($crate::ParseIdError {
                        typename: stringify!($struct_name),
                        expected: stringify!(id to start with $prefix $(or $alt_prefix)*),
                    })
                } else {
                    Ok($struct_name(s.into()))
                }
            }
        }

        $crate::def_id_serde_impls!($struct_name);
    };
}

#[derive(Copy, Clone, Debug)]
pub struct ParseIdError {
    pub typename: &'static str,
    pub expected: &'static str,
}

impl std::fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid `{}`, expected {}", self.typename, self.expected)
    }
}

impl std::error::Error for ParseIdError {
    fn description(&self) -> &str {
        "error parsing an id"
    }
}

// Allow dead code in the tests to avoid warnings around unused code in macro expansions
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde::de::DeserializeOwned;
    use serde::Serialize;

    def_id!(ChargeId, "ch_" | "py_");
    def_id!(InvoiceId, "in_");
    def_id!(CustomerId, "cus_");
    def_id!(PriceId, "price_");
    def_id!(ProductId);
    def_id!(PaymentMethodId, "pm_" | "card_" | "src_" | "ba_");
    def_id!(RefundId, "re_" | "pyr_");
    def_id!(SubscriptionId, "sub_");

    fn assert_ser_de_roundtrip<T>(id: &str)
    where
        T: DeserializeOwned + Serialize + FromStr + std::fmt::Display + std::fmt::Debug,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let parsed_id = T::from_str(id).expect("Could not parse id");
        let ser = serde_json::to_string(&parsed_id).expect("Could not serialize id");
        let deser: T = serde_json::from_str(&ser).expect("Could not deserialize id");
        assert_eq!(deser.to_string(), id.to_string());
    }

    fn assert_deser_err<T: DeserializeOwned + std::fmt::Debug>(id: &str) {
        let json_str = format!(r#""{}""#, id);
        let deser: Result<T, _> = serde_json::from_str(&json_str);
        assert!(deser.is_err(), "Expected error, got {:?}", deser);
    }

    #[test]
    fn test_ser_de_roundtrip() {
        // Single prefixes
        for id in ["in_12345", "in_"] {
            assert_ser_de_roundtrip::<InvoiceId>(id);
        }
        assert_ser_de_roundtrip::<PriceId>("price_abc");

        // Case where multiple possible prefixes
        for id in ["re_bcd", "pyr_123"] {
            assert_ser_de_roundtrip::<RefundId>(id);
        }

        // Case where id can be anything
        for id in ["anything", ""] {
            assert_ser_de_roundtrip::<ProductId>(id);
        }
    }

    #[test]
    fn test_deser_err() {
        // Single prefix
        assert_deser_err::<InvoiceId>("in");

        for id in ["sub", ""] {
            assert_deser_err::<SubscriptionId>(id);
        }

        // Case where multiple possible prefixes
        for id in ["abc_bcd", "pyr_123"] {
            assert_deser_err::<PaymentMethodId>(id);
        }
    }

    #[test]
    fn test_parse_customer() {
        assert!("cus_123".parse::<CustomerId>().is_ok());
        let bad_parse = "zzz_123".parse::<CustomerId>();
        assert!(bad_parse.is_err());
        if let Err(err) = bad_parse {
            assert_eq!(
                format!("{}", err),
                "invalid `CustomerId`, expected id to start with \"cus_\""
            );
        }
    }

    #[test]
    fn test_parse_charge() {
        assert!("ch_123".parse::<ChargeId>().is_ok());
        assert!("py_123".parse::<ChargeId>().is_ok());
        let bad_parse = "zz_123".parse::<ChargeId>();
        assert!(bad_parse.is_err());
        if let Err(err) = bad_parse {
            assert_eq!(
                format!("{}", err),
                "invalid `ChargeId`, expected id to start with \"ch_\" or \"py_\""
            );
        }
    }
}

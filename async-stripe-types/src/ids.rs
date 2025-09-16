#[allow(clippy::crate_in_macro_def)]
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

            /// Unwrap to the underlying `SmolStr`.
            #[inline]
            pub fn into_inner(self) -> smol_str::SmolStr {
                self.0
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
            type Err = std::convert::Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(smol_str::SmolStr::from(s)))
            }
        }

        impl serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                self.as_str().serialize(serializer)
            }
        }

        impl From<String> for $struct_name {
            #[inline]
            fn from(text: String) -> Self {
                Self(smol_str::SmolStr::from(text))
            }
        }

        impl From<&str> for $struct_name {
            #[inline]
            fn from(text: &str) -> Self {
                Self(smol_str::SmolStr::from(text))
            }
        }

        impl From<$struct_name> for String {
            #[inline]
            fn from(id: $struct_name) -> Self {
                id.0.to_string()
            }
        }

        impl From<&$struct_name> for String {
            #[inline]
            fn from(id: &$struct_name) -> Self {
                id.0.to_string()
            }
        }

        impl From<&$struct_name> for $struct_name {
            #[inline]
            fn from(id: &$struct_name) -> Self {
                id.clone()
            }
        }

        #[cfg(feature = "deserialize")]
        impl<'de> serde::Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let s: String = serde::Deserialize::deserialize(deserializer)?;
                Ok(Self::from(s))
            }
        }

        impl stripe_types::FromCursor for $struct_name {
            fn from_cursor(val: &str) -> Option<Self> {
                use std::str::FromStr;
                Self::from_str(val).ok()
            }
        }

        impl miniserde::Deserialize for $struct_name {
            fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
                crate::Place::new(out)
            }
        }

        impl miniserde::de::Visitor for crate::Place<$struct_name> {
            fn string(&mut self, s: &str) -> miniserde::Result<()> {
                self.out = Some(s.parse::<$struct_name>().map_err(|_| miniserde::Error)?);
                Ok(())
            }
        }
        $crate::impl_from_val_with_from_str!($struct_name);
    };
}

// Allow dead code in the tests to avoid warnings around unused code in macro expansions
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde::Serialize;

    def_id!(TestId);

    fn assert_ser_de_roundtrip<T>(id: &str)
    where
        T: miniserde::Deserialize + Serialize + FromStr + std::fmt::Display + std::fmt::Debug,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let parsed_id = T::from_str(id).expect("Could not parse id");
        let ser = serde_json::to_string(&parsed_id).expect("Could not serialize");
        let deser: T = miniserde::json::from_str(&ser).expect("Could not deserialize id");
        assert_eq!(deser.to_string(), id.to_string());
    }

    #[test]
    fn test_ser_de_roundtrip() {
        for id in ["in_12345", "in_", "", "price_abc"] {
            assert_ser_de_roundtrip::<TestId>(id);
        }
    }
}

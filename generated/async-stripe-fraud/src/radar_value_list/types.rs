/// Value lists allow you to group values together which can then be referenced in rules.
///
/// Related guide: [Default Stripe lists](https://docs.stripe.com/radar/lists#managing-list-items)
///
/// For more details see <<https://stripe.com/docs/api/radar/value_lists/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RadarValueList {
    /// The name of the value list for use in rules.
    pub alias: String,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who created this value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListId,
    /// The type of items in the value list.
    /// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, `customer_id`, `sepa_debit_fingerprint`, or `us_bank_account_fingerprint`.
    pub item_type: stripe_fraud::RadarValueListItemType,
    /// List of items contained within this value list.
    pub list_items: stripe_types::List<stripe_fraud::RadarValueListItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The name of the value list.
    pub name: String,
}
#[doc(hidden)]
pub struct RadarValueListBuilder {
    alias: Option<String>,
    created: Option<stripe_types::Timestamp>,
    created_by: Option<String>,
    id: Option<stripe_fraud::RadarValueListId>,
    item_type: Option<stripe_fraud::RadarValueListItemType>,
    list_items: Option<stripe_types::List<stripe_fraud::RadarValueListItem>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    name: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for RadarValueList {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarValueList>,
        builder: RadarValueListBuilder,
    }

    impl Visitor for Place<RadarValueList> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RadarValueListBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RadarValueListBuilder {
        type Out = RadarValueList;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alias" => Deserialize::begin(&mut self.alias),
                "created" => Deserialize::begin(&mut self.created),
                "created_by" => Deserialize::begin(&mut self.created_by),
                "id" => Deserialize::begin(&mut self.id),
                "item_type" => Deserialize::begin(&mut self.item_type),
                "list_items" => Deserialize::begin(&mut self.list_items),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "name" => Deserialize::begin(&mut self.name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                alias: Deserialize::default(),
                created: Deserialize::default(),
                created_by: Deserialize::default(),
                id: Deserialize::default(),
                item_type: Deserialize::default(),
                list_items: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(alias),
                Some(created),
                Some(created_by),
                Some(id),
                Some(item_type),
                Some(list_items),
                Some(livemode),
                Some(metadata),
                Some(name),
            ) = (
                self.alias.take(),
                self.created,
                self.created_by.take(),
                self.id.take(),
                self.item_type.take(),
                self.list_items.take(),
                self.livemode,
                self.metadata.take(),
                self.name.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                alias,
                created,
                created_by,
                id,
                item_type,
                list_items,
                livemode,
                metadata,
                name,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for RadarValueList {
        type Builder = RadarValueListBuilder;
    }

    impl FromValueOpt for RadarValueList {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RadarValueListBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "alias" => b.alias = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "created_by" => b.created_by = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "item_type" => b.item_type = FromValueOpt::from_value(v),
                    "list_items" => b.list_items = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for RadarValueList {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("RadarValueList", 10)?;
        s.serialize_field("alias", &self.alias)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("created_by", &self.created_by)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("item_type", &self.item_type)?;
        s.serialize_field("list_items", &self.list_items)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("name", &self.name)?;

        s.serialize_field("object", "radar.value_list")?;
        s.end()
    }
}
impl stripe_types::Object for RadarValueList {
    type Id = stripe_fraud::RadarValueListId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(RadarValueListId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RadarValueListItemType {
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CustomerId,
    Email,
    IpAddress,
    SepaDebitFingerprint,
    String,
    UsBankAccountFingerprint,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RadarValueListItemType {
    pub fn as_str(&self) -> &str {
        use RadarValueListItemType::*;
        match self {
            CardBin => "card_bin",
            CardFingerprint => "card_fingerprint",
            CaseSensitiveString => "case_sensitive_string",
            Country => "country",
            CustomerId => "customer_id",
            Email => "email",
            IpAddress => "ip_address",
            SepaDebitFingerprint => "sepa_debit_fingerprint",
            String => "string",
            UsBankAccountFingerprint => "us_bank_account_fingerprint",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RadarValueListItemType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RadarValueListItemType::*;
        match s {
            "card_bin" => Ok(CardBin),
            "card_fingerprint" => Ok(CardFingerprint),
            "case_sensitive_string" => Ok(CaseSensitiveString),
            "country" => Ok(Country),
            "customer_id" => Ok(CustomerId),
            "email" => Ok(Email),
            "ip_address" => Ok(IpAddress),
            "sepa_debit_fingerprint" => Ok(SepaDebitFingerprint),
            "string" => Ok(String),
            "us_bank_account_fingerprint" => Ok(UsBankAccountFingerprint),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "RadarValueListItemType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RadarValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RadarValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RadarValueListItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for RadarValueListItemType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RadarValueListItemType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RadarValueListItemType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(RadarValueListItemType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RadarValueListItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

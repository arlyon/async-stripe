#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceLinkAccountSessionFilters {
    /// Restricts the Session to subcategories of accounts that can be linked.
    /// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
    pub account_subcategories:
        Option<Vec<BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories>>,
    /// List of countries from which to filter accounts.
    pub countries: Option<Vec<String>>,
}
#[doc(hidden)]
pub struct BankConnectionsResourceLinkAccountSessionFiltersBuilder {
    account_subcategories:
        Option<Option<Vec<BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories>>>,
    countries: Option<Option<Vec<String>>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BankConnectionsResourceLinkAccountSessionFilters {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceLinkAccountSessionFilters>,
        builder: BankConnectionsResourceLinkAccountSessionFiltersBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceLinkAccountSessionFilters> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceLinkAccountSessionFiltersBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceLinkAccountSessionFiltersBuilder {
        type Out = BankConnectionsResourceLinkAccountSessionFilters;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_subcategories" => Deserialize::begin(&mut self.account_subcategories),
                "countries" => Deserialize::begin(&mut self.countries),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_subcategories: Deserialize::default(),
                countries: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account_subcategories), Some(countries)) =
                (self.account_subcategories.take(), self.countries.take())
            else {
                return None;
            };
            Some(Self::Out { account_subcategories, countries })
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

    impl ObjectDeser for BankConnectionsResourceLinkAccountSessionFilters {
        type Builder = BankConnectionsResourceLinkAccountSessionFiltersBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceLinkAccountSessionFilters {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceLinkAccountSessionFiltersBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_subcategories" => {
                        b.account_subcategories = FromValueOpt::from_value(v)
                    }
                    "countries" => b.countries = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Restricts the Session to subcategories of accounts that can be linked.
/// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Savings,
}
impl BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::*;
        match self {
            Checking => "checking",
            CreditCard => "credit_card",
            LineOfCredit => "line_of_credit",
            Mortgage => "mortgage",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::*;
        match s {
            "checking" => Ok(Checking),
            "credit_card" => Ok(CreditCard),
            "line_of_credit" => Ok(LineOfCredit),
            "mortgage" => Ok(Mortgage),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories"))
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceLinkAccountSessionFilters").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BankConnectionsResourceLinkAccountSessionFiltersBuilder {
                    account_subcategories: Deserialize::default(),
                    countries: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_subcategories" => {
                    Deserialize::begin(&mut self.builder.account_subcategories)
                }
                "countries" => Deserialize::begin(&mut self.builder.countries),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(account_subcategories), Some(countries)) =
                (self.builder.account_subcategories.take(), self.builder.countries.take())
            else {
                return Ok(());
            };
            *self.out = Some(BankConnectionsResourceLinkAccountSessionFilters {
                account_subcategories,
                countries,
            });
            Ok(())
        }
    }
};
/// Restricts the Session to subcategories of accounts that can be linked.
/// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Savings,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::*;
        match self {
            Checking => "checking",
            CreditCard => "credit_card",
            LineOfCredit => "line_of_credit",
            Mortgage => "mortgage",
            Savings => "savings",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::*;
        match s {
            "checking" => Ok(Checking),
            "credit_card" => Ok(CreditCard),
            "line_of_credit" => Ok(LineOfCredit),
            "mortgage" => Ok(Mortgage),
            "savings" => Ok(Savings),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories
        ))
        .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize
    for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

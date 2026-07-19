#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountBusinessProfile {
    /// The applicant's gross annual revenue for its preceding fiscal year.
    pub annual_revenue: Option<stripe_shared::AccountAnnualRevenue>,
    /// An estimated upper bound of employees, contractors, vendors, etc.
    /// currently working for the business.
    pub estimated_worker_count: Option<u64>,
    /// [The merchant category code for the account](/connect/setting-mcc).
    /// MCCs are used to classify businesses based on the goods or services they provide.
    pub mcc: Option<String>,
    /// Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business.
    pub minority_owned_business_designation:
        Option<Vec<AccountBusinessProfileMinorityOwnedBusinessDesignation>>,
    pub monthly_estimated_revenue: Option<stripe_shared::AccountMonthlyEstimatedRevenue>,
    /// The customer-facing business name.
    pub name: Option<String>,
    /// Internal-only description of the product sold or service provided by the business.
    /// It's used by Stripe for risk and underwriting purposes.
    pub product_description: Option<String>,
    /// A publicly available mailing address for sending support issues to.
    pub support_address: Option<stripe_shared::Address>,
    /// A publicly available email address for sending support issues to.
    pub support_email: Option<String>,
    /// A publicly available phone number to call with support issues.
    pub support_phone: Option<String>,
    /// A publicly available website for handling support issues.
    pub support_url: Option<String>,
    /// The business's publicly available website.
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountBusinessProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountBusinessProfile").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountBusinessProfileBuilder {
    annual_revenue: Option<Option<stripe_shared::AccountAnnualRevenue>>,
    estimated_worker_count: Option<Option<u64>>,
    mcc: Option<Option<String>>,
    minority_owned_business_designation:
        Option<Option<Vec<AccountBusinessProfileMinorityOwnedBusinessDesignation>>>,
    monthly_estimated_revenue: Option<Option<stripe_shared::AccountMonthlyEstimatedRevenue>>,
    name: Option<Option<String>>,
    product_description: Option<Option<String>>,
    support_address: Option<Option<stripe_shared::Address>>,
    support_email: Option<Option<String>>,
    support_phone: Option<Option<String>>,
    support_url: Option<Option<String>>,
    url: Option<Option<String>>,
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

    impl Deserialize for AccountBusinessProfile {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountBusinessProfile>,
        builder: AccountBusinessProfileBuilder,
    }

    impl Visitor for Place<AccountBusinessProfile> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountBusinessProfileBuilder {
                    annual_revenue: Deserialize::default(),
                    estimated_worker_count: Deserialize::default(),
                    mcc: Deserialize::default(),
                    minority_owned_business_designation: Deserialize::default(),
                    monthly_estimated_revenue: Deserialize::default(),
                    name: Deserialize::default(),
                    product_description: Deserialize::default(),
                    support_address: Deserialize::default(),
                    support_email: Deserialize::default(),
                    support_phone: Deserialize::default(),
                    support_url: Deserialize::default(),
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "annual_revenue" => Deserialize::begin(&mut self.builder.annual_revenue),
                "estimated_worker_count" => {
                    Deserialize::begin(&mut self.builder.estimated_worker_count)
                }
                "mcc" => Deserialize::begin(&mut self.builder.mcc),
                "minority_owned_business_designation" => {
                    Deserialize::begin(&mut self.builder.minority_owned_business_designation)
                }
                "monthly_estimated_revenue" => {
                    Deserialize::begin(&mut self.builder.monthly_estimated_revenue)
                }
                "name" => Deserialize::begin(&mut self.builder.name),
                "product_description" => Deserialize::begin(&mut self.builder.product_description),
                "support_address" => Deserialize::begin(&mut self.builder.support_address),
                "support_email" => Deserialize::begin(&mut self.builder.support_email),
                "support_phone" => Deserialize::begin(&mut self.builder.support_phone),
                "support_url" => Deserialize::begin(&mut self.builder.support_url),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(annual_revenue),
                Some(estimated_worker_count),
                Some(mcc),
                Some(minority_owned_business_designation),
                Some(monthly_estimated_revenue),
                Some(name),
                Some(product_description),
                Some(support_address),
                Some(support_email),
                Some(support_phone),
                Some(support_url),
                Some(url),
            ) = (
                self.builder.annual_revenue.take(),
                self.builder.estimated_worker_count,
                self.builder.mcc.take(),
                self.builder.minority_owned_business_designation.take(),
                self.builder.monthly_estimated_revenue.take(),
                self.builder.name.take(),
                self.builder.product_description.take(),
                self.builder.support_address.take(),
                self.builder.support_email.take(),
                self.builder.support_phone.take(),
                self.builder.support_url.take(),
                self.builder.url.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(AccountBusinessProfile {
                annual_revenue,
                estimated_worker_count,
                mcc,
                minority_owned_business_designation,
                monthly_estimated_revenue,
                name,
                product_description,
                support_address,
                support_email,
                support_phone,
                support_url,
                url,
            });
            Ok(())
        }
    }
};
/// Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountBusinessProfileMinorityOwnedBusinessDesignation {
    LgbtqiOwnedBusiness,
    MinorityOwnedBusiness,
    NoneOfTheseApply,
    PreferNotToAnswer,
    WomenOwnedBusiness,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountBusinessProfileMinorityOwnedBusinessDesignation {
    pub fn as_str(&self) -> &str {
        use AccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match self {
            LgbtqiOwnedBusiness => "lgbtqi_owned_business",
            MinorityOwnedBusiness => "minority_owned_business",
            NoneOfTheseApply => "none_of_these_apply",
            PreferNotToAnswer => "prefer_not_to_answer",
            WomenOwnedBusiness => "women_owned_business",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountBusinessProfileMinorityOwnedBusinessDesignation {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match s {
            "lgbtqi_owned_business" => Ok(LgbtqiOwnedBusiness),
            "minority_owned_business" => Ok(MinorityOwnedBusiness),
            "none_of_these_apply" => Ok(NoneOfTheseApply),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "women_owned_business" => Ok(WomenOwnedBusiness),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountBusinessProfileMinorityOwnedBusinessDesignation"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountBusinessProfileMinorityOwnedBusinessDesignation))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for AccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<AccountBusinessProfileMinorityOwnedBusinessDesignation>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AccountBusinessProfileMinorityOwnedBusinessDesignation::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}

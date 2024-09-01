#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountBusinessProfile {
    /// The applicant's gross annual revenue for its preceding fiscal year.
    pub annual_revenue: Option<stripe_shared::AccountAnnualRevenue>,
    /// An estimated upper bound of employees, contractors, vendors, etc.
    /// currently working for the business.
    pub estimated_worker_count: Option<u64>,
    /// [The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc).
    /// MCCs are used to classify businesses based on the goods or services they provide.
    pub mcc: Option<String>,
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
#[doc(hidden)]
pub struct AccountBusinessProfileBuilder {
    annual_revenue: Option<Option<stripe_shared::AccountAnnualRevenue>>,
    estimated_worker_count: Option<Option<u64>>,
    mcc: Option<Option<String>>,
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
                builder: AccountBusinessProfileBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountBusinessProfileBuilder {
        type Out = AccountBusinessProfile;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "annual_revenue" => Deserialize::begin(&mut self.annual_revenue),
                "estimated_worker_count" => Deserialize::begin(&mut self.estimated_worker_count),
                "mcc" => Deserialize::begin(&mut self.mcc),
                "monthly_estimated_revenue" => {
                    Deserialize::begin(&mut self.monthly_estimated_revenue)
                }
                "name" => Deserialize::begin(&mut self.name),
                "product_description" => Deserialize::begin(&mut self.product_description),
                "support_address" => Deserialize::begin(&mut self.support_address),
                "support_email" => Deserialize::begin(&mut self.support_email),
                "support_phone" => Deserialize::begin(&mut self.support_phone),
                "support_url" => Deserialize::begin(&mut self.support_url),
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                annual_revenue: Deserialize::default(),
                estimated_worker_count: Deserialize::default(),
                mcc: Deserialize::default(),
                monthly_estimated_revenue: Deserialize::default(),
                name: Deserialize::default(),
                product_description: Deserialize::default(),
                support_address: Deserialize::default(),
                support_email: Deserialize::default(),
                support_phone: Deserialize::default(),
                support_url: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(annual_revenue),
                Some(estimated_worker_count),
                Some(mcc),
                Some(monthly_estimated_revenue),
                Some(name),
                Some(product_description),
                Some(support_address),
                Some(support_email),
                Some(support_phone),
                Some(support_url),
                Some(url),
            ) = (
                self.annual_revenue.take(),
                self.estimated_worker_count,
                self.mcc.take(),
                self.monthly_estimated_revenue,
                self.name.take(),
                self.product_description.take(),
                self.support_address.take(),
                self.support_email.take(),
                self.support_phone.take(),
                self.support_url.take(),
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                annual_revenue,
                estimated_worker_count,
                mcc,
                monthly_estimated_revenue,
                name,
                product_description,
                support_address,
                support_email,
                support_phone,
                support_url,
                url,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for AccountBusinessProfile {
        type Builder = AccountBusinessProfileBuilder;
    }

    impl FromValueOpt for AccountBusinessProfile {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountBusinessProfileBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "annual_revenue" => b.annual_revenue = FromValueOpt::from_value(v),
                    "estimated_worker_count" => {
                        b.estimated_worker_count = FromValueOpt::from_value(v)
                    }
                    "mcc" => b.mcc = FromValueOpt::from_value(v),
                    "monthly_estimated_revenue" => {
                        b.monthly_estimated_revenue = FromValueOpt::from_value(v)
                    }
                    "name" => b.name = FromValueOpt::from_value(v),
                    "product_description" => b.product_description = FromValueOpt::from_value(v),
                    "support_address" => b.support_address = FromValueOpt::from_value(v),
                    "support_email" => b.support_email = FromValueOpt::from_value(v),
                    "support_phone" => b.support_phone = FromValueOpt::from_value(v),
                    "support_url" => b.support_url = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

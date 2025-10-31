#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FinancialReportingFinanceReportRunRunParameters {
    /// The set of output columns requested for inclusion in the report run.
    pub columns: Option<Vec<String>>,
    /// Connected account ID by which to filter the report run.
    pub connected_account: Option<String>,
    /// Currency of objects to be included in the report run.
    pub currency: Option<stripe_types::Currency>,
    /// Ending timestamp of data to be included in the report run.
    /// Can be any UTC timestamp between 1 second after the user specified `interval_start` and 1 second before this report's last `data_available_end` value.
    pub interval_end: Option<stripe_types::Timestamp>,
    /// Starting timestamp of data to be included in the report run.
    /// Can be any UTC timestamp between 1 second after this report's `data_available_start` and 1 second before the user specified `interval_end` value.
    pub interval_start: Option<stripe_types::Timestamp>,
    /// Payout ID by which to filter the report run.
    pub payout: Option<String>,
    /// Category of balance transactions to be included in the report run.
    pub reporting_category: Option<String>,
    /// Defaults to `Etc/UTC`.
    /// The output timezone for all timestamps in the report.
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    /// Has no effect on `interval_start` or `interval_end`.
    pub timezone: Option<String>,
}
#[doc(hidden)]
pub struct FinancialReportingFinanceReportRunRunParametersBuilder {
    columns: Option<Option<Vec<String>>>,
    connected_account: Option<Option<String>>,
    currency: Option<Option<stripe_types::Currency>>,
    interval_end: Option<Option<stripe_types::Timestamp>>,
    interval_start: Option<Option<stripe_types::Timestamp>>,
    payout: Option<Option<String>>,
    reporting_category: Option<Option<String>>,
    timezone: Option<Option<String>>,
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

    impl Deserialize for FinancialReportingFinanceReportRunRunParameters {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FinancialReportingFinanceReportRunRunParameters>,
        builder: FinancialReportingFinanceReportRunRunParametersBuilder,
    }

    impl Visitor for Place<FinancialReportingFinanceReportRunRunParameters> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FinancialReportingFinanceReportRunRunParametersBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FinancialReportingFinanceReportRunRunParametersBuilder {
        type Out = FinancialReportingFinanceReportRunRunParameters;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "columns" => Deserialize::begin(&mut self.columns),
                "connected_account" => Deserialize::begin(&mut self.connected_account),
                "currency" => Deserialize::begin(&mut self.currency),
                "interval_end" => Deserialize::begin(&mut self.interval_end),
                "interval_start" => Deserialize::begin(&mut self.interval_start),
                "payout" => Deserialize::begin(&mut self.payout),
                "reporting_category" => Deserialize::begin(&mut self.reporting_category),
                "timezone" => Deserialize::begin(&mut self.timezone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                columns: Deserialize::default(),
                connected_account: Deserialize::default(),
                currency: Deserialize::default(),
                interval_end: Deserialize::default(),
                interval_start: Deserialize::default(),
                payout: Deserialize::default(),
                reporting_category: Deserialize::default(),
                timezone: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(columns),
                Some(connected_account),
                Some(currency),
                Some(interval_end),
                Some(interval_start),
                Some(payout),
                Some(reporting_category),
                Some(timezone),
            ) = (
                self.columns.take(),
                self.connected_account.take(),
                self.currency.take(),
                self.interval_end,
                self.interval_start,
                self.payout.take(),
                self.reporting_category.take(),
                self.timezone.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                columns,
                connected_account,
                currency,
                interval_end,
                interval_start,
                payout,
                reporting_category,
                timezone,
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

    impl ObjectDeser for FinancialReportingFinanceReportRunRunParameters {
        type Builder = FinancialReportingFinanceReportRunRunParametersBuilder;
    }

    impl FromValueOpt for FinancialReportingFinanceReportRunRunParameters {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FinancialReportingFinanceReportRunRunParametersBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "columns" => b.columns = FromValueOpt::from_value(v),
                    "connected_account" => b.connected_account = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "interval_end" => b.interval_end = FromValueOpt::from_value(v),
                    "interval_start" => b.interval_start = FromValueOpt::from_value(v),
                    "payout" => b.payout = FromValueOpt::from_value(v),
                    "reporting_category" => b.reporting_category = FromValueOpt::from_value(v),
                    "timezone" => b.timezone = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

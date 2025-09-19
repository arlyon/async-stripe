#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationNetworkData {
    /// Identifier assigned to the acquirer by the card network.
    /// Sometimes this value is not provided by the network; in this case, the value will be `null`.
    pub acquiring_institution_id: Option<String>,
    /// The System Trace Audit Number (STAN) is a 6-digit identifier assigned by the acquirer.
    /// Prefer `network_data.transaction_id` if present, unless you have special requirements.
    pub system_trace_audit_number: Option<String>,
    /// Unique identifier for the authorization assigned by the card network used to match subsequent messages, disputes, and transactions.
    pub transaction_id: Option<String>,
}
#[doc(hidden)]
pub struct IssuingAuthorizationNetworkDataBuilder {
    acquiring_institution_id: Option<Option<String>>,
    system_trace_audit_number: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
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

    impl Deserialize for IssuingAuthorizationNetworkData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationNetworkData>,
        builder: IssuingAuthorizationNetworkDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationNetworkData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationNetworkDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationNetworkDataBuilder {
        type Out = IssuingAuthorizationNetworkData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acquiring_institution_id" => {
                    Deserialize::begin(&mut self.acquiring_institution_id)
                }
                "system_trace_audit_number" => {
                    Deserialize::begin(&mut self.system_trace_audit_number)
                }
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acquiring_institution_id: Deserialize::default(),
                system_trace_audit_number: Deserialize::default(),
                transaction_id: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(acquiring_institution_id),
                Some(system_trace_audit_number),
                Some(transaction_id),
            ) = (
                self.acquiring_institution_id.take(),
                self.system_trace_audit_number.take(),
                self.transaction_id.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { acquiring_institution_id, system_trace_audit_number, transaction_id })
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

    impl ObjectDeser for IssuingAuthorizationNetworkData {
        type Builder = IssuingAuthorizationNetworkDataBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationNetworkData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationNetworkDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acquiring_institution_id" => {
                        b.acquiring_institution_id = FromValueOpt::from_value(v)
                    }
                    "system_trace_audit_number" => {
                        b.system_trace_audit_number = FromValueOpt::from_value(v)
                    }
                    "transaction_id" => b.transaction_id = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

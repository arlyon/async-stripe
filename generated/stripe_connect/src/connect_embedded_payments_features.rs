#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedPaymentsFeatures {
    /// Whether to allow capturing and cancelling payment intents. This is `true` by default.
    pub capture_payments: bool,
    /// Whether to allow responding to disputes, including submitting evidence and accepting disputes.
    /// This is `true` by default.
    pub dispute_management: bool,
    /// Whether to allow sending refunds. This is `true` by default.
    pub refund_management: bool,
}
#[doc(hidden)]
pub struct ConnectEmbeddedPaymentsFeaturesBuilder {
    capture_payments: Option<bool>,
    dispute_management: Option<bool>,
    refund_management: Option<bool>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ConnectEmbeddedPaymentsFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedPaymentsFeatures>,
        builder: ConnectEmbeddedPaymentsFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedPaymentsFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedPaymentsFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedPaymentsFeaturesBuilder {
        type Out = ConnectEmbeddedPaymentsFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_payments" => Deserialize::begin(&mut self.capture_payments),
                "dispute_management" => Deserialize::begin(&mut self.dispute_management),
                "refund_management" => Deserialize::begin(&mut self.refund_management),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                capture_payments: Deserialize::default(),
                dispute_management: Deserialize::default(),
                refund_management: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                capture_payments: self.capture_payments?,
                dispute_management: self.dispute_management?,
                refund_management: self.refund_management?,
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

    impl ObjectDeser for ConnectEmbeddedPaymentsFeatures {
        type Builder = ConnectEmbeddedPaymentsFeaturesBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedPaymentsFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedPaymentsFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "capture_payments" => b.capture_payments = Some(FromValueOpt::from_value(v)?),
                    "dispute_management" => {
                        b.dispute_management = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_management" => b.refund_management = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

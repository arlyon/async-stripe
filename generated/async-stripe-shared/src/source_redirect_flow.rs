#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceRedirectFlow {
    /// The failure reason for the redirect, either `user_abort` (the customer aborted or dropped out of the redirect flow), `declined` (the authentication failed or the transaction was declined), or `processing_error` (the redirect failed due to a technical error).
    /// Present only if the redirect status is `failed`.
    pub failure_reason: Option<String>,
    /// The URL you provide to redirect the customer to after they authenticated their payment.
    pub return_url: String,
    /// The status of the redirect, either `pending` (ready to be used by your customer to authenticate the transaction), `succeeded` (successful authentication, cannot be reused) or `not_required` (redirect should not be used) or `failed` (failed authentication, cannot be reused).
    pub status: String,
    /// The URL provided to you to redirect a customer to as part of a `redirect` authentication flow.
    pub url: String,
}
#[doc(hidden)]
pub struct SourceRedirectFlowBuilder {
    failure_reason: Option<Option<String>>,
    return_url: Option<String>,
    status: Option<String>,
    url: Option<String>,
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

    impl Deserialize for SourceRedirectFlow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceRedirectFlow>,
        builder: SourceRedirectFlowBuilder,
    }

    impl Visitor for Place<SourceRedirectFlow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceRedirectFlowBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceRedirectFlowBuilder {
        type Out = SourceRedirectFlow;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "failure_reason" => Deserialize::begin(&mut self.failure_reason),
                "return_url" => Deserialize::begin(&mut self.return_url),
                "status" => Deserialize::begin(&mut self.status),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                failure_reason: Deserialize::default(),
                return_url: Deserialize::default(),
                status: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(failure_reason), Some(return_url), Some(status), Some(url)) = (
                self.failure_reason.take(),
                self.return_url.take(),
                self.status.take(),
                self.url.take(),
            ) else {
                return None;
            };
            Some(Self::Out { failure_reason, return_url, status, url })
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

    impl ObjectDeser for SourceRedirectFlow {
        type Builder = SourceRedirectFlowBuilder;
    }

    impl FromValueOpt for SourceRedirectFlow {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceRedirectFlowBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "failure_reason" => b.failure_reason = FromValueOpt::from_value(v),
                    "return_url" => b.return_url = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct NotificationEventRequest {
    /// ID of the API request that caused the event.
    /// If null, the event was automatic (e.g., Stripe's automatic subscription handling).
    /// Request logs are available in the [dashboard](https://dashboard.stripe.com/logs), but currently not in the API.
    pub id: Option<String>,
    /// The idempotency key transmitted during the request, if any.
    /// *Note: This property is populated only for events on or after May 23, 2017*.
    pub idempotency_key: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for NotificationEventRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NotificationEventRequest").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct NotificationEventRequestBuilder {
    id: Option<Option<String>>,
    idempotency_key: Option<Option<String>>,
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

    impl Deserialize for NotificationEventRequest {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<NotificationEventRequest>,
        builder: NotificationEventRequestBuilder,
    }

    impl Visitor for Place<NotificationEventRequest> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: NotificationEventRequestBuilder {
                    id: Deserialize::default(),
                    idempotency_key: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.builder.id),
                "idempotency_key" => Deserialize::begin(&mut self.builder.idempotency_key),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(id), Some(idempotency_key)) =
                (self.builder.id.take(), self.builder.idempotency_key.take())
            else {
                return Ok(());
            };
            *self.out = Some(NotificationEventRequest { id, idempotency_key });
            Ok(())
        }
    }
};

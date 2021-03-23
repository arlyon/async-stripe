use crate::{resources::SetupIntent, Expandable, PaymentMethod};

impl SetupIntent {
    pub async fn confirm(&self, payment_method: Option<Expandable<PaymentMethod>>) -> SetupIntent {
        unimplemented!()
    }

    pub async fn cancel(
        &self,
        cancellation_reason: Option<SetupIntentCancellationReason>,
    ) -> SetupIntent {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {}

# Examples
This folder contains examples showing how to use `async-stripe`. Each example
is setup as its own crate so the dependencies are clear.

The examples include:
- `endpoints`: General usage, including API calls for commonly used resources, along
    with examples of using different request strategies.
- `pagination`: Async streaming of Stripe API calls which allow [Pagination](https://stripe.com/docs/api/pagination)
- `webhook-actix`: Receiving Stripe webhooks using the web framework `actix-web`
- `webhook-rocket`: Receiving Stripe webhooks using the web framework `rocket`
- `webhook-axum`: Receiving Stripe webhooks using the web framework `axum`
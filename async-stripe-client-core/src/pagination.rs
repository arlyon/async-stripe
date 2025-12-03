use std::collections::VecDeque;

use miniserde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use stripe_types::{AsCursorOpt, List, Object, SearchList};

use crate::{RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod};

/// A trait allowing `List<T>` and `SearchList<T>` to be treated the same. Not part of the
/// public API.
///
/// NB: this trait is designed specifically for `List` and `SearchList` and may not be sensible
/// in other cases. One gotcha is that `into_parts` and `from_parts` do not necessarily
/// round-trip, e.g. `SearchList<T>` will lose the `next_page` field since that
/// is not part of the shared list impl. We account for this by ensuring to call `update_params`
/// before breaking the `SearchList` into pieces.
#[doc(hidden)]
pub trait PaginableList: Deserialize {
    /// Underlying single element type, e.g. `Account`
    type Data;

    /// Break into the shared parts list pagination requires
    fn into_parts(self) -> ListParts<Self::Data>;

    /// Reconstruct from the shared parts list pagination requires
    fn from_parts(parts: ListParts<Self::Data>) -> Self;

    /// Update the current parameter set, with `self` as the most
    /// recently fetched page.
    ///
    /// NB: this should also set `has_more` to `false` explicitly if we don't have a new cursor.
    /// (This seems redundant with `has_more` but is used to provide extra protection
    /// against any possibility where `has_more` is `true`, but the cursor is back to `None`,
    /// potentially leading to an infinite pagination loop).
    fn update_params(&mut self, params: &mut Value);
}

/// Specific list parts relied on by the client for pagination.
#[doc(hidden)]
#[derive(Debug)]
pub struct ListParts<T> {
    pub total_count: Option<u64>,
    pub url: String,
    pub data: VecDeque<T>,
    pub has_more: bool,
}

impl<T> PaginableList for List<T>
where
    T: Object,
    List<T>: Deserialize,
{
    type Data = T;

    fn into_parts(self) -> ListParts<Self::Data> {
        ListParts {
            // total_count is not present in `List`, but still present in `ListParts` because the stripe API
            // claims search pagination can still include `total_count` if requested
            total_count: None,
            url: self.url,
            data: VecDeque::from(self.data),
            has_more: self.has_more,
        }
    }

    fn from_parts(parts: ListParts<Self::Data>) -> Self {
        Self { data: Vec::from(parts.data), has_more: parts.has_more, url: parts.url }
    }

    fn update_params(&mut self, params: &mut Value) {
        if let Some(new_cursor) = self.data.last().and_then(|l| l.id().as_cursor_opt()) {
            params["starting_after"] = Value::String(new_cursor.into());
        } else {
            self.has_more = false;
        }
    }
}

impl<T> PaginableList for SearchList<T>
where
    SearchList<T>: Deserialize,
{
    type Data = T;

    /// NB: here we lose `next_page`, so we should be sure to `update_params`
    /// before calling this.
    fn into_parts(self) -> ListParts<Self::Data> {
        ListParts {
            total_count: self.total_count,
            url: self.url,
            data: VecDeque::from(self.data),
            has_more: self.has_more,
        }
    }

    fn from_parts(parts: ListParts<Self::Data>) -> Self {
        Self {
            url: parts.url,
            has_more: parts.has_more,
            data: Vec::from(parts.data),
            next_page: None,
            total_count: parts.total_count,
        }
    }

    fn update_params(&mut self, params: &mut Value) {
        if let Some(next_page) = self.next_page.take() {
            params["page"] = Value::String(next_page);
        } else {
            self.has_more = false;
        }
    }
}

/// An extension trait to allow converting `List<T>` and `SearchList<T>` into
/// a type that can be paginated. Not meant to be implemented by any other types.
pub trait PaginationExt {
    /// The underlying pagination type, e.g. `List<T>` or `SearchList<T>`.
    type Data;

    /// Use the current page state to construct an adaptor capable of paginating
    /// from where the current data left off.
    fn into_paginator(self) -> ListPaginator<Self::Data>;
}

impl<T> PaginationExt for List<T>
where
    T: Sync + Send + 'static,
    List<T>: PaginableList,
{
    type Data = List<T>;

    fn into_paginator(mut self) -> ListPaginator<List<T>> {
        let mut params = Default::default();
        self.update_params(&mut params);
        ListPaginator { page: self, params }
    }
}

impl<T> PaginationExt for SearchList<T>
where
    T: Sync + Send + 'static,
    SearchList<T>: PaginableList,
{
    type Data = SearchList<T>;

    fn into_paginator(mut self) -> ListPaginator<SearchList<T>> {
        let mut params = Default::default();
        self.update_params(&mut params);
        ListPaginator { page: self, params }
    }
}

/// Stream designed to support pagination.
#[derive(Debug)]
pub struct ListPaginator<T> {
    page: T,
    params: Value,
}

impl<T> ListPaginator<SearchList<T>> {
    /// Kept public so that the generated code crates can access this trait. Used by `Search*` params
    /// to implement `PaginationExt`. Not part of the public API.
    #[doc(hidden)]
    pub fn new_search_list(url: impl Into<String>, params: impl Serialize) -> Self {
        let page = SearchList {
            url: url.into(),
            has_more: true,
            data: vec![],
            next_page: None,
            total_count: None,
        };
        Self {
            page,
            params: serde_json::to_value(params)
                // Expect should be safe since we control which types call this
                .expect("all Stripe types implement `Serialize` infallibly"),
        }
    }
}

impl<T> ListPaginator<List<T>> {
    /// Kept public so that the generated code crates can access this trait. Used by `List*` params
    /// to implement `PaginationExt`. Not part of the public API.
    #[doc(hidden)]
    pub fn new_list(url: impl Into<String>, params: impl Serialize) -> Self {
        let page = List { data: vec![], has_more: true, url: url.into() };
        Self {
            page,
            params: serde_json::to_value(params)
                .expect("all Stripe types implement `Serialize` infallibly"),
        }
    }
}

fn req_builder(url: &str) -> RequestBuilder {
    RequestBuilder::new(StripeMethod::Get, url.trim_start_matches("/v1"))
}

impl<T> ListPaginator<T>
where
    T: Sync + Send + 'static + PaginableList,
{
    /// Repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// Stripe's default page size.
    ///
    /// # Errors
    /// If any pagination request returns an error.
    pub fn get_all<C: StripeBlockingClient>(self, client: &C) -> Result<Vec<T::Data>, C::Err> {
        let mut data = VecDeque::new();
        let mut parts = self.page.into_parts();
        let mut params = self.params;
        loop {
            // `append` empties `parts.data`
            data.append(&mut parts.data);

            if !parts.has_more {
                break;
            }

            let req = req_builder(&parts.url).query(&params);
            let mut next_page: T = req.customize().send_blocking(client)?;
            next_page.update_params(&mut params);
            parts = next_page.into_parts();
        }
        Ok(Vec::from(data))
    }

    /// Get all values in this List, consuming self and lazily paginating until all values are fetched.
    ///
    /// This function repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// the page size specified in params, or Stripe's default page size if none is specified.
    pub fn stream<C: StripeClient + Clone>(
        self,
        client: &C,
    ) -> impl futures_util::Stream<Item = Result<T::Data, C::Err>> + Unpin {
        // Using VecDeque, we can pop from the front without needing to reverse
        Box::pin(futures_util::stream::unfold(Some((self, client.clone())), Self::unfold_stream))
    }

    /// Unfold a single item from the stream.
    async fn unfold_stream<C: StripeClient + Clone>(
        state: Option<(Self, C)>,
    ) -> Option<(Result<T::Data, C::Err>, Option<(Self, C)>)> {
        let (paginator, client) = state?; // If none, our last request was an error, so stop here
        let mut parts = paginator.page.into_parts();
        if let Some(next_val) = parts.data.pop_front() {
            // We have more data on this page
            return Some((
                Ok(next_val),
                Some((Self { page: T::from_parts(parts), params: paginator.params }, client)),
            ));
        }

        // Final value of the stream, no errors
        if !parts.has_more {
            return None;
        }

        let req = req_builder(&parts.url).query(&paginator.params);
        match req.customize::<T>().send(&client).await {
            Ok(mut next_page) => {
                let mut params = paginator.params;
                next_page.update_params(&mut params);

                let mut parts = next_page.into_parts();

                let next_val = parts.data.pop_front()?;

                // Yield last value of this page, the next page (and client) becomes the state
                Some((Ok(next_val), Some((Self { page: T::from_parts(parts), params }, client))))
            }
            Err(err) => Some((Err(err), None)), // We ran into an error. The last value of the stream will be the error.
        }
    }
}

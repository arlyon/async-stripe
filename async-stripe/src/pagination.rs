// Necessary under tokio-blocking since `Response` is a type alias to a `Result`
#![allow(clippy::missing_errors_doc)]
use serde::Serialize;
use stripe_types::{List, PaginableList, SearchList};

use crate::Client;

#[derive(Debug)]
pub struct ListPaginator<T> {
    page: T,
    params: serde_json::Value,
}

pub trait PaginationExt {
    type Data;

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

impl<T> ListPaginator<SearchList<T>> {
    /// Kept public so that the generated code crates can access this trait. Used by `Search*` params
    /// to implement `PaginationExt`. Not part of the public API.
    #[doc(hidden)]
    pub fn from_search_params(url: &str, params: impl Serialize) -> Self {
        let page = SearchList {
            url: url.to_string(),
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
    pub fn from_list_params(url: &str, params: impl Serialize) -> Self {
        let page = List { data: vec![], has_more: true, total_count: None, url: url.to_string() };
        Self {
            page,
            params: serde_json::to_value(params)
                .expect("all Stripe types implement `Serialize` infallibly"),
        }
    }
}

impl<T> ListPaginator<T>
where
    T: PaginableList + Send + Sync + 'static,
{
    /// Repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// Stripe's default page size.
    ///
    /// Requires `feature = "blocking"`.
    #[cfg(feature = "blocking")]
    pub fn get_all(self, client: &Client) -> crate::Response<Vec<T::Data>> {
        let mut data = vec![];
        let mut parts = self.page.into_parts();
        let mut params = self.params;
        loop {
            // `append` empties `parts.data`
            data.append(&mut parts.data);

            if !parts.has_more {
                break;
            }

            let mut next_page: T =
                client.get_query(parts.url.trim_start_matches("/v1/"), &params)?;
            next_page.update_params(&mut params);
            parts = next_page.into_parts();
        }
        Ok(data)
    }

    /// Get all values in this List, consuming self and lazily paginating until all values are fetched.
    ///
    /// This function repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// the page size specified in params, or Stripe's default page size if none is specified.
    ///
    ///
    /// Requires `feature = ["async", "stream"]`.
    #[cfg(all(feature = "async", feature = "stream"))]
    pub fn stream(
        self,
        client: &Client,
    ) -> impl futures_util::Stream<Item = Result<T::Data, crate::StripeError>> + Unpin {
        // We are going to be popping items off the end of the list, so we need to reverse it.
        let mut page = self.page.into_parts();
        page.data.reverse();
        let paginator = ListPaginator { page: T::from_parts(page), params: self.params };

        Box::pin(futures_util::stream::unfold(
            Some((paginator, client.clone())),
            Self::unfold_stream,
        ))
    }

    /// unfold a single item from the stream
    #[cfg(all(feature = "async", feature = "stream"))]
    async fn unfold_stream(
        state: Option<(Self, Client)>,
    ) -> Option<(Result<T::Data, crate::StripeError>, Option<(Self, Client)>)> {
        let (paginator, client) = state?; // If none, our last request was an error, so stop here
        let mut parts = paginator.page.into_parts();
        if let Some(next_val) = parts.data.pop() {
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

        match client
            .get_query::<T, _>(parts.url.trim_start_matches("/v1/"), &paginator.params)
            .await
        {
            Ok(mut next_page) => {
                let mut params = paginator.params;
                next_page.update_params(&mut params);

                let mut parts = next_page.into_parts();

                // We are going to be popping items off the end of the list, so we need to reverse it.
                parts.data.reverse();

                let next_val = parts.data.pop()?;

                // Yield last value of this page, the next page (and client) becomes the state
                Some((Ok(next_val), Some((Self { page: T::from_parts(parts), params }, client))))
            }
            Err(err) => Some((Err(err), None)), // We ran into an error. The last value of the stream will be the error.
        }
    }
}

use serde::de::DeserializeOwned;
use serde::Serialize;
use stripe_types::{List, Object};

use crate::{Client, Response};

pub trait PaginationParams: Serialize {}

#[derive(Debug)]
pub struct ListPaginator<T> {
    data: Vec<T>,
    url: String,
    has_more: bool,
    total_count: Option<usize>,
    params: serde_json::Value,
}

pub trait PaginationExt<T> {
    fn into_paginator(self) -> ListPaginator<T>;
}

impl<T> PaginationExt<T> for List<T>
where
    T: Object + DeserializeOwned + Send + Sync + 'static,
    T::Id: ToString,
{
    fn into_paginator(self) -> ListPaginator<T> {
        ListPaginator::new_from_list(self)
    }
}

impl<T> ListPaginator<T> {
    pub fn from_params<U: ToString>(url: U, params: impl PaginationParams) -> Self {
        ListPaginator {
            data: vec![],
            url: url.to_string(),
            has_more: true,
            total_count: None,
            params: serde_json::to_value(params).unwrap(),
        }
    }
}

impl<T> ListPaginator<T>
where
    T: Object + DeserializeOwned + Send + Sync + 'static,
    T::Id: ToString,
{
    /// Repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// Stripe's default page size.
    ///
    /// Requires `feature = "blocking"`.
    #[cfg(feature = "blocking")]
    pub fn get_all(self, client: &Client) -> Response<Vec<T>> {
        let mut data = Vec::with_capacity(self.total_count.unwrap_or(0));
        let mut paginator = self;
        loop {
            if !paginator.has_more {
                data.extend(paginator.data.into_iter());
                break;
            }
            let next_page = paginator.fetch_page_with_curr_params(client)?;
            paginator.update_with_new_data(next_page);
            if let Some(last) = paginator.data.last() {
                paginator.update_cursor(last.id().to_string());
            }
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
        mut self,
        client: &Client,
    ) -> impl futures_util::Stream<Item = Result<T, crate::StripeError>> + Unpin {
        // We are going to be popping items off the end of the list, so we need to reverse it.
        self.data.reverse();

        Box::pin(futures_util::stream::unfold(Some((self, client.clone())), Self::unfold_stream))
    }

    /// unfold a single item from the stream
    #[cfg(all(feature = "async", feature = "stream"))]
    async fn unfold_stream(
        state: Option<(Self, Client)>,
    ) -> Option<(Result<T, crate::StripeError>, Option<(Self, Client)>)> {
        let (mut paginator, client) = state?; // If none, we sent the last item in the last iteration

        if let Some(next_val) = paginator.data.pop() {
            // We have more data on this page
            return Some((Ok(next_val), Some((paginator, client))));
        }

        // Final value of the stream, no errors
        if !paginator.has_more {
            return None;
        }

        match paginator.fetch_page_with_curr_params(&client).await {
            Ok(mut next_page) => {
                // NB: we get the next cursor _before_ reversing the data
                let new_cursor = next_page.data.last().map(|t| t.id().to_string());

                // We are going to be popping items off the end of the list, so we need to reverse it.
                next_page.data.reverse();
                paginator.update_with_new_data(next_page);

                let next_val = paginator.data.pop()?;

                if let Some(new_cursor) = new_cursor {
                    paginator.update_cursor(new_cursor);
                }

                // Yield last value of this page, the next page (and client) becomes the state
                Some((Ok(next_val), Some((paginator, client))))
            }
            Err(e) => Some((Err(e), None)), // We ran into an error. The last value of the stream will be the error.
        }
    }

    fn fetch_page_with_curr_params(&self, client: &Client) -> Response<List<T>> {
        client.get_query(&self.url, &self.params)
    }

    fn new_from_list(list: List<T>) -> Self {
        let mut paginator = Self {
            data: list.data,
            // the url we get back is prefixed
            url: list.url.trim_start_matches("/v1/").to_string(),
            has_more: list.has_more,
            total_count: list.total_count.map(|t| t as usize),
            params: Default::default(),
        };
        if let Some(last) = &paginator.data.last() {
            let new_cursor = last.id().to_string();
            paginator.update_cursor(new_cursor);
        }
        paginator
    }

    fn update_cursor(&mut self, id: String) {
        self.params["starting_after"] = serde_json::Value::String(id);
    }

    fn update_with_new_data(&mut self, list: List<T>) {
        if let Some(last) = &list.data.last() {
            let new_cursor = last.id().to_string();
            self.update_cursor(new_cursor);
        }
        self.data.extend(list.data);
        self.has_more = list.has_more;
        self.total_count = list.total_count.map(|t| t as usize);
    }
}

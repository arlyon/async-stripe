use futures_util::StreamExt;
use futures_util::TryStreamExt;
use stripe::PaginationExt;
use stripe::{AccountId, Client};
use stripe_connect::account::ListAccount;
use stripe_core::customer::{ListCustomer, SearchCustomer};
use stripe_core::{Customer, CustomerId};

use crate::mock::get_client;
use crate::pagination_utils::{cons_cus_id, parse_cus_id, PaginationMock, PaginationMockKind};

const PAGINATION_KINDS: [PaginationMockKind; 2] =
    [PaginationMockKind::List, PaginationMockKind::Search];

#[tokio::test]
async fn is_account_listable() {
    let client = get_client();
    let expected_id: AccountId = "acct_1OPouMJN5vQBdWEx".parse().unwrap();

    // Paginating from nothing
    let result =
        ListAccount::new().paginate().stream(&client).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result.first().unwrap().id, expected_id);

    // Should be same result making a single request, then paginating that returned list since there's no
    // additional data
    let result = ListAccount::new().send(&client).await.unwrap();
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data.first().unwrap().id, expected_id);

    let result = result.into_paginator().stream(&client).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result.first().unwrap().id, expected_id);
}

#[tokio::test]
async fn is_customer_searchable() {
    let client = get_client();
    let expected_id: CustomerId = "cus_PEHTtYpY7elppN".parse().unwrap();

    // Paginating from nothing
    let result = SearchCustomer::new("unused_query")
        .paginate()
        .stream(&client)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result.first().unwrap().id, expected_id);

    // Should be same result making a single request, then paginating that returned list since there's no
    // additional data
    let result = SearchCustomer::new("unused_query").send(&client).await.unwrap();
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data.first().unwrap().id, expected_id);

    let result = result.into_paginator().stream(&client).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result.first().unwrap().id, expected_id);
}

/// `PaginationMock` docs best explain how the mock pagination works. This function ensures
/// that we both fetched the correct data and made the correct series of API calls when paginating
/// all data from `initial_cursor` onward.
///
// FIXME: add `track_caller` once that's supported for async on stable so easier to see any
// failing cases
async fn check_get_all(
    kind: PaginationMockKind,
    customer_count: usize,
    limit: Option<i64>,
    initial_cursor: Option<usize>,
    expected_cursors: Vec<Option<usize>>,
) {
    let mocker = PaginationMock::new(customer_count, kind).await;
    let client = Client::from_url(&*mocker.url(), "fake_key");
    let initial_cursor_str = initial_cursor.map(cons_cus_id);
    let items: Vec<Customer> = match kind {
        PaginationMockKind::List => {
            let params = ListCustomer {
                limit,
                starting_after: initial_cursor_str.as_deref(),
                ..Default::default()
            };
            params.paginate().stream(&client).try_collect().await.unwrap()
        }
        PaginationMockKind::Search => {
            let params = SearchCustomer {
                limit,
                page: initial_cursor_str.as_deref(),
                query: "unused",
                expand: None,
            };
            params.paginate().stream(&client).try_collect().await.unwrap()
        }
    };

    mocker.assert_cursors_received(&expected_cursors).await;
    let all_ids_received = items.into_iter().map(|c| c.id.to_string()).collect::<Vec<_>>();
    let expected_ids = if let Some(start_cursor) = initial_cursor {
        mocker.all_ids_after(start_cursor)
    } else {
        mocker.all_ids()
    };
    assert_eq!(all_ids_received, expected_ids);
}

/// `PaginationMock` docs best explain how the mock pagination works. This function ensures
/// that we both fetched the correct data and made the correct series of API calls when paginating
/// any amount of data from `initial_cursor` onward.
///
// FIXME: add `track_caller` once that's supported for async on stable so easier to see any
// failing cases
async fn check_partial(
    kind: PaginationMockKind,
    count_to_get: usize,
    initial_cursor: Option<usize>,
    expected_cursors: Vec<Option<usize>>,
    expected_ids_received: Vec<usize>,
) {
    let mocker = PaginationMock::new(10, kind).await;
    let client = Client::from_url(&*mocker.url(), "fake_key");
    let initial_cursor_str = initial_cursor.map(cons_cus_id);
    let items: Vec<Customer> = match kind {
        PaginationMockKind::List => {
            let params = ListCustomer {
                limit: Some(5),
                starting_after: initial_cursor_str.as_deref(),
                ..Default::default()
            };
            params.paginate().stream(&client).take(count_to_get).try_collect().await.unwrap()
        }
        PaginationMockKind::Search => {
            let params = SearchCustomer {
                limit: Some(5),
                page: initial_cursor_str.as_deref(),
                query: "unused",
                expand: None,
            };
            params.paginate().stream(&client).take(count_to_get).try_collect().await.unwrap()
        }
    };

    mocker.assert_cursors_received(&expected_cursors).await;
    let all_ids_received =
        items.into_iter().map(|c| parse_cus_id(c.id.as_str())).collect::<Vec<_>>();
    assert_eq!(all_ids_received, expected_ids_received);
}

#[tokio::test]
async fn pagination_get_all() {
    for kind in PAGINATION_KINDS {
        check_get_all(kind, 0, None, None, vec![None]).await;
        check_get_all(kind, 2, None, None, vec![None]).await;
        check_get_all(kind, 2, Some(1), None, vec![None, Some(1)]).await;
        check_get_all(kind, 10, Some(3), None, vec![None, Some(3), Some(6), Some(9)]).await;
    }
}

#[tokio::test]
async fn pagination_starting_not_at_beginning() {
    for kind in PAGINATION_KINDS {
        check_get_all(kind, 2, None, Some(1), vec![Some(1)]).await;
        check_get_all(kind, 10, Some(3), Some(4), vec![Some(4), Some(7), Some(10)]).await;
    }
}

#[tokio::test]
async fn partial_pagination() {
    for kind in PAGINATION_KINDS {
        check_partial(kind, 2, None, vec![None], vec![1, 2]).await;
        check_partial(kind, 2, Some(2), vec![Some(2)], vec![3, 4]).await;
        check_partial(kind, 7, Some(1), vec![Some(1), Some(6)], vec![2, 3, 4, 5, 6, 7, 8]).await;
    }
}

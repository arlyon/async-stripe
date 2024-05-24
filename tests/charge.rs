mod mock;

#[test]
#[cfg(feature = "blocking")]
fn is_charge_retrievable() {
    mock::with_client(|client| {
        let id = "ch_123".parse().unwrap();
        let result = stripe::Charge::retrieve(client, &id, &[]);
        let charge = match result {
            Err(err) => panic!("{}", err),
            Ok(ok) => ok,
        };
        assert_eq!(charge.id, "ch_123");
        if let Some(cus) = charge.customer {
            assert!(!cus.is_object());
        }
        if let Some(inv) = charge.invoice {
            assert!(!inv.is_object());
        }
    });
}

#[test]
#[cfg(feature = "blocking")]
fn is_charge_expandable() {
    mock::with_client(|client| {
        let id = "ch_123".parse().unwrap();
        let result = stripe::Charge::retrieve(client, &id, &[]);
        let charge = match result {
            Err(err) => panic!("{}", err),
            Ok(ok) => ok,
        };
        assert_eq!(charge.id, "ch_123");
        if let Some(cus) = charge.customer {
            assert!(cus.is_object());
        }
        if let Some(inv) = charge.invoice {
            assert!(inv.is_object());
        }
    });
}

/*
 Use this test to fix deserialization
 issues. Just drop some JSON in a file
 then point to that file,
 then specify `stripe::Struct` where Struct is
 what you're trying to deserialize to.

 In case of untagged enum, like BalanceTransaction.source try commenting
 out possible problem keys, like `invoice` to find out
 if that was the problem key, then isolate
 that JSON and change to trying to debug that
 data structure
*/
#[test]
#[cfg(feature = "blocking")]
fn try_charge() {
    use std::fs;
    use std::path::Path;

    // let path_str = "./files/problem-trx.json";
    let path_str = "./files/problem-invoice.json";
    let path = Path::new(path_str);
    let bytes = fs::read(path).expect("should have read file");
    let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);

    // try deserialize
    // let result: Result<stripe::Invoice, serde_path_to_error::Error<serde_json::Error>> =
    //     serde_path_to_error::deserialize(json_deserializer);

    match result {
        Ok(de) => Ok(de),
        Err(e) => {

            println!("error {:?}", e);

            // UNCOMMENT THIS FOR DEBUGGING AROUND THE DESERIALIZATION ISSUE
            // Convert bytes to string for printing. Assuming the response is UTF-8 encoded.
            let response_string = String::from_utf8(bytes.clone().to_vec()) // Clones bytes to keep for JSON parsing
                .expect("Response was not valid UTF-8"); // Handles potential UTF-8 conversion error

            // Extract line and column from the error
            let error_line = e.inner().line(); // 1-based index of the error line
                                               // let error_column = e.inner().column(); // 1-based index of the error column
                                               // Split the response into lines and find the error line
            let lines: Vec<&str> = response_string.lines().collect();
            if error_line > 0 && error_line <= lines.len() {
                // Calculate range for context lines
                let start_line = (error_line as isize - 1000).max(0) as usize; // 10 lines before the error
                let end_line = std::cmp::min(lines.len(), error_line + 10); // 10 lines after the error

                // Collect context lines
                let context_lines = &lines[start_line..end_line];

                println!("Context around deserialization error:");
                for (i, line) in context_lines.iter().enumerate() {
                    println!("{:4}: {}", start_line + i + 1, line); // Print line numbers with context
                }
            } else {
                println!("Error line out of bounds."); // In case the error line is not in the range
            }

            Err(stripe::StripeError::from(e))
        }
    };
}

use regex::Regex;

fn replace_ids_in_url(url: &str) -> String {
    // UUID pattern (e.g., fa77c3e6-0514-465b-9962-320643a3ac97)
    let uuid_regex = Regex::new(r"[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}").unwrap();
    
    // Numeric ID pattern (e.g., 1234, 567)
    let numeric_regex = Regex::new(r"\b\d{3,}\b").unwrap();
    
    // Alphanumeric ID pattern (e.g., 2lbN-4Dr, ABC123DEF)
    let alphanumeric_regex = Regex::new(r"\b[a-zA-Z0-9]{7,}\b").unwrap();

    let mut replaced_url = url.to_string();

    // Replace UUIDs with __ID__
    replaced_url = uuid_regex.replace_all(&replaced_url, "__ID__").to_string();
    
    // Replace Numeric IDs with __ID__
    replaced_url = numeric_regex.replace_all(&replaced_url, "__ID__").to_string();
    
    // Replace Alphanumeric IDs with __ID__
    replaced_url = alphanumeric_regex.replace_all(&replaced_url, "__ID__").to_string();

    // Handle multiple occurrences
    if replaced_url.matches("__ID__").count() > 1 {
        replaced_url = replaced_url.replace("__ID__", "__IDs__");
    }

    replaced_url
}

fn main() {
    let test_cases = vec![
        ("/ping/fa77c3e6-0514-465b-9962-320643a3ac97", "/ping/__ID__"),
        ("/workspaces/ws-1406ef2f-5758-4ebd-8c0e-bf2f9f5a1952/api/v1/status/buildinfo", "/workspaces/__ID__/api/v1/status/buildinfo"),
        ("/exec/o9JLMK1", "/exec/__ID__"),
        ("/exec/2lbN-4Dr", "/exec/__ID__"),
        ("/exec/1seRxK0t", "/exec/__ID__"),
        ("/product-categories/[3B845d68-d5a9-4f19-aac6-47b8f5fde632]", "/product-categories/__IDs__"),
        ("/store_items2/_doc/01RNK09D9XM", "/store_items2/_doc/__ID__"),
        ("/store_items2/_doc/02DGL9W3WA", "/store_items2/_doc/__ID__"),
        ("/upstreams/183d46f8-04dc-44dc-8d66-f1d8805c42cf", "/upstreams/__ID__"),
        ("/v1/availability/XFRS39N80W", "/v1/availability/__ID__"),
        ("/v1/skus/01S1MJP9M,47U1XZNNUW0,LYM6X3NBJX,V3ZB91G6DWJ,CDQ456G7JM,3GPCUM6CM1,2D08R6ROS,Q1BL7M7X65,1U06QJX1NB,K3YQDB8J5DJ,J2Z9H367HJ,ZU45H4Y27F,04KFFWO9P8T,8HNU3AQ3NY,B8GLJ7R0RK,AZL63VH2MC", "/v1/skus/__IDs__"),
    ];

    for (input, expected) in test_cases {
        let result = replace_ids_in_url(input);
        assert_eq!(result, expected, "Failed for URL: {}", input);
    }

    println!("All test cases passed!");
}

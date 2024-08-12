use regex::Regex;

fn replace_ids(url: &str) -> String {
    // Regular expression to match full segments with a prefix followed by a UUID
    let full_segment_regex = Regex::new(r"\b\w+-[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\b").unwrap();
    
    // Regular expression to match UUIDs or similar patterns
    let uuid_regex = Regex::new(r"\b[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\b").unwrap();
    
    // Regular expression to match IDs inside square brackets
    let bracketed_id_regex = Regex::new(r"\[([a-fA-F0-9-]{36})\]").unwrap();

    // Regular expression to match alphanumeric IDs starting with a capital letter or digit, allowing hyphens, at least 6 characters long
    let alphanumeric_regex = Regex::new(r"\b([A-Z0-9]{1}[a-zA-Z0-9-]{5,})\b").unwrap();

    // Step 1: Replacing IDs inside square brackets with __ID__ (removing the brackets)
    let processed_url = bracketed_id_regex.replace_all(url, "__ID__").to_string();

    // Step 2: Replacing full segments with a prefix and UUID with __ID__
    let processed_url = full_segment_regex.replace_all(&processed_url, "__ID__").to_string();

    // Step 3: Replacing standalone UUIDs
    let processed_url = uuid_regex.replace_all(&processed_url, "__ID__").to_string();

    // Step 4: Replacing alphanumeric IDs that start with a capital letter or digit and are at least 6 characters long
    let processed_url = alphanumeric_regex.replace_all(&processed_url, |caps: &regex::Captures| {
        let matched = &caps[0];
        if matched.contains('-') || matched.len() >= 6 {
            "__ID__".to_string()
        } else {
            matched.to_string()
        }
    }).to_string();

    // Step 5: Handle cases with multiple IDs in a segment (e.g., comma-separated IDs)
    let processed_url = if processed_url.contains(",") {
        let multiple_ids_regex = Regex::new(r"(__ID__(,)?)+").unwrap();
        multiple_ids_regex.replace_all(&processed_url, "__IDs__").to_string()
    } else {
        processed_url
    };

    processed_url
}

fn main() {
    let test_cases = vec![
        "/ping/fa77c3e6-0514-465b-9962-320643a3ac97",
        "/workspaces/ws-1406ef2f-5758-4ebd-8c0e-bf2f9f5a1952/api/v1/status/buildinfo",
        "/exec/9jLMK1",
        "/exec/21Bn-4Dr",
        "/exec/1seRxK0t",
        "/product-categories/[33845d68-d5a9-4f19-aac6-47b8f5fde632]",
        "/store_items2/_doc/01RNk09D9XM",
        "/store_items2/_doc/02DGL9W3WA",
        "/upstreams/183d46f8-04dc-44dc-8d66-f1d8085c42cf",
        "/v1/availability/XFRS39N80W",
        "/v1/skus/01S1MJP9M,47U1ZXNNUW0,LMY6X3NBJX,V3ZB91O6DWJ,CDQ456G7JM,3GPCUM6CM1,2D08R6ROS,Q1BL7MTX65,1UO6JXJNL8,K3YQVDB8SJD,JZ29H36H7J,ZU4SH4Y27F,04KFFWO9P8,8HNU3AQ3NY,B8GL7J0RKK,AZL63VH2MC"
    ];

    for url in test_cases {
        let processed_url = replace_ids(url);
        println!("Original: {}\nProcessed: {}\n", url, processed_url);
    }
}

# Fuzzy Replace IDS in Endpoint URLs

## Overview

This Rust script processes a list of URLs and replaces segments that are identified as IDs (such as UUIDs, alphanumeric IDs, or specific patterns) with `__ID__` or `__IDs__` when there are multiple IDs in a segment. The script handles various ID patterns and considers specific segments as complete IDs, even when they include prefixes or are enclosed in square brackets.

## Features

- **UUID Replacement:** Replaces universally unique identifiers (UUIDs) in URLs.
- **Alphanumeric ID Replacement:** Replaces alphanumeric strings that start with a capital letter or a digit and are at least 6 characters long.
- **Full Segment Matching:** Treats segments like `ws-1406ef2f-5758-4ebd-8c0e-bf2f9f5a1952` as full IDs.
- **Bracketed ID Replacement:** Identifies and replaces IDs enclosed in square brackets, removing the brackets in the process.
- **Multiple ID Handling:** Combines multiple comma-separated IDs into `__IDs__`.

## How to Run the Code

### Prerequisites

- Rust (Make sure you have [Rust installed](https://www.rust-lang.org/tools/install)).

### Steps

1. **Clone the Repository or Create a New Rust Project:**

   ```sh
   git clone <repository_url>
   cd <repository_name>
   ```

   followed by:

   ```sh
   cd fuzzy_replace
   ```

2. **Replace the `main.rs` Content:**

   Copy the code provided above into the `src/main.rs` file of your project.

3. **Build and Run the Project:**

   Use Cargo to build and run the project:

   ```sh
   cargo build
   cargo run
   ```

   This will compile and run the script, displaying the original and processed URLs in the console.

## Approach

### Regular Expressions

The script uses a combination of regular expressions to identify and replace specific patterns in the URLs:

1. **Full Segment Matching with Prefix and UUID:**
   - The first regex identifies segments with prefixes (e.g., `ws-`) followed by a UUID-like pattern. These segments are replaced entirely with `__ID__`.
   
   ```rust
   let full_segment_regex = Regex::new(r"\b\w+-[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\b").unwrap();
   ```

2. **UUID Matching:**
   - The second regex matches standard UUIDs and replaces them with `__ID__`.
   
   ```rust
   let uuid_regex = Regex::new(r"\b[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\b").unwrap();
   ```

3. **Bracketed ID Matching:**
   - The third regex identifies IDs enclosed in square brackets, removes the brackets, and replaces the content with `__ID__`.
   
   ```rust
   let bracketed_id_regex = Regex::new(r"\[([a-fA-F0-9-]{36})\]").unwrap();
   ```

4. **Alphanumeric ID Matching:**
   - The fourth regex captures alphanumeric IDs that start with a capital letter or digit and are at least 6 characters long.
   
   ```rust
   let alphanumeric_regex = Regex::new(r"\b([A-Z0-9]{1}[a-zA-Z0-9-]{5,})\b").unwrap();
   ```

5. **Multiple ID Handling:**
   - Finally, the script checks for segments containing multiple IDs (comma-separated) and replaces them with `__IDs__`.
   
   ```rust
   let multiple_ids_regex = Regex::new(r"(__ID__(,)?)+").unwrap();
   ```

### Execution Flow

1. **Step 1:** The script first replaces any bracketed IDs and removes the brackets.
2. **Step 2:** It then replaces full segments that match the pattern with a prefix and UUID.
3. **Step 3:** Next, it replaces standalone UUIDs.
4. **Step 4:** It replaces alphanumeric IDs that match the defined pattern.
5. **Step 5:** Finally, it handles segments with multiple IDs.

By following this structured approach, the script ensures that all relevant IDs are replaced correctly while preserving other parts of the URL.
# string-manip-benchmark

**Benchmarks for the Rust library string_manipulation_utf8**

This code was used for simple benchmarking on multiple implementations of string manipulation functions, to be used in the Rust library [string_manipulation_utf8](https://github.com/guntherwillems/rust-string-manipulation-utf8). All functions use character indexing instead of bytes. It uses UTF-8 encoded strings as implemented in Rust.

To start the benchmarks:  
`cargo run --release`

To start the unit testing:  
`cargo test`

These benchmarks were tested on multiple processors with multiple values for total_words_u. Functions were retained that were clearly or mostly faster. Indicated as (faster) in the lists.


## indexof

Get the character position from one string into another. Start searching from character index 'start_index'. Returns None if not found. Index of the first character is 0.

`indexof(s: &str, searchstring: &str, start_index: usize) -> Option<usize>`

- indexof:  using loop over chars() (**faster**)
- indexof2: using enumerate
- indexof3: using vectors
- indexof4: loop chars() iterator and count bytes until .find() position
- indexof5: populate a vector with characters as they are used
- indexof6: loop char_indices, character byte positions
- indexof7: loop characters, check from starts_with byte position


## substr

Get a substring of a string, beginning at character index 'start_index' and take 'length' characters.  
Negative numbers count backwards:  
  'start_index' from the end of the string.  
  'length' from 'start_index'.  
If start_index exceeds the string boundary limits, return an empty string.
Index of the first character is 0.

`substr(s: &str, start_index: isize, length: isize) -> String`

- substr:  using chars().skip().take() (**faster**)
- substr2: Using character enumeration
- substr3: using UTF-8 bytes counting with 'for' loop
- substr4: using UTF-8 bytes counting with infinite loop


## str_remove

Remove a substring from a string. Beginning at character index 'start_index' and take 'length' characters. Index of the first character is 0.

`str_remove(s: &str, start_index: usize, length: usize) -> String`

- str_remove:  using 2 char_indices
- str_remove2: add 2 chars() iterators with + operator
- str_remove3: add 2 chars() iterators with str_concat!
- str_remove4: chain() a second character iterator
- str_remove5: counting bytes over chars() iterator (**faster**)


## str_concat

Macro to concatenate multiple strings.  
All strings are borrowed. Allocates the needed capacity and adds the stings.

`str_concat!(&str1, &str2, ...)`

Benchmarks to compare str_concat with standard Rust statements.

- str_concat
- std::format macro
- add with + operator

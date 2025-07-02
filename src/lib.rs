/// Library string_manip_benchmark
/// Author: Gunther Willems
/// Started 11/2024
/// License: MIT
///
/// This code was used for simple benchmarking on multiple implementations of string
/// manipulation functions, to be used in the Rust library [string_manipulation](https://github.com/guntherwillems/rust-string-manipulation).
/// All functions use character indexing instead of bytes. It uses UTF-8 encoded
/// strings as implemented in Rust.
/// Not all functions are fully functional. The code is used to test different algorithms.

/// ---

/// Macro to concatenate multiple strings.
/// Allocates the needed capacity and adds the stings.
/// All strings are borrowed.
/// Example: let s: String = str_concat!(&s1, &s2, &s3);
/// The arguments are string slices (&str). The number of arguments can be 2 or more.
/// Example:
///   fn main() {
///     let s1: String = "string1".to_owned();
///     let s2: String = "string2".to_owned();
///     let s3: String = "string3".to_owned();
///     let result1: String = str_concat!(&s1, &s2, &s3);
///     let result2: String = str_concat!(&s1, "string2", &s3);
///   }
#[macro_export]
macro_rules! str_concat {
        ($($arg:expr),+) => {
            {
                let mut len = 0;
                $(
                    len += $arg.len();
                )*
                let mut result = String::with_capacity(len);
                $(
                    result.push_str($arg);
                )*
                result
            }
        };
    }

// -------------------------------------------------------------------------

/// Get the character position from one string into another. Start searching
/// from character index 'start_index'. Returns None if not found. Index of
/// the first character is 0.
/// Using loop over chars()
pub fn indexof(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if searchstring.is_empty() {
        return None;
    }

    let mut char_index: usize = start_index;
    let search_len: usize = searchstring.chars().count();
    let total_len: usize = s.len();
    let mut match_count: usize;
    let mut next_index: usize;

    // Iterate through the String characters from start_index
    for c in s.chars().skip(start_index) {
        // Relatively fast because searchstring is mostly short
        if c == searchstring.chars().next().unwrap() {
            match_count = 1;

            // sc: char = each subsequent character in searchstring
            for sc in searchstring.chars().skip(1) {
                next_index = char_index + match_count;
                if next_index >= total_len || s.chars().nth(next_index).unwrap() != sc {
                    break;
                }
                match_count += 1;
            }
            // If entire search value matches, return index
            if match_count == search_len {
                return Some(char_index);
            }
        }
        char_index += 1;
    }

    None // No match found
}

// -----------------------------------------------------------------------------

/// Same code as indexof, replacing chars().nth(next_index) with one chars iterator for each searchstring loop.
/// Fastest.
pub fn indexof1a(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if searchstring.is_empty() {
        return None;
    }

    let mut char_index: usize = start_index;
    let search_len: usize = searchstring.chars().count();
    let total_len: usize = s.len();
    let mut match_count: usize;
    let mut next_index: usize;
    let mut self_tmp: std::iter::Skip<std::str::Chars<'_>>;

    // Iterate through the String characters from start_index
    for c in s.chars().skip(start_index) {
        // Relatively fast because searchstring is mostly short
        if c == searchstring.chars().next().unwrap() {
            match_count = 1;

            self_tmp = s.chars().skip(char_index + 1);
            // Check subsequent characters (sc) for a match
            // sc: char = each subsequent character in searchstring
            for sc in searchstring.chars().skip(1) {
                next_index = char_index + match_count;
                // println!("{} - {} {}", self.chars().nth(next_index).unwrap(), j, sc);
                if next_index >= total_len || self_tmp.next().unwrap() != sc {
                    break;
                }
                match_count += 1;
            }
            // If entire search value matches, return index
            if match_count == search_len {
                return Some(char_index);
            }
        }
        char_index += 1;
    }

    // No match found
    None
}

// -----------------------------------------------------------------------------

/// Same code as indexof, replacing chars().nth(next_index) with one chars iterator for each searchstring loop.
/// Using a variable for temporary searchstring chars.
pub fn indexof1b(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if searchstring.is_empty() {
        return None;
    }

    let mut char_index: usize = start_index;
    let search_len: usize = searchstring.chars().count();
    let total_len: usize = s.len();
    let mut match_count: usize;
    let mut next_index: usize;
    let mut self_tmp: std::iter::Skip<std::str::Chars<'_>>;
    let mut search_tmp: std::str::Chars<'_>;

    // Iterate through the String characters from start_index
    for c in s.chars().skip(start_index) {
        search_tmp = searchstring.chars();
        // Relatively fast because searchstring is mostly short
        if c == search_tmp.next().unwrap() {
            match_count = 1;

            self_tmp = s.chars().skip(char_index + 1);
            // Check subsequent characters (sc) for a match
            // sc: char = each subsequent character in searchstring
            for sc in search_tmp {
                next_index = char_index + match_count;
                // println!("{} - {} {}", self.chars().nth(next_index).unwrap(), j, sc);
                if next_index >= total_len || self_tmp.next().unwrap() != sc {
                    break;
                }
                match_count += 1;
            }
            // If entire search value matches, return index
            if match_count == search_len {
                return Some(char_index);
            }
        }
        char_index += 1;
    }

    // No match found
    None
}

// -----------------------------------------------------------------------------

/// Using enumerate
pub fn indexof2(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if searchstring.is_empty() {
        return None;
    }

    let mut char_index: usize = start_index;
    let search_len: usize = searchstring.chars().count();
    let total_len: usize = s.len();

    // Iterate through the String characters from start_index
    // i: usize = index from 0 - end, c: char = each character
    for (i, c) in s.chars().enumerate().skip(start_index) {
        if c == searchstring.chars().next().unwrap() {
            let mut match_count: usize = 1;

            // Check subsequent characters (sc) for a match
            // j: usize = 1 -> end searchstring, sc: char = each subsequent character in searchstring
            for (j, sc) in searchstring.chars().enumerate().skip(1) {
                let next_index: usize = i + j;
                if next_index >= total_len || s.chars().nth(next_index).unwrap() != sc {
                    break;
                }
                match_count += 1;
            }
            // If entire search value matches, return index
            if match_count == search_len {
                return Some(char_index);
            }
        }
        char_index += 1;
    }

    // No match found
    None
}

// -----------------------------------------------------------------------------

// Using vectors
pub fn indexof3(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if searchstring.is_empty() {
        return None;
    }

    // Writing ::<Vec<char>> after collect is optional if annotated
    let string_chars: Vec<char> = s.chars().collect::<Vec<char>>();
    let search_chars: Vec<char> = searchstring.chars().collect::<Vec<char>>();
    let string_len: usize = string_chars.len();
    let search_len: usize = search_chars.len();
    let mut match_count: usize;

    // Iterate through the String characters
    // Check if characters match from start_index
    for i in start_index..string_len {
        match_count = 0;
        loop {
            if string_chars[i + match_count] == search_chars[match_count] {
                match_count += 1;
                // If entire search value matches, return index
                if match_count == search_len {
                    return Some(i);
                }
            } else {
                break;
            }
        }
    }

    // No match found
    None
}

// -------------------------------------------------------------------------

// Loop chars() iterator and count bytes until .find() position
pub fn indexof4(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if searchstring.is_empty() {
        return None;
    }

    let s: String = s.chars().skip(start_index).collect::<String>();
    let found_bytes: Option<usize> = s.find(searchstring);

    if found_bytes.is_none() {
        return None;
    }

    let found_bytes_pos: usize = found_bytes.unwrap();
    let mut index: usize = start_index;
    let mut pos_byte: usize = 0;

    // Count bytes to get the character position
    for c in s.chars() {
        if pos_byte == found_bytes_pos {
            break;
        }

        pos_byte += c.len_utf8();
        index += 1;
    }

    Some(index)
}

// -----------------------------------------------------------------------------

// Populate a vector with characters as they are used
pub fn indexof5(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if s.is_empty() || searchstring.is_empty() {
        return None;
    }

    // Search string is mostly small. Minimal speed penalty.
    let search_chars: Vec<char> = searchstring.chars().collect::<Vec<char>>();
    let search_chars_len: usize = search_chars.len() - 1;
    let mut char_vec: Vec<char> = Vec::with_capacity(s.len()); // For speed, preallocate the maximum size for all character bytes
    let mut string_chars = s.chars().peekable(); // Is at the position where last char was copied to the vector
    let mut char_vec_pos: usize = 0; // Last position index added to the vector + 1. The next empty position.
    let mut pos: usize = 0; // Active position in the string to check. 0 = first character
    let mut c: char = ' '; // Active character
    let mut end_of_search: bool = false;
    let mut sc_pos: usize;
    let mut sc_pos_total: usize;

    while !end_of_search {
        // Skip until start_index
        if pos < start_index {
            // If this is the last character, it's not found
            if string_chars.peek().is_none() {
                return None;
            }
            c = string_chars.next().unwrap();
            char_vec.push(c);
            char_vec_pos += 1; // Set to the next free position
            pos += 1;
            continue;
        }

        // If active position to check is equal to the next free vector position
        if pos == char_vec_pos {
            c = string_chars.next().unwrap();
            char_vec.push(c);
            char_vec_pos += 1; // Set to the next free position
        } else if pos < char_vec_pos {
            c = char_vec[pos];
        }

        sc_pos = 0;
        // Check if searchstring is found at this position
        for sc in search_chars.iter() {
            sc_pos_total = pos + sc_pos;
            if sc_pos_total == char_vec_pos {
                c = string_chars.next().unwrap();
                char_vec.push(c);
                char_vec_pos += 1; // Set to the next free position
            } else if sc_pos_total < char_vec_pos {
                c = char_vec[sc_pos_total];
            }
            if c != *sc {
                break;
            }

            // Position found in String
            if sc_pos == search_chars_len {
                return Some(pos);
            }

            sc_pos += 1;
        }

        pos += 1;

        // End of string
        if string_chars.peek().is_none() {
            end_of_search = true;
        }
    }

    None
}

// -----------------------------------------------------------------------------

// Loop char_indices, character byte positions
pub fn indexof6(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if searchstring.is_empty() {
        return None;
    }

    let mut char_count: usize = 0;

    for (char_index, _) in s.char_indices() {
        // (byte position, char)
        if char_count >= start_index {
            if s[char_index..].starts_with(searchstring) {
                return Some(char_count);
            }
        }
        char_count += 1;
    }

    None
}

// -----------------------------------------------------------------------------

// Loop characters, check from starts_with byte position
pub fn indexof7(s: &str, searchstring: &str, start_index: usize) -> Option<usize> {
    if searchstring.is_empty() {
        return None;
    }

    let mut char_index: usize = 0;
    let mut byte_index: usize = 0;

    for c in s.chars() {
        if char_index >= start_index {
            if s[byte_index..].starts_with(searchstring) {
                return Some(char_index);
            }
        }
        char_index += 1;
        byte_index += c.len_utf8();
    }

    None
}

// -----------------------------------------------------------------------------

/// Get a substring of a string, beginning at character index 'start_index'
/// and take 'length' characters. Index of the first character is 0. Negative
/// numbers count backwards from the end of the string.
/// Using chars().skip().take()
pub fn substr(s: &str, start_index: isize, length: isize) -> String {
    let total_length: usize = s.chars().count();
    let (start, end) = calc_start_end(total_length, start_index, length);

    s.chars().skip(start).take(end - start).collect::<String>()
}

// -----------------------------------------------------------------------------

/// Finding substring using character enumeration
pub fn substr2(s: &str, start_index: isize, length: isize) -> String {
    let mut result = String::new();

    let total_length: usize = s.chars().count();
    let (start, end) = calc_start_end(total_length, start_index, length);

    // i: usize = character index, c: char
    for (i, c) in s.chars().enumerate() {
        if i >= end {
            break;
        }

        if i >= start && i <= end {
            result.push(c);
        }
    }
    result
}

// -------------------------------------------------------------------------

/// Finding substring using UTF-8 bytes counting with for loop
pub fn substr3(s: &str, start_index: isize, length: isize) -> String {
    let mut position: usize = 0;
    let mut start_byte: usize = 0;
    let mut position_byte: usize = 0;

    let total_length: usize = s.chars().count();
    let (start, end) = calc_start_end(total_length, start_index, length);

    if end == 0 {
        return String::new();
    }

    for c in s.chars() {
        if position == start {
            start_byte = position_byte;
        } else if position == end {
            break;
        }

        position += 1;
        position_byte += c.len_utf8();
    }

    s[start_byte..position_byte].to_owned()
}

// -------------------------------------------------------------------------

/// Finding substring using UTF-8 bytes counting with infinite loop.
/// Code not complete for all possible values.
/// Already the slowest solution with this code.
pub fn substr4(s: &str, start_index: isize, length: isize) -> String {
    let mut position: usize = 0;
    let mut start_byte: usize = 0;
    let mut position_byte: usize = 0;
    let mut chars: std::str::Chars = s.chars();
    let mut c: char;

    let total_length: usize = s.chars().count(); // chars.clone().count() is slower
    let (start, end) = calc_start_end(total_length, start_index, length);

    if start > total_length {
        return String::new();
    }

    loop {
        c = chars.next().unwrap();

        if position == start {
            start_byte = position_byte;
        } else if position == end {
            break;
        }

        position += 1;
        position_byte += c.len_utf8();
    }

    s[start_byte..position_byte].to_owned()
}

// -------------------------------------------------------------------------

/// Theoretically calculate the start and end position within a string with a total of length characters.
/// Numbers can be negative to count from the end of the string (start_index) and backwards (length).
/// total_length = total length of the string.
/// start_index = start index (can be negative).
/// length = how many characters to count from the start_index (can be negative).
/// String boundary limits cannot be exceeded.
/// If length is 0, start and end are equal to 0.
/// Returns Tuple (start, end) positions. Result is a range from 'start' index till 'end' index (not included).
#[inline]
fn calc_start_end(total_length: usize, start_index: isize, length: isize) -> (usize, usize) {
    let start: isize;
    let last: isize;
    let total_length: isize = total_length as isize;

    if total_length == 0
        || length == 0
        || start_index < -total_length
        || start_index >= total_length {
        return (0, 0);
    }

    let length: isize = length.clamp(-total_length, total_length);

    // Negative start_index, count backwards from the end
    if start_index >= 0 {
        start = start_index
    } else {
        start = total_length + start_index
    }

    // Negative length, count backwards from start position
    if length > 0 {
        last = (start + length - 1).clamp(0, total_length - 1) // Overflow possible: 2, isize::MAX
    } else {
        last = (start + length + 1).clamp(0, total_length + 1)
    }

    if start > last {
        (last as usize, (start + 1) as usize)
    } else {
        (start as usize, (last + 1) as usize)
    }
}

// -------------------------------------------------------------------------

/// Remove a substring from a string. Beginning at character index 'start_index'
/// and take 'length' characters. Index of the first character is 0.
/// Using char_indices 2 times.
pub fn str_remove(s: &str, start_index: usize, length: usize) -> String {
    let mut s: String = s.to_owned();
    let s_max_pos: usize = s.chars().count() - 1; // last position string

    if length == 0 || start_index > s_max_pos {
        return s;
    }

    let end_index: usize = start_index + length - 1;
    let start_byte: usize = s.char_indices().nth(start_index).unwrap().0; // byte position from char at start_index

    // Character position end_index character
    // Protect against giving an end_index character position > length of the given string 's'
    let end_pos: usize = if end_index > s_max_pos {
        s_max_pos
    } else {
        end_index
    };

    // Indice of char at the end_index position (byte position, char)
    let end_indice: (usize, char) = s.char_indices().nth(end_pos).unwrap();

    // Add byte length last character to position end_index character
    let end_byte: usize = end_indice.0 + end_indice.1.len_utf8();

    s.replace_range(start_byte..end_byte, ""); // Remove the range
    s
}

// -------------------------------------------------------------------------

// Remove characters by adding 2 character iterators with + operator.
pub fn str_remove2(s: &str, start_index: usize, length: usize) -> String {
    s.chars().take(start_index).collect::<String>() 
        + &s.chars().skip(start_index
        + length).collect::<String>()
}

// -------------------------------------------------------------------------

// Remove characters by adding 2 chars() iterators with str_concat!
pub fn str_remove3(s: &str, start_index: usize, length: usize) -> String {
    let s1: String = s.chars().take(start_index).collect::<String>();
    let s2: String = s.chars().skip(start_index + length).collect::<String>();

    str_concat!(&s1, &s2)
}

// -------------------------------------------------------------------------

// Remove characters using take() + chain() a second character iterator.
pub fn str_remove4(s: &str, start_index: usize, length: usize) -> String {
    s.chars()
        .take(start_index)
        .chain(s.chars().skip(start_index + length))
        .collect::<String>()
}

// -----------------------------------------------------------------------------

// Same code as str_remove5, but chars() on string slice instead of String
pub fn str_remove5(s: &str, start_index: usize, length: usize) -> String {
    let mut chars: std::str::Chars = s.chars();
    let mut s: String = s.to_owned();

    if length == 0 {
        return s;
    }

    // Counted in bytes
    let len_byte: usize = s.len();
    let mut start_byte: usize = 0;
    let mut start_byte_found: bool = false;
    let end_byte: usize;
    let mut pos_byte: usize = 0; // character byte position

    // Counted in characters
    let end_index: usize = start_index + length;
    let mut c: char;
    let mut pos: usize = 0; // character position

    loop {
        c = chars.next().unwrap();

        if pos == start_index {
            start_byte = pos_byte;
            start_byte_found = true;
        } else if pos == end_index {
            end_byte = pos_byte;
            break;
        }

        pos += 1;
        pos_byte += c.len_utf8();

        // If end of String is reached
        if pos_byte == len_byte {
            if start_byte_found {
                end_byte = pos_byte;
                break;
            }

            return s;
        }
    }

    s.replace_range(start_byte..end_byte, ""); // Remove the range
    s
}

// -------------------------------------------------------------------------

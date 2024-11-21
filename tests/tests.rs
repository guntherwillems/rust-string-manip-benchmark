use string_manip_benchmark::*;

// -----------------------------------------------------------------------------

#[test]
fn test_indexof() {
    let s1: &str = "Test 123 éèçà 123 test home";
    let searchstring: String = "test".to_owned();

    assert_eq!(indexof(s1, &searchstring, 0), Some(18));
    assert_eq!(indexof(s1, &searchstring, 10), Some(18));
    assert_eq!(indexof(s1, "123", 8), Some(14));
    assert_eq!(indexof(s1, "home", 10), Some(23));
    assert_eq!(indexof(s1, "home", 100), None); // Out of bounds
    assert_eq!(indexof(s1, "", 0), None); // Search nothing
    assert_eq!(indexof(s1, "not found", 0), None); // Not found
}

// -----------------------------------------------------------------------------

#[test]
fn test_indexof2() {
    let s1: &str = "Test 123 éèçà 123 test home";
    let searchstring: String = "test".to_owned();

    assert_eq!(indexof2(s1, &searchstring, 0), Some(18));
    assert_eq!(indexof2(s1, &searchstring, 10), Some(18));
    assert_eq!(indexof2(s1, "123", 8), Some(14));
    assert_eq!(indexof2(s1, "home", 10), Some(23));
    assert_eq!(indexof2(s1, "home", 100), None); // Out of bounds
    assert_eq!(indexof2(s1, "", 0), None); // Search nothing
    assert_eq!(indexof2(s1, "not found", 0), None); // Not found
}

// -----------------------------------------------------------------------------

#[test]
fn test_indexof3() {
    let s1: &str = "Test 123 éèçà 123 test home";
    let searchstring: String = "test".to_owned();

    assert_eq!(indexof3(s1, &searchstring, 0), Some(18));
    assert_eq!(indexof3(s1, &searchstring, 10), Some(18));
    assert_eq!(indexof3(s1, "123", 8), Some(14));
    assert_eq!(indexof3(s1, "home", 10), Some(23));
    assert_eq!(indexof3(s1, "home", 100), None); // Out of bounds
    assert_eq!(indexof3(s1, "", 0), None); // Search nothing
    assert_eq!(indexof3(s1, "not found", 0), None); // Not found
}

// -----------------------------------------------------------------------------

#[test]
fn test_indexof4() {
    let s1: &str = "Test 123 éèçà 123 test home";
    let searchstring: String = "test".to_owned();

    assert_eq!(indexof4(s1, &searchstring, 0), Some(18));
    assert_eq!(indexof4(s1, &searchstring, 10), Some(18));
    assert_eq!(indexof4(s1, "123", 8), Some(14));
    assert_eq!(indexof4(s1, "home", 10), Some(23));
    assert_eq!(indexof4(s1, "home", 100), None); // Out of bounds
    assert_eq!(indexof4(s1, "", 0), None); // Search nothing
    assert_eq!(indexof4(s1, "not found", 0), None); // Not found
}

// -----------------------------------------------------------------------------

#[test]
fn test_indexof5() {
    let s1: &str = "Test 123 éèçà 123 test home";
    let searchstring: String = "test".to_owned();

    assert_eq!(indexof5(s1, &searchstring, 0), Some(18));
    assert_eq!(indexof5(s1, &searchstring, 10), Some(18));
    assert_eq!(indexof5(s1, "123", 8), Some(14));
    assert_eq!(indexof5(s1, "home", 10), Some(23));
    assert_eq!(indexof5(s1, "home", 100), None); // Out of bounds
    assert_eq!(indexof5(s1, "", 0), None); // Search nothing
    assert_eq!(indexof5(s1, "not found", 0), None); // Not found
}

// -----------------------------------------------------------------------------

#[test]
fn test_indexof6() {
    let s1: &str = "Test 123 éèçà 123 test home";
    let searchstring: String = "test".to_owned();

    assert_eq!(indexof6(s1, &searchstring, 0), Some(18));
    assert_eq!(indexof6(s1, &searchstring, 10), Some(18));
    assert_eq!(indexof6(s1, "123", 8), Some(14));
    assert_eq!(indexof6(s1, "home", 10), Some(23));
    assert_eq!(indexof6(s1, "home", 100), None); // Out of bounds
    assert_eq!(indexof6(s1, "", 0), None); // Search nothing
    assert_eq!(indexof6(s1, "not found", 0), None); // Not found
}

// -----------------------------------------------------------------------------

#[test]
fn test_indexof7() {
    let s1: &str = "Test 123 éèçà 123 test home";
    let searchstring: String = "test".to_owned();

    assert_eq!(indexof7(s1, &searchstring, 0), Some(18));
    assert_eq!(indexof7(s1, &searchstring, 10), Some(18));
    assert_eq!(indexof7(s1, "123", 8), Some(14));
    assert_eq!(indexof7(s1, "home", 10), Some(23));
    assert_eq!(indexof7(s1, "home", 100), None); // Out of bounds
    assert_eq!(indexof7(s1, "", 0), None); // Search nothing
    assert_eq!(indexof7(s1, "not found", 0), None); // Not found
}

// -----------------------------------------------------------------------------

#[test]
fn test_remove1() {
    let s1: &str = "Test 123 éèçà 123 test home";

    assert_eq!(str_remove(s1, 14, 4), "Test 123 éèçà test home");
    assert_eq!(str_remove(s1, 3, 100), "Tes"); // Remove all characters from position 3. 100 > number of characters.
    assert_eq!(str_remove(s1, 0, 0), s1); // Remove nothing.
    assert_eq!(str_remove(s1, 500, 2), s1); // Remove characters that don't exist. Out of bounds.
}

// -----------------------------------------------------------------------------

#[test]
fn test_remove2() {
    let s1: &str = "Test 123 éèçà 123 test home";

    assert_eq!(str_remove2(s1, 14, 4), "Test 123 éèçà test home");
    assert_eq!(str_remove2(s1, 3, 100), "Tes"); // Remove all characters from position 3. 100 > number of characters.
    assert_eq!(str_remove2(s1, 0, 0), s1); // Remove nothing.
    assert_eq!(str_remove2(s1, 500, 2), s1); // Remove characters that don't exist. Out of bounds.
}

// -----------------------------------------------------------------------------

#[test]
fn test_remove3() {
    let s1: &str = "Test 123 éèçà 123 test home";

    assert_eq!(str_remove3(s1, 14, 4), "Test 123 éèçà test home");
    assert_eq!(str_remove3(s1, 3, 100), "Tes"); // Remove all characters from position 3. 100 > number of characters.
    assert_eq!(str_remove3(s1, 0, 0), s1); // Remove nothing.
    assert_eq!(str_remove3(s1, 500, 2), s1); // Remove characters that don't exist. Out of bounds.
}

// -----------------------------------------------------------------------------

#[test]
fn test_remove4() {
    let s1: &str = "Test 123 éèçà 123 test home";

    assert_eq!(str_remove4(s1, 14, 4), "Test 123 éèçà test home");
    assert_eq!(str_remove4(s1, 3, 100), "Tes"); // Remove all characters from position 3. 100 > number of characters.
    assert_eq!(str_remove4(s1, 0, 0), s1); // Remove nothing.
    assert_eq!(str_remove4(s1, 500, 2), s1); // Remove characters that don't exist. Out of bounds.
}

// -----------------------------------------------------------------------------

#[test]
fn test_remove5() {
    let s1: &str = "Test 123 éèçà 123 test home";

    assert_eq!(str_remove5(s1, 14, 4), "Test 123 éèçà test home");
    assert_eq!(str_remove5(s1, 3, 100), "Tes"); // Remove all characters from position 3. 100 > number of characters.
    assert_eq!(str_remove5(s1, 0, 0), s1); // Remove nothing.
    assert_eq!(str_remove5(s1, 500, 2), s1); // Remove characters that don't exist. Out of bounds.
}

// -----------------------------------------------------------------------------

#[test]
fn test_substr() {
    let s1: &str = "0123456789";

    assert_eq!(substr(s1, 2, 3), "234");
    assert_eq!(substr(s1, -5, 3), "567");
    assert_eq!(substr(s1, -5, -3), "345");
    assert_eq!(substr(s1, 5, -3), "345");
    assert_eq!(substr(s1, 0, 1), "0");

    assert_eq!(substr(s1, 10, 2), "");
    assert_eq!(substr(s1, 9, 2), "9");
    assert_eq!(substr(s1, 8, 2), "89");

    assert_eq!(substr(s1, -10, 3), "012");
    assert_eq!(substr(s1, 0, 3), "012"); // Same as previous

    // Empty string results
    assert_eq!(substr(s1, 2, 0), ""); // 0 length
    assert_eq!(substr(s1, 0, 0), ""); // 0 length
    assert_eq!(substr(s1, -4, 0), ""); // 0 length

    assert_eq!(substr("", 3, 2), "");
    assert_eq!(substr("", -3, 2), "");
    assert_eq!(substr("", -3, -2), "");

    // Empty string results past boundaries
    assert_eq!(substr("", 100, -100), "");
    assert_eq!(substr("", isize::MAX, isize::MIN), "");

    assert_eq!(substr("", -100, 100), "");
    assert_eq!(substr("", isize::MIN, isize::MAX), "");

    // Past string boundaries
    assert_eq!(substr(s1, -5, -100), "012345");
    assert_eq!(substr(s1, -5, isize::MIN), "012345");

    assert_eq!(substr(s1, -100, 100), "");
    assert_eq!(substr(s1, isize::MIN, isize::MAX), "");

    assert_eq!(substr(s1, -100, 1), "");
    assert_eq!(substr(s1, isize::MIN, 1), "");
    assert_eq!(substr(s1, isize::MIN, 2), "");

    assert_eq!(substr(s1, 5, -100), "012345");
    assert_eq!(substr(s1, 5, isize::MIN), "012345");

    assert_eq!(substr(s1, 2, 100), "23456789");
    assert_eq!(substr(s1, 2, isize::MAX), "23456789");

    assert_eq!(substr(s1, 100, 1), "");
    assert_eq!(substr(s1, isize::MAX, 1), "");

    assert_eq!(substr(s1, 100, -3), "");
    assert_eq!(substr(s1, isize::MAX, -3), "");

    assert_eq!(substr(s1, 100, -100), "");
    assert_eq!(substr(s1, isize::MAX, isize::MIN), "");

    assert_eq!(substr(s1, -100, 100), "");
    assert_eq!(substr(s1, isize::MIN, isize::MAX), "");

    assert_eq!(substr(s1, -100, -3), "");
    assert_eq!(substr(s1, isize::MIN, -3), "");

    // 1 position less than maximum isize values
    assert_eq!(substr(s1, isize::MIN + 1, 5), "");
    assert_eq!(substr(s1, isize::MAX - 1, -3), "");
    assert_eq!(substr(s1, 5, isize::MAX - 1), "56789");
    assert_eq!(substr(s1, 5, isize::MIN + 1), "012345");
    assert_eq!(substr(s1, isize::MAX - 1, isize::MIN + 1), "");
}

// -----------------------------------------------------------------------------

#[test]
fn test_substr2() {
    let s1: &str = "0123456789";

    assert_eq!(substr2(s1, 2, 3), "234");
    assert_eq!(substr2(s1, -5, 3), "567");
    assert_eq!(substr2(s1, -5, -3), "345");
    assert_eq!(substr2(s1, 5, -3), "345");
    assert_eq!(substr2(s1, 0, 1), "0");

    assert_eq!(substr2(s1, 10, 2), "");
    assert_eq!(substr2(s1, 9, 2), "9");
    assert_eq!(substr2(s1, 8, 2), "89");

    assert_eq!(substr2(s1, -10, 3), "012");
    assert_eq!(substr2(s1, 0, 3), "012"); // Same as previous

    // Empty string results
    assert_eq!(substr2(s1, 2, 0), ""); // 0 length
    assert_eq!(substr2(s1, 0, 0), ""); // 0 length
    assert_eq!(substr2(s1, -4, 0), ""); // 0 length

    assert_eq!(substr2("", 3, 2), "");
    assert_eq!(substr2("", -3, 2), "");
    assert_eq!(substr2("", -3, -2), "");

    // Empty string results past boundaries
    assert_eq!(substr2("", 100, -100), "");
    assert_eq!(substr2("", isize::MAX, isize::MIN), "");

    assert_eq!(substr2("", -100, 100), "");
    assert_eq!(substr2("", isize::MIN, isize::MAX), "");

    // Past string boundaries
    assert_eq!(substr2(s1, -5, -100), "012345");
    assert_eq!(substr2(s1, -5, isize::MIN), "012345");

    assert_eq!(substr2(s1, -100, 100), "");
    assert_eq!(substr2(s1, isize::MIN, isize::MAX), "");

    assert_eq!(substr2(s1, -100, 1), "");
    assert_eq!(substr2(s1, isize::MIN, 1), "");
    assert_eq!(substr2(s1, isize::MIN, 2), "");

    assert_eq!(substr2(s1, 5, -100), "012345");
    assert_eq!(substr2(s1, 5, isize::MIN), "012345");

    assert_eq!(substr2(s1, 2, 100), "23456789");
    assert_eq!(substr2(s1, 2, isize::MAX), "23456789");

    assert_eq!(substr2(s1, 100, 1), "");
    assert_eq!(substr2(s1, isize::MAX, 1), "");

    assert_eq!(substr2(s1, 100, -3), "");
    assert_eq!(substr2(s1, isize::MAX, -3), "");

    assert_eq!(substr2(s1, 100, -100), "");
    assert_eq!(substr2(s1, isize::MAX, isize::MIN), "");

    assert_eq!(substr2(s1, -100, 100), "");
    assert_eq!(substr2(s1, isize::MIN, isize::MAX), "");

    assert_eq!(substr2(s1, -100, -3), "");
    assert_eq!(substr2(s1, isize::MIN, -3), "");

    // 1 position less than maximum isize values
    assert_eq!(substr2(s1, isize::MIN + 1, 5), "");
    assert_eq!(substr2(s1, isize::MAX - 1, -3), "");
    assert_eq!(substr2(s1, 5, isize::MAX - 1), "56789");
    assert_eq!(substr2(s1, 5, isize::MIN + 1), "012345");
    assert_eq!(substr2(s1, isize::MAX - 1, isize::MIN + 1), "");
}

// -----------------------------------------------------------------------------

#[test]
fn test_substr3() {
    let s1: &str = "0123456789";

    assert_eq!(substr3(s1, 2, 3), "234");
    assert_eq!(substr3(s1, -5, 3), "567");
    assert_eq!(substr3(s1, -5, -3), "345");
    assert_eq!(substr3(s1, 5, -3), "345");
    assert_eq!(substr3(s1, 0, 1), "0");

    assert_eq!(substr3(s1, 10, 2), "");
    assert_eq!(substr3(s1, 9, 2), "9");
    assert_eq!(substr3(s1, 8, 2), "89");

    assert_eq!(substr3(s1, -10, 3), "012");
    assert_eq!(substr3(s1, 0, 3), "012"); // Same as previous

    // Empty string results
    assert_eq!(substr3(s1, 2, 0), ""); // 0 length
    assert_eq!(substr3(s1, 0, 0), ""); // 0 length
    assert_eq!(substr3(s1, -4, 0), ""); // 0 length

    assert_eq!(substr3("", 3, 2), "");
    assert_eq!(substr3("", -3, 2), "");
    assert_eq!(substr3("", -3, -2), "");

    // Empty string results past boundaries
    assert_eq!(substr3("", 100, -100), "");
    assert_eq!(substr3("", isize::MAX, isize::MIN), "");

    assert_eq!(substr3("", -100, 100), "");
    assert_eq!(substr3("", isize::MIN, isize::MAX), "");

    // Past string boundaries
    assert_eq!(substr3(s1, -5, -100), "012345");
    assert_eq!(substr3(s1, -5, isize::MIN), "012345");

    assert_eq!(substr3(s1, -100, 100), "");
    assert_eq!(substr3(s1, isize::MIN, isize::MAX), "");

    assert_eq!(substr3(s1, -100, 1), "");
    assert_eq!(substr3(s1, isize::MIN, 1), "");
    assert_eq!(substr3(s1, isize::MIN, 2), "");

    assert_eq!(substr3(s1, 5, -100), "012345");
    assert_eq!(substr3(s1, 5, isize::MIN), "012345");

    assert_eq!(substr3(s1, 2, 100), "23456789");
    assert_eq!(substr3(s1, 2, isize::MAX), "23456789");

    assert_eq!(substr3(s1, 100, 1), "");
    assert_eq!(substr3(s1, isize::MAX, 1), "");

    assert_eq!(substr3(s1, 100, -3), "");
    assert_eq!(substr3(s1, isize::MAX, -3), "");

    assert_eq!(substr3(s1, 100, -100), "");
    assert_eq!(substr3(s1, isize::MAX, isize::MIN), "");

    assert_eq!(substr3(s1, -100, 100), "");
    assert_eq!(substr3(s1, isize::MIN, isize::MAX), "");

    assert_eq!(substr3(s1, -100, -3), "");
    assert_eq!(substr3(s1, isize::MIN, -3), "");

    // 1 position less than maximum isize values
    assert_eq!(substr3(s1, isize::MIN + 1, 5), "");
    assert_eq!(substr3(s1, isize::MAX - 1, -3), "");
    assert_eq!(substr3(s1, 5, isize::MAX - 1), "56789");
    assert_eq!(substr3(s1, 5, isize::MIN + 1), "012345");
    assert_eq!(substr3(s1, isize::MAX - 1, isize::MIN + 1), "");
}

// -----------------------------------------------------------------------------

#[test]
fn test_substr4() {
    // substr4 code not complete for all possible values.
    // Already the slowest solution with this code.
    let s1: &str = "0123456789";

    assert_eq!(substr4(s1, 2, 3), "234");
    assert_eq!(substr4(s1, -5, 3), "567");
    assert_eq!(substr4(s1, -5, -3), "345");
    assert_eq!(substr4(s1, 5, -3), "345");
}

// -----------------------------------------------------------------------------

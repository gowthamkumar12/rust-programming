pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return String::from("");
    }

    let mut result: String = String::from(input);

    let mut i = 0;
    let mut j = result.chars().count() - 1;

    while i < j {
        let char_at_i = result.chars().nth(i).unwrap();
        let char_at_j = result.chars().nth(j).unwrap();

        result.replace_range(
            result.char_indices().nth(i).unwrap().0..
            result.char_indices().nth(i).unwrap().0 + char_at_i.len_utf8(),
            &char_at_j.to_string(),
        );
        result.replace_range(
            result.char_indices().nth(j).unwrap().0..
            result.char_indices().nth(j).unwrap().0 + char_at_j.len_utf8(),
            &char_at_i.to_string(),
        );

        i += 1;
        j -= 1;
    }

    result
}

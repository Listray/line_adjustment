fn is_space_enough(capacity_left: i32, word_len: i32, has_words: bool) -> bool {
    capacity_left >= word_len + if has_words { 1 } else { 0 }
}

fn add_line_to_result(last_word_added_idx: i32,
                      line_capacity_left: i32,
                      words: &Vec<&str>,
                      cur_idx: usize,
                      result: &mut String) {
    let words_amount = cur_idx as i32 - last_word_added_idx - 1;
    
    if words_amount == 1 {
        let mut capacity_left = line_capacity_left;
        result.push_str(words[cur_idx - 1]);

        while capacity_left > 0 {
            result.push_str(" ");
            capacity_left -= 1;
        }
    } else {
        let mut extra_spaces = line_capacity_left % (words_amount - 1);
        let additional_spaces = line_capacity_left / (words_amount - 1);
        
        for j in last_word_added_idx + 1..cur_idx as i32{
            result.push_str(words[j as usize]);

            if j == cur_idx as i32 - 1 {
                return;
            }

            result.push_str(" ");

            if extra_spaces > 0 {
                result.push_str(" ");
                extra_spaces -= 1;
            }
            
            let mut add_spaces = additional_spaces;
                
            while add_spaces > 0 {
                result.push_str(" ");
                add_spaces -= 1;
            }

        }
    }
}

pub fn transform(input: &str, line_width: u32) -> String {
    let words: Vec<&str> = input.split(" ").collect();
    let mut result = String::new();
    
    let mut last_word_added_idx: i32 = -1;
    let mut line_capacity_left: i32 = line_width as i32;

    for i in 0..words.len() {
        let word = words[i];
        let word_len = word.len() as i32;

        if !is_space_enough(line_capacity_left,
                            word_len,
                            i as i32 - 1 != last_word_added_idx) {
            let words_amount = i as i32 - last_word_added_idx - 1;
            add_line_to_result(last_word_added_idx, line_capacity_left, &words, i, &mut result);
            
            result.push_str("\n");
            
            line_capacity_left = line_width as i32;
            last_word_added_idx = i as i32 - 1;
        }

        line_capacity_left -= word_len as i32 + if i as i32 - 1 != last_word_added_idx { 1 } else { 0 };
    }
    
    if line_capacity_left != line_width as i32 {
        add_line_to_result(last_word_added_idx, line_capacity_left, &words, words.len(), &mut result);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      ")
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}

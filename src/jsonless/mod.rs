use std::fmt::Write;

const JSON_STR: &str = r#"
{
    "id": 0,
    // Aloha.
    "value": "Hi.",
}
"#;

pub fn main() {
    println!("{}", JSON_STR);
    let mut json_buffer = String::new();
    for c in JSON_STR.lines() {
        let c = c.trim_start();
        if ! c.starts_with("//") {
            json_buffer.write_str(c).expect("failed writing buffer");
        }
    }
    println!("{}", json_buffer);
}

fn pair(s: &str, l: char, r: char) -> Result<(usize, usize), ()> {
    let mut res = (0, 0);
    let mut depth: Option<usize> = None;
    let dive = |depth| -> Option<usize> {
        match depth {
            None => Some(1),
            Some(i) => Some(i+1)
        }
    };
    let float = |depth| -> Option<usize> {
        match depth {
            None => None,
            Some(i) => Some(i-1)
        }
    };
    let should_pop = |depth| -> bool {
        match depth {
            Some(0) => true,
            Some(_) => false,
            None => false,
        }
    };
    for (i, c) in s.chars().enumerate() {
        if c == r {
            depth = float(depth);
            res.1 = i;
        }
        else 
        if c == l {
            depth = dive(depth);
            res.0 = i;
        } 

        if should_pop(depth) {
            return Ok(res);
        }
    }
    return Err(());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pair() {
        println!("{:?}", pair("adss_ddda", 's', 'd'));
    }
}

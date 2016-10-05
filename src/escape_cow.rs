use std::borrow::Cow;

pub fn naive<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    fn is_trouble(c: char) -> bool {
        c == '<' || c == '>' || c == '&'
    }

    if input.contains(is_trouble) {
        let mut output = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '<' => output.push_str("&lt;"),
                '>' => output.push_str("&gt;"),
                '&' => output.push_str("&amp;"),
                _ => output.push(c)
            }
        }
        Cow::Owned(output)
    } else {
        input
    }
}


pub fn find<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    fn is_trouble(c: char) -> bool {
        c == '<' || c == '>' || c == '&'
    }
    let first = input.find(is_trouble);
    if let Some(first) = first {
        let mut output = String::from(&input[0..first]);
        output.reserve(input.len() - first);
        let rest = input[first..].chars();
        for c in rest {
            match c {
                '<' => output.push_str("&lt;"),
                '>' => output.push_str("&gt;"),
                '&' => output.push_str("&amp;"),
                _ => output.push(c),
            }
        }

        Cow::Owned(output)
    } else {
        input.into()
    }
}

pub fn find_morecap<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    fn is_trouble(c: char) -> bool {
        c == '<' || c == '>' || c == '&'
    }
    let first = input.find(is_trouble);
    if let Some(first) = first {
        let mut output = String::from(&input[0..first]);
        let len = input.len();
        output.reserve(len + len/2 - first);
        let rest = input[first..].chars();
        for c in rest {
            match c {
                '<' => output.push_str("&lt;"),
                '>' => output.push_str("&gt;"),
                '&' => output.push_str("&amp;"),
                _ => output.push(c),
            }
        }

        Cow::Owned(output)
    } else {
        input.into()
    }
}

pub fn find_recursive<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    fn is_trouble(c: char) -> bool {
        c == '<' || c == '>' || c == '&'
    }
    let first = input.find(is_trouble);
    if let Some(first) = first {
        let mut output = String::from(&input[0..first]);
        output.reserve(input.len() - first);
        match input[first..].bytes().next().unwrap() {
            b'<' => output.push_str("&lt;"),
            b'>' => output.push_str("&gt;"),
            b'&' => output.push_str("&amp;"),
            _ => unreachable!()
        }
        if first < input.len() {
            output.push_str(find_recursive(&input[first+1..]).as_ref());
        }
        Cow::Owned(output)
    } else {
        input.into()
    }
}


pub fn find_chars<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    let input = input.into();
    let first = input.chars().position(|c| match c {
        '<'| '>'| '&' => true,
        _ => false
    });
    if let Some(first) = first {
        let mut chars = input.chars().collect::<Vec<_>>();
        let rest = chars.split_off(first);
        let mut output = chars.into_iter().collect::<String>();
        for c in rest {
            match c {
                '<' => output.push_str("&lt;"),
                '>' => output.push_str("&gt;"),
                '&' => output.push_str("&amp;"),
                _ => output.push(c),
            }
        }

        Cow::Owned(output)
    } else {
        input.into()
    }
}

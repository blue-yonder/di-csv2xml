fn mask(c: char) -> Option<&'static str> {
    match c {
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&apos;"),
        '&' => Some("&amp;"),
        _ => None,
    }
}

pub fn escape_char_data<'a>(unescaped: &'a str, buf: &'a mut String) -> &'a str {
    // Search for the first character to be escaped. It is worth treating the case of having nothing
    // to escape seperatly as it is quite common and we do not need to allocate or copy memory in
    // this case.
    if let Some((index, _)) = unescaped.char_indices().find(|&(_,c)| mask(c).is_some()){
        // We need to escape the string. Let's fill the buffer we have been passed, with the
        // characters we checked so far.
        let (does_not_need_escaping, to_escape) = unescaped.split_at(index);
        buf.clear();
        buf.push_str(does_not_need_escaping);

        for c in to_escape.chars(){
            match mask(c) {
                None => buf.push(c),
                Some(s) => buf.push_str(s),
            }
        }

        buf.as_str()
    } else {
        unescaped
    }
}
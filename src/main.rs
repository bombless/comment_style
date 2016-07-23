enum Slice {
    Outside(String),
    Comment(String),
}

enum Status {
    Comment,
    String,
    Dumb,
    Broken,
}

enum Output {
    Outside(char),
    Comment(char),
}


fn handle_in_comment(prev: char, curr: char, next: Option<char>, prev_status: Status) -> (Status, Option<Output>) {
    if prev == '*' && curr == '/' {
        return (Status::Dumb, None)
    }
    let next = match next {
        Some(x) => x,
        None => return (Status::Broken, None)
    };
    if next == '/' && curr == '*' {
        return (Status::Comment, None)
    }
    if let Status::Comment = prev_status {
        return (Status::Comment, Some(Output::Comment(curr)))
    }
    return (Status::Comment, None)
}


fn handle_in_string(prev: char, curr: char, _next: (), _prev_status: ()) -> (Status, Option<Output>) {
    if prev != '\\' && curr == '"' {
        return (Status::Dumb, Some(Output::Outside(curr)))
    }
    return (Status::String, Some(Output::Outside(curr)))
}

fn handle_in_dumb(_prev: (), curr: char, next: Option<char>, _prev_status: ()) -> (Status, Option<Output>) {
    if curr == '"' {
        return (Status::String, Some(Output::Outside(curr)))
    }
    if curr == '/' && next == Some('*') {
        return (Status::Comment, None)
    }
    return (Status::Dumb, Some(Output::Outside(curr)))
}


fn main() {
    let src = "/* hey, just comments */";
    let mut status = Status::Dumb;
    let mut prev_status = Status::Broken;
    let mut prev = None;
    let mut curr = None;
    let mut iter = src.chars();
    loop {
        let next = iter.next();
        let mut output = None;
        match (prev, curr) {
            (None, Some(curr)) => {
                let (s, o) = handle_in_dumb((), curr, next, ());
                output = o;
                prev_status = status;
                status = s
            }
            (Some(prev), Some(curr)) => {
                match status {
                    Status::Comment => {
                        let (s, o) = handle_in_comment(prev, curr, next, prev_status);
                        output = o;
                        prev_status = status;
                        status = s
                    }
                    Status::Dumb => {
                        let (s, o) = handle_in_dumb((), curr, next, ());
                        output = o;
                        prev_status = status;
                        status = s
                    }
                    Status::String => {
                        let (s, o) = handle_in_string(prev, curr, (), ());
                        output = o;
                        prev_status = status;
                        status = s
                    },
                    Status::Broken => {
                        break
                    }
                }
            }
            _ => ()
        }           
            

        prev = curr;
        curr = next;
        if let Status::Broken = status {
            break
        }
        if next.is_none() {
            break
        }
    }
}

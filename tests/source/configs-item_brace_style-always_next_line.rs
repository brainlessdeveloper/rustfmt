// rustfmt-item_brace_style: AlwaysNextLine
// Item brace style

struct Lorem {
    ipsum: bool,
}

struct Dolor<T> where T: Eq {
    sit: T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;
#[cfg(doctest)]
doctest!("../README.md");

#[macro_export]
macro_rules! inline_const {
    ( [ $t:ty ] $e:expr) => {{
        const C: $t = $e;
        C
    }}
}

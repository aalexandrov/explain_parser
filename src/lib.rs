use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::spanned::Spanned;

pub fn parse(explain: &str) {
    let stream = TokenStream::from_str(explain);
    println!("{:?}", stream.into_iter().next().unwrap().span().start());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parse(
            r###"
            Project (#3, #2)
              Filter (#1 < 0) AND (#0 > 0) AND (#2 > 0)
                Map ((#0 + #1), 1)
                  Get materialize.public.mv
            "###,
        );
    }
}

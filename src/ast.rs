use syn::parse::{Parse, ParseStream};
use syn::LitInt;

struct DToken {}

impl Parse for DToken {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        // if lookahead.peek() {

        // }
        todo!()
    }
}

struct DieRollLiteral {
    count: Option<LitInt>,
    // d_token: ,
}

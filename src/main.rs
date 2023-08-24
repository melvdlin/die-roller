use syn::parse_str;

fn main() {
    let e: syn::ExprBinary = parse_str("1+2").unwrap();
}

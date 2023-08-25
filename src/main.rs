fn main() {
    let s = "asdfghi";
    let ss = &s[1..4];
    let sss = &s[4..7];
    println!("{s}, {ss}, {sss}");
    println!("{:?}", ss.char_indices().collect::<Vec<_>>());
}

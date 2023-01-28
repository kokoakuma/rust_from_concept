fn main() {
    let x = 1;
    // !は何も返さないという意味、nothing型に近い
    println!("x = {}", x);

    // mutでミュータブルに宣言
    let mut y = 2;
    y += 1;
    println!("y = {}", y);
}

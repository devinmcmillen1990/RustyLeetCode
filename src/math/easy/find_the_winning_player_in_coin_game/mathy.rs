pub fn find_winner(x: i32, y: i32) -> String {
    let moves = x.min(y / 4);
    if moves % 2 == 1 {
        "Alice".to_string()
    } else {
        "Bob".to_string()
    }
}

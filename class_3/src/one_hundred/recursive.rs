pub fn recursion(mut param: i32) {
    if param <= 100 {
        println!("{}", param);
        param += 1;
        recursion(param);
    }
}
use super::*;

#[test]
fn one_line_result() {
    let query = "Yv";
    let content = "\
Hello Word,
My Name is Yves Kalume, and
This is my small rust program. ";
    assert_eq!(vec!["My Name is Yves Kalume, and"], search(query, content))
}

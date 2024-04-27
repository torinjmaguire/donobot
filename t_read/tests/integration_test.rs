use t_read::t_read;

#[test]
fn string_return() {
    for _i in 0..100 {
        println!("{}", t_read("tests/test.txt").unwrap());
        assert!(t_read("tests/test.txt").is_ok())
    }
}

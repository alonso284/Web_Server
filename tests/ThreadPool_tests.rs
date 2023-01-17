use web_server::*;

#[test]
#[should_panic(expected = "Size must be greater than zero")]
fn invalid_size(){
    if let Err(e) = ThreadPool::new(0) {
        panic!("{}", e)
    }
}

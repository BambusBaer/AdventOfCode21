#[cfg(test)]
mod file_handler_tests {

    #[test]
    fn read_file_of_integer(){
        file_handler::read_all::<i32>("int_test.txt");
        assert_eq!(1, 2);
    }
}
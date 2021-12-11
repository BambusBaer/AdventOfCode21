use std::str::FromStr;

pub fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}

#[cfg(test)]
mod file_handler_tests {
    use super::*;

    #[test]
    fn read_file_of_integer(){
        let file_content = read_all::<i32>("./src/int_test.txt");
        let first_item = file_content[0].as_ref().unwrap();
        let middle_item = file_content[100].as_ref().unwrap();
        let last_item = file_content[1999].as_ref().unwrap();
        assert_eq!(*first_item, 134);
        assert_eq!(*middle_item, 566);
        assert_eq!(*last_item, 10753);
    }
}
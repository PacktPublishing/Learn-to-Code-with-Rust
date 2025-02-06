fn double_the_length<T>(collection: &Vec<T>) -> usize {
    collection.len() * 2
}

fn last_two<T>(collection: &[T]) -> &[T] {
    let two_from_the_end = collection.len() - 2;
    &collection[two_from_the_end..]
}

fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{announcement}");
    &text[..5]
}

fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second // &str != &'a str
    }
}

fn main() {
    println!("{}", double_the_length(&vec![1, 2, 3]));

    let my_vec = vec![1, 2, 3, 4, 5, 6];
    let my_arr = [1, 2, 3, 4, 5, 6];
    println!("{:?}", last_two(&my_vec));
    println!("{:?}", last_two(&my_vec[0..3]));
    println!("{:?}", last_two(&my_arr));
    println!("{:?}", last_two(&my_arr[0..3]));

    println!("{:?}", first_five("refrigerator", "Hello"));

    println!(
        "{:?}",
        find_string_that_has_content("programming", "dining", "gram")
    );
}

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 5;
    let element = vec.get(index);
    match element {
        Some(value) => println!("Element at index {} is: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    }
}
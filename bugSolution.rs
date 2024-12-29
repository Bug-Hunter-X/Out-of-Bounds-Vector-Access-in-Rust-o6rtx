fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 5;

    // Safe way to handle out-of-bounds access
    if index < vec.len() {
        let element = vec.get(index).unwrap();
        println!("Element at index {} is: {}", index, element);
    } else {
        println!("Index {} is out of bounds", index);
    }

    //Alternative solution using a match statement
    match vec.get(index) {
        Some(value) => println!("Element at index {} is: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    }
} 
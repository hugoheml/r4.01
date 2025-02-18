#[derive(Clone, Copy)]
struct Rectangle {
    length: f64,
    width: f64
}

fn average(a: f64, b:f64) -> f64 {
    return (a + b) / 2.0;
}

fn perimeter(rectangle: Rectangle) -> f64 {
    return 2.0 * (rectangle.length + rectangle.width);
}

// fn perimeter2(rectangle: &Rectangle) -> f64 {
//     return 2.0 * (rectangle.length + rectangle.width);
// }

fn swap(a: &mut f64, b: &mut f64) {
    let temp = *a;
    *a = *b;
    *a = temp;
}

#[allow(dead_code)]
fn print_references() {
    let mut entier = 5;
    // let reference1 = &entier;
    // let reference2 = &entier;
    let reference3 = &mut entier;
    // let reference4 = &mut entier;

    // println!("Référence 1: {}, Référence 2: {}", reference1, reference2);
    println!("Référence 3: {}", reference3);
}

fn main() {
    let a: f64 = 5.0;
    let b: f64 = 10.0;

    // Passage par valeur
    let result = average(a, b);
    println!("The average of {} and {} is {}", a, b, result);
    
    let result2 = average(a, b);
    println!("The average of {} and {} is {}", a, b, result2);

    let rect = Rectangle {
        length: 5.0,
        width: 10.0
    };

    // Passage par valeur
    let result3 = perimeter(rect);
    println!("The perimeter of the rectangle is {}", result3);

    let result3 = perimeter(rect);
    println!("The perimeter of the rectangle is {}", result3);

    let mut x = 5.0;
    let mut y = 10.0;

    swap(&mut x, &mut y);
    println!("x: {}, y: {}", x, y);
}
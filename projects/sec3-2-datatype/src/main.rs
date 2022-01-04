fn main() {
    println!("Hello, world!");
    //let guess: u32 = "42".parse().expect("Not a number");

    let x = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c={}, z={}, heart_eyed_cat={}", c,z, heart_eyed_cat);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
    
        
}

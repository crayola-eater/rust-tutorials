fn main() {
    let tup1 = (20, "Rust", 3.4, false);
    println!("The third tuple element is {}", tup1.2);

    let tup2 = (1, (1, 4, 7));
    println!("The second tuple element (in second tuple) is {}", (tup2.1).1);

    let (a, b, c, d) = tup1;
    println!("The variables are {}, {}, {}, {}", a, b, c, d);

    // Nested destructuring.
    let (e, (f, g, h)) = tup2;
    println!("The variables are {}, {}, {}, {}", e, f, g, h)
}

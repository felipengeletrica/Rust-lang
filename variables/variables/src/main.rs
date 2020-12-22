fn main() {

    let mut _cont: u128 = 0;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    _cont = 36;

  
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
    println!("{}", _heart_eyed_cat);



    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = _tup;
    println!("The value of y is: {}", _y);
    println!("Tuple {:?}", _tup);


    let five_hundred = _tup.0;
    let six_point_four = _tup.1;
    let one = _tup.2;
    println!("{}", five_hundred);
    println!("{}", six_point_four);
    println!("{}", one);


    let _a = [1, 2, 3, 4, 5];
    let _first = _a[0];
    let _second = _a[1];
    println!("{}", _first);
    println!("{}", _second);

    another_function();
    let value = plus_one(x);
    println!("{}", value)
}


fn another_function() {
    println!("Another function.");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
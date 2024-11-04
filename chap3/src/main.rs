use std::io;


fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);


    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // addition
    // 足し算
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    // subtraction
    // 引き算
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // multiplication
    // 掛け算
    let product = 4 * 30;
    println!("The value of product is: {}", product);

    // division
    // 割り算
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
                         // 結果は0
    println!("The value of quotient is: {}", quotient);
    println!("The value of floored is: {}", floored);

    // remainder
    // 余り
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation
                         // 明示的型注釈付きで
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';    //ハート目の猫
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);


    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    let a = [3; 5];
    println!("The value of a is: {:?}", a);


    // 配列は、固定長で全て同じ型の要素を持つような月の名前を格納するために使うことができます。
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The value of months is: {:?}", months);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
           // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
              // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
                // 入力された値は数字ではありません

    // ここで配列の数を超えるインデックスを指定した場合、プログラムはパニックを起こします。
    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index, element
    );


}
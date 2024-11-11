fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    println!("v: {:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        //                      "3つ目の要素は{}です"
        Some(third) => println!("The third element is {}", third),
        //               "3つ目の要素はありません。"
        None => println!("There is no third element."),
    }

    // 以下はコンパイルエラー
    // なぜなら、vの不変の参照があるため、可変の参照を作成できない 
    // ベクターは新しい要素が追加されると、古い要素がメモリ上の別の場所にコピーする必要がある
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("s: {}", s);

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意

    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let len = String::from("Hola").len();
    println!("len: {}", len);

    let len = String::from("Здравствуйте").len();
    println!("len: {}", len);

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    // 0..1の範囲を指定したら、エラーになる。なぜなら、文字列スライスはバイト単位でなければならない
    let answer = &hello[0..4];
    println!("answer: {}", answer);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);


    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("scores: {:?}", scores);

}
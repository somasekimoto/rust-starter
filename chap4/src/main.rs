fn main() {
    // 文字列リテラルとString型の違い
    // 文字列リテラルは、コンパイル時に既知で、固定されたテキスト
    // スタックに直接格納されるため、高速で効率的
    {                      // sは、ここでは有効ではない。まだ宣言されていない
        let s = "hello";   // sは、ここから有効になる

        // sで作業をする

        println!("{}", s);  // ここでsは有効である
    }

    // String型
    // String型は、可変、変更可能、所有権を持つ、UTF-8エンコードされたテキスト
    // ヒープに確保されるため、サイズが可変で、コンパイル時には不明
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    println!("{}", s); // これは`hello, world!`と出力する

    // drop関数
    // drop関数は、値がスコープを抜けたときに呼び出される
    // この関数は、値がスコープを抜けるときに、その値を解放するために使用される
    // Rustは、スコープを抜けるときに、自動的にdrop関数を呼び出す
    // String型の場合、drop関数は、ヒープメモリを解放する
    {
        let s = String::from("hello"); // sはここから有効
        // sはここでスコープを抜ける
    } // このスコープが終わると、sは無効になり、drop関数が呼び出される

    // ムーブ
    // Rustの変数は、デフォルトでムーブされる
    // ムーブとは、変数の値が別の変数にコピーされることなく、変数から別の変数に移動されること
    // これにより、Rustは、メモリの二重解放やデータ競合を防ぐ
    // なので以下のコードはエラーになる

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    // クローン
    // クローンとは、ヒープデータをコピーすること
    // 以下のコードはエラーにならない
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // コピー
    // Rustは、スタックデータをコピーする
    // スタックデータは、固定サイズで、コンパイル時に既知
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // 所有権と関数

    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫
    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
    // ムーブする

    let s2 = String::from("hello");     // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる

    // 参照と借用
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);

    // 可変な参照
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

    let r2 = &mut s;

    // これはエラーになる
    // let reference_to_nothing = dangle();

    // スライス
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // 文字列スライス
    // 文字列スライスは、Stringの一部を参照する
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    let slice = &s[0..len];
    let slice = &s[..];

}

fn takes_ownership(some_string: String) { // some_stringがスコープに入る
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、drop関数が呼ばれる。メモリが解放される

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない

fn gives_ownership() -> String {             // gives_ownershipが呼ばれる
    let some_string = String::from("hello"); // some_stringがスコープに入る
    some_string                              // some_stringが返され、呼び出し元関数にムーブされる
}

fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る
    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}

fn calculate_length(s: &String) -> usize { // sはStringへの参照
    s.len()
} // ここで、sはスコープを抜けるが、何も特別なことはない

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// これはエラーになる
// fn dangle() -> &String { // dangleはStringへの参照を返す
//     let s = String::from("hello"); // sは新しいString

//     &s // String sへの参照を返す
// } // ここでsがスコープを抜け、drop関数が呼ばれる。そのメモリは解放される
// // 危険！

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // バイト列で文字列をエンコード

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


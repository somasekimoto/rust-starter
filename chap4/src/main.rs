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

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);
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

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返す

    (s, length)
}

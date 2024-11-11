use std::fs::File;

fn main() {
    let v = vec![1, 2, 3];

    // v[99];


    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // ファイルを開く際に問題がありました
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}

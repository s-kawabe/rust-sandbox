// 数値型
// i: 符号付き整数
// u: 符号なし整数
// f: 浮動小数点数
// 8, 16, 32, 64, 128 ビットの変数の種類がある（浮動小数点数はf32, f64のみ）
// isize, usize: プラットフォームに依存する整数型（64ビット環境ではi64, u64）
#[allow(dead_code)]
fn numeric_sample() {
    let _a: i8 = 10;
}

// 文字列型
// str, &str: 固定長の文字列、スライスと呼ばれる
// String: 可変長の文字列
// お互いに型変換可能
// String -> &str: ポインタと文字列長をコピーしてスライスを作る。メモリを圧迫しない
// &str -> String: メモリの確保が行われる
#[allow(dead_code)]
fn string_sample() {
    let s1: String = String::from("Hello, world!");
    let s2: &str = &s1; // String -> &str
    let s3: String = s2.to_string(); // &str -> String
}

// タプル型 - 異なる型を収めることができる集合
#[allow(dead_code)]
fn tuple_sample() {
    let mut t1 = (10, 10.0, "Hello, world!");
    t1.0 = 20; // 変数.n でアクセスできる
    t1.2 = "Hello, Rust!";
}

// 配列型 - 特定の方の値を連続に収めた集合（配列のサイズは固定でコンパイル時に決まっている必要がある）
#[allow(dead_code)]
fn array_sample() {
    let a1 = [10, 20, 30, 40, 50];
    let a2: [i32; 5] = [10, 20, 30, 40, 50];
    let a3: [i32; 5] = [0; 5]; // 0で5つの要素を初期化

    // 配列を参照するときは自動的にスライスとして扱われる
    println!("a1: {:?}", a1);
    println!("a2: {:?}", a2);
    println!("a3: {:?}", a3[0..2]); // スライスとして扱うと範囲指定ができる
}

// 列挙型
//  Rustの列挙型はそれぞれの列挙子に追加のデータを付与できる
#[allow(dead_code)]
fn enum_sample() {
    enum Color {
        Red(u8),
        Green(String),
        Blue(i32, f64),  // タプルバリアント。値が2つ
        Yellow([u8; 3]), // 配列1個を持つ。Yellow(u8; 3) のような書き方はできない
    }
    let _blue = Color::Blue(10, 1.5);
    let _yellow = Color::Yellow([255, 255, 0]);
}

// ユーザー定義型
#[allow(dead_code)]
fn user_defined_type_sample() {
    struct User {
        name: String,
        age: u32,
    }
    let u = User {
        name: String::from("John"),
        age: 23,
    }
}

// よく出てくる標準ライブラリの型

#[allow(dead_code)]
fn option_sample() {
    // Option<T> - 値があるかどうかを表す。Some(T) か None
    // 内部実装
    //   enum Option_<T> {
    //       Some(T),
    //       None,
    //   }
    let o1: Option<i32> = Some(10);

    // Result<T, E> - 成功か失敗かを表す。Ok(T) か Err(E)
    // 内部実装
    //   enum Result_<T, E> {
    //       Ok(T),
    //       Err(E),
    //   }
    let result: Result<i32, String> = Ok(200);
    match result {
        Ok(code) => println!("Success: {}", code),
        Err(e) => println!("Error: {}", e),
    }
    // パターンマッチングを用いずにもできる
    if let Ok(code) = result {
        println!("Success: {}", code);
    } else {
        println!("Error: {}", result.unwrap_err());
    }
    // unwrap_err(), unwrap_or(), unwrap_or_else() などのメソッドがある
    let result: Result<i32, String> = Ok(200);
    println!("code: {}", result.unwrap_or(-1));
    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1));
    // and_then()
    fn func(code: i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100) // return Ok(100); と同じ
    }
    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func); // func()は実行される
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func); // func()は実行されない
    // ? 構文
    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let code = result?;
        println!("code: {}", code);
        Ok(100)
    }
    // let-else 構文
    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let Ok(code) = result else {
            let err = result.unwrap_err();
            println!("error: {}", err);
            return Err(err);
        };
        println!("code: {}", code);
        Ok(100)
    }

    // Vec<T> - 可変長の配列。Vec::new() で空の配列を作成できる

    // Box<T> - ヒープ上にデータを確保する。メモリ管理を自動で行う
    
}
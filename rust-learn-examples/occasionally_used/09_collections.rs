// 09_collections.rs

// Rust 标准库提供了几种非常有用的集合类型。
// Vec<T>: 动态数组（vector），允许你存储多个相同类型的值，并且可以在末尾添加或删除元素。
// String: 可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。
// HashMap<K, V>: 哈希映射（hash map），允许你将键（K类型）与值（V类型）关联起来。

use std::collections::HashMap;

fn main() {
    // --- Vec<T> (Vector) ---
    // 创建一个新的空 vector
    let mut v: Vec<i32> = Vec::new();

    // 使用 vec! 宏创建包含初始值的 vector
    let v2 = vec![1, 2, 3];

    // 更新 vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector v: {:?}", v);
    println!("Vector v2: {:?}", v2);

    // 读取 vector 的元素
    let third: &i32 = &v[2]; // 使用索引访问，如果索引越界会 panic
    println!("The third element of v is {}", third);

    match v.get(2) { // 使用 get 方法访问，返回 Option<&T>，更安全
        Some(third_val) => println!("The third element of v (via get) is {}", third_val),
        None => println!("There is no third element in v."),
    }

    // 遍历 vector 中的值
    println!("Iterating over v:");
    for i in &v { // 不可变引用遍历
        println!("  {}", i);
    }

    println!("Iterating and modifying v:");
    for i in &mut v { // 可变引用遍历，可以修改元素
        *i += 50; // 解引用并修改
    }
    println!("Vector v after modification: {:?}", v);

    // 使用枚举来存储多种类型
    // Vector 只能存储相同类型的值。如果需要存储不同类型的值，可以使用枚举。
    #[derive(Debug)]
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
    println!("Spreadsheet row: {:?}", row);

    // --- String ---
    // String 是在堆上分配的，UTF-8 编码，可增长。
    let mut s = String::new(); // 创建空字符串

    let data = "initial contents";
    let s_from_literal = data.to_string(); // 从 &str 创建 String
    let s_from_string_new = String::from("initial contents"); // 另一种从 &str 创建 String 的方法

    println!("s_from_literal: {}", s_from_literal);
    println!("s_from_string_new: {}", s_from_string_new);

    // 更新 String
    s.push_str("bar"); // 追加 &str
    s.push('!'); // 追加 char
    println!("String s after push: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // `+` 运算符连接 String，s1 被移动，s2 需要是 &str
                       // 注意：s1 在这里之后不再有效
    println!("Concatenated string s3: {}", s3);
    // println!("s1 after concatenation: {}", s1); // 这行会报错

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let game = format!("{}-{}-{}", tic, tac, toe); // format! 宏用于连接字符串，不获取所有权
    println!("Formatted game string: {}", game);
    println!("tic, tac, toe are still valid: {}, {}, {}", tic, tac, toe);

    // 索引 String
    // Rust 的 String 不支持直接通过索引访问字符，因为 UTF-8 编码中字符可能占用多个字节。
    // let char_s = s[0]; // 这行会报错

    // 遍历 String
    println!("Characters in 'नमस्ते':");
    for c in "नमस्ते".chars() { // chars() 方法返回字符迭代器
        println!("  {}", c);
    }

    println!("Bytes in 'नमस्ते':");
    for b in "नमस्ते".bytes() { // bytes() 方法返回字节迭代器
        println!("  {}", b);
    }

    // --- HashMap<K, V> ---
    // HashMap 存储键值对。键的类型 K 必须实现 Eq 和 Hash trait。
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores HashMap: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // get 返回 Option<&V>
    match score {
        Some(s) => println!("Score for Blue team: {}", s),
        None => println!("Blue team not found."),
    }

    // 遍历 HashMap
    println!("Iterating over scores HashMap:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // 更新 HashMap
    // 覆盖一个值
    scores.insert(String::from("Blue"), 25);
    println!("Scores after updating Blue: {:?}", scores);

    // 只在键没有对应值时插入
    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(100); // Blue 已存在，不会改变
    println!("Scores after or_insert: {:?}", scores);

    // 基于旧值更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", map);

    println!("\nAnchor 合约中集合类型的提示：");
    println!("合约中常用 Vec<T> 来存储动态列表，例如账户列表、日志等。");
    println!("String 也常用，例如存储名称、URI等。");
    println!("HashMap 在链上合约中不能直接使用，因为其大小不固定且可能涉及复杂的内存分配。");
    println!("如果需要在合约中实现类似映射的功能，通常会使用 PDA (Program Derived Address) 结合账户数据，或者使用 BTreeMap（需要外部库且谨慎使用）。");
}
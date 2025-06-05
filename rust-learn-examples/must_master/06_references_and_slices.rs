// 06_references_and_slices.rs

// 引用允许你使用值但不获取其所有权。
// 切片允许你引用集合中连续的元素序列，而不是整个集合。

fn main() {
    // --- 引用 (References) ---
    // 在 Rust 中，引用默认是不可变的。
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1 创建一个指向 s1 的引用
    println!("The length of '{}' is {}.", s1, len);

    // 可变引用 (Mutable References)
    // 你可以使用 &mut 来创建可变引用，允许你修改借用的值。
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 after change: {}", s2);

    // 可变引用的规则：
    // 1. 在特定作用域内，对某一块数据，你只能有一个可变引用。
    //    let r1 = &mut s2;
    //    let r2 = &mut s2; // 错误！不能有两个可变引用
    //    println!("{}, {}", r1, r2);

    // 2. 不能在拥有不可变引用的同时拥有可变引用。
    //    let r1 = &s2;       // 不可变引用
    //    let r2 = &mut s2; // 错误！
    //    println!("{}, {}", r1, r2);
    //    不过，如果不可变引用的作用域在可变引用创建之前结束，则是允许的。
    let mut s_rule_check = String::from("hello");
    let r_immutable1 = &s_rule_check;
    let r_immutable2 = &s_rule_check;
    println!("Immutable refs: {} and {}", r_immutable1, r_immutable2);
    // r_immutable1 和 r_immutable2 的作用域到此结束

    let r_mutable = &mut s_rule_check;
    r_mutable.push_str(" world");
    println!("Mutable ref: {}", r_mutable);


    // --- 切片 (Slices) ---
    // 切片让你引用集合中某一段连续的元素序列，而不是整个集合。
    // 切片是一种引用，所以它不拥有数据。

    // 字符串切片 (String Slices)
    // 字符串切片是对 String 中一部分内容的引用。
    let s = String::from("hello world");

    let hello = &s[0..5]; // 切片从索引 0 开始，到索引 5 结束（不包括 5）
    let world = &s[6..11]; // 切片从索引 6 开始，到索引 11 结束（不包括 11）
    println!("Slice 'hello': {}", hello);
    println!("Slice 'world': {}", world);

    // 如果切片从索引 0 开始，可以省略开头的 0：
    let slice_from_start = &s[..5]; // 等同于 &s[0..5]
    println!("Slice from start: {}", slice_from_start);

    // 如果切片包含 String 的最后一个字节，可以省略结尾的长度：
    let slice_to_end = &s[6..]; // 等同于 &s[6..s.len()]
    println!("Slice to end: {}", slice_to_end);

    // 也可以同时省略开头和结尾，得到整个字符串的切片：
    let whole_slice = &s[..]; // 等同于 &s[0..s.len()]
    println!("Whole slice: {}", whole_slice);

    // 字符串字面量就是切片
    // 字符串字面量的类型是 &str，它是一个指向二进制程序特定位置的切片。
    let s_literal: &str = "I am a string literal.";
    println!("String literal: {}", s_literal);

    // 函数中使用字符串切片
    let my_string = String::from("hello rust");
    let word = first_word(&my_string[..]); // first_word 接收一个字符串切片
    println!("First word of '{}' is '{}'", my_string, word);

    // my_string.clear(); // 错误！因为 word 仍然借用了 my_string 的一部分
    // println!("First word after clear is '{}'", word); // 如果上面 clear() 成功，这里会出问题

    // 数组切片 (Array Slices)
    // 就像字符串切片一样，数组切片也可以引用数组的一部分。
    let arr = [1, 2, 3, 4, 5];
    let slice_arr: &[i32] = &arr[1..3]; // 引用数组中索引为 1 和 2 的元素
    println!("Array slice: {:?}", slice_arr);
    assert_eq!(slice_arr, &[2, 3]);
}

// 函数接收一个字符串引用，返回其长度
fn calculate_length(s: &String) -> usize {
    s.len()
} // s 在这里移出作用域。但因为它不拥有数据，所以什么也不会发生。

// 函数接收一个可变的字符串引用，并修改它
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 函数接收一个字符串切片，并返回它找到的第一个单词的切片。
// 如果字符串中没有空格，则整个字符串被认为是一个单词，并返回整个字符串的切片。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // 将字符串转换为字节数组

    for (i, &item) in bytes.iter().enumerate() { // iter() 返回集合中的每个元素，enumerate() 将其包装成元组 (索引, 引用)
        if item == b' ' { // 如果找到空格
            return &s[0..i]; // 返回空格前的部分作为切片
        }
    }

    &s[..] // 如果没有找到空格，返回整个字符串的切片
}
// 02_data_types.rs

// Rust 中的数据类型分为标量类型和复合类型。

fn main() {
    // --- 标量类型 ---
    // 标量类型代表单个值。Rust 有四种主要的标量类型：整数、浮点数、布尔值和字符。

    // 整数类型
    // Rust 有多种整数类型，可以是有符号的（i）或无符号的（u），以及不同的大小（8, 16, 32, 64, 128位，以及arch，其大小取决于底层计算机的架构）。
    let a: i32 = -10; // 有符号32位整数
    let b: u64 = 100; // 无符号64位整数
    let c = 98_222;   // 类型推断，默认为 i32
    let d = 0xff;     // 十六进制
    let e = 0o77;     // 八进制
    let f = 0b1111_0000; // 二进制
    let g = b'A';    // Byte (u8 only)

    println!("整数: a={}, b={}, c={}, d={}, e={}, f={}, g={}", a, b, c, d, e, f, g);

    // 浮点数类型
    // Rust 有两种浮点数类型：f32 和 f64。默认类型是 f64。
    let x = 2.0; // f64 (默认)
    let y: f32 = 3.0; // f32
    println!("浮点数: x={}, y={}", x, y);

    // 布尔类型
    // Rust 中的布尔类型有两个可能的值：true 和 false。
    let t = true;
    let f: bool = false; // 显式类型注解
    println!("布尔值: t={}, f={}", t, f);

    // 字符类型
    // Rust 的 char 类型是语言中最原始的字母类型。
    // char 字面量使用单引号指定，与字符串字面量不同，字符串字面量使用双引号。
    // Rust 的 char 类型大小为四个字节，并表示一个 Unicode 标量值，这意味着它可以表示比 ASCII 多得多的内容。
    let c = 'z';
    let z: char = 'ℤ'; // 显式类型注解
    let heart_eyed_cat = '😻';
    println!("字符: c={}, z={}, heart_eyed_cat={}", c, z, heart_eyed_cat);

    // --- 复合类型 ---
    // 复合类型可以将多个值组合成一个类型。Rust 有两种主要的复合类型：元组（tuple）和数组（array）。

    // 元组类型
    // 元组是将多个不同类型的值组合进一个复合类型的主要方式。
    // 元组有固定的长度：一旦声明，其大小不能增长或缩小。
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 可以使用模式匹配来解构元组值，像这样：
    let (x, y, z) = tup;
    println!("元组解构: x={}, y={}, z={}", x, y, z);

    // 也可以通过索引并使用点号（.）来直接访问元组的元素。
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("元组元素访问: {}, {}, {}", five_hundred, six_point_four, one);

    // 没有任何值的元组 () 是一种特殊类型，只有一个值，也写作 ()。该类型被称为单元类型，该值被称为单元值。
    // 如果表达式不返回任何其他值，则隐式返回单元值。
    let unit_tuple = ();
    println!("单元元组: {:?}", unit_tuple);

    // 数组类型
    // 另一种包含多个值的方式是数组。与元组不同，数组的每个元素必须具有相同的类型。
    // Rust 中的数组与某些其他语言中的数组不同，因为 Rust 中的数组具有固定长度。
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // 你可以显式指定数组的类型和长度：
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组 b 的第一个元素: {}", b[0]);

    // 你也可以通过指定初始值和长度来创建一个包含相同值的数组：
    let c = [3; 5]; // 等同于 let c = [3, 3, 3, 3, 3];
    println!("数组 c 的所有元素相同: {:?}", c);

    // 访问数组元素
    // 数组是一块分配在栈上的已知固定大小的内存。
    // 你可以使用索引来访问数组的元素，像这样：
    let first = a[0];
    let second = a[1];
    println!("数组 a 的前两个元素: first={}, second={}", first, second);

    // 无效的数组元素访问
    // 如果你尝试访问超出数组末尾的元素，例如索引等于或大于数组长度，程序会 panic。
    // let invalid_access = a[10]; // 这行会引起 panic
    // println!("无效访问 (不会执行到这里): {}", invalid_access);

    // Vec<T> (Vector)
    // Vec<T>，通常称为 vector，是一种可增长的数组类型。
    // 它在堆上分配数据，因此其大小可以在运行时改变。
    let mut v: Vec<i32> = Vec::new(); // 创建一个空的 vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Vector v: {:?}", v);

    let v2 = vec![1, 2, 3]; // 使用 vec! 宏创建 vector
    println!("Vector v2: {:?}", v2);

    // Option<T>
    // Option<T> 枚举用于表示一个值可能存在（Some(value)）或不存在（None）的情况。
    // 这有助于处理空值，避免了其他语言中常见的 null 指针错误。
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("Option some_number: {:?}", some_number);
    println!("Option some_string: {:?}", some_string);
    println!("Option absent_number: {:?}", absent_number);

    match some_string {
        Some(i) => println!("some_number 包含: {}", i),
        None => println!("some_number 是 None"),
        
    }

    // 字符串 (String 和 &str)
    // Rust 有两种主要的字符串类型：String 和字符串切片 &str。
    // String 是一个可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。它存储在堆上。
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加一个字面值
    println!("String s: {}", s);

    // &str 是一个字符串切片，它是一个指向存储在其他地方（例如，String 或二进制文件中的字面量）的 UTF-8 编码字符串的引用。
    // 字符串字面量是 &str 类型。
    let s_literal = "hello world literal";
    println!("字符串字面量 s_literal: {}", s_literal);

    // 可以从 String 创建 &str
    let s_ref: &str = &s[..];
    println!("从 String 创建的 &str s_ref: {}", s_ref);
}
// 15_std_lib_and_macros.rs

// Rust 的标准库提供了许多有用的功能，宏则提供了一种代码生成的方式，可以减少重复代码。

fn main() {
    // --- 常用宏 ---

    // println! 和 format!
    // println! 用于打印到控制台。
    // format! 用于创建格式化的 String。
    let name = "Rust Developer";
    let age = 3;
    println!("Hello, {}! You are {} years old.", name, age);

    let greeting = format!("My name is {} and I am {} years old.", name, age);
    println!("{}", greeting);

    // vec![]
    // 用于创建 Vec (vector) 的便捷宏。
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers vector: {:?}", numbers);

    // assert! 和 assert_eq!
    // 用于在测试和代码中进行断言，如果断言失败则 panic。
    // assert!(expression) // 如果 expression 为 false 则 panic
    // assert_eq!(left, right) // 如果 left != right 则 panic
    let a = 5;
    let b = 5;
    assert_eq!(a, b, "a and b should be equal");
    // let c = 6;
    // assert_eq!(a, c, "a and c should be equal"); // 这会 panic

    assert!(a == b, "a should be equal to b");

    // dbg!
    // 一个方便的宏，用于打印表达式的值及其在代码中的位置，主要用于调试。
    // 它会获取表达式的所有权，然后打印文件名、行号和表达式的值，最后返回表达式的值。
    let x = 2 * 3;
    dbg!(x);
    let y = dbg!(x + 1);
    println!("y after dbg: {}", y);

    // panic!
    // 用于使当前线程立即 panic。
    // panic!("This is a deliberate panic!");

    // unreachable!
    // 用于标记代码中理论上不应该被执行到的路径。如果执行到，会 panic。
    // fn foo(x: Option<i32>) {
    //     match x {
    //         Some(val) => println!("Value: {}", val),
    //         None => unreachable!("This should never be None!"),
    //     }
    // }
    // foo(Some(10));
    // foo(None); // 这会 panic

    // todo!
    // 用于标记未完成的代码。如果执行到，会 panic。
    // fn not_implemented_yet() {
    //     todo!("Implement this function later");
    // }
    // not_implemented_yet(); // 这会 panic

    // --- #[derive(...)] 宏 ---
    // 这是一个属性宏，用于自动为结构体和枚举实现某些 trait。
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone(); // 需要 Clone trait
    println!("Point p1: {:?}", p1); // 需要 Debug trait
    println!("Are p1 and p2 equal? {}", p1 == p2); // 需要 PartialEq trait

    // --- 常用标准库函数示例 ---

    // String 操作
    let mut my_string = String::from("Hello");
    my_string.push_str(" Rust");
    println!("my_string: {}", my_string);
    println!("Length of my_string: {}", my_string.len());
    println!("Is my_string empty? {}", my_string.is_empty());
    if my_string.contains("Rust") {
        println!("my_string contains 'Rust'");
    }
    for word in my_string.split_whitespace() {
        println!("Word: {}", word);
    }

    // Vec 操作
    let mut my_vec = vec![10, 20, 30];
    my_vec.push(40);
    println!("my_vec: {:?}", my_vec);
    if let Some(last_val) = my_vec.pop() {
        println!("Popped value: {}", last_val);
    }
    println!("my_vec after pop: {:?}", my_vec);
    println!("Element at index 1: {}", my_vec[1]);

    // Option<T> 方法
    let some_num: Option<i32> = Some(5);
    let none_num: Option<i32> = None;

    println!("Is some_num Some? {}", some_num.is_some());
    println!("Is none_num None? {}", none_num.is_none());

    // unwrap_or: 如果是 Some，返回值；如果是 None，返回提供的默认值。
    println!("some_num unwrapped or default: {}", some_num.unwrap_or(0));
    println!("none_num unwrapped or default: {}", none_num.unwrap_or(0));

    // map: 如果是 Some，对内部值应用函数；如果是 None，返回 None。
    let mapped_some = some_num.map(|x| x * 2);
    println!("Mapped some_num: {:?}", mapped_some);

    // Result<T, E> 方法
    let ok_result: Result<i32, &str> = Ok(100);
    let err_result: Result<i32, &str> = Err("Something went wrong");

    println!("Is ok_result Ok? {}", ok_result.is_ok());
    println!("Is err_result Err? {}", err_result.is_err());

    // unwrap_or_else: 如果是 Ok，返回值；如果是 Err，调用闭包并返回其结果。
    println!("ok_result unwrapped or else: {}", ok_result.unwrap_or_else(|e| {
        println!("Error occurred: {}", e);
        -1
    }));
    println!("err_result unwrapped or else: {}", err_result.unwrap_or_else(|e| {
        println!("Error occurred: {}", e);
        -1
    }));

    // 迭代器 (Iterators) 和相关方法
    let numbers_for_iter = vec![1, 2, 3, 4, 5];
    // map: 对迭代器的每个元素应用一个函数，生成新的迭代器
    let doubled: Vec<i32> = numbers_for_iter.iter().map(|x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    // filter: 根据条件过滤迭代器的元素
    let evens: Vec<&i32> = numbers_for_iter.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);

    // sum: 计算迭代器元素的总和
    let sum: i32 = numbers_for_iter.iter().sum();
    println!("Sum of numbers: {}", sum);

    // collect: 将迭代器转换为集合类型
    let collected_vec: Vec<i32> = (1..=5).collect();
    println!("Collected vec from range: {:?}", collected_vec);

    println!("\nAnchor 合约中常用宏提示：");
    println!("msg!(\"...\"): 用于在 Solana 程序日志中打印消息。");
    println!("#[derive(Accounts)]: 用于自动为账户结构体实现 Anchor 需要的 trait。");
    println!("#[program]: 标记包含指令处理函数的模块。");
    println!("#[account]: 用于定义账户结构体的属性，如空间大小、种子等。");
    println!("#[error_code]: 用于定义自定义错误枚举。");
    println!("err!(ErrorCode::MyError): 用于从指令中返回错误。");
}
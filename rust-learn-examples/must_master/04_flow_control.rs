// 04_flow_control.rs

// Rust 提供了多种流程控制结构，允许你根据条件执行代码块或重复执行代码块。

fn main() {
    // --- if 表达式 ---
    // `if` 表达式允许你根据条件来执行不同的代码分支。
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // `if` 是一个表达式，所以可以在 `let` 语句的右侧使用它。
    let condition = true;
    let num = if condition {
        5
    } else {
        6
        // "six" // 如果分支的类型不匹配，会导致编译错误
    };
    println!("The value of num is: {}", num);

    // --- 循环 ---
    // Rust 有三种循环：`loop`、`while` 和 `for`。

    // `loop` 循环
    // `loop` 关键字告诉 Rust 一遍又一遍地执行一块代码，直到你明确告诉它停止。
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // `break` 可以返回值
        }
    };
    println!("The result from loop is: {}", result);

    // `while` 循环
    // 当条件为真时，`while` 循环会一直执行。
    let mut number_while = 3;
    while number_while != 0 {
        println!("{}!", number_while);
        number_while -= 1;
    }
    println!("LIFTOFF!!! (from while loop)");

    // `for` 循环
    // `for` 循环用于遍历集合中的每个元素，如数组或范围。
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        // .iter() 返回集合中每个元素的引用
        println!("the value is: {}", element);
    }

    // 使用范围 (Range)
    // `for` 循环常与范围一起使用，范围是一种特殊的迭代器，可以生成从一个数字开始到另一个数字结束（不包括结束数字）的序列。
    for number_for in (1..4).rev() {
        // (1..4) 生成 1, 2, 3。 .rev() 反转范围
        println!("{}!", number_for);
    }
    println!("LIFTOFF!!! (from for loop with range)");

    // --- match 表达式 ---
    // `match` 表达式允许你将一个值与一系列模式进行比较，并根据匹配的模式执行代码。
    // `match` 类似于其他语言中的 `switch` 语句，但功能更强大。
    // `match` 表达式必须是穷尽的，意味着必须覆盖所有可能的值。
    let value = 5;
    match value {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"), // `|` 用于匹配多个模式
        13..=19 => println!("a teen"),                     // `..=` 用于匹配一个闭区间范围
        _ => println!("ain't special"),                    // `_` 是一个通配符，匹配任何值
    }

    let optional_value = Some(7);
    match optional_value {
        Some(x) => println!("The value is {}", x),
        None => println!("No value"),
    }

    // `if let` 表达式
    // `if let` 是一种更简洁的方式来处理只关心一种 `match` 情况的场景。
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age_val) = age {
        if age_val > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

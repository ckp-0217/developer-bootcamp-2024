// 03_functions_and_scope.rs

// Rust 代码使用函数（function）来组织。函数声明以 `fn` 关键字开始。
// 函数名和参数名遵循 snake_case 命名规范，即所有字母都是小写并用下划线分隔单词。

// main 函数是特殊的：它总是在每个可执行的 Rust 程序中第一个运行的代码。
fn main() {
    println!("Hello from main function!");

    // 调用另一个函数
    another_function();

    // 调用带参数的函数
    print_number(5);

    // 调用带多个参数的函数
    print_sum(10, 20);

    // 函数可以返回值
    let sum_result = add_five(10);
    println!("10 + 5 = {}", sum_result);

    // 语句和表达式
    // Rust 是一门基于表达式的语言，理解语句（statement）和表达式（expression）的区别很重要。
    // 语句是执行一些操作但不返回值的指令。
    // 表达式计算并产生一个值。

    // `let y = 6;` 是一个语句。函数定义也是语句。
    let y = 6; // 这是一个语句

    // 表达式可以作为语句的一部分。例如 `6` 在 `let y = 6;` 中是一个表达式。
    // 调用函数是一个表达式。调用宏是一个表达式。用 `{}` 创建的新作用域块是一个表达式。

    let x = 5;
    let y = {
        let x_squared = x * x;
        x_squared + 1 // 这个块的最后一行是表达式，没有分号，所以它的值会返回
    };
    println!("The value of y is: {}", y);

    // --- 作用域 ---
    // 作用域是程序中一个项（item）有效的一部分。
    // Rust 中的变量有其作用域，它们只在声明它们的代码块（通常是花括号 `{}` 内）中有效。

    let outer_scope_var = 100;
    println!("Outer scope variable: {}", outer_scope_var);

    {
        // 这是一个新的作用域
        let inner_scope_var = 200;
        println!("Inner scope variable: {}", inner_scope_var);
        println!("Outer scope variable from inner scope: {}", outer_scope_var); // 内部作用域可以访问外部作用域的变量
    }

    // println!("Inner scope variable from outer scope: {}", inner_scope_var); // 这行会报错，因为 inner_scope_var 在这里无效

    // 变量遮蔽 (Shadowing)
    // Rust 允许用相同的名字声明一个新的变量，新的变量会“遮蔽”之前声明的同名变量。
    let s = 5;
    println!("The value of s is: {}", s);
    let s = s * 2; // s 被遮蔽了
    println!("The value of s is now: {}", s);

    {
        let s = "hello"; // 在内部作用域再次遮蔽 s，类型也可以改变
        println!("The value of s in inner scope is: {}", s);
    }

    println!("The value of s after inner scope: {}", s); // 内部作用域的遮蔽结束后，s 恢复为之前的数值类型的值
}

// 定义另一个函数
fn another_function() {
    println!("Hello from another_function!");
}

// 定义带参数的函数
// 在函数签名中，必须声明每个参数的类型。
fn print_number(x: i32) {
    println!("The number is: {}", x);
}

// 定义带多个参数的函数
fn print_sum(a: i32, b: i32) {
    println!("The sum of {} and {} is: {}", a, b, a + b);
}

// 定义带返回值的函数
// 返回值的类型在箭头 `->` 之后声明。
// 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
// 你可以使用 `return` 关键字从函数中提前返回一个值，但大多数函数隐式返回最后一个表达式。
fn add_five(x: i32) -> i32 {
    x + 5 // 没有分号，这是一个表达式，其值将作为函数的返回值
    // 如果写成 `x + 5;` (带分号)，它就变成了一个语句，函数将返回 `()` (unit type)
}
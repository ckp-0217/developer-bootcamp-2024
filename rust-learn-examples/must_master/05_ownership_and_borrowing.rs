// 05_ownership_and_borrowing.rs

// 所有权是 Rust 最独特的功能，它让 Rust 无需垃圾回收器（GC）就能保证内存安全。
// 理解所有权对于编写高效且安全的 Rust 代码至关重要。

// 所有权规则：
// 1. Rust 中的每一个值都有一个被称为其所有者（owner）的变量。
// 2. 值在任一时刻有且只有一个所有者。
// 3. 当所有者（变量）离开作用域，这个值将被丢弃。

fn main() {
    // --- 变量作用域 ---
    // 作用域是变量在程序中有效的范围。
    {
        let s = "hello"; // s 从这里开始有效
        // 使用 s
        println!("s 在内部作用域: {}", s);
    } // s 的作用域到此结束，s 不再有效
    // println!("s 在外部作用域: {}", s); // 这行会报错，因为 s 已经离开作用域

    // --- String 类型 --- 
    // 为了演示所有权规则，我们需要一个比基本数据类型更复杂的数据类型。
    // String 类型是在堆上分配数据的，因此它的大小可以在运行时改变。

    let mut s1 = String::from("hello"); // s1 在堆上分配了内存
    s1.push_str(", world!"); // push_str() 方法将一个字面值追加到 String
    println!("s1: {}", s1);

    // --- 移动 (Move) ---
    // 当我们将一个 String 赋给另一个变量时，数据不会被复制，而是发生“移动”。
    let s2 = s1; // s1 的所有权被移动到 s2
    // println!("s1 在移动后: {}", s1); // 这行会报错，因为 s1 的所有权已经移动，s1 不再有效
    println!("s2 (原 s1): {}", s2);

    // 如果我们确实需要深度复制 String 的堆数据，而不仅仅是栈数据，可以使用 clone 方法。
    let s3 = String::from("original");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4); // s3 和 s4 都是有效的，因为 s4 是 s3 的一个克隆

    // --- 栈上数据的复制 (Copy) ---
    // 对于存储在栈上的简单类型（如整数、布尔值、浮点数、字符、元组（如果其元素都是可 Copy 的）），
    // 赋值操作会进行复制，而不是移动。
    // 这些类型实现了 `Copy` trait。
    let x = 5;
    let y = x; // x 的值被复制到 y
    println!("x = {}, y = {}", x, y); // x 和 y 都有效

    // --- 所有权与函数 ---
    // 将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样。

    let s_func = String::from("hello from s_func"); // s_func 进入作用域
    takes_ownership(s_func); // s_func 的值移动到函数里，所以 s_func 在这里不再有效
    // println!("s_func after takes_ownership: {}", s_func); // 这行会报错

    let x_func = 5; // x_func 进入作用域
    makes_copy(x_func); // x_func 的值被复制到函数里，但 i32 是 Copy 的，所以 x_func 仍然有效
    println!("x_func after makes_copy: {}", x_func);

    // --- 返回值与作用域 ---
    // 函数也可以将所有权返回给调用者。
    let s_ret1 = gives_ownership(); // gives_ownership 将返回值移动给 s_ret1
    println!("s_ret1: {}", s_ret1);

    let s_ret2 = String::from("hello from s_ret2"); // s_ret2 进入作用域
    let s_ret3 = takes_and_gives_back(s_ret2); // s_ret2 被移动进 takes_and_gives_back, 它也将返回值移给 s_ret3
    // println!("s_ret2 after takes_and_gives_back: {}", s_ret2); // 这行会报错
    println!("s_ret3: {}", s_ret3);

    // --- 借用 (Borrowing) 与引用 (&) ---
    // 如果我们希望函数使用一个值但不获取其所有权，可以使用引用。
    // 引用允许你引用（refer to）某个值而不取得其所有权。
    let s_borrow = String::from("hello borrow");
    let len = calculate_length(&s_borrow); // &s_borrow 创建一个指向 s_borrow 的引用，但不拥有它
    println!("The length of '{}' is {}. s_borrow is still valid: {}", s_borrow, len, s_borrow);

    // --- 可变引用 (&mut) ---
    // 引用默认是不可变的。如果你想修改引用的值，需要使用可变引用。
    let mut s_mut_borrow = String::from("hello");
    change(&mut s_mut_borrow);
    println!("s_mut_borrow after change: {}", s_mut_borrow);

    // 可变引用的规则：
    // 1. 在特定作用域内，对某一块数据，你只能有一个可变引用。
    // 2. 不能在拥有不可变引用的同时拥有可变引用。
    //    （例外：多个不可变引用是可以的）
    // 这些规则在编译时防止数据竞争。

    let mut s_rules = String::from("hello rules");

    let r1 = &s_rules; // 没问题
    let r2 = &s_rules; // 没问题
    println!("{} and {}", r1, r2);
    // r1 和 r2 在这里之后不再使用

    let r3 = &mut s_rules; // 没问题，因为 r1 和 r2 的作用域已经结束
    println!("{}", r3);

    // --- 悬垂引用 (Dangling References) ---
    // Rust 编译器保证引用永远不会指向无效的内存。
    // 如果你有一个对某数据的引用，编译器会确保数据不会在其引用存在时离开作用域。

    // let reference_to_nothing = dangle(); // 这会编译失败，因为 dangle 返回一个指向已释放内存的引用

    let no_dangle_string = no_dangle();
    println!("String from no_dangle: {}", no_dangle_string);
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("Inside takes_ownership: {}", some_string);
} // 这里，some_string 移出作用域并调用 `drop`。占用的内存被释放。

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("Inside makes_copy: {}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作。

fn gives_ownership() -> String { // gives_ownership 会将返回值移动给调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该字符串
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}

// calculate_length 函数接收一个 String 的引用作为参数
fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 移出作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生。

// change 函数接收一个可变的 String 引用
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
// dangle 函数尝试返回一个指向已释放内存的引用，这会导致编译错误
fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello dangle"); // s 进入作用域
    &s // 我们返回对 s 的引用
} // 这里 s 移出作用域，并被丢弃。它的内存被释放。
  // 危险！
*/

// no_dangle 函数通过返回值本身（而不是引用）来避免悬垂引用
fn no_dangle() -> String {
    let s = String::from("hello no dangle");
    s
}
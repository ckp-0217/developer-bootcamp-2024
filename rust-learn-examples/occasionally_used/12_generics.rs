// 12_generics.rs

// 泛型（Generics）是 Rust 中一种强大的功能，它允许我们编写灵活且可重用的代码，
// 而无需在编译时知道具体的类型。这有助于减少代码重复。

// --- 泛型函数 ---
// 我们可以定义一个函数，使其可以处理多种不同类型的数据。
// `<T>` 是类型参数声明，表示 T 是一个泛型类型。
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest { // 需要 PartialOrd trait 来比较大小
            largest = item;
        }
    }
    largest // 需要 Copy trait，因为我们将值从 list 复制到 largest 变量
}

// --- 泛型结构体 ---
// 结构体也可以使用泛型类型参数。
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// --- 泛型枚举 ---
// 枚举也可以是泛型的，例如标准库中的 Option<T> 和 Result<T, E>。
// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// --- impl 块中的泛型 ---
// 我们可以在 impl 块中为泛型结构体或枚举实现方法。
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    // 也可以为特定具体类型的泛型结构体实现方法
    // 例如，只为 Point<f32, f32> 实现 distance_from_origin 方法
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 混合不同类型的泛型参数
impl<T1, U1> Point<T1, U1> {
    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T1, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // 使用泛型函数 largest
    let number_list = vec![34, 50, 25, 100, 65];
    let result_num = largest(&number_list);
    println!("The largest number is {}", result_num);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result_char = largest(&char_list);
    println!("The largest char is {}", result_char);

    // 使用泛型结构体 Point
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let mixed_point = Point { x: 5, y: 4.0 }; // x 是 i32, y 是 f64

    println!("integer_point: {:?}", integer_point);
    println!("float_point: {:?}", float_point);
    println!("mixed_point: {:?}", mixed_point);

    // 调用泛型结构体的方法
    println!("integer_point.x = {}", integer_point.x());

    // 调用特定类型实现的方法
    let p_f32 = Point { x: 3.0f32, y: 4.0f32 };
    println!("Distance from origin for p_f32: {}", p_f32.distance_from_origin());
    // let p_i32 = Point { x:3, y:4 };
    // p_i32.distance_from_origin(); // 错误：Point<i32, i32> 没有这个方法

    // 调用 mixup 方法
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    println!("\nAnchor 合约中泛型的提示：");
    println!("Anchor 合约本身直接定义泛型指令或账户结构体的场景不多。");
    println!("但理解泛型对于阅读 Anchor 框架源码、Solana SDK 以及其他 Rust 库非常重要。");
    println!("例如，`Account<'info, T>` 就是一个泛型结构体，其中 T 是账户数据的类型。");
    println!("Context<T> 也是泛型的，T 是具体的 Accounts 结构体。");

}
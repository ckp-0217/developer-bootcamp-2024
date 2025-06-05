// 07_structs_and_methods.rs

// 结构体（struct）是一种自定义数据类型，允许你将多个相关的值组合在一起并命名它们。
// 方法（method）是与结构体（或枚举、trait）相关联的函数。

// 定义一个结构体来表示一个矩形
#[derive(Debug)] // 派生 Debug trait 以便能够打印结构体实例
struct Rectangle {
    width: u32,
    height: u32,
}

// 为 Rectangle 结构体定义方法
// impl 块（implementation block）用于定义与类型相关联的函数和方法。
impl Rectangle {
    // 方法：计算矩形的面积
    // 方法的第一个参数总是 self，它代表调用该方法的结构体实例。
    // &self 表示该方法借用了结构体实例的不可变引用。
    // &mut self 表示可变引用。
    // self 表示获取实例的所有权。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法：检查一个矩形是否能容纳另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数 (Associated Function)
    // 关联函数不以 self 作为第一个参数（因此不是方法）。
    // 它们通常用作构造函数，返回类型的新实例。
    // 通过 `StructName::function_name()` 的方式调用。
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // 创建 Rectangle 结构体的实例
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1); // 使用 Debug trait 打印
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // 调用 area 方法
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 使用关联函数创建正方形实例
    let sq = Rectangle::square(25);
    println!("Square sq is {:?}", sq);
    println!("Area of square sq is {}", sq.area());

    // 字段初始化简写语法
    // 当变量名与字段名相同时，可以使用简写语法。
    let width = 100;
    let height = 200;
    let rect_shorthand = Rectangle {
        width, // 等同于 width: width,
        height, // 等同于 height: height,
    };
    println!("rect_shorthand area: {}", rect_shorthand.area());

    // 结构体更新语法
    // 当你想要基于现有实例创建一个新实例，但只改变其中一些字段的值时，可以使用结构体更新语法。
    let rect_base = Rectangle {
        width: 50,
        height: 50,
    };

    let rect_updated = Rectangle {
        height: 75,
        ..rect_base // `..` 语法指定未显式设置的其余字段应与给定实例中的字段具有相同的值。
    };
    println!("rect_updated: {:?}", rect_updated);
    // 注意：结构体更新语法像赋值语句一样使用 `=`，因此它会移动数据。
    // 如果 rect_base 中的字段实现了 Copy trait，则它们会被复制；否则，它们会被移动。
    // 在这个例子中，u32 实现了 Copy，所以 rect_base 仍然有效。
    println!("rect_base after update syntax: {:?}", rect_base);

    // 元组结构体 (Tuple Structs)
    // 元组结构体是一种定义类似元组的结构体的方式，但它们有名称，使其成为独立的类型。
    // 当你想给整个元组一个名称，并且使元组与其他元组不同，而命名每个字段又显得冗余或不必要时，元组结构体很有用。
    struct Color(i32, i32, i32); // RGB 颜色
    struct Point(i32, i32, i32); // 3D 空间中的点

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);
    // black 和 origin 是不同类型的值，因为它们是不同元组结构体的实例。

    // 单元结构体 (Unit-Like Structs)
    // 你也可以定义没有任何字段的结构体！它们被称为单元结构体，因为它们类似于 ()，即单元类型。
    // 当你需要在某个类型上实现 trait 但不需要在该类型中存储任何数据时，单元结构体非常有用。
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // 我们可能稍后为 AlwaysEqual 实现某个行为，比如一个总是返回 true 的比较 trait。

    // --- 结构体在 Anchor 开发中的应用 ---
    // 在 Solana Anchor 开发中，结构体扮演着至关重要的角色：
    // 1. 定义账户数据 (Account Data): 你会使用带有 `#[account]` 宏的结构体来定义存储在 Solana 账户中的数据格式。
    //    例如: `#[account] pub struct MyAccount { data: u64, owner: Pubkey }`
    // 2. 定义指令的账户上下文 (Accounts Structs): 使用 `#[derive(Accounts)]` 的结构体来声明一个指令需要访问的所有账户及其约束。
    //    例如: `#[derive(Accounts)] pub struct Initialize<'info> { ... }`
    // 理解 Rust 结构体是掌握 Anchor 开发的基础。
}
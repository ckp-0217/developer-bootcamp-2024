// 10_modules_and_packages.rs

// Rust 使用模块（module）系统来组织代码，并通过包（package）和 crate 来管理和分发代码。
// 一个 crate 是 Rust 的编译单元，它可以生成一个库或一个可执行文件。
// 一个包（package）是一个或多个 crate 的集合，它包含一个 Cargo.toml 文件，描述了如何构建这些 crate。

// --- 模块定义 ---
// 模块允许你将相关的代码组织在一起，并控制其可见性（公有或私有）。
// 你可以使用 `mod` 关键字定义模块。

mod front_of_house {
    // 模块默认是私有的。pub 关键字使其公有。
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist in front_of_house::hosting");
            // 可以调用同模块下的其他函数
            seat_at_table();
        }

        fn seat_at_table() {
            println!("Seated at table in front_of_house::hosting");
        }
    }

    mod serving {
        fn take_order() {
            println!("Took order in front_of_house::serving");
        }

        fn serve_order() {
            println!("Served order in front_of_house::serving");
        }

        fn take_payment() {
            println!("Took payment in front_of_house::serving");
        }
    }
}

// --- 路径 (Paths) --- 
// 路径用于引用模块中的项（函数、结构体、枚举等）。
// 绝对路径：从 crate 根开始，使用 crate 名或字面值 `crate`。
// 相对路径：从当前模块开始，使用 `self`、`super` 或当前模块中的标识符。

pub fn eat_at_restaurant_absolute() {
    println!("\nEating at restaurant (absolute path):");
    // 使用绝对路径调用
    crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist(); // 如果 front_of_house 在当前 crate 根，也可以这样
}

mod customer {
    pub fn eat_at_restaurant_relative() {
        println!("\nEating at restaurant (relative path from customer module):");
        // 使用相对路径调用，从 customer 模块出发
        // super::front_of_house::hosting::add_to_waitlist(); // 使用 super 指向父模块
        // 或者，如果 front_of_house 在 crate 根
        crate::front_of_house::hosting::add_to_waitlist();
    }
}

// --- `use` 关键字 --- 
// `use` 关键字可以将路径引入作用域，从而避免重复写出长路径。
use crate::front_of_house::hosting;
// use self::front_of_house::hosting; // 另一种写法，如果 front_of_house 在当前模块

pub fn eat_at_restaurant_with_use() {
    println!("\nEating at restaurant (with use):");
    hosting::add_to_waitlist();
    hosting::add_to_waitlist(); // 可以多次调用
}

// 习惯上，引入函数时，通常指定到其父模块，然后在调用时指定父模块。
// 引入结构体、枚举和其他项时，习惯是指定完整路径。
// use crate::front_of_house::hosting::add_to_waitlist; // 也可以直接引入函数

// --- `pub use` 重导出 --- 
// 使用 `pub use` 可以将一个项引入作用域，并使其在外部代码中也可用，就好像它是在当前模块中定义的一样。
// 这对于创建库的公有 API 非常有用。
mod kitchen {
    pub fn cook_order() {
        println!("Order cooked in kitchen!");
    }
    pub struct Appetizer {
        pub name: String,
    }
}

// 将 kitchen::cook_order 重导出到 my_library 模块的顶层
pub use self::kitchen::cook_order as cook;
pub use self::kitchen::Appetizer;

// --- 模块可以嵌套 --- 
mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order(); // 调用同级模块的函数
        super::front_of_house::serving::serve_order(); // 调用父模块下的兄弟模块的函数 (假设 serving 是 pub)
    }

    fn cook_order() {
        println!("Order re-cooked in back_of_house");
    }

    // 结构体字段默认是私有的，需要 pub 来使其公有
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,     // 这个字段是公有的
        seasonal_fruit: String, // 这个字段是私有的
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 如果枚举是公有的，它的所有变体也都是公有的
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    eat_at_restaurant_absolute();
    customer::eat_at_restaurant_relative();
    eat_at_restaurant_with_use();

    // 使用重导出的项
    println!("\nUsing re-exported items:");
    cook(); // 调用重导出的 cook_order
    let appetizer = Appetizer { name: String::from("Fries") };
    println!("Appetizer: {}", appetizer.name);


    // 访问模块中的结构体
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 修改公有字段
    meal.toast = String::from("Wheat");
    println!("\nI'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // 错误！seasonal_fruit 是私有的
    // println!("Seasonal fruit: {}", meal.seasonal_fruit); // 错误！

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("Order 1: {:?}, Order 2: {:?}", order1, order2); // 需要为 Appetizer 派生 Debug

    println!("\nCargo 和 Crates.io 提示：");
    println!("Cargo 是 Rust 的构建系统和包管理器。");
    println!("  `cargo new project_name` 创建新项目。");
    println!("  `cargo build` 构建项目。");
    println!("  `cargo run` 构建并运行项目。");
    println!("  `cargo test` 运行测试。");
    println!("  `cargo doc --open` 构建文档并在浏览器中打开。");
    println!("  `Cargo.toml` 文件用于配置包，包括依赖项。");
    println!("Crates.io 是 Rust 社区的 crate 仓库，可以从中获取第三方库。");
    println!("在 Anchor 合约中，`Anchor.toml` 和 `Cargo.toml` 用于管理程序和依赖。");
    println!("  `programs/my_program/src/lib.rs` 是合约代码的主要位置。");
    println!("  模块用于组织合约的不同部分，例如指令处理函数、账户结构体、错误定义等。");

}

// 如果模块在单独的文件中，可以使用 `mod file_name;` 来声明它。
// 例如，如果 front_of_house 在 src/front_of_house.rs 文件中，则在 lib.rs 或 main.rs 中：
// mod front_of_house;
// 然后 front_of_house.rs 文件内容就是模块的内容。
// 如果是 front_of_house/hosting.rs，则在 front_of_house.rs 中 `pub mod hosting;`
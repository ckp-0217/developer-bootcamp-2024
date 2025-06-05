// 14_lifetimes.rs

// 生命周期（Lifetimes）是 Rust 用来确保引用总是有效的一种机制。
// 大多数情况下，生命周期是隐式的并且可以被推断出来，就像类型推断一样。
// 但是，当引用的生命周期可能以多种方式相关联时，Rust 需要你使用泛型生命周期参数来显式注解关系，以便编译器可以确保实际使用的引用绝对是有效的。

// 函数返回一个引用，其生命周期与传入的某个引用的生命周期相关联
// 'a 是一个生命周期参数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体定义中的生命周期注解
// 这个结构体持有一个字符串切片的引用，所以它的定义需要一个生命周期参数。
// 这意味着 ImportantExcerpt 的实例不能比它在 part 字段中保存的引用活得更久。
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 方法定义中的生命周期注解
impl<'a> ImportantExcerpt<'a> {
    // 第一个 elision 规则：每个输入引用参数都有自己的生命周期参数。
    // 第二个 elision 规则：如果只有一个输入生命周期参数，那么该生命周期被赋给所有输出生命周期参数。
    // 第三个 elision 规则：如果方法有多个输入生命周期参数，但其中一个是 &self 或 &mut self，
    // 那么 self 的生命周期被赋给所有输出生命周期参数。
    fn level(&self) -> i32 {
        3
    }

    // 这里需要显式生命周期，因为 announce_and_return_part 的返回值生命周期与 self.part 的生命周期相关，
    // 而不是与 &self 的生命周期相关。
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 静态生命周期 ('static)
// 'static 生命周期表示引用可以在程序的整个持续时间内有效。
// 所有字符串字面量都拥有 'static 生命周期。
let s_static: &'static str = "I have a static lifetime.";

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // println!("The longest string is {}", result); // 这行会报错，因为 string2 的生命周期在这里已经结束，而 result 引用了 string2

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("ImportantExcerpt: {:?}", i);
    println!("Level: {}", i.level());
    println!("Part: {}", i.announce_and_return_part("Urgent announcement!"));

    println!("Static string: {}", s_static);

    // 泛型、Trait Bound 和生命周期一起使用
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let s1 = "abcd";
    let s2 = "efghijkl";
    let announcement = "Today's longest string is:";
    let result_with_ann = longest_with_an_announcement(s1, s2, announcement);
    println!("{}", result_with_ann);

    // --- 生命周期在 Anchor 开发中的应用 ---
    // 在 Solana Anchor 开发中，生命周期参数（特别是 `'info`）在定义账户上下文时非常重要。
    //
    // 1. `AccountInfo` 和 `'info` 生命周期：
    //    Solana 程序直接与 `AccountInfo` 结构体交互，这些结构体包含账户的底层数据和元数据。
    //    `AccountInfo` 本身带有一个生命周期参数，通常表示为 `'a` 或 `'info`。
    //    这个生命周期参数确保了 `AccountInfo` 引用的数据（如账户的 lamports 或 data slice）
    //    在指令执行期间是有效的。
    //
    // 2. Anchor 的 `Accounts` 结构体：
    //    当你使用 `#[derive(Accounts)]` 定义一个结构体时，例如 `MyAccounts<'info>`，
    //    这个 `'info` 生命周期会被传递给内部包装的账户类型，如 `Account<'info, MyData>` 或 `Signer<'info>`。
    //    `Account<'info, T>` 是 Anchor 提供的一个包装器，它反序列化 `AccountInfo<'info>` 的数据到类型 `T` 中。
    //    `Signer<'info>` 包装了 `AccountInfo<'info>` 并确保它是交易的签名者。
    //
    //    示例 (通常在 lib.rs 中定义):
    //    // use anchor_lang::prelude::*;
    //    // #[account]
    //    // pub struct MyData { pub data: u64 }
    //    // 
    //    // #[derive(Accounts)]
    //    // pub struct UpdateContext<'info> {
    //    //     #[account(mut)]
    //    //     pub my_account: Account<'info, MyData>, // 'info 生命周期传递给 Account
    //    //     pub user: Signer<'info>,               // 'info 生命周期传递给 Signer
    //    // }
    //
    // 3. 为什么重要？
    //    `'info` 生命周期帮助 Rust 编译器在编译时验证所有账户引用都是安全的，
    //    防止了悬垂引用等问题，这对于编写安全可靠的智能合约至关重要。
    //    它确保了在你的指令函数中访问账户数据时，这些数据仍然存在并且是有效的。
    //
    // 总结：`'info` 是 Anchor 中处理账户数据安全性的核心机制之一，它源于 Solana 对 `AccountInfo` 的生命周期管理。
    println!("\n--- 生命周期在 Anchor 开发中的应用 ---");
    println!("在 Anchor 中，账户上下文结构体 (如 `MyAccounts<'info>`) 强制使用 `'info` 生命周期。");
    println!("这个 `'info` 生命周期与 Solana 运行时提供的 `AccountInfo<'info>` 相关联。");
    println!("它确保了指令执行期间对账户数据的所有引用都是有效的，防止悬垂引用。");
    println!("例如: `Account<'info, MyData>` 或 `Signer<'info>` 都携带这个生命周期。");
    println!("这是 Anchor 和 Rust 保证链上程序安全性的一个重要方面。");
}
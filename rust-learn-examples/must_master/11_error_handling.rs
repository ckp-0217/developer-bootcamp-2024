// 11_error_handling.rs

// Rust 使用 Result<T, E> 枚举来处理可能失败的操作。
// T 是成功时返回的值的类型，E 是失败时返回的错误的类型。
// Option<T> 枚举用于表示一个值可能存在（Some(value)）或不存在（None）的情况。

use std::fs::File;
use std::io::{self, Read};

// 函数返回 Result<T, E>
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 提前返回错误
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // 成功读取，返回 Ok 包裹的字符串
        Err(e) => Err(e), // 读取失败，返回 Err 包裹的错误
    }
}

// 使用 `?` 运算符简化错误处理
// `?` 运算符只能用于返回 Result 或 Option（或其他实现了 FromResidual 的类型）的函数。
// 如果 Result 的值是 Ok，`?` 会从中提取值并继续执行。
// 如果 Result 的值是 Err，`?` 会立即返回整个 Err 给调用者。
fn read_username_from_file_simplified() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 更简洁的写法
fn read_username_from_file_even_more_simplified() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("another_username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 标准库甚至提供了更短的方式 (std::fs::read_to_string)
// fn read_username_from_file_shortest() -> Result<String, io::Error> {
//     std::fs::read_to_string("shortest_username.txt")
// }

// 处理 Option<T> 类型
fn find_first_a(s: &str) -> Option<usize> {
    s.chars().position(|c| c == 'a')
}

fn main() {
    // --- Result<T, E> --- 
    // 尝试读取文件，这可能会失败
    match read_username_from_file() {
        Ok(username) => println!("Username from hello.txt: {}", username),
        Err(error) => println!("Error reading from hello.txt: {}", error),
    }

    match read_username_from_file_simplified() {
        Ok(username) => println!("Username from username.txt: {}", username),
        Err(error) => println!("Error reading from username.txt: {}", error),
    }

    match read_username_from_file_even_more_simplified() {
        Ok(username) => println!("Username from another_username.txt: {}", username),
        Err(error) => println!("Error reading from another_username.txt: {}", error),
    }

    // --- Option<T> --- 
    let text1 = "banana";
    let text2 = "rust";

    match find_first_a(text1) {
        Some(index) => println!("Found 'a' in '{}' at index: {}", text1, index),
        None => println!("'a' not found in '{}'", text1),
    }

    match find_first_a(text2) {
        Some(index) => println!("Found 'a' in '{}' at index: {}", text2, index),
        None => println!("'a' not found in '{}'", text2),
    }

    // 使用 unwrap 和 expect
    // unwrap()：如果 Option 是 Some，返回内部的值；如果是 None，则 panic。
    // expect(msg)：类似于 unwrap，但允许你提供一个自定义的 panic 消息。
    // 在生产代码中应谨慎使用 unwrap 和 expect，它们主要用于原型开发或你确定 Option 不会是 None 的情况。

    // let an_option: Option<i32> = None;
    // let value = an_option.unwrap(); // 这会 panic!
    // let value_expect = an_option.expect("Option was None, this is bad!"); // 这也会 panic!

    let some_value = Some(10);
    println!("Unwrapped some_value: {}", some_value.unwrap());
    println!("Expected some_value: {}", some_value.expect("This should not happen"));

    // --- Anchor 合约中的错误处理 ---
    // Anchor 提供了更结构化的方式来处理程序错误。
    // 指令处理函数通常返回 `Result<()>` (或 `anchor_lang::ProgramResult`)。
    // 错误被定义为带有 `#[error_code]` 宏的枚举。
    // 每个错误变体可以关联一个自定义的错误消息，通过 `#[msg("...")]` 属性指定。

    // 示例 (通常在 lib.rs 或专门的 error.rs 文件中定义):
    // ```rust
    // // 假设在你的 Anchor 项目的 lib.rs 或 error.rs 中
    // use anchor_lang::prelude::*;
    // 
    // // ... 其他程序代码 ...
    // 
    // // 指令函数示例
    // // pub fn do_something(ctx: Context<DoSomethingAccounts>, input_value: u64) -> Result<()> {
    // //     if input_value == 0 {
    // //         return err!(MyProgramError::InputValueCannotBeZero);
    // //     }
    // //     if input_value > 100 {
    // //         return err!(MyProgramError::InputValueTooLarge);
    // //     }
    // //     // ... 成功逻辑 ...
    // //     msg!("Successfully processed input: {}", input_value);
    // //     Ok(())
    // // }
    // 
    // // 自定义错误枚举
    // #[error_code]
    // pub enum MyProgramError {
    //     #[msg("Input value cannot be zero.")] // 错误码 6000 (默认从6000开始，或自定义)
    //     InputValueCannotBeZero,
    // 
    //     #[msg("Input value is too large. Maximum allowed is 100.")] // 错误码 6001
    //     InputValueTooLarge,
    // 
    //     #[msg("Unauthorized access attempt.")] // 错误码 6002
    //     Unauthorized,
    // }
    // 
    // // 账户上下文结构体 (仅为示例完整性)
    // // #[derive(Accounts)]
    // // pub struct DoSomethingAccounts<'info> {
    // //     // ... 账户定义 ...
    // //     pub user: Signer<'info>,
    // // }
    // ```

    println!("\n--- Anchor 合约错误处理详解 ---");
    println!("1. 指令函数返回类型: `Result<()>` 或 `anchor_lang::ProgramResult`.");
    println!("2. 自定义错误: 使用 `#[error_code]` 宏定义一个枚举 (e.g., `MyProgramError`).");
    println!("3. 错误消息: 每个错误变体可以使用 `#[msg(\"Your message\")]` 提供描述性信息。");
    println!("4. 返回错误: 在指令函数中，使用 `err!(MyProgramError::VariantName)` 来返回一个错误。");
    println!("5. 错误码: Anchor 会为每个错误变体自动分配一个唯一的错误码 (通常从6000开始)。");
    println!("这种方式使得错误更易于管理、测试，并且客户端可以更好地解析程序错误。");
}
// 16_anchor_core_concepts.rs
// 介绍 Solana Anchor 框架的核心概念和常用宏
// Anchor 简化了 Solana 程序的开发，提供了许多便利的宏和约定。

// 引入 Anchor 相关的包，这在实际的 Anchor 项目中是必需的
// use anchor_lang::prelude::*;

// --- 声明程序 ID --- 
// 在实际的 Anchor 项目中，你需要声明你的程序 ID
// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); // 示例程序ID，请替换为你自己的

// --- #[program] 宏 --- 
// `#[program]` 宏标记一个模块，其中包含你的 Solana 指令处理函数。
// 这个模块通常命名为 `lib` 或者你的项目名。

// 假设我们有一个模块 `my_anchor_program`
// #[program]
// pub mod my_anchor_program {
//     use super::*; // 导入外部定义的 Accounts 结构体等

//     // --- 指令处理函数 --- 
//     // 指令函数通常以 `Context<T>` 作为第一个参数，其中 T 是一个定义了所需账户的结构体。
//     // 指令函数返回 `Result<()>` 或 `anchor_lang::ProgramResult`。
//     pub fn initialize(ctx: Context<InitializeAccounts>, data: u64) -> Result<()> {
//         msg!("Instruction: Initialize called with data: {}", data);

//         // 从 Context 中访问账户
//         let my_account = &mut ctx.accounts.my_account;
//         my_account.data = data;
//         my_account.authority = *ctx.accounts.user.key;

//         msg!("MyAccount initialized with data: {} and authority: {}", my_account.data, my_account.authority);
//         Ok(())
//     }

//     pub fn update_data(ctx: Context<UpdateDataAccounts>, new_data: u64) -> Result<()> {
//         msg!("Instruction: UpdateData called with new_data: {}", new_data);

//         // 权限校验：只有 my_account 的 authority 才能更新数据
//         if *ctx.accounts.user.key != ctx.accounts.my_account.authority {
//             // return Err(ProgramError::IllegalOwner.into()); // 旧版方式
//             return err!(MyAnchorError::Unauthorized); // 使用 Anchor 自定义错误
//         }

//         ctx.accounts.my_account.data = new_data;
//         msg!("MyAccount data updated to: {}", new_data);
//         Ok(())
//     }
// }

// --- #[derive(Accounts)] 宏 --- 
// `#[derive(Accounts)]` 用于为一个结构体自动实现 Anchor 所需的 traits，
// 这个结构体用于定义指令需要访问的账户列表和约束。

// // 为 `initialize` 指令定义的账户结构体
// #[derive(Accounts)]
// pub struct InitializeAccounts<'info> {
//     // --- #[account(...)] 宏 --- 
//     // `#[account(...)]` 用于定义账户的属性和约束。
//     #[account(
//         init, // `init` 表示这个指令会创建并初始化这个账户
//         payer = user, // `payer` 指定创建账户所需租金的支付者
//         space = 8 + 8 + 32, // `space` 定义账户需要分配的字节空间 (discriminator + u64 + Pubkey)
//         // seeds = [b"my_account_seed", user.key().as_ref()], // 可选：用于 PDA (Program Derived Address)
//         // bump // 可选：用于 PDA，存储计算得到的 bump seed
//     )]
//     pub my_account: Account<'info, MyAccountData>,

//     #[account(mut)] // `mut` 表示这个账户在指令中会被修改
//     pub user: Signer<'info>, // `Signer` 表示这个账户必须是交易的签名者

//     pub system_program: Program<'info, System>, // `System` 程序，创建账户时需要
// }

// // 为 `update_data` 指令定义的账户结构体
// #[derive(Accounts)]
// pub struct UpdateDataAccounts<'info> {
//     #[account(mut, has_one = authority)] // `mut` 表示账户可修改, `has_one = authority` 约束 my_account.authority == authority.key()
//     pub my_account: Account<'info, MyAccountData>,
    
//     // `authority` 字段名需要与 `MyAccountData` 中的 `authority` 字段名对应，
//     // 或者通过 `#[account(address = my_account.authority)]` 显式指定。
//     // Signer<'info> 意味着这个 `authority` 账户必须是交易的签名者。
//     // 注意：这里的 `authority` 账户本身并不一定需要是 `Signer`，
//     // 真正的签名者是 `user`，通过 `my_account.authority == *ctx.accounts.user.key` 进行校验。
//     // 如果 `authority` 字段本身就是签名者，则可以直接用 `pub authority: Signer<'info>`
//     // 并在 `has_one` 中使用它。
//     // 更常见的做法是直接让 `user` 作为 `Signer` 参与校验。
//     pub user: Signer<'info>, // 交易的签名者，用于校验权限
//     // pub authority: Signer<'info>, // 如果 authority 字段本身就是签名者
// }

// --- 账户数据结构体 --- 
// 这是实际存储在 Solana 账户中的数据结构。
// 通常会派生 `AnchorSerialize`, `AnchorDeserialize`, `Clone`, `Default` 等 traits。
// #[account]
// pub struct MyAccountData {
//     pub data: u64,
//     pub authority: Pubkey, // 存储管理这个账户的公钥
// }

// --- 自定义错误处理 --- 
// Anchor 使用 `#[error_code]` 宏来定义自定义错误枚举。
// #[error_code]
// pub enum MyAnchorError {
//     #[msg("You are not authorized to perform this action.")]
//     Unauthorized,
//     #[msg("Data value cannot be zero.")]
//     DataCannotBeZero,
// }

// --- msg! 宏 --- 
// `msg!("...")` 用于在 Solana 程序日志中打印消息，方便调试。
// 示例已在上面的指令函数中使用。

// --- Program Derived Addresses (PDAs) --- 
// PDA 是由程序 ID 和一组种子（seeds）唯一确定的地址，它没有私钥。
// Anchor 提供了便利的方式来处理 PDA。
// 示例：在 `InitializeAccounts` 中的 `my_account` 使用 `seeds` 和 `bump`。
// `seeds = [b"some_prefix", some_variable.key().as_ref()]`
// `bump` 会自动填充。
// 在客户端，可以使用 `PublicKey.findProgramAddressSync` (或 async 版本) 来找到 PDA。

fn main() {
    println!("This file explains core Anchor concepts.");
    println!("To use these concepts, you would typically create an Anchor project using `anchor init <project_name>`.");
    println!("The actual Anchor code (like #[program], #[account], etc.) needs the Anchor framework and Solana runtime to compile and run.");
    println!("The commented-out sections above provide a template for how these macros are used.");

    println!("\nKey Anchor Macros and Concepts Covered (in comments):");
    println!("- `declare_id!`: To declare your program's on-chain ID.");
    println!("- `#[program]`: Marks the module containing your instruction handlers.");
    println!("- Instruction Functions: Logic for your program, taking a `Context<T>`.");
    println!("- `Context<T>`: Provides access to accounts and program state for an instruction.");
    println!("- `#[derive(Accounts)]`: Defines the accounts structure for an instruction.");
    println!("- `#[account(...)]`: Specifies constraints and properties for individual accounts within an `Accounts` struct (e.g., `init`, `mut`, `payer`, `space`, `seeds`, `has_one`).");
    println!("- `Account<'info, T>`: Wrapper type for deserialized account data.");
    println!("- `Signer<'info>`: Represents an account that must have signed the transaction.");
    println!("- `Program<'info, T>`: Represents another Solana program.");
    println!("- Account Data Structs: Plain Rust structs (often decorated with `#[account]`) that define the data stored on-chain.");
    println!("- `msg!`: For logging messages from your program.");
    println!("- `#[error_code]`: For defining custom error types for your program.");
    println!("- `err!(ErrorCode::Variant)`: For returning custom errors.");
    println!("- Program Derived Addresses (PDAs): Accounts controlled by your program, derived from seeds.");

    // 实际的 Anchor 项目结构通常如下：
    // my_project/
    // |- Anchor.toml      (Project configuration)
    // |- Cargo.toml       (Rust dependencies)
    // |- programs/
    //    |- my_project/
    //        |- src/
    //            |- lib.rs (Main program logic with #[program], Accounts structs, etc.)
    // |- tests/
    //    |- my_project.ts (Client-side tests in TypeScript/JavaScript)
    // |- migrations/
    //    |- deploy.js
}

// 注意：
// 要实际编译和运行 Anchor 代码，你需要：
// 1. 安装 Rust 和 Solana tool suite.
// 2. 安装 Anchor CLI (`avm install latest`, `avm use latest`).
// 3. 创建一个新的 Anchor 项目 (`anchor init my_project`).
// 4. 在 `programs/my_project/src/lib.rs` 中编写你的代码。
// 5. 使用 `anchor build`, `anchor deploy`, `anchor test` 等命令。
// 这个文件中的代码片段是为了教学目的，展示宏的用法和概念，而不是一个可直接编译的独立程序。
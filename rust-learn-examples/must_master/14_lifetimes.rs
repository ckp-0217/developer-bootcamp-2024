// 14_lifetimes.rs

// 生命周期（Lifetimes）是 Rust 用来确保引用总是有效的一种机制。
// 大多数情况下，生命周期是隐式的并且可以被推断出来，就像类型推断一样。
// 但是，当引用的生命周期可能以多种方式相关联时，Rust 需要你使用泛型生命周期参数来显式注解关系。

// 生命周期标识符介绍：
// 1. 生命周期标识符以单引号（'）开头，后跟一个标识符名称，如 'a, 'b, 'static 等
// 2. 常见的生命周期标识符：
//    - 'a, 'b, 'c: 通用生命周期参数，类似于泛型类型参数 T, U, V
//    - 'static: 特殊生命周期，表示引用在整个程序运行期间都有效
//    - 'self: 表示与 self 相关的生命周期
//    - 'info: 在 Solana/Anchor 中常用，表示与账户信息相关的生命周期
// 3. 生命周期标识符的常见位置：
//    - 函数签名中：fn example<'a>(x: &'a str) -> &'a str
//    - 结构体定义中：struct Example<'a> { field: &'a str }
//    - impl 块中：impl<'a> Example<'a> { ... }
//    - trait 定义和实现中：trait MyTrait<'a> { ... }

// 基础部分：生命周期的基本概念和用法
// ===================================

// 1. 函数中的生命周期注解
// 这个函数返回一个引用，其生命周期与传入的两个引用的生命周期相关联
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 2. 结构体中的生命周期注解
// 当结构体持有引用时，必须为每个引用添加生命周期注解
#[derive(Debug)]
struct ImportantExcerpt<'info> {
    part: &'info str,
}

// 3. 结构体方法中的生命周期注解
impl<'a> ImportantExcerpt<'a> {
    // 简单方法，返回值不是引用，不需要关注生命周期
    fn level(&self) -> i32 {
        3
    }

    // 返回引用的方法，需要考虑生命周期
    // 这里生命周期省略规则自动应用：返回值的生命周期与self相同
    fn get_part(&self) -> &str {
        self.part
    }

    // 多个引用参数的方法
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("公告: {}", announcement);
        self.part
    }
}

// 4. 静态生命周期
// 'static 表示引用在整个程序运行期间都有效
// 所有字符串字面量都有 'static 生命周期
fn get_static_str() -> &'static str {
    "我有一个静态生命周期"
}

// 5. 生命周期与泛型、Trait Bounds 结合使用
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("公告! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Solana/Anchor 相关部分
// =====================

// 模拟 Anchor 中的账户结构
// 在实际 Anchor 程序中，这些类型由框架提供
struct AccountInfo<'info> {
    // 账户的关键信息
    key: &'info [u8; 32],
    // 账户中的 lamports 余额
    lamports: &'info mut u64,
    // 账户数据
    data: &'info mut [u8],
    // 其他字段省略...
}

// 模拟 Anchor 的 Account 包装器
struct Account<'info, T> {
    account_info: &'info AccountInfo<'info>,
    data: T,
}

// 模拟 Anchor 的 Signer 包装器
struct Signer<'info> {
    account_info: &'info AccountInfo<'info>,
}

// 模拟 Anchor 的账户数据结构
#[derive(Debug)]
struct UserAccount {
    balance: u64,
    name: String,
}

// 模拟 Anchor 的 Context 结构
struct Context<T> {  // 移除未使用的 'info 生命周期参数
    accounts: T,
    // 其他字段省略...
}

// 模拟 Anchor 的 #[derive(Accounts)] 生成的结构
struct TransferAccounts<'info> {
    // 付款人账户，必须是交易签名者
    payer: Signer<'info>,
    // 付款人的代币账户，需要修改
    // #[account(mut)]  // 移除不存在的属性
    from: Account<'info, UserAccount>,
    // 接收人的代币账户，需要修改
    // #[account(mut)]  // 移除不存在的属性
    to: Account<'info, UserAccount>,
}

// 模拟 Anchor 指令处理函数
fn process_transfer<'info>(
    mut ctx: Context<TransferAccounts<'info>>,  // 添加 mut 关键字
    amount: u64,
) -> Result<(), &'static str> {
    // 检查余额
    if ctx.accounts.from.data.balance < amount {
        return Err("余额不足");
    }
    
    // 执行转账
    ctx.accounts.from.data.balance -= amount;
    ctx.accounts.to.data.balance += amount;
    
    Ok(())
}

fn main() {
    println!("=== 基础生命周期示例 ===");
    
    // 示例1: 基本的生命周期函数使用
    let string1 = String::from("长字符串很长");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("最长的字符串是: {}", result);
        // 这里使用 result 是安全的，因为 string1 和 string2 都还有效
    }
    // 下面的代码会导致编译错误，因为 string2 已经不再有效
    // println!("最长的字符串是: {}", result);
    
    // 示例2: 结构体中的生命周期
    let novel = String::from("请叫我伊斯梅尔。几年后...");
    let first_sentence = novel.split('.').next().expect("找不到句号");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("摘录: {:?}", excerpt);
    println!("级别: {}", excerpt.level());
    println!("部分内容: {}", excerpt.get_part());
    println!("公告后返回部分: {}", excerpt.announce_and_return_part("重要公告!"));
    
    // 示例3: 静态生命周期
    let static_str = get_static_str();
    println!("静态字符串: {}", static_str);
    
    // 示例4: 结合泛型和生命周期
    let s1 = "abcd";
    let s2 = "efghijkl";
    let announcement = "今天最长的字符串是:";
    let result_with_ann = longest_with_announcement(s1, s2, announcement);
    println!("{}", result_with_ann);
    
    println!("\n=== Solana/Anchor 生命周期应用 ===");
    println!("在 Anchor 程序中，'info 生命周期是核心概念:");
    println!("1. 它确保所有账户引用在指令执行期间保持有效");
    println!("2. 它从 Solana 运行时传递到你的账户结构和指令处理函数");
    println!("3. 典型用法: Account<'info, MyData>, Signer<'info>");
    println!("4. Context<'info, MyAccounts<'info>> 包含所有指令账户");
    
    println!("\n实际 Anchor 程序中的生命周期应用示例:");
    println!(r##"
    // 账户数据结构
    #[account]
    pub struct TokenAccount {{
        pub owner: Pubkey,
        pub balance: u64,
    }}
    
    // 账户上下文结构 - 注意 'info 生命周期
    #[derive(Accounts)]
    pub struct Transfer<'info> {{
        #[account(mut, has_one = owner)]
        pub from: Account<'info, TokenAccount>,
        #[account(mut)]
        pub to: Account<'info, TokenAccount>,
        #[account(signer)]
        pub owner: AccountInfo<'info>,
    }}
    
    // 指令处理函数
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {{
        // 安全地访问账户，因为 'info 确保所有引用在此函数执行期间有效
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;
        
        // 执行转账逻辑
        from.balance = from.balance.checked_sub(amount)?;
        to.balance = to.balance.checked_add(amount)?;
        
        Ok(())
    }}
    "##);
}
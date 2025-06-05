// 01_variable_and_mutability.rs
// 变量和可变性示例
// Rust 变量默认不可变，使用 mut 关键字声明可变变量
// 在 Solana/Anchor 合约开发中，经常需要可变变量来处理账户数据，例如修改账户余额、更新状态等。

fn main() {
    // 不可变变量，不能被修改
    let x = 10;
    println!("x 的初始值: {}", x);
    // x = 20; // 这一行会报错，因为 x 是不可变的

    // 可变变量，使用 mut 关键字
    let mut y: i64 = 5;
    println!("y 的初始值: {}", y);
    y = 15; // 可以修改
    println!("y 修改后的值: {}", y);

    // 变量遮蔽(Shadowing)示例 - 在Solana合约中很有用
    let token_amount = "-1000";
    println!("代币数量(字符串): {}", token_amount);

    // 同名变量遮蔽前一个变量，可以改变类型
    let mut token_amount = token_amount.parse::<i64>().unwrap();
    println!("代币数量(数字): {}", token_amount);
    token_amount = 100;
    println!("代币数量(数字): {}", token_amount);

    // 常量示例 - 在Solana合约中定义固定值
    const LAMPORTS_PER_SOL: u64 = 1_000_000_000; // 1 SOL = 10^9 lamports
    println!("1 SOL 等于 {} lamports", LAMPORTS_PER_SOL);

    // 元组示例 - 在Solana合约中可用于返回多个值
    let account_info = (100, true, "active");
    println!(
        "账户余额: {}, 是否已初始化: {}, 状态: {}",
        account_info.0, account_info.1, account_info.2
    );

    // 可变元组
    let mut user_data = (1, "Alice", 500);
    println!(
        "用户ID: {}, 名称: {}, 余额: {}",
        user_data.0, user_data.1, user_data.2
    );

    // 修改元组中的值
    user_data.2 = 650;
    println!(
        "用户ID: {}, 名称: {}, 更新后余额: {}",
        user_data.0, user_data.1, user_data.2
    );

    // 数组示例 - 在Solana合约中存储固定长度数据
    let token_balances = [100, 200, 300, 400, 500];
    println!("第三个代币余额: {}", token_balances[2]);

    // 可变数组
    let mut votes = [0; 5]; // 初始化5个元素都为0的数组
    println!("初始投票: {:?}", votes);

    // 修改数组元素
    votes[1] = 1;
    votes[3] = 1;
    println!("投票结果: {:?}", votes);
}

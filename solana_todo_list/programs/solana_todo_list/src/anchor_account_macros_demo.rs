// Anchor账户宏示例
// 本文件展示了Anchor框架中常用的账户宏及其约束条件

use anchor_lang::prelude::*;

// 声明程序ID
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod anchor_account_macros_demo {
    use super::*;

    // 初始化用户账户的指令
    pub fn initialize_user(ctx: Context<InitializeUser>, name: String, age: u8) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.name = name;
        user_account.age = age;
        user_account.authority = ctx.accounts.authority.key();
        Ok()
    }

    // 更新用户账户的指令
    pub fn update_user(ctx: Context<UpdateUser>, name: String, age: u8) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.name = name;
        user_account.age = age;
        Ok()
    }

    // 创建待办事项的指令
    pub fn create_todo(ctx: Context<CreateTodo>, content: String) -> Result<()> {
        let todo = &mut ctx.accounts.todo;
        let user = &ctx.accounts.user_account;

        todo.authority = ctx.accounts.authority.key();
        todo.user = user.key();
        todo.content = content;
        todo.completed = false;

        Ok()
    }

    // 使用PDA创建配置账户的指令
    pub fn initialize_config(ctx: Context<InitializeConfig>, data: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.data = data;
        config.authority = ctx.accounts.authority.key();
        config.bump = *ctx.bumps.get("config").unwrap();
        Ok()
    }
}

// 用户账户数据结构
#[account]
pub struct UserAccount {
    pub name: String,      // 用户名
    pub age: u8,          // 年龄
    pub authority: Pubkey, // 账户所有者
}

// 待办事项数据结构
#[account]
pub struct Todo {
    pub authority: Pubkey, // 创建者
    pub user: Pubkey,     // 关联的用户账户
    pub content: String,  // 内容
    pub completed: bool,  // 是否完成
}

// 配置账户数据结构
#[account]
pub struct Config {
    pub data: u64,        // 配置数据
    pub authority: Pubkey, // 管理员
    pub bump: u8,         // PDA的bump值
}

// 初始化用户账户的上下文
#[derive(Accounts)]
#[instruction(name: String, age: u8)]
pub struct InitializeUser<'info> {
    // #[account(init)] - 创建并初始化一个新账户
    // payer = authority - 指定支付账户创建费用的账户
    // space = 8 + 4 + name.len() + 1 + 32 - 计算账户所需空间
    // 8字节用于账户鉴别器，4+name.len()用于String，1字节用于u8，32字节用于Pubkey
    #[account(init, payer = authority, space = 8 + 4 + name.len() + 1 + 32)]
    pub user_account: Account<'info, UserAccount>,
    
    // #[account(mut)] - 标记账户为可变，允许修改账户数据
    // #[account(signer)] - 验证此账户已签名此交易
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    
    // 系统程序账户，用于创建新账户
    pub system_program: Program<'info, System>,
}

// 更新用户账户的上下文
#[derive(Accounts)]
pub struct UpdateUser<'info> {
    // #[account(mut)] - 标记账户为可变
    // has_one = authority - 验证账户的authority字段与传入的authority账户匹配
    // @ ErrorCode::Unauthorized - 自定义错误信息
    #[account(mut, has_one = authority @ ErrorCode::Unauthorized)]
    pub user_account: Account<'info, UserAccount>,
    
    // 验证签名者
    pub authority: Signer<'info>,
}

// 创建待办事项的上下文
#[derive(Accounts)]
#[instruction(content: String)]
pub struct CreateTodo<'info> {
    // 创建新的待办事项账户
    #[account(init, payer = authority, space = 8 + 32 + 32 + 4 + content.len() + 1)]
    pub todo: Account<'info, Todo>,
    
    // 验证用户账户存在且所有者匹配
    #[account(constraint = user_account.authority == authority.key() @ ErrorCode::Unauthorized)]
    pub user_account: Account<'info, UserAccount>,
    
    // 支付者和签名者
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// 初始化配置账户的上下文
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    // 使用PDA(程序派生地址)创建账户
    // seeds - 用于派生PDA的种子
    // bump - 自动查找并使用有效的bump值
    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 32 + 1,
        seeds = [b"config", authority.key().as_ref()],
        bump
    )]
    pub config: Account<'info, Config>,
    
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// 自定义错误代码
#[error_code]
pub enum ErrorCode {
    #[msg("您没有权限执行此操作")]
    Unauthorized,
}

// 以下是Anchor账户宏的主要类型和约束：

// 1. 账户类型宏：
//    - #[account] - 用于定义账户数据结构
//    - #[derive(Accounts)] - 用于定义指令的账户上下文

// 2. 常用账户约束：
//    - #[account(init)] - 创建并初始化新账户
//    - #[account(mut)] - 标记账户为可变
//    - #[account(signer)] - 验证账户已签名交易
//    - #[account(has_one = field)] - 验证账户字段与另一个账户匹配
//    - #[account(seeds = [...], bump)] - 验证或创建PDA
//    - #[account(constraint = expr)] - 添加自定义约束条件
//    - #[account(close = target)] - 关闭账户并将lamports发送到目标
//    - #[account(owner = expr)] - 验证账户所有者
//    - #[account(executable)] - 验证账户是否可执行(程序)
//    - #[account(address = expr)] - 验证账户地址

// 3. 高级约束：
//    - #[account(init_if_needed)] - 仅在账户不存在时初始化
//    - #[account(realloc = space, realloc::payer = target, realloc::zero = bool)] - 重新分配账户空间
//    - #[account(zero)] - 验证账户鉴别器是否为零

// 4. SPL代币约束：
//    - #[account(token::*)] - 代币账户约束
//    - #[account(mint::*)] - 铸币账户约束
//    - #[account(associated_token::*)] - 关联代币账户约束
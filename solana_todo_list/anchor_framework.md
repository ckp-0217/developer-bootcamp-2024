---
title: Anchor框架
description: Solana的Anchor框架概述
---

# Anchor框架 ⚓

Anchor是Solana的Sealevel运行时的框架，提供了多种便捷的开发者工具。

## 主要特性

- Rust eDSL，用于编写安全、可靠和高级的Solana程序
- IDL规范
- 从IDL生成客户端的TypeScript包
- 用于开发完整应用程序的CLI和工作区管理

如果你熟悉以太坊的Solidity、Truffle、web3.js或Parity的Ink!，那么这种体验会很熟悉。虽然语法和语义针对的是Solana，但编写RPC请求处理程序、发出IDL和从IDL生成客户端的高级工作流程是相同的。

有关如何使用Anchor的详细教程和示例，请参阅[指导教程](https://www.anchor-lang.com/docs/intro-to-anchor)或GitHub仓库中的[示例](https://github.com/coral-xyz/anchor/tree/master/examples)。

## 重要模块

### accounts

可以在账户验证结构中使用的账户类型。

```rust
use anchor_lang::prelude::*;

#[account]
pub struct MyAccount {
    pub data: u64,
    pub authority: Pubkey,
}
```

### context

用于向程序端点提供非参数输入的数据结构。

```rust
pub fn process(ctx: Context<Initialize>, data: u64) -> Result<()> {
    ctx.accounts.my_account.data = data;
    ctx.accounts.my_account.authority = ctx.accounts.authority.key();
    Ok(())
}
```

### error

错误处理相关功能。

```rust
#[error_code]
pub enum MyError {
    #[msg("账户数据无效")]
    InvalidAccountData,
    #[msg("权限不足")]
    InsufficientAuthority,
}
```

### prelude

prelude包含crate中所有常用的组件。所有程序都应通过`anchor_lang::prelude::*`包含它。

```rust
use anchor_lang::prelude::*;
```

### system_program

Solana系统程序相关功能。

## 常用宏

### declare_id

定义程序的ID。这应该在所有基于Anchor的程序的根部使用。

```rust
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
```

### declare_program

基于其IDL声明外部程序。

```rust
declare_program!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
```

### emit

记录客户端可以订阅的事件。使用sol_log_data系统调用。

```rust
#[event]
pub struct MyEvent {
    pub data: u64,
    pub label: String,
}

emit!(MyEvent {
    data: 100,
    label: "hello".to_string(),
});
```

### err

返回给定错误。与自定义错误类型一起使用。

```rust
if data > 100 {
    return err!(MyError::InvalidAccountData);
}
```

### error

生成包含文件和行信息的Error::AnchorError。

```rust
error!(MyError::InvalidAccountData);
```

### pubkey

定义静态公钥的便捷宏。

```rust
pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
```

### require

确保条件为真，否则返回给定错误。可以与自定义错误类型一起使用，也可以不使用。

```rust
require!(data < 100, MyError::InvalidAccountData);
```

### require_eq

确保两个非公钥值相等。

```rust
require_eq!(data, 100, MyError::InvalidAccountData);
```

## 账户约束

Anchor提供了多种账户约束，用于验证和操作Solana账户。以下是一些常见的约束：

### `#[account(signer)]`

检查给定账户是否签署了交易。

```rust
#[account(signer)]
pub authority: Signer<'info>,
```

### `#[account(mut)]`

检查给定账户是否可变，使Anchor保持任何状态更改。

```rust
#[account(mut)]
pub my_account: Account<'info, MyAccount>,
```

### `#[account(init)]`

通过对系统程序的CPI创建账户并初始化它。

```rust
#[account(
    init,
    payer = user,
    space = 8 + 8 + 32
)]
pub my_account: Account<'info, MyAccount>,
```

### `#[account(seeds, bump)]`

检查给定账户是否是从当前执行的程序、种子以及bump派生的PDA。

```rust
#[account(
    seeds = [b"my-seed", user.key().as_ref()],
    bump
)]
pub my_pda: Account<'info, MyAccount>,
```

## 常用特性（Traits）

### AccountDeserialize

可以从账户存储中反序列化的数据结构。

### AccountSerialize

可以序列化并存储到账户存储中的数据结构。

### Accounts

可以从Solana程序输入中反序列化的已验证账户的数据结构。

### AnchorDeserialize/AnchorSerialize

Borsh是指令和账户的默认序列化格式。

## 属性宏

### access_control

在运行装饰的指令处理程序之前执行给定的访问控制方法。

```rust
#[access_control(check_authority(&ctx))]
pub fn update(ctx: Context<Update>, new_data: u64) -> Result<()> {
    ctx.accounts.my_account.data = new_data;
    Ok(())
}

fn check_authority(ctx: &Context<Update>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.authority.key(),
        ctx.accounts.my_account.authority,
        MyError::InsufficientAuthority
    );
    Ok(())
}
```

### account

表示Solana账户的数据结构的属性。

```rust
#[account]
pub struct MyAccount {
    pub data: u64,
    pub authority: Pubkey,
}
```

### error_code

生成Error和类型Result<T> = Result<T, Error>类型，用作Anchor指令处理程序的返回类型。

```rust
#[error_code]
pub enum MyError {
    #[msg("账户数据无效")]
    InvalidAccountData,
    #[msg("权限不足")]
    InsufficientAuthority,
}
```

### event

event属性允许结构与emit!一起使用，以便程序可以记录客户端可以订阅的重要事件。

```rust
#[event]
pub struct MyEvent {
    pub data: u64,
    pub label: String,
}
```

### instruction

此属性用于覆盖Anchor的默认值。

```rust
#[instruction(data: u64)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

## 完整示例

以下是一个简单的Anchor程序示例，展示了如何创建和更新账户：

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.my_account.data = data;
        ctx.accounts.my_account.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_data: u64) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.authority.key(),
            ctx.accounts.my_account.authority,
            MyError::InsufficientAuthority
        );
        ctx.accounts.my_account.data = new_data;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(data: u64)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
    pub authority: Pubkey,
}

#[error_code]
pub enum MyError {
    #[msg("账户数据无效")]
    InvalidAccountData,
    #[msg("权限不足")]
    InsufficientAuthority,
}
```

## 参考资源

- [Anchor官方文档](https://www.anchor-lang.com/)
- [Anchor GitHub仓库](https://github.com/coral-xyz/anchor)
- [Solana开发者文档](https://docs.solana.com/)
- [Anchor示例](https://github.com/coral-xyz/anchor/tree/master/examples)
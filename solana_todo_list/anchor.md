---
title: 账户约束
description: Anchor 账户约束示例
---

Anchor 账户[约束](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html)的最小参考示例。

查看账户约束的[源代码](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/syn/src/codegen/accounts/constraints.rs)了解实现细节。

## 普通约束

### `#[account(signer)]`

描述：检查给定账户是否签署了交易。如果您只对账户有这个约束，请考虑使用 Signer 类型。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/signer)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/signer)

```rust title="attribute"
#[account(signer)]
#[account(signer @ <custom_error>)]
```

### `#[account(mut)]`

描述：检查给定账户是否可变。使 anchor 保持任何状态更改。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/mut)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/mut)

```rust title="attribute"
#[account(mut)]
#[account(mut @ <custom_error>)]
```

### `#[account(init)]`

描述：通过对系统程序的 CPI 创建账户并初始化它（设置其账户鉴别器）。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/init)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/init)

```rust title="attribute"
#[account(
    init,
    payer = <target_account>,
    space = <num_bytes>
)]
```

### `#[account(init_if_needed)]`

描述：与 init 相同，但仅在账户尚不存在时运行。需要 init-if-needed cargo 功能。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/init_if_needed)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/init_if_needed)

```rust title="attribute"
#[account(
    init_if_needed,
    payer = <target_account>
)]

#[account(
    init_if_needed,
    payer = <target_account>,
    space = <num_bytes>
)]
```

### `#[account(seeds, bump)]`

描述：检查给定账户是否是从当前执行的程序、种子以及（如果提供）bump 派生的 PDA。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/seed-bump)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/seed-bump)

```rust title="attribute"
#[account(
    seeds = <seeds>,
    bump
)]

#[account(
    seeds = <seeds>,
    bump,
    seeds::program = <expr>
)]

#[account(
    seeds = <seeds>,
    bump = <expr>
)]

#[account(
    seeds = <seeds>,
    bump = <expr>,
    seeds::program = <expr>
)]
```

### `#[account(has_one = target)]`

描述：检查账户上的目标字段是否与 Accounts 结构中目标字段的键匹配。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/has_one)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/has_one)

```rust title="attribute"
#[account(
    has_one = <target_account>
)]

#[account(
    has_one = <target_account> @ <custom_error>
)]
```

### `#[account(address = expr)]`

描述：检查账户密钥是否与公钥匹配。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/address)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/address)

```rust title="attribute"
#[account(address = <expr>)]
#[account(address = <expr> @ <custom_error>)]
```

### `#[account(owner = expr)]`

描述：检查账户所有者是否与 expr 匹配。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/owner)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/owner)

```rust title="attribute"
#[account(owner = <expr>)]
#[account(owner = <expr> @ <custom_error>)]
```

### `#[account(executable)]`

描述：检查账户是否可执行（即账户是程序）。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/executable)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/executable)

```rust title="attribute"
#[account(executable)]
```

### `#[account(zero)]`

描述：检查账户鉴别器是否为零。用于大于 10 Kibibyte 的账户。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/zero)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/zero)

```rust title="attribute"
#[account(zero)]
```

### `#[account(close = target)]`

描述：通过将 lamports 发送到目标并重置数据来关闭账户。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/close)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/close)

```rust title="attribute"
#[account(close = <target_account>)]
```

### `#[account(constraint = expr)]`

描述：自定义约束，检查给定表达式是否评估为真。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/constraint)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/constraint)

```rust title="attribute"
#[account(constraint = <expr>)]
#[account(
    constraint = <expr> @ <custom_error>
)]
```

### `#[account(realloc)]`

描述：用于在指令开始时重新分配程序账户空间。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/realloc)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/realloc)

```rust title="attribute"
#[account(
    realloc = <space>,
    realloc::payer = <target>,
    realloc::zero = <bool>
)]
```

## SPL 约束

### `#[account(token::*)]`

描述：创建或验证具有指定铸币和权限的代币账户。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/token)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/token)

```rust title="attribute"
#[account(
    token::mint = <target_account>,
    token::authority = <target_account>
)]

#[account(
    token::mint = <target_account>,
    token::authority = <target_account>,
    token::token_program = <target_account>
)]
```

### `#[account(mint::*)]`

描述：创建或验证具有指定参数的铸币账户。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/mint)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/mint)

```rust title="attribute"
#[account(
    mint::authority = <target_account>,
    mint::decimals = <expr>
)]

#[account(
    mint::authority = <target_account>,
    mint::decimals = <expr>,
    mint::freeze_authority = <target_account>
)]
```

### `#[account(associated_token::*)]`

描述：创建或验证关联代币账户。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/associated_token)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/associated_token)

```rust title="attribute"
#[account(
    associated_token::mint = <target_account>,
    associated_token::authority = <target_account>
)]

#[account(
    associated_token::mint = <target_account>,
    associated_token::authority = <target_account>,
    associated_token::token_program = <target_account>
)]
```

### `#[account(*::token_program = expr)]`

描述：token_program 可以被可选地覆盖。  
示例：[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/token_program)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/token_program)

```rust title="attribute"
#[account(*::token_program = <target_account>)]
```

## 指令属性

### `#[instruction(...)]`

描述：您可以使用 `#[instruction(..)]` 属性访问指令的参数。您必须按照与指令处理程序相同的顺序列出它们，但可以省略最后一个所需参数之后的所有参数。跳过参数将导致错误。

示例：
[Github](https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/instruction)
|
[Solpg](https://beta.solpg.io/https://github.com/solana-developers/anchor-examples/tree/main/account-constraints/instruction)

```rust title="snippet"
// [!code word:input]
#[program]
pub mod example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: String) -> Result<()> {
        // --snip--
    }
}

#[derive(Accounts)]
// [!code highlight]
#[instruction(input: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 4 + input.len(),
    )]
    pub new_account: Account<'info, DataAccount>,
    // --snip--
}
```

有效用法：

```rust title="snippet"
// [!code word:input_one]
// [!code word:input_two]
#[program]
pub mod example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input_one: String, input_two: String) -> Result<()> {
        // --snip--
    }
}

#[derive(Accounts)]
// [!code highlight]
#[instruction(input_one: String, input_two: String)]
pub struct Initialize<'info> {
    // --snip--
}
```

```rust title="snippet"
// [!code word:input_one]
#[program]
pub mod example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input_one: String, input_two: String) -> Result<()> {
        // --snip--
    }
}

#[derive(Accounts)]
// [!code highlight]
#[instruction(input_one: String)]
pub struct Initialize<'info> {
    // --snip--
}
```

无效用法，将导致错误：

```rust title="snippet"
// [!code word:input_two]
#[program]
pub mod example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input_one: String, input_two: String) -> Result<()> {
        // --snip--
    }
}

#[derive(Accounts)]
// [!code highlight]
#[instruction(input_two: String)]
pub struct Initialize<'info> {
    // --snip--
}
```
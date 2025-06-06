# Rust 与 Solana Anchor 关键字、符号及注解速查表

## Rust 关键字

| 关键字      | 描述                         | 举例                                     | 举例说明                                     |
|-------------|------------------------------|------------------------------------------|----------------------------------------------|
| `as`        | 类型转换/重命名导入项        | `let x = 1u8 as u32;`                    | 将 u8 类型的 1 转换为 u32 类型                 |
| `async`     | 声明异步函数/块              | `async fn fetch_data() {}`               | 定义一个名为 `fetch_data` 的异步函数         |
| `await`     | 等待异步操作                 | `let data = future.await;`               | 等待 `future` 这个异步操作完成并获取结果     |
| `break`     | 跳出循环                     | `loop { if condition { break; } }`       | 如果 `condition` 为真，则跳出当前的 `loop` 循环 |
| `const`     | 常量定义                     | `const MAX_POINTS: u32 = 100_000;`        | 定义一个名为 `MAX_POINTS` 的 u32 类型常量    |
| `continue`  | 跳过本次循环                 | `for i in 0..10 { if i % 2 == 0 { continue; } }` | 在循环中如果 `i` 是偶数，则跳过后续代码，开始下一次迭代 |
| `crate`     | 当前 crate                   | `use crate::module::my_function;`        | 从当前 crate 的 `module` 模块中导入 `my_function` |
| `dyn`       | 动态分发 trait 对象          | `fn process(item: &dyn MyTrait) {}`      | 函数 `process` 接受一个实现了 `MyTrait` 的动态分发对象引用 |
| `else`      | 条件分支                     | `if x > 0 { /* ... */ } else { /* ... */ }` | 如果 `x > 0` 不成立，则执行 `else` 块中的代码 |
| `enum`      | 枚举类型                     | `enum Message { Quit, Write(String) }`   | 定义一个名为 `Message` 的枚举类型，包含不同变体 |
| `extern`    | 外部函数/模块                | `extern "C" { fn c_function(); }`        | 声明一个遵循 C 语言 ABI 的外部函数 `c_function` |
| `false`     | 布尔值 false                 | `let is_ready = false;`                  | 将布尔变量 `is_ready` 初始化为 `false`       |
| `fn`        | 函数定义                     | `fn add(a: i32, b: i32) -> i32 { a + b }` | 定义一个名为 `add` 的函数，接收两个 i32 参数并返回它们的和 |
| `for`       | for 循环/生命周期            | `for item in collection {}`              | 遍历 `collection` 中的每一个 `item`          |
| `if`        | 条件语句                     | `if score > 90 {}`                       | 如果 `score` 大于 90，则执行块内代码         |
| `impl`      | 实现 trait/方法              | `impl MyStruct { fn new() -> Self {} }`  | 为 `MyStruct` 结构体实现方法，如 `new` 构造函数 |
| `in`        | for 循环成员                 | `for x in 0..5 {}`                       | 用于 `for` 循环，指定迭代的范围或集合        |
| `let`       | 变量绑定                     | `let count = 0;`                         | 定义一个名为 `count` 的变量并绑定值为 0      |
| `loop`      | 无限循环                     | `loop { println!("Again!"); }`           | 创建一个会无限打印 "Again!" 的循环         |
| `match`     | 模式匹配                     | `match value { Some(v) => {}, None => {} }` | 根据 `value` 的不同模式执行相应的代码块    |
| `mod`       | 模块定义                     | `mod utils { pub fn helper() {} }`       | 定义一个名为 `utils` 的模块，其中包含一个公共函数 `helper` |
| `move`      | 闭包所有权转移               | `let data = vec![1]; let c = move || println!("{:?}", data);` | 创建一个闭包 `c`，它会获取 `data` 的所有权 |
| `mut`       | 可变变量/引用                | `let mut counter = 0; counter += 1;`     | 定义一个可变变量 `counter`，并修改它的值     |
| `pub`       | 公有可见性                   | `pub struct User { name: String }`       | 定义一个公共的结构体 `User`                  |
| `ref`       | 模式中引用绑定               | `match &val { Some(ref r) => {} }`       | 在 `match` 模式中，通过引用绑定 `r` 到 `val` 内部的值 |
| `return`    | 返回语句                     | `fn get_val() -> i32 { return 10; }`     | 从函数 `get_val` 中返回值 10               |
| `self`      | 当前实例/模块(小写)          | `impl Point { fn x(&self) -> i32 { self.x } }` | 在方法中，`self` 指代结构体实例本身        |
| `Self`      | 当前类型(大写)               | `impl MyTrait for String { fn new() -> Self { String::new() } }` | 在 trait 实现中，`Self` 指代实现该 trait 的具体类型（这里是 `String`） |
| `static`    | 静态变量/生命周期            | `static APP_NAME: &str = "My App";`       | 定义一个在整个程序生命周期内都有效的静态字符串变量 `APP_NAME` |
| `struct`    | 结构体定义                   | `struct Point { x: i32, y: i32 }`        | 定义一个名为 `Point` 的结构体，包含 x 和 y 两个字段 |
| `super`     | 父模块                       | `fn call_parent() { super::parent_func(); }` | 调用父模块中的 `parent_func` 函数          |
| `trait`     | trait 定义                   | `trait Summary { fn summarize(&self) -> String; }` | 定义一个名为 `Summary` 的 trait，包含一个 `summarize` 方法 |
| `true`      | 布尔值 true                  | `let is_active = true;`                  | 将布尔变量 `is_active` 初始化为 `true`       |
| `type`      | 类型别名                     | `type Kilometers = u32; let dist: Kilometers = 5;` | 为 `u32` 类型创建一个别名 `Kilometers`       |
| `unsafe`    | 不安全代码块                 | `unsafe { let ptr = &mut val as *mut i32; *ptr = 10; }` | 在不安全块中执行可能违反 Rust 内存安全的操作，如裸指针解引用 |
| `use`       | 导入模块/项                  | `use std::collections::HashMap;`         | 从标准库的 `collections` 模块中导入 `HashMap` |
| `where`     | 泛型约束                     | `fn print_debug<T>(item: T) where T: std::fmt::Debug {}` | 为泛型函数 `print_debug` 的类型参数 `T` 添加 `Debug` trait 约束 |
| `while`     | while 循环                   | `while count < 5 { count += 1; }`        | 当 `count < 5` 条件为真时，重复执行循环体内的代码 |

### Rust 保留关键字（当前未启用或特定场景使用）

`abstract`、`become`、`box`、`do`、`final`、`macro`、`override`、`priv`、`typeof`、`unsized`、`virtual`、`yield`、`try`
*(这些关键字目前大部分为保留，供未来语言发展使用，或在特定实验性功能中使用)*

---

## Rust 关键符号

| 符号   | 描述                       | 举例                                     | 举例说明                                     |
|--------|----------------------------|------------------------------------------|----------------------------------------------|
| `;`    | 语句结束                   | `let x = 5;`                             | 标记 `let x = 5` 这个语句的结束              |
| `,`    | 参数/元素分隔              | `fn f(a: i32, b: i32) {}` `let arr = [1, 2];` | 分隔函数参数 `a` 和 `b`；分隔数组元素 1 和 2 |
| `.`    | 成员访问                   | `my_struct.field`                        | 访问结构体 `my_struct` 的 `field` 成员       |
| `..`   | 范围/结构体更新/模式       | `1..5`  `Foo { x, ..default }`           | 创建一个从1到4的范围；使用 `default` 的其余字段更新 `Foo` |
| `..=`  | 包含上界的范围             | `1..=5`                                  | 创建一个从1到5（包含5）的范围                |
| `:`    | 类型注解/trait约束/字段    | `let x: i32 = 1;` `fn f<T: Clone>() {}`  | 指定变量 `x` 的类型为 `i32`；约束泛型 `T` 实现 `Clone` |
| `::`   | 路径分隔                   | `std::io::Result`                        | 访问 `std` crate 下 `io` 模块中的 `Result` 类型 |
| `=`    | 赋值                       | `let x = 1;`                             | 将值 1 赋给变量 `x`                          |
| `==`   | 相等比较                   | `if a == b {}`                           | 判断变量 `a` 是否等于 `b`                    |
| `!=`   | 不等比较                   | `if a != b {}`                           | 判断变量 `a` 是否不等于 `b`                  |
| `=>`   | match 分支/闭包简写        | `match x { Some(v) => println!("{}", v) }` | `match` 表达式中，如果匹配 `Some(v)`，则执行打印操作 |
| `->`   | 函数返回类型               | `fn get_number() -> i32 {}`              | 表明函数 `get_number` 返回一个 `i32` 类型的值 |
| `&`    | 引用/按位与                 | `let r = &x;` `let z = a & b;`           | 创建对变量 `x` 的引用 `r`；执行 `a` 和 `b` 的按位与操作 |
| `&mut` | 可变引用                   | `let r_mut = &mut x;`                    | 创建对变量 `x` 的可变引用 `r_mut`            |
| `*`    | 解引用/乘法                 | `let val = *r;` `let area = w * h;`       | 解引用指针/引用 `r` 获取其值；计算 `w` 和 `h` 的乘积 |
| `|`    | 闭包参数/模式或/按位或      | `let add = |a, b| a + b;` `1 | 2`        | 定义一个接收 `a` 和 `b` 的闭包；模式匹配中的“或”；按位或操作 |
| `@`    | 模式绑定                   | `match opt { Some(val @ 1..=5) => {} }`  | 在模式匹配时，将匹配到的值（1到5之间）绑定到 `val` |
| `$`    | 宏变量                     | `macro_rules! print_expr { ($e:expr) => { println!("{:?}", $e); } }` | 在声明宏中，`$e` 代表一个表达式类型的捕获变量 |
| `?`    | 错误传播/Option解包         | `let content = fs::read_to_string("f.txt")?;` | 如果 `read_to_string` 返回 `Err`，则函数立即返回该错误 |
| `!`    | 宏调用/永不返回类型(Never)  | `println!("Hello");` `fn exit() -> ! { loop {} }` | 调用 `println!` 宏；声明函数 `exit` 永不返回 |
| `_`    | 通配符/未使用变量           | `let (first, _) = (1, 2);` `let _ = func();` | 忽略元组的第二个元素；忽略 `func()` 的返回值 |
| `'a`   | 生命周期标注                | `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}` | 声明一个生命周期参数 `'a`，用于标记引用的有效范围 |
| `{}`   | 代码块/结构体初始化         | `{ let temp = 5; temp }` `Point { x:0, y:0 }` | 定义一个代码块；初始化 `Point` 结构体实例    |
| `[]`   | 数组/切片/索引              | `let arr = [1, 2, 3];` `arr[0]`          | 定义一个数组；访问数组的第一个元素           |
| `()`   | 单元类型/元组/函数参数/分组 | `fn empty() {}` `let t = (1, "hi");`     | 表示无返回值的单元类型；定义一个元组；函数参数列表；表达式分组 |
| `+`    | trait 约束组合/加法         | `fn process<T: Debug + Clone>(item: T) {}` | 要求类型 `T` 同时实现 `Debug` 和 `Clone` trait；加法运算 |
| `-`    | 减法/负号                   | `let diff = a - b;` `let neg = -5;`      | 计算 `a` 和 `b` 的差；表示负数 -5            |
| `/`    | 除法                        | `let ratio = total / count;`             | 计算 `total` 除以 `count` 的商               |
| `%`    | 取模                        | `let remainder = 10 % 3;`                | 计算 10 除以 3 的余数                        |
| `^`    | 按位异或                    | `let xor_val = 0b1010 ^ 0b1100;`         | 计算两个二进制数的按位异或                   |

---

## Rust 进阶与常用宏相关

| 关键字/符号    | 描述     | 举例                                     | 举例说明                                     |
|----------------|----------|------------------------------------------|----------------------------------------------|
| `macro_rules!` | 声明宏定义 | `macro_rules! my_vec { ($($x:expr),*) => { ... }; }` | 定义一个名为 `my_vec` 的声明宏               |
| `$`            | 宏变量   | `($item:ident)`                          | 在宏规则中，`$item` 捕获一个标识符           |
| `!`            | 宏调用   | `println!("Processing...");`             | 调用名为 `println` 的宏                      |

---

## Rust 生命周期与泛型相关

| 关键字/符号 | 描述         | 举例                                     | 举例说明                                     |
|-------------|--------------|------------------------------------------|----------------------------------------------|
| `'static`   | 静态生命周期 | `let s: &'static str = "Hello";`          | 字符串字面量 `s` 拥有静态生命周期，在整个程序运行期间都有效 |
| `'_`        | 匿名生命周期 | `fn print_str(s: &'_ str) {}`            | 编译器会自动推断引用的生命周期               |
| `<T>`       | 泛型参数     | `struct Container<T> { value: T }`       | 定义一个泛型结构体 `Container`，它可以持有任何类型 `T` 的值 |

---

## Solana Anchor 特有注解

| 注解                      | 描述                         | 举例                                                         | 举例说明                                                         |
|---------------------------|------------------------------|--------------------------------------------------------------|------------------------------------------------------------------|
| `#[program]`              | 标记 Anchor 程序模块         | `#[program] pub mod my_solana_program { ... }`               | 将 `my_solana_program` 模块标记为一个 Anchor 程序（智能合约）      |
| `#[account]`              | 定义账户数据结构             | `#[account] pub struct GameState { score: u64 }`             | 定义一个名为 `GameState` 的账户状态结构，Anchor 会处理其序列化 |
| `#[derive(Accounts)]`     | 定义指令账户结构             | `#[derive(Accounts)] pub struct Initialize<'info> { ... }`   | 为 `Initialize` 结构体派生处理账户验证和反序列化的逻辑         |
| `#[account(mut)]`         | 标记可变账户                 | `#[account(mut)] pub user_account: Account<'info, UserData>` | 指明 `user_account` 在指令执行期间是可变的                   |
| `#[account(init, ...)]`   | 初始化账户                 | `#[account(init, payer = user, space = 8 + 32)] pub data_account: Account<'info, MyData>` | 指明 `data_account` 将在指令中被初始化，指定付款人和空间大小 |
| `#[account(seeds = ..., bump)]` | PDA (程序派生地址) 账户 | `#[account(seeds = [b"my_seed"], bump)] pub pda_account: Account<'info, Data>` | 定义一个 PDA 账户，使用 "my_seed" 作为种子，并存储 bump 值 |
| `#[account(has_one = owner)]` | 验证账户关系(所有权)       | `#[account(mut, has_one = owner)] pub token_account: Account<'info, TokenData>` | 验证 `token_account` 的 `owner` 字段与指令中提供的 `owner` 账户匹配 |
| `#[account(signer)]`      | 要求账户是签名者             | `#[account(signer)] pub authority: Signer<'info>`            | 要求 `authority` 账户必须是当前交易的签名者                  |
| `#[event]`                | 事件定义                     | `#[event] pub struct MyEvent { data: u64, label: String }`    | 定义一个名为 `MyEvent` 的事件，可在链上记录和被客户端监听      |
| `#[error_code]`           | 自定义错误码定义             | `#[error_code] pub enum MyError { InvalidInput, InsufficientFunds }` | 定义一组自定义错误码，方便在程序中返回具体的错误类型         |
| `#[zero_copy]`            | 零拷贝账户 (用于大数据)      | `#[account(zero_copy)] pub large_data: AccountLoader<'info, LargeStruct>` | 标记账户使用零拷贝反序列化，适合存储大量数据，通过 `AccountLoader` 访问 |
| `#[interface]`            | CPI (跨程序调用) 接口定义    | `#[interface] pub trait ExternalCpi { fn external_instruction(&self, value: u64) -> Result<()>; }` | 定义一个用于跨程序调用的接口 `ExternalCpi`                   |
| `#[state]`                | (已废弃) 定义程序状态账户    | `#[state] pub struct GlobalState { admin: Pubkey }`          | (旧版 Anchor) 定义一个全局状态账户，通常用于存储程序级别的配置 |

---

## 说明

- 上述表格涵盖了 Rust 语言的绝大多数常用关键字、符号以及 Solana Anchor 框架中常见的注解。
- Rust 的保留关键字目前大部分未在稳定版中启用，但为语言未来的发展保留了可能性。
- 某些符号在不同的 Rust 上下文中有不同的含义，具体用法需结合实际代码理解。
- Solana Anchor 的注解极大地简化了 Solana 智能合约的开发，提供了账户验证、序列化、错误处理等功能。

---

> 建议结合 [Rust 官方文档](https://doc.rust-lang.org/reference/keywords.html) 和 [Anchor 官方文档](https://book.anchor-lang.com/) 深入学习。
// 08_enums_and_pattern_matching.rs

// 枚举（enum）允许你通过列举其可能的成员来定义一个类型。
// 模式匹配（pattern matching）通过 `match` 表达式使你能够根据值的结构执行不同的代码路径。

// 定义一个表示 IP 地址类型的枚举
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 定义一个包含不同类型数据的枚举
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 为 Message 枚举定义方法
impl Message {
    fn call(&self) {
        // 方法体
        println!("Called message: {:?}", self);
    }
}

// Option<T> 枚举是标准库中定义的，用于表示一个值可能存在或不存在的情况。
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    // 创建 IpAddrKind 枚举的实例
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("Home IP address: {:?}", home);
    println!("Loopback IP address: {:?}", loopback);

    // 创建 Message 枚举的实例
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);

    m1.call();
    m2.call();
    m3.call();
    m4.call();

    // --- match 表达式 ---
    // `match` 表达式允许你将一个值与一系列模式进行比较，并根据匹配的模式执行代码。
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    #[derive(Debug)] // 派生 Debug 以便可以打印州名
    enum UsState {
        Alabama,
        Alaska,
        // ... 其他州
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // Quarter 变体包含一个 UsState 值
    }

    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alaska);

    println!("Value of coin1: {} cents", value_in_cents(coin1));
    println!("Value of coin2: {} cents", value_in_cents(coin2));

    // Option<T> 与 match
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Five: {:?}, Six: {:?}, None: {:?}", five, six, none);

    // `match` 必须是穷尽的
    // 你必须覆盖所有可能的情况。`_` 通配符可以匹配任何值，通常用在最后一个分支。
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // `other` 会绑定到匹配的值
        // _ => reroll(), // 如果不想使用值，可以用 `_`
        // _ => (), // 如果不想做任何事，可以用单元值
    }

    fn add_fancy_hat() {
        println!("Added fancy hat!");
    }
    fn remove_fancy_hat() {
        println!("Removed fancy hat!");
    }
    fn move_player(num_spaces: u8) {
        println!("Moved player {} spaces!", num_spaces);
    }
    // fn reroll() { println!("Rerolled!"); }

    // --- if let 表达式 ---
    // `if let` 是一种更简洁的方式来处理只关心一种 `match` 情况的场景。
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // 也可以与 `else` 一起使用
    let mut count = 0;
    let coin_for_if_let = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin_for_if_let {
        println!("State quarter from {:?}! (using if let)", state);
    } else {
        count += 1;
    }
    println!("Count: {}", count);
}
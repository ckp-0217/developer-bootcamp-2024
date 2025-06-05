// 13_traits_and_trait_bounds.rs

// Trait 是一种定义共享行为的方式。类似于其他语言中的接口（interface）。
// Trait Bound 用于在泛型类型参数上约束其必须实现某些 trait。

// --- 定义 Trait ---
// Trait 定义了一组方法签名，实现该 trait 的类型必须提供这些方法的具体实现。
pub trait Summary {
    fn summarize_author(&self) -> String;

    // Trait 可以提供默认的方法实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// --- 实现 Trait ---
// 为自定义类型实现 trait
#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // summarize 方法使用默认实现
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// --- Trait 作为参数 --- 
// 我们可以使用 `impl Trait` 语法或 trait bound 语法来接受实现了特定 trait 的类型作为参数。

// `impl Trait` 语法 (适用于简单情况)
pub fn notify_impl_trait(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound 语法 (更灵活，适用于复杂情况)
pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news (trait bound)! {}", item.summarize());
}

// 多个 trait bound
pub fn notify_multiple_bounds<T: Summary + std::fmt::Debug>(item: &T) {
    println!("Item: {:?}", item);
    println!("News: {}", item.summarize());
}

// 使用 `where` 子句来指定 trait bound (更清晰)
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: std::fmt::Debug + Clone,
          U: Clone + Summary
{
    println!("t: {:?}, u summary: {}", t, u.summarize());
    0
}

// --- 返回实现了 Trait 的类型 --- 
// 你也可以在函数返回值中使用 `impl Trait` 来表示函数返回某个实现了特定 trait 的类型，
// 但不指定具体的返回类型。这对于返回闭包或迭代器特别有用。
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
    // 注意：如果函数返回 `impl Trait`，则所有可能的返回路径都必须返回相同的具体类型。
    // 例如，不能在一个分支返回 Tweet，在另一个分支返回 NewsArticle。
}

// --- 使用 Trait Bound 来有条件地实现方法 ---
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只有当 Pair<T> 中的类型 T 实现了 PartialOrd 和 Display trait 时，
// cmp_display 方法才会被实现。
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("john_doe"),
        content: String::from("Hello world, this is my first tweet!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify_impl_trait(&tweet);
    notify_trait_bound(&article);
    notify_multiple_bounds(&tweet); // Tweet 实现了 Summary 和 Debug (通过 derive)

    let summarizable_item = returns_summarizable();
    println!("Returned summarizable: {}", summarizable_item.summarize());

    let pair_of_numbers = Pair::new(5, 10);
    // pair_of_numbers.cmp_display(); // i32 实现了 Display 和 PartialOrd
    pair_of_numbers.cmp_display();

    let pair_of_strings = Pair::new(String::from("apple"), String::from("banana"));
    pair_of_strings.cmp_display(); // String 实现了 Display 和 PartialOrd

    println!("\nAnchor 合约中 Trait 的提示：");
    println!("Anchor 自身大量使用 Trait 来实现其核心功能，例如：");
    println!("  `AccountSerialize` 和 `AccountDeserialize` 用于账户数据的序列化和反序列化。");
    println!("  `Accounts` trait 用于定义指令的账户上下文结构体。");
    println!("  `ToAccountInfo`, `ToAccountMeta`, `Owner`, `Key` 等。");
    println!("合约开发者通常不需要自己定义很多 Trait，但理解它们对于深入使用 Anchor 和 Solana SDK 非常重要。");
    println!("Trait bounds 在理解 Anchor 宏生成的代码时也很有用。");
}
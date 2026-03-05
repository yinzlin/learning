/// 泛型与特性示例模块
/// 涵盖：泛型数据类型、特性、生命周期

/// 演示泛型数据类型
fn demo_generics() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 泛型数据类型                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n泛型函数：");
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 12, 100, 65];
    let result = largest(&number_list);
    println!("最大数字：{}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大字符：{}", result);

    println!("\n泛型结构体：");
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("整数点：{:?}", integer_point);
    println!("浮点数点：{:?}", float_point);

    println!("\n多类型参数：");
    #[derive(Debug)]
    struct PointMixed<T, U> {
        x: T,
        y: U,
    }

    let mixed = PointMixed { x: 5, y: 4.0 };
    println!("混合类型点：{:?}", mixed);

    println!("\n泛型方法：");
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 3.0, y: 4.0 };
    println!("x 坐标：{}", p.x());
    println!("到原点距离：{}", p.distance_from_origin());

    println!("\n泛型枚举：");
    let integer = Some(5);
    let float = Some(5.0);
    println!("Option<i32>: {:?}", integer);
    println!("Option<f64>: {:?}", float);
}

/// 演示特性
fn demo_traits() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 特性（Trait）                   │");
    println!("└─────────────────────────────────────┘");

    println!("\n定义特性：");
    pub trait Summary {
        fn summarize(&self) -> String;

        fn default_summary(&self) -> String {
            String::from("(阅读更多...)")
        }
    }

    println!("\n实现特性：");
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let article = NewsArticle {
        headline: String::from("Rust 1.70 发布"),
        location: String::from("互联网"),
        author: String::from("Rust 团队"),
        content: String::from("新版本带来了许多改进..."),
    };
    println!("文章摘要：{}", article.summarize());

    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Rust 真的很棒！"),
        reply: false,
        retweet: false,
    };
    println!("推文摘要：{}", tweet.summarize());

    println!("\n特性作为参数：");
    fn notify(item: &impl Summary) {
        println!("突发新闻！{}", item.summarize());
    }

    notify(&article);
    notify(&tweet);

    println!("\n特性约束语法：");
    fn notify2<T: Summary>(item: &T) {
        println!("新闻：{}", item.summarize());
    }

    notify2(&article);

    println!("\n多个特性约束：");
    fn notify_both<T: Summary + std::fmt::Display>(item: &T) {
        println!("显示：{}", item);
        println!("摘要：{}", item.summarize());
    }

    println!("\nwhere 子句：");
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: std::fmt::Display + std::fmt::Debug,
        U: Summary + Clone,
    {
        println!("{:?}", t);
        println!("{}", u.summarize());
        0
    }

    println!("\n返回实现了特性的类型：");
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("当然，正如你可能已经知道的"),
            reply: false,
            retweet: false,
        }
    }

    let summary = returns_summarizable();
    println!("返回值：{}", summary.summarize());

    println!("\n条件实现：");
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: std::fmt::Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("最大的是 x = {}", self.x);
            } else {
                println!("最大的是 y = {}", self.y);
            }
        }
    }

    let pair = Pair::new(5, 10);
    pair.cmp_display();
}

/// 演示生命周期
fn demo_lifetimes() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 生命周期                        │");
    println!("└─────────────────────────────────────┘");

    println!("\n生命周期标注：");
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("更长的字符串：{}", result);

    println!("\n结构体中的生命周期：");
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("请注意：{}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("找不到 '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("摘录：{}", excerpt.part);
    println!("级别：{}", excerpt.level());

    println!("\n生命周期省略规则：");
    println!("规则 1：每个引用参数获得自己的生命周期");
    println!("规则 2：如果只有一个输入生命周期参数，它被赋予所有输出生命周期参数");
    println!("规则 3：如果有多个输入生命周期参数但其中一个是 &self 或 &mut self，self 的生命周期被赋予所有输出生命周期参数");

    println!("\n静态生命周期：");
    let s: &'static str = "我有静态生命周期";
    println!("静态字符串：{}", s);
}

/// 公开的演示函数
pub fn demo_generics_traits() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块五：泛型与特性                  ║");
    println!("╚═════════════════════════════════════════╝");

    demo_generics();
    demo_traits();
    demo_lifetimes();
}

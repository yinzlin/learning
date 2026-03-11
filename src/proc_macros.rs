/// 过程宏深入示例模块
/// 涵盖：声明式宏进阶、派生宏概念、属性宏概念、函数式宏概念

/// 演示声明式宏进阶
fn demo_declarative_macros() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 声明式宏进阶                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n复杂模式匹配：");
    macro_rules! match_expr {
        ($expr:expr) => {
            match $expr {
                0 => "零",
                1..=10 => "一到十",
                11..=100 => "十一到一百",
                _ => "其他",
            }
        };
    }

    println!("match_expr!(0) = {}", match_expr!(0));
    println!("match_expr!(5) = {}", match_expr!(5));
    println!("match_expr!(50) = {}", match_expr!(50));
    println!("match_expr!(200) = {}", match_expr!(200));

    println!("\nHashMap 宏：");
    macro_rules! hash_map {
        ($($key:expr => $value:expr),* $(,)?) => {{
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }};
    }

    let scores = hash_map! {
        "Alice" => 95,
        "Bob" => 87,
        "Carol" => 92,
    };
    println!("scores = {:?}", scores);

    println!("\n递归宏（概念演示）：");
    println!("注意：Rust 声明式宏不支持编译时算术运算");
    println!("以下展示递归宏的模式匹配概念：");
    macro_rules! count_down {
        (0) => { println!("完成！") };
        ($n:tt) => {
            println!("计数: {}", $n);
        };
    }

    count_down!(0);
    count_down!(3);
    count_down!(5);

    println!("\n卫生性（Hygiene）：");
    macro_rules! hygienic {
        ($x:expr) => {
            {
                let x = 10;
                $x + x
            }
        };
    }

    let x = 5;
    let result = hygienic!(x);
    println!("hygienic!(x) where x=5, result = {}", result);
    println!("宏内部 x=10, 外部 x=5, 结果 = 5 + 10 = 15");
}

/// 演示宏的重复模式
fn demo_repetition_patterns() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 宏的重复模式                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n零次或多次 (*)：");
    macro_rules! count_items {
        ($($item:expr),*) => {{
            let mut count = 0;
            $(
                let _ = $item;
                count += 1;
            )*
            count
        }};
    }

    println!("count_items!() = {}", count_items!());
    println!("count_items!(1, 2, 3) = {}", count_items!(1, 2, 3));

    println!("\n一次或多次 (+)：");
    macro_rules! sum_items {
        ($first:expr $(, $rest:expr)*) => {{
            let mut total = $first;
            $(
                total += $rest;
            )*
            total
        }};
    }

    println!("sum_items!(10) = {}", sum_items!(10));
    println!("sum_items!(1, 2, 3, 4, 5) = {}", sum_items!(1, 2, 3, 4, 5));

    println!("\n可选 (?)：");
    macro_rules! greet {
        ($name:expr) => {
            format!("Hello, {}!", $name)
        };
        ($name:expr, $greeting:expr) => {
            format!("{}, {}!", $greeting, $name)
        };
    }

    println!("{}", greet!("World"));
    println!("{}", greet!("Rust", "Welcome"));

    println!("\n嵌套重复：");
    macro_rules! matrix {
        ($([$($x:expr),*]),* $(,)?) => {{
            vec![
                $(
                    vec![$($x),*],
                )*
            ]
        }};
    }

    let m = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("matrix = {:?}", m);
}

/// 演示宏的片段分类符
fn demo_fragment_specifiers() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 宏的片段分类符                  │");
    println!("└─────────────────────────────────────┘");

    println!("\n常用片段分类符：");
    println!("expr - 表达式");
    println!("ident - 标识符");
    println!("ty - 类型");
    println!("literal - 字面量");
    println!("path - 路径（如 std::mem::size_of）");
    println!("stmt - 语句");
    println!("block - 代码块");
    println!("pat - 模式");
    println!("tt - 标记树");

    println!("\n示例 - ident：");
    macro_rules! create_function {
        ($name:ident) => {
            fn $name() {
                println!("函数 {} 被调用", stringify!($name));
            }
        };
    }

    create_function!(foo);
    create_function!(bar);
    foo();
    bar();

    println!("\n示例 - ty：");
    macro_rules! print_size {
        ($ty:ty) => {
            println!("{} 的大小是 {} 字节", stringify!($ty), std::mem::size_of::<$ty>());
        };
    }

    print_size!(i32);
    print_size!(f64);
    print_size!(bool);

    println!("\n示例 - pat：");
    macro_rules! match_pattern {
        ($value:expr, $pat:pat => $result:expr) => {
            match $value {
                $pat => $result,
                _ => "不匹配",
            }
        };
    }

    println!("match_pattern!(5, 1..=10 => \"匹配\") = {}", match_pattern!(5, 1..=10 => "匹配"));
    println!("match_pattern!(20, 1..=10 => \"匹配\") = {}", match_pattern!(20, 1..=10 => "匹配"));
}

/// 演示过程宏概念
fn demo_procedural_macros() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. 过程宏概念                      │");
    println!("└─────────────────────────────────────┘");

    println!("\n过程宏类型：");
    println!("1. 派生宏 (Derive Macros)");
    println!("   - 使用 #[derive(...)] 语法");
    println!("   - 自动实现 trait");
    println!("   - 示例：#[derive(Debug, Clone, Serialize)]");

    println!("\n2. 属性宏 (Attribute Macros)");
    println!("   - 使用 #[attribute] 语法");
    println!("   - 可以修改或增强项");
    println!("   - 示例：#[route(GET, \"/users\")]");

    println!("\n3. 函数式宏 (Function-like Macros)");
    println!("   - 使用 macro_name!(...) 语法");
    println!("   - 可以接受任意输入");
    println!("   - 示例：sql!(SELECT * FROM users)");

    println!("\n过程宏项目结构：");
    println!("my_derive_macro/");
    println!("├── Cargo.toml");
    println!("│   [lib]");
    println!("│   proc-macro = true");
    println!("└── src/");
    println!("    └── lib.rs");

    println!("\n常用依赖：");
    println!("syn - 解析 Rust 代码为 AST");
    println!("quote - 生成 Rust 代码");
    println!("proc-macro2 - 稳定的 proc_macro API");

    println!("\n派生宏示例（概念）：");
    println!("#[derive(Builder)]");
    println!("pub struct User {{");
    println!("    name: String,");
    println!("    email: String,");
    println!("}}");
    println!("// 自动生成 UserBuilder 类型");
}

/// 演示宏最佳实践
fn demo_macro_best_practices() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. 宏最佳实践                      │");
    println!("└─────────────────────────────────────┘");

    println!("\n1. 优先使用函数：");
    println!("   - 函数更易读、更易调试");
    println!("   - 编译器提供更好的错误信息");
    println!("   - IDE 支持更好");

    println!("\n2. 宏的适用场景：");
    println!("   - 需要可变参数");
    println!("   - 需要编译时代码生成");
    println!("   - 需要操作语法结构");
    println!("   - 消除重复代码");

    println!("\n3. 文档注释：");
    println!("   - 使用 /// 为宏添加文档");
    println!("   - 解释宏的用途和参数");

    println!("\n4. 错误处理：");
    println!("   - 使用 compile_error! 报告错误");
    println!("   - 提供清晰的错误信息");

    println!("\n示例 - 带文档的宏：");
    /// 创建一个包含默认值的 HashMap
    /// 
    /// # 示例
    /// ```
    /// let map = default_map!("key" => "value");
    /// ```
    macro_rules! default_map {
        ($($key:expr => $value:expr),* $(,)?) => {{
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.to_string());
            )*
            map
        }};
    }

    let map = default_map!("name" => "Alice", "city" => "Beijing");
    println!("default_map! = {:?}", map);

    println!("\n5. 宏卫生性：");
    println!("   - 避免意外捕获外部变量");
    println!("   - 使用独特的内部变量名");
    println!("   - 使用 _ 前缀表示内部变量");
}

/// 公开的演示函数
pub fn demo_proc_macros() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块十：过程宏深入                  ║");
    println!("╚═════════════════════════════════════════╝");

    demo_declarative_macros();
    demo_repetition_patterns();
    demo_fragment_specifiers();
    demo_procedural_macros();
    demo_macro_best_practices();
}

/// 性能优化与基准测试示例模块
/// 涵盖：性能分析工具、基准测试、编译优化、内存布局

/// 演示性能分析工具
fn demo_profiling() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 性能分析工具                    │");
    println!("└─────────────────────────────────────┘");

    println!("\nperf - Linux 性能分析：");
    println!("# 编译带调试信息的发布版本");
    println!("RUSTFLAGS=\"-g\" cargo build --release");
    println!();
    println!("# 运行 perf");
    println!("perf record -g ./target/release/my_program");
    println!("perf report");

    println!("\ncargo-flamegraph - 火焰图：");
    println!("cargo install flamegraph");
    println!("cargo flamegraph");

    println!("\nvalgrind - 内存分析：");
    println!("# 内存泄漏检测");
    println!("valgrind --leak-check=full ./target/release/my_program");
    println!();
    println!("# 缓存分析");
    println!("valgrind --tool=cachegrind ./target/release/my_program");

    println!("\ncargo-bloat - 二进制大小分析：");
    println!("cargo install cargo-bloat");
    println!("cargo bloat --release");

    println!("\n常用分析场景：");
    println!("1. CPU 热点分析 - perf/flamegraph");
    println!("2. 内存泄漏 - valgrind/heaptrack");
    println!("3. 二进制大小 - cargo-bloat");
    println!("4. 编译时间 - cargo-timing");
}

/// 演示基准测试
fn demo_benchmarking() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 基准测试                        │");
    println!("└─────────────────────────────────────┘");

    println!("\n使用 criterion.rs：");
    println!("Cargo.toml:");
    println!("[dev-dependencies]");
    println!("criterion = {{ version = \"0.5\", features = [\"html_reports\"] }}");
    println!();
    println!("[[bench]]");
    println!("name = \"my_benchmark\"");
    println!("harness = false");

    println!("\n基准测试代码：");
    println!("use criterion::{{black_box, criterion_group, criterion_main, Criterion}};");
    println!();
    println!("fn fibonacci(n: u64) -> u64 {{ ... }}");
    println!();
    println!("fn criterion_benchmark(c: &mut Criterion) {{");
    println!("    c.bench_function(\"fib 20\", |b| b.iter(|| fibonacci(black_box(20))));");
    println!("}}");
    println!();
    println!("criterion_group!(benches, criterion_benchmark);");
    println!("criterion_main!(benches);");

    println!("\n运行基准测试：");
    println!("cargo bench");

    println!("\nblack_box 的作用：");
    println!("防止编译器优化掉计算结果");
    println!("确保代码按预期执行");

    println!("\n基准测试最佳实践：");
    println!("1. 测量真实场景");
    println!("2. 避免微基准测试陷阱");
    println!("3. 比较多个实现");
    println!("4. 记录历史结果");
}

/// 演示编译优化
fn demo_compiler_optimizations() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. 编译优化选项                    │");
    println!("└─────────────────────────────────────┘");

    println!("\nProfile 配置：");
    println!("[profile.release]");
    println!("opt-level = 3        # 优化级别 (0-3, s, z)");
    println!("lto = \"fat\"          # 链接时优化");
    println!("codegen-units = 1    # 代码生成单元");
    println!("panic = \"abort\"      # panic 策略");
    println!("strip = true         # 移除符号");

    println!("\nopt-level 选项：");
    println!("0 - 不优化（最快编译）");
    println!("1 - 基本优化");
    println!("2 - 推荐优化");
    println!("3 - 最大优化（最慢编译）");
    println!("s - 优化大小");
    println!("z - 更激进的大小优化");

    println!("\nLTO (Link-Time Optimization)：");
    println!("false - 不使用 LTO");
    println!("thin - 薄 LTO（较快）");
    println!("fat - 完全 LTO（最优化）");

    println!("\nPGO (Profile-Guided Optimization)：");
    println!("# 1. 编译带插桩版本");
    println!("RUSTFLAGS=\"-C profile-generate=./pgo-data\" cargo build --release");
    println!();
    println!("# 2. 运行典型工作负载");
    println!("./target/release/my_program");
    println!();
    println!("# 3. 使用收集的数据重新编译");
    println!("RUSTFLAGS=\"-C profile-use=./pgo-data\" cargo build --release");

    println!("\n自定义 Profile：");
    println!("[profile.release-with-debug]");
    println!("inherits = \"release\"");
    println!("debug = true");
}

/// 演示内存布局优化
fn demo_memory_layout() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. 内存布局优化                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n字段排序影响大小：");
    println!("struct Unoptimized {{");
    println!("    a: u8,   // 1 byte + 7 padding");
    println!("    b: u64,  // 8 bytes");
    println!("    c: u8,   // 1 byte + 7 padding");
    println!("}}  // 总共 24 bytes");
    println!();
    println!("struct Optimized {{");
    println!("    b: u64,  // 8 bytes");
    println!("    a: u8,   // 1 byte");
    println!("    c: u8,   // 1 byte + 6 padding");
    println!("}}  // 总共 16 bytes");

    println!("\n#[repr] 属性：");
    println!("#[repr(C)] - C 内存布局");
    println!("#[repr(packed)] - 紧凑布局（无填充）");
    println!("#[repr(align(n))] - 指定对齐");
    println!("#[repr(u8)] - 枚举大小");

    println!("\n小型容器优化：");
    println!("SmallVec - 栈上小数组");
    println!("SmartString - 内联小字符串");
    println!("SmallBox - 栈上小 Box");

    println!("\n避免不必要的分配：");
    println!("1. 使用引用代替克隆");
    println!("2. 使用 Cow<str> 延迟克隆");
    println!("3. 预分配容量 (Vec::with_capacity)");
    println!("4. 重用缓冲区");

    println!("\n内存分析工具：");
    println!("std::mem::size_of::<T>() - 类型大小");
    println!("std::mem::align_of::<T>() - 类型对齐");
    println!("std::mem::size_of_val(&val) - 值大小");
}

/// 演示 SIMD 向量化
fn demo_simd() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. SIMD 向量化                     │");
    println!("└─────────────────────────────────────┘");

    println!("\nSIMD 概念：");
    println!("Single Instruction, Multiple Data");
    println!("一条指令处理多个数据");
    println!("显著提升数值计算性能");

    println!("\nRust SIMD 支持：");
    println!("#![feature(portable_simd)]");
    println!("use std::simd::*;");

    println!("\n示例 - 向量加法：");
    println!("fn sum_simd(data: &[f32]) -> f32 {{");
    println!("    let chunks = data.chunks_exact(8);");
    println!("    let mut sum = Simd::splat(0.0f32);");
    println!("    for chunk in chunks {{");
    println!("        let vec = Simd::from_slice(chunk);");
    println!("        sum += vec;");
    println!("    }}");
    println!("    sum.reduce_sum()");
    println!("}}");

    println!("\n自动向量化：");
    println!("编译器可以自动向量化简单循环");
    println!("条件：");
    println!("1. 循环次数已知");
    println!("2. 无数据依赖");
    println!("3. 无函数调用");

    println!("\n手动向量化提示：");
    println!("#[target_feature(enable = \"avx2\")]");
    println!("unsafe fn vectorized_add(...) {{ ... }}");

    println!("\nSIMD 最佳实践：");
    println!("1. 先测量，再优化");
    println!("2. 考虑可移植性");
    println!("3. 处理边界情况");
    println!("4. 使用 benchmark 验证");
}

/// 公开的演示函数
pub fn demo_performance() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块十四：性能优化与基准测试        ║");
    println!("╚═════════════════════════════════════════╝");

    demo_profiling();
    demo_benchmarking();
    demo_compiler_optimizations();
    demo_memory_layout();
    demo_simd();
}

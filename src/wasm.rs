/// WebAssembly 开发示例模块
/// 涵盖：编译到 WASM、wasm-bindgen、JavaScript 交互、性能优化

/// 演示编译到 WASM
fn demo_compile_to_wasm() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 编译到 WebAssembly              │");
    println!("└─────────────────────────────────────┘");

    println!("\n添加目标：");
    println!("rustup target add wasm32-unknown-unknown");
    println!("rustup target add wasm32-wasi");

    println!("\n目标说明：");
    println!("wasm32-unknown-unknown - 纯 WASM，无系统接口");
    println!("wasm32-wasi - WASI 系统接口");
    println!("wasm32-unknown-emscripten - Emscripten 兼容");

    println!("\n编译命令：");
    println!("cargo build --target wasm32-unknown-unknown --release");

    println!("\nCargo.toml 配置：");
    println!("[lib]");
    println!("crate-type = [\"cdylib\", \"rlib\"]");
    println!();
    println!("[profile.release]");
    println!("opt-level = \"s\"");
    println!("lto = true");

    println!("\n输出文件：");
    println!("target/wasm32-unknown-unknown/release/my_lib.wasm");

    println!("\n工具链：");
    println!("wasm-pack - 打包和发布");
    println!("wasm-bindgen - JavaScript 绑定");
    println!("wasm-opt - 优化 WASM 二进制");
}

/// 演示 wasm-bindgen
fn demo_wasm_bindgen() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. wasm-bindgen                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n基本用法：");
    println!("use wasm_bindgen::prelude::*;");
    println!();
    println!("#[wasm_bindgen]");
    println!("pub fn add(a: i32, b: i32) -> i32 {{");
    println!("    a + b");
    println!("}}");

    println!("\n导出结构体：");
    println!("#[wasm_bindgen]");
    println!("pub struct Calculator {{");
    println!("    value: i32,");
    println!("}}");
    println!();
    println!("#[wasm_bindgen]");
    println!("impl Calculator {{");
    println!("    #[wasm_bindgen(constructor)]");
    println!("    pub fn new() -> Self {{ Self {{ value: 0 }} }}");
    println!();
    println!("    pub fn add(&mut self, n: i32) {{ self.value += n; }}");
    println!("    pub fn get(&self) -> i32 {{ self.value }}");
    println!("}}");

    println!("\n调用 JavaScript：");
    println!("#[wasm_bindgen]");
    println!("extern \"C\" {{");
    println!("    #[wasm_bindgen(js_namespace = console)]");
    println!("    fn log(s: &str);");
    println!();
    println!("    #[wasm_bindgen(js_namespace = Math)]");
    println!("    fn random() -> f64;");
    println!("}}");

    println!("\n类型映射：");
    println!("Rust           JavaScript");
    println!("i32, u32       Number");
    println!("f64            Number");
    println!("bool           Boolean");
    println!("String         String");
    println!("Vec<T>         Array");
    println!("JsValue        any");

    println!("\n构建命令：");
    println!("wasm-pack build --target web");
    println!("wasm-pack build --target nodejs");
}

/// 演示 JavaScript 交互
fn demo_js_interop() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. JavaScript 交互                 │");
    println!("└─────────────────────────────────────┘");

    println!("\n接收 JavaScript 对象：");
    println!("#[wasm_bindgen]");
    println!("pub fn process_object(obj: JsValue) {{");
    println!("    if let Some(obj) = obj.dyn_ref::<js_sys::Object>() {{");
    println!("        // 处理对象");
    println!("    }}");
    println!("}}");

    println!("\n返回 JavaScript 对象：");
    println!("#[wasm_bindgen]");
    println!("pub fn create_object() -> JsValue {{");
    println!("    let obj = js_sys::Object::new();");
    println!("    js_sys::Reflect::set(&obj, &\"name\".into(), &\"Rust\".into());");
    println!("    obj.into()");
    println!("}}");

    println!("\nPromise 支持：");
    println!("#[wasm_bindgen]");
    println!("pub async fn fetch_data(url: String) -> Result<JsValue, JsValue> {{");
    println!("    let promise = js_sys::Promise::resolve(&url);");
    println!("    wasm_bindgen_futures::JsFuture::from(promise).await");
    println!("}}");

    println!("\n回调函数：");
    println!("#[wasm_bindgen]");
    println!("pub fn set_callback(callback: js_sys::Function) {{");
    println!("    let this = JsValue::null();");
    println!("    callback.call1(&this, &\"Hello\".into()).unwrap();");
    println!("}}");

    println!("\n常用库：");
    println!("js-sys - JavaScript 标准库绑定");
    println!("web-sys - Web API 绑定");
    println!("wasm-bindgen-futures - Future/Promise 转换");
}

/// 演示 WASM 性能优化
fn demo_wasm_performance() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. WASM 性能优化                   │");
    println!("└─────────────────────────────────────┘");

    println!("\n减小二进制大小：");
    println!("[profile.release]");
    println!("opt-level = \"s\"      # 优化大小");
    println!("lto = true           # 链接时优化");
    println!("codegen-units = 1    # 单代码生成单元");
    println!("panic = \"abort\"      # 不需要 unwind");
    println!("strip = true         # 移除符号");

    println!("\nwasm-opt 优化：");
    println!("wasm-opt -Oz -o output.wasm input.wasm");
    println!("-Oz - 最大大小优化");
    println!("-O3 - 最大性能优化");
    println!("-Os - 平衡优化");

    println!("\n内存管理：");
    println!("避免频繁分配");
    println!("使用 wasm-bindgen 的内存视图");
    println!("复用缓冲区");

    println!("\nSIMD 支持：");
    println!("RUSTFLAGS=\"-C target-feature=+simd128\" \\");
    println!("  cargo build --target wasm32-unknown-unknown --release");

    println!("\n性能分析：");
    println!("console.time/timeEnd");
    println!("浏览器开发者工具");
    println!("wasm-pack 的 --profiling 选项");

    println!("\n最佳实践：");
    println!("1. 减少 JS/WASM 边界调用");
    println!("2. 批量传递数据");
    println!("3. 使用共享内存");
    println!("4. 避免字符串转换");
}

/// 演示 Web 框架集成
fn demo_web_integration() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. Web 框架集成                    │");
    println!("└─────────────────────────────────────┘");

    println!("\nwasm-pack 输出目标：");
    println!("--target web - ES 模块");
    println!("--target nodejs - Node.js");
    println!("--target bundler - Webpack/Rollup");
    println!("--target no-modules - 无模块系统");

    println!("\nJavaScript 使用：");
    println!("import init, {{ add, Calculator }} from './my_lib.js';");
    println!();
    println!("async function main() {{");
    println!("    await init();");
    println!("    console.log(add(1, 2));");
    println!("    const calc = Calculator.new();");
    println!("    calc.add(10);");
    println!("}}");

    println!("\nYew - WASM 前端框架：");
    println!("use yew::prelude::*;");
    println!();
    println!("#[function_component(App)]");
    println!("fn app() -> Html {{");
    println!("    html! {{");
    println!("        <div>{{ \"Hello, Yew!\" }}</div>");
    println!("    }}");
    println!("}}");

    println!("\nLeptos - 响应式框架：");
    println!("#[component]");
    println!("fn App() -> impl IntoView {{");
    println!("    let count = create_rw_signal(0);");
    println!("    view! {{");
    println!("        <button on:click=move |_| count.update(|n| *n += 1)>");
    println!("            {{count}}");
    println!("        </button>");
    println!("    }}");
    println!("}}");

    println!("\n框架选择：");
    println!("Yew - 类 React，成熟稳定");
    println!("Leptos - 响应式，性能优秀");
    println!("Sycamore - 细粒度响应式");
    println!("Seed - Elm 架构");
}

/// 公开的演示函数
pub fn demo_wasm() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块十八：WebAssembly 开发          ║");
    println!("╚═════════════════════════════════════════╝");

    demo_compile_to_wasm();
    demo_wasm_bindgen();
    demo_js_interop();
    demo_wasm_performance();
    demo_web_integration();
}

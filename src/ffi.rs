/// FFI 与跨语言互操作示例模块
/// 涵盖：调用 C 代码、为 C 提供接口、安全封装模式

/// 演示调用 C 代码
fn demo_calling_c() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. 调用 C 代码                     │");
    println!("└─────────────────────────────────────┘");

    println!("\n基本 extern 声明：");
    println!("extern \"C\" {{");
    println!("    fn abs(x: c_int) -> c_int;");
    println!("    fn strlen(s: *const c_char) -> usize;");
    println!("}}");

    println!("\n调用 C 函数：");
    println!("unsafe {{");
    println!("    let result = abs(-42);");
    println!("    println!(\"abs(-42) = {{}}\", result);");
    println!("}}");

    println!("\nC 类型映射：");
    println!("Rust 类型        C 类型");
    println!("c_int          int");
    println!("c_char         char");
    println!("c_void         void");
    println!("*const T       const T*");
    println!("*mut T         T*");

    println!("\n链接 C 库：");
    println!("// build.rs");
    println!("fn main() {{");
    println!("    cc::Build::new()");
    println!("        .file(\"src/native.c\")");
    println!("        .compile(\"native\");");
    println!("}}");

    println!("\n回调函数：");
    println!("type Callback = extern \"C\" fn(i32, i32) -> i32;");
    println!();
    println!("extern \"C\" fn my_callback(a: i32, b: i32) -> i32 {{");
    println!("    a + b");
    println!("}}");
}

/// 演示为 C 提供接口
fn demo_exporting_to_c() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 为 C 提供接口                   │");
    println!("└─────────────────────────────────────┘");

    println!("\n导出函数：");
    println!("#[no_mangle]");
    println!("pub extern \"C\" fn rust_add(a: i32, b: i32) -> i32 {{");
    println!("    a + b");
    println!("}}");

    println!("\n#[no_mangle] 的作用：");
    println!("防止名称修饰（name mangling）");
    println!("保持函数名与 C 兼容");

    println!("\nextern \"C\" 的作用：");
    println!("使用 C 调用约定");
    println!("确保 ABI 兼容");

    println!("\n导出结构体：");
    println!("#[repr(C)]");
    println!("pub struct Point {{");
    println!("    pub x: f64,");
    println!("    pub y: f64,");
    println!("}}");

    println!("\n#[repr(C)] 的作用：");
    println!("使用 C 内存布局");
    println!("字段顺序与定义一致");
    println!("对齐方式与 C 兼容");

    println!("\n不透明类型模式：");
    println!("// Rust 侧");
    println!("pub struct Database {{ ... }}");
    println!();
    println!("// C 侧");
    println!("typedef struct DatabaseHandle DatabaseHandle;");
    println!("DatabaseHandle* database_new();");
    println!("void database_free(DatabaseHandle* db);");
}

/// 演示字符串处理
fn demo_string_handling() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. FFI 字符串处理                  │");
    println!("└─────────────────────────────────────┘");

    println!("\nCString 和 CStr：");
    println!("CString - 拥有所有权的 C 字符串");
    println!("CStr - 借用的 C 字符串");

    println!("\nRust 字符串 -> C 字符串：");
    println!("let s = CString::new(\"Hello\")?;");
    println!("let ptr: *const c_char = s.as_ptr();");

    println!("\nC 字符串 -> Rust 字符串：");
    println!("let c_str: &CStr = unsafe {{ CStr::from_ptr(ptr) }};");
    println!("let rust_str: &str = c_str.to_str()?;");

    println!("\n处理 NULL 字符：");
    println!("CString::new() 会在遇到 \\0 时返回错误");
    println!("使用 CString::new() 确保字符串有效");

    println!("\n字符串所有权：");
    println!("CString::into_raw() - 转移所有权给 C");
    println!("CString::from_raw() - 从 C 取回所有权");
    println!("注意：必须配对使用，否则内存泄漏");

    println!("\n示例函数：");
    println!("#[no_mangle]");
    println!("pub extern \"C\" fn create_string() -> *mut c_char {{");
    println!("    let s = CString::new(\"Hello from Rust\").unwrap();");
    println!("    s.into_raw()");
    println!("}}");
    println!();
    println!("#[no_mangle]");
    println!("pub extern \"C\" fn free_string(s: *mut c_char) {{");
    println!("    unsafe {{ let _ = CString::from_raw(s); }}");
    println!("}}");
}

/// 演示安全封装模式
fn demo_safe_wrappers() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. 安全封装模式                    │");
    println!("└─────────────────────────────────────┘");

    println!("\nRAII 包装：");
    println!("pub struct FileDescriptor {{");
    println!("    fd: i32,");
    println!("}}");
    println!();
    println!("impl FileDescriptor {{");
    println!("    pub fn open(path: &str) -> io::Result<Self> {{ ... }}");
    println!("    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {{ ... }}");
    println!("}}");
    println!();
    println!("impl Drop for FileDescriptor {{");
    println!("    fn drop(&mut self) {{");
    println!("        unsafe {{ libc::close(self.fd); }}");
    println!("    }}");
    println!("}}");

    println!("\n类型状态模式：");
    println!("pub struct Owned;");
    println!("pub struct Borrowed<'a>(PhantomData<&'a ()>);");
    println!();
    println!("pub struct CString<Ownership = Owned> {{");
    println!("    ptr: *mut c_char,");
    println!("    _ownership: PhantomData<Ownership>,");
    println!("}}");

    println!("\n错误处理：");
    println!("将 C 错误码转换为 Rust Result");
    println!("使用 std::io::Error 封装系统错误");
    println!("提供清晰的错误信息");

    println!("\n安全保证：");
    println!("1. 所有 unsafe 代码封装在安全 API 内");
    println!("2. 使用类型系统防止误用");
    println!("3. 正确处理资源生命周期");
    println!("4. 提供清晰的文档");
}

/// 演示 bindgen 和 cbindgen
fn demo_codegen_tools() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. bindgen 与 cbindgen             │");
    println!("└─────────────────────────────────────┘");

    println!("\nbindgen - 从 C 生成 Rust 绑定：");
    println!("// build.rs");
    println!("fn main() {{");
    println!("    let bindings = bindgen::Builder::default()");
    println!("        .header(\"wrapper.h\")");
    println!("        .generate()");
    println!("        .unwrap();");
    println!("    bindings.write_to_file(out_path.join(\"bindings.rs\"));");
    println!("}}");

    println!("\ncbindgen - 从 Rust 生成 C 头文件：");
    println!("// build.rs");
    println!("fn main() {{");
    println!("    cbindgen::Builder::new()");
    println!("        .with_crate(crate_dir)");
    println!("        .generate();");
    println!("        .unwrap();");
    println!("        .write_to_file(\"target/mylib.h\");");
    println!("}}");

    println!("\n常用配置：");
    println!("bindgen.toml:");
    println!("- 指定要生成的类型");
    println!("- 配置类型映射");
    println!("- 处理宏定义");

    println!("\ncbindgen.toml:");
    println!("- 配置输出格式");
    println!("- 指定导出的项");
    println!("- 设置命名约定");

    println!("\n自动化工作流：");
    println!("1. 编写 Rust 代码");
    println!("2. cbindgen 生成 C 头文件");
    println!("3. C 代码调用 Rust 库");
    println!("4. 或反向：C 库 -> bindgen -> Rust 调用");
}

/// 公开的演示函数
pub fn demo_ffi() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块十二：FFI 与跨语言互操作        ║");
    println!("╚═════════════════════════════════════════╝");

    demo_calling_c();
    demo_exporting_to_c();
    demo_string_handling();
    demo_safe_wrappers();
    demo_codegen_tools();
}

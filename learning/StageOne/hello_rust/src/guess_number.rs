// 导入随机数生成器的 trait（特征）
// rand 是外部包（在 Cargo.toml 中声明的依赖）
// 你需要先在 Cargo.toml 的 [dependencies] 部分添加 rand = "0.8"
// :: 是路径分隔符，类似文件系统的 /，表示"的"或"里面的"
// Rng 是 Random Number Generator 的缩写，是一个 trait（后面会用到）
// trait 类似 Java 的 interface，定义了一组方法的签名
// 导入 trait 后，才能使用它的方法（如 gen_range）
use rand::Rng;

// 导入标准库中的比较结果枚举类型
// std 是 Rust 标准库，自动可用，不需要在 Cargo.toml 中声明
// cmp 是 compare（比较）模块
// Ordering 是一个枚举，有三个值：Less（小于）、Greater（大于）、Equal（等于）
use std::cmp::Ordering;

// 导入标准库的输入输出模块
// io 模块包含了处理输入输出的功能
use std::io;

pub fn guess_number() {
    // 【println!】是一个宏（注意感叹号！），不是函数
    // 宏在编译时展开，可以接受可变数量的参数
    // 宏可以做函数做不到的事，比如接受可变数量和类型的参数
    // println! 打印到标准输出（stdout），并在末尾自动添加换行符 \n
    // 还有其他相关的宏：
    // - print! - 打印但不换行
    // - eprintln! - 打印到标准错误（stderr），用于错误信息
    // - dbg! - 打印变量的调试信息，包括文件名、行号和值
    // 双引号 "" 表示字符串字面量（&str 类型）
    println!("Guess the number!");
    println!("Please input your number!");

    // 【let】用于声明变量（类似 Java 的 final，默认不可变）
    // secret_number 是变量名，Rust 使用 snake_case 命名规范
    // 变量名规则：必须以字母或下划线开头，可包含字母、数字、下划线
    // Rust 会根据赋值自动推断类型，这里推断为 i32（32位有符号整数）
    // = 右边是一个表达式链，让我们逐步分解：
    // 1. rand::thread_rng() - 调用 rand 包的 thread_rng 函数
    //    返回一个线程本地的随机数生成器（ThreadRng 类型）
    //    线程本地意味着每个线程有自己的生成器，避免竞争
    // 2. .gen_range(1..=100) - 在生成器上调用 gen_range 方法
    //    gen_range 来自 Rng trait（这就是为什么要 use rand::Rng）
    //    如果没有 use rand::Rng，这个方法将不可用
    //    1..=100 是一个包含式范围（inclusive range），包含 1 和 100
    //    如果写 1..100 则是 1 到 99（不包含 100）
    //    还有其他范围语法：
    //    - 1.. 表示从 1 到无穷大
    //    - ..100 表示从负无穷到 99
    //    - ..=100 表示从负无穷到 100
    //    - .. 表示完整范围
    // 语句末尾的分号 ; 表示这是一个语句，不返回值
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // 【let mut】声明一个可变变量
    // mut 是 mutable（可变）的缩写
    // 在 Rust 中，变量默认不可变，这是为了安全性
    // 不可变意味着一旦赋值就不能改变，类似 const 但更严格
    // count 被初始化为 0，Rust 自动推断类型为 i32（有符号 32 位整数）
    // 为什么推断为 i32 而不是 u32？因为整数字面量默认是 i32
    // 如果想要其他类型，可以：
    // - 显式标注：let mut count: u32 = 0;
    // - 后缀标注：let mut count = 0u32;
    // - 后续使用会影响推断：如果后面有 count += 1u32，会推断为 u32
    let mut count = 0;

    // 【loop】创建一个无限循环
    // 与 while true 类似，但在 Rust 中 loop 是推荐的无限循环方式
    // 只能通过 break 退出循环，或者通过 return 退出函数
    // 退出 loop 的几种方式：
    // 1. break - 正常退出循环，继续执行循环后的代码
    // 2. break 值 - 退出循环并返回一个值（loop 可以有返回值）
    // 3. return - 直接退出整个函数
    // 4. panic! - 程序崩溃，打印错误信息和调用栈（不推荐，除非遇到不可恢复的错误）
    // 5. std::process::exit(code) - 立即终止进程（非常规手段）
    // 另外，continue 可以跳过本次循环，直接开始下一次循环
    loop {
        // 每次循环都创建一个新的空字符串
        // String 是 Rust 的字符串类型（堆分配的、可增长的 UTF-8 字符串）
        // String::new() 是调用 String 类型的关联函数（类似静态方法）
        // :: 在这里表示"关联函数"，不需要实例就可以调用
        let mut guess = String::new();

        // 【方法链式调用】Rust 支持链式调用方法
        // io::stdin() - 获取标准输入的句柄（handle）
        //   stdin 是 standard input（标准输入）的缩写
        // .read_line(&mut guess) - 从标准输入读取一行
        //   & 表示引用（借用），不转移所有权
        //   &mut 表示可变引用，允许函数修改 guess 的内容
        //   为什么需要引用？因为 Rust 的所有权规则：
        //   - 每个值都有一个所有者
        //   - 值在任意时刻只能有一个所有者
        //   - 当所有者离开作用域，值被丢弃
        //   如果不用引用，guess 的所有权会转移给 read_line，之后就不能再用了
        //   read_line 会将用户输入（包括换行符 \n）追加到 guess 字符串
        //   返回值是 io::Result<usize>，其中 usize 是读取的字节数
        // .expect("Failed to read line") - 错误处理
        //   read_line 返回 Result<T, E> 类型，这是 Rust 的错误处理方式
        //   Result 是个枚举，有两个变体：
        //   - Ok(T) 表示操作成功，包含结果值
        //   - Err(E) 表示操作失败，包含错误信息
        //   expect 在 Ok 时返回内部的值，在 Err 时 panic 并显示消息
        //   可能的错误：EOF（文件结束）、权限问题、IO 中断等
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 【变量遮蔽（Shadowing）】
        // 这里重新声明了 guess 变量，类型从 String 变为 u32
        // 这在 Rust 中是允许的，称为变量遮蔽
        // : u32 是类型注解，告诉 Rust 我们想要解析成 u32 类型
        // u32 是无符号（unsigned）32 位整数，范围是 0 到 4,294,967,295
        let guess: u32 = guess
            .trim() // 去除字符串两端的空白字符（空格、换行符等）
            .parse() // 将字符串解析为数字，返回 Result<u32, ParseIntError>
            .expect("Please type a number!"); // 如果解析失败则 panic

        // 【注意】上面的 expect 会导致程序 panic（崩溃）
        // 如果用户输入了非数字（如 "abc"），程序会：
        // 1. 打印 "Please type a number!"
        // 2. 打印 panic 的详细信息和调用栈
        // 3. 整个程序终止
        //
        // 更好的错误处理方式是使用 match：
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("请输入有效数字！");
        //         continue;  // 跳过本次循环，重新开始
        //     }
        // };
        // 这样程序不会崩溃，而是友好地提示用户重新输入

        // 【字符串插值】
        // {} 是占位符，会被变量的值替换
        // {guess} 是简写形式，等价于 "{}", guess
        // 其他格式化选项：
        // - {:?} Debug 格式（用于调试）
        // - {:#?} 美化的 Debug 格式
        // - {:b} 二进制
        // - {:x} 十六进制（小写）
        // - {:X} 十六进制（大写）
        // - {:.2} 浮点数保留 2 位小数
        // - {:>10} 右对齐，宽度 10
        // - {:<10} 左对齐，宽度 10
        // - {:^10} 居中对齐，宽度 10
        // 可以用位置参数：println!("{0} {1} {0}", "hello", "world")
        // 也可以用命名参数：println!("{name} {age}", name="Alice", age=25)
        println!("You guessed: {guess}");

        // 【if 条件语句】
        // 条件不需要括号（与 C/Java 不同），但条件必须是 bool 类型
        // Rust 不会自动将数字转换为 bool（0 和 1 不等同于 false 和 true）
        // == 是相等比较运算符，返回 bool
        // 其他比较运算符：!= (不等), < (小于), > (大于), <= (小于等于), >= (大于等于)
        // if 是表达式，可以有返回值：
        // let result = if condition { value1 } else { value2 };
        // 大括号 {} 定义代码块，即使只有一行也必须使用（与 Python 不同）
        if count == 5 {
            // 在猜了 5 次后给出提示，显示秘密数字
            // 这行代码会在 count 恰好等于 5 时执行
            println!("The secret number is: {secret_number}");
        }
        // 没有 else 分支时，如果条件为 false，什么都不做

        // 检查是否已经猜了太多次
        // 注意这里是 >（大于）而不是 >=（大于等于）
        // 所以第 11 次猜测时才会触发失败
        if count > 10 {
            println!("You lose!");
            // 【break】退出最近的循环（这里是 loop）
            // break 后的代码不会执行，直接跳到 loop 结束的地方
            // 如果有嵌套循环，break 只退出最内层循环
            // 可以用标签退出指定循环：
            // 'outer: loop {
            //     loop {
            //         break 'outer; // 退出外层循环
            //     }
            // }
            break;
        }

        // 【match 模式匹配】
        // match 是 Rust 的强大特性，类似 switch 但更强大
        // 必须覆盖所有可能的情况（穷尽性检查）
        // guess.cmp(&secret_number) 比较两个数字
        // cmp 方法定义在 Ord trait 中，所有可比较的类型都实现了它
        // & 是因为 cmp 需要另一个值的引用作为参数
        match guess.cmp(&secret_number) {
            // 每个分支的格式是：模式 => 表达式/代码块
            // Ordering::Less 表示 guess < secret_number
            Ordering::Less => {
                println!("Too small!");
                // += 是复合赋值运算符，等价于 count = count + 1
                count += 1;
            }
            // Ordering::Greater 表示 guess > secret_number
            Ordering::Greater => {
                println!("Too big!");
                count += 1;
            }
            // Ordering::Equal 表示 guess == secret_number
            Ordering::Equal => {
                println!("You win!");
                // 猜对了，退出循环
                break;
            } // 注意：match 必须处理所有情况
              // Ordering 只有这三个值，所以我们已经覆盖了所有情况
        }
    }
    // 循环结束后，程序也结束了
    // main 函数没有显式的 return，默认返回 ()（unit 类型）
    // () 是 Rust 的 unit 类型，类似其他语言的 void，表示"没有值"
    // 实际上 main 函数可以返回 Result：
    // fn main() -> Result<(), Box<dyn std::error::Error>> {
    //     // 这样可以用 ? 运算符处理错误
    //     Ok(())
    // }
    // 程序正常结束时返回退出码 0，panic 时返回非 0
}

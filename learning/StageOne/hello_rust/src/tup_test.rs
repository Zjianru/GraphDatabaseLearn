// tup_test.rs
// 本文件演示了 Rust 的基础语法，包括元组、数组、函数、作用域、循环、条件判断等
// 每个函数、变量、语句、参数、语法点都配有详细注释，适合零基础初学者学习

/// 运行所有演示代码的主入口函数
pub fn run_tup_test() {
    // 定义一个元组，元组类型为 (i32, f64, u8)
    // i32: 32位有符号整数，f64: 64位浮点数，u8: 8位无符号整数
    let tup: (i32, f64, u8) = (500, 6.4, 1); // 变量名 tup，类型由右侧赋值推断
    // 解构元组，将元组的每个元素分别赋值给变量
    // _x: 第一个元素，未使用，前加下划线避免警告
    // y: 第二个元素，f64 类型
    // _z: 第三个元素，未使用
    let (_x, y, _z) = tup;
    // 打印元组中第二个元素 y，{} 占位符用于格式化输出
    println!("The value of y is: {}", y); // println! 是宏，用于输出内容到终端

    // 定义一个整型数组，类型为 [i32; 5]，即 5 个 i32 元素
    let a = [1, 2, 3, 4, 5]; // 直接用中括号和逗号分隔的方式初始化
    // 定义一个所有元素都为 3 的数组，长度为 5
    let b = [3; 5]; // [值; 长度] 语法，等价于 [3, 3, 3, 3, 3]
    // {:?} 用于调试格式输出数组，适合打印复杂类型
    println!("a: {:?}, b: {:?}", a, b);

    // 调用 test_return_method 函数，传入参数 1 和 2
    // test_return_method 返回两个参数的和，类型为 i32
    // {} 占位符用于输出返回值
    println!("testReturnMethod: {}", test_return_method(1, 2));
    // 调用 test_method 函数，传入参数 1 和 2，无返回值
    test_method(1, 2);

    // 演示表达式块的返回值
    println!("evl");
    evl(); // evl 函数内部有注释解释
    // 演示表达式块返回 unit 类型 ()
    println!("evl2");
    evl2(); // evl2 函数内部有注释解释

    // 调用 check 函数，传入参数 11，演示 if-else 返回值
    check(11);

    // 调用 for 循环演示函数，遍历数组
    loop_for();
    // 调用 for 循环遍历范围并反向输出
    loop_for_range_rev();
    // 调用带标签的嵌套循环演示
    loop_tag();
    // 调用 while 循环演示
    loop_while();
}

/// 一个带返回值的函数，返回两个 i32 相加的结果
/// 参数 value1: 第一个加数，类型 i32
/// 参数 value2: 第二个加数，类型 i32
/// 返回值: 两个参数的和，类型 i32
pub fn test_return_method(value1: i32, value2: i32) -> i32 {
    // 打印当前函数名，方便调试
    println!("now in testReturnMethod");
    // Rust 函数的返回值可以省略 return 关键字，直接写表达式且不加分号
    // 这里返回 value1 + value2 的结果
    value1 + value2 // 没有分号，作为表达式返回结果
}

/// 一个无返回值的函数，演示参数传递和打印
/// 参数 value1: 第一个参数，类型 i32
/// 参数 value2: 第二个参数，类型 i32
/// 无返回值，返回类型为 ()（unit 类型）
pub fn test_method(value1: i32, value2: i32) {
    // 打印当前函数名
    println!("now in testMethod");
    // 打印 value1 的值，{} 占位符用于格式化输出
    println!("The value of value1 is: {}", value1);
    // 打印 value2 的值
    println!("The value of value2 is: {}", value2);
}

/// 演示 Rust 表达式块的返回值
/// y 的值为 7，因为块的最后一行是 x + 1，没有分号
pub fn evl() {
    // 定义变量 y，类型由右侧表达式块推断
    let y = {
        // 块作用域内定义变量 x，类型 i32，值为 6
        let x = 6;
        // 块的最后一行是 x + 1，没有分号，作为块的返回值
        x + 1
    };
    // 打印 y 的值，应该为 7
    println!("y: {}", y);
}

/// 演示 Rust 表达式块带分号时返回 unit 类型 ()
/// y 的值为 ()，因为块的最后一行有分号，返回 unit
pub fn evl2() {
    // 定义变量 y，类型为 ()，即 unit 类型
    let y = {
        let x = 6; // 块作用域内定义变量 x
        x + 1;     // 有分号，块返回 ()
    };
    // 打印 y 的值，使用 {:?} 因为 y 是 unit 类型
    println!("y: {:?}", y);
}

/// 演示 if-else 条件判断，返回 bool 类型
/// 参数 value: 需要判断的整数，类型 i32
/// 返回值: bool 类型，根据条件判断结果返回 true 或 false
pub fn check(value: i32) -> bool {
    // 打印函数名
    println!("check");
    // if 语句判断 value 是否大于 0
    // Rust 的 if 语句条件必须是 bool 类型，不能像 C 语言那样用数字
    if value > 0 {
        // 如果 value > 0，返回 true
        true
    } else if value < 5 {
        // 如果 value <= 0 且 < 5，返回 false
        false
    } else {
        // 其他情况返回 true
        true
    }
}

/// 演示带标签的嵌套循环和 break 的用法
pub fn loop_tag() {
    // 打印函数名
    println!("loop_tag");
    // 定义可变变量 remaining，类型 i32，初始值 10
    let mut remaining = 10;
    // 外层循环，标签为 'counting_up，方便 break 跳出指定循环
    'counting_up: loop {
        // 内层无限循环
        loop {
            // 打印当前 remaining 的值
            println!("remaining = {}", remaining);
            // remaining 自减 1
            remaining -= 1;
            // 如果 remaining == 9，跳出内层循环（不带标签）
            if remaining == 9 {
                println!("break from inner loop, no tag");
                break; // 只跳出内层循环
            }
            // 如果 remaining == 2，跳出外层循环（带标签）
            if remaining == 2 {
                println!("break from inner loop, with tag");
                break 'counting_up; // 跳出外层循环
            }
        }
    }
}

/// 演示 while 循环的用法
pub fn loop_while() {
    // 打印函数名
    println!("loop_while");
    // 定义可变变量 number，类型 i32，初始值 3
    let mut number = 3;
    // while 循环，条件为 number != 0 时继续循环
    while number != 0 {
        // 打印当前 number 的值
        println!("{}!", number);
        // number 自减 1
        number -= 1;
    }
    // 循环结束后打印 LIFTOFF!!!
    println!("LIFTOFF!!!");
}

/// 演示 for 循环遍历数组
pub fn loop_for() {
    // 打印函数名
    println!("loop_for");
    // 定义数组 a，类型 [i32; 5]
    let a = [10, 20, 30, 40, 50];
    // for 循环遍历数组 a，.iter() 返回数组的迭代器
    for element in a.iter() {
        // element 类型为 &i32，表示对数组元素的引用
        println!("the value is: {}", element);
    }
}

/// 演示 for 循环遍历范围并反向输出
pub fn loop_for_range_rev() {
    // 打印函数名
    println!("loop_for_range_rev");
    // (1..4) 创建一个范围，包含 1, 2, 3（不包含 4）
    // .rev() 方法将范围反转，输出顺序为 3, 2, 1
    for number in (1..4).rev() {
        // number 类型为 i32
        println!("{}!", number);
    }
    // 循环结束后打印 LIFTOFF!!!
    println!("LIFTOFF!!!");
}

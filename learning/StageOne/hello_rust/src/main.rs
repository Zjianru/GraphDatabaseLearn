// 【use 语句】用于引入其他模块的内容到当前作用域
// 避免每次都写完整路径，类似 Java 的 import 或 Python 的 from...import
// use 语句通常放在文件开头，在任何函数定义之前
// 如果不用 use，你需要写完整路径，如 rand::Rng::gen_range()

mod tup_test;

mod guess_number;
// 【fn】是 function 的缩写，用于声明函数
// main 函数是程序的入口点，每个可执行的 Rust 程序都必须有一个 main 函数
// () 表示这个函数不接收任何参数
// 没有 -> 表示这个函数不返回任何值（返回类型是 ()，称为 unit 类型）

fn main() {
    // 调用 tupTest 模块中的 run_tupTest 函数
    tup_test::run_tup_test();
    // 调用 guess_number 函数
    // guess_number::guess_number();
}

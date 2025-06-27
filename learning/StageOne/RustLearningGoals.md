# RustLearningGoals

本阶段目标：按照 Rust 官方文档《The Rust Programming Language》的章节顺序，系统学习 Rust 语言，为后续分布式图数据库开发打下坚实基础。

## 第一部分：基础入门

### 1. Getting Started (入门) - 对应官方文档第1章
- 安装 Rust 工具链（rustup、cargo）
- 编写第一个 Hello World 程序
- 了解 Cargo 的基本使用（new、build、run、check）
- 理解 Rust 项目的基本结构

### 2. Programming a Guessing Game (编程实战) - 对应官方文档第2章
- 通过猜数字游戏学习基本语法
- 使用外部 crate（rand）
- 理解 match 表达式
- 循环和条件判断
- 错误处理基础

### 3. Common Programming Concepts (常见编程概念) - 对应官方文档第3章
- 变量与可变性（let、mut、const）
- 数据类型（标量类型、复合类型）
- 函数定义与调用
- 注释的使用
- 控制流（if、loop、while、for）

## 第二部分：核心概念

### 4. Understanding Ownership (理解所有权) - 对应官方文档第4章
- 所有权规则
- 内存与分配
- 所有权转移（move）
- 克隆（clone）
- 引用与借用（&、&mut）
- 切片（slice）类型

### 5. Using Structs (使用结构体) - 对应官方文档第5章
- 定义和实例化结构体
- 使用元组结构体
- 方法语法（impl块）
- 关联函数
- 结构体的打印（Debug trait）

### 6. Enums and Pattern Matching (枚举和模式匹配) - 对应官方文档第6章
- 定义枚举
- match 控制流运算符
- if let 简洁控制流
- Option<T> 枚举的使用
- 模式匹配的强大功能

## 第三部分：项目管理

### 7. Managing Growing Projects (管理项目) - 对应官方文档第7章
- 包（package）和 crate
- 模块（module）系统
- 路径（path）引用
- use 关键字
- 将模块分割到不同文件

## 第四部分：常用功能

### 8. Common Collections (常见集合) - 对应官方文档第8章
- Vector（Vec<T>）的使用
- String 字符串的深入理解
- HashMap<K, V> 的使用
- 集合的遍历和更新

### 9. Error Handling (错误处理) - 对应官方文档第9章
- panic! 与不可恢复错误
- Result<T, E> 与可恢复错误
- ? 运算符
- 何时使用 panic! 或 Result
- 自定义错误类型

### 10. Generic Types, Traits, and Lifetimes (泛型、Trait和生命周期) - 对应官方文档第10章
- 泛型数据类型
- Trait：定义共享行为
- 生命周期与引用有效性
- 泛型、trait bounds 和生命周期的结合使用

## 第五部分：进阶特性

### 11. Writing Automated Tests (自动化测试) - 对应官方文档第11章
- 编写测试函数
- 控制测试运行
- 测试组织结构
- 单元测试和集成测试

### 12. Functional Language Features (函数式编程特性) - 对应官方文档第13章
- 闭包：可以捕获环境的匿名函数
- 迭代器：处理元素序列
- 改进 I/O 项目
- 性能比较：循环 vs 迭代器

### 13. More about Cargo (深入 Cargo) - 对应官方文档第14章
- 自定义构建配置
- 发布 crate 到 crates.io
- Cargo 工作空间
- 使用 cargo install 安装二进制文件
- 自定义命令扩展

## 第六部分：系统编程

### 14. Smart Pointers (智能指针) - 对应官方文档第15章
- Box<T> 指向堆上的数据
- Rc<T> 引用计数智能指针
- RefCell<T> 和内部可变性模式
- 循环引用与内存泄漏
- Weak<T> 弱引用

### 15. Fearless Concurrency (无畏并发) - 对应官方文档第16章
- 线程的创建与管理
- 消息传递并发（channel）
- 共享状态并发（Mutex<T>、Arc<T>）
- Sync 和 Send trait
- 并发编程的最佳实践

## 第七部分：高级主题

### 16. Async Programming (异步编程) - 补充内容
- async/await 语法
- Future trait
- tokio 运行时基础
- 异步 I/O 操作
- 并发任务管理

### 17. Advanced Features (高级特性) - 对应官方文档第19章
- 不安全 Rust（unsafe）
- 高级 trait
- 高级类型
- 高级函数和闭包
- 宏（macro）

## 第八部分：实战项目

### 18. Building a Mini Database (构建迷你数据库) - 实战项目
- 设计简单的 KV 存储
- 实现基本的持久化
- 添加网络接口
- 实现并发访问
- 性能优化

## 建议学习方法

1. **跟随官方文档**：使用 [The Rust Programming Language](https://doc.rust-lang.org/book/) 作为主要教材
2. **动手实践**：每章的示例代码都要亲手敲一遍
3. **完成练习**：使用 [Rustlings](https://github.com/rust-lang/rustlings) 巩固每章知识点
4. **阅读标准库文档**：熟悉常用类型和方法的文档
5. **参与社区**：遇到问题时查阅 Rust 用户论坛和 Stack Overflow

## 学习资源

- 官方文档：https://doc.rust-lang.org/book/
- 中文翻译：https://kaisery.github.io/trpl-zh-cn/
- Rust by Example：https://doc.rust-lang.org/rust-by-example/
- 标准库文档：https://doc.rust-lang.org/std/
- Rust 游乐场：https://play.rust-lang.org/

---

完成本阶段后，你将：
- 掌握 Rust 的核心概念（所有权、借用、生命周期）
- 能够编写安全、高效的 Rust 程序
- 理解 Rust 的并发模型
- 具备阅读和理解 Rust 代码的能力
- 为学习分布式图数据库开发打下坚实基础 
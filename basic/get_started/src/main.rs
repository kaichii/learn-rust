// 程序入口
fn main() {
    // 声明变量 a，类型不指定，自动推断类型为 i32
    // a 不可变
    let a = 10;

    // 声明变量 b，指定类型为 i32
    let b: i32 = 20;

    // 30i32 表示数值 30， 类型是 i32
    // c 可变的， mut 是 mutable 的缩写
    let mut c = 30i32;

    // 30_i32 比 30i32 可读性更好
    let d = 30_i32;

    // 将函数的返回值作为另一个函数的参数
    let e = add(add(a, b), add(c, d));

    // println!是宏调用，看起来像是函数但是返回的是宏定义的代码块（不是很懂，先当成是普通函数吧）
    // 和其他语言类似，输出格式化字符串到标准输出（控制台）
    // 其他语言使用 %s，%d 等占位符，Rust 使用 {}，由于 println! 会自动推断类型，无需手动指定
    println!("(a + b) + (c + d) = {}", e);
}

// 定义 add 函数，接收两个 i32 类型的参数，返回它们的和
fn add(i: i32, j: i32) -> i32 {
    // 返回表达式的值，省略 return (不可加分号; 加了分号应该会认为是语句而不是表达式)
    i + j
}

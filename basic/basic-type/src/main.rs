use num::complex::Complex;

fn main() {
    // 数值类型

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 运算
    // 加
    let sum = 5 + 10;

    // 减
    let difference = 95.5 - 4.3;

    // 乘
    let product = 4 * 30;

    // 除
    let quotient = 56.7 / 32.2;

    // 求余
    let remainder = 43 % 5;

    // 类型推导为 i32
    let twenty = 20;

    // 类型标注
    let twenty_one: i32 = 21;

    // 类型后缀标注
    let twenty_two = 22_i32;

    // 同类型运算
    let addition = twenty + twenty_one + twenty_two;

    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    // 浮点数陷阱
    // 由于存储浮点数位数限制，不能准确的表示浮点数。经典问题 0.1 + 0.2 == 0.3?
    // 避免测试浮点数的相等性

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2); // error

    // NaN: 数学上为定义的结果

    let x = (-42.0_f32).sqrt();

    if x.is_nan() {
        println!("x 不是一个数字")
    }

    // 序列

    // 1..5 => 1,2,3,4
    // 1..=5 => 1,2,3,4,5
    for i in 1..=5 {
        println!("{}", i)
    }

    // 复杂数值
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);

    let result = a + b;

    println!("{} + {}i", result.re, result.im);

    // 使用方法
    let x = 12.2_f32.round();

    println!("{}", x);

    // 字符类型

    // Rust 的字符不仅仅是 ASCII，包括所有的 Unicode 值
    let c = 'c';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    // Unicode 都是 4 个字节编码
    println!("字符'国'占用了{}字节的内存大小", std::mem::size_of_val(&g));

    // 布尔类型
    let t = true;
    // 标注类型
    let f: bool = false;

    // 单元类型 `()` 其唯一值就是 `()`
    // `fn main()` 的返回值就是 `()`，不能看作没有返回值，没有 返回值的函数是 发散函数(Rust 中单独定义)

    // 语句和表达式

    // 语句和表达式有所区别，语句执行操作没有返回值，而表达式有返回值
    // 例如 js 中
    // 可以看作是语句
    // let a = 1; => undefined
    // 可以看作表达式
    // a = 2; => 2

    // 语句
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);

    // 表达式
    5 + 6;
    let y = {
        let x = 3;
        // 表达式充当返回值，不能以分号结尾
        5 + 6 + x
    };

    println!("y: {}", y);

    // 函数
    // 函数命名为蛇形 add_two
    // 函数参数需标明类型
    // 函数位置无所谓
    fn add(i: i32, j: i32) -> i32 {
        // 表达式返回
        i + j
        // return
        // return i + j;
    }

    println!("1 + 2 = {}", add(1, 2));

    use std::fmt::Debug;

    // 无返回
    fn report<T: Debug>(item: T) {
        println!("{:?}", item); // 隐式返回 `()`
    }

    fn add(x: u32, y: u32) -> u32 {
        // 语句而不是表达式 同样返回 `()`
        x + y; //ERROR: expected `u32`, found `()`
    }

    // 永不返回

    fn dead_end() -> ! {
        panic!("永远不会返回")
    }

    dead_end();
}

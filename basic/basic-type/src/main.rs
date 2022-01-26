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
    println!("字符'国'占用了{}字节的内存大小",std::mem::size_of_val(&g));
}

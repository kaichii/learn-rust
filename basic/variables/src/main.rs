fn main() {
    // 变量可变性
    let mut x = 5;
    println!("x 的值是 {}", x);

    x = 6;
    println!("x 的值是 {}", x);

    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);
    // a 可变，b 不可变
    println!("a = {:?}，b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    // 常量 使用 const 关键字声明
    const MAX_POINTS: u32 = 100_100;

    // 变量遮蔽
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // -> 12
    }

    println!("The value of x is: {}", x); // -> 6

    // 变量遮蔽的用处在于可以重复使用变量名字
    // 字符串类型
    let spaces = "    ";
    // usize 数值类型
    let spaces = spaces.len();

    // 不用 let， 使用 mut 修改变量
    let mut spaces = "    ";
    spaces = spaces.len(); // 类型错误
}

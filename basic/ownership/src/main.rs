fn main() {
    // Rust 中每个值有且只有一个所有者
    // 所有者离开作用域范围时，这个值将被丢弃

    {
        // 不可变的字符串字面量
        let s: &str = "hello";
        println!("{}", s); // 在作用域内
    }

    // println!("{}", s); // s 不再可用

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = s;

    // println!("s = {}, s1 = {}", s, s1); // s 不再可用，其所有权转移到了 s1

    let x = 5;
    let y = x; // 实现了 `Copy` 特征的数据类型直接拷贝（深/浅 没有区别，因为是数据存放在栈上，不用分配内存），所以不会转移所有权，任何基本类型都是 `Copy` 的
    println!("x = {}, y = {}", x, y);

    // 函数参数与返回值

    let s = String::from("hello world");
    takes_ownership(s); // s 的值移动到函数里
                        // println!("s 移动进函数后继续使用 {}", s);
    let x = 5;
    let y = makes_copy(x);
    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // some_string 移出作用域

fn makes_copy(some_integer: i32) -> i32 {
    // some_integer 进入作用域
    println!("{}", some_integer);
    some_integer // 返回 some_integer 并移除给调用的函数
}

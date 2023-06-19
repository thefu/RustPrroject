/**切片是对数据值的部分引用
* 切片这个名字往往出现在生物课上，我们做样本玻片的时候要从生物体上获取切片，以供在显微镜上观察，在Rust中，切片的意思大致也是h泽阳，只不过它从数据取材引用。
*/
fn main() {
    let s = String::from("broadcast");
    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println("{} = {} + {}", s, part1, part2);
}

/**
 * Rust 中的字符串类型实质上记录了字符在内存中的起始位置和其长度，我们暂时了解到这一点
 * 使用 .. 表示范围的语法在循环章节中出现过。x..y 表示 [x, y) 的数学含义。.. 两边可以没有运算数：
 * 
 * ..y 等价于 0..y
 * x.. 等价于位置 x 到数据结束
 * .. 等价于位置 0 到结束
 */

//被切片引用的字符串禁止更改其值
fn main() {
    let mut s = String::from("runoob");
    let slice = &s[0..3];
    s.push_str("yes!");//错误
    println!("slice = {}", slice);
}
/**
 * 这段程序不正确。
 * s 被部分引用，禁止更改其值。
 */

/**
 * 实际上，到目前为止你一定疑惑为什么每一次使用字符串都要这样写String::from("runoob")，直接"runoob"不行吗？
 * 在Rust中有两种常用的字符串类型：str和String。str是Rust核心语言类型，就是本章一直在讲的字符串切片，常常以引用的形式出现（&str）.
 * 凡是使用双引号包括的字符串常量整体的类型性质都是&str
 * let s = "hello";
 * 这里的s就是一个&str类型的变量。
 * String类型是Rust标准公共库提供的一种数据类型，它的功能更完善——它支持字符串的追加、清空等实用的操作。String和str除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性
 * String和str都支持切片，切片的结果是&str类型的数据。
 * 注意：切片结果必须是引用类型，但开发者必须自己明示这一点。
 * let slice = &s[0..3];
 * 有一个快速的办法可以将String转换成&str：
 * let s1 =String::from("hello");
 * let s = &s1[..];
 */

//除了字符串以外，其他一些线性数据结构也支持切片操作，例如数组：
fn main() {
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}
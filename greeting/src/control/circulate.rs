fn main() {
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");
    //while循环是最典型的条件语句循环。Rust语言还没有do-while的用法，但是do被规定为保留字
    
    let mut i = 0;
    while i < 10 {
        //循环体
        i += 1;
    }
    //在C语言中for循环使用三元语句控制循环，但是在Rust中没有这种用法，需要用while循环来代替

    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为：{}", i);
    }
    //for循环是最常用的循环结构，经常用来遍历一个线性数据结构。a.iter()代表a的迭代器。当然，for循环也可以通过下标来访问数组：
    for i in 0..5 {
        println!("a[{}] = {}",i, a[i]);
    }

    //某个循环无法在开头或结尾判断是否继续进行循环，必须在循环体中间某处控制循环的进行。如果遇到这种情况，我们i经常会在一个while(true)循环体里实现中途退出循环的操作。
    let s = ["R", "S", "N", "N"];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'N' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }

    //loop循环可以通过break关键字类似于return一样使整个循环退出并给予外部一个返回值，这是一个十分巧妙的设计，因为loop这样的循环常被用来当做查找工具使用，如果找到了某个东西当然要将这个东西交出去；
    let mut j = 0;
    let location = loop {
        let ch = s[j];
        if ch == 'N' {
            break j
        }
        j += 1;
    };
    println!(" \'N\' 的索引为 {}", location);
}
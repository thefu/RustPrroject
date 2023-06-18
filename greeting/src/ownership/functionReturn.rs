fn main() {
    let s1 = gives_ownership();
    //gives_ownership 移动到它的返回值s1

    let s2 = String::from("hello");
    //s2被声明有效

    let s3 = takes_and_gives_back(s2);
    //s2被当作参数释放，s3获得返回值所有权
}//s3无效被释放，s2被移动，s1无效被释放

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    //some_string被声明有效，

    return some_string;
    //some_string被当作返回值而移出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    //a_string被声明有效
    a_string
}

/**
 * 被当作函数返回值的变量所有权将会被移动出函数并返回到调用函数的地方，而不会直接被无效释放
 */
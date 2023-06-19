fn ipv4_to_int(ipv4: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let mut octets = ipv4.split('.');

    for _ in 0..4 {
        if let Some(octet) = octets.next() {
            if let Ok(num) = octet.parse::<u32>() {
                if num > 255 {
                    return None; // 无效的IPv4地址
                }
                result = (result << 8) | num;
            } else {
                return None; // 无效的IPv4地址
            }
        } else {
            return None; // 无效的IPv4地址
        }
    }

    Some(result)
}

fn main() {
    let ipv4_address = "192.168.0.1";
    if let Some(ipv4_int) = ipv4_to_int(ipv4_address) {
        println!("IPv4: {}", ipv4_address);
        println!("Integer: {}", ipv4_int);
    } else {
        println!("Invalid IPv4 address");
    }
}

/**
 * 在上面的代码中，ipv4_to_int函数使用.作为分隔符将IP地址字符串拆分为四个部分（即四个八位的十进制数字）。然后，它逐个解析每个部分，并将其转换为u32类型。
 * 每个部分都按顺序左移8位，并与前面的部分进行按位或操作，以构建最终的32位整数表示。如果IP地址字符串无效，解析过程中遇到错误或超出允许的范围（0到255），则函数返回None。
 * 否则，它将返回成功转换的整数。
 * 请注意，这只是一个简单的示例实现，并未进行完整的错误处理或范围检查。在实际应用中，你可能需要添加更多的错误处理和边界条件检查来确保准确性和安全性。
 */
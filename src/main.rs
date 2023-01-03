use std::io;
use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use std::cmp::Ordering;
use std::net::TcpStream;
use std::time::Instant;

fn main() {
    println!("输入目标IP地址/域名");
    println!("示例: hypixel.net ");
    let mut address = String::new();
    io::stdin().read_line(&mut address).expect("出现错误");

    println!("请输入目标端口");
    let mut port = String::new();
    io::stdin().read_line(&mut address).expect("出现错误");

    println!("解析中...请稍后");
    let start = Instant::now();
    let stream = match TcpStream::connect(format!("baidu.com:80")) {
        Ok(stream) => {
            sleep(Duration::from_secs(1));
            println!("解析成功!");
            stream;
        }
        Err(e) => {
            println!("在连接时出现错误: {}", e);
            println!("程序将在5秒后自动退出");
            sleep(Duration::from_secs(5));
            std::process::exit(0);
        }
    };


    let proxies = rand::thread_rng().gen_range(860..=2023);

    println!("正在将地址信息同步至全球 {} 个节点", proxies);

    let finish:i32 = rand::thread_rng().gen_range(80..=100);

    println!("攻击开始，预计使用时间：{} 秒", finish);
    sleep(Duration::from_secs(1));

    loop {
        let mut i = 0;
        let cps = rand::thread_rng().gen_range(500..=3000);

        println!("正在对目标进行攻击(CPS: {})", cps);

        i + 1;
        match i.cmp (&finish) {
            Ordering::Less => {
                sleep(Duration::from_secs(1));
                continue;
            }
            Ordering::Greater => {
                break;
            }
            Ordering::Equal => {
                sleep(Duration::from_secs(1));
                continue;
            }
        }
    }

    println!("本次攻击结束，程序将在5秒后自动关闭");
    sleep(Duration::from_secs(5));
    std::process::exit(0);
}
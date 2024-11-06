use std::io::{self, Write};
mod sort;
mod ip;
use sort::sort_array_file::*;
use ip::move_file::*;
use ip::unique_ip::*;
use ip::ip_data::*;
use rusqlite::Connection;

use tokio;
#[tokio::main]
async fn main() {
    let conn = Connection::open("E:\\Projects\\RustProjects\\Example\\Practice01\\src\\resources\\ip_data.db").expect("无法打开数据库");
    //创建表
    create_table(&conn).expect("无法创建表");
    loop {
        println!("请选择功能:");
        println!("1: (练习)排序数组");
        println!("2: 移动文件");
        println!("3: 处理唯一IP");
        println!("4: 检查文本文件");
        println!("5: 填充数据");
        println!("6: 分类区域");
        println!("7: 格式化文件");
        println!("0: 退出");

        let mut input = String::new();
        print!("请输入你的选择: ");
        io::stdout().flush().unwrap(); // 刷新标准输出以确保提示信息被显示

        io::stdin().read_line(&mut input).expect("读取输入失败");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("无效输入，请输入一个数字。");
                continue; // 如果输入无效，继续循环
            }
        };

        match choice {
            1 => {
                let mut numbers = vec![1, 5, 8, 3, 7];
                quick_sort(&mut numbers); // 调用排序函数
                println!("排序后的数组: {:?}", numbers); // 打印排序后的数组
            }
            2 => {
                println!("请将文件放置在D:/Download/IP/type01_IP中");
                move_file().await.unwrap();
            }
            3 => {
                 // 处理文件夹下所有文件，输出到output文件夹，得到唯一IP
                if let Err(e) = unique_ip("D:\\Download\\IP\\type02_IP").await {
                    println!("处理唯一IP时出错: {}", e);
                }
            }
            4 => {
                let _ = check_txt_file("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt").await;
            }
            5 => {
                let _ = fill_data("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt").await;
            }
            6 => {
                // 调用分类区域的函数
                let _ = classify_by_region("D:\\Download\\IP\\type01_IP\\output\\unique_ip.txt").await.unwrap();
            }
            7 => {
                // 写入数据库
                let _ = read_file_and_write_to_db("D:\\Download\\IP\\test.txt", &conn, 1, 2, 5).await.unwrap();
            }
            0 => {
                println!("退出程序。");
                break; 
            }
            _ => {
                println!("无效选择，请输入有效的数字。");
            }
        }
    }
}
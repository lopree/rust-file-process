use rusqlite::Connection;
use std::io::{self,Write};
mod file_process; 
use file_process::ip_data::*;
use file_process::process::unique_ip;

#[tokio::main] // 使用 tokio 作为异步运行时
async fn main() {
    let file01_path = "./assets/ips/type01"; 
    let file02_path = "./assets/ips/type02"; 
    let conn = Connection::open("./assets/database.db").expect("Failed to open database");
    create_table(&conn).expect("Failed to create table");
    loop {
        println!("请选择功能:");
        println!("1: 清空数据库");
        println!("2: 读取文件类型（区域在第3行）");
        println!("3: 读取文件类型（区域在第4行）");
        println!("0: 退出程序");


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
                println!("清空数据库");
                clear_database(&conn).expect("Failed to clear database");
            }
            2 => {
                match unique_ip(file02_path, &conn,3).await {
                    Ok(_) => println!("Unique IP processed successfully."),
                    Err(e) => println!("Error processing unique IP: {}", e),
                }
            }
            3 => {
                match unique_ip(file01_path, &conn,4).await {
                    Ok(_) => println!("Unique IP processed successfully."),
                    Err(e) => println!("Error processing unique IP: {}", e),
                }
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
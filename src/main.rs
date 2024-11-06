use std::io::{self, Write};
mod sort;
mod ip;
use sort::sort_array_file::*;
use ip::move_file::*;
use ip::unique_ip::*;
use std::path::Path;
fn main() {
    //move_file().unwrap();
    //let _ =unique_ip("D:\\Download\\IP\\type02_IP");
    //check_txt_file(Path::new("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt")).unwrap();
    //fill_data(Path::new("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt")).unwrap();
    //fill_data_nard(Path::new("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt")).unwrap();
    //let _ = unique_ip("D:\\Download\\IP\\type01_IP");
    //classify_by_region(Path::new("D:\\Download\\IP\\type01_IP\\output\\unique_ip.txt")).unwrap();
    //format_space_to_comma(Path::new("D:\\Download\\IP\\type01_IP\\output\\sort\\output_HKG.txt")).unwrap();
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
                move_file().unwrap();
            }
            3 => {
                //处理文件夹下所有文件，输出到output文件夹，得到唯一IP
                let _ = unique_ip("D:\\Download\\IP\\type02_IP");
            }
            4 => {
                // 调用检查文本文件的函数
                let _ = check_txt_file("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt");
            }
            5 => {
                // 调用填充数据的函数
                // fill_data(Path::new("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt")).unwrap();
                println!("填充数据的功能尚未实现。");
            }
            6 => {
                // 调用分类区域的函数
                // classify_by_region(Path::new("D:\\Download\\IP\\type01_IP\\output\\unique_ip.txt")).unwrap();
                println!("分类区域的功能尚未实现。");
            }
            7 => {
                // 调用格式化文件的函数
                // format_space_to_comma(Path::new("D:\\Download\\IP\\type01_IP\\output\\sort\\output_HKG.txt")).unwrap();
                println!("格式化文件的功能尚未实现。");
            }
            0 => {
                println!("退出程序。");
                break; // 退出循环
            }
            _ => {
                println!("无效选择，请输入有效的数字。");
            }
        }
    }
}
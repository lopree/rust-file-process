use regex::Regex;
use base64::{Engine as _, engine::general_purpose};
use rusqlite::Connection;
use std::net::IpAddr;
use std::str::FromStr;
use std::io::{BufReader,Error,BufRead};
use std::path::{Path,PathBuf};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use crate::file_process::ip_data::{ConnectionData, insert_connection,change_connected};
///读取文件中的uuid，以及假地址
pub fn const_value()->Result<Vec<String>,Error>{
    let mut all_content = Vec::new();
    let file = File::open("./assets/ips/value.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for line in lines{
        match line {
            Ok(content) => all_content.push(content),
            Err(e) => println!("读取时出错：{}",  e),
        }
    }
    Ok(all_content)
}
pub async fn unique_ip(file_path : &str,conn : &Connection,region_index : usize) ->Result<(),Error> {
    let mut all_lines = Vec::new();
    let mut ip_list = HashSet::new();
    let files = get_files(file_path)?;  // 先获取文件列表
    for file in files {  // 直接遍历 PathBuf
        let reader = BufReader::new(File::open(&file).map_err(|e|{
            println!("无法打开文件 {:?} : {}", file, e);
            e
        })?);
        //读取文件内容
        let lines = reader.lines();
        for line in lines {
            match line {
                Ok(content) => all_lines.push(content),
                Err(e) => println!("读取文件{}时出错：{}", file.display(), e),
            }
        }
    }

    println!("所有地址数量：{}",all_lines.len());
    //从第二行开始提取
    for line in all_lines.iter().skip(1) {
        // 使用正则表达式分割行内容，匹配逗号或空格
        let re = Regex::new(r"[,\s]+").unwrap(); // 匹配一个或多个逗号或空格
        let parts : Vec<&str> = re.split(&line).collect();
        if parts.len() < 2 {
            println!("行格式不正确: {:?}", line);
            continue; // 跳过格式不正确的行
        }
        //写入数据库
        // 第一列是ip地址
        let ip = parts[0];
        // 提取端口
        let port = parts[1].parse().unwrap_or(0);
        // 创建一个唯一的标识符
        let unique_key = format!("{}:{}", ip, port);

        // 如果ip地址已经存在，检查端口是否相同
        if ip_list.contains(&unique_key) {
            continue; 
        } else {
            // 验证IP地址是否有效
            match IpAddr::from_str(ip) {
                Ok(_valid_ip) => {
                    ip_list.insert(unique_key.clone());
                    let region = parts.get(region_index).map(|&s| s.to_string());

                    let data = ConnectionData {
                        ip: ip.to_string(),
                        port,
                        region,
                        can_connected: None,
                        is_high_speed: None,
                    };

                    match insert_connection(conn, &data) {
                        Ok(_) => (),
                        Err(e) => println!("记录重复或插入失败: {}", e),
                    }
                },
                Err(_) => {
                    println!("无效的IP地址: {}", ip);
                    continue;
                }
            }
        }
    }

    println!("去重后地址数量: {}", ip_list.len());
    Ok(())
}
///获得文件夹下的所有文件路径
fn get_files(file_path :&str) ->Result<Vec<PathBuf>,Error> {
    let mut files = Vec::<PathBuf>::new();
    let dir = Path::new(file_path);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }else if path.is_dir() {
                // 这里需要将 path 转换为字符串
                if let Some(path_str) = path.to_str() {
                    files.extend(get_files(path_str)?);
                }
            }
        }
    }
    Ok(files)
}

///读取文件，修改数据库中的是否链接数据
pub async fn change_can_connected(file_path : &str,conn : &Connection)->Result<(), Box<dyn std::error::Error>>{
    if let Ok(file) = File::open(file_path) {
        for line in BufReader::new(file).lines() {
            if let Ok(link) = line {
                if link.starts_with("vless://") {
                    // 移除 "vless://" 前缀
                    let without_prefix = link.trim_start_matches("vless://");
                    
                    // 分割基本链接和参数
                    let parts: Vec<&str> = without_prefix.split('?').collect();
                    if parts.is_empty() {
                        continue;
                    }

                    let base_part = parts[0];
                    // 尝试判断是否是 Base64 编码
                    let connection_info = if base_part.contains('@') {
                        // 直接格式：uuid@ip:port
                        base_part.to_string()
                    } else {
                        // Base64 编码格式
                        match general_purpose::STANDARD.decode(base_part) {
                            Ok(decoded) => String::from_utf8(decoded).unwrap_or_default(),
                            Err(_) => continue,
                        }
                    };
                    // 解析基本连接信息
                    let re = Regex::new(r"(?:(.+?):)?(.+?)@([\d.]+):(\d+)").unwrap();
                    if let Some(caps) = re.captures(&connection_info) {
                        let ip = caps.get(3).map_or("", |m| m.as_str());
                        let port = caps.get(4).map_or("", |m| m.as_str());
                        change_connected(conn, ip, port.parse().unwrap_or(0), true)?;
                    }
                }
            }
        }
    }
    Ok(())
}
///从数据库中获取链接
pub async fn get_links_from_data()->(){

}

///按照指定格式输出链接
pub fn target_links (ip : &str , port : u16,region_code : &str,uuid: &str,fake_addr : &str) -> String{
    // 构造反代地址
    let reverse_proxy_address = format!("ProxyIP.{}.fxxk.dedyn.io", region_code);

    // 构造 VLESS 链接
    format!(
        "vless://{}@{}:{}?encryption=none&security=tls&sni={}&fp=random&type=ws&host={}&path=%2Fpyip%3D%5B{}%5D%3A443#{}",
        uuid, ip, port, fake_addr, fake_addr, reverse_proxy_address, region_code
    )
}
use std::{collections::HashSet, error::Error, fs::{File, OpenOptions}, io::{BufRead,BufReader, Write,BufWriter}, path::{PathBuf, Path}};

use rand::seq::SliceRandom; // 导入 SliceRandom trait
use rand::thread_rng; // 导入 thread_rng
use regex::Regex;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC}; // 导入 URL 编码功能
use std::collections::HashMap;
use crate::ip::region_code::get_region_name;
use crate::ip::move_file::get_csv_files;
///telegram 获得的文件,去除重复ip
pub fn unique_ip(file_path: &str) -> Result<(), Box<dyn Error>> {
    // 读取文件夹下的所有csv文件,忽略子文件夹
    let files = get_csv_files(file_path.as_ref(),true);
    //遍历所有文件，将内容聚合到一起
    let mut ip_list = HashSet::new();
    let mut all_lines = Vec::new();
    let mut unique_line = Vec::new();

    for entry in files {
        let file = entry.as_path();
        let reader = BufReader::new(File::open(&file).map_err(|e| {
            println!("无法打开文件 {:?}: {}", file, e);
            e
        })?);
        //读取文件内容
        let lines = reader.lines();
        // 逐行提取
        for line in lines {
            match line {
                Ok(content) => all_lines.push(content),
                Err(e) => println!("读取文件 {:?} 时出错: {}", file, e),
            }
        }
    }
    println!("所有地址数量: {}", all_lines.len());
    // 根据all_lines中的ip地址，提取ip，对重复ip的lines，只保留一个
    for line in all_lines.iter().skip(1) { // 从第二行开始提取
        // 使用正则表达式分割行内容，匹配逗号或空格
        let re = Regex::new(r"[,\s]+").unwrap(); // 匹配一个或多个逗号或空格
        let parts: Vec<&str> = re.split(line).collect(); // 分割行内容
    
        if parts.len() < 2 {
            println!("行格式不正确: {:?}", line);
            continue; // 跳过格式不正确的行
        }
    
        // 第一列是ip地址
        let ip = parts[0]; // 第一列
        // 提取端口
        let port = parts[1]; // 假设端口是第二列
        // 创建一个唯一的标识符
        let unique_key = format!("{}:{}", ip, port);
    
        // 如果ip地址已经存在，检查端口是否相同
        if ip_list.contains(&unique_key) {
            continue; // 如果相同，跳过该行
        } else {
            ip_list.insert(unique_key.clone()); // 插入唯一标识符
            unique_line.push(line); // 添加行
        }
    }
    println!("去重后地址数量: {}", unique_line.len());
    //将unique_line写入文件
    let output_file_path = format!("{}/output/unique_ip.txt", file_path); 
    let mut file = File::create(&output_file_path)?;
    // 使用 UTF-8 编码的字节字符串
    file.write_all("IP地址,端口,回源端口,TLS,数据中心,地区,国家,城市,TCP延迟(ms),速度(MB/s)\n".as_bytes())?;

    for line in unique_line {
        file.write_all(format!("{}\n", line).as_bytes())?; 
    }
    
    
    Ok(())
}
///保留第一行的标题栏，删除多余的标题
pub fn check_txt_file(file_path: &str) -> Result<(), Box<dyn Error>> { 
    let file = File::open(file_path)?; // 打开指定路径的文件
    let reader = BufReader::new(file);
    
    let mut valid_lines = Vec::new();
    let mut line_count = 0;
    let mut ip_count = 0; // 计数器，用于统计以 "IP" 开头的行

    // 定义正则表达式以匹配以汉字结尾的行
    let re = Regex::new(r"[\u4e00-\u9fa5]$").unwrap(); // 匹配汉字

    // 逐行读取文件
    for line in reader.lines().skip(1) {
        match line {
            Ok(content) => {
                line_count += 1;
                // 检查行的前两个字符是否为 "IP"，并去除首尾空白
                if content.trim().starts_with("IP") || re.is_match(&content) {
                    ip_count += 1; // 增加计数
                    println!("符合条件的行: {:?}", content); // 打印符合条件的行
                } else {
                    valid_lines.push(content); // 保存有效行
                }
            }
            Err(e) => println!("读取行时出错: {}", e),
        }
    }

    println!("以 'IP' 开头或以汉字结尾的行数量: {}", ip_count); // 打印符合条件的行的数量
    println!("文件 {:?} 有 {} 行", file_path, line_count);

    // 将有效行写回文件
    let output_file = File::create(file_path)?; // 使用传入的 file_path
    let mut writer = BufWriter::new(output_file);
    
    // 写入有效行
    for line in valid_lines {
        writeln!(writer, "{}", line)?; 
    }
    
    writer.flush()?; // 确保所有数据都被写入
    Ok(())
}

// 按照指定格式填入数据
//标准：
//将第一列的地址和第二列的端口填到上面的@后面，第四列填到#号之后
///CF格式批量改写
pub fn fill_data(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?; // 打开指定路径的文件
    let reader = BufReader::new(file);

    let uuid = "43ee9711-f9e6-47ba-aba8-ea179eb6ada3";
    let output_file_path = Path::new("D:\\Download\\IP\\type02_IP\\output\\vless\\formatted_output.txt");

    
    let mut formatted_lines = Vec::new();
    let mut line_count = 0;
    // 定义国家代码数组
    let countries = ["KR", "JP", "SG", "HK"];

    // 逐行读取文件
    for line in reader.lines() {
        match line {
            Ok(content) => {
                line_count += 1;
                // 分割行内容
                let parts: Vec<&str> = content.split(',').collect();
                if parts.len() >= 4 {
                    // 提取 IP 地址、端口和第四列
                    let ip_address = parts[0]; // 第一列
                    let port = parts[1]; // 第二列
                     // 随机选择国家代码
                     let mut rng = thread_rng();
                     let country = countries.choose(&mut rng).unwrap(); // 随机选择一个国家代码
                    let fourth_column = parts[3]; // 第四列
                     // 将地区缩写转换为中文
                     let region_name = get_region_name(fourth_column);

                     // 将中文进行 URL 编码
                     let encoded_region_name = utf8_percent_encode(&region_name.unwrap(), NON_ALPHANUMERIC).to_string();
                    

                    // 格式化为指定字符串
                    let formatted_line = format!(
                        "vless://{}@{}:{}?encryption=none&security=tls&sni=img.rookstein.filegear-sg.me&fp=random&type=ws&host=img.rookstein.filegear-sg.me&path=%2Fpyip%3D%5BProxyIP.{}.fxxk.dedyn.io%5D%3A443#{}",
                        uuid, ip_address, port, country, encoded_region_name
                    );

                    formatted_lines.push(formatted_line); // 保存格式化后的行
                }
            }
            Err(e) => println!("读取行时出错: {}", e),
        }
    }

    println!("文件 {:?} 有 {} 行", file_path, line_count);

    // 将格式化后的行写回文件或输出
    let output_file = File::create(output_file_path)?; // 使用传入的 file_path
    let mut writer = BufWriter::new(output_file);
    
    // 写入格式化后的行
    for line in formatted_lines {
        writeln!(writer, "{}", line)?; 
    }
    
    writer.flush()?; // 确保所有数据都被写入
    Ok(())


}


pub fn fill_data_nard(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?; // 打开指定路径的文件
    let reader = BufReader::new(file);

    let uuid = "030d38b6-2f58-4704-e6b3-d4db837aaa5c";
    let output_file_path = Path::new("D:\\Download\\IP\\type02_IP\\output\\vless\\nard_formatted_output.txt");

    
    let mut formatted_lines = Vec::new();
    let mut line_count = 0;
    // 定义国家代码数组
    let countries = ["KR", "JP", "SG", "HK"];
    // 定义地区缩写到中文的映射
    let region_map = [
        ("SJC", "美国圣何塞"),
        ("ARN", "瑞典阿尔斯塔"),
        ("IAD", "美国华盛顿"),
        ("SEA", "美国西雅图"),
        ("NRT", "日本成田"),
        ("LHR", "英国伦敦"),
        ("BOM", "印度孟买"),
        ("ORD", "美国芝加哥"),
        ("YUL", "加拿大蒙特利尔"),
        ("YYZ", "加拿大多伦多"),
        ("ZRH", "瑞士苏黎世"),
        ("SIN", "新加坡"),
        ("SYD", "澳大利亚悉尼"),
        ("HKG", "中国香港"),
        ("TYO", "日本东京"),
        ("ICN", "韩国仁川"),
        ("KUL", "马来西亚吉隆坡"),
        ("MXP", "意大利米兰"),
        ("FRA", "德国法兰克福"),
        ("AMS", "荷兰阿姆斯特丹"),
        ("LAX", "美国洛杉矶"),
        ("IAD", "美国华盛顿"),
        ("GRU", "巴西圣保罗"),
        ("CDG", "法国巴黎"),
    ].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    // 逐行读取文件
    for line in reader.lines() {
        match line {
            Ok(content) => {
                line_count += 1;
                // 分割行内容
                let parts: Vec<&str> = content.split(',').collect();
                if parts.len() >= 4 {
                    // 提取 IP 地址、端口和第四列
                    let ip_address = parts[0]; // 第一列
                    let port = parts[1]; // 第二列
                     // 随机选择国家代码
                     let mut rng = thread_rng();
                     let country = countries.choose(&mut rng).unwrap(); // 随机选择一个国家代码
                    let fourth_column = parts[3]; // 第四列
                     // 将地区缩写转换为中文
                     let region_name = region_map.get(fourth_column).unwrap_or(&"未知").to_string();

                     // 将中文进行 URL 编码
                     let encoded_region_name = utf8_percent_encode(&region_name, NON_ALPHANUMERIC).to_string();
                    

                     // 格式化为指定字符串
                     let formatted_line = format!(
                        "vless://{}@{}:{}?encryption=none&security=tls&sni=notls.rookstein.top&alpn=http%2F1.1&allowInsecure=1&type=ws&host=notls.rookstein.top&path=%2F#{}",
                        uuid, ip_address, port, encoded_region_name // 更新为新的格式
                    );

                    formatted_lines.push(formatted_line); // 保存格式化后的行
                }
            }
            Err(e) => println!("读取行时出错: {}", e),
        }
    }

    println!("文件 {:?} 有 {} 行", file_path, line_count);

    // 将格式化后的行写回文件或输出
    let output_file = File::create(output_file_path)?; // 使用传入的 file_path
    let mut writer = BufWriter::new(output_file);
    
    // 写入格式化后的行
    for line in formatted_lines {
        writeln!(writer, "{}", line)?; 
    }
    
    writer.flush()?; // 确保所有数据都被写入
    Ok(())


}

///根据区域代码分类
pub fn classify_by_region(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?; // 打开指定路径的文件
    let reader = BufReader::new(file);
    //读取文件内容
    let lines = reader.lines();
    // 创建正则表达式以匹配逗号或空格
    let re = Regex::new(r"[,\s]+").unwrap(); // 匹配一个或多个逗号或空格
    // 创建一个 HashMap 来存储区域代码及其对应的行
    let mut region_map: HashMap<String, Vec<String>> = HashMap::new();
    // 遍历所有行，获得第五列的值
    for line in lines {
        match line {
            Ok(content) => {
                let parts: Vec<&str> = re.split(&content).collect(); // 使用正则表达式分割行内容
                if parts.len() >= 5 {
                    // 第五列是区域代码，判断是否为英文大写
                    if parts[4].chars().all(|c| c.is_ascii_uppercase()) {
                        let region_code = parts[4].to_string(); // 提取第五列并转换为 String
                        // 将行添加到对应区域代码的 Vec 中
                        region_map.entry(region_code).or_insert_with(Vec::new).push(content);
                    }
                } else {
                    println!("行格式不正确: {:?}", content); // 打印格式不正确的行
                }
            }
            Err(e) => println!("读取行时出错: {}", e), // 处理读取错误
        }
    }

    // 将每个区域的行写入不同的文件
    for (region_code, lines) in region_map {
        let output_file_path = format!("{}/sort/output_{}.txt", file_path.parent().unwrap().display(),region_code); // 根据区域代码生成输出文件名
        let output_file = File::create(&output_file_path)?;
        let mut writer = BufWriter::new(output_file);

        for line in lines {
            writeln!(writer, "{}", line)?; // 写入行
        }

        println!("区域代码: {} 的地址已写入文件: {}", region_code, output_file_path);
    }

    Ok(())
}

///格式化一个文件将空格或多个空格转换为逗号
pub fn format_space_to_comma(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?; // 打开指定路径的文件
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut formatted_lines = Vec::new(); // 用于存储格式化后的行

    // 逐行读取文件
    for line in lines {
        match line {
            Ok(content) => {
                // 使用正则表达式将空格或制表符替换为逗号
                let re = Regex::new(r"\s+").unwrap(); // 匹配一个或多个空白字符
                let formatted_line = re.replace_all(&content, ",").to_string();
                formatted_lines.push(formatted_line); // 保存格式化后的行
            }
            Err(e) => println!("读取行时出错: {}", e), // 处理读取错误
        }
    }

    // 将格式化后的行写回文件
    let output_file = OpenOptions::new()
        .write(true)
        .truncate(true) // 清空文件内容
        .open(file_path)?; // 使用传入的 file_path
    let mut writer = BufWriter::new(output_file);

    // 写入格式化后的行
    for line in formatted_lines {
        writeln!(writer, "{}", line)?; 
    }

    writer.flush()?; // 确保所有数据都被写入
    Ok(())
}

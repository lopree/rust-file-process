use std::{path::Path, error::Error, fs,fs::File, io::BufReader, path::PathBuf};
use glob::glob;
use csv::ReaderBuilder;
use chrono::Local;
///区分文件，移动到指定文件夹
pub async fn move_file() -> Result<(), Box<dyn Error>> {
    let dir_path = Path::new("D:\\Download\\IP\\type01_IP"); // 替换为实际文件夹路径
    let target_dir = Path::new("D:\\Download\\IP\\type02_IP"); // 替换为目标文件夹路径
    let ignore_subfolders = false; // true: 只获取当前文件夹的 CSV，false: 包括所有子文件夹

    // 确保目标文件夹存在
    fs::create_dir_all(target_dir)?;

    let csv_files = get_csv_files(dir_path, ignore_subfolders);
    let mut index = 1;

    for file in csv_files {
        if target_csv_file(&file)? {
            move_and_rename_file(&file, target_dir, index)?;
            index += 1;
        }
    }

    Ok(())
}
///文件分类，将第二列是tls的文件移动到指定文件夹
fn move_and_rename_file(file_path: &Path, target_dir: &Path, mut index: usize) -> Result<(), Box<dyn Error>> {
    // 获取当前日期
    let date = Local::now().format("%Y%m%d").to_string();

    loop {
        let new_file_name = format!("{}_{}.csv", date, index);
        let target_path = target_dir.join(&new_file_name);

        // 检查目标路径是否已存在文件
        if !target_path.exists() {
            fs::rename(file_path, &target_path)?;
            println!("文件 {:?} 已移动并重命名为 {:?}", file_path, target_path);
            break;
        }

        // 如果文件已存在，index + 1 尝试下一个名称
        index += 1;
    }

    Ok(())
}

pub fn get_csv_files(dir_path: &Path, ignore_subfolders: bool) -> Vec<PathBuf> {
    let mut csv_files = Vec::new();

    if ignore_subfolders {
        // 忽略子文件夹中的文件
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("csv") {
                    csv_files.push(path);
                }
            }
        }
    } else {
        // 递归查找所有子文件夹中的 CSV 文件
        let pattern = format!("{}/**/*.csv", dir_path.display());
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            if let Ok(path) = entry {
                csv_files.push(path);
            }
        }
    }

    csv_files
}

/// 检查 CSV 文件的第一行第三列是否为 "TLS"。区分是CF还是优选出的csv
fn target_csv_file(file_path: &Path) -> Result<bool, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true) // 允许缺少列
        .from_reader(reader);

    // 读取第二行记录
    let mut records = csv_reader.records();
    // 跳过第一行
    records.next(); // 忽略第一行

    match records.next() {
        Some(Ok(record)) => {
            // 检查第二行第三列是否为数字
            if let Some(third_column) = record.get(2) {
                // 尝试将第三列转换为数字
                if third_column.parse::<f64>().is_err() {
                    return Ok(true); // 如果转换失败，返回true
                }
            }
            Ok(false) // 如果是数字，返回false
        }
        Some(Err(_)) => {
            // 如果读取或解码出错，打印警告信息并返回 false
            println!("警告：文件 {:?} 不是有效的 UTF-8 编码，跳过该文件。", file_path);
            Ok(false)
        }
        None => Ok(false), // 如果文件为空或没有记录，返回 false
    }
}
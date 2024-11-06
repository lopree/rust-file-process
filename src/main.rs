mod sort;
mod ip;
use sort::sort_array_file::*;
use ip::unique_ip::*;
use std::path::Path;
fn main() {
    let mut numbers = vec![1,5,8,3,7];
    quick_sort(&mut numbers); // 调用冒泡排序
    println!("排序后的数组: {:?}", numbers); // 打印排序后的数组
    //move_file().unwrap();
    //let _ =unique_ip("D:\\Download\\IP\\type02_IP");
    //check_txt_file(Path::new("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt")).unwrap();
    //fill_data(Path::new("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt")).unwrap();
    //fill_data_nard(Path::new("D:\\Download\\IP\\type02_IP\\output\\unique_ip.txt")).unwrap();
    //let _ = unique_ip("D:\\Download\\IP\\type01_IP");
    //classify_by_region(Path::new("D:\\Download\\IP\\type01_IP\\output\\unique_ip.txt")).unwrap();
    //format_space_to_comma(Path::new("D:\\Download\\IP\\type01_IP\\output\\sort\\output_HKG.txt")).unwrap();
}
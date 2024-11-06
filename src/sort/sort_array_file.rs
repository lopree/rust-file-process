//冒泡排序
pub fn bubble_sort_function02(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        for j in 0..array.len()-1-i {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
            }
        }
    }
}

// 选择排序
pub fn select_sort(array: &mut Vec<i32>) {
    let len = array.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        array.swap(i, min_index); // 交换当前元素和找到的最小元素
    }
}

// 快速排序
pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 { // 如果数组长度小于等于1，直接返回
        return;
    }

    let pivot_index = partition(arr); // 对数组进行分区，确定枢轴

    let (left, right) = arr.split_at_mut(pivot_index); // 将数组分成两部分
    quick_sort(left); // 对左半部分进行快速排序
    quick_sort(&mut right[1..]); // 对右半部分进行快速排序
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1]; // 选择最后一个元素作为枢轴
    let mut i = 0;        //记录比枢轴的值小的值的个数

    for j in 0..arr.len() - 1 { // 遍历数组中除了最后一个元素以外的所有元素
        if arr[j] < pivot { // 如果当前元素小于枢轴，将它与arr[i]交换位置，并将i加1
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1); // 将枢轴的值与arr[i]交换位置,确保右边的都比枢轴的值大
    i // 返回枢轴的下标
}

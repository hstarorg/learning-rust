// 常见集合

fn main() {
    // Vector
    let mut arr = vec![1, 2, 3];
    println!(
        "arr.length = {}, arr = {:?}, second value = {}",
        arr.len(),
        arr,
        &arr[1]
    );
    arr.push(4);
    let v = &arr[3];
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中
    // arr.push(4); // 如果将 push 操作放在持有值得引用后，将会报错。
    println!("v={}", v);
    let last_value = arr.pop();
    println!("last value is {:?}, arr = {:?}", last_value, arr);

    // 遍历
    for v in &arr {
        println!("{}-done", v);
    }

    // 遍历操作值
    for v in &mut arr {
        // 取出值来 * 5
        *v *= 5;
        println!("{}-done", v);
    }
    println!("{:?}", &arr);

    // HashMap
    use std::collections::HashMap;
    let mut map = HashMap::new();
    // 新增
    map.insert("key1".to_string(), "val1".to_string());
    map.insert("key2".to_string(), "val2".to_string());

    // 更新
    map.insert("key1".to_string(), "val1_update".to_string());
    // key2 存在，则不更新值
    map.entry("key2".to_string())
        .or_insert("val2_update".to_string());
    // key3 不存在，则写入值
    map.entry("key3".to_string()).or_insert("val3".to_string());
    println!("map={:?}", map);
    // 删除
    map.remove(&"key1".to_string());
    println!("after remove key1, map={:?}", map);

    // 统计一句话中字符出现的次数
    let sentence = String::from("Hello rust, Hello world");
    let mut c_counter_map = HashMap::new();
    for c in sentence.replace(" ", "").replace(",", "").chars() {
        // 不存在就写入0，然后返回 count
        let count = c_counter_map.entry(c).or_insert(0);
        *count += 1;
    }
    println!("sentence char map = {:#?}", c_counter_map);
}

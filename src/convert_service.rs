use crate::data::Map;

pub fn bin_convert(map: Map) {
    for row in map.data {
        let base: usize = 2;
        let mut number: usize = 0;
        for (index, num) in row.iter().enumerate() {
            if *num as i32 == 1 {
                number = number + base.pow(row.len() as u32 - 1 - index as u32);
            }
        }
        println!("{}", number);
    }
}
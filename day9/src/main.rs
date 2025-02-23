use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt").trim();

    let mut free_space: HashMap<usize, Vec<i128>> = HashMap::new();
    let mut files: HashMap<usize, Vec<i128>> = HashMap::new();

    let mut file_id = 0;
    let mut input = input
        .chars()
        .enumerate()
        .map(|x| {
            let num = x.1.to_digit(10).unwrap();
            if x.0 % 2 == 0 {
                println!("{:?}", num);
                let out = vec![file_id; num as usize];
                file_id = file_id + 1;
                out
            } else {
                if let Some(k) = free_space.get_mut(&(num as usize)) {
                    k.push(x.0 as i128);
                } else {
                    free_space.insert(num as usize, vec![x.0 as i128]);
                }
                vec![-1; num as usize]
            }
        })
        .flatten()
        .collect::<Vec<i128>>();

    let mut input2 = input.clone();


    loop {
        let left_most_free_block = input.iter().position(|x| x == &-1).unwrap();
        let last_data_block = input.iter().rposition(|x| x != &-1).unwrap();

        if left_most_free_block > last_data_block {
            break;
        }

        input.swap(left_most_free_block, last_data_block);
    }

    let sol1 = input
        .iter()
        .enumerate()
        .filter(|x| x.1 != &-1)
        .map(|x| x.0 as i128 * x.1)
        .sum::<i128>();

    println!("Solution 1: {}", sol1);

    let mut index_right = input2.len() - 1;

    println!("{:?}", input2);

    while index_right > 0 {
        while index_right > 0 && input2[index_right] == -1 {
            index_right -= 1;
        }
        if index_right == 0 {
            break;
        }
    
        let block_id = input2[index_right];
        let end_of_block = index_right;
        while index_right > 0 && input2[index_right] == block_id {
            index_right -= 1;
        }
   
        if input2[0] == block_id && index_right == 0 {
        }
        let start_of_block = if input2[index_right] == block_id {
            0
        } else {
            index_right + 1
        };
        let next_block_size = end_of_block - start_of_block + 1;
    
        let mut index_left = 0;
        let mut spot_found = false;
        
        while !spot_found && index_left < input2.len() {
            while index_left < input2.len() && input2[index_left] != -1 {
                index_left += 1;
            }
            let initial_index_left = index_left;
            
            let mut free_block_size = 0;
            while index_left < input2.len() && input2[index_left] == -1 {
                index_left += 1;
                free_block_size += 1;
            }
            
            if index_left <= start_of_block && free_block_size >= next_block_size {

                for i in 0..next_block_size {
                    input2.swap(start_of_block + i, initial_index_left + i);
                }
                spot_found = true;
            }
        }
    
    }


    let sol2 = input2
        .iter()
        .enumerate()
        .filter(|x| x.1 != &-1)
        .map(|x| x.0 as u128 * *x.1 as u128)
        .sum::<u128>();

    println!("Solution 2: {}", sol2);
}

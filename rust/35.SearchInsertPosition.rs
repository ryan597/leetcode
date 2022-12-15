impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut upper = nums.len();
        let mut lower = 0;
        let mut index = (upper -1) / 2;
        loop {
            let candidate = nums[index];
            if candidate > target {
                upper = index;  // set new upper bound
                index -= (upper - lower) / 2;
            }
            else if candidate < target {
                lower = index;  // set new lower bound
                index += (upper - lower) / 2;
            }
            else if candidate == target { 
                return index as i32
            }

            if upper - lower <= 1 {
                if nums[lower] >= target {
                    return lower as i32
                }
                else {
                    return upper as i32
                }
            }
        }
    }
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        match nums.len() {
            0 => return 0,
            1 => return 1,
            _ => {},
        };
        let mut counter = 0;
        let mut prev = -111;

        for i in 0..nums.len() {
            if nums[i] != prev {
                nums[counter] = nums[i];
                prev = nums[i];
                counter += 1;
            }
        }
        return counter as i32;
    }
}

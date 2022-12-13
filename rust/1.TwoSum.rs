impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        for i in (0..nums.len())
        {
            let mut num_search: i32 = target - nums[i];
            if (hashmap.contains_key(&num_search))
            {
                return vec!(*hashmap.get(&num_search).unwrap(), i as i32);
            }
            hashmap.insert(nums[i], i as i32);
        }
        return vec!(0,1);  // make the compiler happy
    }
}

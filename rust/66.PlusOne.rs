impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let size = digits.len();
        digits[size-1] += 1;
        for i in (0..size).rev() {
            if digits[i] > 9 {
                digits[i] = 0;
                if i != 0 {
                    digits[i-1] += 1;    
                }
                else {
                    digits.insert(0, 1);
                }
            }
            else {
                break;
            }
        }
        return digits;
    }
}

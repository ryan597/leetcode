impl Solution {
    pub fn is_valid(s: String) -> bool 
    {
        use std::collections::VecDeque;
        let mut iter = s.chars();
        let mut deq : VecDeque<char> = VecDeque::new();
        loop 
        {
            match iter.next()
            {
                // push back opening brackets (or rather what they must match)
                Some('(') => deq.push_back(')'),
                Some('[') => deq.push_back(']'),
                Some('{') => deq.push_back('}'),
                // returns false if not same as back of deque
                Some(b) => if (deq.pop_back() != Some(b)) {return false},
                None => return deq.is_empty(), // iterator has finished
                _ => return false,
            }  
        }
    }
}

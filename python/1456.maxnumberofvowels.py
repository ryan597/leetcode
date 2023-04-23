"""
1456. Maximum Number of Vowels in a Substring of Given Length

Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.

Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.

Example 1:

Input: s = "abciiidef", k = 3
Output: 3
Explanation: The substring "iii" contains 3 vowel letters.

Example 2:

Input: s = "aeiou", k = 2
Output: 2
Explanation: Any substring of length 2 contains 2 vowels.

Example 3:

Input: s = "leetcode", k = 3
Output: 2
Explanation: "lee", "eet" and "ode" contain 2 vowels.

Constraints:
    1 <= s.length <= 105
    s consists of lowercase English letters.
    1 <= k <= s.length
"""
class Solution:
    def maxVowels(self, s: str, k: int) -> int:
        max_vowels = 0
        vowels = {'a', 'e', 'i', 'o', 'u'}

        # max_vowels = sum(l in vowels for l in s[:k])
        for l in s[:k]:
            if l in vowels:
                max_vowels += 1

        tmp_vowels = max_vowels
        for l1, l2 in zip(s, s[k:]):
            if l2 in vowels:
                tmp_vowels += 1

            if l1 in vowels:
                tmp_vowels -= 1

            if tmp_vowels == k:
                return k
            elif tmp_vowels > max_vowels:
                max_vowels = tmp_vowels

        return max_vowels

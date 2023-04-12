class Solution:
    def longestPalindrome(self, s: str) -> str:
        """
        5. Longest Palindromic Substring
        Medium

        Given a string s, return the longest
        palindromic
        substring
        in s.

        Example 1:

        Input: s = "babad"
        Output: "bab"
        Explanation: "aba" is also a valid answer.

        Example 2:

        Input: s = "cbbd"
        Output: "bb"

        Constraints:

            1 <= s.length <= 1000
            s consist of only digits and English letters.
        """
        result = s[0]
        N = len(s)
        for i, letter in enumerate(s):
            tmp = 1
            j = 1
            # check for odd length palindromes
            while True:
                if i+j == N or i-j < 0:
                    break
                plus = s[i+j]
                minus = s[i-j]
                if plus == minus:
                    tmp += 2
                    j += 1
                else:
                    break

            if tmp > len(result):
                result = s[i-j+1:i+j]

            tmp = 0
            j = 1
            # check for even length palindromes
            while True:
                if i+j == N or i-j+1 < 0:
                    break
                plus = s[i+j]
                minus = s[i-j+1]
                if plus == minus:
                    tmp += 2
                    j += 1
                else:
                    break

            if tmp > len(result):
                result = s[i-j+2:i+j]

        return result

"""
1071. Greatest Common Divisor of Strings

For two strings s and t, we say "t divides s" if and only if s = t + ... + t (i.e., t is concatenated with itself
one or more times).

Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.


Example 1:

Input: str1 = "ABCABC", str2 = "ABC"
Output: "ABC"

Example 2:

Input: str1 = "ABABAB", str2 = "ABAB"
Output: "AB"

Example 3:

Input: str1 = "LEET", str2 = "CODE"
Output: ""


Constraints:

    1 <= str1.length, str2.length <= 1000
    str1 and str2 consist of English uppercase letters.

"""
class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        n1 = len(str1)
        n2 = len(str2)
        x = ""
        for i, [l1, l2] in enumerate(zip(str1, str2), start=1):
            if l1 == l2:
                if n1 % i == 0 and n2 % i == 0:
                    # possible gcd
                    x = str1[:i]
            else:
                return ""

        if n1 < n2:
            a = str2.split(x)
            if any(a):
                return ""
        elif n2 < n1:
            a = str1.split(x)
            if any(a):
                return ""
        return x

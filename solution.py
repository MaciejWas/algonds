import sys

def longest_common_substring(s1, s2):
    m = [[0] * (1 + len(s2)) for _ in range(1 + len(s1))]

    longest, x_longest = 0, 0

    for x in range(1, 1 + len(s1)):
        for y in range(1, 1 + len(s2)):
            if s1[x - 1] == s2[y - 1]:
                m[x][y] = m[x - 1][y - 1] + 1
                if m[x][y] > longest:
                    longest = m[x][y]
                    x_longest = x
            else:
                m[x][y] = 0

    return s1[x_longest - longest: x_longest]


str1 = sys.argv[1]
str2 = sys.argv[2]

print(longest_common_substring(str1, str2))

# https://adventofcode.com/2015/day/4

from hashlib import md5

def gethex(input):
    return md5(input.encode()).hexdigest()

for line in open('example.txt'):
    for i in range(10000000):
        hexmd5 = gethex(line + str(i))
        if hexmd5[:5] == '00000':
            print(i)
            break

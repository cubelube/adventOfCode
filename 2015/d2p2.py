allribbon = 0
for line in open('part1input.txt'):
    length, width, height = line.split('x')
    wrappresent = min(2 * (int(length) + int(width)), 2 * (int(width) + int(height)), 2 * (int(length) + int(height)))
    cubicvolume = int(length) * int(width) * int(height)
    allribbon += wrappresent + cubicvolume

print(allribbon)

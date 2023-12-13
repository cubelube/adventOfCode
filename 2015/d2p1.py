allpaper = 0
for line in open('part1input.txt'):
    length, width, height = line.split('x')
    area = 2 * int(length) * int(width) + 2 * int(width) * int(height) + 2 * int(length) * int(height)
    extrapaper = min(int(length) * int(width), int(width) * int(height), int(length) * int(height))
    allpaper += area + extrapaper

print(allpaper)

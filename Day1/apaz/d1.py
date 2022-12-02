print(max([sum([int(i) if i[-1:] else 0 for i in e.split("\n")]) for e in open("input.txt").read().split("\n\n")]))
print(sum(sorted([sum([int(i) if i[-1:] else 0 for i in e.split("\n")]) for e in open("input.txt").read().split("\n\n")])[-3:]))

# python3 convert.py && dot -T png input.gv > graph.png

with open("input.txt") as f:
    lines = list(map(lambda line: line.strip(), f.readlines()))

with open("input.gv", "w") as f:
    f.write("digraph {\n")
    for line in lines:
        f.write("\t")
        if line.startswith("%"):
            f.write(line[1:].split()[0] + " [color=red shape=box]")
        elif line.startswith("&"):
            f.write(line[1:].split()[0] + " [color=blue shape=diamond]")
        else:
            f.write(line.split()[0] + " [color=green shape=circle]")
        f.write("\n")

    for line in lines:
        f.write("\t")
        if line.startswith("%"):
            f.write(line[1:])
        elif line.startswith("&"):
            f.write(line[1:])
        else:
            f.write(line)
        f.write("\n")


    f.write("\n}")

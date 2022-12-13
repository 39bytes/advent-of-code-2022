import os
import sys
from typing import Self

class Node:
    least_size = sys.maxsize

    def __init__(self, size, children, parent):
        self.size = size
        self.children = children
        self.parent = parent
    
    def get_total_size(self):
        if not self.children:
            return self.size
        
        return sum([node.get_total_size() for node in self.children.values()])
    
    @classmethod
    def get_least_size(cls, node: Self, space_needed: int):
        size = node.get_total_size()
        if size >= space_needed and size < cls.least_size:
            cls.least_size = size
        if not node.children:
            return
        for node in node.children.values():
            cls.get_least_size(node, space_needed)

cur_node = Node(size=0, children={}, parent=None)
root = cur_node
    
with open(os.path.join(os.path.dirname(__file__), "..\\..\\inputs\\day7.txt")) as f:
    lines = f.readlines()
    i = 1
    while i < len(lines):
        line = lines[i]
        parts = line.split()
        cmd = parts[1]
        if cmd == "cd":
            if parts[2] == "..":
                cur_node = cur_node.parent
            else:
                cur_node = cur_node.children[parts[2]]
            i += 1
        elif cmd == "ls":
            i += 1
            while i < len(lines) and not lines[i].startswith("$"):
                output = lines[i].split()
                if output[0] == "dir":
                    node = Node(0, children={}, parent=cur_node)
                else:
                    node = Node(int(output[0]), children=None, parent=cur_node)
                cur_node.children[output[1]] = node
                
                i += 1

space_needed = 30000000 - (70000000 - root.get_total_size())
Node.get_least_size(root, space_needed)
print(Node.least_size)
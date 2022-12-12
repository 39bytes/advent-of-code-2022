import os
import sys
import copy

class Graph:
    def __init__(self):
        self.vertices = {}

    def add_vertex(self, vertex, adjacent):
        self.vertices[vertex] = adjacent
    
    def __str__(self):
        return str(self.vertices)
    
    def bfs(self, start, dest):
        queue = []
        visited = set()

        prev = {}
        
        visited.add(start)
        queue.append(start)
        while queue:
            u = queue.pop(0)
            
            for adjacent in self.vertices[u]:
                if adjacent not in visited:
                    visited.add(adjacent)
                    prev[adjacent] = u
                    queue.append(adjacent)

                    if adjacent == dest:
                        path = []
                        while prev.get(adjacent):
                            path.append(prev[adjacent])
                            adjacent = prev[adjacent]
                        return len(path)
        return sys.maxsize
                    

DIRECTIONS = [(1,0), (-1,0), (0,1), (0,-1)]

with open(os.path.join(os.path.dirname(__file__), "..\\..\\inputs\\day12.txt")) as f:
    grid = []
    for line in f.readlines():
        grid.append(list(line.rstrip('\n')))
    
    graph = Graph()
    starting_points = []
    
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            char = grid[i][j]
            if char == "S":
                char = 'a'
            if char == 'a':
                starting_points.append((i, j))
            if char == "E":
                dest = (i, j)
                char = 'z'
            height = ord(char)

            adjacent = []
            for dir in DIRECTIONS:
                if 0 <= i + dir[0] < len(grid) and 0 <= j + dir[1] < len(grid[0]):
                    adj_char = grid[i + dir[0]][j + dir[1]]
                    adj_height = ord(adj_char)
                    if adj_height - height <= 1:
                        adjacent.append((i + dir[0], j + dir[1]))
            graph.add_vertex((i, j), adjacent)

    print(min([graph.bfs(start, dest) for start in starting_points]))
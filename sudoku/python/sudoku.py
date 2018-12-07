import csv

with open("../sudoku.csv", "r") as csvfile:
    reader = csv.reader(csvfile)

    for row in reader:
        print row
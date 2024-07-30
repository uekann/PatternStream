import csv

items = dict() 
with open("data/OnlineRetail.csv") as f:
    f.readline()
    reader = csv.reader(f)
    for line in reader:
        if line[5] == "0":
            continue
        if not line[1] in items:
            items[line[1]] = (line[1], line[2], line[5])

with open("data/ItemList.csv", "w") as f:
    f.write("StockCode,Description,UnitPrice")
    for stock_code, description, unit_price in items.values():
        f.write("\n")
        if description.find(",") != -1 and description[0] != '"':
            description = f'"{description}"'
        f.write(f"{stock_code},{description},{unit_price}")

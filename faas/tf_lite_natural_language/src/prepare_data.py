import json

file_reader = open('labelmap.txt')
category_index = {}
for i, val in enumerate(file_reader):
    if i != 0:
        val = val[:-1]
        if val != '???':
            category_index.update({(i-1): {'id': (i-1), 'name': val}})
with open('labelmap_v2.txt', 'w') as file_writer:
    json.dump(category_index, file_writer)
file_writer.close()
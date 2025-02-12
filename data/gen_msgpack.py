import json
import msgpack
import os

dirname = os.path.dirname(__file__)

input_filename = os.path.join(dirname, 'classes.json')
output_filename = os.path.join(dirname, '../assets/classes.msgpack')

# Read the JSON file
with open(input_filename, "r", encoding="utf-8") as json_file:
    data = json.load(json_file)

# Convert to MessagePack format
with open(output_filename, "wb") as msgpack_file:
    msgpack_file.write(msgpack.packb(data))

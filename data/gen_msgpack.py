import json
import msgpack
import os

dirname = os.path.dirname(__file__)

classes_filename = os.path.join(dirname, "classes.json")
relations_filename = os.path.join(dirname, "relations.json")
output_filename = os.path.join(dirname, "../assets/classes.msgpack")


data = dict()

# Read the JSON files
with open(classes_filename, "r", encoding="utf-8") as json_file:
    data["classes"] = json.load(json_file)

with open(relations_filename, "r", encoding="utf-8") as json_file:
    data["relations"] = json.load(json_file)

# Convert to MessagePack format
with open(output_filename, "wb") as msgpack_file:
    msgpack_file.write(msgpack.packb(data))

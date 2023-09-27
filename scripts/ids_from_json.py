import json

# Open the JSON file and load the data
with open("sat-hunting.json", "r") as f:
    data = json.load(f)

# Extract the sat_hunter_id values
sat_hunter_ids = [item["sat_hunter_id"] for item in data]

# Write the sat_hunter_id values to a text file
with open("sat-hunter-ids.txt", "w") as f:
    for id in sat_hunter_ids:
        f.write(id + "\n")

import random
import json
# Generate a list of 100 random integers between 1 and 100
numbers = [random.randint(1, 100) for _ in range(100)]
# Create a JSON object
json_object = {
    "integers": numbers
}
# Convert the JSON object to a string
json_string = json.dumps(json_object)
# Print the JSON string
with open("small.json", "w") as f:
    f.write(json_string)

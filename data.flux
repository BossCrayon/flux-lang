print("--- FLUX DATA STRUCTURES ---")

// 1. Create a Dictionary
mut user = {
    "name": "Medici",
    "role": "Architect",
    "level": 100,
    "active": true
}

// 2. Access Data
print("User Profile:")
print(user)

print("Name: " + user["name"])
print("Level: " + user["level"])

// 3. Dynamic Access
mut key = "role"
print("Dynamic Role Lookup: " + user[key])
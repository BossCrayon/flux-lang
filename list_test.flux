print("--- FLUX LIST MANAGER ---")

// 1. Start with an empty list
mut todo = []

// 2. Add Items
todo = push(todo, "Buy Milk")
todo = push(todo, "Write Code")
todo = push(todo, "Sleep")

print("Current List: " + todo)
print("Task Count: " + len(todo))

// 3. List Logic
print("First Task: " + first(todo))
print("Last Task: " + last(todo))

// 4. Concatenation (Merging Lists)
mut urgent = ["Fix Server", "Call Boss"]
mut combined = urgent + todo

print("Combined Priority List: " + combined)
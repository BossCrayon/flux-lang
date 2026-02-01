// FLUX LOGBOOK v1.0
print("--- SYSTEM LOGIN ---")

mut name = input("Enter Username: ")
print("Welcome, " + name)

mut entry = input("Log Entry: ")
mut filename = name + "_log.txt"

// Save to disk
mut result = write_file(filename, entry)

if (result) {
    print("Success: Entry saved to " + filename)
} else {
    print("Error: Save failed.")
}

// database.flux
// A persistent append-only storage system (Robust Version)

mut filename = "flux_db.txt"

print("--- FLUX DATABASE v2.0 ---")
print("1. Read Database")
print("2. Add Record")
print("3. Clear Database")

mut choice = input("Select Option: ")

if (choice == "1") {
    // READ MODE
    mut content = read_file(filename)
    if (content == "") {
        print("[Database is empty]")
    } else {
        print("--- RECORDS ---")
        print(content)
        print("---------------")
    }
} 

if (choice == "2") {
    // WRITE MODE
    mut new_entry = input("Enter Data: ")
    mut old_content = read_file(filename)
    
    mut combined = ""
    
    // Logic: Only add the separator " | " if there is existing data
    if (old_content == "") {
        mut combined = new_entry
    } else {
        mut combined = old_content + " | " + new_entry
    }
    
    write_file(filename, combined)
    print("Saved: " + new_entry)
}

if (choice == "3") {
    // CLEAR MODE
    write_file(filename, "")
    print("Database cleared.")
}
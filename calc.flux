// calc.flux
// An interactive calculator shell

print("--- FLUX CALC v1.0 ---")
print("Type 'exit' as the first number to quit.")

mut running = true

while (running) {
    print("") // Empty line for spacing
    
    // 1. Get First Number
    mut s1 = input("Num 1 > ")
    
    if (s1 == "exit") {
        // Break the loop
        mut running = false
        print("Goodbye.")
    } else {
        // 2. Get Operator and Second Number
        mut op = input("Op (+ - * /) > ")
        mut s2 = input("Num 2 > ")
        
        // 3. Convert Strings to Integers (using our built-in int())
        mut n1 = int(s1)
        mut n2 = int(s2)
        
        // 4. Calculate
        if (op == "+") { print("= ", n1 + n2) }
        if (op == "-") { print("= ", n1 - n2) }
        if (op == "*") { print("= ", n1 * n2) }
        if (op == "/") { print("= ", n1 / n2) }
    }
}
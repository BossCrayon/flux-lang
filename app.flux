// app.flux
print("--- LOADING MODULE ---")

// 1. Import the library
// This runs math.flux and saves its variables into 'math'
mut math = import("math.flux")

// 2. Use variables
print("Loaded Module: ")
math["describe"]()

print("Value of PI: " + math["PI"])

// 3. Use functions
mut result = math["square"](10)
print("10 Squared is: " + result)

mut sum = math["add"](50, 50)
print("50 + 50 is: " + sum)
// game.flux
// The Final Challenge: Guess the Number
// Uses a custom Pseudo-Random Number Generator (LCG)

print("--- GUESS THE NUMBER v1.0 ---")

// 1. Initialize Random Seed (we use a fixed start for now)
mut seed = 12345

// 2. The Random Function (LCG Algorithm)
mut rand = fn(max) {
    // Formula: (a * seed + c) % m
    mut a = 1103515245
    mut c = 12345
    mut m = 2147483648
    
    // Update global seed (simulated by shadowing)
    mut temp = (a * seed + c)
    // We don't have modulo (%) yet! We simulate it: a - (m * (a / m))
    // This creates the "remainder"
    mut remainder = temp - (m * (temp / m))
    
    // Update the seed for next time (manually passed back in a real app, 
    // but here we just use the result as the 'random' number)
    
    // Scale it down to our range [0, max]
    mut scaled = remainder - (max * (remainder / max))
    return scaled
}

// Generate a secret number between 0 and 100
mut secret = rand(100)
mut attempts = 0
mut win = false

print("I have picked a number between 0 and 100.")

while (win == false) {
    mut guess_str = input("Your Guess: ")
    mut guess = int(guess_str)
    mut attempts = attempts + 1
    
    if (guess == secret) {
        print("CORRECT! You won in " + attempts + " tries.")
        mut win = true
    } else {
        if (guess < secret) {
            print("Too Low!")
        } else {
            print("Too High!")
        }
    }
}
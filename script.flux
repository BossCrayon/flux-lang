mut fib = fn(x) {
  mut a = 0
  mut b = 1
  mut i = 0
  
  while (i < x) {
    mut temp = a
    mut a = b
    mut b = temp + b
    mut i = i + 1
  }
  
  return a
}

fib(10)

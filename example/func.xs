def fact:i32 x:i32 {
  if x == 1 {
    1
  } else {
    x * fact(x - 1)
  }
}

print(fact(10))


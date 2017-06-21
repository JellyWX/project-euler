done = False
key = 1

def modulo(n):
  for i in range(1,21):
    if n % i != 0:
      return False
  return True

while not done:
  num = key*20
  done = modulo(num)

  key += 1

print(num)

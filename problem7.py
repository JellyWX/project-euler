primes = [2,3,5,7,11,13]
num = 15

while len(primes) < 10001:
  prime = True
  for p in primes:
    if num % p == 0:
      prime = False
      break
  if prime:
    primes.append(num)

  num += 2

print(primes)

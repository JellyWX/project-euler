'''nums = [i for i in range(2,20000)]
primes = 0

while len(nums) > 0:

  prime = nums.pop(0)
  print(prime)
  primes += prime

  for n in nums:
    if n % prime == 0:
      nums.remove(n)'''

primes = []
for n in range(2,20000):
  for p in primes:
    if n % p == 0:
      break
    if n < p*p:
      primes.append(n)
      break
  else:
    primes.append(n)

print(sum(primes))

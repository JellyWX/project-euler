prime = 600851475143
current_num = prime
factors = []
ran = False
hit = False

while current_num != 1 and not ran:
  for i in range(2,current_num + 1):
    if current_num % i == 0:
      print('got a hit: ' + str(current_num) + str(factors))
      factors.append(i)
      current_num //= i
      hit = True
  if not hit:
    ran = False

print(factors)
print('the largest prime factor is: ' + str(max(factors)))

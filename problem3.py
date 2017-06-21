prime = 13195
current_num = prime
factors = []

while current_num != 1:
  for i in range(2,current_num + 1):
    if current_num % i == 0:
      print('got a hit: ' + str(current_num) + str(factors))
      factors.append(i)
      current_num /= i

print(factors)

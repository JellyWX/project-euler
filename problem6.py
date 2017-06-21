sum_sq = 0
sq_sum = 0

for i in range(101):
  sum_sq += i**2
  sq_sum += i

sq_sum *= sq_sum

diff = sq_sum - sum_sq

print(diff)

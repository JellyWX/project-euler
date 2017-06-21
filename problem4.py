highest_value = 0

for i in range(100,1000):
  for j in range(100,1000):
    if str(i*j) == str(i*j)[::-1]:
      if i*j > highest_value:
        highest_value = i*j

print(highest_value)

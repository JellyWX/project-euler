from math import sqrt

def main():
  a = 1
  b = 1
  c = 1

  for c in range(1000):
    for b in range(c):
      for a in range(b):
        if a + b + c == 1000:
          if sqrt(a**2 + b**2) == c:
            print(str(a) + ' ' + str(b) + ' ' + str(c))
            print(a*b*c)
            return

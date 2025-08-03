def narcissistic(number: int):
    sum = 0
    s = str(number)
    for digit in s:
        print(digit)
        sum += int(digit) ** len(s)
    return sum == number


print(narcissistic(153))

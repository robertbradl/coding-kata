def high_and_low(numbers: str):
    return f"{min(map(int, numbers.split()))} {max(map(int, numbers.split()))}"

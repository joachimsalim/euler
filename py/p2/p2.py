def fibonacci_sequence_of_evens(y, x, ceiling):
    z = 0
    while True:
        w = y + x
        y = x
        x = w

        if y >= ceiling:
            break

        z += is_even(y)
    return z

def is_even(x):
    if x % 2 == 0:
        return x
    return 0

if __name__ == '__main__':
    ceiling = 4000000
    sum = fibonacci_sequence_of_evens(1, 2, ceiling)
    print(sum)


def multiples_of_three_or_five(ceiling):
    y = 0
    for i in range(3, ceiling):
        if i % 3 == 0 or i % 5 == 0:
            y += i
    return y

if __name__ == '__main__':
    ceiling = 1000
    sum = multiples_of_three_or_five(ceiling)
    print(sum)


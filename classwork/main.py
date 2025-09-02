def make_change(amount):
    changes = []
    coins = [100, 50, 25, 10, 5, 1]

    remaining = amount
    for coin in coins:
        count = remaining // coin
        if count > 0:
            changes.append((coin, count))
            remaining -= count * coin
    return changes



if __name__ == "__main__":
    amount = 287
    change = make_change(amount)
    for coin, count in change:
        print(f"Coin: {coin}, Count: {count}")
























"""
class Dot:
    pass

class Group:
    def __init__(self):
        self.dots = []

    def __add__(self, other):
        new_group = Group()

        if type(other) == Group:
            for dot in other.dots:
                new_group.dots.append(Dot())
        
        elif type(other) == Dot:
            new_group.dots.append(Dot())

        for dots in self.dots:
            new_group.dots.append(Dot())

        return new_group
    
    def __repr__(self):
        return f"[{"." * len(self.dots)}]"

group_1 = (Group() + Dot()) + Dot()
group_2 = (Group() + Dot())
group_3 = Group() + Dot() + Dot() + Dot() + Dot()

print(group_1 + group_2, "=", group_2 + group_1)
print((group_1 + group_2) + group_3 , "=", group_1 + (group_2 + group_3))

"""





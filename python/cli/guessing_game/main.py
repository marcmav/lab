import random

class Score:
    def __init__(self, ) -> None:
        pass

def main() -> None:
    random_number: int = random.randint(0, 100)
    for i in range(1, 11):
        guess: int = int(input("Enter your guess: "))
        if guess < random_number:
            print("Your guess is to low")
        elif guess > random_number:
            print("Your guess is to high")
        else:
            print("You got it right")
            break


if __name__ == "__main__":
    main()

import random


def begin() -> None:
    print("Welcome to the Number Guessing Game!")
    print("I'm thinking of a number between 1 and 100.")
    print("You have 5 chances to guess the correct number.")


def get_attempts() -> int:
    print("\nPlease select the difficulty level:")
    print("1. Easy (10 chances)")
    print("2. Medium (5 chances)")
    print("3. Hard (3 chances)\n")
    option: int = int(input("Enter your choice: "))
    difficulty: str = ""
    attempts: int = 0
    if option == 1:
        difficulty = "Easy"
        attempts = 10
    elif option == 2:
        difficulty = "Medium"
        attempts = 5
    elif option == 3:
        difficulty == "Hard"
        attempts = 3
    print(f"Great! You have selected the {difficulty} difficulty level.")
    return attempts


def process_game() -> None:
    attempts: int = get_attempts()
    print("Let's start the game!\n")
    random_number: int = random.randint(1, 100)
    i: int = 1
    while i <= attempts:
        choice: int = int(input("Enter your guess: "))
        if choice < random_number:
            print(f"Incorrect! The number is higher than {choice}")
        elif choice > random_number:
            print(f"Incorrect! The number is lower than {choice}")
        else:
            print(f"Congratulations! You guessed the correct number in {i} attempts.")
            return
        i += 1
    print(f"You lost, the number was {random_number}")


def main() -> None:
    begin()
    repeat: bool = True
    while repeat:
        process_game()
        reply: str = input("\nDo you want to play again? ")
        if reply.lower() != "yes":
            repeat = False
    print("Thank you for playing")


if __name__ == "__main__":
    main()

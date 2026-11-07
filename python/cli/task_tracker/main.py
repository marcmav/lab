import sys
import json


def main() -> None:
    args: list[str] = sys.argv[1:]
    if len(args) == 0:
        print("Error: Not enought arguments", file=sys.stderr)
        return

    tasks: TaskList = TaskList()
    with open("todo.json", "r") as f:
        tasks = json.load(f)

    if args[0] == "add":
        if len(args) == 2:
            tasks.add(args[1])
        else:
            print(f"Usage: {args[0]} add 'the task description'")

    with open("todo.json", "r") as f:
        json.dump(tasks, f)


if __name__ == "__main__":
    main()

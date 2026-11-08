import sys
import json
from task import *


def main() -> None:
    p_name: str = sys.argv[0]
    args: list[str] = sys.argv[1:]
    if len(args) == 0:
        print("Error: Not enought arguments", file=sys.stderr)
        return

    tasks: list[Task] = [json.load(open("TODO.json", "r"))]

    if args[0] == "add":
        if len(args) == 2:
            add_task(tasks, args[1])
        else:
            print(f"Usage: {p_name} add [description]")

    print(tasks)
    """
    elif args[0] == "update":
        if len(args) == 3:
            try:
                id: int = int(args[1])
            except ValueError:
                print(f"Usage: {p_name} update [id] [description]")
            else:
                update_task(tasks, id, args[2])
        else:
            print(f"Usage: {p_name} update [id] [description]")

    with open("TODO.json", "r") as f:
        json.dump(tasks, f)
    """


if __name__ == "__main__":
    main()

import sys
import json
from task import *


def main() -> None:
    p_name: str = sys.argv[0]
    args: list[str] = sys.argv[1:]
    if len(args) == 0:
        print("Error: Not enought arguments", file=sys.stderr)
        return

    try:
        tasks: list[Task] = json.load(open("TODO.json", "r"))
    except ValueError:
        tasks: list[Task] = []

    if args[0] == "add":
        if len(args) == 2:
            add_task(tasks, args[1])
        else:
            print(f"Usage: {p_name} add [description]")

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

    elif args[0] == "delete":
        if len(args) == 2:
            delete_task(tasks, args[1])
        else:
            print(f"Usage: {p_name} delete [id]")

    elif args[0] in ("mark-in-progress", "mark-done"):
        if len(args) == 2:
            mark_task(tasks, args[1], args[0])
        else:
            print(f"Usage: {p_name} [mark-in-progress | mark-done]")

    elif args[0] == "list":
        if len(args) == 2:
            list_task(tasks, "todo")
        elif len(args) == 2:
            if args[1] in ("done", "todo", "in-progress"):
                list_task(tasks, args[1])
        else:
            print(f"Usage: {p_name} list [status]")

    json.dump(tasks, open("TODO.json", "w"))


if __name__ == "__main__":
    main()

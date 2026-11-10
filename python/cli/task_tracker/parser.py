from task import *


def parse(tasks: Task, args: list[str]):
    if args[1] == "add":
        if len(args) == 3:
            tasks.add(args[2])
        else:
            print(f"Usage: {args[0]} add [description]")


    elif args[1] == "update":
        if len(args) == 4 and args[2].isdigit():
            tasks.update(args[2], args[3])
        else:
            print(f"Usage: {args[0]} update [id] [description]")

    elif args[1] == "delete":
        if len(args) == 3:
            tasks.delete(args[2])
        else:
            print(f"Usage: {args[0]} delete [id]")

    elif args[1] in ("mark-in-progress", "mark-done"):
        if len(args) == 3:
            tasks.mark(args[2], args[1])
        else:
            print(f"Usage: {args[0]} [mark-in-progress | mark-done]")

    elif args[1] == "list":
        if len(args) == 2:
            tasks.list_task()
        elif len(args) == 3:
            if args[2] in ("done", "todo", "in-progress"):
                list_task(tasks, args[2])
        else:
            print(f"Usage: {args[0]} list [status]")

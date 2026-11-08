from task import *


def parser(tasks: list[Task], args: list[str]):
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

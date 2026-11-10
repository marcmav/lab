import sys
import json
import serde
import task
#from parser import parse


def main() -> None:
    args: list[str] = sys.argv[:]
    if len(args) == 1:
        print("Error: Not enought arguments", file=sys.stderr)
        return

    tasks: task.Task = task.Task()
    try:
        with open("TODO.json", "r") as f:
            saved: list[dict[str, str]] = json.load(open("TODO.json", "r"))
    except (FileNotFoundError, json.JSONDecodeError):
        pass
    else:
        tasks.tasks = saved

    print(tasks.tasks)
    if args[1] == "add":
        if len(args) == 3:
            tasks.add(args[2])
        else:
            print(f"Usage: {args[0]} add [description]")

    with open("TODO.json", "w") as f:
        json.dump(tasks, f, default=serde.encode, indent=4)


if __name__ == "__main__":
    main()

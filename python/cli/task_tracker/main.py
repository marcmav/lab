import sys
import json
import serde
import task
import parser


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

    parser.parse(tasks, args)

    with open("TODO.json", "w") as f:
        json.dump(tasks, f, default=serde.encode, indent=4)


if __name__ == "__main__":
    main()

import sys
import json
import serde
#from parser import parse


def main() -> None:
    args: list[str] = sys.argv[:]
    if len(args) == 1:
        print("Error: Not enought arguments", file=sys.stderr)
        return

    try:
        tasks: list[Task] = json.load(open("TODO.json", "r"), object_hook=serde.decode)
    except ValueError:
        tasks: list[Task] = []

    #parse(tasks, args)

    json.dump(tasks, open("TODO.json", "w"), default=serde.encode)


if __name__ == "__main__":
    main()

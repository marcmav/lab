import sys


class Task:
    def __init__(self, id: int, description: str, status: str) -> None:
        self.id: int = id
        self.description: str = description
        self.status: str = status

    @classmethod
    def new(cls, id: int, description: str, status: str) -> cls:
        return cls(id, description, status)


class TaskList():
    def __init__(self) -> None:
        self.tasks: list[Task] = []

    def add(self, description: str) -> None:
        id: int = len(self.tasks)
        self.tasks.append(Task.new(id, description, "Todo"))


def main() -> None:
    args: list[str] = sys.argv[1:]
    if len(args) == 0:
        print("Error: Not enought arguments", file=sys.stderr)
        return
    tasks: TaskList = TaskList()
    if args[0] == "add":
        if len(args) == 2:
            tasks.add(args[1])
        else:
            print(f"Usage: {args[0]} add 'the task description'")
    print(tasks.tasks)



if __name__ == "__main__":
    main()

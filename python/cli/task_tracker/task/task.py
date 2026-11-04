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

    @staticmethod
    def get_missing_id(tasks: list[Task]) -> int:
        highest: int = 0
        exists: set[int] = set([task.id for task in tasks])
        for task in tasks:
            if task.id > highest:
                highest = task.id
        all: set[int] = set(range(1, highest))
        missing: int = 0
        all -= exists
        if len(all) == 0:
            return highest + 1
        return list(all)[0]


    def add(self, description: str) -> None:
        id: int = self.get_missing_id(self.tasks)
        self.tasks.append(Task.new(id, description, "Todo"))



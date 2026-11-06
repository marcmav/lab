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

    @staticmethod
    # TODO
    def is_valid_id(id: int, tasks: list[task]) -> tuple[bool, int]:
            pass

    # TODO - update functions that change id so that id will always be index + 1
    def add(self, description: str) -> None:
        id: int = len(self.tasks) + 1
        self.tasks.append(Task.new(id, description, "Todo"))
        print(f"Task added successfully (ID: {id})")

    def delete(self, id: int) -> None:
        size: int = len(self.tasks)
        if size == 0:
            print(f"Task (ID: {id}) doesn't exist")
            return
        elif size > 1:
            self.tasks[id - 1].description = self.tasks[-1].description
            self.tasks[id - 1].status = self.tasks[-1].status
        self.tasks.pop()
        print(f"Task deleted successfully (ID: {id})")

    def update(self, id: int, description: str) -> None:
        if len(self.tasks) > id:
            self.tasks[id - 1].description = description
            print(f"Task updated successfully (ID: {id})")
        else:
            print(f"Task (ID: {id}) doesn't exist")


    def mark(self, id: int, status: str) -> None:
        if len(self.tasks) > id:
            self.tasks[id - 1].status = status
            print(f"Task updated successfully (ID: {id})")
        else:
            print(f"Task (ID: {id}) doesn't exist")

    def list_task(self, status: str) -> None:
        for task in self.tasks:
            if task.status == status:
                print(task.description)

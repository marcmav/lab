class Task():
    def __init__(self) -> None:
        self.tasks: list[dict[str, str]] = []

    def new(id: str, description: str, status: str) -> dict[str, str]:
        return {"id": id, "description": description, "status": status}

    def add(self, description: str) -> None:
        id: str = str(len(self.tasks) + 1)
        self.tasks.append(new_task(id, description, "todo"))
        print(f"Task added successfully (ID: {id})")

    def delete(self, id: str) -> None:
        size: int = len(self.tasks)
        if size >= int(id):
            self.tasks[int(id) - 1]["description"] = self.tasks[-1]["description"]
            self.tasks[int(id) - 1]["status"] = self.tasks[-1]["status"]
            self.tasks.pop()
            print(f"Task deleted successfully (ID: {id})")
        else:
            print(f"Task (ID: {id}) doesn't exist")

    def update(self, id: str, description: str) -> None:
        if len(self.tasks) >= int(id):
            self.tasks[int(id) - 1]["description"] = description
            print(f"Task updated successfully (ID: {id})")
        else:
            print(f"Task (ID: {id}) doesn't exist")

    def mark(self, id: str, status: str) -> None:
        if len(self.tasks) >= int(id):
            self.tasks[int(id) - 1]["status"] = status
            print(f"Task updated successfully (ID: {id})")
        else:
            print(f"Task (ID: {id}) doesn't exist")

    def list_task(self, status: str | None = None) -> None:
        for task in self.tasks:
            if task["status"] == status or not status:
                print(task["description"])

Task = dict[str, str]


def new_task(id: str, description: str, status: str) -> Task:
    return {"id": str, "description": description, "status": status}


def add_task(tasks: list[Task], description: str) -> None:
    id: str = str(len(tasks) + 1)
    tasks.append(new_task(id, description, "Todo"))
    print(f"Task added successfully (ID: {id})")


def delete_task(tasks: list[Task], id: str) -> None:
    size: int = len(tasks)
    if size >= int(id):
        tasks[int(id) - 1]["description"] = tasks[-1]["description"]
        tasks[int(id) - 1]["status"] = tasks[-1]["status"]
        tasks.pop()
        print(f"Task deleted successfully (ID: {id})")
    else:
        print(f"Task (ID: {id}) doesn't exist")


def update_task(tasks: list[Task], id: str, description: str) -> None:
    if len(tasks) >= int(id):
        tasks[int(id) - 1]["description"] = description
        print(f"Task updated successfully (ID: {id})")
    else:
        print(f"Task (ID: {id}) doesn't exist")


def mark_task(tasks: list[Task], id: str, status: str) -> None:
    if len(tasks) >= int(id):
        tasks[int(id) - 1]["status"] = status
        print(f"Task updated successfully (ID: {id})")
    else:
        print(f"Task (ID: {id}) doesn't exist")


def list_task(tasks: list[Task], status: str) -> None:
    for task in tasks:
        if task["status"] == status:
            print(task["description"])

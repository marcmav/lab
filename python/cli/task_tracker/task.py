def new(id: int, description: str, status: str) -> dict[str, int | str]:
    return {"id": id, "description": description, "status": status}


def add(tasks: list[dict[str, int | str]], description: str) -> None:
    id: int = len(tasks) + 1
    tasks.append(new(id, description, "Todo"))
    print(f"Task added successfully (ID: {id})")


def delete(tasks: list[dict[str, int | str]], id: int) -> None:
    size: int = len(tasks)
    if size >= id:
        tasks[id - 1]["description"] = tasks[-1]["description"]
        tasks[id - 1]["status"] = tasks[-1]["status"]
        tasks.pop()
        print(f"Task deleted successfully (ID: {id})")
    else:
        print(f"Task (ID: {id}) doesn't exist")


def update(tasks: list[dict[str, int | str]], id: int, description: str) -> None:
    if len(tasks) >= id:
        tasks[id - 1]["description"] = description
        print(f"Task updated successfully (ID: {id})")
    else:
        print(f"Task (ID: {id}) doesn't exist")


def mark(tasks: list[dict[str, int | str]], id: int, status: str) -> None:
    if len(tasks) >= id:
        tasks[id - 1]["status"] = status
        print(f"Task updated successfully (ID: {id})")
    else:
        print(f"Task (ID: {id}) doesn't exist")


def list_task(tasks: list[dict[str, int | str]], status: str) -> None:
    for task in tasks:
        if task["status"] == status:
            print(task["description"])

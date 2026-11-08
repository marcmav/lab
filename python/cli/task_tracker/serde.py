import task


def encode(obj: object) -> object:
    if isinstance(obj, task.Task):
        return obj.tasks
    raise TypeError(f"Cannot serialize {type(obj).__name__}")


def decode(tasks: list[dict[str, str]]) -> task.Task:
    for task in tasks:
        if "id" in task:
            obj: task.Task = task.Task()
            obj.tasks = tasks
    return obj

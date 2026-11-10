import task


def encode(obj: object) -> object:
    if isinstance(obj, task.Task):
        return obj.tasks
    raise TypeError(f"Cannot serialize {type(obj).__name__}")

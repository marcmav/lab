from fastapi import FastAPI
import json


app = FastAPI()


@app.get("/users")
def fetch_users() -> list[str]:
    users: list[str] = []
    try:
        with open("users.json", "r") as f:
            users = load(f)
    except Exception:
        pass
    return users


@app.post("/users/{user_name}")
def create_user(user_name: str) -> list[str]:
    users: list[str] = []
    try:
        with open("users.json", "r") as f:
            users = json.load(f)
    except Exception:
        pass
    users.append(user_name)
    try:
        with open("users.json", "w") as f:
            json.dump(users, f)
    except Exception:
        pass
    return users

import sys
try:
    import requests
    from requests import Response
except ModuleNotFoundError as e:
    print(e)
    print("To install dependencies run: pip install -r requirements.txt")
    exit(1)


def main() -> None:
    if len(sys.argv) != 2:
        print("Error: Invalid Start")
        print(f"Usage: {sys.argv[0]} <username>")
        exit(1)
    username: str = sys.argv[1]
    url: str = f"https://api.github.com/users/{username}/events"
    response: Response = requests.get(url)
    print("Output:")
    body: list[dict[object, object]] = response.json()
    push_event: dict[str, int] = {}
    for content in body:
        repo: str = content["repo"]["name"]
        match content.get("type"):
            case "PushEvent":
                push_event[repo] = push_event.get(repo, 0) + 1
            case _:
                pass
    for event in push_event:
        print(f"- Pushed {push_event[event]} commits to {event}")



if __name__ == "__main__":
    main()

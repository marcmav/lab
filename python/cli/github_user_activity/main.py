import sys
try:
    import requests
    from requests import Response


def main() -> None:
    if len(sys.argv) != 2:
        print("Error: Invalid Start")
        print(f"Usage: {sys.argv[0]} <username>")
        exit(1)
    username: str = sys.argv[1]
    url: str = f"https://api.github.com/users/{username}/events"
    response: Response = requests.get(url)
    print(response.status_code)
    print(response.json())


if __name__ == "__main__":
    main()

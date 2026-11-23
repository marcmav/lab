import requests
from requests import Response


url: str = "http://127.0.0.1:8000/users/marcmav"
response: Response = requests.post(url)
if response.ok:
    print(response.json())

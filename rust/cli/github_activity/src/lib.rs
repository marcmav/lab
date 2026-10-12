use serde:: {Deserialize};

#[derive(Deserialize)]
struct Actor {
    id: u32,
    login: String,
    display_login: String,
    gravator_id: String,
    url: String,
    avatar_url: String,
}

#[derive(Deserialize)]
struct Repo {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct EventData {
    id: u32,
    type_: String,
    actor: Actor,
    repo: Repo,
    payload: 
    public: bool,
    created_at: String,
}

/* map
  {
    "id": "22249084947",
    "type": "WatchEvent",
    "actor": {
      "id": 583231,
      "login": "octocat",
      "display_login": "octocat",
      "gravatar_id": "",
      "url": "https://api.github.com/users/octocat",
      "avatar_url": "https://avatars.githubusercontent.com/u/583231?v=4"
    },
    "repo": {
      "id": 1296269,
      "name": "octocat/Hello-World",
      "url": "https://api.github.com/repos/octocat/Hello-World"
    },
    "payload": {
      "action": "started"
    },
    "public": true,
    "created_at": "2022-06-09T12:47:28Z"
  },
*/

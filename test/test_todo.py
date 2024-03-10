import requests


def test_add_todo():
    json = {
        "name": "Buy milk",
        "description": "Go to the store and buy milk",
        "deadline": "2021-12-31T23:59:59.999Z",
    }
    response = requests.post("http://localhost:3000", json=json)
    assert response.status_code == 200

    response_index = requests.get("http://localhost:3000")
    assert response_index.json()[0]["name"] == "Buy milk"
    assert response_index.json()[0]["description"] == "Go to the store and buy milk"
    assert response_index.json()[0]["deadline"] == "2021-12-31T23:59:59.999Z"

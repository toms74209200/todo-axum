import requests


def test_index():
    response = requests.get("http://localhost:3000")
    assert response.status_code == 200
    assert response.text == "Hello, world!"

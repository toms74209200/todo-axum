import datetime
import requests

from lib.utils import random_string


def test_update_task_normal():
    email = f"{random_string(10)}@example.com"
    password = "password"

    user_response = requests.post(
        "http://localhost:3000/users",
        json={
            "email": email,
            "password": password,
        },
    )
    assert user_response.status_code == 201

    auth_response = requests.post(
        "http://localhost:3000/auth",
        json={
            "email": email,
            "password": password,
        },
    )
    assert auth_response.status_code == 200
    token = auth_response.json()["token"]

    deadline = datetime.datetime.now() + datetime.timedelta(days=1)
    task_response = requests.post(
        "http://localhost:3000/tasks",
        headers={
            "Authorization": f"Bearer {token}",
        },
        json={
            "name": "task1",
            "description": "description1",
            "deadline": deadline.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": False,
        },
    )
    assert task_response.status_code == 201
    requests.post(
        "http://localhost:3000/tasks",
        headers={
            "Authorization": f"Bearer {token}",
        },
        json={
            "name": "task2",
            "description": "description2",
            "deadline": datetime.datetime.now().strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": False,
        },
    )

    name = "updated_task1"
    description = "updated"
    deadline = datetime.datetime.now() + datetime.timedelta(days=2)
    completed = True
    update_response = requests.put(
        f"http://localhost:3000/tasks/{task_response.json()['id']}",
        headers={
            "Authorization": f"Bearer {token}",
        },
        json={
            "name": name,
            "description": description,
            "deadline": deadline.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": completed,
        },
    )

    assert update_response.status_code == 200
    assert update_response.json()["name"] == name
    assert update_response.json()["description"] == description
    assert update_response.json()["deadline"] == deadline.strftime("%Y-%m-%dT%H:%M:%SZ")
    assert update_response.json()["completed"] == completed

    get_response = requests.get(
        f"http://localhost:3000/tasks?userId={user_response.json()['id']}",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )

    assert get_response.status_code == 200
    assert get_response.json()[0]["name"] == name
    assert get_response.json()[0]["description"] == description
    assert get_response.json()[0]["deadline"] == deadline.strftime("%Y-%m-%dT%H:%M:%SZ")
    assert get_response.json()[0]["completed"] == completed


def test_update_tasks_with_invalid_request_then_bad_request():
    update_response = requests.put("http://localhost:3000/tasks/1", json={})
    assert update_response.status_code == 400


def test_update_tasks_with_invalid_token_then_unauthorized():
    update_response = requests.put(
        "http://localhost:3000/tasks/1",
        headers={
            "Authorization": "Bearer " + random_string(100),
        },
        json={},
    )
    assert update_response.status_code == 401


def test_update_tasks_with_invalid_id_then_not_found():
    email = f"{random_string(10)}@example.com"
    password = "password"

    user_response = requests.post(
        "http://localhost:3000/users",
        json={
            "email": email,
            "password": password,
        },
    )
    assert user_response.status_code == 201

    auth_response = requests.post(
        "http://localhost:3000/auth",
        json={
            "email": email,
            "password": password,
        },
    )
    assert auth_response.status_code == 200
    token = auth_response.json()["token"]

    update_response = requests.put(
        "http://localhost:3000/tasks/1",
        headers={
            "Authorization": f"Bearer {token}",
        },
        json={
            "name": "task1",
            "description": "description1",
            "deadline": datetime.datetime.now().strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": False,
        },
    )
    assert update_response.status_code == 404

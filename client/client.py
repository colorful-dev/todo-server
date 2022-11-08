import requests


def index_all():
    res = requests.get('http://localhost:8080/')
    print(res.json())


index_all()

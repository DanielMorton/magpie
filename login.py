import getpass
import requests
from lxml import html

LOGIN_URL = 'https://secure.birds.cornell.edu/cassso/login'


def login():
    session = requests.session()
    result = session.get(LOGIN_URL)

    tree = html.fromstring(result.text)
    authenticity_token = list(set(tree.xpath("//input[@name='lt']/@value")))[0]

    print("Enter Username")
    user = input("Username: ")
    print("Enter Password")
    password = getpass.getpass()
    login_data = dict(username=user, password=password,
                      lt=authenticity_token,
                      execution="e1s1",
                      _eventId="submit"
                      )

    session.post(LOGIN_URL, data=login_data)
    del password, login_data
    return session

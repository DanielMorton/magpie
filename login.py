import getpass
import requests
from lxml import html

LOGIN_URL = 'https://secure.birds.cornell.edu/cassso/login?service=https%3A%2F%2Febird.org%2Flogin%2Fcas%3Fportal%3Debird&locale=en_US'


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

    session.post(LOGIN_URL, data=login_data,
                 headers=dict(referer=LOGIN_URL))
    del password, login_data
    return session

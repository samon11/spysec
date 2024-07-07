import time

from client import SECClient
from form_parser import Form4Parser
from data import MongoDatabase

DATABASE_URI = 'mongodb://mongo:mongo@localhost:27017/?authMechanism=DEFAULT'
DATABASE_NAME = 'spysec'

def main():
    db = MongoDatabase(DATABASE_URI, DATABASE_NAME)
    client = SECClient()
    rss_feed = client.get_rss_feed('4', 40)

    db.insert_one('rssfeeds', rss_feed.model_dump())
    for form in client.parse_rss_xml(rss_feed):
        try:
            existing = db.find_many('forms', {'xmlUrl': form.xmlUrl})
            if not existing:
                print("Processing form", form.xmlUrl)
                model = Form4Parser.parseXML(form)
                db.insert_one('forms', model.model_dump())
        except Exception as e:
            print(f'Error processing form {form.xmlUrl}: {e}')
            continue


if __name__ == '__main__':
    main()

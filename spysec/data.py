import os
from typing import List
import pymongo
from dotenv import load_dotenv

from models import Form4
load_dotenv()

DB_URL = os.environ.get("MONGO_URL")

class MongoDatabase:
    def __init__(self, uri: str, database: str):
        self.client = pymongo.MongoClient(uri)
        self.db = self.client.get_database(database)

    def insert_one(self, collection: str, document: dict):
        self.db[collection].insert_one(document)

    def insert_many(self, collection: str, documents: list[dict]):
        self.db[collection].insert_many(documents)

    def find_many(self, collection: str, query: dict) -> List[dict]:
        cursor = self.db[collection].find(query)
        return [result for result in cursor]
    
    def get_latest_rss_hash(self) -> str:
        return self.db['rssfeeds']\
            .find_one(
                sort=[('createdAtUtc', pymongo.DESCENDING)])['feedHash']
    
    def insert_form4(self, form: Form4):
        existing = self.db['forms'].find_one({'xmlUrl': form.xmlUrl})
        if existing:
            self.db['forms'].replace_one({'xmlUrl': form.xmlUrl}, form.model_dump())
        else:
            self.insert_one('forms', form.model_dump())

    def query_forms(self, query: dict):
        return [Form4(**doc) for doc in self.db['forms'].find(query).sort({"formDate": -1})]

db = MongoDatabase(uri=DB_URL, database="spysec")

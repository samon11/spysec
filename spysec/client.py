import json
from typing import Generator, List, Optional
import requests
import time
from bs4 import BeautifulSoup

from form_parser import Form4Parser
from models import RSSFeed, WebForm

class SECClient:
    BASE_URL = 'https://www.sec.gov'

    def __init__(self):
        self.rss_url = 'https://www.sec.gov/cgi-bin/browse-edgar?action=getcurrent&CIK=&type=:type&company=&dateb=&owner=include&start=0&count=:count&output=atom'
        
        self.session = requests.Session()
        self.session.headers.update({
            "User-Agent": "Michael Samon mjsamon@icloud.com"
        })

        self.filings = []

    @staticmethod
    def parse_rss_urls(rss_feed: str) -> List[str]:
        soup = BeautifulSoup(rss_feed, 'xml')
        entries = soup.find_all('entry')
        return set([entry.find('link').get('href') for entry in entries])
    
    @staticmethod
    def parse_xml_from_filing(filing_response: str) -> Optional[str]:
        '''Parse the URL to the XML file from the filing response.'''
        soup = BeautifulSoup(filing_response, 'html.parser')
        all_links = soup.find_all('a')
        for link in all_links:
            if link.text.lower().endswith('xml'):
                return SECClient.BASE_URL + link.get('href')

        return None

    def get_rss_feed(self, form_type: str, count: int) -> RSSFeed:
        url = self.rss_url.replace(':type', form_type).replace(':count', str(count))
        response = self.session.get(url)
        response.raise_for_status()

        return RSSFeed(content=response.text, formType=form_type)
    
    def _get_filing(self, url: str):
        response = self.session.get(url)
        response.raise_for_status()
        return response.text

    def parse_rss_xml(self, rss_feed: RSSFeed) -> Generator[WebForm, None, None]:
        url_list = self.parse_rss_urls(rss_feed.content)
        existing = set()
        for filing_url in url_list:
            try:
                filing_response = self._get_filing(filing_url)
                xml_url = self.parse_xml_from_filing(filing_response)
                if xml_url and xml_url not in existing:
                    response = self.session.get(xml_url)
                    response.raise_for_status()
                    existing.add(xml_url)
                    yield WebForm(xmlUrl=xml_url, content=response.text, htmlUrl=filing_url)
            except Exception as e:
                print(f'Error processing filing: {e}')
                continue

    def get_recent_forms(self, form_type: str, count: int = 40) -> Generator[WebForm, None, None]:
        rss_feed = self.get_rss_feed(form_type, count)
        url_list = self.parse_rss_urls(rss_feed)
        existing = set()
        for filing_url in url_list:
            try:
                filing_response = self._get_filing(filing_url)
                xml_url = self.parse_xml_from_filing(filing_response)
                if xml_url and xml_url not in existing:
                    response = self.session.get(xml_url)
                    response.raise_for_status()
                    existing.add(xml_url)
                    yield WebForm(xmlUrl=xml_url, content=response.text, htmlUrl=filing_url)
            except Exception as e:
                print(f'Error processing filing: {e}')
                continue
    
if __name__ == '__main__':
    client = SECClient()
    rss_feed = client.get_recent_forms('4')
    form_data = []
    for form in rss_feed:
        print("Processing form", form.xmlUrl)
        model = Form4Parser.parseXML(form)
        form_data.append(model.model_dump(mode='json'))

    with open('form_data.json', 'w') as f:
        json.dump(form_data, f, indent=4)

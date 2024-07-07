import os
import logging
import azure.functions as func
from data import MongoDatabase
from client import SECClient
from form_parser import Form4Parser

DATABASE_URI = os.environ['AZURE_COSMOS_CONNECTIONSTRING']
BACKFILL_WINDOW = os.environ.get('BACKFILL_WINDOW', 40)
DATABASE_NAME = 'spysec'

app = func.FunctionApp()
db = MongoDatabase(DATABASE_URI, DATABASE_NAME)
client = SECClient()

@app.schedule(schedule="0 * * * * *", arg_name="timer", run_on_startup=True,
              use_monitor=True) 
def timer_trigger(timer: func.TimerRequest) -> None:
    if timer.past_due:
        logging.info('The timer is past due!')

    logging.info('Python timer trigger function executed.')
    rss_feed = client.get_rss_feed('4', BACKFILL_WINDOW)

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

from email.mime.application import MIMEApplication
from email.mime.multipart import MIMEMultipart
import arrow
import os
import smtplib, ssl
from secret import CONN, PASS, EMAIL, TO, PORT
import psycopg2

def track_sent():
    now = arrow.utcnow().to("US/Eastern").shift(days=-1).format("M-D-YYYY")
    with open("last_sent.txt", "w") as f:
        f.write(now)

def check_today_sent() -> bool:
    if not os.path.exists("last_sent.txt"):
        f = open("last_sent.txt", "w+")
        f.close()
        return False

    now = arrow.utcnow().to("US/Eastern").shift(days=-1).format("M-D-YYYY")
    with open("last_sent.txt", "r") as f:
        last_sent = f.read()

    return now == last_sent

def send_summary(summary: str):
    now = arrow.utcnow().to("US/Eastern").shift(days=-1).format("M-D-YYYY")
    message = MIMEMultipart()
    message["To"] = TO
    message["From"] = EMAIL
    message["Subject"] = "Insiders Summary " + now
    message.attach(MIMEApplication(summary, NAME=now+".csv"))

    context = ssl.create_default_context()
    with smtplib.SMTP_SSL("smtp.gmail.com", PORT, context=context) as server:
        server.login(EMAIL, PASS)
        server.sendmail(EMAIL, TO, message.as_string())

def main():
    weeks_ago = arrow.utcnow().to("US/Eastern").shift(days=-14).format("YYYY-MM-DD")
    now = arrow.utcnow().to("US/Eastern").format("M-D-YYYY")
    print(f"Send summary from {weeks_ago} to {now}")

    try:
        with psycopg2.connect(CONN) as conn:
            with conn.cursor() as cur:
                cur.execute(
                    f"""
                    SELECT f."DateReported", "FullName", "Symbol", "ActionCode",
                    CAST(SUM("Amount") as money) as "Amount ($)", cast(AVG("AvgPrice") as money) as "AvgPrice"
                    from non_deriv_transaction
                    inner join form f on f."FormId" = non_deriv_transaction."FormId"
                    inner join issuer i on i."IssuerId" = f."IssuerId"
                    inner join individual i2 on i2."IndividualId" = non_deriv_transaction."IndividualId"
                    where "AvgPrice" > 0  and f."DateReported" > '{weeks_ago}'
                    group by f."DateReported", "FullName", "Symbol", "ActionCode"
                    order by "Symbol", "DateReported" desc, "Amount ($)" desc;
                    """
                )

                result = cur.fetchall()
                header = "Date,Name,Symbol,ActionCode,Amount,AvgPrice\n"
                csv_rows = [",".join([f"\"{c}\"" for c in r]) for r in result]
                csv = header + "\n".join(csv_rows)

                send_summary(csv)
                track_sent()

    except Exception as e:
        print("Error occurred:", e);

if __name__ == "__main__":
    while True:
        now = arrow.utcnow().to("US/Eastern")
        if now.time().hour == 1 and not check_today_sent() and now.weekday() not in [0, 6]:
            main()

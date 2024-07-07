from datetime import datetime
from typing import Generic, List, Type, TypeVar
import arrow
from bs4 import BeautifulSoup, PageElement
from models import Form4, OwnershipType, RelationshipType, NonDerivativeTransaction, DerivativeTransaction, TransactionType, WebForm

"""
        let company_cik = Self::traverse(&root, &["issuer", "issuerCik"]).unwrap().text;
        let rpt_owner_cik = Self::traverse(&root, &["reportingOwner", "reportingOwnerId", "rptOwnerCik"]).unwrap().text;
        let form_type = Self::traverse(&root, &["documentType"]).unwrap().text;
        let company = Self::traverse(&root, &["issuer", "issuerName"]).unwrap().text;
        let symbol =  Self::traverse(&root, &["issuer", "issuerTradingSymbol"]).unwrap().text;
        let owner = Self::traverse(&root, &["reportingOwner", "reportingOwnerId", "rptOwnerName"]).unwrap().text;
        let relationships = Self::get_relationship(&root);
        let form_date = Self::traverse(&root, &["periodOfReport"]).unwrap().parse_date();
        let web_url = self.get_web_url(&rpt_owner_cik);
    """
T = TypeVar('T')

def get_text_value(element: PageElement, tag: str, type: Type[T] = str) -> T | None:
    if child := element.find(tag):
        try:            
            if value := child.find('value'):
                stripped = value.text.upper().strip()
                if stripped:
                    return type(stripped)

            if value := child.text:
                stripped = value.upper().strip()
                if stripped:
                    return type(value)
        except Exception:
            return None

    return None

def parse_date(date: str) -> datetime:
    return datetime.strptime(date, '%Y-%m-%d')

class Form4Parser:
    @staticmethod
    def get_relationship(soup: BeautifulSoup) -> RelationshipType:
        for relationship in soup.find('reportingOwnerRelationship').children:
            if relationship.text == '1':
                return RelationshipType.from_xml(relationship.name)
        return RelationshipType.OTHER


    @staticmethod
    def get_non_deriv_transactions(soup: BeautifulSoup) -> List[NonDerivativeTransaction]:
        nonDerivativeTable = soup.find('nonDerivativeTable')
        if nonDerivativeTable is None:
            return []

        transactions = []
        for transaction in nonDerivativeTable.children:
            if transaction.name == 'nonDerivativeTransaction':
                transactions.append(NonDerivativeTransaction(
                    title=get_text_value(transaction, 'securityTitle'),
                    transactionDate=get_text_value(transaction, 'transactionDate', parse_date),
                    transactionType=TransactionType.from_xml(get_text_value(transaction, 'transactionAcquiredDisposedCode')),
                    transactionCode=get_text_value(transaction, 'transactionCode'),
                    ownership=OwnershipType.from_xml(get_text_value(transaction, 'directOrIndirectOwnership')),
                    price=get_text_value(transaction, 'transactionPricePerShare', float),
                    shares=get_text_value(transaction, 'transactionShares', float),
                    postOwnerShares=get_text_value(transaction, 'sharesOwnedFollowingTransaction', float)
                ))
        return transactions
    
    @staticmethod
    def get_derivative_transactions(soup: BeautifulSoup) -> List[DerivativeTransaction]:
        derivativeTable = soup.find('derivativeTable')
        if derivativeTable is None:
            return []

        transactions = []
        for transaction in derivativeTable.children:
            if transaction.name == 'derivativeTransaction':
                transactions.append(DerivativeTransaction(
                    title=get_text_value(transaction, 'securityTitle'),
                    transactionDate=get_text_value(transaction, 'transactionDate', parse_date),
                    transactionCode=get_text_value(transaction, 'transactionCode'),
                    transactionType=TransactionType.from_xml(get_text_value(transaction, 'transactionAcquiredDisposedCode')),
                    ownership=OwnershipType.from_xml(get_text_value(transaction, 'directOrIndirectOwnership')),
                    exercisePrice=get_text_value(transaction, 'conversionOrExercisePrice', float),
                    sharePrice=get_text_value(transaction, 'transactionPricePerShare', float),
                    expirationDate=get_text_value(transaction, 'expirationDate', parse_date),
                    exercisableDate=get_text_value(transaction, 'exerciseDate', parse_date),
                    postOwnerShares=get_text_value(transaction, 'sharesOwnedFollowingTransaction', float),
                    sharesTransacted=get_text_value(transaction, 'transactionShares', float)
                ))
        return transactions


    @staticmethod
    def parseXML(form: WebForm) -> Form4:
        soup = BeautifulSoup(form.content, 'xml')

        return Form4(
            companyCIK=get_text_value(soup, 'issuerCik'),
            ownerCIK=get_text_value(soup, 'rptOwnerCik'),
            formType=get_text_value(soup, 'documentType'),
            company=get_text_value(soup, 'issuerName'),
            symbol=get_text_value(soup, 'issuerTradingSymbol'),
            owner=get_text_value(soup, 'rptOwnerName'),
            formDate=get_text_value(soup, 'periodOfReport'),
            relationship=Form4Parser.get_relationship(soup),
            nonDerivativeTransactions=Form4Parser.get_non_deriv_transactions(soup),
            derivativeTransactions=Form4Parser.get_derivative_transactions(soup),
            formUrl=form.htmlUrl,
            xmlUrl=form.xmlUrl
        )

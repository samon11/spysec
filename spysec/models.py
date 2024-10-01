from typing import List, Optional, Union
from pydantic import BaseModel, Field, computed_field, model_validator
from datetime import datetime, timezone
from enum import Enum, auto

class TransactionType(str, Enum):
    BUY = 'B'
    SELL = 'S'

    @staticmethod
    def from_xml(value: str):
        if value == 'P':
            return TransactionType.BUY
        elif value == 'A':
            return TransactionType.BUY
        elif value == 'D':
            return TransactionType.SELL
        
class WebForm(BaseModel):
    htmlUrl: str
    xmlUrl: str
    content: str

class RelationshipType(str, Enum):
    DIRECTOR = 'D'
    OFFICER = 'O'
    TEN_PERCENT = '10'
    OTHER = '?'

    @staticmethod
    def from_xml(value: str):
        if value == 'isTenPercentOwner':
            return RelationshipType.TEN_PERCENT
        elif value == 'isDirector':
            return RelationshipType.DIRECTOR
        elif value == 'isOfficer':
            return RelationshipType.OFFICER
        else:
            return RelationshipType.OTHER

class OwnershipType(str, Enum):
    DIRECT = 'D'
    INDIRECT = 'I'

    @staticmethod
    def from_xml(value: str):
        if value == 'D':
            return OwnershipType.DIRECT
        elif value == 'I':
            return OwnershipType.INDIRECT

class DerivativeTransaction(BaseModel):
    title: str
    transactionDate: Optional[datetime]
    transactionType: Optional[TransactionType]
    transactionCode: Optional[str]
    ownership: OwnershipType
    exercisePrice: Optional[float]
    sharePrice: Optional[float]
    expirationDate: Optional[datetime]
    exercisableDate: Optional[datetime]
    postOwnerShares: Optional[float]
    sharesTransacted: Optional[float] = 0

    @property
    def monetaryValue(self):
        if self.exercisePrice and self.sharesTransacted:
            return self.exercisePrice * self.sharesTransacted
        
        return 0

class NonDerivativeTransaction(BaseModel):
    title: str
    transactionDate: datetime
    transactionCode: str
    transactionType: TransactionType
    ownership: OwnershipType
    price: Optional[float]
    shares: Optional[float]
    postOwnerShares: Optional[float]

    @property
    def monetaryValue(self):
        if self.price and self.shares:
            return self.price * self.shares
        
        return 0

class Filing(BaseModel):
    formDate: datetime
    company: str
    symbol: str
    owner: str
    relationship: RelationshipType
    companyCIK: str
    ownerCIK: str
    formType: str
    formUrl: str

class Form4(Filing):
    formType: str = "4"
    createdAtUtc: datetime = datetime.now(tz=timezone.utc)
    derivativeTransactions: Optional[List[DerivativeTransaction]]
    nonDerivativeTransactions: Optional[List[NonDerivativeTransaction]]
    xmlUrl: str

    def get_direction(self) -> str:
        total_buy = 0
        total_sell = 0
        
        if self.derivativeTransactions is not None:
            for transaction in self.derivativeTransactions:
                if transaction.transactionType == TransactionType.BUY:
                    total_buy += transaction.monetaryValue
                elif transaction.transactionType == TransactionType.SELL:
                    total_sell += transaction.monetaryValue
        
        if self.nonDerivativeTransactions is not None:
            for transaction in self.nonDerivativeTransactions:
                if transaction.transactionType == TransactionType.BUY:
                    total_buy += transaction.monetaryValue
                elif transaction.transactionType == TransactionType.SELL:
                    total_sell += transaction.monetaryValue
        
        if total_buy > total_sell:
            return "BUY"
        else:
            return "SELL"
        
    @property
    def monetaryValue(self):
        value = 0
        for transaction in self.nonDerivativeTransactions:
            value += transaction.monetaryValue

        for transaction in self.derivativeTransactions:
            value += transaction.monetaryValue

        return value
    
    @property
    def averageSharePrice(self):
        price = 0.
        i = 0

        for transaction in self.nonDerivativeTransactions:
            if transaction.price and transaction.price > 0:
                price += transaction.price
                i += 1

        for transaction in self.derivativeTransactions:
            if transaction.sharePrice and transaction.sharePrice > 0:
                price += transaction.sharePrice
                i += 1 

        return 0 if price == 0 else price / i

class RSSFeed(BaseModel):
    formType: str
    content: str
    createdAtUtc: datetime = datetime.now(tz=timezone.utc)
    
    @computed_field
    @property
    def feedHash(self) -> str:
        return str(hash(self.content))

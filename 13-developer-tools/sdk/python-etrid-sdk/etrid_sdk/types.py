"""Common types used throughout the SDK"""

from typing import TypeAlias
from pydantic import BaseModel

class Balance(BaseModel):
    """Account balance information"""
    free: int
    """Free balance"""
    reserved: int
    """Reserved balance"""
    frozen: int = 0
    """Frozen balance"""

class Block(BaseModel):
    """Block information"""
    number: int
    """Block number"""
    hash: str
    """Block hash"""
    parent_hash: str
    """Parent hash"""
    state_root: str
    """State root"""

TxHash: TypeAlias = str
"""Transaction hash"""

Address: TypeAlias = str
"""Account address (SS58 encoded)"""

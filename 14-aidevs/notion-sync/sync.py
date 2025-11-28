#!/usr/bin/env python3
"""
Notion Sync Service - Syncs governance docs from Notion to VectorDB
"""
import os
import time
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def main():
    logger.info("Notion sync service starting...")
    logger.info("Note: This is a placeholder. Implement full sync logic when Notion is configured.")
    
    # Keep the container running
    while True:
        time.sleep(300)  # Sleep 5 minutes
        logger.debug("Notion sync check...")

if __name__ == "__main__":
    main()

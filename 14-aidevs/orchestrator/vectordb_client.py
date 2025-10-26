"""
VectorDB Client - Manages persistent memory for AI agents
Uses Qdrant for vector storage
"""

import logging
from typing import Dict, List, Optional, Any
from datetime import datetime
from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams, PointStruct
from qdrant_client.http.exceptions import UnexpectedResponse
import hashlib

logger = logging.getLogger(__name__)

class VectorDBClient:
    """Client for VectorDB (Qdrant) operations"""

    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.client = None
        self.collection_name = config.get('collection', 'etrid-memory')
        self.vector_size = config.get('vector_size', 1536)

    async def initialize_collection(self):
        """Initialize VectorDB client and create collection if needed"""
        try:
            # Extract host and port from endpoint
            endpoint = self.config['endpoint']
            if endpoint.startswith('http://'):
                endpoint = endpoint[7:]
            elif endpoint.startswith('https://'):
                endpoint = endpoint[8:]

            parts = endpoint.split(':')
            host = parts[0]
            port = int(parts[1]) if len(parts) > 1 else 6333

            logger.info(f"Connecting to VectorDB at {host}:{port}")
            self.client = QdrantClient(host=host, port=port, timeout=10)

            # Create collection if it doesn't exist
            try:
                self.client.get_collection(self.collection_name)
                logger.info(f"Collection '{self.collection_name}' exists")
            except UnexpectedResponse:
                self.client.create_collection(
                    collection_name=self.collection_name,
                    vectors_config=VectorParams(
                        size=self.vector_size,
                        distance=Distance.COSINE
                    )
                )
                logger.info(f"Created collection '{self.collection_name}'")

        except Exception as e:
            logger.error(f"Error initializing VectorDB: {str(e)}")
            logger.warning("Continuing without VectorDB - will retry later")
            # Don't raise - allow the service to start without VectorDB
            return

    async def store_execution(
        self,
        agent: str,
        skill: str,
        result: Dict[str, Any],
        execution_time: float
    ):
        """Store a skill execution in memory"""
        try:
            # Generate a unique ID for this execution
            execution_id = hashlib.sha256(
                f"{agent}:{skill}:{datetime.utcnow().isoformat()}".encode()
            ).hexdigest()

            # For now, store without vectors (we'll add embeddings later)
            # This is a simplified version - in production you'd generate embeddings
            point = PointStruct(
                id=execution_id,
                vector=[0.0] * self.vector_size,  # Placeholder vector
                payload={
                    "agent": agent,
                    "skill": skill,
                    "result": result,
                    "execution_time": execution_time,
                    "timestamp": datetime.utcnow().isoformat()
                }
            )

            self.client.upsert(
                collection_name=self.collection_name,
                points=[point]
            )

            logger.debug(f"Stored execution: {agent}/{skill}")

        except Exception as e:
            logger.error(f"Error storing execution: {str(e)}")

    async def query_memories(self, agent: str, limit: int = 10) -> List[Dict[str, Any]]:
        """Retrieve recent memories for an agent"""
        try:
            # Use scroll to get recent memories for this agent
            results, _ = self.client.scroll(
                collection_name=self.collection_name,
                scroll_filter={
                    "must": [
                        {"key": "agent", "match": {"value": agent}}
                    ]
                },
                limit=limit,
                with_payload=True,
                with_vectors=False
            )

            memories = [point.payload for point in results]
            return memories

        except Exception as e:
            logger.error(f"Error querying memories: {str(e)}")
            return []

    async def health_check(self) -> Dict[str, Any]:
        """Check VectorDB health"""
        try:
            if not self.client:
                return {"status": "disconnected"}

            collection_info = self.client.get_collection(self.collection_name)
            return {
                "status": "healthy",
                "collection": self.collection_name,
                "points_count": collection_info.points_count
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "error": str(e)
            }

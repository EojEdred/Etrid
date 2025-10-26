#!/usr/bin/env python3
"""
Ëtrid AI Devs MCP Orchestrator
Main FastAPI server that manages all AI Dev agents
"""

from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.responses import JSONResponse
from pydantic import BaseModel
from typing import Dict, List, Optional, Any
import asyncio
import yaml
import os
import logging
from datetime import datetime
from pathlib import Path

from agents.compiler_agent import CompilerAgent
from agents.governance_agent import GovernanceAgent
from agents.runtime_agent import RuntimeAgent
from agents.economics_agent import EconomicsAgent
from agents.security_agent import SecurityAgent
from agents.oracle_agent import OracleAgent
from skills_loader import SkillsLoader
from vectordb_client import VectorDBClient
from blockchain_client import BlockchainClient

# Configure logging
logging.basicConfig(
    level=os.getenv('LOG_LEVEL', 'INFO').upper(),
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

# Initialize FastAPI
app = FastAPI(
    title="Ëtrid AI Devs Orchestrator",
    description="MCP Orchestrator for autonomous AI development agents",
    version="1.0.0"
)

# Helper function to expand environment variables in config
def expand_env_vars(config_obj):
    """Recursively expand environment variables in config"""
    if isinstance(config_obj, dict):
        return {k: expand_env_vars(v) for k, v in config_obj.items()}
    elif isinstance(config_obj, list):
        return [expand_env_vars(item) for item in config_obj]
    elif isinstance(config_obj, str):
        # Expand ${VAR} or $VAR patterns
        import re
        def replacer(match):
            var_name = match.group(1) or match.group(2)
            return os.getenv(var_name, match.group(0))
        return re.sub(r'\$\{([^}]+)\}|\$(\w+)', replacer, config_obj)
    else:
        return config_obj

# Load configuration
config_path = Path("/app/config/mcp_config.yaml")
with open(config_path) as f:
    config = yaml.safe_load(f)
    config = expand_env_vars(config)

# Initialize components
skills_loader = SkillsLoader(config['skills']['execution_timeout'])
vectordb = VectorDBClient(config['vectordb'])
blockchain = BlockchainClient(config['blockchain'])

# Initialize agents
agents: Dict[str, Any] = {}

class SkillExecutionRequest(BaseModel):
    agent_name: str
    skill_name: str
    parameters: Optional[Dict[str, Any]] = {}

class SkillExecutionResponse(BaseModel):
    success: bool
    agent: str
    skill: str
    output: Optional[str]
    error: Optional[str]
    execution_time: float
    timestamp: str

@app.on_event("startup")
async def startup_event():
    """Initialize all AI agents on startup"""
    logger.info("Starting Ëtrid AI Devs Orchestrator...")

    # Load all skills
    skills_path = Path(config['skills_path'])
    logger.info(f"Loading skills from {skills_path}")
    await skills_loader.load_all_skills(skills_path)

    # Initialize VectorDB collection
    await vectordb.initialize_collection()

    # Connect to blockchain
    await blockchain.connect()

    # Initialize agents
    agent_configs = config['agents']

    if agent_configs['compiler_ai']['auto_start']:
        agents['compiler'] = CompilerAgent(
            config=agent_configs['compiler_ai'],
            skills_loader=skills_loader,
            vectordb=vectordb,
            blockchain=blockchain
        )
        await agents['compiler'].start()
        logger.info("✅ Compiler AI started")

    if agent_configs['governance_ai']['auto_start']:
        agents['governance'] = GovernanceAgent(
            config=agent_configs['governance_ai'],
            skills_loader=skills_loader,
            vectordb=vectordb,
            blockchain=blockchain
        )
        await agents['governance'].start()
        logger.info("✅ Governance AI started")

    if agent_configs['runtime_ai']['auto_start']:
        agents['runtime'] = RuntimeAgent(
            config=agent_configs['runtime_ai'],
            skills_loader=skills_loader,
            vectordb=vectordb,
            blockchain=blockchain
        )
        await agents['runtime'].start()
        logger.info("✅ Runtime AI started")

    if agent_configs['economics_ai']['auto_start']:
        agents['economics'] = EconomicsAgent(
            config=agent_configs['economics_ai'],
            skills_loader=skills_loader,
            vectordb=vectordb,
            blockchain=blockchain
        )
        await agents['economics'].start()
        logger.info("✅ Economics AI started")

    if agent_configs['security_ai']['auto_start']:
        agents['security'] = SecurityAgent(
            config=agent_configs['security_ai'],
            skills_loader=skills_loader,
            vectordb=vectordb,
            blockchain=blockchain
        )
        await agents['security'].start()
        logger.info("✅ Security AI started")

    if agent_configs['oracle_ai']['auto_start']:
        agents['oracle'] = OracleAgent(
            config=agent_configs['oracle_ai'],
            skills_loader=skills_loader,
            vectordb=vectordb,
            blockchain=blockchain
        )
        await agents['oracle'].start()
        logger.info("✅ Oracle AI started")

    logger.info(f"🚀 All {len(agents)} AI Devs operational")

@app.on_event("shutdown")
async def shutdown_event():
    """Cleanup on shutdown"""
    logger.info("Shutting down AI Devs...")
    for name, agent in agents.items():
        await agent.stop()
        logger.info(f"Stopped {name}")
    await blockchain.disconnect()
    logger.info("Shutdown complete")

@app.get("/")
async def root():
    """Health check endpoint"""
    return {
        "status": "operational",
        "orchestrator": config['orchestrator']['name'],
        "version": config['orchestrator']['version'],
        "active_agents": len(agents),
        "timestamp": datetime.utcnow().isoformat()
    }

@app.get("/health")
async def health_check():
    """Detailed health check"""
    agent_status = {}
    for name, agent in agents.items():
        agent_status[name] = await agent.get_status()

    return {
        "status": "healthy",
        "agents": agent_status,
        "vectordb": await vectordb.health_check(),
        "blockchain": await blockchain.health_check(),
        "timestamp": datetime.utcnow().isoformat()
    }

@app.get("/agents")
async def list_agents():
    """List all active agents and their capabilities"""
    agent_list = []
    for name, agent in agents.items():
        agent_list.append({
            "name": name,
            "did": agent.config['did'],
            "description": agent.config['description'],
            "skills": agent.config['skills'],
            "priority": agent.config['priority'],
            "status": await agent.get_status()
        })
    return {"agents": agent_list}

@app.get("/skills")
async def list_skills():
    """List all available skills"""
    return {
        "total_skills": len(skills_loader.skills),
        "skills": list(skills_loader.skills.keys())
    }

@app.post("/execute", response_model=SkillExecutionResponse)
async def execute_skill(request: SkillExecutionRequest):
    """Execute a skill on a specific agent"""
    start_time = datetime.utcnow()

    # Validate agent exists
    if request.agent_name not in agents:
        raise HTTPException(
            status_code=404,
            detail=f"Agent '{request.agent_name}' not found"
        )

    agent = agents[request.agent_name]

    # Validate skill is available to this agent
    if request.skill_name not in agent.config['skills']:
        raise HTTPException(
            status_code=403,
            detail=f"Skill '{request.skill_name}' not available to agent '{request.agent_name}'"
        )

    try:
        # Execute skill
        result = await agent.execute_skill(
            skill_name=request.skill_name,
            parameters=request.parameters
        )

        execution_time = (datetime.utcnow() - start_time).total_seconds()

        # Store execution in memory
        await vectordb.store_execution(
            agent=request.agent_name,
            skill=request.skill_name,
            result=result,
            execution_time=execution_time
        )

        return SkillExecutionResponse(
            success=True,
            agent=request.agent_name,
            skill=request.skill_name,
            output=result.get('output'),
            error=None,
            execution_time=execution_time,
            timestamp=start_time.isoformat()
        )

    except Exception as e:
        logger.error(f"Skill execution failed: {str(e)}")
        execution_time = (datetime.utcnow() - start_time).total_seconds()

        return SkillExecutionResponse(
            success=False,
            agent=request.agent_name,
            skill=request.skill_name,
            output=None,
            error=str(e),
            execution_time=execution_time,
            timestamp=start_time.isoformat()
        )

@app.get("/memory/{agent_name}")
async def get_agent_memory(agent_name: str, limit: int = 10):
    """Retrieve recent memory entries for an agent"""
    if agent_name not in agents:
        raise HTTPException(status_code=404, detail=f"Agent '{agent_name}' not found")

    memories = await vectordb.query_memories(agent_name, limit=limit)
    return {"agent": agent_name, "memories": memories}

@app.get("/metrics")
async def get_metrics():
    """Get orchestrator metrics"""
    metrics = {
        "total_agents": len(agents),
        "total_skills": len(skills_loader.skills),
        "uptime": "calculated_here",  # TODO: implement uptime tracking
        "executions_total": 0,  # TODO: track from vectordb
        "executions_success": 0,
        "executions_failed": 0,
    }

    # Get per-agent metrics
    agent_metrics = {}
    for name, agent in agents.items():
        agent_metrics[name] = await agent.get_metrics()

    metrics['agents'] = agent_metrics
    return metrics

@app.post("/trigger/compile")
async def trigger_compile(background_tasks: BackgroundTasks):
    """Trigger compilation on Compiler AI"""
    background_tasks.add_task(
        agents['compiler'].execute_skill,
        skill_name='etrid-compile-build',
        parameters={}
    )
    return {"message": "Compilation triggered in background"}

@app.post("/trigger/governance")
async def trigger_governance_proposal(
    background_tasks: BackgroundTasks,
    proposal_type: str = "general"
):
    """Trigger governance proposal generation"""
    background_tasks.add_task(
        agents['governance'].execute_skill,
        skill_name='proposal-generator',
        parameters={'type': proposal_type}
    )
    return {"message": "Governance proposal generation triggered"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=4000)

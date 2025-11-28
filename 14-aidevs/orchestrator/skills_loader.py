"""
Skills Loader - Parses SKILL.md files and executes associated scripts
"""

import os
import re
import subprocess
import logging
from pathlib import Path
from typing import Dict, Optional, Any
import yaml

logger = logging.getLogger(__name__)

class Skill:
    """Represents a single skill package"""

    def __init__(self, name: str, path: Path):
        self.name = name
        self.path = path
        self.metadata = {}
        self.script_path = None
        self.memory_path = None
        self._load_skill()

    def _load_skill(self):
        """Load skill metadata from SKILL.md"""
        skill_md_path = self.path / "SKILL.md"

        if not skill_md_path.exists():
            logger.warning(f"SKILL.md not found for {self.name}")
            return

        with open(skill_md_path, 'r') as f:
            content = f.read()

        # Parse metadata from SKILL.md (YAML frontmatter)
        yaml_match = re.search(r'^---\n(.*?)\n---', content, re.DOTALL)
        if yaml_match:
            self.metadata = yaml.safe_load(yaml_match.group(1))

        # Find script path
        scripts_dir = self.path / "scripts"
        if scripts_dir.exists():
            # Look for main execution script
            for script_name in ["run.sh", "execute.sh", "main.sh", f"{self.name}.sh"]:
                script = scripts_dir / script_name
                if script.exists():
                    self.script_path = script
                    break

        # Find memory file
        memory_file = self.path / "MEMORY.md"
        if memory_file.exists():
            self.memory_path = memory_file

    def execute(self, parameters: Dict[str, Any] = None, workspace_path: str = "/workspace") -> Dict[str, Any]:
        """Execute the skill script"""
        if not self.script_path:
            return {
                "success": False,
                "error": f"No executable script found for skill {self.name}"
            }

        try:
            # Prepare environment
            env = os.environ.copy()
            env['SKILL_NAME'] = self.name
            env['WORKSPACE_PATH'] = workspace_path

            # Add parameters as env vars
            if parameters:
                for key, value in parameters.items():
                    env[f'PARAM_{key.upper()}'] = str(value)

            # Execute script
            logger.info(f"Executing skill: {self.name}")
            result = subprocess.run(
                ["/bin/bash", str(self.script_path)],
                cwd=workspace_path,
                env=env,
                capture_output=True,
                text=True,
                timeout=300  # 5 minutes default
            )

            return {
                "success": result.returncode == 0,
                "output": result.stdout,
                "error": result.stderr if result.returncode != 0 else None,
                "return_code": result.returncode
            }

        except subprocess.TimeoutExpired:
            return {
                "success": False,
                "error": f"Skill execution timed out after 300 seconds"
            }
        except Exception as e:
            logger.error(f"Skill execution error: {str(e)}")
            return {
                "success": False,
                "error": str(e)
            }

    def get_memory(self) -> Optional[str]:
        """Read the skill's memory file"""
        if not self.memory_path:
            return None

        try:
            with open(self.memory_path, 'r') as f:
                return f.read()
        except Exception as e:
            logger.error(f"Error reading memory for {self.name}: {str(e)}")
            return None


class SkillsLoader:
    """Manages loading and execution of all skills"""

    def __init__(self, execution_timeout: int = 300):
        self.skills: Dict[str, Skill] = {}
        self.execution_timeout = execution_timeout

    async def load_all_skills(self, skills_path: Path):
        """Load all skill packages from the skills directory"""
        if not skills_path.exists():
            logger.error(f"Skills path does not exist: {skills_path}")
            return

        # Iterate through skill directories
        for skill_dir in skills_path.iterdir():
            if skill_dir.is_dir() and not skill_dir.name.startswith('.'):
                skill_name = skill_dir.name
                skill = Skill(name=skill_name, path=skill_dir)
                self.skills[skill_name] = skill
                logger.info(f"Loaded skill: {skill_name}")

        logger.info(f"Total skills loaded: {len(self.skills)}")

    def get_skill(self, name: str) -> Optional[Skill]:
        """Get a skill by name"""
        return self.skills.get(name)

    def list_skills(self) -> list:
        """List all available skills"""
        return list(self.skills.keys())

    async def execute_skill(self, name: str, parameters: Dict[str, Any] = None, workspace_path: str = "/workspace") -> Dict[str, Any]:
        """Execute a skill by name"""
        skill = self.get_skill(name)
        if not skill:
            return {
                "success": False,
                "error": f"Skill '{name}' not found"
            }

        return skill.execute(parameters=parameters, workspace_path=workspace_path)

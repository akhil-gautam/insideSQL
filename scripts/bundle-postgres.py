"""Copy the pinned relocatable runtime; never connect to its databases."""
import json, os, shutil, subprocess
from pathlib import Path
source=Path(os.environ.get('ACADEMY_POSTGRES_SOURCE','/Applications/Postgres.app/Contents/Versions/18'))
target=Path(__file__).resolve().parents[1]/'src-tauri/resources/postgres'
version=subprocess.check_output([str(source/'bin/postgres'),'--version'],text=True).strip()
if '18.3' not in version: raise SystemExit('Phase 1 runtime is pinned to PostgreSQL 18.3')
target.mkdir(parents=True,exist_ok=True)
for directory in ['bin','lib','share']:
    shutil.copytree(source/directory,target/directory,dirs_exist_ok=True,symlinks=True)
(target/'runtime.json').write_text(json.dumps({'version':version,'origin':'Postgres.app','platform':'macOS universal','directories':['bin','lib','share']},indent=2))
print(f'Bundled {version} in {target}')

"""Request the user-authorized read-only Claude review; skip if nonresponsive."""
import argparse, json, subprocess, time
from pathlib import Path
parser=argparse.ArgumentParser();parser.add_argument('phase',type=int);args=parser.parse_args()
root=Path(__file__).resolve().parents[1]
files=[root/'docs/PLAN.md',root/'docs/PROGRESS.md',root/'docs/LICENSING.md']
for directory,patterns in [('crates',['*.rs','Cargo.toml']),('src-tauri',['*.rs','*.json','Cargo.toml']),('web/src',['*.tsx','*.ts']),('scripts',['*.py','*.mjs']),('course',['chapter*.json'])]:
    for pattern in patterns:
        files.extend(p for p in (root/directory).rglob(pattern) if not any(part in ['resources','icons','generated','gen','node_modules'] for part in p.parts))
body=f'Review completed Phase {args.phase} of this implementation. Read-only: do not edit files or execute commands. Return concrete correctness, security, lesson-accuracy and requirement gaps with file/line references and severity. Distinguish phase requirements from future phases. Do not request credentials. Only report what the supplied code proves.\n'
for file in dict.fromkeys(files):body+=f'\n--- {file.relative_to(root)} ---\n{file.read_text()}\n'
output=root/'docs/reviews';output.mkdir(parents=True,exist_ok=True)
start=time.monotonic()
try:
    result=subprocess.run(['claude','-p','--tools','','--strict-mcp-config','--mcp-config','{"mcpServers":{}}','--no-session-persistence','--settings','{"disableAllHooks":true}'],input=body,text=True,capture_output=True,cwd=root,timeout=90)
    text=result.stdout or result.stderr
    status='responded' if result.returncode==0 and text.strip() else 'unavailable'
except subprocess.TimeoutExpired:
    text='Claude did not respond within the 90-second review deadline; review skipped as authorized.';status='timeout'
record={'phase':args.phase,'status':status,'elapsed_seconds':round(time.monotonic()-start,1),'review':text}
(output/f'phase-{args.phase}.json').write_text(json.dumps(record,indent=2))
(output/f'phase-{args.phase}.md').write_text(f'# Claude phase {args.phase} review\n\nStatus: {status}\n\n{text}\n')
print(f'Claude phase {args.phase}: {status}. Review saved in docs/reviews/.')

"""Produce local English narration and captions with exact clip boundaries."""
import json, re, subprocess
from pathlib import Path
root=Path(__file__).resolve().parents[1]
chapter=json.loads((root/'course/chapter01.json').read_text())
output=root/'web/public/media/01';output.mkdir(parents=True,exist_ok=True)
cache=root/'.academy/narration';cache.mkdir(parents=True,exist_ok=True)
captions=[];offset=0.0;files=[]
def clip_duration(path):
    if not path.exists(): return None
    value=subprocess.check_output(['ffprobe','-v','error','-show_entries','format=duration','-of','default=noprint_wrappers=1:nokey=1',str(path)],text=True).strip()
    try: return float(value)
    except ValueError: return None
for section,paragraph in enumerate(chapter['transcript']):
    for sentence in re.split(r'(?<=[.!?])\s+',paragraph):
        words=sentence.split()
        for first in range(0,len(words),18):
            text=' '.join(words[first:first+18]);index=len(captions)
            audio=cache/f'{index:03}.aiff';stamp=cache/f'{index:03}.txt'
            duration=clip_duration(audio)
            if duration is None or not stamp.exists() or stamp.read_text()!=text:
                subprocess.run(['say','-v','Samantha','-r','155','-o',str(audio),text],check=True)
                duration=clip_duration(audio)
                if duration is None or duration<=0: raise RuntimeError(f'Speech synthesis did not produce a valid clip: {audio}')
                stamp.write_text(text)
            assert duration>0
            captions.append({'text':text,'start':offset,'end':offset+duration,'section':section})
            offset+=duration;files.append(audio)
    print(f'Narrated section {section+1}/{len(chapter["transcript"])}',flush=True)
concat=cache/'concat.txt';concat.write_text(''.join(f"file '{p}'\n" for p in files))
subprocess.run(['ffmpeg','-y','-v','error','-f','concat','-safe','0','-i',str(concat),'-c:a','aac','-b:a','160k',str(output/'narration.m4a')],check=True)
def timestamp(seconds):
    ms=round(seconds*1000);return f'{ms//3600000:02}:{ms//60000%60:02}:{ms//1000%60:02}.{ms%1000:03}'
(output/'captions.vtt').write_text('WEBVTT\n\n'+'\n\n'.join(f'{i+1}\n{timestamp(c["start"])} --> {timestamp(c["end"])}\n{c["text"]}' for i,c in enumerate(captions))+'\n')
metadata={'fps':30,'durationInFrames':int(offset*30)+30,'durationSeconds':offset,'captions':captions}
generated=root/'web/src/generated';generated.mkdir(parents=True,exist_ok=True)
(generated/'narration.json').write_text(json.dumps(metadata,indent=2))
(output/'narration.json').write_text(json.dumps(metadata,indent=2))
(output/'transcript.txt').write_text('\n\n'.join(chapter['transcript']))
print(f'Created {len(captions)} synchronized captions and {offset:.1f} seconds of narration',flush=True)

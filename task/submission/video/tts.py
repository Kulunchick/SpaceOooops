# Generates voice.mp3 + words.json with ElevenLabs (AI-generated speech). Needs ELEVEN_LABS_API_KEY in ../../../.env
import json, base64, urllib.request, urllib.error, pathlib, sys
env=dict(l.strip().split('=',1) for l in open(pathlib.Path(__file__).resolve().parents[3]/'.env') if '=' in l)
key=env['ELEVEN_LABS_API_KEY'].strip().strip('"')
body=json.load(open(pathlib.Path(__file__).with_name('tts.json')))
req=urllib.request.Request('https://api.elevenlabs.io/v1/text-to-speech/uOAJfsnu0eswzATsYCJq/with-timestamps?output_format=mp3_44100_128',
    data=json.dumps(body).encode(), headers={'xi-api-key':key,'Content-Type':'application/json'})
try: d=json.load(urllib.request.urlopen(req))
except urllib.error.HTTPError as e: sys.exit(f'ElevenLabs HTTP {e.code}: {e.read().decode()[:300]}')
open('voice.mp3','wb').write(base64.b64decode(d['audio_base64']))
a=d['alignment']; w='';st=None;out=[]
for c,s,e in zip(a['characters'],a['character_start_times_seconds'],a['character_end_times_seconds']):
    if c==' ':
        if w: out.append((w,st,pe)); w='';st=None
    else:
        if st is None: st=s
        w+=c; pe=e
if w: out.append((w,st,pe))
json.dump(out,open('words.json','w')); print('voice',round(out[-1][2],2),'s,',len(out),'words')

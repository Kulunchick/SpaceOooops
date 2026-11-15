import json, os, subprocess, sys
A='D:/Projects/SpaceOooops/assets/audio/'
VOICE_AT=0.2
voice=os.path.exists('voice.mp3') and os.path.exists('words.json')
W=json.load(open('words.json')); T=lambda i: W[i][1]+VOICE_AT
t_title=T(18)-0.15                      # "SpaceOooops: design it..."
t_click=T(36)-0.1                       # "...instrument costs"
t_review=min(T(42)-0.1, t_click+2.87)   # "Pass the launch review"
t_lift=max(T(46)-1.0, t_review+1.0)     # lift-off just before "survive solar flares"
t_flare=t_lift+0.7
t_ev=T(49)-0.1                         # "Fail, learn, fly again..." -> three more events
t_deb=T(60)-0.1                        # "Then read a debrief..."
e=(t_deb-t_ev)/3
TOTAL=round(W[-1][2]+VOICE_AT+1.5,2); assert TOTAL<=29.5, TOTAL
assert TOTAL-t_deb<=7.4
slow=3.63/(t_click-t_title)
segs=[('take1.mp4',0.5,t_title,1),('take2_60.mp4',0.27,3.63,slow),('take2.mp4',3.9,t_review-t_click,1),('take2.mp4',6.9,t_lift-t_review,1),
 ('take3.mp4',6.6,t_flare-t_lift,1),('take6.mp4',1.3,t_ev-t_flare,1),('fx14.mp4',0.2,e,1),('fx133.mp4',2.75,e,1),('take5.mp4',20.45,e,1),('take4.mp4',23.25,TOTAL-t_deb,1)]
t_end=t_deb
print('cuts',[round(x,2) for x in (t_title,t_click,t_review,t_lift,t_flare,t_ev,t_deb,TOTAL)],'slow',round(slow,2))
cues_def=[("We are team SpaceOooops.",5),("Students rarely get to fly a whole space mission,@@so we built one.",13),
 ("SpaceOooops: design it, launch it,@@find out why it failed.",11),("Every choice is a trade-off: a better instrument@@costs mass, power and money.",13),
 ("Pass the launch review, survive solar flares.",7),("Fail, learn, fly again,@@solo or as a crew of four.",11),("Then read a debrief grounded in@@NASA mission design guides.",10)]
est=[(0.3,2.0),(2.1,5.4),(5.7,9.7),(10.1,15.1),(15.4,19.2),(19.2,23.2),(23.6,28.5)]
cues=[]
if voice:
    w=json.load(open('words.json')); assert len(w)==sum(c[1] for c in cues_def), len(w)
    i=0
    for text,n in cues_def:
        cues.append((w[i][1]+VOICE_AT, w[i+n-1][2]+VOICE_AT+0.15, text)); i+=n
    for k in range(len(cues)-1):
        if cues[k][1]>cues[k+1][0]: cues[k]=(cues[k][0],cues[k+1][0],cues[k][2])
    assert cues[-1][1] < TOTAL-0.2, f'voice too long: {cues[-1][1]}'
else:
    cues=[(a,b,c[0]) for (a,b),c in zip(est,cues_def)]
def ts(t): return f'{int(t//3600)}:{int(t//60)%60:02d}:{t%60:05.2f}'
ass="""[Script Info]
ScriptType: v4.00+
PlayResX: 1920
PlayResY: 1080
WrapStyle: 2

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
Style: S,Arial,46,&H00FFFFFF,&H00FFFFFF,&H38160D0A,&H38160D0A,-1,0,0,0,100,100,0,0,3,10,0,2,60,60,104,1

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
"""+''.join(f'Dialogue: 0,{ts(a)},{ts(b)},S,,0,0,0,,{t.replace('@@',chr(92)+'N')}'+chr(10) for a,b,t in cues)
def st(t): return f'{int(t//3600):02d}:{int(t//60)%60:02d}:{int(t%60):02d},{int(round((t%1)*1000))%1000:03d}'
open('SpaceOooops-demo.en.srt','w',encoding='utf8').write(''.join(f'{i+1}'+chr(10)+f'{st(a)} --> {st(b)}'+chr(10)+t.replace('@@',chr(10))+chr(10)+chr(10) for i,(a,b,t) in enumerate(cues)))
cmd=['ffmpeg','-v','error','-y']
for f,ss,d,sp in segs: cmd+=['-ss',str(ss),'-t',str(d),'-i',f]
n=len(segs)
for o in ('o1.png','o2.png','o3.png'): cmd+=['-loop','1','-t',str(TOTAL),'-i',o]
cmd+=['-ss','4','-t',str(TOTAL),'-i',A+'music-run.mp3','-i',A+'launch-engine.ogg']
if voice: cmd+=['-i','voice.mp3']
ov=[(0.3,t_title-0.3),(t_title+0.3,T(29)-0.1),(t_end+0.3,TOTAL)]
fc=''.join(f'[{i}:v]setpts=(PTS-STARTPTS)/{segs[i][3]},fps=30,format=yuv420p[v{i}];' for i in range(n))
fc+=''.join(f'[v{i}]' for i in range(n))+f'concat=n={n}:v=1:a=0[base];'
last='base'
for k,(a,b) in enumerate(ov):
    fo = f',fade=out:st={b-0.3}:d=0.3:alpha=1' if b<TOTAL else ''
    fc+=f'[{n+k}:v]format=rgba,fade=in:st={a}:d=0.3:alpha=1{fo}[o{k}];[{last}][o{k}]overlay=enable=\'between(t,{a},{b})\'[b{k}];'; last=f'b{k}'
fc+=f'[{last}]fade=out:st={TOTAL-0.4}:d=0.4,format=yuv420p[vout];'
mv=0.08 if voice else 0.45
fc+=f'[{n+3}:a]volume={mv},afade=in:st=0:d=0.5,afade=out:st={TOTAL-1.2}:d=1.2[m];'
fc+=f'[{n+4}:a]atrim=0:1.2,afade=in:st=0:d=0.15,afade=out:st=0.8:d=0.4,volume=0.12,adelay={int(t_lift*1000)}:all=1[e];'
if voice:
    fc+=f'[{n+5}:a]adelay={int(VOICE_AT*1000)}:all=1,acompressor=threshold=-20dB:ratio=3:attack=5:release=120:makeup=2,loudnorm=I=-15:TP=-1.5:LRA=7,aresample=44100[vo];[m][e][vo]amix=inputs=3:normalize=0:duration=first,alimiter=limit=0.95[aout]'
else:
    fc+='[m][e]amix=inputs=2:normalize=0:duration=first[aout]'
comment=('Voice-over generated with ElevenLabs (AI-generated speech). ' if voice else '')+'Gameplay captured from the SpaceOooops game. Music: "Space Flight" by wipics (CC0). Launch sound: Kenney Sci-Fi Sounds (CC0).'
out='SpaceOooops-demo.mp4' if voice else 'SpaceOooops-demo-novoice.mp4'
cmd+=['-filter_complex',fc,'-map','[vout]','-map','[aout]','-t',str(TOTAL),'-c:v','libx264','-crf','18','-preset','slow','-pix_fmt','yuv420p','-c:a','aac','-b:a','192k','-movflags','+faststart',
 '-metadata','title=SpaceOooops - Space Mission Design Game (30 s demo)','-metadata','artist=Team SpaceOooops','-metadata','comment='+comment,out]
subprocess.run(cmd,check=True)
print(out, subprocess.run(['ffprobe','-v','error','-show_entries','format=duration,size','-of','csv=p=0',out],capture_output=True,text=True).stdout.strip(), 'voice' if voice else 'NO VOICE')

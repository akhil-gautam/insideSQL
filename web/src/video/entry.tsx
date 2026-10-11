import {Composition,registerRoot} from 'remotion';
import {ChapterVideo,EngineCinematic} from './ChapterVideo';
import narration from '../generated/narration.json';
function Root(){return <><Composition id="EngineCinematic" component={EngineCinematic} durationInFrames={630} fps={30} width={1920} height={1080}/><Composition id="Chapter01" component={ChapterVideo} durationInFrames={narration.durationInFrames} fps={30} width={1920} height={1080}/></>;}
registerRoot(Root);

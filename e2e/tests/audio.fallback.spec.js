// Audio must work with WebGL disabled (it never depends on the renderer).
import { test } from '@playwright/test';
import { registerAudioTests } from './audio.js';

registerAudioTests(test);
